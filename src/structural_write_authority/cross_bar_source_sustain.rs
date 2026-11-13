//! Source-bound sounding continuations at the FinalChord serialization owner.
use super::*;
use std::ops::Range;

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

// Extends a chain that may already end in a tie-stop and contain other
// notations. Keep those original XML bytes in their original order.
fn open_terminal_tie(source: &str) -> io::Result<String> {
    let document = roxmltree::Document::parse(source).map_err(io::Error::other)?;
    let note = document.root_element();
    let duration = note
        .children()
        .find(|n| n.has_tag_name("duration"))
        .ok_or_else(|| invalid("terminal occurrence has no duration"))?;
    let ties = note
        .children()
        .filter(|n| n.has_tag_name("tie"))
        .collect::<Vec<_>>();
    let notations = note
        .children()
        .filter(|n| n.has_tag_name("notations"))
        .collect::<Vec<_>>();
    if !note.has_tag_name("note")
        || ties.len() > 1
        || notations.len() > 1
        || ties.iter().any(|n| n.attribute("type") != Some("stop"))
    {
        return Err(invalid(
            "terminal occurrence has an open or unsupported tie",
        ));
    }
    let graphical = notations
        .iter()
        .flat_map(|n| n.children())
        .filter(|n| n.has_tag_name("tied"))
        .collect::<Vec<_>>();
    if graphical.len() > ties.len()
        || graphical
            .iter()
            .any(|n| n.attribute("type") != Some("stop"))
    {
        return Err(invalid("terminal sound and notation ties disagree"));
    }
    let sound_end = ties.last().unwrap_or(&duration).range().end;
    let (notation_at, notation) = if let Some(tied) = graphical.last() {
        (tied.range().end, "<tied type=\"start\"/>")
    } else if let Some(existing) = notations.last() {
        let start = existing
            .children()
            .find(|n| n.is_element())
            .ok_or_else(|| invalid("empty terminal notations"))?
            .range()
            .start;
        (start, "<tied type=\"start\"/>")
    } else {
        let close = source
            .rfind("</note>")
            .ok_or_else(|| invalid("missing terminal note close"))?;
        (close, "<notations><tied type=\"start\"/></notations>")
    };
    final_chord_project_ranges(
        source,
        vec![
            (sound_end, sound_end, "<tie type=\"start\"/>".to_string()),
            (notation_at, notation_at, notation.to_string()),
        ],
        "terminal tie edits overlap",
    )
}

fn exact_value(duration: i64, divisions: i64) -> Option<(&'static str, usize)> {
    let (kind, dots) = final_chord_exact_type_and_dots(duration, divisions)?;
    let divisor = match kind {
        "whole" => 1,
        "half" => 2,
        "quarter" => 4,
        "eighth" => 8,
        "16th" => 16,
        "32nd" => 32,
        "64th" => 64,
        _ => return None,
    };
    let dot_denominator = 1i128 << dots;
    (i128::from(duration) * divisor * dot_denominator
        == i128::from(divisions) * 4 * (2 * dot_denominator - 1))
        .then_some((kind, dots))
}

fn continuation_note(
    source: &str,
    duration: i64,
    divisions: i64,
    chord: bool,
    continues: bool,
) -> io::Result<String> {
    let doc = roxmltree::Document::parse(source).map_err(io::Error::other)?;
    let note = doc.root_element();
    let field = |tag| {
        note.children()
            .find(|n| n.has_tag_name(tag))
            .map(|n| &source[n.range()])
            .ok_or_else(|| invalid("terminal occurrence lacks a bound field"))
    };
    let pitch = field("pitch")?;
    let voice = field("voice")?;
    let staff = field("staff")?;
    let (kind, dots) = exact_value(duration, divisions)
        .ok_or_else(|| invalid("continuation duration is not exactly representable"))?;
    let chord = if chord { "<chord/>" } else { "" };
    let dots = "<dot/>".repeat(dots);
    let plain = format!(
        "<note>{chord}{pitch}<duration>{duration}</duration>{voice}<type>{kind}</type>{dots}{staff}</note>"
    );
    final_chord_add_tie_markup(&plain, continues, true)
        .ok_or_else(|| invalid("continuation tie markup refused"))
}

#[derive(Debug, PartialEq, Eq)]
struct NewOccurrence {
    source: usize,
    attack: usize,
    // Pre-edit splice start and relative range in the emitted bytes.
    splice: usize,
    replacement: Range<usize>,
}

#[derive(Debug)]
struct RestProjection {
    edits: Vec<FinalChordEncodingEdit>,
    new_occurrences: Vec<NewOccurrence>,
}

