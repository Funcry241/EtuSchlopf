// v0.10.725 FoundationIoRealModuleSplitNoBehaviorChange.
// Real crate-root module split from the former mechanical IO grouping.
// Preserves behavior from the former FLP module; no wrapper/include shell.

// --- mechanical module boundary: former src/flp.rs ---
use crate::midi::{MidiKeySignature, read_standard_midi};
use crate::midi::notation_time::{NotationEventKind, NotationTimePlan};
use crate::midi::source_time::MetricTimeline;
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
#[cfg(test)]
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
    pub(crate) notation_time: std::sync::Arc<NotationTimePlan>,
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
    voice: Option<usize>,
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
    independent: bool,
    opposite_stem: bool,
    source_voice: Option<usize>,
    tie_stop: bool,
    tie_start: bool,
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
    metric: MetricTimeline,
    emission_snapshot: Option<String>,
    source_voices: bool,
    preserve_attacks: bool,
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

/// Returns the source of the independently held note when an exact
/// source pair has two written releases inside one measure.
fn select_exact_pair_sustain_source(
    onsets: &[i64],
    notes_by_onset: &BTreeMap<i64, Vec<MidiSeedNote>>,
    measure_end: i64,
) -> Option<usize> {
    let [start, next, ..] = onsets else { return None; };
    let [first, second] = notes_by_onset.get(start)?.as_slice() else { return None; };
    let (short, long) = if first.len <= second.len { (first, second) } else { (second, first) };
    let end = start.checked_add(long.len)?;
    if short.len != next - start || *next >= end || end > measure_end
        || !matches!(short.len, 96 | 72 | 48 | 36 | 24 | 18 | 12 | 9 | 6)
        || !matches!(long.len, 288 | 192 | 144 | 96 | 72 | 48 | 36 | 24 | 18 | 12 | 9 | 6)
    {
        return None;
    }

    for (&onset, release) in onsets[1..].iter()
        .zip(onsets[2..].iter().copied().chain(std::iter::once(measure_end)))
    {
        let notes = notes_by_onset.get(&onset)?;
        if release > end || notes.iter().any(|note| note.midi == long.midi || note.len != release - onset) {
            return None;
        }
        if release == end {
            return Some(long.source_ordinal);
        }
    }
    None
}

// Keep an independently held note when the primary lane can carry the
// intervening attacks and the auxiliary lane has a supported written value.
fn select_inferred_sustain_source(
    onsets: &[i64],
    notes_by_onset: &BTreeMap<i64, Vec<MidiSeedNote>>,
    measure_end: i64,
) -> Option<usize> {
    let [start, next, ..] = onsets else { return None };
    let group = notes_by_onset.get(start)?;
    if !group.iter().any(|note| note.len == next - start) {
        return None;
    }
    group.iter().filter(|note| {
        let end = start + note.len;
        end > *next && end <= measure_end
            && matches!(note.len, 192 | 144 | 96 | 72 | 48 | 36 | 24)
            && onsets[1..].iter().take_while(|&&onset| onset < end).all(|onset| {
                notes_by_onset[onset].iter().all(|attack| attack.midi != note.midi)
            })
    }).max_by_key(|note| (note.len, std::cmp::Reverse(note.source_ordinal)))
        .map(|note| note.source_ordinal)
}

// A complete upper note may span several lower attacks even if its
// first attack has no short partner on the same staff.
fn select_upper_independent_sustain(
    onsets: &[i64],
    notes_by_onset: &BTreeMap<i64, Vec<MidiSeedNote>>,
    measure_end: i64,
) -> Option<usize> {
    let [start, next, ..] = onsets else { return None; };
    let group = notes_by_onset.get(start)?;
    let upper = group.iter().max_by_key(|note| note.midi)?;
    let release = start + upper.len;
    if release > measure_end || !matches!(upper.len, 192 | 144 | 96 | 72 | 48 | 36)
        || group.iter().filter(|note| note.midi == upper.midi).count() != 1
        || group.iter().any(|note| note.source_ordinal != upper.source_ordinal && note.len > next - start)
    {
        return None;
    }
    let mut lower_attacks = onsets[1..].iter().take_while(|&&onset| onset < release);
    if lower_attacks.clone().count() < 2
        || !lower_attacks.all(|onset| notes_by_onset[onset].iter().all(|note| note.midi < upper.midi))
    {
        return None;
    }
    Some(upper.source_ordinal)
}

fn select_upper_sustains_with_distinct_releases(
    onsets: &[i64], notes_by_onset: &BTreeMap<i64, Vec<MidiSeedNote>>,
    bar_start: i64, bar_end: i64, beat: i64,
) -> Option<[(MidiSeedNote, Vec<std::ops::Range<i64>>); 2]> {
    let [start, _, ..] = onsets else { return None; };
    let group = notes_by_onset.get(start)?;
    let mut ranked = group.to_vec();
    ranked.sort_by_key(|note| std::cmp::Reverse(note.midi));
    let [upper, lower, ..] = ranked.as_slice() else { return None; };
    if upper.midi == lower.midi || upper.len == lower.len
        || ranked[2..].iter().any(|note| note.midi == lower.midi)
    {
        return None;
    }
    let parts = |source: MidiSeedNote| {
        let release = source.pos + source.len;
        let following = onsets[1..].iter().take_while(|&&onset| onset < release);
        if release > bar_end || following.clone().count() < 2
            || following.clone().any(|onset| notes_by_onset[onset].iter().any(|note| note.midi == source.midi))
            || !following.clone().any(|onset| notes_by_onset[onset].iter().any(|note| note.midi < lower.midi))
            || (source.midi == upper.midi && following.clone().any(|onset| {
                notes_by_onset[onset].iter().any(|note| note.midi > upper.midi)
            }))
        {
            return None;
        }
        let mut fragments = Vec::new();
        let mut cursor = source.pos;
        while cursor < release {
            let end = (cursor + beat - (cursor - bar_start) % beat).min(release);
            if !matches!(end - cursor, 96 | 72 | 48 | 36 | 24 | 18 | 12 | 9 | 6) {
                return None;
            }
            fragments.push(cursor..end);
            cursor = end;
        }
        Some((source, fragments))
    };
    Some([parts(*upper)?, parts(*lower)?])
}

// An upper held note can continue through the rest of the beat as its own
// melodic line. Require exact releases, stepwise motion and lower activity.
fn upper_sustain_continuation(
    onsets: &[i64],
    notes_by_onset: &BTreeMap<i64, Vec<MidiSeedNote>>,
    sustain: MidiSeedNote,
    beat_end: i64,
) -> Option<Vec<MidiSeedNote>> {
    let end = sustain.pos + sustain.len;
    if end >= beat_end || !onsets.iter().any(|&onset| sustain.pos < onset && onset < end) {
        return None;
    }
    let mut continuation = vec![sustain];
    let mut pos = end;
    let mut pitch = sustain.midi;
    while pos < beat_end {
        let group = notes_by_onset.get(&pos)?;
        let head = group.iter().max_by_key(|note| note.midi)?;
        if !(1..=2).contains(&(head.midi - pitch).abs())
            || !matches!(head.len, 48 | 36 | 24 | 18 | 12 | 9 | 6)
            || head.len > beat_end - pos
            || group.iter().filter(|note| note.midi == head.midi).count() != 1
        {
            return None;
        }
        continuation.push(*head);
        pos += head.len;
        pitch = head.midi;
    }
    (continuation.len() > 2 && onsets.iter().any(|&onset| {
        onset >= end && onset < beat_end && notes_by_onset[&onset].len() > 1
    })).then_some(continuation)
}

#[derive(Debug, Clone, Copy)]
struct SourceVoice {
    staff: u8,
    voice: usize,
    opposite_stem: bool,
}

// Only explicit, unambiguous vocal track names establish written voices.
fn qualified_source_voices(
    notes: &[MidiSeedNote],
    source: &crate::midi::DecodedMidiSource,
    metric: &MetricTimeline,
) -> Option<BTreeMap<usize, SourceVoice>> {
    use crate::midi::MidiVoiceRole::{Alto, Bass, Soprano, Tenor};

    let mut lanes = BTreeMap::<_, Vec<_>>::new();
    let by_ordinal = notes.iter().map(|note| (note.source_ordinal, note)).collect::<BTreeMap<_, _>>();
    for entry in &source.source_event_causal_ledger.entries {
        let note = *by_ordinal.get(&entry.source_ordinal)?;
        let track = entry.note_on.track_index;
        if note.len <= 0 || track != entry.note_off.track_index { return None; }
        let role = source.voice_roles.get(track).copied().flatten()?;
        lanes.entry((role, track, entry.note_on.channel)).or_default().push(note);
    }
    let roles = lanes.keys().map(|&(role, _, _)| role).collect::<BTreeSet<_>>();
    if roles.len() < 2 || roles.len() != lanes.len() { return None; }
    let bass = lanes.iter().find_map(|(&(role, _, _), lane)| (role == Bass).then_some(lane));
    let lower_alto = match (bass, roles.contains(&Soprano), roles.contains(&Tenor)) {
        (Some(bass), true, false) => lanes.iter().find_map(|(&(role, _, _), lane)|
            (role == Alto).then(|| low_alto_source_measures(lane, bass, metric))).unwrap_or_default(),
        _ => BTreeSet::new(),
    };
    let mut assignments = BTreeMap::new();
    for ((role, _, _), mut lane) in lanes {
        lane.sort_by_key(|note| (note.pos, note.midi));
        if lane.windows(2).any(|w| {
            w[0].pos + w[0].len > w[1].pos
                && (w[0].pos != w[1].pos || w[0].len != w[1].len || w[0].midi == w[1].midi)
        }) {
            return None;
        }
        let (staff, voice, opposite_stem) = match role {
            Soprano => (1, 1, false),
            Alto if roles.contains(&Soprano) => (1, 3, true),
            Alto => (1, 1, false),
            Tenor => (2, 2, roles.contains(&Bass)),
            Bass if roles.contains(&Tenor) => (2, 4, false),
            Bass => (2, 2, false),
        };
        for note in lane {
            let move_alto = role == Alto
                && metric.ordinal_at(note.pos).is_some_and(|index| lower_alto.contains(&index));
            let assignment = if move_alto {
                SourceVoice { staff: 2, voice, opposite_stem: true }
            } else {
                SourceVoice { staff, voice, opposite_stem }
            };
            assignments.insert(note.source_ordinal, assignment);
        }
    }
    (assignments.len() == notes.len()).then_some(assignments)
}

