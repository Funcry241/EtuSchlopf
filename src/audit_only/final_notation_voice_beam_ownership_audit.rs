use roxmltree::Document;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

pub(crate) const SECTION: &str = "FINAL-NOTATION-VOICE-BEAM-OWNERSHIP-AUDIT";
pub(crate) const SECTION_RULE: &str = "------------------------------------------";
pub(crate) const SHADOW_SECTION: &str = "NOTATION-VOICE-SHADOW-CONSTRAINT-AUDIT";
pub(crate) const SHADOW_SECTION_RULE: &str = "--------------------------------------";

type BeamEventIdentity = (i64, i64, String);
type PrimaryChainKey = (String, String, String, Vec<BeamEventIdentity>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct BeamOwnershipEvent {
    part: String,
    measure: String,
    staff: String,
    voice: String,
    onset: i64,
    duration: i64,
    visible: bool,
    is_rest: bool,
    is_chord: bool,
    pitch_midi: Option<i32>,
    pitched_ordinal: Option<usize>,
    beam1: Option<String>,
    divisions: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VoiceFreePrimaryBeamMemberEvidence {
    pub(crate) part: String,
    pub(crate) measure: usize,
    pub(crate) staff: u8,
    pub(crate) onset: i64,
    pub(crate) duration: i64,
    pub(crate) pitch_midi: i32,
    pub(crate) pitched_ordinal: usize,
    pub(crate) current_voice: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VoiceFreePrimaryBeamChainEvidence {
    pub(crate) members: Vec<VoiceFreePrimaryBeamMemberEvidence>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct VoiceBeamOwnershipAudit {
    parse_ok: bool,
    primary_beam_chains: usize,
    same_staff_cross_voice_primary_beam_chains: usize,
    same_staff_cross_voice_metric_runs: usize,
    same_staff_cross_voice_metric_runs_with_primary_chain: usize,
    same_staff_cross_voice_metric_runs_without_primary_chain: usize,
    same_staff_cross_voice_notated_overlap_scopes: usize,
    same_staff_cross_voice_notated_overlap_pairs: usize,
    shadow_merge_candidates: usize,
    shadow_unresolved_candidates: usize,
    rows: Vec<String>,
    shadow_rows: Vec<String>,
}

fn child_text(node: roxmltree::Node<'_, '_>, name: &str) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.has_tag_name(name))
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn child_duration(node: roxmltree::Node<'_, '_>) -> i64 {
    child_text(node, "duration")
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(0)
}

fn primary_beam_state(node: roxmltree::Node<'_, '_>) -> Option<String> {
    node.children()
        .filter(|child| child.is_element() && child.has_tag_name("beam"))
        .find(|beam| beam.attribute("number").unwrap_or("1") == "1")
        .and_then(|beam| beam.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase)
}

fn pitch_midi(node: roxmltree::Node<'_, '_>) -> Option<i32> {
    let pitch = node
        .children()
        .find(|child| child.is_element() && child.has_tag_name("pitch"))?;
    let step = child_text(pitch, "step")?;
    let octave = child_text(pitch, "octave")?.parse::<i32>().ok()?;
    let alter = child_text(pitch, "alter")
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(0);
    let semitone = match step.as_str() {
        "C" => 0,
        "D" => 2,
        "E" => 4,
        "F" => 5,
        "G" => 7,
        "A" => 9,
        "B" => 11,
        _ => return None,
    };
    Some((octave + 1).saturating_mul(12).saturating_add(semitone + alter))
}

fn collect_events(xml: &str) -> Option<Vec<BeamOwnershipEvent>> {
    let document = Document::parse(xml).ok()?;
    let mut events = Vec::new();

    for part in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("part"))
    {
        let part_id = part.attribute("id").unwrap_or("-").to_string();
        let mut inherited_divisions = 1_i64;
        for (measure_index, measure) in part
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("measure"))
            .enumerate()
        {
            if let Some(divisions) = measure
                .descendants()
                .find(|node| node.is_element() && node.has_tag_name("divisions"))
                .and_then(|node| node.text())
                .and_then(|value| value.trim().parse::<i64>().ok())
            {
                inherited_divisions = divisions.max(1);
            }
            let measure_id = measure
                .attribute("number")
                .map(str::to_string)
                .unwrap_or_else(|| (measure_index + 1).to_string());
            let mut cursor = 0_i64;
            let mut last_note_onset = 0_i64;
            let mut next_pitched_ordinal = 0usize;

            for node in measure.children().filter(|node| node.is_element()) {
                if node.has_tag_name("backup") {
                    cursor = cursor.saturating_sub(child_duration(node).max(0));
                    continue;
                }
                if node.has_tag_name("forward") {
                    cursor = cursor.saturating_add(child_duration(node).max(0));
                    continue;
                }
                if !node.has_tag_name("note") {
                    continue;
                }

                let duration = child_duration(node).max(0);
                let is_chord = node
                    .children()
                    .any(|child| child.is_element() && child.has_tag_name("chord"));
                let onset = if is_chord { last_note_onset } else { cursor };
                if !is_chord {
                    last_note_onset = onset;
                }
                let is_rest = node
                    .children()
                    .any(|child| child.is_element() && child.has_tag_name("rest"));
                let note_pitch_midi = pitch_midi(node);
                let pitched_ordinal = if !is_rest && note_pitch_midi.is_some() {
                    let ordinal = next_pitched_ordinal;
                    next_pitched_ordinal = next_pitched_ordinal.saturating_add(1);
                    Some(ordinal)
                } else {
                    None
                };
                let visible = node.attribute("print-object") != Some("no");
                let staff = child_text(node, "staff").unwrap_or_else(|| "1".to_string());
                let voice = child_text(node, "voice").unwrap_or_else(|| "1".to_string());

                events.push(BeamOwnershipEvent {
                    part: part_id.clone(),
                    measure: measure_id.clone(),
                    staff,
                    voice,
                    onset,
                    duration,
                    visible,
                    is_rest,
                    is_chord,
                    pitch_midi: note_pitch_midi,
                    pitched_ordinal,
                    beam1: primary_beam_state(node),
                    divisions: inherited_divisions,
                });

                if !is_chord {
                    cursor = cursor.saturating_add(duration);
                }
            }
        }
    }

    Some(events)
}

fn metric_run_key(event: &BeamOwnershipEvent) -> (String, String, String, i64) {
    let beat = event.onset.div_euclid(event.divisions.max(1));
    (
        event.part.clone(),
        event.measure.clone(),
        event.staff.clone(),
        beat,
    )
}

fn exact_primary_chain_keys(events: &[BeamOwnershipEvent]) -> BTreeSet<PrimaryChainKey> {
    let mut out = BTreeSet::new();
    let mut chain = Vec::<&BeamOwnershipEvent>::new();
    for event in events {
        match event.beam1.as_deref() {
            Some("begin") => {
                chain.clear();
                chain.push(event);
            }
            Some("continue") | Some("forward hook") | Some("backward hook") => {
                if !chain.is_empty() {
                    chain.push(event);
                }
            }
            Some("end") => {
                if !chain.is_empty() {
                    chain.push(event);
                    if chain.len() >= 2 {
                        let first = chain[0];
                        if chain.iter().all(|member| {
                            member.part == first.part
                                && member.measure == first.measure
                                && member.staff == first.staff
                                && member.visible
                                && !member.is_rest
                                && !member.is_chord
                        }) {
                            out.insert((
                                first.part.clone(),
                                first.measure.clone(),
                                first.staff.clone(),
                                chain
                                    .iter()
                                    .map(|member| {
                                        (member.onset, member.duration, member.voice.clone())
                                    })
                                    .collect(),
                            ));
                        }
                    }
                    chain.clear();
                }
            }
            _ => {
                chain.clear();
            }
        }
    }
    out
}


pub(crate) fn voice_free_primary_beam_chain_evidence(
    xml: &str,
) -> Option<Vec<VoiceFreePrimaryBeamChainEvidence>> {
    let events = collect_events(xml)?;
    let mut chains = Vec::new();
    let mut active_chain = Vec::<&BeamOwnershipEvent>::new();

    for event in &events {
        match event.beam1.as_deref() {
            Some("begin") => {
                active_chain.clear();
                active_chain.push(event);
            }
            Some("continue") | Some("forward hook") | Some("backward hook") => {
                if !active_chain.is_empty() {
                    active_chain.push(event);
                }
            }
            Some("end") => {
                if active_chain.is_empty() {
                    continue;
                }
                active_chain.push(event);
                if active_chain.len() >= 2 {
                    let first = active_chain[0];
                    let same_surface = active_chain.iter().all(|member| {
                        member.part == first.part
                            && member.measure == first.measure
                            && member.staff == first.staff
                            && member.visible
                            && !member.is_rest
                            && !member.is_chord
                            && member.duration > 0
                            && member.pitch_midi.is_some()
                            && member.pitched_ordinal.is_some()
                    });
                    if same_surface {
                        let measure = first.measure.parse::<usize>().ok();
                        let staff = first.staff.parse::<u8>().ok();
                        if let (Some(measure), Some(staff)) = (measure, staff)
                            && matches!(staff, 1 | 2)
                        {
                            let members = active_chain
                                .iter()
                                .map(|member| VoiceFreePrimaryBeamMemberEvidence {
                                    part: member.part.clone(),
                                    measure,
                                    staff,
                                    onset: member.onset,
                                    duration: member.duration,
                                    pitch_midi: member.pitch_midi.unwrap_or_default(),
                                    pitched_ordinal: member.pitched_ordinal.unwrap_or_default(),
                                    current_voice: member.voice.clone(),
                                })
                                .collect::<Vec<_>>();
                            chains.push(VoiceFreePrimaryBeamChainEvidence { members });
                        }
                    }
                }
                active_chain.clear();
            }
            _ => {
                active_chain.clear();
            }
        }
    }

    Some(chains)
}

fn build_audit(xml: &str) -> VoiceBeamOwnershipAudit {
    let Some(events) = collect_events(xml) else {
        return VoiceBeamOwnershipAudit::default();
    };
    let mut audit = VoiceBeamOwnershipAudit {
        parse_ok: true,
        ..VoiceBeamOwnershipAudit::default()
    };

    let mut active_chain = Vec::<&BeamOwnershipEvent>::new();
    for event in &events {
        match event.beam1.as_deref() {
            Some("begin") => {
                active_chain.clear();
                active_chain.push(event);
            }
            Some("continue") | Some("forward hook") | Some("backward hook") => {
                if !active_chain.is_empty() {
                    active_chain.push(event);
                }
            }
            Some("end") => {
                if active_chain.is_empty() {
                    continue;
                }
                active_chain.push(event);
                if active_chain.len() >= 2 {
                    audit.primary_beam_chains = audit.primary_beam_chains.saturating_add(1);
                    let first = active_chain[0];
                    let voices = active_chain
                        .iter()
                        .map(|member| member.voice.clone())
                        .collect::<BTreeSet<_>>();
                    let same_staff = active_chain.iter().all(|member| {
                        member.part == first.part
                            && member.measure == first.measure
                            && member.staff == first.staff
                    });
                    let visible_pitched = active_chain.iter().all(|member| {
                        member.visible && !member.is_rest && !member.is_chord
                    });
                    if same_staff && visible_pitched && voices.len() > 1 {
                        audit.same_staff_cross_voice_primary_beam_chains = audit
                            .same_staff_cross_voice_primary_beam_chains
                            .saturating_add(1);
                        audit.rows.push(format!(
                            "SAME_STAFF_CROSS_VOICE_PRIMARY_BEAM part={} measure={} staff={} interval={}-{} voices={}",
                            first.part,
                            first.measure,
                            first.staff,
                            first.onset,
                            active_chain
                                .last()
                                .map(|member| member.onset.saturating_add(member.duration))
                                .unwrap_or(first.onset),
                            voices.into_iter().collect::<Vec<_>>().join(",")
                        ));
                    }
                }
                active_chain.clear();
            }
            _ => {
                active_chain.clear();
            }
        }
    }

    let mut overlap_scopes = BTreeMap::<(String, String, String, String, String), (usize, i64, i64)>::new();
    let mut visible_pitched_by_scope = BTreeMap::<(String, String, String), Vec<&BeamOwnershipEvent>>::new();
    for event in events.iter().filter(|event| {
        event.visible && !event.is_rest && !event.is_chord && event.duration > 0
    }) {
        visible_pitched_by_scope
            .entry((event.part.clone(), event.measure.clone(), event.staff.clone()))
            .or_default()
            .push(event);
    }
    for ((part, measure, staff), group) in &visible_pitched_by_scope {
        for left_index in 0..group.len() {
            for right_index in left_index + 1..group.len() {
                let left = group[left_index];
                let right = group[right_index];
                if left.voice == right.voice {
                    continue;
                }
                let left_end = left.onset.saturating_add(left.duration);
                let right_end = right.onset.saturating_add(right.duration);
                if left.onset >= right_end || right.onset >= left_end {
                    continue;
                }
                let (voice_a, voice_b) = if left.voice <= right.voice {
                    (left.voice.clone(), right.voice.clone())
                } else {
                    (right.voice.clone(), left.voice.clone())
                };
                let entry = overlap_scopes
                    .entry((part.clone(), measure.clone(), staff.clone(), voice_a, voice_b))
                    .or_insert((0, i64::MAX, i64::MIN));
                entry.0 = entry.0.saturating_add(1);
                entry.1 = entry.1.min(left.onset.min(right.onset));
                entry.2 = entry.2.max(left_end.max(right_end));
            }
        }
    }
    audit.same_staff_cross_voice_notated_overlap_scopes = overlap_scopes.len();
    audit.same_staff_cross_voice_notated_overlap_pairs = overlap_scopes
        .values()
        .map(|(pairs, _, _)| *pairs)
        .sum();
    for ((part, measure, staff, voice_a, voice_b), (pairs, start, end)) in &overlap_scopes {
        audit.shadow_rows.push(format!(
            "SHADOW_NOTATION_VOICE_DECISION part={} measure={} staff={} interval={}-{} current_voices={},{} proposed=PRESERVE_SEPARATE reason=real_notated_overlap overlap_pairs={}",
            part, measure, staff, start, end, voice_a, voice_b, pairs
        ));
    }

    let primary_chain_keys = exact_primary_chain_keys(&events);
    let mut by_metric_scope =
        BTreeMap::<(String, String, String, i64), Vec<&BeamOwnershipEvent>>::new();
    for event in events.iter().filter(|event| {
        event.visible
            && !event.is_rest
            && !event.is_chord
            && event.duration > 0
            && event.duration <= event.divisions.max(1) / 2
    }) {
        by_metric_scope
            .entry(metric_run_key(event))
            .or_default()
            .push(event);
    }

    for ((part, measure, staff, beat), mut group) in by_metric_scope {
        group.sort_by_key(|event| (event.onset, event.duration, event.voice.clone()));
        if group.len() < 2 {
            continue;
        }
        let voices = group
            .iter()
            .map(|event| event.voice.clone())
            .collect::<BTreeSet<_>>();
        if voices.len() < 2 {
            continue;
        }
        if group.windows(2).any(|pair| pair[0].onset == pair[1].onset) {
            continue;
        }
        let contiguous = group
            .windows(2)
            .all(|pair| pair[0].onset.saturating_add(pair[0].duration) == pair[1].onset);
        if !contiguous {
            continue;
        }
        let beat_unit = group[0].divisions.max(1);
        let beat_start = beat.saturating_mul(beat_unit);
        let beat_end = beat_start.saturating_add(beat_unit);
        if group[0].onset < beat_start
            || group
                .last()
                .is_some_and(|event| event.onset.saturating_add(event.duration) > beat_end)
        {
            continue;
        }

        audit.same_staff_cross_voice_metric_runs = audit
            .same_staff_cross_voice_metric_runs
            .saturating_add(1);
        let key = (
            part.clone(),
            measure.clone(),
            staff.clone(),
            group
                .iter()
                .map(|event| (event.onset, event.duration, event.voice.clone()))
                .collect::<Vec<_>>(),
        );
        let has_chain = primary_chain_keys.contains(&key);
        if has_chain {
            audit.same_staff_cross_voice_metric_runs_with_primary_chain = audit
                .same_staff_cross_voice_metric_runs_with_primary_chain
                .saturating_add(1);
        } else {
            audit.same_staff_cross_voice_metric_runs_without_primary_chain = audit
                .same_staff_cross_voice_metric_runs_without_primary_chain
                .saturating_add(1);
        }

        let run_start = group[0].onset;
        let run_end = group
            .last()
            .map(|event| event.onset.saturating_add(event.duration))
            .unwrap_or(run_start);
        let group_identities = group
            .iter()
            .map(|event| (event.onset, event.duration, event.voice.clone()))
            .collect::<BTreeSet<_>>();
        let external_polyphony = visible_pitched_by_scope
            .get(&(part.clone(), measure.clone(), staff.clone()))
            .is_some_and(|scope| {
                scope.iter().any(|other| {
                    let identity = (other.onset, other.duration, other.voice.clone());
                    if group_identities.contains(&identity) {
                        return false;
                    }
                    let other_end = other.onset.saturating_add(other.duration);
                    other.onset < run_end && run_start < other_end
                })
            });
        if has_chain && !external_polyphony {
            audit.shadow_merge_candidates = audit.shadow_merge_candidates.saturating_add(1);
            audit.shadow_rows.push(format!(
                "SHADOW_NOTATION_VOICE_DECISION part={} measure={} staff={} interval={}-{} current_voices={} proposed=MERGE_SINGLE_VISIBLE_VOICE reason=sequential_same_line_primary_beam external_polyphony=no",
                part,
                measure,
                staff,
                run_start,
                run_end,
                voices.iter().cloned().collect::<Vec<_>>().join(",")
            ));
        } else {
            audit.shadow_unresolved_candidates = audit.shadow_unresolved_candidates.saturating_add(1);
            audit.shadow_rows.push(format!(
                "SHADOW_NOTATION_VOICE_DECISION part={} measure={} staff={} interval={}-{} current_voices={} proposed=UNRESOLVED reason={} external_polyphony={}",
                part,
                measure,
                staff,
                run_start,
                run_end,
                voices.iter().cloned().collect::<Vec<_>>().join(","),
                if has_chain {
                    "primary_beam_present_but_polyphonic_context"
                } else {
                    "contiguous_cross_voice_without_primary_beam"
                },
                if external_polyphony { "yes" } else { "no" }
            ));
        }
        audit.rows.push(format!(
            "CROSS_VOICE_METRIC_RUN part={} measure={} staff={} beat={} interval={}-{} voices={} primary_chain={}",
            part,
            measure,
            staff,
            beat,
            group[0].onset,
            group
                .last()
                .map(|event| event.onset.saturating_add(event.duration))
                .unwrap_or(group[0].onset),
            voices.into_iter().collect::<Vec<_>>().join(","),
            if has_chain { "yes" } else { "no" }
        ));
    }

    audit.rows.sort();
    audit.rows.dedup();
    audit.shadow_rows.sort();
    audit.shadow_rows.dedup();
    audit
}

pub(crate) fn append_final_notation_voice_beam_ownership_audit_section(
    out: &mut String,
    master_xml: &str,
    render_xml: &str,
) {
    let master = build_audit(master_xml);
    let render = build_audit(render_xml);
    let verdict = if !master.parse_ok || !render.parse_ok {
        "UNAVAILABLE_PARSE_FAILURE"
    } else if render.same_staff_cross_voice_primary_beam_chains > 0
        || render.same_staff_cross_voice_metric_runs > 0
    {
        "VISUAL_VOICE_BEAM_OWNERSHIP_DEBT_PRESENT"
    } else {
        "CLEAN"
    };

    let _ = writeln!(out, "ProjectionOnly: true");
    let _ = writeln!(out, "Authority: none");
    let _ = writeln!(out, "MusicMutation: nein");
    let _ = writeln!(out, "RuleChange: nein");
    let _ = writeln!(out, "LocatorRowsAreDiagnosticOnly: yes");
    let _ = writeln!(
        out,
        "ParseMaster/Render: {}/{}",
        if master.parse_ok { "ok" } else { "fail" },
        if render.parse_ok { "ok" } else { "fail" }
    );
    let _ = writeln!(
        out,
        "MasterPrimaryBeamChains: {}",
        master.primary_beam_chains
    );
    let _ = writeln!(
        out,
        "RenderPrimaryBeamChains: {}",
        render.primary_beam_chains
    );
    let _ = writeln!(
        out,
        "MasterSameStaffCrossVoicePrimaryBeamChains: {}",
        master.same_staff_cross_voice_primary_beam_chains
    );
    let _ = writeln!(
        out,
        "RenderSameStaffCrossVoicePrimaryBeamChains: {}",
        render.same_staff_cross_voice_primary_beam_chains
    );
    let _ = writeln!(
        out,
        "RenderSameStaffCrossVoiceMetricRuns: {}",
        render.same_staff_cross_voice_metric_runs
    );
    let _ = writeln!(
        out,
        "RenderCrossVoiceMetricRunsWithPrimaryChain: {}",
        render.same_staff_cross_voice_metric_runs_with_primary_chain
    );
    let _ = writeln!(
        out,
        "RenderCrossVoiceMetricRunsWithoutPrimaryChain: {}",
        render.same_staff_cross_voice_metric_runs_without_primary_chain
    );
    let _ = writeln!(out, "Verdict: {verdict}");
    if render.rows.is_empty() {
        let _ = writeln!(out, "Rows: none");
    } else {
        for row in render.rows {
            let _ = writeln!(out, "Row: {row}");
        }
    }
}

pub(crate) fn append_notation_voice_shadow_constraint_audit_section(
    out: &mut String,
    master_xml: &str,
    render_xml: &str,
) {
    let master = build_audit(master_xml);
    let render = build_audit(render_xml);
    let verdict = if !master.parse_ok || !render.parse_ok {
        "UNAVAILABLE_PARSE_FAILURE"
    } else if render.shadow_unresolved_candidates > 0 {
        "SHADOW_HAS_UNRESOLVED"
    } else {
        "SHADOW_CLASSIFIED"
    };

    let _ = writeln!(out, "ProjectionOnly: true");
    let _ = writeln!(out, "Authority: none");
    let _ = writeln!(out, "MusicMutation: nein");
    let _ = writeln!(out, "RuleChange: nein");
    let _ = writeln!(out, "CurrentVoiceMutation: nein");
    let _ = writeln!(out, "ShadowMergeIsWriteAuthority: nein");
    let _ = writeln!(out, "UnresolvedPolicy: preserve_current_voice_fail_closed");
    let _ = writeln!(out, "MergeEvidence: contiguous_metric_run_plus_exact_primary_beam_plus_no_external_notated_polyphony");
    let _ = writeln!(out, "PreserveEvidence: positive_same_staff_cross_voice_notated_overlap");
    let _ = writeln!(out, "TechnicalLaneAuthority: none_for_shadow_decision");
    let _ = writeln!(out, "MasterShadowMergeCandidates: {}", master.shadow_merge_candidates);
    let _ = writeln!(out, "RenderShadowMergeCandidates: {}", render.shadow_merge_candidates);
    let _ = writeln!(out, "RenderShadowUnresolvedCandidates: {}", render.shadow_unresolved_candidates);
    let _ = writeln!(out, "RenderNotatedOverlapScopesPreserveSeparate: {}", render.same_staff_cross_voice_notated_overlap_scopes);
    let _ = writeln!(out, "RenderNotatedOverlapPairs: {}", render.same_staff_cross_voice_notated_overlap_pairs);
    let _ = writeln!(out, "Verdict: {verdict}");
    if render.shadow_rows.is_empty() {
        let _ = writeln!(out, "Rows: none");
    } else {
        for row in render.shadow_rows {
            let _ = writeln!(out, "Row: {row}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(body: &str) -> String {
        format!(
            "<score-partwise><part id=\"P1\"><measure number=\"1\"><attributes><divisions>96</divisions></attributes>{body}</measure></part></score-partwise>"
        )
    }

    fn note(onset_gap: i64, voice: &str, beam: &str) -> String {
        let gap = if onset_gap > 0 {
            format!("<forward><duration>{onset_gap}</duration></forward>")
        } else {
            String::new()
        };
        format!(
            "{gap}<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>{voice}</voice><type>16th</type><staff>1</staff><beam number=\"1\">{beam}</beam></note>"
        )
    }

    #[test]
    fn detects_same_staff_cross_voice_primary_beam_chain() {
        let xml = score(&format!(
            "{}{}{}{}",
            note(0, "1", "begin"),
            note(0, "3", "continue"),
            note(0, "1", "continue"),
            note(0, "3", "end")
        ));
        let audit = build_audit(&xml);
        assert!(audit.parse_ok);
        assert_eq!(audit.same_staff_cross_voice_primary_beam_chains, 1);
        assert_eq!(audit.same_staff_cross_voice_metric_runs, 1);
        assert_eq!(audit.same_staff_cross_voice_metric_runs_with_primary_chain, 1);
        assert_eq!(audit.shadow_merge_candidates, 1);
        assert_eq!(audit.shadow_unresolved_candidates, 0);
    }

    #[test]
    fn same_voice_metric_run_is_not_cross_voice_debt() {
        let xml = score(&format!(
            "{}{}{}{}",
            note(0, "1", "begin"),
            note(0, "1", "continue"),
            note(0, "1", "continue"),
            note(0, "1", "end")
        ));
        let audit = build_audit(&xml);
        assert_eq!(audit.same_staff_cross_voice_primary_beam_chains, 0);
        assert_eq!(audit.same_staff_cross_voice_metric_runs, 0);
    }

    #[test]
    fn detects_cross_voice_metric_run_even_without_beam_tags() {
        let xml = score(concat!(
            "<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>",
            "<note><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff></note>",
            "<note><pitch><step>E</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>",
            "<note><pitch><step>F</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff></note>"
        ));
        let audit = build_audit(&xml);
        assert_eq!(audit.same_staff_cross_voice_metric_runs, 1);
        assert_eq!(audit.same_staff_cross_voice_metric_runs_with_primary_chain, 0);
        assert_eq!(audit.same_staff_cross_voice_metric_runs_without_primary_chain, 1);
        assert_eq!(audit.shadow_merge_candidates, 0);
        assert_eq!(audit.shadow_unresolved_candidates, 1);
    }

    #[test]
    fn real_notated_overlap_preserves_cross_voice_separation() {
        let xml = score(concat!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>",
            "<backup><duration>96</duration></backup>",
            "<note><pitch><step>E</step><octave>5</octave></pitch><duration>48</duration><voice>3</voice><type>eighth</type><staff>1</staff></note>"
        ));
        let audit = build_audit(&xml);
        assert_eq!(audit.same_staff_cross_voice_notated_overlap_scopes, 1);
        assert_eq!(audit.same_staff_cross_voice_notated_overlap_pairs, 1);
        assert!(audit.shadow_rows.iter().any(|row| {
            row.contains("proposed=PRESERVE_SEPARATE")
                && row.contains("reason=real_notated_overlap")
        }));
    }

    #[test]
    fn beam_candidate_with_external_polyphony_stays_unresolved() {
        let xml = score(&format!(
            "{}{}{}{}<backup><duration>96</duration></backup><note><pitch><step>G</step><octave>3</octave></pitch><duration>96</duration><voice>5</voice><type>quarter</type><staff>1</staff></note>",
            note(0, "1", "begin"),
            note(0, "3", "continue"),
            note(0, "1", "continue"),
            note(0, "3", "end")
        ));
        let audit = build_audit(&xml);
        assert_eq!(audit.shadow_merge_candidates, 0);
        assert_eq!(audit.shadow_unresolved_candidates, 1);
        assert!(audit.shadow_rows.iter().any(|row| {
            row.contains("primary_beam_present_but_polyphonic_context")
        }));
    }

    #[test]
    fn same_voice_overlap_is_not_cross_voice_preserve_scope() {
        let xml = score(concat!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>",
            "<backup><duration>96</duration></backup>",
            "<note><pitch><step>E</step><octave>5</octave></pitch><duration>48</duration><voice>1</voice><type>eighth</type><staff>1</staff></note>"
        ));
        let audit = build_audit(&xml);
        assert_eq!(audit.same_staff_cross_voice_notated_overlap_scopes, 0);
        assert_eq!(audit.same_staff_cross_voice_notated_overlap_pairs, 0);
    }
    #[test]
    fn beam_evidence_pitched_ordinals_count_hidden_and_chord_pitched_notes() {
        let xml = score(concat!(
            "<note print-object=\"no\"><pitch><step>G</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>",
            "<note><pitch><step>A</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>",
            "<note><chord/><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>",
            "<note><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff><beam number=\"1\">begin</beam></note>",
            "<note><pitch><step>E</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff><beam number=\"1\">end</beam></note>"
        ));
        let chains = voice_free_primary_beam_chain_evidence(&xml).expect("beam evidence");
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].members[0].pitched_ordinal, 3);
        assert_eq!(chains[0].members[1].pitched_ordinal, 4);
    }

}
