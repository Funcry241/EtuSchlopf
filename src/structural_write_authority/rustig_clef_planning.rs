use crate::master_state_identity::PreservingTransformationReceipt;
use super::clef_surface_materialization::ClefSurfaceMaterializationPlan;
use crate::clef_readability_observation::{
    observe_lower_staff_octave_display_shift, observe_pitch, ClefPitchObservation, PianoClef,
    LowerChordAttack, LowerChordObservation,
};
use std::collections::{BTreeMap, BTreeSet};

#[path = "rustig_clef_boundary.rs"]
mod beam_boundary;

const REDUNDANT_CLEF_COST: i64 = 1;
const READABILITY_SHORT_EXCURSION_MAX_ATTACKS: usize = 2;
const READABILITY_SHORT_EXCURSION_REFERENCE_COST: i64 = 24;
const READABILITY_CLEF_TRANSITION_COST: i64 = READABILITY_SHORT_EXCURSION_REFERENCE_COST;
const MAX_MARGINAL_G2_ADVANTAGE: i32 = 3;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum ClefKind {
    Treble,
    Bass,
}

impl ClefKind {
    fn ledger_pressure(self, pitch: ClefPitchObservation) -> i64 {
        let clef = match self {
            Self::Treble => PianoClef::TrebleG2,
            Self::Bass => PianoClef::BassF4,
        };
        let ledgers = clef.ledger_lines(pitch);
        ledgers * ledgers + (ledgers - 3).max(0) * 8
    }
}

#[derive(Debug, Clone, Copy)]
enum ClefSurface {
    None,
    Known { kind: ClefKind, visible: bool },
    Unsupported,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum PrimaryBeamState {
    Begin,
    Continue,
    End,
    Hook,
}

#[derive(Debug, Clone)]
struct SurfaceEvent {
    start: usize,
    end: usize,
    kind: ClefKind,
    visible: bool,
    removable: bool,
}

#[derive(Debug, Clone, Copy)]
struct PitchedEvent {
    start: usize,
    pitch: ClefPitchObservation,
    starts_attack: bool,
    counted_attack: bool,
    clef_transition_anchor: Option<usize>,
}

#[derive(Debug, Clone)]
struct AttackEvent {
    pitches: Vec<ClefPitchObservation>,
    clef_transition_anchor: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
struct ClefReadabilityTransitionCandidate {
    start: usize,
    kind: ClefKind,
}

#[derive(Debug, Clone, Copy)]
enum ClefReadabilityTimelineEvent {
    Surface(usize),
    Candidate(usize),
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct ClefReadabilityCost {
    ledger_pressure: i64,
    clef_transitions: usize,
    redundant_clefs: usize,
    short_excursions: usize,
}

impl ClefReadabilityCost {
    fn total(self) -> i64 {
        self.ledger_pressure
            + (self.clef_transitions as i64) * READABILITY_CLEF_TRANSITION_COST
            + (self.redundant_clefs as i64) * REDUNDANT_CLEF_COST
            + (self.short_excursions as i64) * READABILITY_SHORT_EXCURSION_REFERENCE_COST
    }