fn rest_replacement(
    xml: &str,
    terminals: &[(&FinalChordEncodingNote, usize, usize)],
    origin: &FinalChordBindingMeasure,
    target: &FinalChordBindingMeasure,
    rest_index: usize,
    length: i64,
    continues: bool,
) -> io::Result<RestProjection> {
    let rest = target
        .notes
        .get(rest_index)
        .ok_or_else(|| invalid("unknown rest"))?;
    if terminals.is_empty()
        || !rest.rest_like
        || rest.chord_member
        || rest.onset != 0
        || length <= 0
        || length > rest.duration
        || target
            .notes
            .iter()
            .any(|n| n.anchor_index == Some(rest_index))
        || target.notes.iter().any(|n| {
            n.range != rest.range && n.voice == rest.voice && n.onset >= 0 && n.onset < length
        })
    {
        return Err(invalid("rest is not an independent free cursor carrier"));
    }
    if xml
        .get(rest.range.clone())
        .is_none_or(|raw| raw.contains("<beam"))
    {
        return Err(invalid("rest has an independent beam obligation"));
    }
    let mut source_ids = std::collections::BTreeSet::new();
    let mut attack_ids = std::collections::BTreeSet::new();
    let mut terminal_ranges = std::collections::BTreeSet::new();
    let mut pitches = std::collections::BTreeSet::new();
    let mut edits = Vec::new();
    let mut replacement = String::new();
    let mut new_occurrences = Vec::new();
    let first = terminals[0].0;
    for (index, &(terminal, source, attack)) in terminals.iter().enumerate() {
        if !source_ids.insert(source)
            || !attack_ids.insert(attack)
            || !terminal_ranges.insert(terminal.range.start)
            || !terminal.pitch_midi.is_some_and(|p| pitches.insert(p))
            || terminal.staff != rest.staff
            || terminal.voice != rest.voice
            || origin
                .notes
                .get(terminal.raw_note_index)
                .map(|note| &note.range)
                != Some(&terminal.range)
            || terminal.onset.checked_add(terminal.duration) != Some(origin.expected_duration)
            || terminal.measure.checked_add(1) != Some(target.measure)
            || terminal.measure != first.measure
            || terminal.onset != first.onset
            || terminal.duration != first.duration
            || terminal.chord_member != (index != 0)
            || (index != 0 && terminal.anchor_index != Some(first.raw_note_index))
            || terminal.tie_start
        {
            return Err(invalid(
                "terminal cohort has conflicting identity or cursor",
            ));
        }
        let raw = xml
            .get(terminal.range.clone())
            .ok_or_else(|| invalid("stale terminal range"))?;
        if !final_chord_note_fragment_is_visible(raw) {
            return Err(invalid("terminal occurrence is hidden"));
        }
        final_chord_push_edit(
            &mut edits,
            terminal.range.start,
            terminal.range.end,
            open_terminal_tie(raw)?,
        );
        let start = replacement.len();
        replacement.push_str(&continuation_note(
            raw,
            length,
            target.divisions,
            index != 0,
            continues,
        )?);
        new_occurrences.push(NewOccurrence {
            source,
            attack,
            splice: rest.range.start,
            replacement: start..replacement.len(),
        });
    }
    if length < rest.duration {
        exact_value(rest.duration - length, target.divisions)
            .ok_or_else(|| invalid("rest remainder is not exactly representable"))?;
        let raw = xml
            .get(rest.range.clone())
            .ok_or_else(|| invalid("stale rest range"))?;
        replacement.push_str(&final_chord_visible_rest_fragment_from_source(
            raw,
            rest.duration - length,
            target.divisions,
        )?);
    }
    final_chord_push_edit(&mut edits, rest.range.start, rest.range.end, replacement);
    Ok(RestProjection {
        edits,
        new_occurrences,
    })
}

// Prove the physical cursor before using the measure-close insertion point.
// No assumption about how many other voices are serialized in the measure.
fn validate_vacant_cursor(
    xml: &str,
    bar: &FinalChordBindingMeasure,
    voice: &str,
) -> io::Result<()> {
    let doc = validate_final_chord_musicxml(xml)?;
    let mut measures = doc
        .root_element()
        .children()
        .filter(|n| n.has_tag_name("part"))
        .flat_map(|part| part.children().filter(|n| n.has_tag_name("measure")))
        .filter(|measure| measure.range().end == bar.measure_close + "</measure>".len());
    let measure = measures
        .next()
        .ok_or_else(|| invalid("unbound vacant measure close"))?;
    if measures.next().is_some() || bar.notes.iter().any(|note| note.voice == voice) {
        return Err(invalid("voice is not vacant at the bound measure"));
    }
    let mut cursor = 0i64;
    for item in measure.children().filter(|n| n.is_element()) {
        let tag = item.tag_name().name();
        if tag == "forward"
            && final_chord_direct_child_text(item, "voice").as_deref() == Some(voice)
        {
            return Err(invalid("vacant voice already has a bound cursor carrier"));
        }
        let advances = tag == "forward"
            || (tag == "note"
                && !final_chord_direct_has_child(item, "chord")
                && !final_chord_direct_has_child(item, "grace"));
        if advances || tag == "backup" {
            let duration = final_chord_direct_child_i64(item, "duration")
                .filter(|d| *d > 0)
                .ok_or_else(|| invalid("unbound cursor duration"))?;
            cursor = if tag == "backup" {
                cursor.checked_sub(duration)
            } else {
                cursor.checked_add(duration)
            }
            .filter(|position| (0..=bar.expected_duration).contains(position))
            .ok_or_else(|| invalid("cursor left the measure"))?;
        }
    }
    if cursor != bar.expected_duration {
        return Err(invalid("vacant insertion requires a closed prior cursor"));
    }
    Ok(())
}

