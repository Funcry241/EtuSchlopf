//! Threshold-free productive phrase-notation policy for r572.
//!
//! An existing `allow_structural_pending_visual` intent may reach the already
//! established phrase writer only when the current pre-phrase Render-MusicXML
//! contains the same complete intrinsic melodic form in another non-overlapping
//! window of the same part/staff/voice lane. The form consists only of relative
//! pitch-centroid deltas plus chord-member counts, so it is transposition
//! invariant and rhythm-independent. Cross-lane-only and rhythm-only repetition
//! grant no authority. No audit report is consumed by this module.

use crate::audit_only::phrase_boundary_evidence::PhraseNotationIntentProjection;
use anyhow::{Context, Result};
use roxmltree::{Document, Node};
use std::collections::BTreeMap;
use std::ops::{Bound::Excluded, Range};

type PickupPhrase = (PhraseNotationIntentProjection, Range<usize>, Range<usize>);

/// A single-note pickup can lead into a connected upper line across written
/// voices. The first sustained arrival must outlive an accompanying attack.
pub(super) fn select_connected_pickup_phrase(xml: &str) -> Result<Option<PickupPhrase>> {
    let document = Document::parse(xml)?;
    let mut parts = document.root_element().children().filter(|node| is_el(*node, "part"));
    let (Some(part), None) = (parts.next(), parts.next()) else { return Ok(None); };
    let Some(part_id) = part.attribute("id") else { return Ok(None); };
    let mut measures = part.children().filter(|node| is_el(*node, "measure"));
    let (Some(pickup), Some(next)) = (measures.next(), measures.next()) else { return Ok(None); };
    if pickup.attribute("implicit") != Some("yes") || next.attribute("implicit") == Some("yes") {
        return Ok(None);
    }
    let pickup_attacks = melody_attacks(pickup);
    let Some((start, [first])) = pickup_attacks.first_key_value()
        .map(|(onset, group)| (onset, group.as_slice())) else { return Ok(None); };
    if pickup_attacks.len() != 1 || *start != 0 || first.tie_start {
        return Ok(None);
    }
    let following = melody_attacks(next);
    let mut expected = 0;
    let mut attack_count = 1;
    let mut previous_pitch = first.pitch;
    for (&onset, group) in &following {
        let Some(upper) = group.iter().max_by_key(|note| note.pitch) else { return Ok(None); };
        if onset < expected && upper.pitch < previous_pitch { continue; }
        if onset != expected { return Ok(None); }
        if upper.staff != first.staff || group.iter().filter(|note| note.pitch == upper.pitch).count() != 1 {
            return Ok(None);
        }
        attack_count += 1;
        expected = onset + upper.duration;
        let arrives_by_step = (1..=2).contains(&upper.pitch.abs_diff(previous_pitch));
        previous_pitch = upper.pitch;
        if !upper.tie_start { continue; }
        let has_accompaniment = following.range((Excluded(onset), Excluded(expected)))
            .next().is_some_and(|(_, group)| group.iter().all(|note| note.pitch < upper.pitch));
        if attack_count < 3 || !arrives_by_step || !has_accompaniment { return Ok(None); }
        return Ok(Some((PhraseNotationIntentProjection {
            part: part_id.to_owned(), staff: first.staff.clone(), voice: first.voice.clone(),
            start_measure_ordinal: 1, start_onset: *start, start_canonical: 0,
            start_pitches: vec![first.pitch], end_measure_ordinal: 2, end_onset: onset,
            end_canonical: i128::from(first.duration) + i128::from(onset),
            end_pitches: vec![upper.pitch], attack_count,
        }, first.range.clone(), upper.range.clone())));
    }
    Ok(None)
}

struct MelodyAttack {
    pitch: i32,
    duration: i64,
    staff: String,
    voice: String,
    tie_start: bool,
    range: Range<usize>,
}

