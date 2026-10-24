// Candidate-only beam XML materialization. No source selection, report projection, or filesystem write lives here.

fn raw_note_text(raw_notes: &[PartialBeatRawNote], start: usize) -> String {
    raw_notes
        .iter()
        .find(|raw| raw.start == start)
        .map(|raw| raw.text.clone())
        .unwrap_or_default()
}

fn partial_beat_strip_beam_tags(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut scan = 0_usize;
    while let Some(rel_start) = text[scan..].find("<beam") {
        let start = scan + rel_start;
        out.push_str(&text[scan..start]);
        let Some(close_rel) = text[start..].find("</beam>") else {
            scan = start;
            break;
        };
        scan = start + close_rel + "</beam>".len();
    }
    out.push_str(&text[scan..]);
    out
}

fn partial_beat_insert_beam_text_tags(note_text: &str, states: &[(usize, String)]) -> String {
    if let Some(close_pos) = note_text.rfind("</note>") {
        let insert_pos = ["<notations", "<lyric", "<play", "<listen"]
            .into_iter()
            .filter_map(|marker| note_text.find(marker))
            .min()
            .unwrap_or(close_pos);
        let insert_pos = note_text[..insert_pos]
            .rfind('\n')
            .map_or(insert_pos, |line_break| line_break + 1);
        let mut beams = String::new();
        let mut ordered = states.to_vec();
        ordered.sort_by_key(|(level, _)| *level);
        ordered.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
        for (level, state) in ordered {
            beams.push_str(&format!("    <beam number=\"{level}\">{state}</beam>\n"));
        }
        let mut out = String::with_capacity(note_text.len() + beams.len());
        out.push_str(&note_text[..insert_pos]);
        if !out.ends_with('\n') {
            out.push('\n');
        }
        out.push_str(&beams);
        out.push_str(&note_text[insert_pos..]);
        out
    } else {
        note_text.to_string()
    }
}

