//! Pure pre-coordinate geometry for the three accepted fragment insertion owners.
//! No output diff, XML parse/reconstruction or mutation of the input occurs here.
use crate::master_state_identity::ExactEdit;





pub(crate) fn bound_tie_fragment_insertions(
    source: &str,
    start: bool,
    stop: bool,
) -> Vec<ExactEdit> {
    let ties = format!(
        "{}{}",
        if stop {
            "\n        <tie type=\"stop\"/>"
        } else {
            ""
        },
        if start {
            "\n        <tie type=\"start\"/>"
        } else {
            ""
        }
    );
    let duration_end = source.find("</duration>").map(|p| p + "</duration>".len());
    let note_end = source.rfind("</note>");
    let notation = format!(
        "        <notations>\n{}{}        </notations>\n",
        if stop {
            "          <tied type=\"stop\"/>\n"
        } else {
            ""
        },
        if start {
            "          <tied type=\"start\"/>\n"
        } else {
            ""
        }
    );
    // At coincident pre points the legacy result is ties then notation. Merge
    // those original insertions; do not infer their geometry from the output.
    if let (Some(duration), Some(note)) = (duration_end, note_end)
        && duration == note
        && !ties.is_empty()
    {
        return vec![ExactEdit::bound(note, note, format!("{ties}{notation}"))];
    }
    let mut edits = Vec::new();
    if let Some(position) = duration_end
        && !ties.is_empty()
    {
        edits.push((position, ties));
    }
    if let Some(position) = note_end {
        edits.push((position, notation));
    }
    edits.sort_by_key(|(position, _)| *position);
    edits
        .into_iter()
        .map(|(position, replacement)| ExactEdit::bound(position, position, replacement))
        .collect()
}