    fn add_passage(&mut self, ledger_pressure: i64) {
        self.ledger_pressure += ledger_pressure;
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct ClefReadabilityState {
    active: ClefKind,
    excursion_origin: Option<ClefKind>,
    excursion_attacks: usize,
}

#[derive(Debug, Clone)]
struct ClefReadabilityCandidate {
    cost: ClefReadabilityCost,
    visible_events: usize,
    retained: Vec<bool>,
    insertions: Vec<(usize, ClefKind)>,
}

impl ClefReadabilityCandidate {
    fn precedes(&self, other: &Self) -> bool {
        (
            self.cost.total(),
            self.visible_events,
            self.cost.short_excursions,
            self.cost.clef_transitions,
            self.cost.ledger_pressure,
        ) < (
            other.cost.total(),
            other.visible_events,
            other.cost.short_excursions,
            other.cost.clef_transitions,
            other.cost.ledger_pressure,
        )
    }
}

#[derive(Debug, Clone)]
struct ClefReadabilityCourseSolution {
    retained: Vec<bool>,
    insertions: Vec<(usize, ClefKind)>,
    cost: ClefReadabilityCost,
}

#[derive(Debug, Clone)]
struct ClefReadabilityPartAudit {
    part_index: usize,
    source_course: String,
    readability_course: String,
    source_cost: ClefReadabilityCost,
    readability_cost: ClefReadabilityCost,
    source_surfaces: usize,
    readability_surfaces: usize,
    omitted_surface_rows: Vec<String>,
    inserted_surface_rows: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ClefReadabilityShadowCourseAudit {
    parts_observed: usize,
    parts_planned: usize,
    changed_parts: usize,
    omitted_surfaces: usize,
    inserted_surfaces: usize,
    primary_materialized_insertions: usize,
    part_rows: Vec<ClefReadabilityPartAudit>,
    beam_anchor_rows: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RustigClefCoursePlan {
    removals: Vec<(usize, usize)>,
    insertions: Vec<(usize, ClefKind)>,
    source_notes: Vec<String>,
    source_clef_count: usize,
    source_len: usize,
    removed_bytes: usize,
    removed_clef_count: usize,
    inserted_bytes: usize,
    inserted_clef_count: usize,
}

#[derive(Debug, Clone)]
pub struct RustigClefSurfaceDecision {
    candidates: Vec<ClefSurfaceMaterializationPlan>,
}

impl RustigClefCoursePlan {
    pub(super) fn removal_ranges(&self) -> &[(usize, usize)] {
        &self.removals
    }

    pub(super) fn verifies(&self, candidate: &str) -> bool {
        let expected_len = self
            .source_len
            .checked_sub(self.removed_bytes)
            .and_then(|len| len.checked_add(self.inserted_bytes));
        let expected_clef_count = self
            .source_clef_count
            .checked_sub(self.removed_clef_count)
            .and_then(|count| count.checked_add(self.inserted_clef_count));
        expected_len == Some(candidate.len())
            && raw_note_fragments(candidate) == self.source_notes
            && expected_clef_count == Some(clef_count(candidate))
    }

    fn source_matches(&self, source: &str) -> bool {
        source.len() == self.source_len
            && raw_note_fragments(source) == self.source_notes
            && clef_count(source) == self.source_clef_count
    }

    pub(super) fn insertion_blocks(&self) -> Vec<(usize, &'static str)> {
        self.insertions
            .iter()
            .map(|&(start, kind)| (start, clef_readability_primary_clef_block(kind)))
            .collect()
    }
}

impl RustigClefSurfaceDecision {
    pub fn materialization_plan(&self) -> Option<ClefSurfaceMaterializationPlan> {
        (self.candidates.len() == 1).then(|| self.candidates[0])
    }

    pub fn candidate_count(&self) -> usize {
        self.candidates.len()
    }
}

pub fn apply_rustig_clef_course_to_buffer(
    xml: &mut String,
) -> (Result<(), &'static str>, Option<PreservingTransformationReceipt>) {
    let pre_xml = xml.clone();
    // Original operation journal of exactly the materialized course (E024).
    // Observational only: it never changes the written bytes below.
    let mut journal: Option<Vec<(std::ops::Range<usize>, String)>> = None;
    let result = (|| -> Result<(), &'static str> {
        let plan = derive_clef_readability_primary_course_plan(xml);
        let insertions = plan.insertion_blocks();
        if plan.removal_ranges().is_empty() && insertions.is_empty() {
            return if lower_clef_surface_inside_open_tie(xml) {
                Err("clef course left a lower clef transition inside an open tie")
            } else if lower_clef_surface_inside_primary_beam_group(xml) {
                Err("clef course left a lower clef transition inside a primary beam group")
            } else {
                Ok(())
            };
        }
        if !plan.source_matches(xml) {
            return Err("clef course source changed before materialization");
        }
        let mut edits = plan
            .removal_ranges()
            .iter()
            .map(|&(start, end)| (start, Some(end), None))
            .chain(
                insertions
                    .iter()
                    .map(|&(start, block)| (start, None, Some(block))),
            )
            .collect::<Vec<_>>();
        edits.sort_unstable_by_key(|(start, _, _)| *start);
        journal = Some(rustig_clef_course_journal(&edits));
        let mut candidate = xml.clone();
        for (start, end, block) in edits.into_iter().rev() {
            if let Some(end) = end {
                candidate.replace_range(start..end, "");
            } else if let Some(block) = block {
                candidate.insert_str(start, block);
            }
        }
        if !plan.verifies(&candidate) {
            return Err("clef course violated note or clef invariants");
        }
        if lower_clef_surface_inside_open_tie(&candidate) {
            return Err("clef course left a lower clef transition inside an open tie");
        }
        if lower_clef_surface_inside_primary_beam_group(&candidate) {
            return Err("clef course left a lower clef transition inside a primary beam group");
        }
        if candidate != *xml {
            *xml = candidate;
        }
        Ok(())
    })();
    // PRESERVING receipt E024, minted by this owner only for a materialized
    // course whose conserved musical content projection is unchanged.
    let receipt = if result.is_ok() {
        PreservingTransformationReceipt::mint_preserving_receipt(
            RustigClefCoursePreservingMint { _sealed: () },
            &pre_xml,
            xml,
        )
    } else {
        None
    };
    // The journal travels only if its replay reproduces the actual write;
    // otherwise the receipt stays journal-less and custody remains Unknown.
    let receipt = receipt.map(|receipt| match journal {
        Some(edits) => {
            let step = crate::arrangement::representation_transport::RepresentationStep::recorded(
                &pre_xml, xml, edits);
            match crate::arrangement::representation_transport::RhythmicRepresentationTransport::rustig_clef_course(
                &RustigClefCoursePreservingMint { _sealed: () }, step) {
                Ok(transport) => receipt.with_recorded_representation(transport),
                Err(_) => receipt,
            }
        }
        None => receipt,
    });
    (result, receipt)
}

/// Forward journal of the sorted course script. At an equal start an
/// insertion precedes a removal, matching the reverse materialization above.
fn rustig_clef_course_journal(
    edits: &[(usize, Option<usize>, Option<&str>)],
) -> Vec<(std::ops::Range<usize>, String)> {
    let mut ordered = edits.to_vec();
    ordered.sort_by_key(|(start, end, _)| (*start, end.is_some()));
    ordered
        .into_iter()
        .filter_map(|(start, end, block)| match (end, block) {
            (Some(end), _) => Some((start..end, String::new())),
            (None, Some(block)) => Some((start..start, block.to_string())),
            (None, None) => None,
        })
        .collect()
}

/// Sealed mint capability of `apply_rustig_clef_course_to_buffer`
/// (PRESERVING E024).
pub(crate) struct RustigClefCoursePreservingMint {
    _sealed: (),
}

pub(super) fn derive_clef_readability_primary_course_plan(xml: &str) -> RustigClefCoursePlan {
    crate::audit_only::clef_planner_input::emit(xml, super::authority_primitives::fingerprint(xml));
    let mut removals = Vec::new();
    let mut insertions = Vec::new();
    for (part_index, (part_start, part_end)) in score_part_ranges(xml).into_iter().enumerate() {
        let part = &xml[part_start..part_end];
        let Some(surfaces) = scan_lower_clef_surfaces(part) else {
            continue;
        };
        if surfaces.is_empty() {
            continue;
        }
        let mut observations = Vec::new();
        let pitches = scan_lower_pitches(part, Some(&mut observations));
        if pitches.first().is_some_and(|note| note.start < surfaces[0].end) {
            continue;
        }
        let solution = solve_readability_course(part, &surfaces, &pitches);
        let trace = beam_anchor_planner_input_trace(
            xml, part, part_index + 1, &observations, &pitches, solution.as_ref(),
        );
        // Diagnostics cannot change planning or turn stderr failure into a veto.
        let _ = std::io::Write::write_all(&mut std::io::stderr().lock(), trace.as_bytes());
        let Some(solution) = solution else {
            continue;
        };
        for (event, keep) in surfaces.iter().zip(solution.retained.iter()) {
            if !keep {
                removals.push((part_start + event.start, part_start + event.end));
            }
        }
        insertions.extend(
            solution
                .insertions
                .into_iter()
                .map(|(start, kind)| (part_start + start, kind)),
        );
    }

    if !normalize_removal_ranges(xml, &mut removals)
        || !normalize_clef_readability_insertions(xml, &removals, &mut insertions)
    {
        removals.clear();
        insertions.clear();
    }
    let removed_bytes = removals.iter().map(|(start, end)| end - start).sum();
    let removed_clef_count = removals
        .iter()
        .map(|&(start, end)| clef_count(&xml[start..end]))
        .sum();
    let inserted_bytes = insertions
        .iter()
        .map(|(_, kind)| clef_readability_primary_clef_block(*kind).len())
        .sum();
    let inserted_clef_count = insertions.len();

    RustigClefCoursePlan {
        removals,
        insertions,
        source_notes: raw_note_fragments(xml),
        source_clef_count: clef_count(xml),
        source_len: xml.len(),
        removed_bytes,
        removed_clef_count,
        inserted_bytes,
        inserted_clef_count,
    }
}

/// Input-bound evidence before normalization or writing; not an error classifier.
fn beam_anchor_planner_input_trace(
    xml: &str,
    part: &str,
    part_index: usize,
    observations: &[BeamAnchorObservation],
    pitches: &[PitchedEvent],
    solution: Option<&ClefReadabilityCourseSolution>,
) -> String {
    let measures = measure_ranges(part);
    let remote = observations.iter().filter(|row| {
        row.stored_beam_origin.is_some_and(|origin| {
            measures.partition_point(|&(start, _)| start <= origin)
                != measures.partition_point(|&(start, _)| start <= row.start)
        })
    }).cloned().collect::<Vec<_>>();
    let source = super::authority_primitives::fingerprint(xml);
    let part_source = super::authority_primitives::fingerprint(part);
    let binding = format!("InputFnv64={source:016x}; InputBytes={}; Part={part_index}; PartFnv64={part_source:016x}", xml.len());
    let mut out = format!(
        "BeamAnchorPlannerInput: {binding}; Observations={}; CrossMeasureRows={}; SourceRole=actual_planner_input; Scope=eligible_part_after_initial_surface_guard; SolutionAvailable={}; WriterCommit=not_observed; ErrorVerdict=none; Authority=none\n",
        observations.len(), remote.len(), solution.is_some(),
    );
    if remote.is_empty() {
        return out;
    }
    let mut rows = Vec::new();
    append_beam_anchor_rows(part, part_index, &remote, &mut rows);
    for row in rows {
        out.push_str(&format!("BeamAnchorPlannerContext: {binding}; {row}\n"));
    }
    let boundaries = beam_boundary::observe(part);
    let candidates = collect_readability_course_candidates(part, pitches, &boundaries);
    out.push_str(&format!(
        "BeamAnchorPlannerCandidates: {binding}; PositionScope=part_local_bytes; Candidates={candidates:?}; Authority=none\n",
    ));
    match solution {
        Some(solution) => out.push_str(&format!(
            "BeamAnchorPlannerSolution: {binding}; Stage=part_solution_before_global_normalization; PositionScope=part_local_bytes; Retained={:?}; Insertions={:?}; Cost={:?}; WriterCommit=not_observed; Authority=none\n",
            solution.retained, solution.insertions, solution.cost,
        )),
        None => out.push_str(&format!(
            "BeamAnchorPlannerSolution: {binding}; Stage=part_solution_before_global_normalization; Status=unavailable; WriterCommit=not_observed; Authority=none\n",
        )),
    }
    out
}

fn normalize_clef_readability_insertions(
    xml: &str,
    removals: &[(usize, usize)],
    insertions: &mut Vec<(usize, ClefKind)>,
) -> bool {
    let boundaries = beam_boundary::observe(xml);
    insertions.sort_unstable_by_key(|(start, kind)| (*start, *kind));
    insertions.dedup();
    for (index, &(start, _)) in insertions.iter().enumerate() {
        if start > xml.len()
            || !xml.is_char_boundary(start)
            || !xml.get(start..).is_some_and(|tail| tail.starts_with("<note"))
            || removals
                .iter()
                .any(|&(remove_start, remove_end)| start >= remove_start && start < remove_end)
            || insertions
                .get(index + 1)
                .is_some_and(|(next_start, _)| *next_start == start)
            || lower_tie_open_at_position(xml, start)
            || (lower_primary_beam_insertion_blocked(xml, start) && !boundaries.contains(&start))
        {
            return false;
        }
    }
    true
}

fn normalize_removal_ranges(xml: &str, ranges: &mut Vec<(usize, usize)>) -> bool {
    ranges.sort_unstable_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| right.1.cmp(&left.1))
    });
    let mut normalized: Vec<(usize, usize)> = Vec::with_capacity(ranges.len());
    for &(start, end) in ranges.iter() {
        if start >= end
            || end > xml.len()
            || !xml.is_char_boundary(start)
            || !xml.is_char_boundary(end)
        {
            return false;
        }
        if let Some(&(_, previous_end)) = normalized.last()
            && start < previous_end {
                if end <= previous_end {
                    continue;
                }
                return false;
            }
        normalized.push((start, end));
    }
    *ranges = normalized;
    true
}

fn measure_ranges(xml: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut cursor = 0usize;
    while let Some(relative_start) = xml[cursor..].find("<measure") {
        let start = cursor + relative_start;
        let Some(open_end) = xml[start..]
            .find('>')
            .map(|offset| start + offset + 1)
        else {
            break;
        };
        if !is_element_start(&xml[start..open_end], "measure") {
            cursor = open_end;
            continue;
        }
        let Some(end) = xml[open_end..]
            .find("</measure>")
            .map(|offset| open_end + offset + "</measure>".len())
        else {
            break;
        };
        ranges.push((start, end));
        cursor = end;
    }
    ranges
}

fn tag_attribute_value(tag: &str, name: &str) -> Option<String> {
    for quote in ['\"', '\''] {
        let needle = format!("{name}={quote}");
        let Some(start) = tag.find(&needle).map(|position| position + needle.len()) else {
            continue;
        };
        let end = tag[start..].find(quote)? + start;
        return Some(tag[start..end].to_string());
    }
    None
}

fn is_element_start(opening: &str, name: &str) -> bool {
    let marker = format!("<{name}");
    opening.starts_with(&marker)
        && opening
            .as_bytes()
            .get(marker.len())
            .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b'>' || *byte == b'/')
}

pub fn is_rustig_surface_course_candidate(
    materialization_plan: Option<ClefSurfaceMaterializationPlan>,
    same_measure_f4_g2_overlap: bool,
    f4_readability_penalty: i32,
    g2_readability_penalty: i32,
) -> bool {
    let g2_advantage = f4_readability_penalty.saturating_sub(g2_readability_penalty);
    same_measure_f4_g2_overlap
        && materialization_plan.is_some()
        && g2_advantage > 0
        && g2_advantage <= MAX_MARGINAL_G2_ADVANTAGE
}

pub fn derive_rustig_surface_clef_decision(
    candidates: impl IntoIterator<Item = ClefSurfaceMaterializationPlan>,
) -> RustigClefSurfaceDecision {
    RustigClefSurfaceDecision {
        candidates: candidates.into_iter().collect(),
    }
}

pub fn derive_clef_readability_shadow_course_audit(
    xml: &str,
) -> ClefReadabilityShadowCourseAudit {
    let mut audit = ClefReadabilityShadowCourseAudit {
        primary_materialized_insertions: xml.matches(CLEF_READABILITY_PRIMARY_MARKER).count(),
        ..ClefReadabilityShadowCourseAudit::default()
    };
    for (part_index, (part_start, part_end)) in score_part_ranges(xml).into_iter().enumerate() {
        audit.parts_observed += 1;
        let part = &xml[part_start..part_end];
        let mut observations = Vec::new();
        let pitches = scan_lower_pitches_with_beam_audit(part, Some(&mut observations));
        append_beam_anchor_rows(part, part_index + 1, &observations, &mut audit.beam_anchor_rows);
        let Some(surfaces) = scan_lower_clef_surfaces(part) else {
            continue;
        };
        if surfaces.is_empty() {
            continue;
        }
        if pitches.first().is_some_and(|note| note.start < surfaces[0].end) {
            continue;
        }
        let Some(solution) = solve_readability_course(part, &surfaces, &pitches) else {
            continue;
        };
        let retained = solution.retained;
        let insertions = solution.insertions;
        let readability_cost = solution.cost;
        let source_retained = vec![true; surfaces.len()];
        let Some(source_cost) =
            score_readability_course(part, &surfaces, &pitches, &source_retained)
        else {
            continue;
        };
        audit.parts_planned += 1;
        let omitted_surface_rows = surfaces
            .iter()
            .enumerate()
            .zip(retained.iter())
            .filter(|(_, keep)| !**keep)
            .map(|((index, event), _)| {
                let measure = measure_number_at_position(part, event.start)
                    .unwrap_or_else(|| "?".to_string());
                format!(
                    "Surface {} BarNo {} CurrentClef {} ShadowAction OMIT",
                    index + 1,
                    measure,
                    clef_kind_label(event.kind),
                )
            })
            .collect::<Vec<_>>();
        let inserted_surface_rows = insertions
            .iter()
            .map(|(start, kind)| {
                let measure = measure_number_at_position(part, *start)
                    .unwrap_or_else(|| "?".to_string());
                format!(
                    "BarNo {} TargetClef {} ShadowAction INSERT_BEFORE_ATTACK",
                    measure,
                    clef_kind_label(*kind),
                )
            })
            .collect::<Vec<_>>();
        if !omitted_surface_rows.is_empty() || !inserted_surface_rows.is_empty() {
            audit.changed_parts += 1;
            audit.omitted_surfaces += omitted_surface_rows.len();
            audit.inserted_surfaces += inserted_surface_rows.len();
        }
        audit.part_rows.push(ClefReadabilityPartAudit {
            part_index: part_index + 1,
            source_course: projected_course(&surfaces, &source_retained),
            readability_course: projected_readability_course(
                &surfaces,
                &retained,
                &insertions,
            ),
            source_cost,
            readability_cost,
            source_surfaces: surfaces.len(),
            readability_surfaces: retained.iter().filter(|keep| **keep).count()
                + insertions.len(),
            omitted_surface_rows,
            inserted_surface_rows,
        });
    }
    audit
}

pub fn append_clef_readability_shadow_course_audit_section(
    out: &mut String,
    master_audit: &ClefReadabilityShadowCourseAudit,
    render_artifact_audit: Option<&ClefReadabilityShadowCourseAudit>,
) {
    out.push_str("\nCLEF-READABILITY-PRIMARY-COURSE-POSTWRITE-AUDIT\n");
    out.push_str("----------------------------------------------------------------\n");
    out.push_str("Mode: primary_owner_postwrite_audit; WriteAllowed yes_via_existing_structural_write_authority; WriterAuthorityChanged nein.\n");
    out.push_str("Policy: ledger lines remain graded costs, never a hard maximum. Original ledger, transition, redundancy and short-excursion weights remain unchanged. The complete original closed-anchor course is preserved as an immutable baseline. An extended course retains its choices and may add only proved untied fresh lower heads inside complete mixed-preference primary beams in one current measure/cursor/octave scope. Original pitch/anchor evidence, all chord members, stable time, other lower voices/groups and incoming/outgoing ties remain protected. One shared boundary observation governs anchors, solver, normalization and postwrite safety; pure materialized clefs before the same head are recognized without trusting a marker. Unknown grants no new boundary. Only strict whole-part total-cost improvement replaces the baseline; equal or worse proposals keep it. This is a readability tradeoff, not a four-ledger prohibition.\n");
    out.push_str("ContourDisruptionProjection: short-excursion count only; no independent contour weight is used in this take.\n");
    append_clef_readability_shadow_source(out, "MasterMusicXML", master_audit);
    if let Some(render_artifact_audit) = render_artifact_audit {
        append_clef_readability_shadow_source(
            out,
            "RenderMusicXMLArtifact",
            render_artifact_audit,
        );
    } else {
        out.push_str("Source: RenderMusicXMLArtifact.\n");
        out.push_str("Availability: unavailable; no MasterMusicXML fallback is projected under the render-artifact label.\n");
        out.push_str("Decision: NO-PLAN; Reason render MusicXML artifact was unavailable for independent shadow evaluation.\n");
    }
    out.push_str("Gate: postwrite evidence. Master MusicXML and the independently existing render MusicXML artifact are re-evaluated by the same writer-free readability solver that owns the productive clef course. Materialization uses only the pre-existing Structural Write Authority boundaries. The retired legacy Rustig DP/hysteresis course is no longer evaluated; earlier local clef surfaces remain non-authoritative inputs only.\n");
}

fn append_clef_readability_shadow_source(
    out: &mut String,
    source_label: &str,
    audit: &ClefReadabilityShadowCourseAudit,
) {
    out.push_str(&format!("Source: {source_label}.\n"));
    out.push_str(&format!(
        "Summary: PartsObserved {}; PartsPlanned {}; ChangedParts {}; OmittedCurrentSurfaces {}; NewClefInsertions {}; MaterializationCandidates {}; PrimaryMaterializedInsertions {}.\n",
        audit.parts_observed,
        audit.parts_planned,
        audit.changed_parts,
        audit.omitted_surfaces,
        audit.inserted_surfaces,
        audit.omitted_surfaces + audit.inserted_surfaces,
        audit.primary_materialized_insertions,
    ));
    out.push_str(&format!("BeamAnchorObservations: {}.\n", audit.beam_anchor_rows.len()));
    out.push_str("BeamAnchorEvidence: final-document rescan only; raw anchors from the unchanged pitch scanner, not a prewrite execution trace or an error verdict. Distances are serialized note and measure ordinals, not musical time; none means no anchor. Each visible pitched lower event with a level-1 beam is listed, without truncation.\n");
    for row in &audit.beam_anchor_rows {
        out.push_str(row);
        out.push('\n');
    }
    if audit.part_rows.is_empty() {
        out.push_str("Decision: NO-PLAN; Reason no supported lower-staff F4/G2 course was available for shadow planning.\n");
    }
    for row in &audit.part_rows {
        let changed = !row.omitted_surface_rows.is_empty() || !row.inserted_surface_rows.is_empty();
        out.push_str(&format!(
            "Part {}: CurrentCourse {}; ReadabilityCourse {}; Surfaces {}->{}; LedgerPressure {}->{}; ClefTransitions {}->{}; RedundantClefs {}->{}; ShortExcursions {}->{}; ContourDisruption {}->{}; Decision {}; Reason {}.\n",
            row.part_index,
            row.source_course,
            row.readability_course,
            row.source_surfaces,
            row.readability_surfaces,
            row.source_cost.ledger_pressure,
            row.readability_cost.ledger_pressure,
            row.source_cost.clef_transitions,
            row.readability_cost.clef_transitions,
            row.source_cost.redundant_clefs,
            row.readability_cost.redundant_clefs,
            row.source_cost.short_excursions,
            row.readability_cost.short_excursions,
            yes_no(row.source_cost.short_excursions > 0),
            yes_no(row.readability_cost.short_excursions > 0),
            if changed { "CHANGE" } else { "KEEP" },
            clef_readability_reason(row),
        ));
        for detail in row
            .omitted_surface_rows
            .iter()
            .chain(row.inserted_surface_rows.iter())
            .take(12)
        {
            out.push_str("  ");
            out.push_str(detail);
            out.push_str(".\n");
        }
        let additional = row.omitted_surface_rows.len() + row.inserted_surface_rows.len();
        if additional > 12 {
            out.push_str(&format!(
                "  AdditionalCourseActions {}.\n",
                additional - 12,
            ));
        }
    }
}

fn solve_readability_course(
    part: &str,
    surfaces: &[SurfaceEvent],
    pitches: &[PitchedEvent],
) -> Option<ClefReadabilityCourseSolution> {
    let original_boundaries = BTreeSet::new();
    let original_pitches = scan_lower_pitches_with_boundary_policy(part, None, &original_boundaries);
    let baseline = solve_readability_course_with_boundaries(part, surfaces, &original_pitches, &original_boundaries)?;
    let boundaries = beam_boundary::observe(part);
    if boundaries.is_empty() {
        return Some(baseline);
    }
    let expanded = solve_readability_course_with_boundaries(part, surfaces, pitches, &boundaries);
    match expanded {
        Some(expanded) if expanded.cost.total() < baseline.cost.total() => Some(expanded),
        _ => Some(baseline),
    }
}

fn collect_readability_course_candidates(
    part: &str,
    pitches: &[PitchedEvent],
    boundaries: &BTreeSet<usize>,
) -> Vec<ClefReadabilityTransitionCandidate> {
    let attacks = collect_lower_staff_attacks(pitches);
    let mut candidates = collect_readability_transition_candidates(&attacks);
    if !boundaries.is_empty() {
        let original = scan_lower_pitches_with_boundary_policy(part, None, &BTreeSet::new());
        for candidate in collect_readability_transition_candidates(&collect_lower_staff_attacks(&original)) {
            if !candidates.iter().any(|present| present.start == candidate.start && present.kind == candidate.kind) {
                candidates.push(candidate);
            }
        }
        candidates.sort_by_key(|candidate| (candidate.start, candidate.kind));
    }
    candidates
}

fn solve_readability_course_with_boundaries(
    part: &str,
    surfaces: &[SurfaceEvent],
    pitches: &[PitchedEvent],
    boundaries: &BTreeSet<usize>,
) -> Option<ClefReadabilityCourseSolution> {
    let first = surfaces.first()?;
    let candidates = collect_readability_course_candidates(part, pitches, boundaries);
    let mut timeline = Vec::with_capacity(surfaces.len().saturating_sub(1) + candidates.len());
    for (index, event) in surfaces.iter().enumerate().skip(1) {
        timeline.push((event.start, 0u8, ClefReadabilityTimelineEvent::Surface(index)));
    }
    for (index, candidate) in candidates.iter().enumerate() {
        if candidate.start >= first.end {
            timeline.push((
                candidate.start,
                1u8,
                ClefReadabilityTimelineEvent::Candidate(index),
            ));
        }
    }
    timeline.sort_by_key(|(start, rank, _)| (*start, *rank));

    let first_interval_end = timeline.first().map_or(part.len(), |(start, _, _)| *start);
    let mut first_cost = ClefReadabilityCost::default();
    first_cost.add_passage(passage_cost(
        pitches,
        first.end,
        first_interval_end,
        first.kind,
    ));
    let mut first_state = ClefReadabilityState {
        active: first.kind,
        excursion_origin: None,
        excursion_attacks: 0,
    };
    add_readability_passage_attacks(
        &mut first_state,
        pitches,
        first.end,
        first_interval_end,
    );
    let mut frontier = BTreeMap::from([(
        first_state,
        ClefReadabilityCandidate {
            cost: first_cost,
            visible_events: if first.visible { 1 } else { 0 },
            retained: vec![true],
            insertions: Vec::new(),
        },
    )]);

    for (timeline_index, (_, _, timeline_event)) in timeline.iter().enumerate() {
        let interval_end = timeline
            .get(timeline_index + 1)
            .map_or(part.len(), |(start, _, _)| *start);
        let mut next_frontier = BTreeMap::new();
        match *timeline_event {
            ClefReadabilityTimelineEvent::Surface(index) => {
                let event = &surfaces[index];
                let tie_open_at_surface =
                    event.visible && lower_tie_open_at_position(part, event.start);
                let beam_open_at_surface = event.visible
                    && lower_primary_beam_group_contains_position(part, event.start)
                    && !boundaries.contains(&event.start);
                let octave_shift_boundary_surface = event.visible
                    && lower_octave_shift_boundary_precedes_position(part, event.start);
                let boundary_closed =
                    (tie_open_at_surface || beam_open_at_surface) && !octave_shift_boundary_surface;
                if boundary_closed && !event.removable {
                    return None;
                }
                for (state, candidate) in &frontier {
                    if event.visible && event.removable {
                        let mut omitted_state = *state;
                        let mut omitted = candidate.clone();
                        omitted.cost.add_passage(passage_cost(
                            pitches,
                            event.end,
                            interval_end,
                            omitted_state.active,
                        ));
                        add_readability_passage_attacks(
                            &mut omitted_state,
                            pitches,
                            event.end,
                            interval_end,
                        );
                        omitted.retained.push(false);
                        retain_best_readability(&mut next_frontier, omitted_state, omitted);
                    }

                    if !boundary_closed {
                        let mut selected_state = *state;
                        let mut selected = candidate.clone();
                        account_readability_surface(
                            &mut selected_state,
                            &mut selected.cost,
                            event,
                        );
                        if event.visible {
                            selected.visible_events += 1;
                        }
                        selected.cost.add_passage(passage_cost(
                            pitches,
                            event.end,
                            interval_end,
                            selected_state.active,
                        ));
                        add_readability_passage_attacks(
                            &mut selected_state,
                            pitches,
                            event.end,
                            interval_end,
                        );
                        selected.retained.push(true);
                        retain_best_readability(&mut next_frontier, selected_state, selected);
                    }
                }
            }
            ClefReadabilityTimelineEvent::Candidate(index) => {
                let transition = candidates[index];
                let tie_open_at_transition =
                    lower_tie_open_at_position(part, transition.start);
                let beam_open_at_transition =
                    lower_primary_beam_insertion_blocked(part, transition.start)
                        && !boundaries.contains(&transition.start);
                let octave_shift_boundary_transition =
                    lower_octave_shift_boundary_precedes_position(part, transition.start);
                let boundary_closed = (tie_open_at_transition || beam_open_at_transition)
                    && !octave_shift_boundary_transition;
                for (state, candidate) in &frontier {
                    let mut omitted_state = *state;
                    let mut omitted = candidate.clone();
                    omitted.cost.add_passage(passage_cost(
                        pitches,
                        transition.start,
                        interval_end,
                        omitted_state.active,
                    ));
                    add_readability_passage_attacks(
                        &mut omitted_state,
                        pitches,
                        transition.start,
                        interval_end,
                    );
                    retain_best_readability(&mut next_frontier, omitted_state, omitted);

                    if !boundary_closed && transition.kind != state.active {
                        let mut selected_state = *state;
                        let mut selected = candidate.clone();
                        account_readability_transition(
                            &mut selected_state,
                            &mut selected.cost,
                            transition.kind,
                        );
                        selected.visible_events += 1;
                        selected.insertions.push((transition.start, transition.kind));
                        selected.cost.add_passage(passage_cost(
                            pitches,
                            transition.start,
                            interval_end,
                            selected_state.active,
                        ));
                        add_readability_passage_attacks(
                            &mut selected_state,
                            pitches,
                            transition.start,
                            interval_end,
                        );
                        retain_best_readability(&mut next_frontier, selected_state, selected);
                    }
                }
            }
        }
        frontier = next_frontier;
    }

    let best = frontier.into_values().min_by(|left, right| {
        (
            left.cost.total(),
            left.visible_events,
            left.cost.short_excursions,
            left.cost.clef_transitions,
            left.cost.ledger_pressure,
        )
            .cmp(&(
                right.cost.total(),
                right.visible_events,
                right.cost.short_excursions,
                right.cost.clef_transitions,
                right.cost.ledger_pressure,
            ))
    })?;
    if best.retained.len() != surfaces.len() {
        return None;
    }
    let mut retained = best.retained;
    let mut insertions = best.insertions;
    canonicalize_primary_materialized_replacements(
        part,
        surfaces,
        &mut retained,
        &mut insertions,
    );
    Some(ClefReadabilityCourseSolution {
        retained,
        insertions,
        cost: best.cost,
    })
}

fn canonicalize_primary_materialized_replacements(
    part: &str,
    surfaces: &[SurfaceEvent],
    retained: &mut [bool],
    insertions: &mut Vec<(usize, ClefKind)>,
) {
    for (index, event) in surfaces.iter().enumerate() {
        if retained.get(index).copied().unwrap_or(true)
            || !part
                .get(event.start..event.end)
                .is_some_and(|surface| surface.contains(CLEF_READABILITY_PRIMARY_MARKER))
        {
            continue;
        }
        let Some(insertion_index) = insertions.iter().position(|(start, kind)| {
            *kind == event.kind
                && *start >= event.end
                && part
                    .get(event.end..*start)
                    .is_some_and(|between| between.trim().is_empty())
        }) else {
            continue;
        };
        retained[index] = true;
        insertions.remove(insertion_index);
    }
}

fn collect_lower_staff_attacks(pitches: &[PitchedEvent]) -> Vec<AttackEvent> {
    let mut attacks = Vec::<AttackEvent>::new();
    for pitch in pitches {
        if pitch.starts_attack || attacks.is_empty() {
            attacks.push(AttackEvent {
                pitches: vec![pitch.pitch],
                clef_transition_anchor: pitch.clef_transition_anchor,
            });
        } else if let Some(attack) = attacks.last_mut() {
            attack.pitches.push(pitch.pitch);
            attack.clef_transition_anchor = match (
                attack.clef_transition_anchor,
                pitch.clef_transition_anchor,
            ) {
                (Some(left), Some(right)) => Some(left.min(right)),
                _ => None,
            };
        }
    }
    attacks
}

fn collect_readability_transition_candidates(
    attacks: &[AttackEvent],
) -> Vec<ClefReadabilityTransitionCandidate> {
    let mut anchor_groups = Vec::<(usize, i64, i64)>::new();
    let mut anchor_group_indexes = BTreeMap::<usize, usize>::new();
    for attack in attacks {
        let Some(start) = attack.clef_transition_anchor else {
            continue;
        };
        let group_index = if let Some(index) = anchor_group_indexes.get(&start) {
            *index
        } else {
            let index = anchor_groups.len();
            anchor_groups.push((start, 0, 0));
            anchor_group_indexes.insert(start, index);
            index
        };
        let group = &mut anchor_groups[group_index];
        group.1 += attack_pressure(attack, ClefKind::Bass);
        group.2 += attack_pressure(attack, ClefKind::Treble);
    }

    let mut candidates = Vec::new();
    let mut previous_preference = None;
    for (start, bass_pressure, treble_pressure) in anchor_groups {
        let preference = if bass_pressure < treble_pressure {
            Some(ClefKind::Bass)
        } else if treble_pressure < bass_pressure {
            Some(ClefKind::Treble)
        } else {
            None
        };
        let Some(preference) = preference else {
            continue;
        };
        if previous_preference == Some(preference) {
            continue;
        }
        candidates.push(ClefReadabilityTransitionCandidate {
            start,
            kind: preference,
        });
        previous_preference = Some(preference);
    }
    candidates
}

fn attack_pressure(attack: &AttackEvent, kind: ClefKind) -> i64 {
    attack
        .pitches
        .iter()
        .map(|pitch| kind.ledger_pressure(*pitch))
        .sum()
}

fn account_readability_transition(
    state: &mut ClefReadabilityState,
    cost: &mut ClefReadabilityCost,
    kind: ClefKind,
) {
    if kind == state.active {
        return;
    }
    cost.clef_transitions += 1;
    if state.excursion_origin == Some(kind)
        && state.excursion_attacks <= READABILITY_SHORT_EXCURSION_MAX_ATTACKS
    {
        cost.short_excursions += 1;
        state.excursion_origin = None;
        state.excursion_attacks = 0;
    } else {
        state.excursion_origin = Some(state.active);
        state.excursion_attacks = 0;
    }
    state.active = kind;
}

fn retain_best_readability(
    frontier: &mut BTreeMap<ClefReadabilityState, ClefReadabilityCandidate>,
    state: ClefReadabilityState,
    candidate: ClefReadabilityCandidate,
) {
    match frontier.get(&state) {
        Some(current) if !candidate.precedes(current) => {}
        _ => {
            frontier.insert(state, candidate);
        }
    }
}

fn projected_readability_course(
    surfaces: &[SurfaceEvent],
    retained: &[bool],
    insertions: &[(usize, ClefKind)],
) -> String {
    let mut events = surfaces
        .iter()
        .zip(retained.iter())
        .filter_map(|(event, keep)| keep.then_some((event.start, 0u8, event.kind)))
        .collect::<Vec<_>>();
    events.extend(
        insertions
            .iter()
            .map(|(start, kind)| (*start, 1u8, *kind)),
    );
    events.sort_by_key(|(start, rank, _)| (*start, *rank));
    let labels = events
        .into_iter()
        .map(|(_, _, kind)| clef_kind_label(kind))
        .collect::<Vec<_>>();
    const MAX_PROJECTED_SURFACES: usize = 24;
    if labels.len() <= MAX_PROJECTED_SURFACES {
        labels.join(">")
    } else {
        format!(
            "{}>...(+{})",
            labels[..MAX_PROJECTED_SURFACES].join(">"),
            labels.len() - MAX_PROJECTED_SURFACES,
        )
    }
}

fn score_readability_course(
    part: &str,
    surfaces: &[SurfaceEvent],
    pitches: &[PitchedEvent],
    retained: &[bool],
) -> Option<ClefReadabilityCost> {
    if surfaces.is_empty() || surfaces.len() != retained.len() || !retained[0] {
        return None;
    }
    let first = &surfaces[0];
    let first_interval_end = surfaces.get(1).map_or(part.len(), |event| event.start);
    let mut state = ClefReadabilityState {
        active: first.kind,
        excursion_origin: None,
        excursion_attacks: 0,
    };
    let mut cost = ClefReadabilityCost::default();
    cost.add_passage(passage_cost(
        pitches,
        first.end,
        first_interval_end,
        state.active,
    ));
    add_readability_passage_attacks(
        &mut state,
        pitches,
        first.end,
        first_interval_end,
    );

    for (index, (event, keep)) in surfaces.iter().zip(retained.iter()).enumerate().skip(1) {
        let interval_end = surfaces
            .get(index + 1)
            .map_or(part.len(), |next| next.start);
        if *keep {
            account_readability_surface(&mut state, &mut cost, event);
        } else if !event.visible {
            return None;
        }
        cost.add_passage(passage_cost(
            pitches,
            event.end,
            interval_end,
            state.active,
        ));
        add_readability_passage_attacks(&mut state, pitches, event.end, interval_end);
    }
    Some(cost)
}

fn account_readability_surface(
    state: &mut ClefReadabilityState,
    cost: &mut ClefReadabilityCost,
    event: &SurfaceEvent,
) {
    if !event.visible {
        state.active = event.kind;
        state.excursion_origin = None;
        state.excursion_attacks = 0;
        return;
    }
    if event.kind == state.active {
        cost.redundant_clefs += 1;
        return;
    }
    account_readability_transition(state, cost, event.kind);
}

fn add_readability_passage_attacks(
    state: &mut ClefReadabilityState,
    pitches: &[PitchedEvent],
    start: usize,
    end: usize,
) {
    if state.excursion_origin.is_none() {
        return;
    }
    let attacks = passage_attack_count(pitches, start, end);
    state.excursion_attacks = state
        .excursion_attacks
        .saturating_add(attacks)
        .min(READABILITY_SHORT_EXCURSION_MAX_ATTACKS + 1);
}

fn passage_attack_count(notes: &[PitchedEvent], start: usize, end: usize) -> usize {
    notes
        .iter()
        .filter(|note| note.start >= start && note.start < end && note.counted_attack)
        .count()
}

fn projected_course(surfaces: &[SurfaceEvent], retained: &[bool]) -> String {
    let labels = surfaces
        .iter()
        .zip(retained.iter())
        .filter_map(|(event, keep)| keep.then_some(clef_kind_label(event.kind)))
        .collect::<Vec<_>>();
    const MAX_PROJECTED_SURFACES: usize = 24;
    if labels.len() <= MAX_PROJECTED_SURFACES {
        return labels.join(">");
    }
    format!(
        "{}>...(+{})",
        labels[..MAX_PROJECTED_SURFACES].join(">"),
        labels.len() - MAX_PROJECTED_SURFACES,
    )
}

const CLEF_READABILITY_PRIMARY_MARKER: &str =
    "etuschlopf-clef-readability-primary-course";

fn clef_readability_primary_clef_block(kind: ClefKind) -> &'static str {
    match kind {
        ClefKind::Treble => {
            "<attributes><!-- etuschlopf-clef-readability-primary-course --><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>"
        }
        ClefKind::Bass => {
            "<attributes><!-- etuschlopf-clef-readability-primary-course --><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>"
        }
    }
}

fn clef_kind_label(kind: ClefKind) -> &'static str {
    match kind {
        ClefKind::Treble => "G2",
        ClefKind::Bass => "F4",
    }
}