fn melody_attacks(measure: Node<'_, '_>) -> BTreeMap<i64, Vec<MelodyAttack>> {
    let mut groups = BTreeMap::<i64, Vec<MelodyAttack>>::new();
    let (mut cursor, mut onset) = (0_i64, 0_i64);
    for note in measure.children().filter(Node::is_element) {
        let duration = note_duration(note);
        match note.tag_name().name() {
            "backup" => cursor -= duration,
            "forward" => cursor += duration,
            "note" => {
                if !note.children().any(|node| is_el(node, "chord")) {
                    onset = cursor;
                    cursor += duration;
                }
                let tied = |kind| note.children().any(|node| is_el(node, "tie") && node.attribute("type") == Some(kind));
                if duration > 0 && note.attribute("print-object") != Some("no")
                    && !tied("stop") && let Some(pitch) = note_pitch_midi(note)
                {
                    groups.entry(onset).or_default().push(MelodyAttack {
                        pitch, duration, staff: child_text(note, "staff").unwrap_or_else(|| "1".into()),
                        voice: child_text(note, "voice").unwrap_or_else(|| "1".into()),
                        tie_start: tied("start"), range: note.range(),
                    });
                }
            }
            _ => {}
        }
    }
    groups
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PolicyLane {
    part: String,
    staff: String,
    voice: String,
}

#[derive(Debug, Clone)]
struct PolicyRawNote {
    source_ordinal: usize,
    measure_ordinal: usize,
    onset: i64,
    pitch: i32,
    tie_stop: bool,
}

#[derive(Debug, Clone)]
struct PolicyAttack {
    measure_ordinal: usize,
    onset: i64,
    pitches: Vec<i32>,
    chord_members: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExactMelodicForm {
    centroid_deltas_x100: Vec<i64>,
    chord_members: Vec<usize>,
}

pub(super) fn approve_pending_visual_intents_from_musicxml(
    musicxml: &str,
    pending: &[PhraseNotationIntentProjection],
) -> Result<Vec<PhraseNotationIntentProjection>> {
    let lane_attacks = project_policy_attacks(musicxml)?;
    let mut approved = Vec::new();

    for intent in pending {
        let source_lane = PolicyLane {
            part: intent.part.clone(),
            staff: intent.staff.clone(),
            voice: intent.voice.clone(),
        };
        let Some(source_attacks) = lane_attacks.get(&source_lane) else {
            eprintln!("PhrasePolicyReadback: reason=missing_lane");
            continue;
        };
        let Some((source_start_index, source_window)) = source_window(source_attacks, intent) else {
            eprintln!("PhrasePolicyReadback: reason=missing_span");
            continue;
        };
        let Some(source_signature) = exact_melodic_form(source_window) else {
            eprintln!("PhrasePolicyReadback: reason=incomplete_form");
            continue;
        };
        let source_end_index = source_start_index
            .saturating_add(intent.attack_count)
            .saturating_sub(1);

        let mut same_lane_match = false;
        if source_attacks.len() >= intent.attack_count {
            for candidate_start_index in 0..=source_attacks.len() - intent.attack_count {
                let candidate_end_index = candidate_start_index + intent.attack_count - 1;
                if index_ranges_overlap(
                    candidate_start_index,
                    candidate_end_index,
                    source_start_index,
                    source_end_index,
                ) {
                    continue;
                }
                let candidate = &source_attacks[candidate_start_index..=candidate_end_index];
                if exact_melodic_form(candidate).as_ref() == Some(&source_signature) {
                    same_lane_match = true;
                    break;
                }
            }
        }

        if same_lane_match {
            approved.push(intent.clone());
        } else {
            eprintln!("PhrasePolicyReadback: reason=no_same_lane_repeat");
        }
    }

    Ok(approved)
}

fn project_policy_attacks(musicxml: &str) -> Result<BTreeMap<PolicyLane, Vec<PolicyAttack>>> {
    let doc = Document::parse(musicxml)
        .context("PhraseNotationPolicy: current MusicXML is not parseable")?;
    let root = doc.root_element();
    let mut lanes = BTreeMap::<PolicyLane, Vec<PolicyRawNote>>::new();
    let mut source_ordinal = 0usize;
    let mut part_ordinal = 0usize;

    for part in root.children().filter(|node| is_el(*node, "part")) {
        part_ordinal = part_ordinal.saturating_add(1);
        let part_id = part
            .attribute("id")
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| format!("P{part_ordinal}"));

        for (measure_index, measure) in part
            .children()
            .filter(|node| is_el(*node, "measure"))
            .enumerate()
        {
            let measure_ordinal = measure_index.saturating_add(1);
            let mut cursor = 0_i64;
            let mut last_non_chord_start = 0_i64;

            for child in measure.children().filter(|node| node.is_element()) {
                if is_el(child, "backup") {
                    cursor = cursor.saturating_sub(note_duration(child));
                    continue;
                }
                if is_el(child, "forward") {
                    cursor = cursor.saturating_add(note_duration(child));
                    continue;
                }
                if !is_el(child, "note") {
                    continue;
                }

                let duration = note_duration(child);
                let chord_member = child.children().any(|node| is_el(node, "chord"));
                let onset = if chord_member {
                    last_non_chord_start
                } else {
                    cursor
                };
                let visible = child.attribute("print-object") != Some("no");
                let is_rest = child.children().any(|node| is_el(node, "rest"));
                let staff = child_text(child, "staff")
                    .filter(|value| !value.is_empty())
                    .unwrap_or_else(|| "1".to_string());
                let voice = child_text(child, "voice")
                    .filter(|value| !value.is_empty())
                    .unwrap_or_else(|| "1".to_string());

                if visible
                    && duration > 0
                    && !is_rest
                    && let Some(pitch) = note_pitch_midi(child)
                {
                    let tie_stop = child.descendants().any(|node| {
                        (is_el(node, "tie") || is_el(node, "tied"))
                            && node.attribute("type") == Some("stop")
                    });
                    lanes
                        .entry(PolicyLane {
                            part: part_id.clone(),
                            staff,
                            voice,
                        })
                        .or_default()
                        .push(PolicyRawNote {
                            source_ordinal,
                            measure_ordinal,
                            onset,
                            pitch,
                            tie_stop,
                        });
                }
                source_ordinal = source_ordinal.saturating_add(1);

                if !chord_member {
                    last_non_chord_start = onset;
                    cursor = cursor.saturating_add(duration);
                }
            }
        }
    }

    let mut projected = BTreeMap::<PolicyLane, Vec<PolicyAttack>>::new();
    for (lane, mut rows) in lanes {
        rows.sort_by_key(|row| (row.measure_ordinal, row.onset, row.source_ordinal));
        let mut attacks = Vec::new();
        let mut cursor = 0usize;
        while cursor < rows.len() {
            let measure_ordinal = rows[cursor].measure_ordinal;
            let onset = rows[cursor].onset;
            let group_start = cursor;
            cursor = cursor.saturating_add(1);
            while cursor < rows.len()
                && rows[cursor].measure_ordinal == measure_ordinal
                && rows[cursor].onset == onset
            {
                cursor = cursor.saturating_add(1);
            }
            let pitches = rows[group_start..cursor]
                .iter()
                .filter(|row| !row.tie_stop)
                .map(|row| row.pitch)
                .collect::<Vec<_>>();
            if pitches.is_empty() {
                continue;
            }
            attacks.push(PolicyAttack {
                measure_ordinal,
                onset,
                chord_members: pitches.len(),
                pitches,
            });
        }
        projected.insert(lane, attacks);
    }
    Ok(projected)
}

fn source_window<'a>(
    attacks: &'a [PolicyAttack],
    intent: &PhraseNotationIntentProjection,
) -> Option<(usize, &'a [PolicyAttack])> {
    let start_index = attacks.iter().position(|attack| {
        attack.measure_ordinal == intent.start_measure_ordinal && attack.onset == intent.start_onset
    })?;
    let end_exclusive = start_index.checked_add(intent.attack_count)?;
    let window = attacks.get(start_index..end_exclusive)?;
    let end = window.last()?;
    if end.measure_ordinal != intent.end_measure_ordinal || end.onset != intent.end_onset {
        return None;
    }
    Some((start_index, window))
}

