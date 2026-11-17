//! Extract [[wikilinks]] from note bodies; indexed as generic `related` edges.
//!
//! Handles the Obsidian forms `[[target]]`, `[[target|alias]]`,
//! `[[target#heading]]`, `![[embed]]` and `[[folder/target.md]]`. Links inside
//! fenced code blocks and inline code are not links.

/// Link targets in order of first appearance, without duplicates.
pub fn extract(body: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut fence: Option<&str> = None;
    for line in body.lines() {
        let t = line.trim_start();
        for marker in ["```", "~~~"] {
            if t.starts_with(marker) {
                fence = match fence {
                    None => Some(marker),
                    Some(m) if m == marker => None,
                    other => other,
                };
            }
        }
        if fence.is_some() || t.starts_with("```") || t.starts_with("~~~") {
            continue;
        }
        // Drop inline code spans: odd segments between backticks.
        let text: String = line
            .split('`')
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, s)| s)
            .collect::<Vec<_>>()
            .join(" ");
        let mut rest = text.as_str();
        while let Some(start) = rest.find("[[") {
            let after = &rest[start + 2..];
            let Some(end) = after.find("]]") else { break };
            if let Some(t) = normalize(&after[..end]) {
                if !out.contains(&t) {
                    out.push(t);
                }
            }
            rest = &after[end + 2..];
        }
    }
    out
}

/// `folder/Target.md#Heading|alias` -> `Target`. Also used for header values.
pub fn normalize(raw: &str) -> Option<String> {
    let t = raw.split('|').next()?;
    let t = t.split('#').next()?;
    let t = t.trim().trim_end_matches(".md");
    let t = t.rsplit('/').next()?.trim();
    (!t.is_empty()).then(|| t.to_string())
}

/// Strip `[[ ]]` from a header value: `"[[rootcause-a12b08]]"` -> `rootcause-a12b08`.
pub fn unwrap_link(value: &str) -> String {
    let v = value.trim();
    match v.strip_prefix("[[").and_then(|s| s.strip_suffix("]]")) {
        Some(inner) => normalize(inner).unwrap_or_default(),
        None => v.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obsidian_forms() {
        let body = "Seen in [[session-middleware]]; see [[bugs/bug-7f3a91.md|the bug]],\n\
                    ![[diagram#Top]] and [[session-middleware]] again. [[ ]] [[unclosed";
        assert_eq!(
            extract(body),
            ["session-middleware", "bug-7f3a91", "diagram"]
        );
    }

    #[test]
    fn code_is_not_linked() {
        let body = "real [[a]]\n```\nlet x = [[b]];\n```\nand `[[c]]` but [[d]]\n~~~\n[[e]]\n~~~";
        assert_eq!(extract(body), ["a", "d"]);
    }

    #[test]
    fn header_values() {
        assert_eq!(unwrap_link("\"[[x-1]]\"".trim_matches('"')), "x-1");
        assert_eq!(unwrap_link("auth::refresh"), "auth::refresh");
    }
}