fn measure_number_at_position(part: &str, position: usize) -> Option<String> {
    for (start, end) in measure_ranges(part) {
        if position < start || position >= end {
            continue;
        }
        let opening_end = part[start..].find('>').map(|offset| start + offset + 1)?;
        return tag_attribute_value(&part[start..opening_end], "number");
    }
    None
}

fn clef_readability_reason(row: &ClefReadabilityPartAudit) -> &'static str {
    if row.omitted_surface_rows.is_empty() && row.inserted_surface_rows.is_empty() {
        return "current_course_already_preferred";
    }
    if row.readability_cost.short_excursions < row.source_cost.short_excursions
        && row.readability_cost.ledger_pressure >= row.source_cost.ledger_pressure
    {
        return "short_excursion_reduced_with_bounded_ledger_tradeoff";
    }
    if row.readability_cost.ledger_pressure < row.source_cost.ledger_pressure
        && row.readability_cost.clef_transitions <= row.source_cost.clef_transitions
    {
        return "ledger_pressure_and_transition_course_improved";
    }
    if row.readability_cost.clef_transitions < row.source_cost.clef_transitions {
        return "clef_transition_course_simplified";
    }
    "readability_cost_course_improved"
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn passage_cost(notes: &[PitchedEvent], start: usize, end: usize, kind: ClefKind) -> i64 {
    notes
        .iter()
        .filter(|note| note.start >= start && note.start < end)
        .map(|note| kind.ledger_pressure(note.pitch))
        .sum()
}