fn exact_melodic_form(attacks: &[PolicyAttack]) -> Option<ExactMelodicForm> {
    if attacks.len() < 2 {
        return None;
    }
    let centroids = attacks
        .iter()
        .map(|attack| pitch_centroid_x100(&attack.pitches))
        .collect::<Option<Vec<_>>>()?;
    let centroid_deltas_x100 = centroids
        .windows(2)
        .map(|pair| pair[1].checked_sub(pair[0]))
        .collect::<Option<Vec<_>>>()?;
    Some(ExactMelodicForm {
        centroid_deltas_x100,
        chord_members: attacks.iter().map(|attack| attack.chord_members).collect(),
    })
}

fn pitch_centroid_x100(pitches: &[i32]) -> Option<i64> {
    if pitches.is_empty() {
        return None;
    }
    let sum = pitches.iter().map(|pitch| i64::from(*pitch)).sum::<i64>();
    Some(sum.saturating_mul(100) / i64::try_from(pitches.len()).ok()?)
}

fn index_ranges_overlap(
    left_start: usize,
    left_end: usize,
    right_start: usize,
    right_end: usize,
) -> bool {
    left_start <= right_end && right_start <= left_end
}

fn note_duration(node: Node<'_, '_>) -> i64 {
    child_text(node, "duration")
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(0)
        .max(0)
}

fn note_pitch_midi(note: Node<'_, '_>) -> Option<i32> {
    let pitch = note.children().find(|node| is_el(*node, "pitch"))?;
    let step = child_text(pitch, "step")?;
    let octave = child_text(pitch, "octave")?.parse::<i32>().ok()?;
    let alter = child_text(pitch, "alter")
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(0);
    let pitch_class = match step.as_str() {
        "C" => 0,
        "D" => 2,
        "E" => 4,
        "F" => 5,
        "G" => 7,
        "A" => 9,
        "B" => 11,
        _ => return None,
    };
    Some((octave + 1) * 12 + pitch_class + alter)
}