fn partial_beat_insert_beam_tags(note_text: &str, states: &[(usize, &'static str)]) -> String {
    let text_states = states
        .iter()
        .map(|(level, state)| (*level, (*state).to_string()))
        .collect::<Vec<_>>();
    partial_beat_insert_beam_text_tags(note_text, &text_states)
}



fn partial_beat_beam_number(open_tag: &str) -> Option<usize> {
    let Some(number_start) = open_tag.find("number=\"") else {
        return Some(1);
    };
    let value_start = number_start + "number=\"".len();
    let value_end = open_tag[value_start..].find('"')? + value_start;
    open_tag[value_start..value_end].trim().parse::<usize>().ok()
}


fn partial_beat_rewrite_owned_beam_levels(
    note_text: &str,
    owned_states: &[(usize, &'static str)],
    max_owned_level: usize,
) -> Option<String> {
    let mut states = partial_beat_observed_beam_states(note_text)?
        .into_iter()
        .filter(|(level, _)| *level > max_owned_level)
        .collect::<Vec<_>>();
    states.extend(
        owned_states
            .iter()
            .map(|(level, state)| (*level, (*state).to_string())),
    );
    states.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    if states.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return None;
    }
    let cleaned = partial_beat_strip_beam_tags(note_text);
    Some(partial_beat_insert_beam_text_tags(&cleaned, &states))
}


pub(crate) struct CommittedCrossStaffBeamPreservingMint { pub(crate) _sealed: () }

#[derive(Default)]
struct ExternalFeedbackBeamClosureStats {
    cross_staff_groups: usize,
    rows_written: usize,
    level1_rows: usize,
    level2_rows: usize,
}


fn close_committed_cross_staff_beam_gestures( xml: &mut String, ) -> (ExternalFeedbackBeamClosureStats, Option<PreservingTransformationReceipt>) {
    let pre = xml.clone();
    let spans = partial_beat_measure_spans(xml);
    if spans.is_empty() {
        return (ExternalFeedbackBeamClosureStats::default(), None);
    }
    let mut out = String::with_capacity(xml.len() + 512);
    let mut last = 0_usize;
    let mut inherited_divisions = 1_i64;
    let mut total = ExternalFeedbackBeamClosureStats::default();
    for span in spans {
        out.push_str(&xml[last..span.start]);
        let measure_text = &xml[span.start..span.end];
        inherited_divisions = partial_beat_first_i64(measure_text, "divisions")
            .unwrap_or(inherited_divisions)
            .max(1);
        let (patched, stats) = close_committed_cross_staff_beam_gestures_in_segment(
            measure_text,
            inherited_divisions,
        );
        total.cross_staff_groups += stats.cross_staff_groups;
        total.rows_written += stats.rows_written;
        total.level1_rows += stats.level1_rows;
        total.level2_rows += stats.level2_rows;
        out.push_str(&patched);
        last = span.end;
    }
    out.push_str(&xml[last..]);
    *xml = out;
    let receipt = crate::master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(
        CommittedCrossStaffBeamPreservingMint { _sealed: () },
        &pre,
        xml,
    );
    assert!(pre == *xml || receipt.is_some(), "E049 changed bytes require preserving receipt");
    (total, receipt)
}

fn close_committed_cross_staff_beam_gestures_in_segment(
    measure_text: &str,
    divisions: i64,
) -> (String, ExternalFeedbackBeamClosureStats) {
    let raw_notes = partial_beat_raw_notes(measure_text);
    let infos = partial_beat_note_infos(measure_text, &raw_notes);
    let beat_unit = divisions.max(1);
    let mut stats = ExternalFeedbackBeamClosureStats::default();
    let mut states: BTreeMap<usize, Vec<(usize, &'static str)>> = BTreeMap::new();
    for mut group in structural_cross_staff_voice_beam_groups(&infos, beat_unit) {
        group.sort_by_key(|info| (info.start, info.index));
        let Some(plan) = structural_metric_beam_hierarchy_plan(&group, 2) else {
            continue;
        };
        if structural_beam_group_needs_owned_plan_normalization(&raw_notes, &group, &plan, 2)
            && extend_beam_states_from_plan(&mut states, &group, &plan)
        {
            stats.cross_staff_groups += 1;
        }
    }
    if states.is_empty() {
        return (measure_text.to_string(), stats);
    }
    let mut rebuilt = String::with_capacity(measure_text.len() + states.len() * 64);
    let mut note_last = 0_usize;
    for raw in &raw_notes {
        rebuilt.push_str(&measure_text[note_last..raw.start]);
        if let Some(state_rows) = states.get(&raw.start) {
            if let Some(normalized) =
                partial_beat_rewrite_owned_beam_levels(&raw.text, state_rows, 2)
            {
                if normalized != raw.text {
                    stats.rows_written += 1;
                    if state_rows.iter().any(|(level, _)| *level == 1) {
                        stats.level1_rows += 1;
                    }
                    if state_rows.iter().any(|(level, _)| *level == 2) {
                        stats.level2_rows += 1;
                    }
                    rebuilt.push_str(&normalized);
                } else {
                    rebuilt.push_str(&raw.text);
                }
            } else {
                rebuilt.push_str(&raw.text);
            }
        } else {
            rebuilt.push_str(&raw.text);
        }
        note_last = raw.end;
    }
    rebuilt.push_str(&measure_text[note_last..]);
    (rebuilt, stats)
}


fn partial_beat_observed_beam_states(note_text: &str) -> Option<Vec<(usize, String)>> {
    let mut states = Vec::new();
    let mut scan = 0_usize;
    while let Some(relative_start) = note_text[scan..].find("<beam") {
        let start = scan + relative_start;
        let open_end = note_text[start..].find('>')? + start;
        let open_tag = &note_text[start..=open_end];
        let content_start = open_end + 1;
        let close = note_text[content_start..].find("</beam>")? + content_start;
        states.push((
            partial_beat_beam_number(open_tag)?,
            note_text[content_start..close].trim().to_ascii_lowercase(),
        ));
        scan = close + "</beam>".len();
    }
    states.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    Some(states)
}

fn structural_beam_group_needs_owned_plan_normalization(
    raw_notes: &[PartialBeatRawNote],
    group: &[&PartialBeatNoteInfo],
    plan: &structural_write_authority::BeamPlan,
    max_owned_level: usize,
) -> bool {
    for (position, info) in group.iter().enumerate() {
        let Some(raw) = raw_notes.iter().find(|raw| raw.start == info.index) else {
            return false;
        };
        let mut desired = plan
            .edits()
            .iter()
            .copied()
            .filter(|edit| edit.event_ordinal() == position)
            .map(|edit| {
                (
                    usize::from(edit.level()),
                    edit.state().as_musicxml_text().to_string(),
                )
            })
            .collect::<Vec<_>>();
        desired.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
        let Some(mut observed) = partial_beat_observed_beam_states(&raw.text) else {
            return true;
        };
        observed.retain(|(level, _)| *level <= max_owned_level);
        if observed != desired {
            return true;
        }
    }
    false
}


#[cfg(test)]
pub(crate) fn ssc_committed_cross_staff_beam_owner_probe(
    source: &str,
) -> (String, Option<PreservingTransformationReceipt>) {
    let mut candidate = source.to_owned();
    let (_, receipt) = close_committed_cross_staff_beam_gestures(&mut candidate);
    (candidate, receipt)
}
