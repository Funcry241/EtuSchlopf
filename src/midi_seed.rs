// v0.10.725 FoundationIoRealModuleSplitNoBehaviorChange.
// Real crate-root module split from the former mechanical IO grouping.
// Preserves behavior from the former FLP module; no wrapper/include shell.

// --- mechanical module boundary: former src/flp.rs ---
use crate::midi::read_standard_midi;
use crate::pipeline_profile::{base_solver_profiles, CandidateProfile};
use crate::source_selection::{
    bind_productive_source_selection_plan,
    project_source_selection_full_causal_audit,
    ExistingSourceSelectionRemovalReason,
    RemovedSourceSelectionReason, RetainedSourceSelectionIdentity,
    SourceSelectionCandidateEvaluation, SourceSelectionDecisionAction,
    SourceSelectionDecisionEvidence, SourceSelectionDecisionPass,
    SourceSelectionFullCausalProjection,
    MidiSeedSerializedSourceIdentityRow,
    SourceSelectionSolverCausalityProjection,
};
use anyhow::{anyhow, Context};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const DIVISIONS: i64 = 96;
const MEASURE_DIVISIONS: i64 = DIVISIONS * 4;

#[derive(Debug, Clone)]
pub struct MidiSeedStats {
    pub solver_profile: String,
    pub source_selection_candidate_id: usize,
    pub source_selection_origin_stage: String,
    pub source_selection_profile_name: String,
    pub profile_span_limit: i32,
    pub profile_jump_limit: i32,
    pub profile_smooth_jumps: bool,
    pub profile_left_high_soft_limit: i32,
    pub profile_right_low_soft_limit: i32,
    pub hotspot_plan_audit: HotspotPlanAuditProjection,
    pub source_selection_full_causal: SourceSelectionFullCausalProjection,
    pub source_selection_candidate_evaluation: SourceSelectionCandidateEvaluation,
    pub source_selection_solver_causality: SourceSelectionSolverCausalityProjection,
    /// GENESIS #68, minted by this generator at the creation event. The solver
    /// takes it out of the stats immediately and transports it with the seed.
    pub(crate) master_genesis: Option<crate::master_state_identity::MasterGenesisIdentity>,
    pub(crate) rhythmic_courses: MidiSeedRhythmicCourses,
}

/// Sealed mint capability of the MIDI-seed generator. Only this module can
/// construct it, so only this generator can mint MIDI-seed GENESIS identities.
pub(crate) struct MidiSeedGenesisMint {
    _sealed: (),
}

pub fn midi_seed_solver_profiles() -> Vec<CandidateProfile> {
    base_solver_profiles()
}

#[derive(Debug, Clone, Copy)]
pub struct HotspotFocus {
    pub measure: usize,
    pub staff: Option<u8>,
    pub score: i64,
}

#[derive(Debug, Clone)]
pub struct HotspotEvidenceAudit {
    pub issue_type: String,
    pub measure: usize,
    pub staff: Option<u8>,
    pub base_weight: i64,
    pub critical_bonus: i64,
    pub hardness: i64,
    pub contribution: i64,
    pub included_after_truncation: bool,
}

#[derive(Debug, Clone, Default)]
pub struct HotspotPlanAuditProjection {
    pub critical_only: bool,
    pub source_issue_count: usize,
    pub group_count_before_truncation: usize,
    pub requested_limit: usize,
    pub truncated_count: usize,
    pub fingerprint_sha256: String,
    pub evidence: Vec<HotspotEvidenceAudit>,
}

#[derive(Debug, Clone, Default)]
pub struct HotspotPlan {
    pub entries: Vec<HotspotFocus>,
    pub audit: HotspotPlanAuditProjection,
}

