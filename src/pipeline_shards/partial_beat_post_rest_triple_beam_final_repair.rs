// E057 materializes only canonical cursor exchanges bound to the input XML.
// A separate read-only detector recognizes cursor splits in committed notation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HumanBoundCursorItemKind {
    Note,
    Forward,
    Backup,
}

#[derive(Debug, Clone)]
struct HumanBoundCursorItem {
    kind: HumanBoundCursorItemKind,
    start: usize,
    end: usize,
    duration: i64,
}

#[derive(Debug, Clone, Default)]
struct PartialBeatLaneSerializationOwnerTransferStats {
    written: bool,
    #[cfg(test)]
    candidate_groups: usize,
}

pub(crate) struct PartialBeatLaneTransferPreservingMint {
    _sealed: (),
}

fn human_bound_cursor_lane_owner_transfer_xml(
    xml: &str,
    exchange_guard: Option<&structural_write_authority::CanonicalFinalChordStructuralPlan>,
) -> (String, PartialBeatLaneSerializationOwnerTransferStats) {
    let Some(guard) = exchange_guard else {
        return (xml.to_owned(), PartialBeatLaneSerializationOwnerTransferStats::default());
    };
    let Ok((_groups, edits)) = guard.cursor_exchange_edits(xml) else {
        return (xml.to_owned(), PartialBeatLaneSerializationOwnerTransferStats::default());
    };
    let mut stats = PartialBeatLaneSerializationOwnerTransferStats {
        written: false,
        #[cfg(test)]
        candidate_groups: _groups,
    };
    if edits.is_empty() {
        return (xml.to_owned(), stats);
    }
    let mut pieces = Vec::with_capacity(edits.len() * 2 + 1);
    let mut end = 0;
    for (start, stop, replacement) in &edits {
        pieces.push(&xml[end..*start]);
        pieces.push(replacement.as_str());
        end = *stop;
    }
    pieces.push(&xml[end..]);
    let post = pieces.concat();
    stats.written = post != xml;
    (post, stats)
}

type HumanBoundCompleteNoteEventKey = (
    usize,
    String,
    String,
    i32,
    i32,
    bool,
    bool,
    bool,
    Option<i32>,
);

fn human_bound_complete_note_event_multiset(
    events: &[RenderXmlNoteEvent],
) -> Vec<HumanBoundCompleteNoteEventKey> {
    let mut keys = events
        .iter()
        .map(|event| {
            (
                event.scope,
                event.staff.clone(),
                event.voice.clone(),
                event.onset,
                event.duration,
                event.is_hidden,
                event.is_rest,
                event.is_chord,
                event.midi,
            )
        })
        .collect::<Vec<_>>();
    keys.sort();
    keys
}

pub(crate) fn e057_existing_event_multiset_projection(xml: &str) -> Option<String> {
    roxmltree::Document::parse(xml).ok()?;
    let (events, cursor_ok) = collect_render_xml_note_events(xml);
    if !cursor_ok {
        return None;
    }
    let keys = human_bound_complete_note_event_multiset(&events);
    Some(format!("{keys:?}"))
}

pub(crate) fn partial_beat_post_rest_lane_split_regression_candidates(
    xml: &str,
) -> Option<usize> {
    parse_musicxml(xml).ok()?;
    let spans = partial_beat_measure_spans(xml);
    let mut inherited_divisions = 1_i64;
    let mut candidates = 0_usize;
    for span in spans {
        let measure_text = &xml[span.start..span.end];
        inherited_divisions = partial_beat_first_i64(measure_text, "divisions")
            .unwrap_or(inherited_divisions)
            .max(1);
        let raw_notes = partial_beat_raw_notes(measure_text);
        let infos = partial_beat_note_infos(measure_text, &raw_notes);
        let cursor_items = human_bound_cursor_items(measure_text, &raw_notes);
        candidates = candidates.saturating_add(count_metric_beam_cursor_splits(
            measure_text, &raw_notes, &infos, &cursor_items, inherited_divisions,
        )?);
    }
    Some(candidates)
}