// Subsequent bars use a separate proved vacant voice. Every planned slice
// must be proved before a single combined edit script is returned.
fn rest_then_vacant(
    xml: &str,
    terminals: &[(&FinalChordEncodingNote, usize, usize)],
    origin: &FinalChordBindingMeasure,
    first: (&FinalChordBindingMeasure, usize),
    remaining: &[(&FinalChordBindingMeasure, i64)],
) -> io::Result<RestProjection> {
    if remaining.is_empty() {
        return Err(invalid("vacant continuation has no target"));
    }
    let (first_bar, rest) = first;
    let mut projection = rest_replacement(
        xml,
        terminals,
        origin,
        first_bar,
        rest,
        first_bar.expected_duration,
        true,
    )?;
    let lane = terminals[0].0;
    let mut previous = first_bar.measure;
    for (index, &(bar, length)) in remaining.iter().enumerate() {
        let continues = index + 1 < remaining.len();
        if previous.checked_add(1) != Some(bar.measure)
            || length <= 0
            || length > bar.expected_duration
            || (continues && length != bar.expected_duration)
        {
            return Err(invalid(
                "vacant continuation has a gap or an invalid release",
            ));
        }
        validate_vacant_cursor(xml, bar, &lane.voice)?;
        let mut replacement = format!(
            "<backup><duration>{}</duration></backup>",
            bar.expected_duration
        );
        for (member, &(terminal, source, attack)) in terminals.iter().enumerate() {
            let original = xml
                .get(terminal.range.clone())
                .ok_or_else(|| invalid("stale terminal range"))?;
            let start = replacement.len();
            replacement.push_str(&continuation_note(
                original,
                length,
                bar.divisions,
                member != 0,
                continues,
            )?);
            projection.new_occurrences.push(NewOccurrence {
                source,
                attack,
                splice: bar.measure_close,
                replacement: start..replacement.len(),
            });
        }
        if length < bar.expected_duration {
            replacement.push_str(&final_chord_staff_voice_forward_fragment(
                bar.expected_duration - length,
                &lane.staff,
                &lane.voice,
            )?);
        }
        final_chord_push_edit(
            &mut projection.edits,
            bar.measure_close,
            bar.measure_close,
            replacement,
        );
        previous = bar.measure;
    }
    Ok(projection)
}