impl HotspotPlan {
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn measure_count(&self) -> usize {
        self.entries.iter().map(|h| h.measure).collect::<BTreeSet<_>>().len()
    }
    fn focus_score_for(&self, measure: usize, staff: Option<u8>) -> i64 {
        self.entries
            .iter()
            .filter(|h| h.measure == measure && (h.staff.is_none() || staff.is_none() || h.staff == staff))
            .map(|h| h.score)
            .max()
            .unwrap_or(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MidiSeedNote {
    source_ordinal: usize,
    pos: i64,
    len: i64,
    midi: i32,
    staff: u8,
}

// A creation identity, never a musical coordinate or XML locator. Its issuer
// is private to the MIDI generator; continuation births require the sealed
// actual arrangement-owner mint capability.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RhythmicCreationIdentity(u64);

impl std::fmt::Debug for RhythmicCreationIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RhythmicCreationIdentity(opaque)")
    }
}

impl RhythmicCreationIdentity {
    pub(crate) fn editorial_rest_birth(_: &crate::structural_write_authority::EditorialVoiceCompleteRewriteExactMint) -> anyhow::Result<Self> { Self::create() }
    pub(crate) fn visible_rhythm_birth(_: &crate::arrangement::VisibleRhythmBlockMint) -> anyhow::Result<Self> { Self::create() }
    pub(crate) fn restification_birth(_: &crate::structural_write_authority::RestificationBatchExactMint) -> anyhow::Result<Self> { Self::create() }
    pub(crate) fn transfer_birth(_: &crate::structural_write_authority::SafeTransferExactMint) -> anyhow::Result<Self> { Self::create() }
    pub(crate) fn gesture_relocation_birth(_: &crate::structural_write_authority::ChordGestureDistributionExactMint) -> anyhow::Result<Self> { Self::create() }
    pub(crate) fn arrangement_fragment(_: &crate::arrangement::ArrangementRhythmicMint) -> anyhow::Result<Self> {
        Self::create()
    }
    fn create() -> anyhow::Result<Self> {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        NEXT.fetch_update(
            std::sync::atomic::Ordering::Relaxed,
            std::sync::atomic::Ordering::Relaxed,
            |value| value.checked_add(1),
        ).map(Self).map_err(|_| anyhow!("Rhythmic creation identity exhausted"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SeedCoursePresentation {
    Primary,
}

#[derive(Debug, Clone)]
struct SeedPrimaryCourse {
    creation: RhythmicCreationIdentity,
    presentation: SeedCoursePresentation,
    staff: u8,
    coverage: std::ops::Range<i64>,
}

#[derive(Debug, Clone)]
struct SeedRhythmicOccurrence {
    creation: RhythmicCreationIdentity,
    course: RhythmicCreationIdentity,
    interval: std::ops::Range<i64>,
    source: Option<usize>,
    bytes: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
struct PlannedSeedOccurrence {
    course: RhythmicCreationIdentity,
    interval: std::ops::Range<i64>,
    source: Option<usize>,
    pitch: Option<i32>,
    chord: bool,
}

/// Deliberately generated piano presentations, issued before XML emission.
/// Private fields/constructors prevent downstream callers from declaring an
/// imported rest or a technical channel to be a newly generated primary course.
/// Source membership is ancestry, not a new canonical pitched-source role.
#[derive(Debug, Clone)]
pub(crate) struct MidiSeedRhythmicCourses {
    courses: Vec<SeedPrimaryCourse>,
    sources: Vec<MidiSeedNote>,
    planned: Vec<PlannedSeedOccurrence>,
    occurrences: Vec<SeedRhythmicOccurrence>,
    metric_measure_duration: i64,
    emission_snapshot: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct SeedArrangementCourse {
    pub(crate) creation: RhythmicCreationIdentity,
    pub(crate) presentation: SeedCoursePresentation,
    pub(crate) staff: u8,
    pub(crate) coverage: std::ops::Range<i64>,
}

pub(crate) struct SeedArrangementOccurrence {
    pub(crate) creation: RhythmicCreationIdentity,
    pub(crate) course: RhythmicCreationIdentity,
    pub(crate) interval: std::ops::Range<i64>,
    pub(crate) source: Option<usize>,
    pub(crate) bytes: std::ops::Range<usize>,
}

pub(crate) struct SeedArrangementInventory {
    pub(crate) courses: Vec<SeedArrangementCourse>,
    pub(crate) occurrences: Vec<SeedArrangementOccurrence>,
}

impl MidiSeedRhythmicCourses {
    pub(crate) fn arrangement_inventory(&self, xml: &str) -> anyhow::Result<SeedArrangementInventory> {
        self.validate_seed_binding(xml)?;
        Ok(SeedArrangementInventory {
            courses: self.courses.iter().map(|c| SeedArrangementCourse {
                creation: c.creation, presentation: c.presentation,
                staff: c.staff, coverage: c.coverage.clone(),
            }).collect(),
            occurrences: self.occurrences.iter().map(|o| SeedArrangementOccurrence {
                creation: o.creation, course: o.course, interval: o.interval.clone(),
                source: o.source, bytes: o.bytes.clone(),
            }).collect(),
        })
    }
    fn create_primary_presentations(
        sources: &[MidiSeedNote], measures: usize, measure_duration: i64,
    ) -> anyhow::Result<Self> {
        let end = i64::try_from(measures).ok()
            .and_then(|count| count.checked_mul(measure_duration))
            .filter(|end| *end > 0)
            .ok_or_else(|| anyhow!("Invalid primary notation domain"))?;
        let mut membership = BTreeSet::new();
        for source in sources {
            if !membership.insert(source.source_ordinal)
                || !matches!(source.staff, 1 | 2)
                || source.pos < 0 || source.pos >= end
            {
                return Err(anyhow!("Invalid primary source membership"));
            }
        }
        // These are the generator's existing explicit piano presentations.
        // Their birth precedes gap search; an empty presentation is also planned.
        let courses = [1, 2].into_iter().map(|staff| {
            Ok(SeedPrimaryCourse {
                creation: RhythmicCreationIdentity::create()?,
                presentation: SeedCoursePresentation::Primary,
                staff,
                coverage: 0..end,
            })
        }).collect::<anyhow::Result<Vec<_>>>()?;
        let mut grouped = BTreeMap::<(i64, u8), BTreeMap<i64, Vec<MidiSeedNote>>>::new();
        for source in sources {
            grouped.entry((source.pos / measure_duration, source.staff)).or_default()
                .entry(source.pos).or_default().push(*source);
        }
        let mut planned = Vec::new();
        for measure in 0..measures {
            let begin = measure as i64 * measure_duration;
            for course in &courses {
                let mut by_start = grouped.remove(&(measure as i64, course.staff)).unwrap_or_default();
                let starts = by_start.keys().copied().collect::<Vec<_>>();
                let mut cursor = begin;
                for (index, start) in starts.iter().copied().enumerate() {
                    if cursor < start {
                        planned.push(PlannedSeedOccurrence {
                            course: course.creation, interval: cursor..start,
                            source: None, pitch: None, chord: false,
                        });
                    }
                    let next = starts.get(index + 1).copied().unwrap_or(begin + measure_duration);
                    let group = by_start.get_mut(&start).expect("planned onset exists");
                    group.sort_by_key(|source| (source.midi, source.source_ordinal));
                    if course.staff == 1 { group.reverse(); }
                    for (member, source) in group.iter().enumerate() {
                        planned.push(PlannedSeedOccurrence {
                            course: course.creation, interval: start..next,
                            source: Some(source.source_ordinal), pitch: Some(source.midi),
                            chord: member > 0,
                        });
                    }
                    cursor = next;
                }
                if cursor < begin + measure_duration {
                    planned.push(PlannedSeedOccurrence {
                        course: course.creation, interval: cursor..(begin + measure_duration),
                        source: None, pitch: None, chord: false,
                    });
                }
            }
        }
        Ok(Self {
            courses, sources: sources.to_vec(), planned, occurrences: Vec::new(),
            metric_measure_duration: measure_duration, emission_snapshot: None,
        })
    }

    fn begin_emission(&self, sources: &[MidiSeedNote], measures: usize) -> anyhow::Result<()> {
        let end = i64::try_from(measures).ok()
            .and_then(|count| count.checked_mul(MEASURE_DIVISIONS));
        if self.emission_snapshot.is_some() || !self.occurrences.is_empty()
            || self.sources != sources || self.metric_measure_duration != MEASURE_DIVISIONS
            || self.courses.iter().any(|course| Some(course.coverage.end) != end)
        {
            return Err(anyhow!("Stale or incompatible rhythmic emission plan"));
        }
        Ok(())
    }

    fn capture_emission(
        &mut self, staff: u8, interval: std::ops::Range<i64>,
        source: Option<usize>, bytes: std::ops::Range<usize>,
    ) -> anyhow::Result<()> {
        let course = self.courses.iter().find(|course| course.staff == staff)
            .ok_or_else(|| anyhow!("Missing planned primary presentation"))?;
        let expected = self.planned.get(self.occurrences.len())
            .ok_or_else(|| anyhow!("Unplanned rhythmic seed occurrence"))?;
        if expected.course != course.creation || expected.interval != interval
            || expected.source != source
            || self.emission_snapshot.is_some() || interval.start < course.coverage.start
            || interval.end > course.coverage.end || interval.start >= interval.end
            || bytes.start >= bytes.end
            || source.is_some_and(|ordinal| !self.sources.iter().any(|note| {
                note.source_ordinal == ordinal && note.staff == staff
            }))
        {
            return Err(anyhow!("Unlicensed rhythmic seed occurrence"));
        }
        self.occurrences.push(SeedRhythmicOccurrence {
            creation: RhythmicCreationIdentity::create()?, course: course.creation,
            interval, source, bytes,
        });
        Ok(())
    }

    fn seal_emission(&mut self, xml: &str) -> anyhow::Result<()> {
        if self.emission_snapshot.is_some() {
            return Err(anyhow!("Rhythmic seed already emitted"));
        }
        self.validate_occurrences(xml)?;
        self.emission_snapshot = Some(xml.to_owned());
        Ok(())
    }

    fn validate_occurrences(&self, xml: &str) -> anyhow::Result<()> {
        let mut identities = self.courses.iter().map(|c| c.creation).collect::<BTreeSet<_>>();
        let mut sources = BTreeSet::new();
        let mut byte_cursor = 0;
        if identities.len() != self.courses.len()
            || self.planned.len() != self.occurrences.len()
            || xml.matches("<note>").count() != self.occurrences.len()
        {
            return Err(anyhow!("Incomplete rhythmic seed creation inventory"));
        }
        for (occurrence, planned) in self.occurrences.iter().zip(&self.planned) {
            let course = self.courses.iter().find(|c| c.creation == occurrence.course)
                .ok_or_else(|| anyhow!("Foreign rhythmic course"))?;
            let fragment = xml.get(occurrence.bytes.clone())
                .ok_or_else(|| anyhow!("Invalid rhythmic seed byte continuation"))?;
            if occurrence.course != planned.course || occurrence.interval != planned.interval
                || occurrence.source != planned.source
                || fragment.contains("<chord/>") != planned.chord
                || !identities.insert(occurrence.creation) || occurrence.bytes.start < byte_cursor
                || !fragment.starts_with("      <note>\n")
                || !fragment.ends_with("      </note>\n")
                || fragment.contains("<rest/>") != occurrence.source.is_none()
                || !fragment.contains(&format!("<staff>{}</staff>", course.staff))
                || !fragment.contains(&format!("<duration>{}</duration>",
                    occurrence.interval.end - occurrence.interval.start))
                || occurrence.interval.start < course.coverage.start
                || occurrence.interval.end > course.coverage.end
                || occurrence.interval.start >= occurrence.interval.end
                || course.presentation != SeedCoursePresentation::Primary
                || occurrence.source.is_some_and(|source| {
                    !sources.insert(source) || !self.sources.iter().any(|n| {
                        n.source_ordinal == source && n.staff == course.staff
                    })
                })
            {
                return Err(anyhow!("Conflicting rhythmic seed creation continuation"));
            }
            byte_cursor = occurrence.bytes.end;
        }
        if sources != self.sources.iter().map(|n| n.source_ordinal).collect() {
            return Err(anyhow!("Incomplete rhythmic source membership"));
        }
        for course in &self.courses {
            let mut activity = self.occurrences.iter().filter(|o| o.course == course.creation)
                .collect::<Vec<_>>();
            activity.sort_by_key(|o| (o.interval.start, o.interval.end));
            let mut end = course.coverage.start;
            let mut rest_end = course.coverage.start;
            for occurrence in &activity {
                if occurrence.interval.start > end || occurrence.source.is_none()
                    && occurrence.interval.start < end
                    || occurrence.source.is_some() && occurrence.interval.start < rest_end
                {
                    return Err(anyhow!("Missing or conflicting primary rhythmic coverage"));
                }
                if occurrence.source.is_none() { rest_end = occurrence.interval.end; }
                end = end.max(occurrence.interval.end);
            }
            if end != course.coverage.end {
                return Err(anyhow!("Incomplete primary rhythmic lifetime"));
            }
        }
        Ok(())
    }

    pub(crate) fn validate_seed_binding(&self, xml: &str) -> anyhow::Result<()> {
        if self.emission_snapshot.as_deref() != Some(xml) {
            return Err(anyhow!("Rhythmic creation plan belongs to another seed state"));
        }
        self.validate_occurrences(xml)
    }
}

fn initial_staff_decision(note: &MidiSeedNote) -> SourceSelectionDecisionEvidence {
    SourceSelectionDecisionEvidence {
        source_ordinal: note.source_ordinal,
        pass: SourceSelectionDecisionPass::InitialPitchStaff,
        pass_iteration: 0,
        action: SourceSelectionDecisionAction::InitialStaff,
        pos: note.pos,
        len: note.len,
        midi: note.midi,
        before_staff: None,
        after_staff: Some(note.staff),
        witness_a_source_ordinal: None,
        witness_b_source_ordinal: None,
        observed_primary: i64::from(note.midi),
        observed_secondary: 0,
        threshold_primary: 60,
        threshold_secondary: 0,
        focus_score: 0,
        tie_break_code: u8::from(note.midi >= 60),
        removal_reason: None,
    }
}

#[derive(Debug, Clone, Copy)]
struct SourceSelectionDecisionContext {
    pass: SourceSelectionDecisionPass,
    pass_iteration: usize,
    witness_a_source_ordinal: Option<usize>,
    witness_b_source_ordinal: Option<usize>,
    observed_primary: i64,
    observed_secondary: i64,
    threshold_primary: i64,
    threshold_secondary: i64,
    focus_score: i64,
    tie_break_code: u8,
}

fn staff_move_decision(
    note: &MidiSeedNote,
    new_staff: u8,
    context: SourceSelectionDecisionContext,
) -> SourceSelectionDecisionEvidence {
    SourceSelectionDecisionEvidence {
        source_ordinal: note.source_ordinal,
        pass: context.pass,
        pass_iteration: context.pass_iteration,
        action: SourceSelectionDecisionAction::StaffMove,
        pos: note.pos,
        len: note.len,
        midi: note.midi,
        before_staff: Some(note.staff),
        after_staff: Some(new_staff),
        witness_a_source_ordinal: context.witness_a_source_ordinal,
        witness_b_source_ordinal: context.witness_b_source_ordinal,
        observed_primary: context.observed_primary,
        observed_secondary: context.observed_secondary,
        threshold_primary: context.threshold_primary,
        threshold_secondary: context.threshold_secondary,
        focus_score: context.focus_score,
        tie_break_code: context.tie_break_code,
        removal_reason: None,
    }
}

fn removal_decision(
    note: &MidiSeedNote,
    reason: ExistingSourceSelectionRemovalReason,
    context: SourceSelectionDecisionContext,
) -> SourceSelectionDecisionEvidence {
    SourceSelectionDecisionEvidence {
        source_ordinal: note.source_ordinal,
        pass: context.pass,
        pass_iteration: context.pass_iteration,
        action: SourceSelectionDecisionAction::Remove,
        pos: note.pos,
        len: note.len,
        midi: note.midi,
        before_staff: Some(note.staff),
        after_staff: None,
        witness_a_source_ordinal: context.witness_a_source_ordinal,
        witness_b_source_ordinal: context.witness_b_source_ordinal,
        observed_primary: context.observed_primary,
        observed_secondary: context.observed_secondary,
        threshold_primary: context.threshold_primary,
        threshold_secondary: context.threshold_secondary,
        focus_score: context.focus_score,
        tie_break_code: context.tie_break_code,
        removal_reason: Some(reason),
    }
}


pub fn midi_to_musicxml_seed_with_profile_and_hotspots(
    path: &Path,
    title: &str,
    profile: CandidateProfile,
    hotspots: Option<&HotspotPlan>,
) -> anyhow::Result<(String, MidiSeedStats)> {
    let mut events = read_standard_midi(path)
        .with_context(|| format!("Konnte MIDI nicht als produktive Quelle lesen: {}", path.display()))?
        .notes;
    events.sort_by_key(|event| event.source_ordinal);
    let mut notes = events
        .iter()
        .map(|event| MidiSeedNote {
            source_ordinal: event.source_ordinal,
            pos: event.pos,
            len: event.len,
            midi: event.midi,
            staff: if event.midi >= 60 { 1 } else { 2 },
        })
        .collect::<Vec<_>>();
    if notes.is_empty() {
        return Err(anyhow!("MIDI enthält keine lesbaren Notenereignisse"));
    }

    let mut decision_trace = notes.iter().map(initial_staff_decision).collect::<Vec<_>>();
    let mut removal_reasons = Vec::new();
    redistribute_midi_seed_hands_for_piano(&mut notes, &profile, hotspots, &mut decision_trace);
    apply_hotspot_reductions(
        &mut notes,
        &profile,
        hotspots,
        &mut removal_reasons,
        &mut decision_trace,
    );
    let trajectory = apply_hand_trajectory_solver(
        &mut notes,
        &profile,
        hotspots,
        &mut removal_reasons,
        &mut decision_trace,
    );
    if trajectory.moved > 0 || trajectory.removed > 0 {
        // v0.1.45: Nach Trajectory-Reparaturen zwingend wieder Spannweiten
        // bereinigen. Sonst wird ein Sprungproblem oft nur in ein Akkord-
        // Spannweitenproblem verschoben.
        redistribute_wide_simultaneities(
            &mut notes,
            &profile,
            hotspots,
            SourceSelectionDecisionPass::PostTrajectoryWideSimultaneity,
            0,
            &mut decision_trace,
        );
    }
    let residual = apply_residual_critical_pass(
        &mut notes,
        &profile,
        hotspots,
        &mut removal_reasons,
        &mut decision_trace,
    );
    if residual.moved > 0 || residual.removed > 0 {
        // v0.1.46: auch nach dem generischen Restfehler-Pass erneut die
        // Gleichzeitigkeit entspannen. Der Pass kennt keine Takte, sondern nur
        // die automatisch erkannten Hotspots und objektive Spielbarkeitsdaten.
        redistribute_wide_simultaneities(
            &mut notes,
            &profile,
            hotspots,
            SourceSelectionDecisionPass::PostResidualWideSimultaneity,
            0,
            &mut decision_trace,
        );
    }
    notes.sort_by_key(|n| (n.pos, n.staff, n.midi));
    let max_end = notes.iter().map(|n| n.pos + n.len.max(1)).max().unwrap_or(0);
    let measures = ((max_end + MEASURE_DIVISIONS - 1) / MEASURE_DIVISIONS).max(1) as usize;
    let retained_source_identities = notes
        .iter()
        .map(|note| RetainedSourceSelectionIdentity {
            source_ordinal: note.source_ordinal,
            pos: note.pos,
            len: note.len,
            midi: note.midi,
            final_staff: note.staff,
        })
        .collect::<Vec<_>>();
    let mut source_selection_full_causal = project_source_selection_full_causal_audit(
        &events,
        &retained_source_identities,
        &removal_reasons,
        decision_trace,
    )?;
    let productive_plan_retained_identities = bind_productive_source_selection_plan(
        &events,
        &retained_source_identities,
        &mut source_selection_full_causal,
    )?;
    notes = productive_plan_retained_identities
        .iter()
        .map(|event| MidiSeedNote {
            source_ordinal: event.source_ordinal,
            pos: event.pos,
            len: event.len,
            midi: event.midi,
            staff: event.final_staff,
        })
        .collect();
    let rhythmic_courses = MidiSeedRhythmicCourses::create_primary_presentations(
        &notes, measures, MEASURE_DIVISIONS,
    )?;
    let stats = MidiSeedStats {
        solver_profile: profile.display_name().to_string(),
        source_selection_candidate_id: 0,
        source_selection_origin_stage: "unassigned_seed_api".to_string(),
        source_selection_profile_name: profile.display_name().to_string(),
        profile_span_limit: profile.span_limit(),
        profile_jump_limit: profile.jump_limit(),
        profile_smooth_jumps: profile.smooth_jumps(),
        profile_left_high_soft_limit: profile.left_high_soft_limit(),
        profile_right_low_soft_limit: profile.right_low_soft_limit(),
        hotspot_plan_audit: hotspots.map_or_else(HotspotPlanAuditProjection::default, |plan| plan.audit.clone()),
        source_selection_full_causal,
        source_selection_candidate_evaluation: SourceSelectionCandidateEvaluation::default(),
        source_selection_solver_causality: SourceSelectionSolverCausalityProjection::default(),
        master_genesis: None,
        rhythmic_courses,
    };

    let mut stats = stats;
    let (xml, measure_raw_note_counts, serialized_source_identity_rows) =
        build_musicxml(title, &notes, measures, &mut stats.rhythmic_courses)?;
    crate::source_selection::bind_midi_seed_serialized_source_identity_plan(
        &mut stats.source_selection_full_causal,
        measure_raw_note_counts,
        serialized_source_identity_rows,
    )?;
    let genesis_input = crate::master_state_identity::GenesisInputBinding::ProductiveSourcePlanSha256(
        stats
            .source_selection_full_causal
            .productive_plan_seed_fingerprint_sha256
            .clone(),
    );
    stats.master_genesis = Some(crate::master_state_identity::MasterGenesisIdentity::mint_genesis(
        MidiSeedGenesisMint { _sealed: () },
        &format!("{profile:?}"),
        genesis_input,
        &xml,
    ));

    Ok((xml, stats))
}


#[derive(Debug, Clone, Copy, Default)]
struct ResidualCriticalStats {
    moved: usize,
    removed: usize,
}

fn apply_residual_critical_pass(
    notes: &mut Vec<MidiSeedNote>,
    profile: &CandidateProfile,
    hotspots: Option<&HotspotPlan>,
    removal_reasons: &mut Vec<RemovedSourceSelectionReason>,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) -> ResidualCriticalStats {
    let Some(plan) = hotspots else { return ResidualCriticalStats::default(); };
    if plan.is_empty() || !profile.runs_residual_pass() {
        return ResidualCriticalStats::default();
    }

    let mut stats = ResidualCriticalStats::default();
    let max_passes = profile.residual_max_passes();
    for pass_iteration in 0..max_passes {
        let moved = residual_move_register_outliers(
            notes,
            profile,
            plan,
            pass_iteration,
            decision_trace,
        );
        let removed_spans = residual_remove_critical_spans(
            notes,
            profile,
            plan,
            removal_reasons,
            pass_iteration,
            decision_trace,
        );
        let removed_jumps = residual_remove_critical_jumps(
            notes,
            profile,
            plan,
            removal_reasons,
            pass_iteration,
            decision_trace,
        );
        stats.moved += moved;
        stats.removed += removed_spans + removed_jumps;
        if moved == 0 && removed_spans == 0 && removed_jumps == 0 {
            break;
        }
    }
    stats
}

fn residual_move_register_outliers(
    notes: &mut [MidiSeedNote],
    profile: &CandidateProfile,
    plan: &HotspotPlan,
    pass_iteration: usize,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) -> usize {
    let mut moves: Vec<(usize, u8, SourceSelectionDecisionEvidence)> = Vec::new();
    for focus in &plan.entries {
        let local_profile = profile.focused(focus.score);
        let mut local = hotspot_note_indices(notes, focus);
        local.sort_by_key(|idx| (notes[*idx].pos, notes[*idx].staff, notes[*idx].midi));
        let cap = profile.register_move_cap(focus.score);
        let mut added = 0usize;
        for idx in local {
            if added >= cap { break; }
            let note = notes[idx];
            let target = if note.staff == 2 && note.midi >= local_profile.left_high_soft_limit() + 2 {
                Some((1_u8, local_profile.left_high_soft_limit() + 2, local_profile.right_low_soft_limit() - 10, 1_u8))
            } else if note.staff == 1 && note.midi <= local_profile.right_low_soft_limit() - 2 {
                Some((2_u8, local_profile.right_low_soft_limit() - 2, local_profile.left_high_soft_limit() + 10, 2_u8))
            } else {
                None
            };
            if let Some((new_staff, trigger_threshold, plausibility_threshold, tie_break_code)) = target
                && residual_move_is_plausible(&note, new_staff, &local_profile)
            {
                let evidence = staff_move_decision(
                    &note,
                    new_staff,
                    SourceSelectionDecisionContext {
                        pass: SourceSelectionDecisionPass::ResidualRegisterMove,
                        pass_iteration,
                        witness_a_source_ordinal: None,
                        witness_b_source_ordinal: None,
                        observed_primary: i64::from(note.midi),
                        observed_secondary: 0,
                        threshold_primary: i64::from(trigger_threshold),
                        threshold_secondary: i64::from(plausibility_threshold),
                        focus_score: focus.score,
                        tie_break_code,
                    },
                );
                moves.push((idx, new_staff, evidence));
                added += 1;
            }
        }
    }

    moves.sort_by_key(|(idx, staff, _)| (*idx, *staff));
    moves.dedup_by_key(|(idx, staff, _)| (*idx, *staff));
    let mut moved = 0usize;
    for (idx, staff, mut evidence) in moves {
        if let Some(note) = notes.get_mut(idx)
            && note.staff != staff
        {
            evidence.before_staff = Some(note.staff);
            decision_trace.push(evidence);
            note.staff = staff;
            moved += 1;
        }
    }
    moved
}

fn residual_move_is_plausible(note: &MidiSeedNote, new_staff: u8, profile: &CandidateProfile) -> bool {
    if new_staff == 1 {
        note.midi >= profile.right_low_soft_limit() - 10
    } else {
        note.midi <= profile.left_high_soft_limit() + 10
    }
}

fn residual_remove_critical_spans(
    notes: &mut Vec<MidiSeedNote>,
    profile: &CandidateProfile,
    plan: &HotspotPlan,
    removal_reasons: &mut Vec<RemovedSourceSelectionReason>,
    pass_iteration: usize,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) -> usize {
    let mut remove = BTreeSet::new();
    let mut removal_decision_by_index = BTreeMap::new();
    for focus in &plan.entries {
        let mut by_start_staff: BTreeMap<(i64, u8), Vec<usize>> = BTreeMap::new();
        for idx in hotspot_note_indices(notes, focus) {
            by_start_staff.entry((notes[idx].pos, notes[idx].staff)).or_default().push(idx);
        }
        let local_profile = profile.focused(focus.score);
        let cap = profile.critical_removal_cap(focus.score);
        let before = remove.len();
        for ((_start, staff), mut group) in by_start_staff {
            if remove.len().saturating_sub(before) >= cap || group.len() < 2 { continue; }
            group.sort_by_key(|idx| notes[*idx].midi);
            let low_idx = *group.first().expect("non-empty residual span group");
            let high_idx = *group.last().expect("non-empty residual span group");
            let low = notes[low_idx].midi;
            let high = notes[high_idx].midi;
            let span = high - low;
            if span <= local_profile.span_limit() { continue; }

            let mut keep = BTreeSet::new();
            if staff == 1 {
                let top = high_idx;
                keep.insert(top);
                for idx in group.iter().rev().skip(1) {
                    if keep.len() >= 2 { break; }
                    if (notes[top].midi - notes[*idx].midi).abs() <= local_profile.span_limit() {
                        keep.insert(*idx);
                    }
                }
            } else {
                let bass = low_idx;
                keep.insert(bass);
                for idx in group.iter().skip(1) {
                    if keep.len() >= 2 { break; }
                    if (notes[*idx].midi - notes[bass].midi).abs() <= local_profile.span_limit() {
                        keep.insert(*idx);
                    }
                }
            }

            for idx in group {
                if remove.len().saturating_sub(before) >= cap { break; }
                if !keep.contains(&idx)
                    && is_residual_removable_note(notes, idx, staff, &local_profile)
                    && remove.insert(idx)
                {
                    removal_decision_by_index.insert(
                        idx,
                        removal_decision(
                            &notes[idx],
                            ExistingSourceSelectionRemovalReason::ResidualCriticalSpan,
                            SourceSelectionDecisionContext {
                                pass: SourceSelectionDecisionPass::ResidualCriticalSpan,
                                pass_iteration,
                                witness_a_source_ordinal: Some(notes[low_idx].source_ordinal),
                                witness_b_source_ordinal: Some(notes[high_idx].source_ordinal),
                                observed_primary: i64::from(span),
                                observed_secondary: i64::try_from(keep.len()).unwrap_or(i64::MAX),
                                threshold_primary: i64::from(local_profile.span_limit()),
                                threshold_secondary: 2,
                                focus_score: focus.score,
                                tie_break_code: staff,
                            },
                        ),
                    );
                }
            }
        }
    }
    record_removed_source_reasons(
        notes,
        &remove,
        ExistingSourceSelectionRemovalReason::ResidualCriticalSpan,
        removal_reasons,
    );
    for idx in &remove {
        if let Some(evidence) = removal_decision_by_index.get(idx).copied() {
            decision_trace.push(evidence);
        }
    }
    retain_without_indices(notes, &remove)
}

fn residual_remove_critical_jumps(
    notes: &mut Vec<MidiSeedNote>,
    profile: &CandidateProfile,
    plan: &HotspotPlan,
    removal_reasons: &mut Vec<RemovedSourceSelectionReason>,
    pass_iteration: usize,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) -> usize {
    let mut remove = BTreeSet::new();
    let mut removal_decision_by_index = BTreeMap::new();
    for focus in &plan.entries {
        let local_profile = profile.focused(focus.score);
        let cap = profile.critical_removal_cap(focus.score);
        let before = remove.len();
        for staff in [1_u8, 2_u8] {
            if focus.staff.is_some_and(|s| s != staff) {
                continue;
            }
            let mut reps = representative_indices_for_focus(notes, focus, staff);
            reps.sort_by_key(|idx| (notes[*idx].pos, notes[*idx].midi));
            for pair in reps.windows(2) {
                if remove.len().saturating_sub(before) >= cap { break; }
                let a = pair[0];
                let b = pair[1];
                if remove.contains(&a) || remove.contains(&b) { continue; }
                let dt = (notes[b].pos - notes[a].pos).abs();
                if dt > 144 { continue; }
                let jump = (notes[b].midi - notes[a].midi).abs();
                if jump <= local_profile.jump_limit() { continue; }
                let target = choose_hotspot_jump_note_to_remove(notes, a, b, staff, &local_profile);
                if is_residual_removable_note(notes, target, staff, &local_profile)
                    && remove.insert(target)
                {
                    let tie_break_code = (if target == a { 1_u8 } else { 2_u8 }) + if staff == 2 { 10_u8 } else { 0_u8 };
                    removal_decision_by_index.insert(
                        target,
                        removal_decision(
                            &notes[target],
                            ExistingSourceSelectionRemovalReason::ResidualCriticalJump,
                            SourceSelectionDecisionContext {
                                pass: SourceSelectionDecisionPass::ResidualCriticalJump,
                                pass_iteration,
                                witness_a_source_ordinal: Some(notes[a].source_ordinal),
                                witness_b_source_ordinal: Some(notes[b].source_ordinal),
                                observed_primary: i64::from(jump),
                                observed_secondary: dt,
                                threshold_primary: i64::from(local_profile.jump_limit()),
                                threshold_secondary: 144,
                                focus_score: focus.score,
                                tie_break_code,
                            },
                        ),
                    );
                }
            }
        }
    }
    record_removed_source_reasons(
        notes,
        &remove,
        ExistingSourceSelectionRemovalReason::ResidualCriticalJump,
        removal_reasons,
    );
    for idx in &remove {
        if let Some(evidence) = removal_decision_by_index.get(idx).copied() {
            decision_trace.push(evidence);
        }
    }
    retain_without_indices(notes, &remove)
}

fn is_residual_removable_note(notes: &[MidiSeedNote], idx: usize, staff: u8, profile: &CandidateProfile) -> bool {
    let Some(note) = notes.get(idx) else { return false; };
    if staff == 1 {
        // Rechte Hand: Oberstimme schuetzen, aber tiefe/mittlere Ausreisser duerfen fallen.
        note.midi <= profile.right_low_soft_limit() + 12
    } else {
        // Linke Hand: Bassanker schuetzen, aber hohe Ausreisser duerfen fallen.
        note.midi >= profile.left_high_soft_limit() - 12
    }
}

fn record_removed_source_reasons(
    notes: &[MidiSeedNote],
    remove: &BTreeSet<usize>,
    reason: ExistingSourceSelectionRemovalReason,
    removal_reasons: &mut Vec<RemovedSourceSelectionReason>,
) {
    removal_reasons.extend(remove.iter().filter_map(|idx| {
        notes.get(*idx).map(|note| RemovedSourceSelectionReason {
            source_ordinal: note.source_ordinal,
            reason,
        })
    }));
}

fn retain_without_indices(notes: &mut Vec<MidiSeedNote>, remove: &BTreeSet<usize>) -> usize {
    if remove.is_empty() { return 0; }
    let original = notes.len();
    let mut idx = 0usize;
    notes.retain(|_| {
        let keep = !remove.contains(&idx);
        idx += 1;
        keep
    });
    original.saturating_sub(notes.len())
}

#[derive(Debug, Clone, Copy, Default)]
struct HandTrajectoryStats {
    moved: usize,
    removed: usize,
}

fn apply_hand_trajectory_solver(
    notes: &mut Vec<MidiSeedNote>,
    profile: &CandidateProfile,
    hotspots: Option<&HotspotPlan>,
    removal_reasons: &mut Vec<RemovedSourceSelectionReason>,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) -> HandTrajectoryStats {
    // v0.1.43: HandTrajectorySolver.
    // Die bisherigen Hotspot-Regeln behandeln Einzelereignisse; die letzten
    // schweren Warnungen sind aber Bewegungsfehler einer Hand. Deshalb wird
    // innerhalb automatisch erkannter Hotspots die Kurve
    // vorige Note -> aktuelle Note -> nächste Note analysiert. Ausreisser
    // werden zuerst zur anderen Hand verschoben; nur wenn das die Kurve nicht
    // stabilisiert, werden unwichtige lokale Noten entfernt.
    let Some(plan) = hotspots else { return HandTrajectoryStats::default(); };
    if plan.is_empty() || !profile.runs_trajectory_pass() {
        return HandTrajectoryStats::default();
    }

    let mut stats = HandTrajectoryStats::default();
    let max_passes = profile.trajectory_max_passes();
    for pass_iteration in 0..max_passes {
        let mut changed = false;

        let moved = trajectory_move_outliers(
            notes,
            profile,
            plan,
            pass_iteration,
            decision_trace,
        );
        if moved > 0 {
            stats.moved += moved;
            changed = true;
        }

        let removed = trajectory_remove_zigzags(
            notes,
            profile,
            plan,
            removal_reasons,
            pass_iteration,
            decision_trace,
        );
        if removed > 0 {
            stats.removed += removed;
            changed = true;
        }

        if !changed {
            break;
        }
    }

    stats
}

fn trajectory_move_outliers(
    notes: &mut [MidiSeedNote],
    profile: &CandidateProfile,
    plan: &HotspotPlan,
    pass_iteration: usize,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) -> usize {
    let mut moves: Vec<(usize, u8, SourceSelectionDecisionEvidence)> = Vec::new();
    for focus in &plan.entries {
        for staff in [1_u8, 2_u8] {
            if focus.staff.is_some_and(|s| s != staff) {
                continue;
            }
            let reps = representative_indices_for_focus(notes, focus, staff);
            if reps.len() < 2 { continue; }

            for pair in reps.windows(2) {
                let a = pair[0];
                let b = pair[1];
                let dt = (notes[b].pos - notes[a].pos).abs();
                let local_focus_score = plan.focus_score_for(note_measure(notes[a].pos), Some(staff));
                let local_profile = profile.focused(local_focus_score);
                let dt_limit = if local_profile.jump_limit() <= 12 { 144 } else { 96 };
                if dt > dt_limit { continue; }
                let jump = (notes[b].midi - notes[a].midi).abs();
                if jump <= local_profile.jump_limit() { continue; }

                let target = choose_trajectory_move_target(notes, a, b, staff, &local_profile);
                let new_staff = if staff == 1 { 2 } else { 1 };
                if should_move_to_other_hand(notes, target, new_staff, &local_profile) {
                    let tie_break_code = (if target == a { 1_u8 } else { 2_u8 })
                        + if staff == 2 { 10_u8 } else { 0_u8 };
                    let evidence = staff_move_decision(
                        &notes[target],
                        new_staff,
                        SourceSelectionDecisionContext {
                            pass: SourceSelectionDecisionPass::TrajectoryMoveOutlier,
                            pass_iteration,
                            witness_a_source_ordinal: Some(notes[a].source_ordinal),
                            witness_b_source_ordinal: Some(notes[b].source_ordinal),
                            observed_primary: i64::from(jump),
                            observed_secondary: dt,
                            threshold_primary: i64::from(local_profile.jump_limit()),
                            threshold_secondary: dt_limit,
                            focus_score: local_focus_score,
                            tie_break_code,
                        },
                    );
                    moves.push((target, new_staff, evidence));
                }
            }
        }
    }

    moves.sort_by_key(|(idx, staff, _)| (*idx, *staff));
    moves.dedup_by_key(|(idx, staff, _)| (*idx, *staff));
    let mut moved = 0usize;
    for (idx, new_staff, mut evidence) in moves {
        if let Some(note) = notes.get_mut(idx)
            && note.staff != new_staff
        {
            evidence.before_staff = Some(note.staff);
            decision_trace.push(evidence);
            note.staff = new_staff;
            moved += 1;
        }
    }
    moved
}

fn trajectory_remove_zigzags(
    notes: &mut Vec<MidiSeedNote>,
    profile: &CandidateProfile,
    plan: &HotspotPlan,
    removal_reasons: &mut Vec<RemovedSourceSelectionReason>,
    pass_iteration: usize,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) -> usize {
    let mut remove = BTreeSet::new();
    let mut removal_decision_by_index = BTreeMap::new();
    for focus in &plan.entries {
        let cap = if focus.score >= 1000 { 10 } else if focus.score >= 700 { 7 } else { 4 };
        let before = remove.len();
        for staff in [1_u8, 2_u8] {
            if focus.staff.is_some_and(|s| s != staff) {
                continue;
            }
            let reps = representative_indices_for_focus(notes, focus, staff);
            if reps.len() < 3 { continue; }
            for triple in reps.windows(3) {
                let a = triple[0];
                let b = triple[1];
                let c = triple[2];
                let local_focus_score = plan.focus_score_for(note_measure(notes[b].pos), Some(staff));
                let local_profile = profile.focused(local_focus_score);
                let ab = (notes[b].midi - notes[a].midi).abs();
                let bc = (notes[c].midi - notes[b].midi).abs();
                let ac = (notes[c].midi - notes[a].midi).abs();
                let dt1 = (notes[b].pos - notes[a].pos).abs();
                let dt2 = (notes[c].pos - notes[b].pos).abs();
                if dt1 > 144 || dt2 > 144 { continue; }
                if ab <= local_profile.jump_limit() || bc <= local_profile.jump_limit() { continue; }
                if ac > local_profile.jump_limit() + 7 { continue; }
                if !is_removable_trajectory_note(notes, b, staff, &local_profile) { continue; }
                if remove.insert(b) {
                    removal_decision_by_index.insert(
                        b,
                        removal_decision(
                            &notes[b],
                            ExistingSourceSelectionRemovalReason::TrajectoryZigzag,
                            SourceSelectionDecisionContext {
                                pass: SourceSelectionDecisionPass::TrajectoryZigzag,
                                pass_iteration,
                                witness_a_source_ordinal: Some(notes[a].source_ordinal),
                                witness_b_source_ordinal: Some(notes[c].source_ordinal),
                                observed_primary: i64::from(ab.max(bc)),
                                observed_secondary: i64::from(ac),
                                threshold_primary: i64::from(local_profile.jump_limit()),
                                threshold_secondary: 144,
                                focus_score: local_focus_score,
                                tie_break_code: staff,
                            },
                        ),
                    );
                }
                if remove.len().saturating_sub(before) >= cap { break; }
            }
        }
    }

    if remove.is_empty() { return 0; }
    record_removed_source_reasons(
        notes,
        &remove,
        ExistingSourceSelectionRemovalReason::TrajectoryZigzag,
        removal_reasons,
    );
    for idx in &remove {
        if let Some(evidence) = removal_decision_by_index.get(idx).copied() {
            decision_trace.push(evidence);
        }
    }
    let original = notes.len();
    let mut idx = 0usize;
    notes.retain(|_| {
        let keep = !remove.contains(&idx);
        idx += 1;
        keep
    });
    original.saturating_sub(notes.len())
}

fn representative_indices_for_focus(notes: &[MidiSeedNote], focus: &HotspotFocus, staff: u8) -> Vec<usize> {
    let measure_start = (focus.measure.saturating_sub(1) as i64) * MEASURE_DIVISIONS;
    let measure_end = measure_start + MEASURE_DIVISIONS;
    let mut by_start: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (idx, note) in notes.iter().enumerate() {
        if note.staff == staff && note.pos >= measure_start && note.pos < measure_end {
            by_start.entry(note.pos).or_default().push(idx);
        }
    }
    let mut reps = Vec::new();
    for (_start, group) in by_start {
        let rep = if staff == 1 {
            group.into_iter().max_by_key(|idx| notes[*idx].midi)
        } else {
            group.into_iter().min_by_key(|idx| notes[*idx].midi)
        };
        if let Some(idx) = rep {
            reps.push(idx);
        }
    }
    reps.sort_by_key(|idx| (notes[*idx].pos, notes[*idx].midi));
    reps
}

fn choose_trajectory_move_target(notes: &[MidiSeedNote], a: usize, b: usize, staff: u8, profile: &CandidateProfile) -> usize {
    if staff == 2 {
        // Links: hohe Ausreisser nach rechts; Bassanker bleiben eher links.
        if notes[a].midi >= profile.left_high_soft_limit() { return a; }
        if notes[b].midi >= profile.left_high_soft_limit() { return b; }
        if notes[a].midi > notes[b].midi { a } else { b }
    } else {
        // Rechts: tiefe Ausreisser nach links; Oberstimme bleibt eher rechts.
        if notes[a].midi <= profile.right_low_soft_limit() { return a; }
        if notes[b].midi <= profile.right_low_soft_limit() { return b; }
        if notes[a].midi < notes[b].midi { a } else { b }
    }
}

fn should_move_to_other_hand(notes: &[MidiSeedNote], idx: usize, new_staff: u8, profile: &CandidateProfile) -> bool {
    let Some(note) = notes.get(idx) else { return false; };
    if new_staff == 1 {
        // Rechts darf hoch/mittig tragen, aber keine sehr tiefen Bassanker.
        note.midi >= profile.right_low_soft_limit() - 8
    } else {
        // Links darf Bass/mittleres Register tragen, aber keine melodischen Spitzen.
        note.midi <= profile.left_high_soft_limit() + 8
    }
}

fn is_removable_trajectory_note(notes: &[MidiSeedNote], idx: usize, staff: u8, profile: &CandidateProfile) -> bool {
    let Some(note) = notes.get(idx) else { return false; };
    if staff == 1 {
        // Rechte Hand: tiefe Ausreisser eher entbehrlich, sehr hohe Oberstimme schützen.
        note.midi <= profile.right_low_soft_limit() + 4
    } else {
        // Linke Hand: hohe Ausreisser eher entbehrlich, Bassanker schützen.
        note.midi >= profile.left_high_soft_limit() - 4
    }
}

fn apply_hotspot_reductions(
    notes: &mut Vec<MidiSeedNote>,
    profile: &CandidateProfile,
    hotspots: Option<&HotspotPlan>,
    removal_reasons: &mut Vec<RemovedSourceSelectionReason>,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) -> usize {
    // v0.1.41: Die Hotspot-Profile dürfen nicht nur andere Namen tragen.
    // Sie erzeugen lokale Kandidaten, indem sie in automatisch erkannten
    // Problemstellen echte Spielbarkeitsentscheidungen treffen: breite
    // Gleichzeitigkeit ausdünnen und schnelle Sprünge lokal glätten.
    //
    // r636 pensionierte die destruktive Soft-Register-Hotspot-Autoritaet,
    // nachdem der Vollkorpus-r634-Beweis gezeigt hatte, dass die damaligen
    // Removals erst durch vorherige Staff-Reassignments entstanden. Solche
    // Registerkonflikte bleiben fuer die source-erhaltenden Trajectory-/
    // Residual-Move-Paesse erhalten. Es werden weiterhin keine neuen Noten
    // erfunden.
    let Some(plan) = hotspots else { return 0; };
    if plan.is_empty() || !profile.runs_hotspot_reduction() {
        return 0;
    }

    let mut remove = BTreeSet::new();
    let mut removal_reason_by_index = BTreeMap::new();
    let mut removal_decision_by_index = BTreeMap::new();
    let per_hotspot_cap = profile.per_hotspot_cap();

    for focus in &plan.entries {
        let before = remove.len();
        reduce_hotspot_spans(
            notes,
            focus,
            profile,
            per_hotspot_cap,
            &mut remove,
            &mut removal_reason_by_index,
            &mut removal_decision_by_index,
        );
        reduce_hotspot_jumps(
            notes,
            focus,
            profile,
            per_hotspot_cap,
            &mut remove,
            &mut removal_reason_by_index,
            &mut removal_decision_by_index,
        );
        // Kein einzelner Hotspot soll das Material entkernen. Wenn die beiden
        // loeschenden Reparaturschritte zu viel markiert haben, werden die spaetesten
        // Markierungen wieder verworfen. Die bestehende BTreeSet-Reihenfolge
        // bleibt dabei unverändert die Autorität.
        let added = remove.len().saturating_sub(before);
        if added > per_hotspot_cap {
            let mut recent = remove
                .iter()
                .copied()
                .skip(before + per_hotspot_cap)
                .collect::<Vec<_>>();
            for idx in recent.drain(..) {
                remove.remove(&idx);
                removal_reason_by_index.remove(&idx);
                removal_decision_by_index.remove(&idx);
            }
        }
    }

    if remove.is_empty() {
        return 0;
    }

    for idx in &remove {
        if let (Some(note), Some(reason)) =
            (notes.get(*idx), removal_reason_by_index.get(idx).copied())
        {
            removal_reasons.push(RemovedSourceSelectionReason {
                source_ordinal: note.source_ordinal,
                reason,
            });
        }
        if let Some(evidence) = removal_decision_by_index.get(idx).copied() {
            decision_trace.push(evidence);
        }
    }

    let original = notes.len();
    let mut idx = 0usize;
    notes.retain(|_| {
        let keep = !remove.contains(&idx);
        idx += 1;
        keep
    });
    original.saturating_sub(notes.len())
}

fn hotspot_note_indices(notes: &[MidiSeedNote], focus: &HotspotFocus) -> Vec<usize> {
    let measure_start = (focus.measure.saturating_sub(1) as i64) * MEASURE_DIVISIONS;
    let measure_end = measure_start + MEASURE_DIVISIONS;
    notes
        .iter()
        .enumerate()
        .filter(|(_, note)| {
            note.pos >= measure_start
                && note.pos < measure_end
                && focus.staff.is_none_or(|staff| note.staff == staff)
        })
        .map(|(idx, _)| idx)
        .collect()
}

fn reduce_hotspot_spans(
    notes: &[MidiSeedNote],
    focus: &HotspotFocus,
    profile: &CandidateProfile,
    cap: usize,
    remove: &mut BTreeSet<usize>,
    removal_reason_by_index: &mut BTreeMap<usize, ExistingSourceSelectionRemovalReason>,
    removal_decision_by_index: &mut BTreeMap<usize, SourceSelectionDecisionEvidence>,
) {
    let mut by_start_staff: BTreeMap<(i64, u8), Vec<usize>> = BTreeMap::new();
    for idx in hotspot_note_indices(notes, focus) {
        if remove.contains(&idx) { continue; }
        by_start_staff.entry((notes[idx].pos, notes[idx].staff)).or_default().push(idx);
    }

    let mut added = 0usize;
    for ((_start, staff), mut group) in by_start_staff {
        if added >= cap || group.len() < 2 { continue; }
        group.sort_by_key(|idx| notes[*idx].midi);
        let low_idx = *group.first().expect("non-empty hotspot span group");
        let high_idx = *group.last().expect("non-empty hotspot span group");
        let low = notes[low_idx].midi;
        let high = notes[high_idx].midi;
        let span = high - low;
        if span <= profile.span_limit() { continue; }

        let mut keep = BTreeSet::new();
        let max_keep = profile.max_notes_kept_per_staff();
        if staff == 1 {
            // Rechts: Melodie/Oberstimme schuetzen, tiefe Ausreisser entfernen.
            let top = high_idx;
            keep.insert(top);
            for idx in group.iter().rev().skip(1) {
                if keep.len() >= max_keep { break; }
                if (notes[top].midi - notes[*idx].midi).abs() <= profile.span_limit() {
                    keep.insert(*idx);
                }
            }
        } else {
            // Links: Bassanker schuetzen, hohe Ausreisser entfernen.
            let bass = low_idx;
            keep.insert(bass);
            for idx in group.iter().skip(1) {
                if keep.len() >= max_keep { break; }
                if (notes[*idx].midi - notes[bass].midi).abs() <= profile.span_limit() {
                    keep.insert(*idx);
                }
            }
        }

        for idx in group {
            if added >= cap { break; }
            if !keep.contains(&idx) && remove.insert(idx) {
                removal_reason_by_index
                    .insert(idx, ExistingSourceSelectionRemovalReason::HotspotSpan);
                removal_decision_by_index.insert(
                    idx,
                    removal_decision(
                        &notes[idx],
                        ExistingSourceSelectionRemovalReason::HotspotSpan,
                        SourceSelectionDecisionContext {
                            pass: SourceSelectionDecisionPass::HotspotSpan,
                            pass_iteration: 0,
                            witness_a_source_ordinal: Some(notes[low_idx].source_ordinal),
                            witness_b_source_ordinal: Some(notes[high_idx].source_ordinal),
                            observed_primary: i64::from(span),
                            observed_secondary: i64::try_from(keep.len()).unwrap_or(i64::MAX),
                            threshold_primary: i64::from(profile.span_limit()),
                            threshold_secondary: i64::try_from(max_keep).unwrap_or(i64::MAX),
                            focus_score: focus.score,
                            tie_break_code: staff,
                        },
                    ),
                );
                added += 1;
            }
        }
    }
}

fn reduce_hotspot_jumps(
    notes: &[MidiSeedNote],
    focus: &HotspotFocus,
    profile: &CandidateProfile,
    cap: usize,
    remove: &mut BTreeSet<usize>,
    removal_reason_by_index: &mut BTreeMap<usize, ExistingSourceSelectionRemovalReason>,
    removal_decision_by_index: &mut BTreeMap<usize, SourceSelectionDecisionEvidence>,
) {
    let mut added = 0usize;
    for staff in [1_u8, 2_u8] {
        let mut indices = hotspot_note_indices(notes, focus)
            .into_iter()
            .filter(|idx| notes[*idx].staff == staff && !remove.contains(idx))
            .collect::<Vec<_>>();
        indices.sort_by_key(|idx| (notes[*idx].pos, notes[*idx].midi));
        for pair in indices.windows(2) {
            if added >= cap { break; }
            let a = pair[0];
            let b = pair[1];
            let dt = (notes[b].pos - notes[a].pos).abs();
            if dt > 96 { continue; }
            let jump = (notes[b].midi - notes[a].midi).abs();
            if jump <= profile.jump_limit() { continue; }
            let target = choose_hotspot_jump_note_to_remove(notes, a, b, staff, profile);
            if remove.insert(target) {
                removal_reason_by_index
                    .insert(target, ExistingSourceSelectionRemovalReason::HotspotJump);
                let tie_break_code = (if target == a { 1_u8 } else { 2_u8 })
                    + if staff == 2 { 10_u8 } else { 0_u8 };
                removal_decision_by_index.insert(
                    target,
                    removal_decision(
                        &notes[target],
                        ExistingSourceSelectionRemovalReason::HotspotJump,
                        SourceSelectionDecisionContext {
                            pass: SourceSelectionDecisionPass::HotspotJump,
                            pass_iteration: 0,
                            witness_a_source_ordinal: Some(notes[a].source_ordinal),
                            witness_b_source_ordinal: Some(notes[b].source_ordinal),
                            observed_primary: i64::from(jump),
                            observed_secondary: dt,
                            threshold_primary: i64::from(profile.jump_limit()),
                            threshold_secondary: 96,
                            focus_score: focus.score,
                            tie_break_code,
                        },
                    ),
                );
                added += 1;
            }
        }
    }
}

fn choose_hotspot_jump_note_to_remove(notes: &[MidiSeedNote], a: usize, b: usize, staff: u8, profile: &CandidateProfile) -> usize {
    if staff == 1 {
        // Rechts: Oberstimme bevorzugt behalten. Tiefe Ausreisser sind meist
        // Ursache der schnellen Sprungwarnung.
        if notes[a].midi <= profile.right_low_soft_limit() { return a; }
        if notes[b].midi <= profile.right_low_soft_limit() { return b; }
        if notes[a].midi < notes[b].midi { a } else { b }
    } else {
        // Links: Bass bevorzugt behalten. Hohe Ausreisser gehoeren eher nicht
        // in dieselbe linke Handbewegung.
        if notes[a].midi >= profile.left_high_soft_limit() { return a; }
        if notes[b].midi >= profile.left_high_soft_limit() { return b; }
        if notes[a].midi > notes[b].midi { a } else { b }
    }
}

fn redistribute_midi_seed_hands_for_piano(
    notes: &mut [MidiSeedNote],
    profile: &CandidateProfile,
    hotspots: Option<&HotspotPlan>,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) {
    // v0.1.38: MIDI-Seed-Material ist Rohmaterial, keine fertige
    // MusicXML-Stimmenachse. Deshalb darf EtuSchlopf hier ausdrücklich
    // zwischen rechter und linker Hand umverteilen. Das alte Prinzip
    // "Stimmenachse erhalten" gilt erst nach dieser internen Handverteilung.
    let mut by_start: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (idx, note) in notes.iter().enumerate() {
        by_start.entry(note.pos).or_default().push(idx);
    }
    for indices in by_start.values() {
        if indices.len() < 2 {
            continue;
        }

        let mut group = indices
            .iter()
            .map(|idx| (*idx, notes[*idx].midi))
            .collect::<Vec<_>>();
        group.sort_by_key(|(_, midi)| *midi);

        let low_idx = group.first().map(|(idx, _)| *idx).unwrap_or(0);
        let high_idx = group.last().map(|(idx, _)| *idx).unwrap_or(low_idx);
        let span = group.last().map_or(0, |(_, midi)| *midi)
            - group.first().map_or(0, |(_, midi)| *midi);
        let measure = note_measure(notes[*indices.first().expect("non-empty onset group")].pos);
        let focus_score = hotspots.map_or(0, |h| h.focus_score_for(measure, None));
        let local_profile = profile.focused(focus_score);
        if span <= local_profile.span_limit() {
            continue;
        }

        let split = choose_piano_hand_split(&group, &local_profile);
        let split_code = u8::try_from(split).unwrap_or(u8::MAX);
        for (order, (idx, _midi)) in group.iter().enumerate() {
            let new_staff = if order < split { 2 } else { 1 };
            if notes[*idx].staff != new_staff {
                let evidence = staff_move_decision(
                    &notes[*idx],
                    new_staff,
                    SourceSelectionDecisionContext {
                        pass: SourceSelectionDecisionPass::PianoHandSplit,
                        pass_iteration: 0,
                        witness_a_source_ordinal: Some(notes[low_idx].source_ordinal),
                        witness_b_source_ordinal: Some(notes[high_idx].source_ordinal),
                        observed_primary: i64::from(span),
                        observed_secondary: i64::try_from(split).unwrap_or(i64::MAX),
                        threshold_primary: i64::from(local_profile.span_limit()),
                        threshold_secondary: i64::try_from(group.len()).unwrap_or(i64::MAX),
                        focus_score,
                        tie_break_code: split_code,
                    },
                );
                decision_trace.push(evidence);
                notes[*idx].staff = new_staff;
            }
        }
    }

    if profile.smooth_jumps() {
        smooth_fast_staff_jumps(notes, profile, hotspots, decision_trace);
        // Nach der Sprungglättung erneut weite Gleichzeitigkeiten entspannen.
        redistribute_wide_simultaneities(
            notes,
            profile,
            hotspots,
            SourceSelectionDecisionPass::SmoothWideSimultaneity,
            0,
            decision_trace,
        );
    }
}

fn redistribute_wide_simultaneities(
    notes: &mut [MidiSeedNote],
    profile: &CandidateProfile,
    hotspots: Option<&HotspotPlan>,
    pass: SourceSelectionDecisionPass,
    pass_iteration: usize,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) {
    let mut by_start: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (idx, note) in notes.iter().enumerate() {
        by_start.entry(note.pos).or_default().push(idx);
    }
    for indices in by_start.values() {
        if indices.len() < 2 {
            continue;
        }
        let mut group = indices.iter().map(|idx| (*idx, notes[*idx].midi)).collect::<Vec<_>>();
        group.sort_by_key(|(_, midi)| *midi);
        let low_idx = group.first().map(|(idx, _)| *idx).unwrap_or(0);
        let high_idx = group.last().map(|(idx, _)| *idx).unwrap_or(low_idx);
        let span = group.last().map_or(0, |(_, midi)| *midi)
            - group.first().map_or(0, |(_, midi)| *midi);
        let measure = note_measure(notes[*indices.first().expect("non-empty onset group")].pos);
        let focus_score = hotspots.map_or(0, |h| h.focus_score_for(measure, None));
        let local_profile = profile.focused(focus_score);
        if span <= local_profile.span_limit() {
            continue;
        }
        let split = choose_piano_hand_split(&group, &local_profile);
        let split_code = u8::try_from(split).unwrap_or(u8::MAX);
        for (order, (idx, _midi)) in group.iter().enumerate() {
            let new_staff = if order < split { 2 } else { 1 };
            if notes[*idx].staff != new_staff {
                let evidence = staff_move_decision(
                    &notes[*idx],
                    new_staff,
                    SourceSelectionDecisionContext {
                        pass,
                        pass_iteration,
                        witness_a_source_ordinal: Some(notes[low_idx].source_ordinal),
                        witness_b_source_ordinal: Some(notes[high_idx].source_ordinal),
                        observed_primary: i64::from(span),
                        observed_secondary: i64::try_from(split).unwrap_or(i64::MAX),
                        threshold_primary: i64::from(local_profile.span_limit()),
                        threshold_secondary: i64::try_from(group.len()).unwrap_or(i64::MAX),
                        focus_score,
                        tie_break_code: split_code,
                    },
                );
                decision_trace.push(evidence);
                notes[*idx].staff = new_staff;
            }
        }
    }
}

fn smooth_fast_staff_jumps(
    notes: &mut [MidiSeedNote],
    profile: &CandidateProfile,
    hotspots: Option<&HotspotPlan>,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) {
    let max_passes = if hotspots.is_none_or(|h| h.is_empty()) { 3 } else { 24 };
    for pass_iteration in 0..max_passes {
        let mut changed = false;
        for staff in [1_u8, 2_u8] {
            let mut indices = notes
                .iter()
                .enumerate()
                .filter(|(_, note)| note.staff == staff)
                .map(|(idx, _)| idx)
                .collect::<Vec<_>>();
            indices.sort_by_key(|idx| (notes[*idx].pos, notes[*idx].midi));

            for pair in indices.windows(2) {
                let a = pair[0];
                let b = pair[1];
                let dt = (notes[b].pos - notes[a].pos).abs();
                let measure = note_measure(notes[a].pos.min(notes[b].pos));
                let focus_score = hotspots.map_or(0, |h| h.focus_score_for(measure, Some(staff)));
                let local_profile = profile.focused(focus_score);
                let dt_limit = if local_profile.jump_limit() < profile.jump_limit() { 144 } else { 96 };
                if dt > dt_limit {
                    continue;
                }
                let jump = (notes[b].midi - notes[a].midi).abs();
                if jump <= local_profile.jump_limit() {
                    continue;
                }

                let target_idx = choose_jump_reassignment_target(notes, a, b, staff, &local_profile);
                let Some(target_idx) = target_idx else {
                    continue;
                };
                let new_staff = if staff == 1 { 2 } else { 1 };
                if notes[target_idx].staff == new_staff {
                    continue;
                }
                let tie_break_code = (if target_idx == a { 1_u8 } else { 2_u8 })
                    + if staff == 2 { 10_u8 } else { 0_u8 };
                let evidence = staff_move_decision(
                    &notes[target_idx],
                    new_staff,
                    SourceSelectionDecisionContext {
                        pass: SourceSelectionDecisionPass::SmoothFastStaffJump,
                        pass_iteration,
                        witness_a_source_ordinal: Some(notes[a].source_ordinal),
                        witness_b_source_ordinal: Some(notes[b].source_ordinal),
                        observed_primary: i64::from(jump),
                        observed_secondary: dt,
                        threshold_primary: i64::from(local_profile.jump_limit()),
                        threshold_secondary: dt_limit,
                        focus_score,
                        tie_break_code,
                    },
                );
                decision_trace.push(evidence);
                notes[target_idx].staff = new_staff;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

fn choose_jump_reassignment_target(
    notes: &[MidiSeedNote],
    a: usize,
    b: usize,
    staff: u8,
    profile: &CandidateProfile,
) -> Option<usize> {
    let ma = notes[a].midi;
    let mb = notes[b].midi;
    if staff == 2 {
        // Linke Hand springt in ein hohes Register: hohe Note eher rechts.
        if ma >= profile.left_high_soft_limit() && mb < ma {
            return Some(a);
        }
        if mb >= profile.left_high_soft_limit() {
            return Some(b);
        }
        if ma > mb { Some(a) } else { Some(b) }
    } else {
        // Rechte Hand greift zu weit nach unten: tiefe Note eher links.
        if ma <= profile.right_low_soft_limit() && mb > ma {
            return Some(a);
        }
        if mb <= profile.right_low_soft_limit() {
            return Some(b);
        }
        if ma < mb { Some(a) } else { Some(b) }
    }
}

fn choose_piano_hand_split(group: &[(usize, i32)], profile: &CandidateProfile) -> usize {
    let len = group.len();
    if len <= 1 {
        return 1;
    }

    let mut best_split = 1usize;
    let mut best_cost = i64::MAX;
    for split in 1..len {
        let left = &group[..split];
        let right = &group[split..];
        let left_low = left.first().map_or(0, |(_, midi)| *midi);
        let left_high = left.last().map_or(left_low, |(_, midi)| *midi);
        let right_low = right.first().map_or(0, |(_, midi)| *midi);
        let right_high = right.last().map_or(right_low, |(_, midi)| *midi);
        let left_span = left_high - left_low;
        let right_span = right_high - right_low;

        let mut cost = 0_i64;
        cost += (left_span.saturating_sub(profile.span_limit()) as i64) * 80;
        cost += (right_span.saturating_sub(profile.span_limit()) as i64) * 80;
        cost += (left.len().saturating_sub(4) as i64) * 25;
        cost += (right.len().saturating_sub(3) as i64) * 25;

        // Register-Plausibilität: tiefe und mittlere Töne eher links,
        // Oberstimme sicher rechts. Nur weiche Strafen, weil Umverteilung
        // wichtiger ist als der alte C4-Split.
        if left_high > profile.left_high_soft_limit() {
            cost += ((left_high - profile.left_high_soft_limit()) as i64) * 10;
        }
        if right_low < profile.right_low_soft_limit() {
            cost += ((profile.right_low_soft_limit() - right_low) as i64) * 10;
        }

        // Oberstimme schützen: die rechte Hand soll mindestens die höchste
        // Note behalten, und bei machbarer Spannweite auch eine zweite Farbe.
        if right.len() == 1 && len >= 4 && right_span <= 14 {
            cost += 5;
        }

        // Bevorzuge echte Entspannung gegenüber minimalen Staff-Wechseln.
        cost += (left_span.max(right_span) as i64) * 3;

        if cost < best_cost {
            best_cost = cost;
            best_split = split;
        }
    }

    best_split
}

fn note_measure(pos: i64) -> usize {
    (pos / MEASURE_DIVISIONS).max(0) as usize + 1
}

fn build_musicxml(
    title: &str,
    notes: &[MidiSeedNote],
    measures: usize,
    rhythmic_courses: &mut MidiSeedRhythmicCourses,
) -> anyhow::Result<(String, Vec<usize>, Vec<MidiSeedSerializedSourceIdentityRow>)> {
    rhythmic_courses.begin_emission(notes, measures)?;
    let mut by_measure_staff: Vec<[Vec<MidiSeedNote>; 2]> = (0..measures)
        .map(|_| [Vec::new(), Vec::new()])
        .collect();

    for note in notes {
        let measure_idx = (note.pos / MEASURE_DIVISIONS).max(0) as usize;
        if measure_idx >= measures {
            continue;
        }
        let staff_idx = if note.staff == 1 { 0 } else { 1 };
        by_measure_staff[measure_idx][staff_idx].push(*note);
    }

    let mut xml = String::new();
    let mut serialized_source_identity_rows =
        Vec::<MidiSeedSerializedSourceIdentityRow>::with_capacity(notes.len());
    let mut measure_raw_note_counts = Vec::<usize>::with_capacity(measures);
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<score-partwise version=\"4.0\">\n");
    xml.push_str(&format!("  <movement-title>{}</movement-title>\n", escape_xml(title)));
    // v0.1.158: Der sichtbare Notenanfang muss vor der Statusprüfung
    // wie professioneller Notensatz wirken. Lektorat/Version bleiben daher
    // in den MusicXML-Metadaten, aber nicht mehr als Staff-Text über dem Notenanfang.
    // So konkurrieren sie nicht mit Tempo und Spielanweisung.
    xml.push_str("  <identification>\n");
    xml.push_str("    <creator type=\"editor\">EtuSchlopf External Review</creator>\n");
    xml.push_str(&format!("    <encoding><software>etuschlopf v{}</software></encoding>\n", env!("CARGO_PKG_VERSION")));
    xml.push_str("  </identification>\n");
    xml.push_str("  <part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list>\n");
    xml.push_str("  <part id=\"P1\">\n");

    for (idx, staff_notes) in by_measure_staff.iter_mut().enumerate() {
        let measure_no = idx + 1;
        xml.push_str(&format!("    <measure number=\"{measure_no}\">\n"));
        if idx == 0 {
            xml.push_str("      <attributes>\n");
            xml.push_str(&format!("        <divisions>{DIVISIONS}</divisions>\n"));
            xml.push_str("        <key><fifths>0</fifths></key>\n");
            xml.push_str("        <time><beats>4</beats><beat-type>4</beat-type></time>\n");
            xml.push_str("        <staves>2</staves>\n");
            xml.push_str("        <clef number=\"1\"><sign>G</sign><line>2</line></clef>\n");
            xml.push_str("        <clef number=\"2\"><sign>F</sign><line>4</line></clef>\n");
            xml.push_str("      </attributes>\n");
        }

        let mut raw_note_index = 0usize;
        let (upper_staff, lower_staff) = staff_notes.split_at_mut(1);
        append_staff_voice(
            &mut xml,
            &mut upper_staff[0],
            idx as i64,
            MidiSeedStaffVoiceSerialization {
                staff: 1,
                voice: "1",
                stem: "up",
                measure_ordinal: idx.saturating_add(1),
                raw_note_index: &mut raw_note_index,
                serialized_source_identity_rows: &mut serialized_source_identity_rows,
                rhythmic_courses,
            },
        )?;
        xml.push_str("      <backup><duration>384</duration></backup>\n");
        append_staff_voice(
            &mut xml,
            &mut lower_staff[0],
            idx as i64,
            MidiSeedStaffVoiceSerialization {
                staff: 2,
                voice: "2",
                stem: "down",
                measure_ordinal: idx.saturating_add(1),
                raw_note_index: &mut raw_note_index,
                serialized_source_identity_rows: &mut serialized_source_identity_rows,
                rhythmic_courses,
            },
        )?;
        measure_raw_note_counts.push(raw_note_index);
        xml.push_str("    </measure>\n");
    }

    xml.push_str("  </part>\n</score-partwise>\n");
    rhythmic_courses.seal_emission(&xml)?;
    Ok((
        xml,
        measure_raw_note_counts,
        serialized_source_identity_rows,
    ))
}

struct MidiSeedStaffVoiceSerialization<'a> {
    staff: u8,
    voice: &'a str,
    stem: &'a str,
    measure_ordinal: usize,
    raw_note_index: &'a mut usize,
    serialized_source_identity_rows: &'a mut Vec<MidiSeedSerializedSourceIdentityRow>,
    rhythmic_courses: &'a mut MidiSeedRhythmicCourses,
}

fn append_staff_voice(
    xml: &mut String,
    notes: &mut [MidiSeedNote],
    measure_idx: i64,
    serialization: MidiSeedStaffVoiceSerialization<'_>,
) -> anyhow::Result<()> {
    let MidiSeedStaffVoiceSerialization {
        staff,
        voice,
        stem,
        measure_ordinal,
        raw_note_index,
        serialized_source_identity_rows,
        rhythmic_courses,
    } = serialization;
    let measure_start = measure_idx * MEASURE_DIVISIONS;
    let course = rhythmic_courses.courses.iter().find(|course| course.staff == staff)
        .ok_or_else(|| anyhow!("Missing generated primary course"))?.creation;
    let planned = rhythmic_courses.planned.iter().filter(|occurrence| {
        occurrence.course == course && occurrence.interval.start >= measure_start
            && occurrence.interval.start < measure_start + MEASURE_DIVISIONS
    }).cloned().collect::<Vec<_>>();
    let expected_sources = planned.iter().filter_map(|occurrence| occurrence.source)
        .collect::<BTreeSet<_>>();
    if expected_sources != notes.iter().map(|note| note.source_ordinal).collect() {
        return Err(anyhow!("Writer input differs from planned rhythmic source membership"));
    }
    for occurrence in planned {
        let duration = occurrence.interval.end - occurrence.interval.start;
        let byte_start = xml.len();
        match (occurrence.source, occurrence.pitch) {
            (Some(source_ordinal), Some(midi)) => {
                serialized_source_identity_rows.push(MidiSeedSerializedSourceIdentityRow {
                    measure_ordinal, raw_note_index: *raw_note_index, source_ordinal,
                });
                append_note(xml, midi, duration, staff, voice, stem, occurrence.chord);
            }
            (None, None) => append_rest(xml, duration, staff, voice),
            _ => return Err(anyhow!("Inconsistent planned rhythmic occurrence")),
        }
        rhythmic_courses.capture_emission(staff, occurrence.interval,
            occurrence.source, byte_start..xml.len())?;
        *raw_note_index = (*raw_note_index).saturating_add(1);
    }
    Ok(())
}

fn append_rest(xml: &mut String, duration: i64, staff: u8, voice: &str) {
    if duration <= 0 {
        return;
    }
    xml.push_str("      <note>\n");
    xml.push_str("        <rest/>\n");
    xml.push_str(&format!("        <duration>{duration}</duration>\n"));
    xml.push_str(&format!("        <voice>{voice}</voice>\n"));
    append_type(xml, duration);
    xml.push_str(&format!("        <staff>{staff}</staff>\n"));
    xml.push_str("      </note>\n");
}

fn append_note(xml: &mut String, midi: i32, duration: i64, staff: u8, voice: &str, stem: &str, chord: bool) {
    let (step, alter, octave) = midi_to_pitch(midi);
    xml.push_str("      <note>\n");
    if chord {
        xml.push_str("        <chord/>\n");
    }
    xml.push_str("        <pitch>\n");
    xml.push_str(&format!("          <step>{step}</step>\n"));
    if alter != 0 {
        xml.push_str(&format!("          <alter>{alter}</alter>\n"));
    }
    xml.push_str(&format!("          <octave>{octave}</octave>\n"));
    xml.push_str("        </pitch>\n");
    xml.push_str(&format!("        <duration>{duration}</duration>\n"));
    xml.push_str(&format!("        <voice>{voice}</voice>\n"));
    append_type(xml, duration);
    xml.push_str(&format!("        <stem>{stem}</stem>\n"));
    xml.push_str(&format!("        <staff>{staff}</staff>\n"));
    xml.push_str("      </note>\n");
}

fn append_type(xml: &mut String, duration: i64) {
    let (typ, dots) = type_and_dots_for_duration(duration);
    xml.push_str(&format!("        <type>{typ}</type>\n"));
    for _ in 0..dots {
        xml.push_str("        <dot/>\n");
    }
}

pub fn type_and_dots_for_duration(duration: i64) -> (&'static str, usize) {
    match duration {
        384 => ("whole", 0),
        288 => ("half", 1),
        192 => ("half", 0),
        144 => ("quarter", 1),
        96 => ("quarter", 0),
        72 => ("eighth", 1),
        48 => ("eighth", 0),
        36 => ("16th", 1),
        24 => ("16th", 0),
        18 => ("32nd", 1),
        12 => ("32nd", 0),
        9 => ("64th", 1),
        6 => ("64th", 0),
        d if d >= 384 => ("whole", 0),
        d if d >= 288 => ("half", 1),
        d if d >= 192 => ("half", 0),
        d if d >= 144 => ("quarter", 1),
        d if d >= 96 => ("quarter", 0),
        d if d >= 72 => ("eighth", 1),
        d if d >= 48 => ("eighth", 0),
        d if d >= 36 => ("16th", 1),
        d if d >= 24 => ("16th", 0),
        d if d >= 18 => ("32nd", 1),
        d if d >= 12 => ("32nd", 0),
        _ => ("64th", 0),
    }
}

fn midi_to_pitch(midi: i32) -> (&'static str, i32, i32) {
    let pc = midi.rem_euclid(12);
    let octave = midi / 12 - 1;
    match pc {
        0 => ("C", 0, octave),
        1 => ("C", 1, octave),
        2 => ("D", 0, octave),
        3 => ("E", -1, octave),
        4 => ("E", 0, octave),
        5 => ("F", 0, octave),
        6 => ("F", 1, octave),
        7 => ("G", 0, octave),
        8 => ("G", 1, octave),
        9 => ("A", 0, octave),
        10 => ("B", -1, octave),
        _ => ("B", 0, octave),
    }
}

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
#[path = "../tests/unit/midi_seed_rhythmic_course_creation.rs"]
pub(crate) mod rhythmic_course_creation_tests;