fn human_bound_cursor_items(
    measure_text: &str,
    raw_notes: &[PartialBeatRawNote],
) -> Vec<HumanBoundCursorItem> {
    let mut items = Vec::new();
    let mut scan = 0_usize;
    loop {
        let next_note = measure_text[scan..].find("<note").map(|p| scan + p);
        let next_backup = measure_text[scan..].find("<backup").map(|p| scan + p);
        let next_forward = measure_text[scan..].find("<forward").map(|p| scan + p);
        let mut next = Vec::new();
        if let Some(pos) = next_note { next.push((pos, HumanBoundCursorItemKind::Note)); }
        if let Some(pos) = next_backup { next.push((pos, HumanBoundCursorItemKind::Backup)); }
        if let Some(pos) = next_forward { next.push((pos, HumanBoundCursorItemKind::Forward)); }
        next.sort_by_key(|(pos, _)| *pos);
        let Some((pos, kind)) = next.first().copied() else { break; };

        match kind {
            HumanBoundCursorItemKind::Note => {
                let Some(raw) = raw_notes.iter().find(|note| note.start == pos) else { break; };
                items.push(HumanBoundCursorItem {
                    kind,
                    start: raw.start,
                    end: raw.end,
                    duration: partial_beat_first_i64(&raw.text, "duration").unwrap_or(0).max(0),
                });
                scan = raw.end;
            }
            HumanBoundCursorItemKind::Forward | HumanBoundCursorItemKind::Backup => {
                let close = if kind == HumanBoundCursorItemKind::Forward {
                    "</forward>"
                } else {
                    "</backup>"
                };
                let Some(end) = measure_text[pos..].find(close).map(|rel| pos + rel + close.len()) else { break; };
                let item_text = &measure_text[pos..end];
                items.push(HumanBoundCursorItem {
                    kind,
                    start: pos,
                    end,
                    duration: partial_beat_first_i64(item_text, "duration").unwrap_or(0).max(0),
                });
                scan = end;
            }
        }
    }
    items
}

fn count_metric_beam_cursor_splits(
    measure: &str,
    raw_notes: &[PartialBeatRawNote],
    infos: &[PartialBeatNoteInfo],
    items: &[HumanBoundCursorItem],
    beat_unit: i64,
) -> Option<usize> {
    if !items.iter().any(|item| item.kind == HumanBoundCursorItemKind::Backup)
        || !items.iter().any(|item| item.kind == HumanBoundCursorItemKind::Forward)
    {
        return Some(0);
    }
    let boundaries = metric_beam_clef_boundaries(measure)?;
    let mut states = BTreeMap::new();
    add_beat_scoped_beam_groups(
        infos, beat_unit, &boundaries, &mut states, &mut BTreeSet::new(),
        &mut BeatScopedBeamPatchStats::default(),
    );

    let mut lanes = BTreeMap::<_, Vec<_>>::new();
    for info in infos.iter().filter(|info| !info.is_chord && info.duration > 0) {
        lanes.entry((&info.staff, &info.voice)).or_default().push(info);
    }
    let mut splits = 0;
    for notes in lanes.values_mut() {
        notes.sort_by_key(|note| (note.start, note.index));
        for (index, pair) in notes.windows(2).enumerate() {
            let [left, right] = pair else { continue };
            if left.start + left.duration != right.start
                || index.checked_sub(1).is_some_and(|prior| {
                    notes[prior].start + notes[prior].duration > left.start
                })
                || notes.get(index + 2).is_some_and(|next| {
                    next.start < right.start + right.duration
                })
            {
                continue;
            }
            let connects = |note: &PartialBeatNoteInfo, permitted: [&str; 2]| {
                states.get(&note.index).is_some_and(|levels| {
                    levels.iter().any(|&(level, state)| level == 1 && permitted.contains(&state))
                })
            };
            if !connects(left, ["begin", "continue"]) || !connects(right, ["continue", "end"]) {
                continue;
            }
            let breaks_beam = |note: &PartialBeatNoteInfo, permitted: [&str; 2]| {
                raw_notes.iter().find(|raw| raw.start == note.index)
                    .and_then(|raw| partial_beat_observed_beam_states(&raw.text))
                    .is_some_and(|levels| levels.iter().any(|(level, state)| {
                        *level == 1 && !permitted.contains(&state.as_str())
                    }))
            };
            if breaks_beam(left, ["begin", "continue"]) || breaks_beam(right, ["continue", "end"]) {
                continue;
            }
            let Some(left_pos) = items.iter().position(|item| item.start == left.index) else { continue };
            let Some(right_pos) = items.iter().position(|item| item.start == right.index) else { continue };
            if let Some([forward, traversal @ ..]) = items.get(left_pos + 1..right_pos)
                && forward.kind == HumanBoundCursorItemKind::Forward
                && forward.duration == right.duration
                && measure[items[left_pos].end..forward.start].trim().is_empty()
                && cursor_forward_has_other_owner(&measure[forward.start..forward.end], left)
                && traversal.iter().any(|item| item.kind == HumanBoundCursorItemKind::Backup)
            {
                splits += 1;
            }
        }
    }
    Some(splits)
}

fn cursor_forward_has_other_owner(forward: &str, note: &PartialBeatNoteInfo) -> bool {
    partial_beat_first_text(forward, "voice").is_some_and(|voice| voice != note.voice)
        || partial_beat_first_text(forward, "staff").is_some_and(|staff| staff != note.staff)
}

#[cfg(test)]
mod partial_beat_post_rest_triple_beam_final_repair_tests {
    use super::*;

    fn split_target_xml(rest_voice: &str, gap_duration: i64) -> String {
        format!(
            "<score-partwise version=\"4.0\"><part id=\"P1\"><measure number=\"99\"><attributes><divisions>96</divisions></attributes><forward><duration>192</duration></forward><note><rest/><duration>24</duration><type>16th</type><voice>{}</voice><staff>2</staff></note><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><note><pitch><step>D</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><forward><duration>{}</duration><voice>1</voice><staff>1</staff></forward><backup><duration>288</duration></backup><forward><duration>264</duration></forward><note><pitch><step>E</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note></measure></part></score-partwise>",
            rest_voice, gap_duration
        )
    }

