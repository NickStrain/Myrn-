//! Body hash computed from the syntax tree with whitespace and comments
//! removed, so reformatting, comment edits and CRLF/LF checkouts of the same
//! code produce the same hash.

use tree_sitter::Node;

pub fn body_hash(node: Node<'_>, src: &[u8]) -> String {
    let mut h = blake3::Hasher::new();
    let mut cursor = node.walk();
    let mut descend = true;
    loop {
        let n = cursor.node();
        if descend && !n.kind().contains("comment") {
            if n.child_count() == 0 {
                h.update(&src[n.byte_range()]);
                h.update(b"\0");
            } else if cursor.goto_first_child() {
                continue;
            }
        }
        if cursor.goto_next_sibling() {
            descend = true;
            continue;
        }
        if !cursor.goto_parent() {
            break;
        }
        descend = false;
    }
    h.finalize().to_hex()[..16].to_string()
}

#[cfg(test)]
mod tests {
    use crate::indexer::lang::Lang;
    use crate::indexer::parse::FileParser;

    fn hash_of(src: &str) -> String {
        let p = FileParser::new()
            .parse(Lang::Rust, "src/lib.rs", src.as_bytes())
            .unwrap();
        p.defs[0].body_hash.clone()
    }

    #[test]
    fn ignores_formatting_and_comments() {
        let a = hash_of("fn f(x: i32) -> i32 { x + 1 }");
        let b = hash_of("fn f(x: i32)\r\n    -> i32 {\r\n    // add one\r\n    x + 1\r\n}\r\n");
        assert_eq!(a, b);
        let c = hash_of("fn f(x: i32) -> i32 { x + 2 }");
        assert_ne!(a, c);
    }
}
