// Materialization of one already-selected original note visibility operation.
// No score selection, ranking, new mint or physical sink is available here.
fn mutate_note_visibility_fragment(fragment: &str) -> String {
    (|| {
        let Ok(document) = roxmltree::Document::parse(fragment) else { return fragment.to_string(); };
        let note = document.root_element();
        if !note.has_tag_name("note") { return fragment.to_string(); }
        if let Some(attribute) = note.attributes().find(|attribute|
            attribute.namespace().is_none() && attribute.name() == "print-object") {
            if attribute.value() != "yes" { return fragment.to_string(); }
            // The full parsed attribute range has no compact value-offset limit.
            // Derive its quoted value locally so long legal spacing stays exact.
            let range = attribute.range();
            let original = &fragment[range.clone()];
            let Some(equal) = original.find('=') else { return fragment.to_string(); };
            let quoted = original[equal + 1..].trim_start();
            let value_start = range.start + original.len() - quoted.len() + 1;
            let mut output = fragment.to_string();
            output.replace_range(value_start..range.end - 1, "no");
            return output;
        }
        // Locate the closing delimiter after all parsed attributes, including
        // quoted values containing '>'. Preserve the original opening spelling.
        let attributes_end = note.attributes().map(|attribute| attribute.range().end)
            .max().unwrap_or(note.range().start + "<note".len());
        let Some(close) = fragment[attributes_end..].find('>') else { return fragment.to_string(); };
        let open_end = attributes_end + close;
        let insertion = if fragment.as_bytes().get(open_end.wrapping_sub(1)) == Some(&b'/') {
            open_end - 1
        } else { open_end };
        let mut output = fragment.to_string();
        output.insert_str(insertion, " print-object=\"no\"");
        output
    })()
}