fn scan_lower_clef_surfaces(part: &str) -> Option<Vec<SurfaceEvent>> {
    let mut events = Vec::new();
    let mut cursor = 0usize;
    while let Some(relative) = part[cursor..].find("<attributes") {
        let start = cursor + relative;
        let end = part[start..]
            .find("</attributes>")
            .map(|offset| start + offset + "</attributes>".len())?;
        let attributes = &part[start..end];
        match read_lower_clef(attributes) {
            ClefSurface::None => {}
            ClefSurface::Known { kind, visible } => events.push(SurfaceEvent {
                start,
                end,
                kind,
                visible,
                removable: visible && contains_only_one_lower_clef(attributes),
            }),
            ClefSurface::Unsupported => return None,
        }
        cursor = end;
    }
    Some(events)
}

fn read_lower_clef(attributes: &str) -> ClefSurface {
    let mut found = None;
    let mut cursor = 0usize;
    while let Some(relative) = attributes[cursor..].find("<clef") {
        let start = cursor + relative;
        let Some(open_end) = attributes[start..]
            .find('>')
            .map(|offset| start + offset + 1)
        else {
            return ClefSurface::Unsupported;
        };
        let Some(end) = attributes[open_end..]
            .find("</clef>")
            .map(|offset| open_end + offset + "</clef>".len())
        else {
            return ClefSurface::Unsupported;
        };
        let opening = &attributes[start..open_end];
        if opening.contains("number=\"2\"") || opening.contains("number='2'") {
            let clef = &attributes[start..end];
            let kind = if clef.contains("<sign>G</sign>") && clef.contains("<line>2</line>") {
                ClefKind::Treble
            } else if clef.contains("<sign>F</sign>") && clef.contains("<line>4</line>") {
                ClefKind::Bass
            } else {
                return ClefSurface::Unsupported;
            };
            let visible = !opening.contains("print-object=\"no\"")
                && !opening.contains("print-object='no'");
            found = Some((kind, visible));
        }
        cursor = end;
    }
    found.map_or(ClefSurface::None, |(kind, visible)| {
        ClefSurface::Known { kind, visible }
    })
}

fn contains_only_one_lower_clef(attributes: &str) -> bool {
    let Some(inner_start) = attributes.find('>').map(|index| index + 1) else {
        return false;
    };
    let Some(inner_end) = attributes.rfind("</attributes>") else {
        return false;
    };
    let inner = &attributes[inner_start..inner_end];
    let mut residue = String::with_capacity(inner.len());
    let mut cursor = 0usize;
    let mut lower_clefs = 0usize;

    while let Some(relative) = inner[cursor..].find("<clef") {
        let start = cursor + relative;
        residue.push_str(&inner[cursor..start]);
        let Some(open_end) = inner[start..]
            .find('>')
            .map(|offset| start + offset + 1)
        else {
            return false;
        };
        let opening = &inner[start..open_end];
        if !opening.contains("number=\"2\"") && !opening.contains("number='2'") {
            return false;
        }
        let Some(end) = inner[open_end..]
            .find("</clef>")
            .map(|offset| open_end + offset + "</clef>".len())
        else {
            return false;
        };
        lower_clefs += 1;
        cursor = end;
    }
    residue.push_str(&inner[cursor..]);
    lower_clefs == 1 && comments_removed(&residue).is_some_and(|text| text.trim().is_empty())
}

#[derive(Debug, Clone, Copy)]
struct BeamAnchorObservation {
    start: usize,
    state: PrimaryBeamState,
    self_anchor: Option<usize>,
    tie_anchor: Option<usize>,
    beam_anchor: Option<usize>,
    stored_beam_origin: Option<usize>,
    effective_anchor: Option<usize>,
}

// The productive caller collects no diagnostics. The final-document audit observes
// the same state at its point of use, without changing any anchor or return value.
fn scan_lower_pitches(
    part: &str,
    observations: Option<&mut Vec<BeamAnchorObservation>>,
) -> Vec<PitchedEvent> {
    scan_lower_pitches_with_beam_audit(part, observations)
}

fn scan_lower_pitches_with_beam_audit(
    part: &str,
    observations: Option<&mut Vec<BeamAnchorObservation>>,
) -> Vec<PitchedEvent> {
    let boundaries = beam_boundary::observe(part);
    scan_lower_pitches_with_boundary_policy(part, observations, &boundaries)
}