// Compare the complete measured line on the two piano staves. Keep the
// bass below every simultaneous alto note and prefer a sustained relocation.
fn low_alto_source_measures(
    alto: &[&MidiSeedNote],
    bass: &[&MidiSeedNote],
    metric: &MetricTimeline,
) -> BTreeSet<usize> {
    let ledger = |midi: i32, low: i32, high: i32| {
        let (step, _, octave) = midi_to_pitch(midi);
        let offset = "CDEFGAB".find(step).expect("seed diatonic pitch") as i32;
        let diatonic = octave * 7 + offset;
        (low - diatonic).max(diatonic - high).max(0) / 2
    };
    let mut regions = BTreeSet::new();
    for index in 0..metric.len() {
        let Some(bar) = metric.measure(index) else { continue; };
        let line: Vec<_> = alto.iter().copied()
            .filter(|note| note.pos < bar.end && bar.start < note.pos + note.len)
            .collect();
        if line.len() < 2 || line.iter().any(|note| note.pos < bar.start || note.pos + note.len > bar.end) {
            continue;
        }
        let mut gain = 0;
        let mut overlapping_bass = false;
        let mut spaced = true;
        for note in &line {
            // E4 to F5 in treble and G2 to A3 in bass.
            gain += ledger(note.midi, 30_i32, 38_i32) - ledger(note.midi, 18_i32, 26_i32);
            for low in bass.iter().filter(|low| low.pos < note.pos + note.len && note.pos < low.pos + low.len) {
                overlapping_bass = true;
                if note.midi - low.midi < 3 { spaced = false; }
            }
        }
        if overlapping_bass && spaced && gain >= line.len() as i32 {
            regions.insert(index);
        }
    }
    regions.iter().copied().filter(|&index| regions.contains(&(index + 1))
        || index.checked_sub(1).is_some_and(|before| regions.contains(&before))).collect()
}

pub(crate) fn aligned_long_tie_value(
    start: i64, duration: i64, measure_duration: i64, beat_duration: i64,
) -> bool {
    measure_duration == 384 && beat_duration == 96
        && matches!((start, duration), (0, 384) | (0 | 192, 192))
}

fn source_voice_fragments(
    sources: &[MidiSeedNote],
    metric: &MetricTimeline,
    assignments: &BTreeMap<usize, SourceVoice>,
    courses: &[SeedPrimaryCourse],
) -> anyhow::Result<Vec<PlannedSeedOccurrence>> {
    let mut measures = BTreeMap::<_, Vec<_>>::new();
    for source in sources {
        let voice = assignments[&source.source_ordinal];
        let course = courses.iter().find(|c| c.staff == voice.staff && c.voice == Some(voice.voice)
            && c.coverage.contains(&source.pos))
            .ok_or_else(|| anyhow!("Missing source voice staff"))?;
        let release = source.pos + source.len;
        let mut start = source.pos;
        while start < release {
            let bar = metric.ordinal_at(start).and_then(|i| metric.measure(i))
                .ok_or_else(|| anyhow!("Source voice escapes the metric"))?;
            let duration = bar.end.min(release) - start;
            let beat = metric.divisions * 4 / i64::from(bar.beat_type);
            let bar_offset = start - bar.start;
            let part = if (start == source.pos && duration == source.len)
                || (bar.beats == 4 && bar.beat_type == 4
                    && aligned_long_tie_value(bar_offset, duration, bar.end - bar.start, beat))
            {
                duration
            } else {
                let until_beat = beat - bar_offset % beat;
                [96, 72, 48, 36, 24, 18, 12, 6]
                    .into_iter().find(|part| *part <= duration.min(until_beat))
                    .ok_or_else(|| anyhow!("Source fragment duration is unsupported"))?
            };
            measures.entry((bar.start, course.creation)).or_default().push(PlannedSeedOccurrence {
                course: course.creation,
                interval: start..start + part,
                source: Some(source.source_ordinal),
                pitch: Some(source.midi),
                chord: false,
                independent: true,
                opposite_stem: voice.opposite_stem,
                source_voice: Some(voice.voice),
                tie_stop: start != source.pos,
                tie_start: start + part != release,
            });
            start += part;
        }
    }
    let mut plan = Vec::new();
    for index in 0..metric.len() {
        let bar = metric.measure(index).ok_or_else(|| anyhow!("Missing source voice measure"))?;
        for course in courses.iter().filter(|course| course.coverage.contains(&bar.start)) {
            let mut notes = measures.remove(&(bar.start, course.creation)).unwrap_or_default();
            notes.sort_by_key(|note| (note.interval.start, note.source_voice, note.pitch));
            let mut covered = bar.start;
            let mut host = None;
            for mut note in notes {
                if covered < note.interval.start {
                    plan.push(PlannedSeedOccurrence {
                        course: course.creation, interval: covered..note.interval.start,
                        source: None, pitch: None, chord: false, independent: false, opposite_stem: false,
                        source_voice: course.voice, tie_stop: false, tie_start: false,
                    });
                }
                note.chord = host.as_ref() == Some(&(note.interval.clone(), note.source_voice));
                host = Some((note.interval.clone(), note.source_voice));
                covered = covered.max(note.interval.end);
                plan.push(note);
            }
            if covered < bar.end {
                plan.push(PlannedSeedOccurrence {
                    course: course.creation, interval: covered..bar.end,
                    source: None, pitch: None, chord: false, independent: false, opposite_stem: false,
                    source_voice: course.voice, tie_stop: false, tie_start: false,
                });
            }
        }
    }
    Ok(plan)
}