fn written_fragment_ranges(projection: &RestProjection) -> io::Result<Vec<WrittenFragment>> {
    let mut edits = projection.edits.iter().collect::<Vec<_>>();
    edits.sort_by_key(|edit| edit.start);
    let mut delta = 0;
    let mut previous = 0;
    let mut ranges = Vec::new();
    for edit in edits {
        if edit.start < previous || edit.end < edit.start {
            return Err(invalid("fragment receipt edit geometry overlaps"));
        }
        let start = final_chord_shift_projection_offset(edit.start, delta)?;
        for row in projection
            .new_occurrences
            .iter()
            .filter(|row| row.splice == edit.start)
        {
            if edit.replacement.get(row.replacement.clone()).is_none() {
                return Err(invalid("fragment receipt is outside the emitted bytes"));
            }
            ranges.push((
                (row.source, row.attack),
                start + row.replacement.start..start + row.replacement.end,
            ));
        }
        delta += edit.replacement.len() as i128 - (edit.end - edit.start) as i128;
        previous = edit.end;
    }
    if ranges.len() != projection.new_occurrences.len() {
        return Err(invalid("fragment receipt has incomplete edit coverage"));
    }
    Ok(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;
    const ORIGIN: usize = 12;

    fn fixture(rest: &str) -> String {
        fixture_at(rest, ORIGIN)
    }

    fn fixture_at(rest: &str, first: usize) -> String {
        let second = first + 1;
        format!(
            r#"<score-partwise><part id="P1"><measure number="{first}">
    <attributes><divisions>24</divisions><time><beats>3</beats><beat-type>4</beat-type></time></attributes>
    <note><rest/><duration>24</duration><voice>3</voice><type>quarter</type><staff>1</staff></note>
    <note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><tie type="start"/><voice>3</voice><type>quarter</type><staff>1</staff><notations><tied type="start"/></notations></note>
    <note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><tie type="stop"/><voice>3</voice><type>quarter</type><staff>1</staff><notations><tied type="stop"/><articulations><tenuto/></articulations></notations></note>
    <note><chord/><pitch><step>F</step><octave>4</octave></pitch><duration>24</duration><voice>3</voice><type>quarter</type><staff>1</staff></note>
    </measure><measure number="{second}">{rest}</measure></part></score-partwise>"#
        )
    }

    const REST: &str = r#"<note><rest measure="yes"/><duration>72</duration><voice>3</voice><type>half</type><dot/><staff>1</staff></note>"#;

    #[test]
    fn owner_replays_two_attacks_and_the_remaining_pause_as_one_exact_edit_script() {
        let xml = fixture(REST);
        let pre = final_chord_parse_binding_measures(&xml).unwrap();
        let notes = &pre[0].notes;
        let trial = rest_replacement(
            &xml,
            &[(&notes[2], 12, 31), (&notes[3], 15, 38)],
            &pre[0],
            &pre[1],
            0,
            24,
            false,
        )
        .unwrap();
        assert_eq!(
            trial
                .new_occurrences
                .iter()
                .map(|row| (row.source, row.attack))
                .collect::<Vec<_>>(),
            [(12, 31), (15, 38)]
        );
        for row in &trial.new_occurrences {
            assert!(
                trial.edits.last().unwrap().replacement[row.replacement.clone()]
                    .contains("<tie type=\"stop\"/>")
            );
        }
        let plan = final_chord_finalize_bound_plan(&xml, trial.edits, vec![], false).unwrap();
        let (post, receipt) = project_final_chord_encoding_plan(&xml, &plan);
        let post = post.unwrap();
        let receipt = receipt.unwrap();
        assert_eq!(
            receipt.realize_post_document(&xml).as_deref(),
            Some(post.as_str())
        );
        let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
        receipts.push_exact(Some(receipt));
        receipts
            .discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        let after = final_chord_parse_binding_measures(&post).unwrap();
        assert!(after[0].notes[2].tie_start && after[0].notes[2].tie_stop);
        assert!(post.contains("<articulations><tenuto/></articulations>"));
        assert!(after[1].notes[0].tie_stop && !after[1].notes[0].tie_start);
        assert!(after[1].notes[1].tie_stop && after[1].notes[1].chord_member);
        assert_eq!(after[1].notes[2].onset, 24);
        assert_eq!(after[1].notes[2].duration, 48);
        assert!(after[1].notes[2].rest_like);
        let attacks = |score: &[FinalChordBindingMeasure]| {
            score
                .iter()
                .flat_map(|bar| bar.notes.iter())
                .filter_map(|note| {
                    note.pitch_midi.filter(|_| !note.tie_stop).map(|pitch| {
                        (
                            note.measure,
                            note.onset,
                            note.staff.clone(),
                            note.voice.clone(),
                            pitch,
                        )
                    })
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(attacks(&pre), attacks(&after));
    }

    #[test]
    fn owner_refuses_a_rest_that_already_anchors_a_chord_member() {
        let chord = r#"<note><chord/><pitch><step>D</step><octave>4</octave></pitch><duration>72</duration><voice>3</voice><staff>1</staff></note>"#;
        let xml = fixture(&format!("{REST}{chord}"));
        let pre = final_chord_parse_binding_measures(&xml).unwrap();
        assert!(
            rest_replacement(
                &xml,
                &[(&pre[0].notes[2], 12, 31)],
                &pre[0],
                &pre[1],
                0,
                24,
                false
            )
            .is_err()
        );
    }

    #[test]
    fn owner_rejects_an_overlap_in_the_same_voice_and_preserves_an_independent_voice() {
        for (label, allowed) in [("3", false), ("8", true)] {
            let other = format!(
                r#"<note><pitch><step>C</step><octave>5</octave></pitch><duration>72</duration><voice>{label}</voice><type>half</type><dot/><staff>1</staff></note>"#
            );
            let xml = fixture(&format!(
                "{REST}<backup><duration>72</duration></backup>{other}"
            ));
            let pre = final_chord_parse_binding_measures(&xml).unwrap();
            let projection = rest_replacement(
                &xml,
                &[(&pre[0].notes[2], 12, 31)],
                &pre[0],
                &pre[1],
                0,
                24,
                false,
            );
            assert_eq!(projection.is_ok(), allowed);
            if let Ok(projection) = projection {
                let edits = projection
                    .edits
                    .into_iter()
                    .map(|edit| (edit.start, edit.end, edit.replacement))
                    .collect();
                let post = final_chord_project_ranges(&xml, edits, "trial edit overlap").unwrap();
                let after = final_chord_parse_binding_measures(&post).unwrap();
                assert_eq!(after[1].notes[2].onset, 0);
                assert_eq!(&post[after[1].notes[2].range.clone()], other);
            }
        }
    }

    #[test]
    fn owner_carries_the_same_attacks_through_rest_and_vacant_voice_in_one_exact_script() {
        let upper = r#"<note><pitch><step>C</step><octave>5</octave></pitch><duration>72</duration><voice>8</voice><type>half</type><dot/><staff>1</staff></note>"#;
        let xml = fixture(REST).replace(
            "</part>",
            &format!(
                r#"<measure number="{tail}">{upper}</measure></part>"#,
                tail = ORIGIN + 2
            ),
        );
        let before = final_chord_parse_binding_measures(&xml).unwrap();
        let notes = &before[0].notes;
        let projection = rest_then_vacant(
            &xml,
            &[(&notes[2], 12, 31), (&notes[3], 15, 38)],
            &before[0],
            (&before[1], 0),
            &[(&before[2], 24)],
        )
        .unwrap();
        let ranges = written_fragment_ranges(&projection).unwrap();
        let plan = final_chord_finalize_bound_plan(&xml, projection.edits, vec![], false).unwrap();
        let (post, receipt) = project_final_chord_encoding_plan(&xml, &plan);
        let post = post.unwrap();
        let receipt = receipt.unwrap();
        assert_eq!(
            receipt.realize_post_document(&xml).as_deref(),
            Some(post.as_str())
        );
        let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
        receipts.push_exact(Some(receipt));
        receipts
            .discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        let after = final_chord_parse_binding_measures(&post).unwrap();
        let rows = [
            &after[1].notes[0],
            &after[1].notes[1],
            &after[2].notes[1],
            &after[2].notes[2],
        ];
        for (((source, attack), range), note) in ranges.into_iter().zip(rows) {
            let pair = if note.pitch_midi == Some(62) {
                (12, 31)
            } else {
                (15, 38)
            };
            assert_eq!((source, attack), pair);
            assert_eq!(range, note.range);
            assert!(note.tie_stop);
            assert_eq!(note.tie_start, note.measure == ORIGIN + 1);
            assert_eq!(note.onset, 0);
        }
        assert_eq!(&post[after[2].notes[0].range.clone()], upper);
        assert!(post.contains(
            "<forward><duration>48</duration><voice>3</voice><staff>1</staff></forward>"
        ));
    }

    #[test]
    fn owner_refuses_the_whole_chain_if_the_later_lane_is_occupied_or_cursor_is_open() {
        for (voice, duration) in [(3, 72), (8, 60)] {
            let tail = format!(
                "<measure number=\"{tail}\"><note><rest/><duration>{duration}</duration><voice>{voice}</voice><staff>1</staff></note></measure></part>",
                tail = ORIGIN + 2
            );
            let xml = fixture(REST).replace("</part>", &tail);
            let before = final_chord_parse_binding_measures(&xml).unwrap();
            assert!(
                rest_then_vacant(
                    &xml,
                    &[(&before[0].notes[2], 12, 31)],
                    &before[0],
                    (&before[1], 0),
                    &[(&before[2], 24)]
                )
                .is_err()
            );
        }
    }

    #[test]
    fn source_timed_projection_carries_generated_occurrences_through_layout() {
        use crate::midi::source_time::{MeterChange, SourceTimeDomain};

        let origin = 1;
        let xml = fixture_at(REST, origin);
        let meter = MetricTimeline::from_midi(
            SourceTimeDomain {
                source_ppq: 24,
                divisions: 24,
            },
            &[MeterChange {
                tick: 0,
                beats: 3,
                beat_type: 4,
            }],
            144,
        )
        .unwrap();
        let time = SustainInputs {
            metric: meter,
            source: vec![Some(24..96), Some(48..96)].into(),
        };
        let rows = [(0, 0, 0), (0, 1, 1), (1, 2, 0)].map(|(source, ordinal, fragment_ordinal)| {
            FinalChordTransactionOutputOccurrenceWitness {
                fragment_id: ordinal + 1,
                source_event_id: source,
                attack_id: source,
                materialization_kind: FinalChordProjectionMaterializationKind::SurvivingSourceNote,
                fragment_ordinal,
                measure: origin,
                pitched_ordinal: ordinal,
            }
        });
        let plan = plan_sustain(&xml, &rows, &time).unwrap().unwrap();
        let ranges = written_fragment_ranges(&plan).unwrap();
        let edits = plan
            .edits
            .into_iter()
            .map(|edit| (edit.start, edit.end, edit.replacement))
            .collect();
        let written = final_chord_project_ranges(&xml, edits, "test edit overlap").unwrap();
        let occurrences = bind_written_sustain(&xml, &written, &rows, ranges, &time).unwrap();
        assert_eq!(occurrences.len(), rows.len() + 2);
        let proof = SustainProof {
            original: Arc::from(xml),
            written: Arc::from(written.as_str()),
            occurrences: occurrences.into(),
        };
        let layout = written.replace("</measure>", "</measure>\n");
        proof.verify(&layout).unwrap();
        let changed = layout.replace("<voice>3</voice>", "<voice>9</voice>");
        assert!(proof.verify(&changed).is_err());
    }

    #[test]
    fn accepts_a_terminal_sound_only_stop_and_preserves_it() {
        let source = r#"<note><pitch><step>E</step><octave>4</octave></pitch><duration>24</duration><tie type="stop"/><voice>7</voice><staff>1</staff></note>"#;
        let written = open_terminal_tie(source).unwrap();
        let document = roxmltree::Document::parse(&written).unwrap();
        let note = document.root_element();
        let sound = note
            .children()
            .filter(|n| n.has_tag_name("tie"))
            .map(|n| n.attribute("type").unwrap())
            .collect::<Vec<_>>();
        let graphic = note
            .descendants()
            .filter(|n| n.has_tag_name("tied"))
            .map(|n| n.attribute("type").unwrap())
            .collect::<Vec<_>>();
        assert_eq!(sound, ["stop", "start"]);
        assert_eq!(graphic, ["start"]);
    }
}

use crate::midi::notation_time::NotationTimePlan;
use crate::midi::source_time::MetricTimeline;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SustainInputs {
    metric: MetricTimeline,
    source: Arc<[Option<Range<i64>>]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SustainProof {
    original: Arc<str>,
    written: Arc<str>,
    occurrences: Arc<[FinalChordTransactionOutputOccurrenceWitness]>,
}

impl SustainProof {
    pub(super) fn verify(&self, xml: &str) -> io::Result<()> {
        if self.written.as_ref() != xml {
            final_chord_verify_pitched_sequence_transition(&self.written, xml, false)?;
        }
        Ok(())
    }

    pub(super) fn original_xml(&self) -> &str {
        &self.original
    }

    pub(super) fn occurrences(&self) -> &[FinalChordTransactionOutputOccurrenceWitness] {
        &self.occurrences
    }
}

impl FinalPdfSourceOccurrenceContinuation {
    pub(crate) fn with_source_sustain_inputs(&self, time: &NotationTimePlan) -> Self {
        let mut result = self.clone();
        if time.metric.divisions == crate::source_event::CANONICAL_DIVISIONS {
            let source = (0..time.events.len())
                .map(|ordinal| time.written_sustain_interval(ordinal).ok())
                .collect::<Vec<_>>();
            result.sustain_inputs = Some(SustainInputs {
                metric: time.metric.clone(),
                source: source.into(),
            });
        }
        result
    }

    pub(crate) fn project_source_sustain(
        &self,
        input: &Self,
        canonical: &CanonicalFinalChordStructuralPlan,
        voice_plan: &FrozenSerializedVoicePlan,
        omissions: &BTreeSet<RenderPrunedFinalChordOmissionKey>,
    ) -> io::Result<Option<(Self, ExactTransformationReceipt)>> {
        let Some(time) = &input.sustain_inputs else {
            return Ok(None);
        };
        let original = self.xml();
        verify_final_pdf_source_occurrence_continuation(
            original,
            canonical,
            voice_plan,
            Some(omissions),
            self,
        )?;
        let Some(projection) = plan_sustain(original, &self.receipt.occurrences, time)? else {
            return Ok(None);
        };
        let ranges = written_fragment_ranges(&projection)?;
        let plan = final_chord_finalize_bound_plan(original, projection.edits, vec![], false)?;
        let (post, receipt) = project_final_chord_encoding_plan(original, &plan);
        let mut pending = crate::master_state_identity::RenderTransitionReceipts::new();
        // Finish all fallible origin readback before offering the exact write.
        let result = post.and_then(|post| {
            let occurrences =
                bind_written_sustain(original, &post, &self.receipt.occurrences, ranges, time)?;
            let mut next = self.clone();
            next.beam_surface = current_occurrence_beam_surface(&post)?;
            next.sustain_proof = Some(SustainProof {
                original: Arc::from(original),
                written: Arc::from(post.as_str()),
                occurrences: occurrences.into(),
            });
            next.xml = post;
            verify_final_pdf_source_occurrence_continuation(
                next.xml(),
                canonical,
                voice_plan,
                Some(omissions),
                &next,
            )?;
            Ok(next)
        });
        match (result, receipt) {
            (Ok(next), Some(mut receipt)) => {
                let proof = next
                    .sustain_proof
                    .as_ref()
                    .expect("proved owner projection");
                let captured =
                    capture_sustain_execution(proof, &self.receipt.occurrences, &receipt);
                if let Err(error) = captured {
                    pending.push_exact(Some(receipt));
                    pending.discard_unadopted(
                        crate::master_state_identity::RenderDiscardReason::LosingCandidate,
                    );
                    return Err(error);
                }
                receipt.record_final_chord_capture(captured.map_err(anyhow::Error::from));
                eprintln!(
                    "ETUSCHLOPF: CrossBarSourceSustain: fragments={} authority=FinalChord",
                    next.current_occurrences().len() - self.receipt.occurrences.len()
                );
                Ok(Some((next, receipt)))
            }
            (result, receipt) => {
                pending.push_exact(receipt);
                pending.discard_unadopted(
                    crate::master_state_identity::RenderDiscardReason::LosingCandidate,
                );
                result.map(|_| None)
            }
        }
    }
}

fn pitched(bar: &FinalChordBindingMeasure) -> impl Iterator<Item = &FinalChordEncodingNote> {
    bar.notes
        .iter()
        .filter(|note| !note.rest_like && note.pitch_midi.is_some())
}

type Identity = (usize, usize);
type WrittenFragment = (Identity, Range<usize>);
type Cohort<'a> = Vec<(&'a FinalChordEncodingNote, usize, usize)>;

fn plan_sustain(
    xml: &str,
    rows: &[FinalChordTransactionOutputOccurrenceWitness],
    time: &SustainInputs,
) -> io::Result<Option<RestProjection>> {
    let doc = validate_final_chord_musicxml(xml)?;
    if doc
        .root_element()
        .children()
        .filter(|n| n.has_tag_name("part"))
        .count()
        != 1
    {
        return Ok(None);
    }
    let bars = final_chord_parse_binding_measures(xml)?;
    if bars.len() != time.metric.len()
        || bars.iter().enumerate().any(|(index, bar)| {
            bar.measure != index + 1
                || bar.divisions != time.metric.divisions
                || time
                    .metric
                    .measure(index)
                    .is_none_or(|m| m.end - m.start != bar.expected_duration)
        })
    {
        return Ok(None);
    }
    let mut chains = BTreeMap::<Identity, Vec<&FinalChordEncodingNote>>::new();
    for row in rows {
        let note = final_chord_transaction_output_occurrence_note(&bars, row)?;
        chains
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(note);
    }

    let mut groups = BTreeMap::<(usize, usize, i64), Cohort<'_>>::new();
    for ((source, attack), mut notes) in chains {
        let Some(interval) = time.source.get(source).and_then(Option::as_ref) else {
            continue;
        };
        notes.sort_by_key(|note| (note.measure, note.onset));
        let [first, ..] = notes.as_slice() else {
            continue;
        };
        let terminal = notes[notes.len() - 1];
        let bar = time
            .metric
            .measure(first.measure - 1)
            .ok_or_else(|| invalid("unbound source measure"))?;
        if interval.start != bar.start + first.onset
            || interval.end <= bar.end
            || terminal.measure != first.measure
            || terminal.onset + terminal.duration != bars[first.measure - 1].expected_duration
            || terminal.tie_start
            || first.tie_stop
            || notes.iter().any(|note| {
                note.duration <= 0
                    || note.staff != first.staff
                    || note.voice != first.voice
                    || note.pitch_midi != first.pitch_midi
                    || !xml
                        .get(note.range.clone())
                        .is_some_and(final_chord_note_fragment_is_visible)
            })
            || notes.windows(2).any(|pair| {
                pair[0].onset + pair[0].duration != pair[1].onset
                    || !pair[0].tie_start
                    || !pair[1].tie_stop
            })
        {
            continue;
        }
        let anchor = terminal.anchor_index.unwrap_or(terminal.raw_note_index);
        groups
            .entry((terminal.measure, anchor, interval.end))
            .or_default()
            .push((terminal, source, attack));
    }

    let mut proposals = Vec::new();
    for ((measure, _, release), mut group) in groups {
        group.sort_by_key(|(note, _, _)| note.raw_note_index);
        let Some(first_target) = bars.get(measure) else {
            continue;
        };
        let first = group[0].0;
        let rest_indices = first_target
            .notes
            .iter()
            .enumerate()
            .filter_map(|(index, note)| {
                (note.rest_like
                    && note.onset == 0
                    && note.staff == first.staff
                    && note.voice == first.voice)
                    .then_some(index)
            })
            .collect::<Vec<_>>();
        let [rest] = rest_indices.as_slice() else {
            continue;
        };
        let target_metric = time
            .metric
            .measure(measure)
            .ok_or_else(|| invalid("unbound target measure"))?;
        let proposal = if release <= target_metric.end {
            rest_replacement(
                xml,
                &group,
                &bars[measure - 1],
                first_target,
                *rest,
                release - target_metric.start,
                false,
            )
        } else {
            let Some(last) = time.metric.ordinal_at(release - 1) else {
                continue;
            };
            let remaining = (measure + 1..=last)
                .map(|index| {
                    let metric = time
                        .metric
                        .measure(index)
                        .ok_or_else(|| invalid("unbound continuation measure"))?;
                    Ok((&bars[index], release.min(metric.end) - metric.start))
                })
                .collect::<io::Result<Vec<_>>>()?;
            rest_then_vacant(
                xml,
                &group,
                &bars[measure - 1],
                (first_target, *rest),
                &remaining,
            )
        };
        if let Ok(proposal) = proposal {
            proposals.push(proposal);
        }
    }
    let mut claims = BTreeMap::<usize, usize>::new();
    for proposal in &proposals {
        for edit in &proposal.edits {
            *claims.entry(edit.start).or_default() += 1;
        }
    }
    let mut selected = RestProjection {
        edits: Vec::new(),
        new_occurrences: Vec::new(),
    };
    for proposal in proposals {
        if proposal.edits.iter().all(|edit| claims[&edit.start] == 1) {
            selected.edits.extend(proposal.edits);
            selected.new_occurrences.extend(proposal.new_occurrences);
        }
    }
    Ok((!selected.edits.is_empty()).then_some(selected))
}

fn bind_written_sustain(
    original: &str,
    post: &str,
    rows: &[FinalChordTransactionOutputOccurrenceWitness],
    ranges: Vec<WrittenFragment>,
    time: &SustainInputs,
) -> io::Result<Vec<FinalChordTransactionOutputOccurrenceWitness>> {
    let before = final_chord_parse_binding_measures(original)?;
    let after = final_chord_parse_binding_measures(post)?;
    let generated = ranges
        .iter()
        .map(|(id, range)| (range.start, (*id, range.end)))
        .collect::<BTreeMap<_, _>>();
    let old = rows
        .iter()
        .map(|row| ((row.measure, row.pitched_ordinal), row))
        .collect::<BTreeMap<_, _>>();
    if before.len() != after.len() || generated.len() != ranges.len() || old.len() != rows.len() {
        return Err(invalid(
            "sustain projection changed the bound measure or occurrence domain",
        ));
    }
    let terminal = ranges.iter().map(|(id, _)| *id).collect::<BTreeSet<_>>();
    let mut bound = Vec::new();
    let mut fresh = rows.iter().map(|row| row.fragment_id).max().unwrap_or(0);
    let mut extended = BTreeSet::new();
    for (base, output) in before.iter().zip(&after) {
        if base.measure != output.measure || base.expected_duration != output.expected_duration {
            return Err(invalid("sustain projection changed measure identity"));
        }
        let mut preceding = pitched(base).enumerate();
        for (ordinal, note) in pitched(output).enumerate() {
            let row = if let Some(&((source_event_id, attack_id), end)) =
                generated.get(&note.range.start)
            {
                if note.range.end != end || !note.tie_stop || note.duration <= 0 {
                    return Err(invalid(
                        "sustain projection did not emit a tied continuation",
                    ));
                }
                fresh = fresh
                    .checked_add(1)
                    .ok_or_else(|| invalid("sustain fragment identity overflow"))?;
                FinalChordTransactionOutputOccurrenceWitness {
                    fragment_id: fresh,
                    source_event_id,
                    attack_id,
                    materialization_kind: FinalChordProjectionMaterializationKind::TiedAnchorChain,
                    fragment_ordinal: 0,
                    measure: output.measure,
                    pitched_ordinal: ordinal,
                }
            } else {
                let (index, original) = preceding
                    .next()
                    .ok_or_else(|| invalid("unclaimed pitched sustain output"))?;
                let owner = old
                    .get(&(base.measure, index))
                    .ok_or_else(|| invalid("unbound original sustain occurrence"))?;
                if original.onset != note.onset
                    || original.duration != note.duration
                    || original.pitch_identity != note.pitch_identity
                    || original.voice != note.voice
                    || original.staff != note.staff
                    || original.chord_member != note.chord_member
                    || original.tie_stop != note.tie_stop
                {
                    return Err(invalid("sustain projection changed an existing occurrence"));
                }
                if original.tie_start != note.tie_start
                    && (!note.tie_start
                        || !terminal.contains(&(owner.source_event_id, owner.attack_id))
                        || !extended.insert((owner.source_event_id, owner.attack_id)))
                {
                    return Err(invalid("sustain projection changed a foreign tie"));
                }
                FinalChordTransactionOutputOccurrenceWitness {
                    pitched_ordinal: ordinal,
                    ..**owner
                }
            };
            bound.push(row);
        }
        if preceding.next().is_some() {
            return Err(invalid("sustain projection removed a pitched occurrence"));
        }
    }
    if bound.len() != rows.len() + generated.len() || extended != terminal {
        return Err(invalid("incomplete sustain projection readback"));
    }
    let mut written_chains = BTreeMap::<Identity, Vec<&FinalChordEncodingNote>>::new();
    for row in &bound {
        let identity = (row.source_event_id, row.attack_id);
        if terminal.contains(&identity) {
            written_chains
                .entry(identity)
                .or_default()
                .push(final_chord_transaction_output_occurrence_note(&after, row)?);
        }
    }
    for ((source, _), mut chain) in written_chains {
        let interval = time.source[source]
            .as_ref()
            .ok_or_else(|| invalid("lost exact source interval"))?;
        chain.sort_by_key(|note| (note.measure, note.onset));
        let mut cursor = interval.start;
        let first = chain[0];
        for (index, note) in chain.iter().enumerate() {
            let bar = time
                .metric
                .measure(note.measure - 1)
                .ok_or_else(|| invalid("unbound written measure"))?;
            if bar.start + note.onset != cursor
                || note.duration <= 0
                || note.pitch_identity != first.pitch_identity
                || note.staff != first.staff
                || note.voice != first.voice
                || note.tie_stop != (index != 0)
                || note.tie_start != (index + 1 < chain.len())
            {
                return Err(invalid("written sustain chain is not contiguous"));
            }
            cursor += note.duration;
        }
        if cursor != interval.end {
            return Err(invalid("written sustain did not reach the source release"));
        }
    }
    Ok(bound)
}

fn capture_sustain_execution(
    proof: &SustainProof,
    original: &[FinalChordTransactionOutputOccurrenceWitness],
    receipt: &ExactTransformationReceipt,
) -> io::Result<crate::master_state_identity::FinalChordCourseCapture> {
    use crate::master_state_identity::{FinalChordCourseCapture, FinalChordCourseRow};
    let before = final_chord_parse_binding_measures(&proof.original)?;
    let after = final_chord_parse_binding_measures(&proof.written)?;
    let key = |row: &FinalChordTransactionOutputOccurrenceWitness| {
        (row.fragment_id, row.fragment_ordinal)
    };
    let mut output = proof
        .occurrences
        .iter()
        .map(|row| (key(row), row))
        .collect::<BTreeMap<_, _>>();
    let mut rows = Vec::new();
    let mut extended = BTreeMap::new();
    for owner in original {
        let new_owner = output
            .remove(&key(owner))
            .ok_or_else(|| invalid("missing original course emission"))?;
        let pre = final_chord_transaction_output_occurrence_note(&before, owner)?;
        let post = final_chord_transaction_output_occurrence_note(&after, new_owner)?;
        if !pre.tie_start && post.tie_start {
            extended.insert((owner.source_event_id, owner.attack_id), rows.len());
        }
        rows.push(FinalChordCourseRow {
            input: pre.range.clone(),
            outputs: vec![(post.range.clone(), false)],
            ordinals: vec![0],
            source: Some(owner.source_event_id),
            tied_chain: false,
            auxiliary: false,
            original_offset: 0,
        });
    }
    for owner in output.into_values() {
        let index = extended
            .get(&(owner.source_event_id, owner.attack_id))
            .ok_or_else(|| invalid("new fragment has no terminal course emission"))?;
        let note = final_chord_transaction_output_occurrence_note(&after, owner)?;
        let row = &mut rows[*index];
        row.ordinals.push(row.outputs.len());
        row.outputs.push((note.range.clone(), false));
        row.tied_chain = true;
    }
    let readback = receipt
        .operation_readback(&proof.original)
        .map_err(io::Error::other)?;
    for note in before
        .iter()
        .flat_map(|bar| &bar.notes)
        .filter(|note| note.rest_like)
    {
        if let Ok(range) = readback.carry_range(note.range.clone())
            && proof.original.get(note.range.clone()) == proof.written.get(range.clone())
        {
            rows.push(FinalChordCourseRow {
                input: note.range.clone(),
                outputs: vec![(range, false)],
                ordinals: vec![0],
                source: None,
                tied_chain: false,
                auxiliary: false,
                original_offset: 0,
            });
        }
    }
    FinalChordCourseCapture::recorded(
        &FinalChordEncodingExactMint { _sealed: () },
        receipt,
        &proof.original,
        &proof.written,
        rows,
    )
    .map_err(io::Error::other)
}