fn scan_lower_pitches_with_boundary_policy(
    part: &str,
    mut observations: Option<&mut Vec<BeamAnchorObservation>>,
    boundaries: &BTreeSet<usize>,
) -> Vec<PitchedEvent> {
    let shifts = scan_octave_events(part);
    let mut shift_index = 0usize;
    let mut active_shift = 0i64;
    let mut open_ties = BTreeMap::<String, usize>::new();
    let mut open_primary_beams = BTreeMap::<String, usize>::new();
    let mut notes = Vec::new();

    let mut chord = LowerChordObservation::default();
    let mut previous_end = 0;
    for (start, end, note) in note_ranges(part) {
        let visible = !note.contains("print-object=\"no\"") && !note.contains("print-object='no'");
        let lower_pitch = (visible
            && first_tag_text(note, "staff").as_deref() == Some("2")
            && !note.contains("<rest"))
        .then(|| observe_pitch(note))
        .flatten();
        let relation = chord.observe(
            &part[previous_end..start],
            start,
            note,
            lower_pitch.is_some(),
        );
        previous_end = end;
        let is_member = note.contains("<chord");
        let self_anchor = relation.head();
        while shift_index < shifts.len() && shifts[shift_index].0 < start {
            active_shift = shifts[shift_index].1;
            open_ties.clear();
            open_primary_beams.clear();
            shift_index += 1;
        }
        if !visible {
            continue;
        }

        let voice = first_tag_text(note, "voice");
        let starts_attack = !matches!(relation, LowerChordAttack::Member { .. });
        let tie_identity = if lower_pitch.is_some() {
            lower_tie_identity(note)
        } else {
            None
        };
        let tie_stop = lower_pitch.is_some() && note_has_tie_type(note, "stop");
        let tie_start = lower_pitch.is_some() && note_has_tie_type(note, "start");
        let continuation_origin = if tie_stop && self_anchor.is_some() {
            tie_identity
                .as_ref()
                .and_then(|key| open_ties.get(key).copied())
        } else {
            None
        };
        let tie_transition_anchor = if lower_pitch.is_some() {
            if tie_stop {
                continuation_origin
            } else {
                self_anchor
            }
        } else {
            None
        };

        let beam_state = primary_beam_state(note);
        let beam_transition_anchor = match (voice.as_ref(), beam_state) {
            (Some(voice), Some(PrimaryBeamState::Begin)) => {
                let beam_origin = if lower_pitch.is_some() && tie_stop {
                    continuation_origin
                } else {
                    self_anchor
                };
                if let Some(origin) = beam_origin {
                    open_primary_beams.insert(voice.clone(), origin);
                } else {
                    open_primary_beams.remove(voice);
                }
                beam_origin
            }
            (Some(voice), Some(PrimaryBeamState::Continue | PrimaryBeamState::End)) => {
                open_primary_beams.get(voice).copied()
            }
            (_, Some(PrimaryBeamState::Hook)) => self_anchor,
            (Some(voice), None) => {
                if !is_member {
                    open_primary_beams.remove(voice);
                }
                self_anchor
            }
            (None, _) => None,
        };

        let beam_transition_anchor = self_anchor.and(beam_transition_anchor);

        if let Some(pitch) = lower_pitch {
            let mut clef_transition_anchor = match (tie_transition_anchor, beam_transition_anchor) {
                (Some(tie_anchor), Some(beam_anchor)) => Some(tie_anchor.min(beam_anchor)),
                _ => None,
            };
            if clef_transition_anchor.is_some()
                && self_anchor.is_some_and(|head| boundaries.contains(&head))
            {
                clef_transition_anchor = self_anchor;
            }

            if let (Some(state), Some(rows)) = (beam_state, observations.as_deref_mut()) {
                rows.push(BeamAnchorObservation {
                    start,
                    state,
                    self_anchor,
                    tie_anchor: tie_transition_anchor,
                    beam_anchor: beam_transition_anchor,
                    stored_beam_origin: voice.as_ref().and_then(|voice| open_primary_beams.get(voice)).copied(),
                    effective_anchor: clef_transition_anchor,
                });
            }

            if let Some(key) = tie_identity {
                if self_anchor.is_none() {
                    if tie_start || tie_stop {
                        open_ties.remove(&key);
                    }
                } else {
                    match (tie_stop, tie_start) {
                        (true, true) => {
                            open_ties.insert(
                                key,
                                clef_transition_anchor
                                    .or(continuation_origin)
                                    .unwrap_or(self_anchor.unwrap_or(start)),
                            );
                        }
                        (true, false) => {
                            open_ties.remove(&key);
                        }
                        (false, true) => {
                            open_ties.insert(
                                key,
                                clef_transition_anchor.unwrap_or(self_anchor.unwrap_or(start)),
                            );
                        }
                        (false, false) => {}
                    }
                }
            }

            notes.push(PitchedEvent {
                start,
                pitch: pitch.shifted(active_shift),
                starts_attack,
                counted_attack: relation.counted(),
                clef_transition_anchor,
            });
        }

        if beam_state == Some(PrimaryBeamState::End)
            && let Some(voice) = voice
        {
            open_primary_beams.remove(&voice);
        }
    }
    notes
}

fn append_beam_anchor_rows(
    part: &str,
    part_index: usize,
    observations: &[BeamAnchorObservation],
    rows: &mut Vec<String>,
) {
    let notes = note_ranges(part);
    let measures = measure_ranges(part);
    let document = roxmltree::Document::parse(part);
    let boundaries = document.as_ref().ok().map(|document| {
        document.descendants().filter(|node| node.is_element()).filter_map(|node| {
            match node.tag_name().name() {
                "backup" => Some((node.range().start, 0)),
                "forward" => Some((node.range().start, 1)),
                "attributes" => Some((node.range().start, 2)),
                _ => None,
            }
        }).collect::<Vec<_>>()
    });
    let note_ordinal = |position: Option<usize>| -> String {
        position.and_then(|position| notes.binary_search_by_key(&position, |&(start, _, _)| start).ok())
            .map(|index| (index + 1).to_string()).unwrap_or_else(|| "none".to_string())
    };
    let location = |position: Option<usize>| -> String {
        let Some(position) = position else {
            return "none".to_string();
        };
        let Ok(index) = notes.binary_search_by_key(&position, |&(start, _, _)| start) else {
            return format!("unmapped_byte_{position}");
        };
        format!("note_{}/bar_{:?}/measure_ordinal_{}/voice_{:?}/staff_{:?}",
            index + 1, measure_number_at_position(part, position),
            measures.partition_point(|&(start, _)| start <= position),
            first_tag_text(notes[index].2, "voice"), first_tag_text(notes[index].2, "staff"))
    };
    for observation in observations {
        let distance = observation.stored_beam_origin.map(|anchor| {
            let here = notes.partition_point(|&(start, _, _)| start < observation.start);
            let origin = notes.partition_point(|&(start, _, _)| start < anchor);
            let here_bar = measures.partition_point(|&(start, _)| start <= observation.start);
            let origin_bar = measures.partition_point(|&(start, _)| start <= anchor);
            format!("notes_{};measures_{};bytes_{}", here.saturating_sub(origin),
                here_bar.saturating_sub(origin_bar), observation.start.saturating_sub(anchor))
        }).unwrap_or_else(|| "none".to_string());
        let gaps = observation.stored_beam_origin.and_then(|origin| {
            let index = notes.binary_search_by_key(&origin, |&(start, _, _)| start).ok()?;
            let end = notes[index].1;
            let boundaries = boundaries.as_ref()?;
            let mut counts = [0usize; 3];
            for &(_, kind) in boundaries.iter().filter(|&&(start, _)| end <= start && start < observation.start) {
                counts[kind] += 1;
            }
            Some(format!("backup_{};forward_{};attributes_{}", counts[0], counts[1], counts[2]))
        }).unwrap_or_else(|| "unknown".to_string());
        rows.push(format!(
            "  BeamAnchor Part {part_index}; Event {}; Level1 {:?}; ChordHeadNote {}; TieNote {}; StoredBeam {}; BeamNote {}; EffectiveNote {}; StoredDistance {}; BetweenXML {}.",
            location(Some(observation.start)), observation.state,
            note_ordinal(observation.self_anchor), note_ordinal(observation.tie_anchor),
            location(observation.stored_beam_origin), note_ordinal(observation.beam_anchor),
            note_ordinal(observation.effective_anchor), distance, gaps,
        ));
    }
}

fn lower_tie_identity(note: &str) -> Option<String> {
    let voice = first_tag_text(note, "voice")?;
    let step = first_tag_text(note, "step")?;
    let alter = first_tag_text(note, "alter").unwrap_or_default();
    let octave = first_tag_text(note, "octave")?;
    Some(format!("{voice}|{step}|{alter}|{octave}"))
}

fn note_has_tie_type(note: &str, expected: &str) -> bool {
    let mut cursor = 0usize;
    while let Some(relative) = note[cursor..].find("<tie") {
        let start = cursor + relative;
        let Some(open_end) = note[start..]
            .find('>')
            .map(|offset| start + offset + 1)
        else {
            return false;
        };
        let opening = &note[start..open_end];
        if (is_element_start(opening, "tie") || is_element_start(opening, "tied"))
            && tag_attribute_value(opening, "type").as_deref() == Some(expected)
        {
            return true;
        }
        cursor = open_end;
    }
    false
}

fn primary_beam_state(note: &str) -> Option<PrimaryBeamState> {
    let mut cursor = 0usize;
    while let Some(relative) = note[cursor..].find("<beam") {
        let start = cursor + relative;
        let open_end = note[start..].find('>').map(|offset| start + offset + 1)?;
        let opening = &note[start..open_end];
        if !is_element_start(opening, "beam") {
            cursor = open_end;
            continue;
        }
        let close = note[open_end..]
            .find("</beam>")
            .map(|offset| open_end + offset)?;
        if tag_attribute_value(opening, "number").as_deref() == Some("1") {
            return match note[open_end..close].trim() {
                "begin" => Some(PrimaryBeamState::Begin),
                "continue" => Some(PrimaryBeamState::Continue),
                "end" => Some(PrimaryBeamState::End),
                "forward hook" | "backward hook" => Some(PrimaryBeamState::Hook),
                _ => None,
            };
        }
        cursor = close + "</beam>".len();
    }
    None
}