fn cross_bar_sustain_group(
    attack: &[MidiSeedNote], sources: &[MidiSeedNote], bar_start: i64, boundary: i64,
    next_end: i64,
) -> Vec<MidiSeedNote> {
    let mut held = attack.iter().copied().filter(|note| note.pos + note.len > boundary)
        .collect::<Vec<_>>();
    let Some(first) = held.first() else { return held; };
    let release = first.pos + first.len;
    let exact_value = |length| matches!(length, 384 | 288 | 192 | 144 | 96 | 72 | 48 | 36 | 24 | 18 | 12 | 9 | 6);
    let head = boundary - first.pos;
    let tail = release - boundary;
    let offset = first.pos - bar_start;
    let mut pitches = BTreeSet::new();
    if release > next_end || !exact_value(head) || !exact_value(tail)
        || (head > 96 - offset % 96 && !aligned_long_tie_value(offset, head, boundary - bar_start, 96))
        || (tail > 96 && !aligned_long_tie_value(0, tail, next_end - boundary, 96))
        || held.iter().any(|note| note.pos + note.len != release || !pitches.insert(note.midi))
        || sources.iter().any(|note| {
            note.pos < release && note.pos + note.len > first.pos && pitches.contains(&note.midi)
                && !held.iter().any(|member| member.source_ordinal == note.source_ordinal)
        })
    {
        return Vec::new();
    }
    held.sort_by_key(|note| note.midi);
    held
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceVoiceGap {
    pub(crate) measure: usize,
    pub(crate) divisions: i64,
    pub(crate) start: i64,
    pub(crate) duration: i64,
    pub(crate) staff: u8,
    pub(crate) seed_voice: usize,
    pub(crate) above_lower_staff: bool,
}

fn assigned_voice_staff_lifetimes(
    sources: &[MidiSeedNote],
    assignments: &BTreeMap<usize, SourceVoice>,
    metric: &MetricTimeline,
    voice: usize,
    staff: u8,
    end: i64,
) -> anyhow::Result<Vec<std::ops::Range<i64>>> {
    let mut staff_by_bar = BTreeMap::new();
    for source in sources {
        let assignment = assignments[&source.source_ordinal];
        if assignment.voice != voice { continue; }
        let index = metric.ordinal_at(source.pos)
            .ok_or_else(|| anyhow!("Assigned voice occurrence outside metric"))?;
        let bar = metric.measure(index)
            .ok_or_else(|| anyhow!("Missing assigned voice measure"))?;
        if staff_by_bar.insert(bar.start, assignment.staff)
            .is_some_and(|previous| previous != assignment.staff)
        {
            return Err(anyhow!("Assigned voice changes staff inside a measure"));
        }
    }
    let mut ranges = Vec::new();
    let mut start = 0;
    let mut current = *staff_by_bar.first_key_value()
        .ok_or_else(|| anyhow!("Missing assigned voice lifetime"))?.1;
    for (boundary, assigned) in staff_by_bar {
        if assigned == current { continue; }
        if current == staff { ranges.push(start..boundary); }
        start = boundary;
        current = assigned;
    }
    if current == staff { ranges.push(start..end); }
    Ok(ranges)
}

impl MidiSeedRhythmicCourses {
    pub(crate) fn physical_measure_duration(&self, ordinal: usize) -> Option<i64> {
        self.metric.measure(ordinal).map(|measure| measure.end - measure.start)
    }

    /// An interior gap of a qualified original voice can cross a barline at
    /// either edge. Full-bar padding and gaps beside absent source notes have
    /// no such witness.
    pub(crate) fn bounded_source_voice_gaps(
        &self,
        active_sources: &BTreeMap<usize, (u8, i64, i64)>,
    ) -> Vec<SourceVoiceGap> {
        if !self.source_voices { return Vec::new(); }
        let mut gaps = Vec::new();
        let staff_by_course = self.courses.iter().map(|course| (course.creation, course.staff))
            .collect::<BTreeMap<_, _>>();
        let mut by_voice = BTreeMap::<_, Vec<_>>::new();
        for occurrence in self.planned.iter().filter(|occurrence| !occurrence.chord) {
            if let Some(voice) = occurrence.source_voice {
                by_voice.entry(voice).or_default().push(occurrence);
            }
        }
        for (seed_voice, lane) in by_voice {
            let mut remaining = lane.as_slice();
            while let Some((left, tail)) = remaining.split_first() {
                let gap_len = tail.iter().take_while(|occurrence| occurrence.source.is_none()).count();
                let (silence, following) = tail.split_at(gap_len);
                remaining = following;
                let (Some(first), Some(last), Some(right)) =
                    (silence.first(), silence.last(), following.first())
                else { continue; };
                if silence.iter().any(|gap| gap.interval.is_empty())
                    || silence.windows(2).any(|pair| pair[0].interval.end != pair[1].interval.start)
                    || left.interval.end != first.interval.start
                    || last.interval.end != right.interval.start
                { continue; }
                let Some((left_staff, _, release)) = left.source.and_then(|id| active_sources.get(&id)) else { continue; };
                let Some((right_staff, attack, _)) = right.source.and_then(|id| active_sources.get(&id)) else { continue; };
                if (*left_staff, *release, *right_staff, *attack)
                    != (staff_by_course[&left.course], first.interval.start,
                        staff_by_course[&right.course], last.interval.end)
                { continue; }
                for gap in silence {
                    let Some(index) = self.metric.ordinal_at(gap.interval.start) else { continue; };
                    let Some(bar) = self.metric.measure(index) else { continue; };
                    if gap.interval.end > bar.end || gap.interval == (bar.start..bar.end) { continue; }
                    let staff = staff_by_course[&gap.course];
                    let adjacent = [left, right];
                    let mut on_staff = adjacent.iter().filter(|note| staff_by_course[&note.course] == staff);
                    let above_lower_staff = staff == 2
                        && on_staff.clone().any(|note| note.opposite_stem)
                        && on_staff.all(|note| note.opposite_stem);
                    gaps.push(SourceVoiceGap {
                        measure: index,
                        divisions: self.metric.divisions,
                        start: gap.interval.start - bar.start,
                        duration: gap.interval.end - gap.interval.start,
                        staff,
                        seed_voice,
                        above_lower_staff,
                    });
                }
            }
        }
        gaps
    }

    pub(crate) fn has_bound_source_voices(&self) -> bool { self.source_voices }

    pub(crate) fn preserves_attacks(&self) -> bool {
        self.preserve_attacks
    }

    pub(crate) fn source_voice_continuation_slots(&self) -> Vec<(usize, usize, usize)> {
        let mut continuations = Vec::new();
        let mut raw_by_measure = BTreeMap::<_, usize>::new();
        for occurrence in &self.planned {
            let measure = self.metric.ordinal_at(occurrence.interval.start)
                .expect("validated seed measure") + 1;
            let raw = raw_by_measure.entry(measure).or_default();
            if occurrence.tie_stop {
                continuations.push((occurrence.source.expect("bound tie continuation"), measure, *raw));
            }
            *raw += 1;
        }
        continuations
    }

    pub(crate) fn independent_note_indices(&self, measure: usize) -> BTreeSet<usize> {
        let Some(bar) = self.metric.measure(measure) else {
            return BTreeSet::new();
        };
        self.planned.iter()
            .filter(|note| (bar.start..bar.end).contains(&note.interval.start))
            .enumerate()
            .filter_map(|(index, note)| note.independent.then_some(index))
            .collect()
    }

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
    #[cfg(test)]
    fn create_primary_presentations(
        sources: &[MidiSeedNote], measures: usize, measure_duration: i64,
    ) -> anyhow::Result<Self> {
        let metric = MetricTimeline::from_midi(crate::midi::source_time::SourceTimeDomain { source_ppq: 96, divisions: 96 },
            &[crate::midi::source_time::MeterChange { tick: 0,
                beats: u8::try_from(measure_duration / 96)?, beat_type: 4 }],
            i64::try_from(measures)?.checked_mul(measure_duration)
                .ok_or_else(|| anyhow!("Test notation domain overflow"))?)?;
        Self::create_metric_presentations(sources, &metric, None, false)
    }

    fn create_metric_presentations(
        sources: &[MidiSeedNote],
        metric: &MetricTimeline,
        source_voices: Option<&BTreeMap<usize, SourceVoice>>,
        preserve_attacks: bool,
    ) -> anyhow::Result<Self> {
        let measures = metric.len();
        let end = metric.measure(measures.saturating_sub(1))
            .ok_or_else(|| anyhow!("Invalid primary notation domain"))?.end;
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
        let presentations = source_voices.map_or_else(
            || BTreeSet::from([(1, None), (2, None)]),
            |voices| {
                let mut presentations = voices.values().map(|v| (v.staff, Some(v.voice)))
                    .collect::<BTreeSet<_>>();
                for staff in [1, 2] {
                    if !presentations.iter().any(|(assigned, _)| *assigned == staff) {
                        presentations.insert((staff, None));
                    }
                }
                presentations
            },
        );
        let mut courses = Vec::new();
        for (staff, voice) in &presentations {
            let mut coverage = std::iter::once(0..end).collect::<Vec<_>>();
            if let Some(voice) = *voice
                && presentations.iter().any(|(other, candidate)| other != staff && *candidate == Some(voice))
                && let Some(voices) = source_voices
            {
                coverage = assigned_voice_staff_lifetimes(sources, voices, metric, voice, *staff, end)?;
            }
            for span in coverage {
                courses.push(SeedPrimaryCourse {
                    creation: RhythmicCreationIdentity::create()?,
                    presentation: SeedCoursePresentation::Primary,
                    staff: *staff,
                    coverage: span,
                    voice: *voice,
                });
            }
        }
        if let Some(source_voices) = source_voices {
            let planned = source_voice_fragments(sources, metric, source_voices, &courses)?;
            return Ok(Self {
                courses, sources: sources.to_vec(), planned, occurrences: Vec::new(),
                metric: metric.clone(), emission_snapshot: None, source_voices: true, preserve_attacks,
            });
        }
        let mut grouped = BTreeMap::<(i64, u8), BTreeMap<i64, Vec<MidiSeedNote>>>::new();
        for source in sources.iter().filter(|s| s.len > 0) {
            grouped.entry((metric.ordinal_at(source.pos).ok_or_else(|| anyhow!("Source outside metric domain"))? as i64, source.staff)).or_default()
                .entry(source.pos).or_default().push(*source);
        }
        let mut planned = Vec::new();
        let mut incoming_sustains = BTreeMap::<(usize, u8), Vec<PlannedSeedOccurrence>>::new();
        for measure in 0..measures {
            let bar = metric.measure(measure).ok_or_else(|| anyhow!("Missing notation measure"))?;
            let begin = bar.start;
            let measure_duration = bar.end - bar.start;
            for course in &courses {
                let mut by_start = grouped.remove(&(measure as i64, course.staff)).unwrap_or_default();
                let starts = by_start.keys().copied().collect::<Vec<_>>();
                let mut cursor = begin;
                let mut independent = incoming_sustains.remove(&(measure, course.staff)).unwrap_or_default();
                let mut second_independent = Vec::<PlannedSeedOccurrence>::new();
                let mut independent_end = independent.last().map_or(begin, |note| note.interval.end);
                let mut upper_line = BTreeSet::new();
                let mut upper_line_end = begin;
                for (index, start) in starts.iter().copied().enumerate() {
                    if cursor < start {
                        planned.push(PlannedSeedOccurrence {
                            course: course.creation, interval: cursor..start,
                            source: None, pitch: None, chord: false, independent: false, opposite_stem: false, source_voice: None, tie_stop: false, tie_start: false,
                        });
                    }
                    let next = starts.get(index + 1).copied().unwrap_or(bar.end);
                    let following = starts.get(index + 2).copied().unwrap_or(bar.end);
                    let across_bar = if preserve_attacks && next < bar.end
                        && start >= independent_end && start >= upper_line_end
                    {
                        metric.measure(measure + 1).map_or_else(Vec::new, |next_bar| {
                            cross_bar_sustain_group(&by_start[&start], sources, bar.start, bar.end, next_bar.end)
                        })
                    } else {
                        Vec::new()
                    };
                    for (member, source) in across_bar.iter().enumerate() {
                        let head = PlannedSeedOccurrence {
                            course: course.creation, interval: start..bar.end,
                            source: Some(source.source_ordinal), pitch: Some(source.midi),
                            chord: member > 0, independent: true, opposite_stem: false,
                            source_voice: None, tie_stop: false, tie_start: true,
                        };
                        independent.push(head.clone());
                        incoming_sustains.entry((measure + 1, course.staff)).or_default().push(
                            PlannedSeedOccurrence { interval: bar.end..source.pos + source.len,
                                tie_stop: true, tie_start: false, ..head });
                        independent_end = bar.end;
                    }
                    let one_exact_intervening_head = by_start.get(&next).is_some_and(|notes| {
                        matches!(notes.as_slice(), [note] if note.len == following - next)
                    });
                    let beat = (metric.divisions * 4 / i64::from(bar.beat_type)).max(1);
                    let exact_pair_sustain = if start >= independent_end {
                        select_exact_pair_sustain_source(&starts[index..], &by_start, bar.end)
                            .or_else(|| preserve_attacks.then(|| {
                                select_inferred_sustain_source(&starts[index..], &by_start, bar.end)
                            }).flatten())
                    } else {
                        None
                    };
                    if preserve_attacks && course.staff == 1
                        && let Some(ordinal) = exact_pair_sustain
                        && let Some(sustain) = by_start[&start].iter().find(|n| n.source_ordinal == ordinal)
                        && by_start[&start].iter().all(|n| n.midi < sustain.midi || n.source_ordinal == ordinal)
                    {
                        let beat_end = (start + beat - (start - begin) % beat).min(bar.end);
                        if let Some(line) = upper_sustain_continuation(&starts[index..], &by_start, *sustain, beat_end) {
                            upper_line = line.iter().map(|n| n.source_ordinal).collect();
                            upper_line_end = beat_end;
                        }
                    }
                    let use_upper_cursor = preserve_attacks && course.staff == 1
                        && start >= upper_line_end && start >= independent_end
                        && exact_pair_sustain.is_none();
                    let paired_sustains = if use_upper_cursor {
                        select_upper_sustains_with_distinct_releases(&starts[index..], &by_start, begin, bar.end, beat)
                    } else { None };
                    let upper_sustain = if use_upper_cursor && paired_sustains.is_none() {
                        select_upper_independent_sustain(&starts[index..], &by_start, bar.end)
                    } else {
                        None
                    };
                    if let Some([upper, lower]) = &paired_sustains {
                        for ((source, fragments), lane) in [(upper, &mut independent), (lower, &mut second_independent)] {
                            let release = source.pos + source.len;
                            for fragment in fragments {
                                lane.push(PlannedSeedOccurrence {
                                    course: course.creation, interval: fragment.clone(),
                                    source: Some(source.source_ordinal), pitch: Some(source.midi),
                                    chord: false, independent: true, opposite_stem: false, source_voice: None,
                                    tie_stop: fragment.start != source.pos, tie_start: fragment.end != release,
                                });
                            }
                            independent_end = independent_end.max(release);
                        }
                    }
                    let group = by_start.get_mut(&start).expect("planned onset exists");
                    group.sort_by_key(|source| (source.midi, source.source_ordinal));
                    if course.staff == 1 { group.reverse(); }
                    if exact_pair_sustain == Some(group[0].source_ordinal) {
                        let primary = group.iter().position(|note| Some(note.source_ordinal) != exact_pair_sustain)
                            .expect("independent sustain has a primary partner");
                        group.swap(0, primary);
                    }
                    for grace in sources.iter().filter(|g| g.len == 0 && g.pos == start && g.staff == course.staff) {
                        planned.push(PlannedSeedOccurrence {
                            course: course.creation, interval: start..start,
                            source: Some(grace.source_ordinal), pitch: Some(grace.midi), chord: false, independent: false, opposite_stem: false, source_voice: None, tie_stop: false, tie_start: false,
                        });
                    }
                    let head_duration = next - start;
                    let head_fits_one_beat = head_duration <= beat - (start - begin) % beat
                        && matches!(head_duration, 384 | 288 | 192 | 144 | 96 | 72 | 48 | 36 | 24 | 18 | 12 | 9 | 6);
                    let primary = group.iter().filter(|source| {
                        Some(source.source_ordinal) != upper_sustain
                            && !paired_sustains.as_ref().is_some_and(|pair| pair.iter().any(|(held, _)| held.source_ordinal == source.source_ordinal))
                            && !across_bar.iter().any(|selected| selected.source_ordinal == source.source_ordinal)
                            && (start >= upper_line_end || !upper_line.contains(&source.source_ordinal))
                    }).collect::<Vec<_>>();
                    if start < upper_line_end || upper_sustain.is_some() {
                        for source in group.iter().filter(|source| {
                            Some(source.source_ordinal) == upper_sustain
                                || (start < upper_line_end && upper_line.contains(&source.source_ordinal))
                        }) {
                            independent.push(PlannedSeedOccurrence {
                                course: course.creation, interval: start..start + source.len,
                                source: Some(source.source_ordinal), pitch: Some(source.midi),
                                chord: false, independent: true, opposite_stem: false,
                                source_voice: None, tie_stop: false, tie_start: false,
                            });
                            independent_end = start + source.len;
                        }
                    }
                    for (member, source) in primary.iter().enumerate() {
                        // The host and member can have independent releases when
                        // the host does not need a tied metric partition.
                        let end = if member > 0 && head_fits_one_beat
                            && matches!(source.len, 384 | 288 | 192 | 144 | 96 | 72 | 48 | 36 | 24 | 18 | 12 | 9 | 6)
                        {
                            next.min(start + source.len)
                        } else {
                            next
                        };
                        let source_end = start + source.len;
                        // A two-note onset can carry a separate sustain across
                        // one complete intervening head within the same beat.
                        let separate = start >= upper_line_end && (exact_pair_sustain == Some(source.source_ordinal)
                            || member > 0 && group.len() == 2
                            && group[0].len == head_duration && one_exact_intervening_head
                            && next < source_end && source_end == following
                            && source.len <= beat - (start - begin) % beat
                            && start >= independent_end);
                        let occurrence = PlannedSeedOccurrence {
                            course: course.creation, interval: start..if separate { source_end } else { end },
                            source: Some(source.source_ordinal), pitch: Some(source.midi),
                            chord: member > 0 && !separate,
                            independent: separate,
                            opposite_stem: start < upper_line_end || upper_sustain.is_some() || paired_sustains.is_some() || !across_bar.is_empty() || exact_pair_sustain == Some(source.source_ordinal)
                                && group[0].midi == source.midi,
                            source_voice: None, tie_stop: false, tie_start: false,
                        };
                        if separate {
                            independent.push(occurrence);
                            independent_end = source_end;
                        } else {
                            planned.push(occurrence);
                        }
                    }
                    if primary.is_empty() && (start < upper_line_end || upper_sustain.is_some() || paired_sustains.is_some() || !across_bar.is_empty()) {
                        planned.push(PlannedSeedOccurrence {
                            course: course.creation, interval: start..next,
                            source: None, pitch: None, chord: false, independent: false, opposite_stem: false,
                            source_voice: None, tie_stop: false, tie_start: false,
                        });
                    }
                    cursor = next;
                }
                if cursor < begin + measure_duration {
                    planned.push(PlannedSeedOccurrence {
                        course: course.creation, interval: cursor..(begin + measure_duration),
                        source: None, pitch: None, chord: false, independent: false, opposite_stem: false, source_voice: None, tie_stop: false, tie_start: false,
                    });
                }
                planned.extend(independent);
                planned.extend(second_independent);
            }
        }
        Ok(Self {
            courses, sources: sources.to_vec(), planned, occurrences: Vec::new(),
            metric: metric.clone(), emission_snapshot: None, source_voices: false, preserve_attacks,
        })
    }

    fn begin_emission(&self, sources: &[MidiSeedNote], measures: usize) -> anyhow::Result<()> {
        let end = self.metric.measure(measures.saturating_sub(1)).map(|bar| bar.end);
        if self.emission_snapshot.is_some() || !self.occurrences.is_empty()
            || self.sources != sources || self.metric.len() != measures
            || self.metric.divisions != 96
            || self.metric.segments().any(|bar| !matches!(bar.beats, 2..=4) || bar.beat_type != 4)
            || self.courses.iter().any(|course| course.coverage.start < 0
                || course.coverage.is_empty() || end.is_none_or(|end| course.coverage.end > end))
        {
            return Err(anyhow!("Stale or incompatible rhythmic emission plan"));
        }
        Ok(())
    }

    fn capture_emission(
        &mut self, staff: u8, voice: Option<usize>, interval: std::ops::Range<i64>,
        source: Option<usize>, bytes: std::ops::Range<usize>,
    ) -> anyhow::Result<()> {
        let expected = self.planned.get(self.occurrences.len())
            .ok_or_else(|| anyhow!("Unplanned rhythmic seed occurrence"))?;
        let course = self.courses.iter().find(|course| course.creation == expected.course
            && course.staff == staff && course.voice == voice)
            .ok_or_else(|| anyhow!("Missing planned primary presentation"))?;
        if expected.interval != interval || expected.source != source
            || self.emission_snapshot.is_some() || interval.start < course.coverage.start
            || interval.end > course.coverage.end || interval.start > interval.end
            || (interval.start == interval.end && !source.is_some_and(|ordinal|
                self.sources.iter().any(|n| n.source_ordinal == ordinal && n.len == 0)))
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
        let mut releases = BTreeMap::new();
        if identities.len() != self.courses.len()
            || self.planned.len() != self.occurrences.len()
            || xml.matches("<note>").count() != self.occurrences.len()
        {
            return Err(anyhow!("Incomplete rhythmic seed creation inventory"));
        }
        for (occurrence, planned) in self.occurrences.iter().zip(&self.planned) {
            let grace = occurrence.source.is_some_and(|ordinal| self.sources.iter()
                .any(|n| n.source_ordinal == ordinal && n.len == 0));
            let course = self.courses.iter().find(|c| c.creation == occurrence.course)
                .ok_or_else(|| anyhow!("Foreign rhythmic course"))?;
            let fragment = xml.get(occurrence.bytes.clone())
                .ok_or_else(|| anyhow!("Invalid rhythmic seed byte continuation"))?;
            if (self.source_voices || planned.tie_start || planned.tie_stop) && let Some(source) = occurrence.source {
                let original = self.sources.iter().find(|n| n.source_ordinal == source)
                    .ok_or_else(|| anyhow!("Missing original source voice interval"))?;
                let expected = releases.entry(source).or_insert(original.pos);
                if occurrence.interval.start != *expected
                    || occurrence.interval.end > original.pos + original.len
                    || planned.tie_stop != (occurrence.interval.start != original.pos)
                    || planned.tie_start != (occurrence.interval.end != original.pos + original.len)
                {
                    return Err(anyhow!("Source voice fragments do not partition their interval"));
                }
                *expected = occurrence.interval.end;
            }
            if occurrence.course != planned.course || occurrence.interval != planned.interval
                || occurrence.source != planned.source
                || fragment.contains("<chord/>") != planned.chord
                || !identities.insert(occurrence.creation) || occurrence.bytes.start < byte_cursor
                || !fragment.starts_with("      <note>\n")
                || !fragment.ends_with("      </note>\n")
                || fragment.contains("<rest/>") != occurrence.source.is_none()
                || !fragment.contains(&format!("<staff>{}</staff>", course.staff))
                || (grace && (!fragment.contains("<grace") || fragment.contains("<duration>") || planned.chord))
                || (!grace && (fragment.contains("<grace") || !fragment.contains(&format!("<duration>{}</duration>",
                    occurrence.interval.end - occurrence.interval.start))))
                || occurrence.interval.start < course.coverage.start
                || occurrence.interval.end > course.coverage.end
                || occurrence.interval.start > occurrence.interval.end
                || (grace != (occurrence.interval.start == occurrence.interval.end))
                || course.presentation != SeedCoursePresentation::Primary
                || occurrence.source.is_some_and(|source| {
                    (if planned.tie_stop { !sources.contains(&source) } else { !sources.insert(source) }) || !self.sources.iter().any(|n| {
                        n.source_ordinal == source && n.staff == course.staff
                    })
                })
            {
                return Err(anyhow!("Conflicting rhythmic seed creation continuation"));
            }
            byte_cursor = occurrence.bytes.end;
        }
        if self.sources.iter().any(|source| {
            (self.source_voices || releases.contains_key(&source.source_ordinal))
                && releases.get(&source.source_ordinal) != Some(&(source.pos + source.len))
        }) {
            return Err(anyhow!("Incomplete source voice releases"));
        }
        if sources != self.sources.iter().map(|n| n.source_ordinal).collect() {
            return Err(anyhow!("Incomplete rhythmic source membership"));
        }
        for course in &self.courses {
            let mut activity = self.occurrences.iter().zip(&self.planned)
                .filter(|(o, planned)| o.course == course.creation && o.interval.start < o.interval.end
                    && (self.source_voices || !planned.independent))
                .map(|(o, _)| o).collect::<Vec<_>>();
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
    let decoded = read_standard_midi(path)
        .with_context(|| format!("Konnte MIDI nicht als produktive Quelle lesen: {}", path.display()))?;
    let notation_time = std::sync::Arc::new(NotationTimePlan::from_midi(&decoded)?);
    let metric = &notation_time.metric;
    let preserve_attacks = notation_time.preserves_attacks();
    let allow_removal = !preserve_attacks;
    let mut events = decoded.notes.clone();
    events.sort_by_key(|event| event.source_ordinal);
    let mut notes = events
        .iter()
        .map(|event| MidiSeedNote {
            source_ordinal: event.source_ordinal,
            pos: notation_time.events[event.source_ordinal].pos,
            len: notation_time.events[event.source_ordinal].len,
            midi: event.midi,
            staff: if event.midi >= 60 { 1 } else { 2 },
        })
        .collect::<Vec<_>>();
    if notes.is_empty() {
        return Err(anyhow!("MIDI enthält keine lesbaren Notenereignisse"));
    }

    let source_voices = qualified_source_voices(&notes, &decoded, metric);
    let mut decision_trace = notes.iter().map(initial_staff_decision).collect::<Vec<_>>();
    let mut grace_notes = notes.iter().filter(|n| n.len == 0).copied().collect::<Vec<_>>();
    notes.retain(|n| n.len > 0);
    let mut removal_reasons = Vec::new();
    if let Some(assignments) = &source_voices {
        for note in &mut notes {
            let assignment = assignments[&note.source_ordinal];
            if assignment.staff != note.staff {
                decision_trace.push(staff_move_decision(note, assignment.staff, SourceSelectionDecisionContext {
                    pass: SourceSelectionDecisionPass::BoundSourceVoiceStaff, pass_iteration: 0,
                    witness_a_source_ordinal: None, witness_b_source_ordinal: None,
                    observed_primary: i64::from(assignment.staff), observed_secondary: assignment.voice as i64,
                    threshold_primary: 0, threshold_secondary: 0, focus_score: 0, tie_break_code: 0,
                }));
                note.staff = assignment.staff;
            }
        }
    } else {
        redistribute_midi_seed_hands_for_piano(metric, &mut notes, &profile, hotspots, preserve_attacks, &mut decision_trace);
        if allow_removal {
            apply_hotspot_reductions(metric, &mut notes, &profile, hotspots,
                &mut removal_reasons, &mut decision_trace);
        }
        let trajectory = apply_hand_trajectory_solver(metric, 
            &mut notes,
            &profile,
            hotspots,
            allow_removal,
            &mut removal_reasons,
            &mut decision_trace,
        );
        if trajectory.moved > 0 || trajectory.removed > 0 {
            // v0.1.45: Nach Trajectory-Reparaturen zwingend wieder Spannweiten
            // bereinigen. Sonst wird ein Sprungproblem oft nur in ein Akkord-
            // Spannweitenproblem verschoben.
            redistribute_wide_simultaneities(metric, 
                &mut notes,
                &profile,
                hotspots,
                SourceSelectionDecisionPass::PostTrajectoryWideSimultaneity,
                0,
                &mut decision_trace,
            );
        }
        let residual = apply_residual_critical_pass(metric, 
            &mut notes,
            &profile,
            hotspots,
            allow_removal,
            &mut removal_reasons,
            &mut decision_trace,
        );
        if residual.moved > 0 || residual.removed > 0 {
            // v0.1.46: auch nach dem generischen Restfehler-Pass erneut die
            // Gleichzeitigkeit entspannen. Der Pass kennt keine Takte, sondern nur
            // die automatisch erkannten Hotspots und objektive Spielbarkeitsdaten.
            redistribute_wide_simultaneities(metric, 
                &mut notes,
                &profile,
                hotspots,
                SourceSelectionDecisionPass::PostResidualWideSimultaneity,
                0,
                &mut decision_trace,
            );
        }
        // Later owners can alter the staff assignment after the initial split.
        if preserve_attacks && (trajectory.moved > 0 || residual.moved > 0) {
            redistribute_wide_simultaneities(
                metric, &mut notes, &profile, hotspots,
                SourceSelectionDecisionPass::AlignedHandSpanSplit, 0, &mut decision_trace,
            );
        }
    }
    notes.sort_by_key(|n| (n.pos, n.staff, n.midi));
    for grace in &mut grace_notes {
        let NotationEventKind::Grace { host } = notation_time.event(grace.source_ordinal)?.kind else {
            return Err(anyhow!("Empty notation interval is not a licensed grace event"));
        };
        let host = notes.iter().find(|n| n.source_ordinal == host)
            .ok_or_else(|| anyhow!("Vorschlags-Hauptnote wurde von der Quellauswahl entfernt"))?;
        if grace.staff != host.staff {
            decision_trace.push(staff_move_decision(grace, host.staff,
                SourceSelectionDecisionContext {
                    pass: SourceSelectionDecisionPass::GraceHostStaff, pass_iteration: 0,
                    witness_a_source_ordinal: Some(host.source_ordinal), witness_b_source_ordinal: None,
                    observed_primary: i64::from(grace.staff), observed_secondary: i64::from(host.staff),
                    threshold_primary: 0, threshold_secondary: 0, focus_score: 0, tie_break_code: 0,
                }));
            grace.staff = host.staff;
        }
    }
    notes.extend(grace_notes);
    notes.sort_by_key(|n| (n.pos, n.staff, n.midi));
    // Retained notes must fit the bound metric; keep exact legacy extent
    // semantics (including the point event's host location), never clip notes.
    let max_end = notes.iter().map(|n| n.pos + n.len.max(1)).max().unwrap_or(0);
    let measures = metric.len();
    anyhow::ensure!(metric.measure(measures.saturating_sub(1)).is_some_and(|bar| max_end <= bar.end),
        "Retained notation extent escapes the bound metric");
    let events_by_ordinal = events.iter().map(|e| (e.source_ordinal, *e)).collect::<BTreeMap<_, _>>();
    for decision in &mut decision_trace {
        let original = events_by_ordinal.get(&decision.source_ordinal)
            .ok_or_else(|| anyhow!("Source decision lost its original source binding"))?;
        decision.pos = original.pos; decision.len = original.len;
    }
    let mut retained_source_identities = notes
        .iter()
        .map(|note| RetainedSourceSelectionIdentity {
            source_ordinal: note.source_ordinal,
            pos: events_by_ordinal[&note.source_ordinal].pos,
            len: events_by_ordinal[&note.source_ordinal].len,
            midi: note.midi,
            final_staff: note.staff,
        })
        .collect::<Vec<_>>();
    retained_source_identities.sort_by_key(|e| (e.pos, e.final_staff, e.midi));
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
            pos: notation_time.events[event.source_ordinal].pos,
            len: notation_time.events[event.source_ordinal].len,
            midi: event.midi,
            staff: event.final_staff,
        })
        .collect();
    let rhythmic_courses = MidiSeedRhythmicCourses::create_metric_presentations(
        &notes, metric, source_voices.as_ref(), preserve_attacks,
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
        notation_time,
    };

    let mut stats = stats;
    let (xml, measure_raw_note_counts, serialized_source_identity_rows) =
        build_musicxml(title, &notes, measures, &mut stats.rhythmic_courses, decoded.opening_key_signature)?;
    crate::source_selection::bind_midi_seed_serialized_source_identity_plan(
        &mut stats.source_selection_full_causal,
        measure_raw_note_counts,
        serialized_source_identity_rows,
    )?;
    let genesis_input = crate::master_state_identity::GenesisInputBinding::ProductiveSourcePlanSha256(
        stats.notation_time.bind_productive_plan(
            &stats.source_selection_full_causal.productive_plan_seed_fingerprint_sha256),
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

fn apply_residual_critical_pass(metric: &MetricTimeline, 
    notes: &mut Vec<MidiSeedNote>,
    profile: &CandidateProfile,
    hotspots: Option<&HotspotPlan>,
    allow_removal: bool,
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
        let moved = residual_move_register_outliers(metric, 
            notes,
            profile,
            plan,
            pass_iteration,
            decision_trace,
        );
        let removed = if allow_removal {
            let spans = residual_remove_critical_spans(metric, notes, profile, plan,
                removal_reasons, pass_iteration, decision_trace);
            let jumps = residual_remove_critical_jumps(metric, notes, profile, plan,
                removal_reasons, pass_iteration, decision_trace);
            spans + jumps
        } else {
            0
        };
        stats.moved += moved;
        stats.removed += removed;
        if moved == 0 && removed == 0 {
            break;
        }
    }
    stats
}

fn residual_move_register_outliers(metric: &MetricTimeline, 
    notes: &mut [MidiSeedNote],
    profile: &CandidateProfile,
    plan: &HotspotPlan,
    pass_iteration: usize,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) -> usize {
    let mut moves: Vec<(usize, u8, SourceSelectionDecisionEvidence)> = Vec::new();
    for focus in &plan.entries {
        let local_profile = profile.focused(focus.score);
        let mut local = hotspot_note_indices(metric, notes, focus);
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

fn residual_remove_critical_spans(metric: &MetricTimeline, 
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
        for idx in hotspot_note_indices(metric, notes, focus) {
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

fn residual_remove_critical_jumps(metric: &MetricTimeline, 
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
            let mut reps = representative_indices_for_focus(metric, notes, focus, staff);
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

fn apply_hand_trajectory_solver(metric: &MetricTimeline, 
    notes: &mut Vec<MidiSeedNote>,
    profile: &CandidateProfile,
    hotspots: Option<&HotspotPlan>,
    allow_removal: bool,
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

        let moved = trajectory_move_outliers(metric, 
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

        if allow_removal {
            let removed = trajectory_remove_zigzags(metric, notes, profile, plan,
                removal_reasons, pass_iteration, decision_trace);
            if removed > 0 {
                stats.removed += removed;
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    stats
}

fn trajectory_move_outliers(metric: &MetricTimeline, 
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
            let reps = representative_indices_for_focus(metric, notes, focus, staff);
            if reps.len() < 2 { continue; }

            for pair in reps.windows(2) {
                let a = pair[0];
                let b = pair[1];
                let dt = (notes[b].pos - notes[a].pos).abs();
                let local_focus_score = plan.focus_score_for(note_measure(metric, notes[a].pos), Some(staff));
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

fn trajectory_remove_zigzags(metric: &MetricTimeline, 
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
            let reps = representative_indices_for_focus(metric, notes, focus, staff);
            if reps.len() < 3 { continue; }
            for triple in reps.windows(3) {
                let a = triple[0];
                let b = triple[1];
                let c = triple[2];
                let local_focus_score = plan.focus_score_for(note_measure(metric, notes[b].pos), Some(staff));
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

fn representative_indices_for_focus(metric: &MetricTimeline, notes: &[MidiSeedNote], focus: &HotspotFocus, staff: u8) -> Vec<usize> {
    let Some(bar) = metric.measure(focus.measure.saturating_sub(1)) else { return Vec::new(); };
    let measure_start = bar.start;
    let measure_end = bar.end;
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

fn apply_hotspot_reductions(metric: &MetricTimeline, 
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
        reduce_hotspot_spans(HotspotMetricContext { metric, focus },
            notes,
            profile,
            per_hotspot_cap,
            &mut remove,
            &mut removal_reason_by_index,
            &mut removal_decision_by_index,
        );
        reduce_hotspot_jumps(HotspotMetricContext { metric, focus },
            notes,
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

fn hotspot_note_indices(metric: &MetricTimeline, notes: &[MidiSeedNote], focus: &HotspotFocus) -> Vec<usize> {
    let Some(bar) = metric.measure(focus.measure.saturating_sub(1)) else { return Vec::new(); };
    let measure_start = bar.start;
    let measure_end = bar.end;
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

struct HotspotMetricContext<'a> {
    metric: &'a MetricTimeline,
    focus: &'a HotspotFocus,
}

fn reduce_hotspot_spans(context: HotspotMetricContext<'_>,
    notes: &[MidiSeedNote],
    profile: &CandidateProfile,
    cap: usize,
    remove: &mut BTreeSet<usize>,
    removal_reason_by_index: &mut BTreeMap<usize, ExistingSourceSelectionRemovalReason>,
    removal_decision_by_index: &mut BTreeMap<usize, SourceSelectionDecisionEvidence>,
) {
    let HotspotMetricContext { metric, focus } = context;
    let mut by_start_staff: BTreeMap<(i64, u8), Vec<usize>> = BTreeMap::new();
    for idx in hotspot_note_indices(metric, notes, focus) {
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

fn reduce_hotspot_jumps(context: HotspotMetricContext<'_>,
    notes: &[MidiSeedNote],
    profile: &CandidateProfile,
    cap: usize,
    remove: &mut BTreeSet<usize>,
    removal_reason_by_index: &mut BTreeMap<usize, ExistingSourceSelectionRemovalReason>,
    removal_decision_by_index: &mut BTreeMap<usize, SourceSelectionDecisionEvidence>,
) {
    let HotspotMetricContext { metric, focus } = context;
    let mut added = 0usize;
    for staff in [1_u8, 2_u8] {
        let mut indices = hotspot_note_indices(metric, notes, focus)
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

fn redistribute_midi_seed_hands_for_piano(metric: &MetricTimeline, 
    notes: &mut [MidiSeedNote],
    profile: &CandidateProfile,
    hotspots: Option<&HotspotPlan>,
    preserve_attacks: bool,
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
        let measure = note_measure(metric, notes[*indices.first().expect("non-empty onset group")].pos);
        let focus_score = hotspots.map_or(0, |h| h.focus_score_for(measure, None));
        let local_profile = profile.focused(focus_score);
        let preferred = wide_piano_hand_split(&group, span, &local_profile);
        let aligned = preserve_attacks.then(|| aligned_hand_span_split(&group, |order, index| {
            preferred.map_or(notes[index].staff, |split| staff_for_hand_split(order, split))
        })).flatten();
        let Some(split) = aligned.map(|(split, _)| split).or(preferred) else {
            continue;
        };
        let (pass, observed_span, span_bound) = aligned.map_or(
            (SourceSelectionDecisionPass::PianoHandSplit, i64::from(span), i64::from(local_profile.span_limit())),
            |(_, current_span)| (SourceSelectionDecisionPass::AlignedHandSpanSplit,
                i64::from(current_span), i64::from(crate::rules::PIANO_HAND_MAX_SPAN)),
        );
        let split_code = u8::try_from(split).unwrap_or(u8::MAX);
        for (order, (idx, _midi)) in group.iter().enumerate() {
            let new_staff = staff_for_hand_split(order, split);
            if notes[*idx].staff != new_staff {
                let evidence = staff_move_decision(
                    &notes[*idx],
                    new_staff,
                    SourceSelectionDecisionContext {
                        pass,
                        pass_iteration: 0,
                        witness_a_source_ordinal: Some(notes[low_idx].source_ordinal),
                        witness_b_source_ordinal: Some(notes[high_idx].source_ordinal),
                        observed_primary: observed_span,
                        observed_secondary: i64::try_from(split).unwrap_or(i64::MAX),
                        threshold_primary: span_bound,
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
        smooth_fast_staff_jumps(metric, notes, profile, hotspots, preserve_attacks, decision_trace);
    }
}

fn redistribute_wide_simultaneities(metric: &MetricTimeline, 
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
        let measure = note_measure(metric, notes[*indices.first().expect("non-empty onset group")].pos);
        let focus_score = hotspots.map_or(0, |h| h.focus_score_for(measure, None));
        let local_profile = profile.focused(focus_score);
        let (split, span, threshold) = if pass == SourceSelectionDecisionPass::AlignedHandSpanSplit {
            let Some((split, current_span)) = aligned_hand_span_split(&group, |_, index| notes[index].staff) else {
                continue;
            };
            (split, i64::from(current_span), i64::from(crate::rules::PIANO_HAND_MAX_SPAN))
        } else {
            let Some(split) = wide_piano_hand_split(&group, span, &local_profile) else {
                continue;
            };
            (split, i64::from(span), i64::from(local_profile.span_limit()))
        };
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
                        observed_primary: span,
                        observed_secondary: i64::try_from(split).unwrap_or(i64::MAX),
                        threshold_primary: threshold,
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

// The alignment commits pitches and times. Only a split that fits both
// hands can override the current assignment, with the fewest staff changes.
fn staff_for_hand_split(order: usize, split: usize) -> u8 {
    if order < split { 2 } else { 1 }
}

fn aligned_hand_span_split(
    group: &[(usize, i32)],
    staff_at: impl Fn(usize, usize) -> u8,
) -> Option<(usize, u32)> {
    use crate::rules::{PIANO_HAND_MAX_KEYS, PIANO_HAND_MAX_SPAN};

    let current_span = [1, 2].into_iter().map(|staff| {
        let mut pitches = group.iter().enumerate().filter_map(|(order, &(index, pitch))| {
            (staff_at(order, index) == staff).then_some(pitch)
        });
        pitches.next().map_or(0, |low| pitches.next_back().unwrap_or(low).abs_diff(low))
    }).max()?;
    if current_span <= PIANO_HAND_MAX_SPAN {
        return None;
    }
    let span = |hand: &[(usize, i32)]| hand.last().unwrap().1.abs_diff(hand[0].1);
    (1..group.len()).filter_map(|split| {
        let (left, right) = group.split_at(split);
        let widest = span(left).max(span(right));
        if widest > PIANO_HAND_MAX_SPAN || left.len().max(right.len()) > PIANO_HAND_MAX_KEYS {
            return None;
        }
        let staff_changes = group.iter().enumerate().filter(|&(order, &(index, _))| {
            staff_at(order, index) != staff_for_hand_split(order, split)
        }).count();
        Some((staff_changes, widest, split))
    }).min().map(|(_, _, split)| (split, current_span))
}

fn smooth_fast_staff_jumps(metric: &MetricTimeline, 
    notes: &mut [MidiSeedNote],
    profile: &CandidateProfile,
    hotspots: Option<&HotspotPlan>,
    preserve_attacks: bool,
    decision_trace: &mut Vec<SourceSelectionDecisionEvidence>,
) {
    let max_passes = if hotspots.is_none_or(|h| h.is_empty()) { 3 } else { 24 };
    let mut ordered: Vec<_> = (0..notes.len()).collect();
    ordered.sort_by_key(|&idx| (notes[idx].pos, notes[idx].midi));
    let jump_context = HandJumpContext { metric, profile, hotspots, preserve_attacks };
    for pass_iteration in 0..max_passes {
        let mut changed = false;
        for staff in [1_u8, 2_u8] {
            let mut previous = None;
            for (b_slot, &b) in ordered.iter().enumerate() {
                if notes[b].staff != staff {
                    continue;
                }
                let Some((a_slot, a)) = previous.replace((b_slot, b)) else {
                    continue;
                };
                let (jump, dt, threshold, dt_limit, focus_score) =
                    jump_context.edge(notes[a], notes[b], staff);
                if dt == 0 || dt > dt_limit || jump <= threshold {
                    continue;
                }
                let local_profile = profile.focused(focus_score);
                let Some(target_idx) =
                    choose_jump_reassignment_target(notes, a, b, staff, &local_profile)
                else {
                    continue;
                };
                let new_staff = if staff == 1 { 2 } else { 1 };
                let slot = if target_idx == a { a_slot } else { b_slot };
                if !jump_context.handoff_improves(notes, &ordered, slot, new_staff) {
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
                        threshold_primary: i64::from(threshold),
                        threshold_secondary: dt_limit,
                        focus_score,
                        tie_break_code,
                    },
                );
                decision_trace.push(evidence);
                notes[target_idx].staff = new_staff;
                if target_idx == b {
                    previous = Some((a_slot, a));
                }
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

struct HandJumpContext<'a> {
    metric: &'a MetricTimeline,
    profile: &'a CandidateProfile,
    hotspots: Option<&'a HotspotPlan>,
    preserve_attacks: bool,
}

impl HandJumpContext<'_> {
    fn edge(&self, a: MidiSeedNote, b: MidiSeedNote, staff: u8) -> (i32, i64, i32, i64, i64) {
        let measure = note_measure(self.metric, a.pos);
        let focus = self.hotspots.map_or(0, |plan| plan.focus_score_for(measure, Some(staff)));
        let profile = self.profile;
        let local_profile = profile.focused(focus);
        let dt_limit = if local_profile.jump_limit() < profile.jump_limit() { 144 } else { 96 };
        ((b.midi - a.midi).abs(), b.pos - a.pos, local_profile.jump_limit(), dt_limit, focus)
    }

    fn handoff_improves(
        &self,
        notes: &[MidiSeedNote],
        ordered: &[usize],
        slot: usize,
        receiving_staff: u8,
    ) -> bool {
        let moving = notes[ordered[slot]];
        let edge_cost = |a, b, staff| match (a, b) {
            (Some(a), Some(b)) => {
                let (jump, dt, threshold, window, _) = self.edge(a, b, staff);
                if dt > 0 && dt <= window { (jump - threshold).max(0) } else { 0 }
            }
            _ => 0,
        };
        let neighbors = |staff| {
            let find = |idx: &usize| {
                let note = notes[*idx];
                (note.staff == staff).then_some(note)
            };
            (
                ordered[..slot].iter().rev().find_map(find),
                ordered[slot + 1..].iter().find_map(find),
            )
        };
        let (from_prev, from_next) = neighbors(moving.staff);
        let (to_prev, to_next) = neighbors(receiving_staff);
        let before = edge_cost(from_prev, Some(moving), moving.staff)
            + edge_cost(Some(moving), from_next, moving.staff)
            + edge_cost(to_prev, to_next, receiving_staff);
        let after = edge_cost(from_prev, from_next, moving.staff)
            + edge_cost(to_prev, Some(moving), receiving_staff)
            + edge_cost(Some(moving), to_next, receiving_staff);
        after < before
            && receiving_hand_fits(notes, moving, receiving_staff)
            && self.onset_split_allows_handoff(notes, ordered, slot, receiving_staff)
    }

    fn onset_split_allows_handoff(
        &self,
        notes: &[MidiSeedNote],
        ordered: &[usize],
        slot: usize,
        receiving_staff: u8,
    ) -> bool {
        let pos = notes[ordered[slot]].pos;
        let first = ordered.partition_point(|&index| notes[index].pos < pos);
        let end = ordered.partition_point(|&index| notes[index].pos <= pos);
        let group: Vec<_> = ordered[first..end].iter().map(|&index| (index, notes[index].midi)).collect();
        let span = group[group.len() - 1].1 - group[0].1;
        let measure = note_measure(self.metric, pos);
        let focus = self.hotspots.map_or(0, |plan| plan.focus_score_for(measure, None));
        let local_profile = self.profile.focused(focus);
        wide_piano_hand_split(&group, span, &local_profile).is_none_or(|split| {
            // Attack-preserving seeds already have the feasible split at this onset.
            // Keep that assignment, including where it differs from the soft split.
            !self.preserve_attacks && receiving_staff == staff_for_hand_split(slot - first, split)
        })
    }
}

fn receiving_hand_fits(notes: &[MidiSeedNote], moving: MidiSeedNote, staff: u8) -> bool {
    use crate::rules::{PIANO_HAND_MAX_KEYS, PIANO_HAND_MAX_SPAN};

    let end = moving.pos.saturating_add(moving.len);
    let overlapping: Vec<_> = notes.iter().filter(|note| {
        note.staff == staff && note.pos < end && note.pos.saturating_add(note.len) > moving.pos
    }).collect();
    // Occupancy can increase only at the transferred attack or a later attack.
    std::iter::once(moving.pos)
        .chain(overlapping.iter().filter_map(|note| (note.pos > moving.pos).then_some(note.pos)))
        .all(|pos| {
            let mut keys = BTreeSet::from([moving.midi]);
            for note in &overlapping {
                if note.pos <= pos && note.pos.saturating_add(note.len) > pos {
                    keys.insert(note.midi);
                }
            }
            keys.len() <= PIANO_HAND_MAX_KEYS
                && keys.last().unwrap().abs_diff(*keys.first().unwrap()) <= PIANO_HAND_MAX_SPAN
        })
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

fn wide_piano_hand_split(group: &[(usize, i32)], span: i32, profile: &CandidateProfile) -> Option<usize> {
    (span > profile.span_limit()).then(|| choose_piano_hand_split(group, profile))
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

fn note_measure(metric: &MetricTimeline, pos: i64) -> usize {
    metric.ordinal_at(pos).map_or(0, |ordinal| ordinal + 1)
}

fn build_musicxml(
    title: &str,
    notes: &[MidiSeedNote],
    measures: usize,
    rhythmic_courses: &mut MidiSeedRhythmicCourses,
    opening_key_signature: Option<MidiKeySignature>,
) -> anyhow::Result<(String, Vec<usize>, Vec<MidiSeedSerializedSourceIdentityRow>)> {
    rhythmic_courses.begin_emission(notes, measures)?;
    let mut by_measure_staff: Vec<[Vec<MidiSeedNote>; 2]> = (0..measures)
        .map(|_| [Vec::new(), Vec::new()])
        .collect();

    for note in notes {
        let measure_idx = rhythmic_courses.metric.ordinal_at(note.pos)
            .ok_or_else(|| anyhow!("Seed note outside notation metric"))?;
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
        let bar = rhythmic_courses.metric.measure(idx)
            .ok_or_else(|| anyhow!("Missing notation meter"))?;
        let partial = bar.end - bar.start < DIVISIONS * i64::from(bar.beats) * 4 / i64::from(bar.beat_type);
        let implicit = if partial { " implicit=\"yes\"" } else { "" };
        xml.push_str(&format!("    <measure number=\"{measure_no}\"{implicit}>\n"));
        if idx == 0 {
            xml.push_str("      <attributes>\n");
            xml.push_str(&format!("        <divisions>{DIVISIONS}</divisions>\n"));
            if let Some(key) = opening_key_signature {
                let mode = if key.minor { "minor" } else { "major" };
                xml.push_str(&format!("        <key><fifths>{}</fifths><mode>{mode}</mode></key>\n", key.fifths));
            } else {
                xml.push_str("        <key><fifths>0</fifths></key>\n");
            }
            let bar = rhythmic_courses.metric.measure(idx)
                .ok_or_else(|| anyhow!("Missing initial meter"))?;
            let time_symbol = match (bar.beats, bar.beat_type) {
                (4, 4) => " symbol=\"common\"",
                _ => "",
            };
            xml.push_str(&format!(
                "        <time{time_symbol}><beats>{}</beats><beat-type>{}</beat-type></time>\n",
                bar.beats, bar.beat_type,
            ));
            xml.push_str("        <staves>2</staves>\n");
            xml.push_str("        <clef number=\"1\"><sign>G</sign><line>2</line></clef>\n");
            xml.push_str("        <clef number=\"2\"><sign>F</sign><line>4</line></clef>\n");
            xml.push_str("      </attributes>\n");
        } else {
            let current = rhythmic_courses.metric.measure(idx).ok_or_else(|| anyhow!("Missing meter"))?;
            let prior = rhythmic_courses.metric.measure(idx - 1).ok_or_else(|| anyhow!("Missing prior meter"))?;
            if (current.beats, current.beat_type) != (prior.beats, prior.beat_type) {
                xml.push_str(&format!("      <attributes><time><beats>{}</beats><beat-type>{}</beat-type></time></attributes>\n", current.beats, current.beat_type));
            }
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
        xml.push_str(&format!("      <backup><duration>{}</duration></backup>\n", bar.end - bar.start));
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
    let bar = rhythmic_courses.metric.measure(usize::try_from(measure_idx)?)
        .ok_or_else(|| anyhow!("Missing serialized notation meter"))?;
    let measure_start = bar.start;
    let courses = rhythmic_courses.courses.iter().filter(|course| course.staff == staff)
        .map(|course| course.creation).collect::<BTreeSet<_>>();
    let planned = rhythmic_courses.planned.iter().filter(|occurrence| {
        courses.contains(&occurrence.course) && occurrence.interval.start >= measure_start
            && occurrence.interval.start < bar.end
    }).cloned().collect::<Vec<_>>();
    let expected_sources = planned.iter().filter(|note| !note.tie_stop).filter_map(|occurrence| occurrence.source)
        .collect::<BTreeSet<_>>();
    if expected_sources != notes.iter().map(|note| note.source_ordinal).collect() {
        return Err(anyhow!("Writer input differs from planned rhythmic source membership"));
    }
    let mut independent_cursor = None;
    let mut source_cursor = bar.start;
    for occurrence in planned {
        if rhythmic_courses.source_voices && !occurrence.chord {
            append_cursor_motion(xml, source_cursor, occurrence.interval.start);
            source_cursor = occurrence.interval.end;
        } else if occurrence.independent && !occurrence.chord && !rhythmic_courses.source_voices {
            let cursor = *independent_cursor.get_or_insert_with(|| {
                let measure_duration = bar.end - bar.start;
                xml.push_str(&format!("      <backup><duration>{measure_duration}</duration></backup>\n"));
                bar.start
            });
            append_cursor_motion(xml, cursor, occurrence.interval.start);
            independent_cursor = Some(occurrence.interval.end);
        }
        let duration = occurrence.interval.end - occurrence.interval.start;
        let source_voice = occurrence.source_voice.map(|voice| voice.to_string());
        let voice = source_voice.as_deref().unwrap_or(voice);
        let byte_start = xml.len();
        match (occurrence.source, occurrence.pitch) {
            (Some(source_ordinal), Some(midi)) => {
                if !occurrence.tie_stop {
                    serialized_source_identity_rows.push(MidiSeedSerializedSourceIdentityRow {
                        measure_ordinal, raw_note_index: *raw_note_index, source_ordinal,
                    });
                }
                let written_stem = if occurrence.opposite_stem {
                    if stem == "up" { "down" } else { "up" }
                } else {
                    stem
                };
                append_note(xml, midi, duration, staff, voice, written_stem, &occurrence);
            }
            (None, None) => append_rest(xml, duration, staff, voice),
            _ => return Err(anyhow!("Inconsistent planned rhythmic occurrence")),
        }
        rhythmic_courses.capture_emission(staff, occurrence.source_voice, occurrence.interval,
            occurrence.source, byte_start..xml.len())?;
        *raw_note_index = (*raw_note_index).saturating_add(1);
    }
    if rhythmic_courses.source_voices {
        append_cursor_motion(xml, source_cursor, bar.end);
    }
    if let Some(cursor) = independent_cursor
        && cursor < bar.end
    {
        let distance = bar.end - cursor;
        xml.push_str(&format!("      <forward><duration>{distance}</duration></forward>\n"));
    }
    Ok(())
}

fn append_cursor_motion(xml: &mut String, cursor: i64, target: i64) {
    let (tag, distance) = if target > cursor { ("forward", target - cursor) }
        else if cursor > target { ("backup", cursor - target) }
        else { return; };
    xml.push_str(&format!("      <{tag}><duration>{distance}</duration></{tag}>\n"));
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

fn append_note(xml: &mut String, midi: i32, duration: i64, staff: u8, voice: &str, stem: &str, occurrence: &PlannedSeedOccurrence) {
    let (step, alter, octave) = midi_to_pitch(midi);
    xml.push_str("      <note>\n");
    if duration == 0 { xml.push_str("        <grace slash=\"yes\"/>\n"); }
    if occurrence.chord {
        xml.push_str("        <chord/>\n");
    }
    xml.push_str("        <pitch>\n");
    xml.push_str(&format!("          <step>{step}</step>\n"));
    if alter != 0 {
        xml.push_str(&format!("          <alter>{alter}</alter>\n"));
    }
    xml.push_str(&format!("          <octave>{octave}</octave>\n"));
    xml.push_str("        </pitch>\n");
    if duration > 0 { xml.push_str(&format!("        <duration>{duration}</duration>\n")); }
    if occurrence.tie_stop { xml.push_str("        <tie type=\"stop\"/>\n"); }
    if occurrence.tie_start { xml.push_str("        <tie type=\"start\"/>\n"); }
    xml.push_str(&format!("        <voice>{voice}</voice>\n"));
    if duration == 0 { xml.push_str("        <type>eighth</type>\n"); } else { append_type(xml, duration); }
    xml.push_str(&format!("        <stem>{stem}</stem>\n"));
    xml.push_str(&format!("        <staff>{staff}</staff>\n"));
    if occurrence.tie_stop || occurrence.tie_start {
        xml.push_str("        <notations>\n");
        if occurrence.tie_stop { xml.push_str("          <tied type=\"stop\"/>\n"); }
        if occurrence.tie_start { xml.push_str("          <tied type=\"start\"/>\n"); }
        xml.push_str("        </notations>\n");
    }
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