    fn two_split_targets_same_measure_xml() -> String {
        "<score-partwise version=\"4.0\"><part id=\"P1\"><measure number=\"99\"><attributes><divisions>96</divisions></attributes><forward><duration>192</duration></forward><note><rest/><duration>24</duration><type>16th</type><voice>1</voice><staff>2</staff></note><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><note><pitch><step>D</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><forward><duration>24</duration><voice>1</voice><staff>1</staff></forward><backup><duration>288</duration></backup><forward><duration>264</duration></forward><note><pitch><step>E</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><note><rest/><duration>24</duration><type>16th</type><voice>1</voice><staff>2</staff></note><note><pitch><step>F</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><note><pitch><step>G</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><forward><duration>24</duration><voice>1</voice><staff>1</staff></forward><backup><duration>96</duration></backup><forward><duration>72</duration></forward><note><pitch><step>A</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note></measure></part></score-partwise>".to_string()
    }

    #[test]
    fn detector_sees_split_with_same_or_separate_rest_voice() {
        for voice in ["1", "2"] {
            assert_eq!(
                partial_beat_post_rest_lane_split_regression_candidates(&split_target_xml(voice, 24)),
                Some(1)
            );
        }
    }

    #[test]
    fn detects_metrically_joined_split_with_different_staff_and_note_values() {
        for staff in ["1", "2"] {
            for (duration, value) in [(48, "eighth"), (24, "16th"), (12, "32nd")] {
                let later = format!(
                    "<note><pitch><step>D</step><octave>4</octave></pitch><duration>{duration}</duration><type>{value}</type><voice>7</voice><staff>{staff}</staff></note>"
                );
                let first = later.replace("<step>D</step>", "<step>C</step>");
                let split = format!(
                    "<forward><duration>{duration}</duration><voice>9</voice></forward><backup><duration>{duration}</duration></backup>"
                );
                let xml = format!(
                    "<score-partwise><part id=\"P1\"><measure number=\"synthetic\"><attributes><divisions>96</divisions></attributes><forward><duration>96</duration></forward>{first}{split}{later}</measure></part></score-partwise>"
                );
                assert_eq!(partial_beat_post_rest_lane_split_regression_candidates(&xml), Some(1));
                for separate in [
                    xml.replace(&split, ""),
                    xml.replace("<voice>9</voice>", ""),
                    xml.replace("<voice>9</voice>", "<voice>7</voice>"),
                    xml.replace(&later, &later.replace("<voice>7", "<voice>8")),
                    xml.replace(&later, &later.replace("</note>", "<beam number=\"1\">begin</beam></note>")),
                    xml.replace(&later, &format!(
                        "<attributes><clef number=\"{staff}\"><sign>F</sign><line>4</line></clef></attributes>{later}"
                    )),
                ] {
                    assert_eq!(partial_beat_post_rest_lane_split_regression_candidates(&separate), Some(0));
                }
            }
        }
    }

    #[test]
    fn detector_rejects_wrong_forward_duration() {
        let xml = split_target_xml("1", 48);
        assert_eq!(
            partial_beat_post_rest_lane_split_regression_candidates(&xml),
            Some(0)
        );
    }

    #[test]
    fn independent_measure_candidates_are_duplication_invariant() {
        let first = split_target_xml("1", 24);
        let measure_start = first.find("<measure").expect("first measure");
        let measure_end = first[measure_start..]
            .find("</measure>")
            .map(|offset| measure_start + offset + "</measure>".len())
            .expect("first measure end");
        let second_measure = first[measure_start..measure_end].replace(
            "number=\"99\"",
            "number=\"100\"",
        );
        let xml = first.replace("</part>", &format!("{second_measure}</part>"));
        assert_eq!(
            partial_beat_post_rest_lane_split_regression_candidates(&xml),
            Some(2)
        );
    }

    #[test]
    fn detector_counts_multiple_candidates_inside_one_measure() {
        let xml = two_split_targets_same_measure_xml();
        assert_eq!(
            partial_beat_post_rest_lane_split_regression_candidates(&xml),
            Some(2)
        );
    }
}

fn human_bound_cursor_lane_owner_transfer_xml_with_receipt(
    (xml, exchange_guard): (&str, &structural_write_authority::CanonicalFinalChordStructuralPlan),
) -> ((String, PartialBeatLaneSerializationOwnerTransferStats), Option<crate::master_state_identity::PreservingTransformationReceipt>) {
    let output = human_bound_cursor_lane_owner_transfer_xml(xml, Some(exchange_guard));
    let receipt = crate::master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(PartialBeatLaneTransferPreservingMint { _sealed: () }, xml, &output.0);
    assert!(output.0 == xml || receipt.is_some(), "E057 changed bytes without own receipt");
    (output, receipt)
}