fn lower_clef_surface_inside_primary_beam_group(xml: &str) -> bool {
    for (part_start, part_end) in score_part_ranges(xml) {
        let part = &xml[part_start..part_end];
        let boundaries = beam_boundary::observe(part);
        let Some(surfaces) = scan_lower_clef_surfaces(part) else {
            continue;
        };
        if surfaces.iter().skip(1).any(|surface| {
            surface.visible
                && !lower_octave_shift_boundary_precedes_position(part, surface.start)
                && lower_primary_beam_group_contains_position(part, surface.start)
                && !boundaries.contains(&surface.start)
        }) {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum PrimaryBeamPosition {
    Outside,
    Interior,
    UnresolvedPrefix,
}

fn lower_primary_beam_group_contains_position(xml: &str, position: usize) -> bool {
    lower_primary_beam_position(xml, position) == PrimaryBeamPosition::Interior
}

fn lower_primary_beam_insertion_blocked(xml: &str, position: usize) -> bool {
    lower_primary_beam_position(xml, position) != PrimaryBeamPosition::Outside
}

fn lower_primary_beam_position(xml: &str, position: usize) -> PrimaryBeamPosition {
    #[derive(Debug, Clone, Copy)]
    struct OpenBeam {
        start: usize,
        touches_lower: bool,
    }

    let Some((part_start, part_end)) = score_part_ranges(xml)
        .into_iter()
        .find(|&(start, end)| start <= position && position < end)
    else {
        return PrimaryBeamPosition::Outside;
    };
    let xml = &xml[part_start..part_end];
    let position = position - part_start;
    let boundaries = scan_octave_events(xml);
    let first_future_boundary = boundaries.partition_point(|&(start, _)| start < position);
    let scope_start = first_future_boundary.checked_sub(1).map_or(0, |index| boundaries[index].0);
    let scope_end = boundaries.get(first_future_boundary).map(|&(start, _)| start);
    let mut passed_future_boundary = false;
    let mut unresolved_prefix = false;
    let mut open = BTreeMap::<String, OpenBeam>::new();
    for (start, end, note) in note_ranges(xml) {
        if start < scope_start {
            continue;
        }
        // Preserve a proven old-scope interior even if its end is serialized
        // beyond a later boundary. An unclosed prefix is not an interior proof.
        if !passed_future_boundary && scope_end.is_some_and(|boundary| start >= boundary) {
            unresolved_prefix = open.values().any(|group| group.touches_lower && group.start < position);
            passed_future_boundary = true;
        }
        if start >= position && open.is_empty() {
            break;
        }
        let visible = !note.contains("print-object=\"no\"")
            && !note.contains("print-object='no'");
        if !visible {
            continue;
        }
        let Some(voice) = first_tag_text(note, "voice") else {
            continue;
        };
        let touches_lower = first_tag_text(note, "staff").as_deref() == Some("2");
        match primary_beam_state(note) {
            Some(PrimaryBeamState::Begin) => {
                open.insert(
                    voice,
                    OpenBeam {
                        start,
                        touches_lower,
                    },
                );
            }
            Some(PrimaryBeamState::Continue) => {
                if let Some(group) = open.get_mut(&voice) {
                    group.touches_lower |= touches_lower;
                }
            }
            Some(PrimaryBeamState::End) => {
                if let Some(mut group) = open.remove(&voice) {
                    group.touches_lower |= touches_lower;
                    if group.touches_lower && group.start < position && position < end {
                        return PrimaryBeamPosition::Interior;
                    }
                }
            }
            Some(PrimaryBeamState::Hook) => {}
            None => {
                if !note.contains("<chord") {
                    open.remove(&voice);
                }
            }
        }
    }
    if unresolved_prefix
        || (scope_end.is_some() && open.values().any(|group| group.touches_lower && group.start < position))
    {
        PrimaryBeamPosition::UnresolvedPrefix
    } else {
        PrimaryBeamPosition::Outside
    }
}

fn lower_clef_surface_inside_open_tie(xml: &str) -> bool {
    for (part_start, part_end) in score_part_ranges(xml) {
        let part = &xml[part_start..part_end];
        let Some(surfaces) = scan_lower_clef_surfaces(part) else {
            continue;
        };
        if surfaces.iter().skip(1).any(|surface| {
            surface.visible
                && !lower_octave_shift_boundary_precedes_position(part, surface.start)
                && lower_tie_open_at_position(part, surface.start)
        }) {
            return true;
        }
    }
    false
}

fn lower_tie_open_at_position(xml: &str, position: usize) -> bool {
    let mut open_ties = BTreeSet::<String>::new();
    for (start, _, note) in note_ranges(xml) {
        if start >= position {
            break;
        }
        if first_tag_text(note, "staff").as_deref() != Some("2") || note.contains("<rest") {
            continue;
        }
        let Some(key) = lower_tie_identity(note) else {
            continue;
        };
        if note_has_tie_type(note, "stop") {
            open_ties.remove(&key);
        }
        if note_has_tie_type(note, "start") {
            open_ties.insert(key);
        }
    }
    !open_ties.is_empty()
}

fn lower_octave_shift_boundary_precedes_position(xml: &str, position: usize) -> bool {
    let Some((boundary_start, _)) = scan_octave_events(xml)
        .into_iter()
        .take_while(|(start, _)| *start < position)
        .last()
    else {
        return false;
    };
    !note_ranges(xml).into_iter().any(|(start, _, note)| {
        start > boundary_start
            && start < position
            && first_tag_text(note, "staff").as_deref() == Some("2")
    })
}

fn scan_octave_events(part: &str) -> Vec<(usize, i64)> {
    let mut events = Vec::new();
    let mut cursor = 0usize;
    while let Some(relative) = part[cursor..].find("<direction") {
        let start = cursor + relative;
        let Some(end) = part[start..]
            .find("</direction>")
            .map(|offset| start + offset + "</direction>".len())
        else {
            break;
        };
        if let Some(shift) = observe_lower_staff_octave_display_shift(&part[start..end]) {
            events.push((start, shift));
        }
        cursor = end;
    }
    events
}

fn first_tag_text(text: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = text.find(&open)? + open.len();
    let end = text[start..].find(&close)? + start;
    Some(text[start..end].trim().to_string())
}

fn score_part_ranges(xml: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut cursor = 0usize;
    while let Some(relative) = xml[cursor..].find("<part") {
        let start = cursor + relative;
        let marker_end = start + "<part".len();
        let is_part = xml
            .as_bytes()
            .get(marker_end)
            .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b'>');
        if !is_part {
            cursor = marker_end;
            continue;
        }
        let Some(open_end) = xml[start..].find('>').map(|offset| start + offset + 1) else {
            break;
        };
        let Some(end) = xml[open_end..]
            .find("</part>")
            .map(|offset| open_end + offset + "</part>".len())
        else {
            break;
        };
        ranges.push((start, end));
        cursor = end;
    }
    if ranges.is_empty() {
        vec![(0, xml.len())]
    } else {
        ranges
    }
}

fn note_ranges(text: &str) -> Vec<(usize, usize, &str)> {
    let mut notes = Vec::new();
    let mut cursor = 0usize;
    while let Some(relative) = text[cursor..].find("<note") {
        let start = cursor + relative;
        let Some(open_offset) = text[start..].find('>') else {
            break;
        };
        let open_end = start + open_offset + 1;
        if text[start..open_end].ends_with("/>") {
            notes.push((start, open_end, &text[start..open_end]));
            cursor = open_end;
            continue;
        }
        let Some(close_offset) = text[open_end..].find("</note>") else {
            break;
        };
        let end = open_end + close_offset + "</note>".len();
        notes.push((start, end, &text[start..end]));
        cursor = end;
    }
    notes
}

fn raw_note_fragments(xml: &str) -> Vec<String> {
    note_ranges(xml)
        .into_iter()
        .map(|(_, _, note)| note.to_string())
        .collect()
}

fn clef_count(xml: &str) -> usize {
    let mut count = 0usize;
    let mut cursor = 0usize;
    while let Some(relative_start) = xml[cursor..].find("<clef") {
        let start = cursor + relative_start;
        let Some(open_end) = xml[start..]
            .find('>')
            .map(|offset| start + offset + 1)
        else {
            break;
        };
        if is_element_start(&xml[start..open_end], "clef") {
            count = count.saturating_add(1);
        }
        cursor = open_end;
    }
    count
}

fn comments_removed(text: &str) -> Option<String> {
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0usize;
    while let Some(relative) = text[cursor..].find("<!--") {
        let start = cursor + relative;
        out.push_str(&text[cursor..start]);
        let end = text[start + 4..]
            .find("-->")
            .map(|offset| start + 4 + offset + 3)?;
        cursor = end;
    }
    out.push_str(&text[cursor..]);
    Some(out)
}

#[cfg(test)]
mod beam_anchor_audit_tests {
    use super::*;

    fn event(voice: &str, staff: &str, beam: &str, extra: &str) -> String {
        format!("<note>{extra}<pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><voice>{voice}</voice><staff>{staff}</staff><beam number=\"1\">{beam}</beam></note>")
    }

    fn observed(part: &str) -> Vec<BeamAnchorObservation> {
        let mut rows = Vec::new();
        let before = scan_lower_pitches(part, None);
        let after = scan_lower_pitches_with_beam_audit(part, Some(&mut rows));
        assert_eq!(format!("{before:?}"), format!("{after:?}"));
        rows
    }

    #[test]
    fn planner_input_trace_binds_real_input_and_preserves_unknown() {
        let first = event("lane", "2", "begin", "");
        let last = event("lane", "2", "continue", "<chord/>");
        let part = format!("<part><measure>{first}</measure><measure>{last}</measure></part>");
        let mut rows = observed(&part);
        // A live stored origin must never be promoted to a usable unknown anchor.
        rows[1].self_anchor = None;
        rows[1].beam_anchor = None;
        rows[1].effective_anchor = None;
        let pitches = scan_lower_pitches(&part, None);
        let text = beam_anchor_planner_input_trace(&part, &part, 1, &rows, &pitches, None);
        assert!(text.contains("CrossMeasureRows=1"));
        assert!(text.contains("EffectiveNote none"));
        assert!(text.contains("Status=unavailable"));
        assert!(text.contains("WriterCommit=not_observed"));
        assert!(text.contains(&format!("InputFnv64={:016x}", super::super::authority_primitives::fingerprint(&part))));
        let other = format!("<score>{part}</score>");
        let rebound = beam_anchor_planner_input_trace(&other, &part, 1, &rows, &pitches, None);
        assert_ne!(text, rebound);
        assert!(rebound.contains(&format!("PartFnv64={:016x}", super::super::authority_primitives::fingerprint(&part))));
    }

    #[test]
    fn planner_input_trace_reports_zero_without_fabricating_solution() {
        let part = format!("<part><measure>{}</measure></part>", event("lane", "2", "begin", ""));
        let rows = observed(&part);
        let text = beam_anchor_planner_input_trace(&part, &part, 1, &rows, &scan_lower_pitches(&part, None), None);
        assert!(text.contains("CrossMeasureRows=0"));
        assert!(text.contains("SolutionAvailable=false"));
        assert!(!text.contains("BeamAnchorPlannerSolution:"));
    }

    #[test]
    fn planner_input_trace_is_uncapped_and_projects_supplied_solution() {
        let mut part = format!("<part><measure>{}</measure>", event("lane", "2", "begin", ""));
        for _ in 0..17 {
            part.push_str(&format!("<measure>{}</measure>", event("lane", "2", "continue", "")));
        }
        part.push_str("</part>");
        let rows = observed(&part);
        let pitches = scan_lower_pitches(&part, None);
        let solution = ClefReadabilityCourseSolution {
            retained: vec![true, false], insertions: vec![(7, ClefKind::Bass)],
            cost: ClefReadabilityCost::default(),
        };
        let text = beam_anchor_planner_input_trace(&part, &part, 1, &rows, &pitches, Some(&solution));
        assert_eq!(text.matches("BeamAnchorPlannerContext:").count(), 17);
        assert!(text.contains("Retained=[true, false]"));
        assert!(text.contains("Insertions=[(7, Bass)]"));
        let second = beam_anchor_planner_input_trace(&part, &part, 2, &rows, &pitches, Some(&solution));
        assert_eq!(second.replace("Part=2", "Part=1").replace("BeamAnchor Part 2", "BeamAnchor Part 1"), text);
        assert_eq!(format!("{pitches:?}"), format!("{:?}", scan_lower_pitches(&part, None)));
    }

    #[test]
    fn beam_anchor_audit_sees_serialized_distance_and_real_xml_boundaries() {
        for base in [13, 81] {
            let begin = event("lower", "2", "begin", "");
            let upper = event("upper", "1", "continue", "");
            let end = event("lower", "2", "end", "");
            let part = format!("<part id=\"synthetic\"><measure number=\"{base}\">{begin}<backup><duration>24</duration></backup>{upper}</measure><measure number=\"{base}\"><!-- <forward/> --><direction><direction-type><words><![CDATA[<attributes/>]]></words></direction-type></direction><forward><duration>24</duration></forward><attributes><divisions>24</divisions></attributes>{end}</measure></part>");
            let rows = observed(&part);
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[1].beam_anchor, Some(part.find(&begin).unwrap()));
            let mut text = Vec::new();
            append_beam_anchor_rows(&part, 1, &rows, &mut text);
            assert!(text[1].contains("notes_2;measures_1;"), "{}", text[1]);
            assert!(text[1].contains("backup_1;forward_1;attributes_1"));
            assert!(text[1].contains("EffectiveNote 1"));
            assert!(text[1].contains("voice_Some(\"lower\")"));
            assert!(text[1].contains("measure_ordinal_2"));
        }
    }

    #[test]
    fn beam_anchor_audit_exposes_stored_origin_but_keeps_unknown_anchorless() {
        let begin = event("v", "2", "begin", "");
        let orphan = event("v", "2", "continue", "<chord/>");
        let part = format!("<part><measure number=\"orphan\">{begin}<forward><duration>24</duration></forward>{orphan}</measure></part>");
        let rows = observed(&part);
        assert_eq!(rows[1].stored_beam_origin, Some(part.find(&begin).unwrap()));
        assert_eq!(rows[1].self_anchor, None);
        assert_eq!(rows[1].beam_anchor, None);
        assert_eq!(rows[1].effective_anchor, None);
    }

    #[test]
    fn beam_anchor_audit_distinguishes_tie_origin_from_beam_begin_and_unmatched_end() {
        let tie = event("v", "2", "forward hook", "<tie type=\"start\"/>");
        let begin = event("v", "2", "begin", "<tie type=\"stop\"/>");
        let end = event("v", "2", "end", "");
        let unmatched = event("w", "2", "end", "");
        let part = format!("<part><measure number=\"tie\">{tie}{begin}{end}{unmatched}</measure></part>");
        let rows = observed(&part);
        let origin = part.find(&tie).unwrap();
        assert_eq!(rows[1].tie_anchor, Some(origin));
        assert_eq!(rows[1].stored_beam_origin, Some(origin));
        assert_eq!(rows[2].beam_anchor, Some(origin));
        assert_eq!(rows[3].self_anchor, Some(part.find(&unmatched).unwrap()));
        assert_eq!(rows[3].stored_beam_origin, None);
        assert_eq!(rows[3].effective_anchor, None);
    }

    #[test]
    fn beam_anchor_audit_retains_foreign_head_metadata_and_part_independence() {
        let upper = event("v", "1", "begin", "");
        let lower = event("v", "2", "continue", "<chord/>");
        let orphan_end = event("v", "2", "end", "");
        let part = format!("<part id=\"one\"><measure number=\"alpha\">{upper}{lower}</measure></part>");
        let second = format!("<part id=\"two\"><measure number=\"omega\">{orphan_end}</measure></part>");
        let xml = format!("<score-partwise>{part}{second}</score-partwise>");
        let audit = derive_clef_readability_shadow_course_audit(&xml);
        assert_eq!(audit.parts_planned, 0); // Unsupported course does not hide evidence.
        assert_eq!(audit.beam_anchor_rows.len(), 2);
        assert!(audit.beam_anchor_rows[0].contains("staff_Some(\"1\")"));
        assert!(audit.beam_anchor_rows[1].contains("StoredBeam none; BeamNote none; EffectiveNote none"));
        let single = derive_clef_readability_shadow_course_audit(&format!("<score-partwise>{part}</score-partwise>"));
        assert_eq!(single.beam_anchor_rows, audit.beam_anchor_rows[..1]);
    }

    #[test]
    fn beam_anchor_audit_reports_all_rows_and_keeps_failed_xml_readback_unknown() {
        let begin = event("v", "2", "begin", "");
        let member = event("v", "2", "continue", "");
        let part = format!("<part><measure number=\"many\">{begin}{}</measure></part>", member.repeat(16));
        let audit = derive_clef_readability_shadow_course_audit(&part);
        assert_eq!(audit.beam_anchor_rows.len(), 17);
        let mut report = String::new();
        append_clef_readability_shadow_course_audit_section(&mut report, &audit, None);
        assert_eq!(report.matches("  BeamAnchor Part ").count(), 17);
        let broken = part.replace("</measure>", "<direction><words>&unbound;</words></direction></measure>");
        let broken_audit = derive_clef_readability_shadow_course_audit(&broken);
        assert_eq!(broken_audit.beam_anchor_rows.len(), 17);
        assert!(broken_audit.beam_anchor_rows.iter().all(|row| row.contains("BetweenXML unknown")));
    }

    #[test]
    fn beam_anchor_audit_octave_reset_clears_origin_without_losing_rows_or_mutating_plan() {
        let begin = event("v", "2", "begin", "");
        let end = event("v", "2", "end", "");
        let shift = "<direction><direction-type><octave-shift type=\"down\" size=\"8\"/></direction-type><staff>2</staff></direction>";
        let part = format!("<part><measure number=\"shift\"><attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>{begin}{shift}{end}</measure></part>");
        let rows = observed(&part);
        assert_eq!(rows[1].stored_beam_origin, None);
        let xml = format!("<score-partwise>{part}</score-partwise>");
        let before = format!("{:?}", derive_clef_readability_primary_course_plan(&xml));
        let audit = derive_clef_readability_shadow_course_audit(&xml);
        let mut report = String::new();
        append_clef_readability_shadow_course_audit_section(&mut report, &audit, None);
        assert_eq!(report.matches("  BeamAnchor Part ").count(), rows.len());
        assert!(report.contains("final-document rescan only"));
        assert!(report.contains("unavailable; no MasterMusicXML fallback"));
        assert_eq!(before, format!("{:?}", derive_clef_readability_primary_course_plan(&xml)));
    }
}

#[cfg(test)]
mod tests {
    use super::{
        derive_clef_readability_primary_course_plan,
        derive_clef_readability_shadow_course_audit, raw_note_fragments, ClefKind,
    };

    fn note(step: &str, octave: i64) -> String {
        format!("<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>")
    }

    fn beamed_note(step: &str, octave: i64, beam_state: &str) -> String {
        format!("<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff><beam number=\"1\">{beam_state}</beam></note>")
    }

    fn note_in_voice(step: &str, octave: i64, voice: i64) -> String {
        format!("<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>24</duration><voice>{voice}</voice><staff>2</staff></note>")
    }

    fn beamed_note_in_voice(
        step: &str,
        octave: i64,
        voice: i64,
        beam_state: &str,
    ) -> String {
        format!("<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>24</duration><voice>{voice}</voice><staff>2</staff><beam number=\"1\">{beam_state}</beam></note>")
    }

    fn tied_note(step: &str, octave: i64, tie_type: &str, chord: bool) -> String {
        let chord_tag = if chord { "<chord/>" } else { "" };
        format!("<note>{chord_tag}<pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>24</duration><tie type=\"{tie_type}\"/><voice>2</voice><staff>2</staff><notations><tied type=\"{tie_type}\"/></notations></note>")
    }

    fn clef(sign: &str, line: i64, generated: bool) -> String {
        let marker = if generated {
            "<!-- etuschlopf-ledger-bound-clef-readback etuschlopf-register-pressure-gould-write -->"
        } else {
            ""
        };
        format!("<attributes>{marker}<clef number=\"2\"><sign>{sign}</sign><line>{line}</line></clef></attributes>")
    }

    fn octave_shift(shift_type: &str) -> String {
        format!("<direction><direction-type><octave-shift type=\"{shift_type}\" size=\"8\"/></direction-type><staff>2</staff></direction>")
    }

    #[test]
    fn readability_shadow_prefers_one_isolated_four_ledger_excursion_over_two_clef_changes() {
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
2\">{}{}{}</measure><measure number=\"\
3\">{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            clef("G", 2, true),
            note("B", 4),
            note("B", 3),
            clef("F", 4, true),
            note("C", 3),
        );
        let audit = derive_clef_readability_shadow_course_audit(&xml);
        assert_eq!(audit.parts_planned, 1);
        assert_eq!(audit.changed_parts, 1);
        assert_eq!(audit.omitted_surfaces, 2);
        assert_eq!(audit.part_rows[0].source_course, "F4>G2>F4");
        assert_eq!(audit.part_rows[0].readability_course, "F4");
        assert!(audit.part_rows[0].readability_cost.ledger_pressure
            > audit.part_rows[0].source_cost.ledger_pressure);
        assert!(audit.part_rows[0].readability_cost.short_excursions
            < audit.part_rows[0].source_cost.short_excursions);
    }

    #[test]
    fn readability_shadow_retains_a_sustained_high_passage() {
        let high = (0..6).map(|_| note("B", 5)).collect::<String>();
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
2\">{}{}</measure><measure number=\"\
3\">{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            clef("G", 2, true),
            high,
            clef("F", 4, true),
            note("C", 3),
        );
        let audit = derive_clef_readability_shadow_course_audit(&xml);
        assert_eq!(audit.parts_planned, 1);
        assert_eq!(audit.changed_parts, 1);
        assert_eq!(audit.omitted_surfaces, 1);
        assert_eq!(audit.part_rows[0].readability_course, "F4>G2");
    }

    #[test]
    fn readability_shadow_retains_a_needed_return_to_low_register() {
        let low = (0..6).map(|_| note("C", 2)).collect::<String>();
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
2\">{}{}</measure><measure number=\"\
3\">{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            clef("G", 2, true),
            (0..5).map(|_| note("B", 5)).collect::<String>(),
            clef("F", 4, true),
            low,
        );
        let audit = derive_clef_readability_shadow_course_audit(&xml);
        assert_eq!(audit.parts_planned, 1);
        assert_eq!(audit.part_rows[0].readability_course, "F4>G2>F4");
    }

    #[test]
    fn readability_shadow_keeps_an_already_stable_course() {
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
        );
        let audit = derive_clef_readability_shadow_course_audit(&xml);
        assert_eq!(audit.parts_planned, 1);
        assert_eq!(audit.changed_parts, 0);
        assert_eq!(audit.part_rows[0].source_course, "F4");
        assert_eq!(audit.part_rows[0].readability_course, "F4");
    }

    #[test]
    fn readability_shadow_does_not_omit_a_non_materializable_clef_attributes_block() {
        let protected_g = "<attributes><divisions>96</divisions><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>";
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
2\">{}{}</measure><measure number=\"\
3\">{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            protected_g,
            note("B", 4),
            clef("F", 4, true),
            note("C", 3),
        );
        let audit = derive_clef_readability_shadow_course_audit(&xml);
        assert_eq!(audit.parts_planned, 1);
        assert!(!audit.part_rows[0]
            .omitted_surface_rows
            .iter()
            .any(|row| row.starts_with("Surface 2 ")));
    }

    #[test]
    fn readability_shadow_inserts_a_missing_sustained_high_course() {
        let high = (0..6).map(|_| note("B", 5)).collect::<String>();
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
2\">{}</measure><measure number=\"\
3\">{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            high,
            note("C", 3),
        );
        let audit = derive_clef_readability_shadow_course_audit(&xml);
        assert_eq!(audit.parts_planned, 1);
        assert_eq!(audit.changed_parts, 1);
        assert_eq!(audit.inserted_surfaces, 1);
        assert_eq!(audit.part_rows[0].source_course, "F4");
        assert_eq!(audit.part_rows[0].readability_course, "F4>G2");
    }

    #[test]
    fn materialized_course_receipt_carries_its_original_operation_journal() {
        for scope in [83usize, 1083] {
            let high = (0..6).map(|_| note("B", 5)).collect::<String>();
            let next = scope + 1;
            let last = scope + 2;
            let xml = format!(
                "<score-partwise><part id=\"P1\"><measure number=\"{scope}\">{}{}</measure><measure number=\"{next}\">{}</measure><measure number=\"{last}\">{}</measure></part></score-partwise>",
                clef("F", 4, false), note("C", 3), high, note("C", 3),
            );
            let mut candidate = xml.clone();
            let (result, receipt) = super::apply_rustig_clef_course_to_buffer(&mut candidate);
            result.unwrap();
            assert_ne!(candidate, xml);
            let receipt = receipt.expect("materialized course mints E024");
            assert!(receipt.has_recorded_representation_for_test());
            receipt.binds_recorded_representation(&xml, &candidate).unwrap();
            receipt.binds_recorded_representation(&candidate, &xml).unwrap_err();
        }
    }

    #[test]
    fn unchanged_course_mints_no_receipt_and_needs_no_journal() {
        for scope in [83usize, 1083] {
            let xml = format!(
                "<score-partwise><part id=\"P1\"><measure number=\"{scope}\">{}{}</measure></part></score-partwise>",
                clef("F", 4, false), note("C", 3),
            );
            let mut candidate = xml.clone();
            let (result, receipt) = super::apply_rustig_clef_course_to_buffer(&mut candidate);
            result.unwrap();
            assert_eq!(candidate, xml);
            assert!(receipt.is_none());
        }
    }

    #[test]
    fn course_journal_replays_equal_start_insertion_before_removal() {
        let source = "abcXYZdef";
        let edits = vec![(3, Some(6), None), (3, None, Some("<clef/>")), (9, None, Some("<e/>"))];
        let journal = super::rustig_clef_course_journal(&edits);
        let step = crate::arrangement::representation_transport::RepresentationStep::recorded(
            source, "abc<clef/>def<e/>", journal);
        crate::arrangement::representation_transport::RhythmicRepresentationTransport::rustig_clef_course(
            &super::RustigClefCoursePreservingMint { _sealed: () }, step).unwrap();
    }

    #[test]
    fn course_journal_refuses_a_rhythmic_fragment() {
        let source = "a<note/>b";
        let step = crate::arrangement::representation_transport::RepresentationStep::recorded(
            source, "ab", vec![(1..8, String::new())]);
        crate::arrangement::representation_transport::RhythmicRepresentationTransport::rustig_clef_course(
            &super::RustigClefCoursePreservingMint { _sealed: () }, step).unwrap_err();
    }

    #[test]
    fn primary_course_materializes_the_same_sustained_high_course_as_the_planner() {
        let high = (0..6).map(|_| note("B", 5)).collect::<String>();
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
2\">{}</measure><measure number=\"\
3\">{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            high,
            note("C", 3),
        );
        let source_notes = raw_note_fragments(&xml);
        let plan = derive_clef_readability_primary_course_plan(&xml);
        let mut candidate = xml.clone();
        super::apply_rustig_clef_course_to_buffer(&mut candidate).0.unwrap();
        assert_ne!(candidate, xml);
        assert!(plan.verifies(&candidate));
        assert_eq!(raw_note_fragments(&candidate), source_notes);
        assert_eq!(
            candidate
                .matches("etuschlopf-clef-readability-primary-course")
                .count(),
            1
        );
        let audit = derive_clef_readability_shadow_course_audit(&candidate);
        assert_eq!(audit.parts_planned, 1);
        assert_eq!(audit.changed_parts, 0);
        assert_eq!(audit.part_rows[0].readability_course, "F4>G2");
    }

    #[test]
    fn primary_course_anchors_a_treble_transition_before_the_tied_chain() {
        let tie_start = tied_note("B", 5, "start", false);
        let tie_stop = tied_note("B", 5, "stop", false);
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
12\">{}{}{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            tie_start,
            tie_stop,
            note("D", 6).replacen("<note>", "<note><chord/>", 1),
            note("F", 6).replacen("<note>", "<note><chord/>", 1),
        );
        let tie_start_position = xml.find("<tie type=\"start\"").unwrap();
        let tie_start_note_position = xml[..tie_start_position].rfind("<note>").unwrap();
        let tie_stop_position = xml.find("<tie type=\"stop\"").unwrap();
        let plan = derive_clef_readability_primary_course_plan(&xml);
        assert!(plan.insertions.iter().any(|(start, kind)| {
            *start == tie_start_note_position && *kind == ClefKind::Treble
        }));
        assert!(!plan
            .insertions
            .iter()
            .any(|(start, _)| *start > tie_start_note_position && *start < tie_stop_position));

        let mut candidate = xml.clone();
        super::apply_rustig_clef_course_to_buffer(&mut candidate).0.unwrap();
        let marker_position = candidate
            .find("etuschlopf-clef-readability-primary-course")
            .unwrap();
        let tied_start_position = candidate.find("<tie type=\"start\"").unwrap();
        assert!(marker_position < tied_start_position);
    }

    #[test]
    fn primary_course_reanchors_an_existing_clef_surface_out_of_an_open_tie() {
        let tie_start = tied_note("B", 5, "start", false);
        let tie_stop = tied_note("B", 5, "stop", false);
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
15\">{}{}{}{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            tie_start,
            clef("G", 2, true),
            tie_stop,
            note("D", 6).replacen("<note>", "<note><chord/>", 1),
            note("F", 6).replacen("<note>", "<note><chord/>", 1),
        );
        let mut candidate = xml.clone();
        super::apply_rustig_clef_course_to_buffer(&mut candidate).0.unwrap();

        let tie_start_position = candidate.find("<tie type=\"start\"").unwrap();
        let tie_stop_position = candidate.find("<tie type=\"stop\"").unwrap();
        let marker_position = candidate
            .find("etuschlopf-clef-readability-primary-course")
            .unwrap();
        assert!(marker_position < tie_start_position);
        assert!(!candidate[tie_start_position..tie_stop_position]
            .contains("<clef number=\"2\">"));
    }

    #[test]
    fn primary_course_keeps_one_isolated_four_ledger_attack_in_bass() {
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
2\">{}</measure><measure number=\"\
3\">{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            note("B", 4),
            note("C", 3),
        );
        let mut candidate = xml.clone();
        super::apply_rustig_clef_course_to_buffer(&mut candidate).0.unwrap();
        assert_eq!(candidate, xml);
    }

    #[test]
    fn readability_shadow_does_not_insert_for_one_isolated_four_ledger_attack() {
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
2\">{}</measure><measure number=\"\
3\">{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            note("B", 4),
            note("C", 3),
        );
        let audit = derive_clef_readability_shadow_course_audit(&xml);
        assert_eq!(audit.parts_planned, 1);
        assert_eq!(audit.changed_parts, 0);
        assert_eq!(audit.inserted_surfaces, 0);
        assert_eq!(audit.part_rows[0].readability_course, "F4");
    }

    #[test]
    fn primary_course_reanchors_an_existing_clef_surface_to_primary_beam_begin() {
        let first_group_note = beamed_note("C", 4, "begin");
        let high_continue = beamed_note("B", 5, "continue");
        let high_end = beamed_note("B", 5, "end");
        let sustained_high = (0..4).map(|_| note("B", 5)).collect::<String>();
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
25\">{}{}{}{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            first_group_note,
            clef("G", 2, true),
            high_continue,
            high_end,
            sustained_high,
        );
        let group_begin = xml.find("<beam number=\"1\">begin</beam>").unwrap();
        let group_begin_note = xml[..group_begin].rfind("<note>").unwrap();

        let plan = derive_clef_readability_primary_course_plan(&xml);
        assert!(plan.insertions.iter().any(|(start, kind)| {
            *start == group_begin_note && *kind == ClefKind::Treble
        }));

        let mut candidate = xml.clone();
        super::apply_rustig_clef_course_to_buffer(&mut candidate).0.unwrap();
        let marker = candidate
            .find("etuschlopf-clef-readability-primary-course")
            .unwrap();
        let begin_after = candidate.find("<beam number=\"1\">begin</beam>").unwrap();
        let begin_note_after = candidate[..begin_after].rfind("<note>").unwrap();
        let end_after = candidate.find("<beam number=\"1\">end</beam>").unwrap();
        assert!(marker < begin_note_after);
        assert!(!candidate[begin_note_after..end_after].contains("<clef number=\"2\">"));
    }

    #[test]
    fn primary_course_does_not_reanchor_a_clef_across_an_octave_shift_boundary() {
        let beam_begin = beamed_note("G", 4, "begin");
        let beam_continue = beamed_note("C", 4, "continue");
        let beam_end = beamed_note("C", 5, "end");
        let high_tail = (0..4).map(|_| note("C", 5)).collect::<String>();
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
18\">{}{}{}{}{}{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            octave_shift("down"),
            beam_begin,
            beam_continue,
            octave_shift("stop"),
            clef("G", 2, true),
            beam_end,
            high_tail,
        );
        let shift_stop = xml.find("<octave-shift type=\"stop\"").unwrap();
        let source_g_sign = xml.find("<sign>G</sign>").unwrap();
        let source_g_surface = xml[..source_g_sign].rfind("<attributes>").unwrap();
        let beam_begin_tag = xml.find("<beam number=\"1\">begin</beam>").unwrap();
        let beam_begin_note = xml[..beam_begin_tag].rfind("<note>").unwrap();

        assert!(shift_stop < source_g_surface);
        // The lower stop terminates the old group for this later query.
        assert!(!super::lower_primary_beam_group_contains_position(
            &xml,
            source_g_surface
        ));
        let before_stop = xml.find(&beam_continue).unwrap();
        assert!(super::lower_primary_beam_group_contains_position(
            &xml,
            before_stop
        ));
        assert!(super::lower_octave_shift_boundary_precedes_position(
            &xml,
            source_g_surface
        ));

        let pitches = super::scan_lower_pitches(&xml, None);
        let attacks = super::collect_lower_staff_attacks(&pitches);
        let candidates = super::collect_readability_transition_candidates(&attacks);
        assert!(!candidates.iter().any(|candidate| {
            candidate.start == beam_begin_note && candidate.kind == ClefKind::Treble
        }));

        let mut candidate = xml.clone();
        super::apply_rustig_clef_course_to_buffer(&mut candidate).0.unwrap();
        let shift_stop_after = candidate.find("<octave-shift type=\"stop\"").unwrap();
        let g_after = candidate.find("<sign>G</sign>").unwrap();
        assert!(shift_stop_after < g_after);
        assert!(!candidate[..shift_stop_after]
            .contains("etuschlopf-clef-readability-primary-course"));
    }

    #[test]
    fn primary_course_never_inserts_on_an_unmatched_primary_beam_continuation() {
        let unmatched = beamed_note("B", 5, "continue");
        let high = (0..5).map(|_| note("B", 5)).collect::<String>();
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
2\">{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            unmatched,
            high,
        );
        let unmatched_position = xml.find("<beam number=\"1\">continue</beam>").unwrap();
        let unmatched_note = xml[..unmatched_position].rfind("<note>").unwrap();
        let plan = derive_clef_readability_primary_course_plan(&xml);
        assert!(!plan
            .insertions
            .iter()
            .any(|(start, _)| *start == unmatched_note));
    }

    #[test]
    fn primary_course_collapses_mixed_preference_primary_beam_to_one_boundary() {
        let high_begin = beamed_note("B", 5, "begin");
        let low_end = beamed_note("F", 3, "end");
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
15\">{}{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            high_begin,
            clef("G", 2, true),
            low_end,
        );
        let group_begin = xml.find("<beam number=\"1\">begin</beam>").unwrap();
        let group_begin_note = xml[..group_begin].rfind("<note>").unwrap();
        let plan = derive_clef_readability_primary_course_plan(&xml);
        let anchored = plan
            .insertions
            .iter()
            .filter(|(start, _)| *start == group_begin_note)
            .collect::<Vec<_>>();

        assert_eq!(anchored.len(), 1);
        assert_eq!(anchored[0].1, ClefKind::Treble);

        let mut candidate = xml.clone();
        super::apply_rustig_clef_course_to_buffer(&mut candidate).0.unwrap();
        let begin_after = candidate.find("<beam number=\"1\">begin</beam>").unwrap();
        let begin_note_after = candidate[..begin_after].rfind("<note>").unwrap();
        let end_after = candidate.find("<beam number=\"1\">end</beam>").unwrap();
        let marker = candidate
            .find("etuschlopf-clef-readability-primary-course")
            .unwrap();
        assert!(marker < begin_note_after);
        assert!(!candidate[begin_note_after..end_after].contains("<clef number=\"2\">"));
    }

    #[test]
    fn primary_course_excludes_cross_voice_transition_inside_open_primary_beam() {
        let foreign_beam_begin = beamed_note_in_voice("C", 3, 3, "begin");
        let unsafe_high = note_in_voice("B", 5, 2);
        let foreign_beam_end = beamed_note_in_voice("C", 3, 3, "end");
        let high_tail = (0..12)
            .map(|_| note_in_voice("B", 5, 2))
            .collect::<String>();
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
2\">{}{}{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            foreign_beam_begin,
            unsafe_high,
            foreign_beam_end,
            high_tail,
        );
        let unsafe_start = xml.find(&unsafe_high).unwrap();
        assert!(super::lower_primary_beam_group_contains_position(
            &xml,
            unsafe_start
        ));
        assert!(!super::lower_octave_shift_boundary_precedes_position(
            &xml,
            unsafe_start
        ));

        let surfaces = super::scan_lower_clef_surfaces(&xml).unwrap();
        let pitches = super::scan_lower_pitches(&xml, None);
        let attacks = super::collect_lower_staff_attacks(&pitches);
        let candidates = super::collect_readability_transition_candidates(&attacks);
        assert!(candidates.iter().any(|candidate| {
            candidate.start == unsafe_start && candidate.kind == ClefKind::Treble
        }));

        let solution = super::solve_readability_course(&xml, &surfaces, &pitches).unwrap();
        assert!(!solution.insertions.iter().any(|(start, _)| {
            *start == unsafe_start
                || (super::lower_primary_beam_group_contains_position(&xml, *start)
                    && !super::lower_octave_shift_boundary_precedes_position(&xml, *start))
        }));
    }

    #[test]
    fn primary_course_carries_beam_closed_origin_through_a_tie_chain() {
        let high_begin = beamed_note("B", 5, "begin");
        let low_tie_start = tied_note("C", 3, "start", false).replacen(
            "</note>",
            "<beam number=\"1\">end</beam></note>",
            1,
        );
        let low_tie_stop = tied_note("C", 3, "stop", false);
        let low_tail = (0..5).map(|_| note("C", 3)).collect::<String>();
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
15\">{}{}{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            high_begin,
            low_tie_start,
            low_tie_stop,
            low_tail,
        );
        let beam_begin = xml.find("<beam number=\"1\">begin</beam>").unwrap();
        let beam_begin_note = xml[..beam_begin].rfind("<note>").unwrap();
        let tie_start = xml.find("<tie type=\"start\"").unwrap();
        let tie_start_note = xml[..tie_start].rfind("<note>").unwrap();

        let pitches = super::scan_lower_pitches(&xml, None);
        let attacks = super::collect_lower_staff_attacks(&pitches);
        let candidates = super::collect_readability_transition_candidates(&attacks);

        assert!(candidates.iter().any(|candidate| {
            candidate.start == beam_begin_note && candidate.kind == ClefKind::Treble
        }));
        assert!(!candidates
            .iter()
            .any(|candidate| candidate.start == tie_start_note));
        assert!(candidates.iter().all(|candidate| {
            !super::lower_primary_beam_group_contains_position(&xml, candidate.start)
        }));
    }


    #[test]
    fn primary_course_carries_tie_closed_origin_into_a_primary_beam_group() {
        let high_tie_start = tied_note("B", 5, "start", false);
        let high_tie_stop_beam_begin = tied_note("B", 5, "stop", false).replacen(
            "</note>",
            "<beam number=\"1\">begin</beam></note>",
            1,
        );
        let high_beam_end = beamed_note("B", 5, "end");
        let high_tail = (0..4).map(|_| note("B", 5)).collect::<String>();
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">{}{}</measure><measure number=\"\
26\">{}{}{}{}{}</measure></part></score-partwise>",
            clef("F", 4, false),
            note("C", 3),
            high_tie_start,
            high_tie_stop_beam_begin,
            clef("G", 2, true),
            high_beam_end,
            high_tail,
        );
        let tie_start = xml.find("<tie type=\"start\"").unwrap();
        let tie_start_note = xml[..tie_start].rfind("<note>").unwrap();
        let beam_begin = xml.find("<beam number=\"1\">begin</beam>").unwrap();
        let beam_begin_note = xml[..beam_begin].rfind("<note>").unwrap();

        let pitches = super::scan_lower_pitches(&xml, None);
        let attacks = super::collect_lower_staff_attacks(&pitches);
        let candidates = super::collect_readability_transition_candidates(&attacks);

        assert!(candidates.iter().any(|candidate| {
            candidate.start == tie_start_note && candidate.kind == ClefKind::Treble
        }));
        assert!(!candidates
            .iter()
            .any(|candidate| candidate.start == beam_begin_note));

        let mut candidate = xml.clone();
        super::apply_rustig_clef_course_to_buffer(&mut candidate).0.unwrap();
        let marker = candidate
            .find("etuschlopf-clef-readability-primary-course")
            .unwrap();
        let begin_after = candidate.find("<beam number=\"1\">begin</beam>").unwrap();
        let begin_note_after = candidate[..begin_after].rfind("<note>").unwrap();
        let end_after = candidate.find("<beam number=\"1\">end</beam>").unwrap();
        assert!(marker < begin_note_after);
        assert!(!candidate[begin_note_after..end_after].contains("<clef number=\"2\">"));
    }

}

#[cfg(test)]
#[path = "../../tests/unit/lower_staff_chord_attack_checkpoint58.rs"]
mod lower_staff_chord_attack_checkpoint58;
