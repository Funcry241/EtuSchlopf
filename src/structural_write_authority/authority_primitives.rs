//! Private, behavior-preserving primitives shared by structural write authorities.
//!
//! These helpers own no selection or write authority. They only centralize byte binding, XML
//! shape checks and report projection that were previously duplicated across sibling modules.

pub(super) fn fingerprint(text: &str) -> u64 {
    fingerprint_bytes(text.as_bytes())
}

pub(super) fn fingerprint_bytes(bytes: &[u8]) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0100_0000_01b3;
    let mut hash = FNV_OFFSET_BASIS;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

pub(super) fn xml_fragment_parses(fragment: &str) -> bool {
    roxmltree::Document::parse(&format!("<authority-root>{fragment}</authority-root>")).is_ok()
}

pub(super) fn optional_xml_fragment_parses(fragment: &str) -> bool {
    fragment.is_empty() || xml_fragment_parses(fragment)
}

pub(super) fn parsed_exact_note(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.starts_with("<note")
        && trimmed.ends_with("</note>")
        && trimmed.matches("</note>").count() == 1
        && xml_fragment_parses(trimmed)
}

pub(super) fn parsed_exact_rest_note(fragment: &str) -> bool {
    parsed_exact_note(fragment) && fragment.contains("<rest") && !fragment.contains("<pitch>")
}

pub(super) fn syntactic_exact_note(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.starts_with("<note")
        && trimmed.ends_with("</note>")
        && trimmed.matches("</note>").count() == 1
}

pub(super) fn syntactic_exact_rest_note(fragment: &str) -> bool {
    syntactic_exact_note(fragment)
        && fragment.contains("<rest")
        && !fragment.contains("<pitch>")
}

pub(super) fn scalar_lane_value(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && trimmed.chars().all(|character| character.is_ascii_digit())
}

pub(super) fn child_text(fragment: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = fragment.find(&open)? + open.len();
    let end = fragment[start..].find(&close)? + start;
    Some(fragment[start..end].trim().to_string())
}

pub(super) fn child_block(fragment: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = fragment.find(&open)?;
    let end = fragment[start..].find(&close)? + start + close.len();
    Some(fragment[start..end].to_string())
}

pub(super) fn contains_document_boundary(fragment: &str) -> bool {
    ["<score-partwise", "<part-list", "<part ", "<measure", "</measure>"]
        .iter()
        .any(|needle| fragment.contains(needle))
}

pub(super) fn is_measure_tag(text: &str, start: usize) -> bool {
    let next = text.as_bytes().get(start + "<measure".len()).copied();
    matches!(
        next,
        Some(b' ' | b'\t' | b'\n' | b'\r' | b'>')
    )
}

pub(super) fn xml_attr_range(tag: &str, attr: &str) -> Option<(usize, usize)> {
    for quote in ['"', '\''] {
        let pattern = format!("{attr}={quote}");
        if let Some(start) = tag.find(&pattern) {
            let value_start = start + pattern.len();
            let end_relative = tag[value_start..].find(quote)?;
            return Some((start, value_start + end_relative + 1));
        }
    }
    None
}

pub(super) fn yes_no(value: bool) -> &'static str {
    if value {
        "ja"
    } else {
        "nein"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_keeps_the_existing_fnv_contract() {
        assert_eq!(fingerprint(""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fingerprint("a"), 0xaf63_dc4c_8601_ec8c);
        assert_ne!(fingerprint("source-a"), fingerprint("source-b"));
    }

    #[test]
    fn exact_note_variants_preserve_parse_and_shape_boundaries() {
        let note = "<note><rest/><duration>1</duration></note>";
        assert!(parsed_exact_note(note));
        assert!(parsed_exact_rest_note(note));
        assert!(syntactic_exact_note(note));
        assert!(syntactic_exact_rest_note(note));
        assert!(!parsed_exact_note("<note><rest/></note><note></note>"));
        assert!(!parsed_exact_note("<note><rest></note>"));
        assert!(syntactic_exact_note("<note><rest></note>"));
    }

    #[test]
    fn fragment_and_child_helpers_keep_empty_and_trimmed_semantics() {
        assert!(optional_xml_fragment_parses(""));
        assert!(xml_fragment_parses("<voice>2</voice>"));
        assert_eq!(
            child_text("<note><voice> 2 </voice></note>", "voice").as_deref(),
            Some("2")
        );
        assert_eq!(
            child_block("<note><pitch><step>C</step></pitch></note>", "pitch").as_deref(),
            Some("<pitch><step>C</step></pitch>")
        );
        assert!(scalar_lane_value(" 12 "));
        assert!(!scalar_lane_value("1a"));
        assert_eq!(yes_no(true), "ja");
        assert_eq!(yes_no(false), "nein");
        assert!(contains_document_boundary("<measure number=\"\
1\">"));
        assert!(is_measure_tag("<measure number=\"\
1\">", 0));
        assert_eq!(
            xml_attr_range("<note print-object=\"no\">", "print-object"),
            Some((6, 23))
        );
    }
}