fn child_text(node: Node<'_, '_>, name: &str) -> Option<String> {
    node.children()
        .find(|child| is_el(*child, name))?
        .text()
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(ToOwned::to_owned)
}

fn is_el(node: Node<'_, '_>, name: &str) -> bool {
    node.is_element() && node.tag_name().name() == name
}

#[cfg(test)]
mod tests {
    use super::*;

    fn intent() -> PhraseNotationIntentProjection {
        PhraseNotationIntentProjection {
            part: "P1".to_string(),
            staff: "1".to_string(),
            voice: "1".to_string(),
            start_measure_ordinal: 1,
            start_onset: 0,
            start_canonical: 0,
            start_pitches: vec![60],
            end_measure_ordinal: 1,
            end_onset: 2,
            end_canonical: 2,
            end_pitches: vec![64],
            attack_count: 3,
        }
    }

    fn score(second_voice: bool, repeat: bool) -> String {
        let repeated = if repeat {
            r#"<note><pitch><step>G</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><staff>1</staff></note>
<note><pitch><step>A</step><octave>4</octave></pitch><duration>2</duration><voice>1</voice><staff>1</staff></note>
<note><pitch><step>B</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><staff>1</staff></note>"#
        } else {
            ""
        };
        let other_voice = if second_voice {
            r#"<backup><duration>7</duration></backup>
<note><pitch><step>G</step><octave>3</octave></pitch><duration>1</duration><voice>2</voice><staff>1</staff></note>
<note><pitch><step>A</step><octave>3</octave></pitch><duration>1</duration><voice>2</voice><staff>1</staff></note>
<note><pitch><step>B</step><octave>3</octave></pitch><duration>1</duration><voice>2</voice><staff>1</staff></note>"#
        } else {
            ""
        };
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="3.1"><part-list><score-part id="P1"><part-name>P</part-name></score-part></part-list>
<part id="P1"><measure number="1">
<note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><staff>1</staff></note>
<note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><staff>1</staff></note>
<note><pitch><step>E</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><staff>1</staff></note>
{repeated}{other_voice}</measure></part></score-partwise>"#
        )
    }

    #[test]
    fn same_lane_transposed_form_with_different_rhythm_is_approved() {
        let approved = approve_pending_visual_intents_from_musicxml(&score(false, true), &[intent()])
            .expect("policy projection");
        assert_eq!(approved.len(), 1);
    }

    #[test]
    fn cross_lane_only_repetition_is_not_approved() {
        let approved = approve_pending_visual_intents_from_musicxml(&score(true, false), &[intent()])
            .expect("policy projection");
        assert!(approved.is_empty());
    }

    #[test]
    fn pickup_phrase_binds_unique_upper_arrival_across_written_voices() {
        fn note(step: &str, octave: u8, voice: u8, duration: u8, prefix: &str, tie: &str) -> String {
            format!("<note>{prefix}<pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration>{tie}<voice>{voice}</voice><staff>1</staff></note>")
        }
        for (opening_voice, melody_voice, ending_voice, transpose) in [(1, 3, 5, 0), (7, 2, 8, 1)] {
            let pickup = note("B", 3 + transpose, opening_voice, 2, "", "");
            let head = note("D", 4 + transpose, melody_voice, 2, "", "");
            let middle = note("E", 4 + transpose, melody_voice, 1, "", "");
            let chord = note("C", 4 + transpose, ending_voice, 3, "", "");
            let arrival = note("F", 4 + transpose, ending_voice, 3, "<chord/>", "<tie type=\"start\"/>");
            let support = note("B", 3 + transpose, opening_voice, 1, "", "");
            let support_run = format!("<backup><duration>2</duration></backup>{support}");
            let first_number = 9 + transpose;
            let next_number = first_number + 1;
            let xml = format!("<score-partwise><part id=\"P\"><measure number=\"{first_number}\" implicit=\"yes\">{pickup}</measure><measure number=\"{next_number}\">{head}{middle}{chord}{arrival}{support_run}</measure></part></score-partwise>");
            let (intent, start, end) = select_connected_pickup_phrase(&xml).unwrap().unwrap();
            assert_eq!(intent.attack_count, 4);
            assert_eq!(&xml[start], pickup);
            assert_eq!(&xml[end], arrival);
            let duplicate = xml.replace(&arrival, &format!("{arrival}{arrival}"));
            assert!(select_connected_pickup_phrase(&duplicate).unwrap().is_none());
            let continuation = xml.replace("type=\"start\"", "type=\"stop\"");
            assert!(select_connected_pickup_phrase(&continuation).unwrap().is_none());
        }
    }
}
