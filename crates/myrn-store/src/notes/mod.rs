//! Markdown notes: one file per record, named by content hash, Obsidian-compatible.
//!
//! The notes are the source of truth for earned knowledge. Write the note
//! first, then index it (`sync`), so the files are always the complete record.

pub mod frontmatter;
pub mod layout;
pub mod wikilink;

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

pub use frontmatter::{Anchor, Evidence, Link, Note, Parsed};

/// Shortest hash length for a new id; grown on the rare collision.
const ID_HEX: usize = 6;

/// Write `note` under `root` (a notes or knowledge directory) into its type's
/// folder. A note without an id gets one from its content. If a different note
/// already has that id, the id is lengthened until it is unique. Returns the
/// file written.
pub fn write(root: &Path, note: &mut Note, note_ids: &dyn Fn(&str) -> bool) -> Result<PathBuf> {
    let Some(folder) = layout::folder_for(&note.ntype) else {
        bail!("unknown note type `{}`", note.ntype);
    };
    let dir = root.join(folder);
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    if note.id.is_empty() {
        let mut len = ID_HEX;
        loop {
            note.id = layout::make_id(&note.ntype, &note.body, len);
            let path = dir.join(layout::file_name(&note.id));
            let clash = match std::fs::read_to_string(&path) {
                Ok(existing) => {
                    let other =
                        frontmatter::parse(&existing, &layout::safe_stem(&note.id), Some(folder));
                    other
                        .note
                        .body
                        .split_whitespace()
                        .ne(note.body.split_whitespace())
                }
                Err(_) => false,
            };
            if !clash || len >= 64 {
                break;
            }
            len += 2;
        }
    }
    let path = dir.join(layout::file_name(&note.id));
    let text = frontmatter::render(note, note_ids);
    let tmp = path.with_extension(format!("md.{}.tmp", std::process::id()));
    std::fs::write(&tmp, text).with_context(|| format!("writing {}", tmp.display()))?;
    std::fs::rename(&tmp, &path).with_context(|| format!("writing {}", path.display()))?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_into_type_folder_with_content_id() {
        let dir = tempfile::tempdir().unwrap();
        let mut n = Note::new("root_cause", "Clock skew makes the window check fail.\n");
        let p = write(dir.path(), &mut n, &|_| false).unwrap();
        assert!(n.id.starts_with("rootcause-"));
        assert_eq!(p, dir.path().join("bugs").join(format!("{}.md", n.id)));
        let back = frontmatter::parse(&std::fs::read_to_string(&p).unwrap(), &n.id, Some("bugs"));
        assert!(back.problems.is_empty(), "{:?}", back.problems);
        assert_eq!(back.note, n);

        // Same content, same id, same file.
        let mut again = Note::new("root_cause", "Clock  skew makes the window check fail.");
        assert_eq!(write(dir.path(), &mut again, &|_| false).unwrap(), p);
    }

    #[test]
    fn id_collision_lengthens_the_id() {
        let dir = tempfile::tempdir().unwrap();
        let mut n = Note::new("fact", "one");
        let id = layout::make_id("fact", "one", ID_HEX);
        // Squat the id with different content.
        std::fs::create_dir_all(dir.path().join("facts")).unwrap();
        std::fs::write(
            dir.path().join("facts").join(format!("{id}.md")),
            format!("---\nid: {id}\ntype: fact\n---\nsomething else\n"),
        )
        .unwrap();
        write(dir.path(), &mut n, &|_| false).unwrap();
        assert_ne!(n.id, id);
        assert!(n.id.starts_with(&id));
    }
}
