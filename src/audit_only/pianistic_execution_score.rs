//! r292 read-only foundation for a later constructive piano-execution proof.
//!
//! This module does not decide hand, finger, difficulty, feasibility or repair. It projects the
//! already generated master MusicXML into an exact 96-ticks-per-quarter execution timeline and
//! collapses sound-level MusicXML tie chains into attack/hold/release events. Source staff and
//! voice remain provenance affinities only; neither is interpreted as a physical hand.

use crate::sha256;
use roxmltree::{Document, Node};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

pub const SECTION: &str = "PIANISTIC-EXECUTION-SCORE-AUTHORITY-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticExecutionScoreAuthority";
pub const SCHEMA: &str = "pianistic-execution-score-v1";
pub const CANONICAL_TICKS_PER_QUARTER: i64 = 96;
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionStatus {
    Complete,
    ModelInputUnavailable,
}

impl ProjectionStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "COMPLETE",
            Self::ModelInputUnavailable => "MODEL_INPUT_UNAVAILABLE",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TempoPoint {
    pub tick: i64,
    pub milli_bpm: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PianisticExecutionEvent {
    pub part_id: String,
    pub source_staff: String,
    pub source_voice: String,
    pub midi: i32,
    pub attack_tick: i64,
    pub release_tick: i64,
    pub attack_time_us_floor: u64,
    pub release_time_us_floor: u64,
    pub note_segments: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PianisticExecutionScore {
    pub status: ProjectionStatus,
    pub reason: String,
    pub source_path: PathBuf,
    pub pitched_note_segments: usize,
    pub execution_events: Vec<PianisticExecutionEvent>,
    pub tie_continuation_segments: usize,
    pub tempo_points: Vec<TempoPoint>,
    pub terminal_tick: i64,
    pub terminal_physical_time_us_floor: Option<u64>,
    pub musical_execution_fingerprint_sha256: String,
    pub source_affinity_fingerprint_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PianisticExecutionScoreAudit {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NoteSegment {
    source_ordinal: usize,
    part_id: String,
    source_staff: String,
    source_voice: String,
    midi: i32,
    start_tick: i64,
    end_tick: i64,
    tie_start: bool,
    tie_stop: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TieKey {
    part_id: String,
    source_staff: String,
    source_voice: String,
    midi: i32,
}

#[derive(Debug, Clone)]
struct ParsedTimeline {
    segments: Vec<NoteSegment>,
    tempo_points: Vec<TempoPoint>,
    terminal_tick: i64,
}

pub fn build_for_score_title(score_title: Option<&str>) -> PianisticExecutionScoreAudit {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("===============================================\n");
    let _ = writeln!(out, "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no.");
    out.push_str("AuthorityBoundary: source staff and source voice are provenance affinities only. StaffIsHand false; VoiceIsHand false; FingerAssignment none; PianistProfile none; CapabilityModel none.\n");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        out.push_str("ProjectionStatus: MODEL_INPUT_UNAVAILABLE; Reason score-title-missing; PlayabilityClaim NOT_EVALUATED.\n\n");
        return PianisticExecutionScoreAudit { text: out };
    };
    let path = PathBuf::from(format!("{title}.musicxml"));
    let score = project_file(&path);
    append_score_audit(&mut out, &score);
    PianisticExecutionScoreAudit { text: out }
}

pub fn project_file(path: &Path) -> PianisticExecutionScore {
    let xml = match fs::read_to_string(path) {
        Ok(xml) => xml,
        Err(error) => {
            return unavailable(path, format!("master-musicxml-unreadable:{error}"));
        }
    };
    project_xml(path.to_path_buf(), &xml)
}

pub fn project_xml(source_path: PathBuf, xml: &str) -> PianisticExecutionScore {
    let parsed = match parse_timeline(xml) {
        Ok(parsed) => parsed,
        Err(reason) => return unavailable(&source_path, reason),
    };
    let pitched_note_segments = parsed.segments.len();
    let tie_continuation_segments = parsed.segments.iter().filter(|segment| segment.tie_stop).count();
    let mut execution_events = match collapse_ties(&parsed.segments) {
        Ok(events) => events,
        Err(reason) => return unavailable_with_timeline(source_path, parsed, reason),
    };
    for event in &mut execution_events {
        let Some(attack_time) = physical_time_us_floor(event.attack_tick, &parsed.tempo_points) else {
            return unavailable_with_timeline(
                source_path,
                parsed,
                "tempo-map-does-not-cover-event-attack".to_string(),
            );
        };
        let Some(release_time) = physical_time_us_floor(event.release_tick, &parsed.tempo_points) else {
            return unavailable_with_timeline(
                source_path,
                parsed,
                "tempo-map-does-not-cover-event-release".to_string(),
            );
        };
        event.attack_time_us_floor = attack_time;
        event.release_time_us_floor = release_time;
    }
    let terminal_physical_time_us_floor = physical_time_us_floor(
        parsed.terminal_tick,
        &parsed.tempo_points,
    );
    if terminal_physical_time_us_floor.is_none() {
        return unavailable_with_timeline(
            source_path,
            parsed,
            "tempo-map-does-not-cover-score-origin".to_string(),
        );
    }
    let musical_execution_fingerprint_sha256 = musical_fingerprint(&execution_events, &parsed.tempo_points);
    let source_affinity_fingerprint_sha256 = source_affinity_fingerprint(&execution_events);
    PianisticExecutionScore {
        status: ProjectionStatus::Complete,
        reason: "complete-exact-attack-hold-release-projection".to_string(),
        source_path,
        pitched_note_segments,
        execution_events,
        tie_continuation_segments,
        tempo_points: parsed.tempo_points,
        terminal_tick: parsed.terminal_tick,
        terminal_physical_time_us_floor,
        musical_execution_fingerprint_sha256,
        source_affinity_fingerprint_sha256,
    }
}

fn unavailable(path: &Path, reason: String) -> PianisticExecutionScore {
    PianisticExecutionScore {
        status: ProjectionStatus::ModelInputUnavailable,
        reason,
        source_path: path.to_path_buf(),
        pitched_note_segments: 0,
        execution_events: Vec::new(),
        tie_continuation_segments: 0,
        tempo_points: Vec::new(),
        terminal_tick: 0,
        terminal_physical_time_us_floor: None,
        musical_execution_fingerprint_sha256: "unavailable".to_string(),
        source_affinity_fingerprint_sha256: "unavailable".to_string(),
    }
}

fn unavailable_with_timeline(
    source_path: PathBuf,
    parsed: ParsedTimeline,
    reason: String,
) -> PianisticExecutionScore {
    PianisticExecutionScore {
        status: ProjectionStatus::ModelInputUnavailable,
        reason,
        source_path,
        pitched_note_segments: parsed.segments.len(),
        execution_events: Vec::new(),
        tie_continuation_segments: parsed.segments.iter().filter(|segment| segment.tie_stop).count(),
        tempo_points: parsed.tempo_points,
        terminal_tick: parsed.terminal_tick,
        terminal_physical_time_us_floor: None,
        musical_execution_fingerprint_sha256: "unavailable".to_string(),
        source_affinity_fingerprint_sha256: "unavailable".to_string(),
    }
}

fn append_score_audit(out: &mut String, score: &PianisticExecutionScore) {
    let _ = writeln!(
        out,
        "ProjectionStatus: {}; Reason {}; SourcePath {}; CanonicalTicksPerQuarter {}; PlayabilityClaim {PLAYABILITY_CLAIM}.",
        score.status.as_str(),
        score.reason,
        score.source_path.display(),
        CANONICAL_TICKS_PER_QUARTER,
    );
    let _ = writeln!(
        out,
        "ExecutionProjection: PitchedNoteSegments {}; AttackEvents {}; HoldObligations {}; ReleaseObligations {}; TieContinuationSegments {}; TerminalTick {}.",
        score.pitched_note_segments,
        score.execution_events.len(),
        score.execution_events.len(),
        score.execution_events.len(),
        score.tie_continuation_segments,
        score.terminal_tick,
    );
    let time_endpoints_complete = score.execution_events.iter().all(|event| {
        event.release_time_us_floor >= event.attack_time_us_floor
    });
    let _ = writeln!(
        out,
        "TempoMap: Points {}; OriginCovered {}; EventTimeEndpointsComplete {}; TerminalPhysicalTimeUsFloor {}; TimeProjection deterministic_floor_microseconds_from_exact_96tpq_and_milli_bpm.",
        score.tempo_points.len(),
        yes_no(score.tempo_points.first().is_some_and(|point| point.tick == 0)),
        yes_no(time_endpoints_complete),
        score
            .terminal_physical_time_us_floor
            .map_or_else(|| "unavailable".to_string(), |value| value.to_string()),
    );
    let _ = writeln!(
        out,
        "Fingerprints: MusicalExecutionSha256 {}; SourceAffinitySha256 {}.",
        score.musical_execution_fingerprint_sha256,
        score.source_affinity_fingerprint_sha256,
    );
    out.push_str("Semantics: each projected execution event has exactly one attack, a continuous hold interval [attack, release), and exactly one release. Sound-level MusicXML tie continuations extend hold/release and never create a second attack.\n");
    out.push_str("NonClaims: no hand assignment, no finger assignment, no reach model, no transition capability, no difficulty score, no feasibility claim, no repair candidate, no writer path.\n\n");
}

fn parse_timeline(xml: &str) -> Result<ParsedTimeline, String> {
    let document = Document::parse(xml).map_err(|error| format!("musicxml-not-well-formed:{error}"))?;
    let root = document.root_element();
    if root.tag_name().name() != "score-partwise" {
        return Err(format!("unsupported-root:{}", root.tag_name().name()));
    }

    let mut segments = Vec::new();
    let mut tempos = Vec::new();
    let mut source_ordinal = 0usize;
    let mut score_terminal_tick = 0i64;
    let parts = root
        .children()
        .filter(|node| is_element(*node, "part"))
        .collect::<Vec<_>>();
    if parts.len() != 1 {
        return Err(format!("piano-part-count-must-be-one:found-{}", parts.len()));
    }

    let part = parts[0];
    let part_id = part.attribute("id").unwrap_or("P1").to_string();
    let mut measure_start_tick = 0i64;
    let mut divisions = 1i64;

    for measure in part.children().filter(|node| is_element(*node, "measure")) {
        let mut cursor_tick = 0i64;
        let mut measure_max_tick = 0i64;
        let mut previous_note_start_tick = None;

        for child in measure.children().filter(|node| node.is_element()) {
            match child.tag_name().name() {
                "attributes" => {
                    if let Some(value) = direct_child_text(child, "divisions") {
                        let parsed = parse_positive_i64(&value, "divisions")?;
                        divisions = parsed;
                    }
                }
                "direction" => {
                    let offset = direct_child_text(child, "offset")
                        .map(|text| scale_divisions(parse_i64(&text, "direction-offset")?, divisions))
                        .transpose()?
                        .unwrap_or(0);
                    let tick = measure_start_tick
                        .checked_add(cursor_tick)
                        .and_then(|value| value.checked_add(offset))
                        .ok_or_else(|| "timeline-overflow-at-tempo".to_string())?;
                    for sound in child.descendants().filter(|node| is_element(*node, "sound")) {
                        if let Some(raw) = sound.attribute("tempo") {
                            tempos.push(TempoPoint {
                                tick,
                                milli_bpm: parse_milli_bpm(raw)?,
                            });
                        }
                    }
                }
                "backup" => {
                    let duration = required_duration(child, "backup")?;
                    let scaled = scale_divisions(duration, divisions)?;
                    cursor_tick = cursor_tick
                        .checked_sub(scaled)
                        .ok_or_else(|| "timeline-underflow-at-backup".to_string())?;
                    if cursor_tick < 0 {
                        return Err("negative-measure-cursor-after-backup".to_string());
                    }
                    previous_note_start_tick = None;
                }
                "forward" => {
                    let duration = required_duration(child, "forward")?;
                    let scaled = scale_divisions(duration, divisions)?;
                    cursor_tick = cursor_tick
                        .checked_add(scaled)
                        .ok_or_else(|| "timeline-overflow-at-forward".to_string())?;
                    measure_max_tick = measure_max_tick.max(cursor_tick);
                    previous_note_start_tick = None;
                }
                "note" => {
                    let is_chord = direct_child(child, "chord").is_some();
                    let is_grace = direct_child(child, "grace").is_some();
                    let duration_raw = direct_child_text(child, "duration");
                    if is_grace && direct_child(child, "pitch").is_some() {
                        return Err("pitched-grace-note-unmodeled".to_string());
                    }
                    let duration = match duration_raw {
                        Some(text) => scale_divisions(parse_nonnegative_i64(&text, "note-duration")?, divisions)?,
                        None if is_grace => 0,
                        None => return Err("note-duration-missing".to_string()),
                    };
                    let local_start = if is_chord {
                        previous_note_start_tick.ok_or_else(|| "chord-without-predecessor".to_string())?
                    } else {
                        cursor_tick
                    };
                    let local_end = local_start
                        .checked_add(duration)
                        .ok_or_else(|| "timeline-overflow-at-local-note-end".to_string())?;
                    let global_start = measure_start_tick
                        .checked_add(local_start)
                        .ok_or_else(|| "timeline-overflow-at-note-start".to_string())?;
                    let global_end = measure_start_tick
                        .checked_add(local_end)
                        .ok_or_else(|| "timeline-overflow-at-note-end".to_string())?;

                    if let Some(pitch) = direct_child(child, "pitch") {
                        if duration <= 0 {
                            return Err("pitched-note-nonpositive-duration".to_string());
                        }
                        let midi = parse_midi_pitch(pitch)?;
                        let source_staff = direct_child_text(child, "staff")
                            .unwrap_or_else(|| "unspecified".to_string());
                        let source_voice = direct_child_text(child, "voice")
                            .unwrap_or_else(|| "unspecified".to_string());
                        let (tie_start, tie_stop) = tie_flags(child)?;
                        segments.push(NoteSegment {
                            source_ordinal,
                            part_id: part_id.clone(),
                            source_staff,
                            source_voice,
                            midi,
                            start_tick: global_start,
                            end_tick: global_end,
                            tie_start,
                            tie_stop,
                        });
                        source_ordinal = source_ordinal
                            .checked_add(1)
                            .ok_or_else(|| "source-ordinal-overflow".to_string())?;
                    }

                    previous_note_start_tick = Some(local_start);
                    if !is_chord {
                        cursor_tick = cursor_tick
                            .checked_add(duration)
                            .ok_or_else(|| "timeline-overflow-at-note-cursor".to_string())?;
                    }
                    measure_max_tick = measure_max_tick.max(local_end);
                    measure_max_tick = measure_max_tick.max(cursor_tick);
                }
                _ => {}
            }
        }

        measure_start_tick = measure_start_tick
            .checked_add(measure_max_tick)
            .ok_or_else(|| "timeline-overflow-at-measure-boundary".to_string())?;
        score_terminal_tick = score_terminal_tick.max(measure_start_tick);
    }

    normalize_tempos(&mut tempos)?;
    if tempos.is_empty() {
        return Err("tempo-map-empty".to_string());
    }
    Ok(ParsedTimeline {
        segments,
        tempo_points: tempos,
        terminal_tick: score_terminal_tick,
    })
}

fn collapse_ties(segments: &[NoteSegment]) -> Result<Vec<PianisticExecutionEvent>, String> {
    let mut ordered = segments.to_vec();
    ordered.sort_by_key(|segment| (segment.start_tick, segment.source_ordinal));
    let mut active: BTreeMap<TieKey, usize> = BTreeMap::new();
    let mut events: Vec<PianisticExecutionEvent> = Vec::new();

    for segment in ordered {
        let key = TieKey {
            part_id: segment.part_id.clone(),
            source_staff: segment.source_staff.clone(),
            source_voice: segment.source_voice.clone(),
            midi: segment.midi,
        };
        if segment.tie_stop {
            let Some(index) = active.get(&key).copied() else {
                return Err(format!(
                    "orphan-tie-stop:{}:{}:{}:{}@{}",
                    key.part_id, key.source_staff, key.source_voice, key.midi, segment.start_tick
                ));
            };
            let event = events
                .get_mut(index)
                .ok_or_else(|| "internal-active-tie-index-invalid".to_string())?;
            if event.release_tick != segment.start_tick {
                return Err(format!(
                    "noncontiguous-tie:{}:{}:{}:{}:{}->{}",
                    key.part_id,
                    key.source_staff,
                    key.source_voice,
                    key.midi,
                    event.release_tick,
                    segment.start_tick
                ));
            }
            event.release_tick = segment.end_tick;
            event.note_segments = event
                .note_segments
                .checked_add(1)
                .ok_or_else(|| "tie-segment-count-overflow".to_string())?;
            if !segment.tie_start {
                active.remove(&key);
            }
            continue;
        }

        if active.contains_key(&key) {
            return Err(format!(
                "new-attack-before-open-tie-stop:{}:{}:{}:{}@{}",
                key.part_id, key.source_staff, key.source_voice, key.midi, segment.start_tick
            ));
        }
        let index = events.len();
        events.push(PianisticExecutionEvent {
            part_id: segment.part_id,
            source_staff: segment.source_staff,
            source_voice: segment.source_voice,
            midi: segment.midi,
            attack_tick: segment.start_tick,
            release_tick: segment.end_tick,
            attack_time_us_floor: 0,
            release_time_us_floor: 0,
            note_segments: 1,
        });
        if segment.tie_start {
            active.insert(key, index);
        }
    }

    if let Some((key, index)) = active.iter().next() {
        let event = &events[*index];
        return Err(format!(
            "open-tie-start:{}:{}:{}:{}@{}",
            key.part_id, key.source_staff, key.source_voice, key.midi, event.attack_tick
        ));
    }
    events.sort_by(|left, right| {
        (
            left.attack_tick,
            left.midi,
            &left.part_id,
            &left.source_staff,
            &left.source_voice,
            left.release_tick,
        )
            .cmp(&(
                right.attack_tick,
                right.midi,
                &right.part_id,
                &right.source_staff,
                &right.source_voice,
                right.release_tick,
            ))
    });
    Ok(events)
}

fn normalize_tempos(points: &mut Vec<TempoPoint>) -> Result<(), String> {
    points.sort_by_key(|point| (point.tick, point.milli_bpm));
    let mut normalized: Vec<TempoPoint> = Vec::new();
    for point in points.drain(..) {
        if point.tick < 0 {
            return Err("tempo-before-score-origin".to_string());
        }
        if let Some(previous) = normalized.last()
            && previous.tick == point.tick {
                if previous.milli_bpm != point.milli_bpm {
                    return Err(format!("conflicting-tempo-at-tick-{}", point.tick));
                }
                continue;
            }
        normalized.push(point);
    }
    *points = normalized;
    Ok(())
}

fn physical_time_us_floor(target_tick: i64, tempos: &[TempoPoint]) -> Option<u64> {
    if target_tick < 0 || tempos.first()?.tick != 0 {
        return None;
    }
    let mut total_us = 0u128;
    let mut cursor = 0i64;
    let mut tempo = tempos[0].milli_bpm;
    for point in tempos.iter().skip(1) {
        if point.tick > target_tick {
            break;
        }
        let delta = point.tick.checked_sub(cursor)?;
        total_us = total_us.checked_add(duration_us_floor(delta, tempo)?)?;
        cursor = point.tick;
        tempo = point.milli_bpm;
    }
    let delta = target_tick.checked_sub(cursor)?;
    total_us = total_us.checked_add(duration_us_floor(delta, tempo)?)?;
    u64::try_from(total_us).ok()
}

fn duration_us_floor(delta_ticks: i64, milli_bpm: u64) -> Option<u128> {
    if delta_ticks < 0 || milli_bpm == 0 {
        return None;
    }
    let numerator = u128::try_from(delta_ticks)
        .ok()?
        .checked_mul(60_000_000_000u128)?;
    let denominator = u128::try_from(CANONICAL_TICKS_PER_QUARTER)
        .ok()?
        .checked_mul(u128::from(milli_bpm))?;
    Some(numerator / denominator)
}

fn musical_fingerprint(events: &[PianisticExecutionEvent], tempos: &[TempoPoint]) -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(SCHEMA.as_bytes());
    bytes.extend_from_slice(&CANONICAL_TICKS_PER_QUARTER.to_le_bytes());
    let mut physical_events = events
        .iter()
        .map(|event| (event.attack_tick, event.release_tick, event.midi, event.note_segments))
        .collect::<Vec<_>>();
    physical_events.sort();
    for (attack_tick, release_tick, midi, note_segments) in physical_events {
        bytes.extend_from_slice(&midi.to_le_bytes());
        bytes.extend_from_slice(&attack_tick.to_le_bytes());
        bytes.extend_from_slice(&release_tick.to_le_bytes());
        bytes.extend_from_slice(&u64::try_from(note_segments).unwrap_or(u64::MAX).to_le_bytes());
    }
    for point in tempos {
        bytes.extend_from_slice(&point.tick.to_le_bytes());
        bytes.extend_from_slice(&point.milli_bpm.to_le_bytes());
    }
    sha256::hex(&bytes)
}

fn source_affinity_fingerprint(events: &[PianisticExecutionEvent]) -> String {
    let mut bytes = Vec::new();
    for event in events {
        append_len_prefixed(&mut bytes, event.part_id.as_bytes());
        append_len_prefixed(&mut bytes, event.source_staff.as_bytes());
        append_len_prefixed(&mut bytes, event.source_voice.as_bytes());
        bytes.extend_from_slice(&event.midi.to_le_bytes());
        bytes.extend_from_slice(&event.attack_tick.to_le_bytes());
        bytes.extend_from_slice(&event.release_tick.to_le_bytes());
    }
    sha256::hex(&bytes)
}

fn append_len_prefixed(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&u64::try_from(bytes.len()).unwrap_or(u64::MAX).to_le_bytes());
    out.extend_from_slice(bytes);
}

fn scale_divisions(value: i64, divisions: i64) -> Result<i64, String> {
    if divisions <= 0 {
        return Err("nonpositive-divisions".to_string());
    }
    let numerator = value
        .checked_mul(CANONICAL_TICKS_PER_QUARTER)
        .ok_or_else(|| "division-scale-overflow".to_string())?;
    if numerator % divisions != 0 {
        return Err(format!(
            "nonexact-canonical-96-projection:value-{value}:divisions-{divisions}"
        ));
    }
    Ok(numerator / divisions)
}

fn parse_milli_bpm(raw: &str) -> Result<u64, String> {
    let raw = raw.trim();
    if raw.is_empty() || raw.starts_with('-') || raw.starts_with('+') {
        return Err(format!("invalid-tempo:{raw}"));
    }
    let (whole, fraction) = raw.split_once('.').unwrap_or((raw, ""));
    if fraction.len() > 3 || !whole.chars().all(|ch| ch.is_ascii_digit()) || !fraction.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!("invalid-tempo:{raw}"));
    }
    let whole = whole.parse::<u64>().map_err(|_| format!("invalid-tempo:{raw}"))?;
    let mut milli = whole.checked_mul(1_000).ok_or_else(|| "tempo-overflow".to_string())?;
    let fraction_value = if fraction.is_empty() {
        0
    } else {
        let parsed = fraction.parse::<u64>().map_err(|_| format!("invalid-tempo:{raw}"))?;
        parsed * 10u64.pow(u32::try_from(3usize.saturating_sub(fraction.len())).unwrap_or(0))
    };
    milli = milli.checked_add(fraction_value).ok_or_else(|| "tempo-overflow".to_string())?;
    if milli == 0 {
        return Err("tempo-must-be-positive".to_string());
    }
    Ok(milli)
}

fn parse_midi_pitch(pitch: Node<'_, '_>) -> Result<i32, String> {
    let step = direct_child_text(pitch, "step").ok_or_else(|| "pitch-step-missing".to_string())?;
    let octave = direct_child_text(pitch, "octave")
        .ok_or_else(|| "pitch-octave-missing".to_string())?
        .parse::<i32>()
        .map_err(|_| "pitch-octave-invalid".to_string())?;
    let alter = direct_child_text(pitch, "alter")
        .map(|text| text.parse::<i32>().map_err(|_| "pitch-alter-invalid".to_string()))
        .transpose()?
        .unwrap_or(0);
    let base = match step.as_str() {
        "C" => 0,
        "D" => 2,
        "E" => 4,
        "F" => 5,
        "G" => 7,
        "A" => 9,
        "B" => 11,
        _ => return Err(format!("pitch-step-invalid:{step}")),
    };
    let midi = (octave + 1)
        .checked_mul(12)
        .and_then(|value| value.checked_add(base))
        .and_then(|value| value.checked_add(alter))
        .ok_or_else(|| "pitch-midi-overflow".to_string())?;
    if !(0..=127).contains(&midi) {
        return Err(format!("pitch-midi-out-of-range:{midi}"));
    }
    Ok(midi)
}

fn tie_flags(note: Node<'_, '_>) -> Result<(bool, bool), String> {
    let mut start = false;
    let mut stop = false;
    for tie in note.children().filter(|node| is_element(*node, "tie")) {
        match tie.attribute("type") {
            Some("start") => start = true,
            Some("stop") => stop = true,
            Some(other) => return Err(format!("unsupported-tie-type:{other}")),
            None => return Err("tie-type-missing".to_string()),
        }
    }
    Ok((start, stop))
}

fn required_duration(node: Node<'_, '_>, owner: &str) -> Result<i64, String> {
    let text = direct_child_text(node, "duration")
        .ok_or_else(|| format!("{owner}-duration-missing"))?;
    parse_nonnegative_i64(&text, &format!("{owner}-duration"))
}

fn parse_positive_i64(raw: &str, label: &str) -> Result<i64, String> {
    let value = parse_i64(raw, label)?;
    if value <= 0 {
        return Err(format!("{label}-must-be-positive:{value}"));
    }
    Ok(value)
}

fn parse_nonnegative_i64(raw: &str, label: &str) -> Result<i64, String> {
    let value = parse_i64(raw, label)?;
    if value < 0 {
        return Err(format!("{label}-must-be-nonnegative:{value}"));
    }
    Ok(value)
}

fn parse_i64(raw: &str, label: &str) -> Result<i64, String> {
    raw.trim()
        .parse::<i64>()
        .map_err(|_| format!("{label}-invalid:{raw}"))
}

fn direct_child<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Option<Node<'a, 'input>> {
    node.children().find(|child| is_element(*child, name))
}

fn direct_child_text(node: Node<'_, '_>, name: &str) -> Option<String> {
    direct_child(node, name)
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_string)
}

fn is_element(node: Node<'_, '_>, name: &str) -> bool {
    node.is_element() && node.tag_name().name() == name
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(body: &str, tempo: &str) -> String {
        format!(
            "<?xml version=\"1.0\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><direction><sound tempo=\"{tempo}\"/></direction>{body}</measure></part></score-partwise>"
        )
    }

    fn note(step: &str, octave: i32, duration: i64, extra: &str) -> String {
        format!(
            "<note>{extra}<pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration><voice>1</voice><staff>1</staff></note>"
        )
    }

    #[test]
    fn single_note_creates_one_attack_hold_release_event() {
        let xml = score(&note("C", 4, 96, ""), "120");
        let projection = project_xml(PathBuf::from("x.musicxml"), &xml);
        assert_eq!(projection.status, ProjectionStatus::Complete);
        assert_eq!(projection.pitched_note_segments, 1);
        assert_eq!(projection.execution_events.len(), 1);
        assert_eq!(projection.execution_events[0].attack_tick, 0);
        assert_eq!(projection.execution_events[0].release_tick, 96);
        assert_eq!(projection.execution_events[0].attack_time_us_floor, 0);
        assert_eq!(projection.execution_events[0].release_time_us_floor, 500_000);
        assert_eq!(projection.terminal_physical_time_us_floor, Some(500_000));
    }

    #[test]
    fn tie_chain_extends_hold_without_second_attack() {
        let body = format!(
            "{}{}",
            note("C", 4, 96, "<tie type=\"start\"/>"),
            note("C", 4, 96, "<tie type=\"stop\"/>")
        );
        let projection = project_xml(PathBuf::from("x.musicxml"), &score(&body, "120"));
        assert_eq!(projection.status, ProjectionStatus::Complete);
        assert_eq!(projection.pitched_note_segments, 2);
        assert_eq!(projection.tie_continuation_segments, 1);
        assert_eq!(projection.execution_events.len(), 1);
        assert_eq!(projection.execution_events[0].release_tick, 192);
        assert_eq!(projection.execution_events[0].note_segments, 2);
    }

    #[test]
    fn chord_notes_share_attack_tick_without_advancing_cursor_twice() {
        let body = format!(
            "{}{}{}",
            note("C", 4, 96, ""),
            note("E", 4, 96, "<chord/>"),
            note("G", 4, 96, "")
        );
        let projection = project_xml(PathBuf::from("x.musicxml"), &score(&body, "120"));
        assert_eq!(projection.status, ProjectionStatus::Complete);
        let events = &projection.execution_events;
        assert_eq!(events.len(), 3);
        assert_eq!(events.iter().filter(|event| event.attack_tick == 0).count(), 2);
        assert_eq!(events.iter().filter(|event| event.attack_tick == 96).count(), 1);
        assert_eq!(projection.terminal_tick, 192);
    }

    #[test]
    fn backup_preserves_polyphonic_score_time() {
        let upper = note("C", 5, 192, "");
        let lower = note("C", 3, 96, "").replace("<voice>1</voice><staff>1</staff>", "<voice>2</voice><staff>2</staff>");
        let body = format!("{upper}<backup><duration>192</duration></backup>{lower}");
        let projection = project_xml(PathBuf::from("x.musicxml"), &score(&body, "120"));
        assert_eq!(projection.status, ProjectionStatus::Complete);
        assert_eq!(projection.execution_events.len(), 2);
        assert!(projection.execution_events.iter().all(|event| event.attack_tick == 0));
        assert_eq!(projection.terminal_tick, 192);
    }

    #[test]
    fn tempo_change_is_integrated_deterministically() {
        let body = format!(
            "{}<direction><sound tempo=\"60\"/></direction>{}",
            note("C", 4, 96, ""),
            note("D", 4, 96, "")
        );
        let projection = project_xml(PathBuf::from("x.musicxml"), &score(&body, "120"));
        assert_eq!(projection.status, ProjectionStatus::Complete);
        assert_eq!(projection.tempo_points.len(), 2);
        assert_eq!(projection.tempo_points[1].tick, 96);
        assert_eq!(projection.terminal_physical_time_us_floor, Some(1_500_000));
    }

    #[test]
    fn nonexact_division_projection_is_not_silently_rounded() {
        let xml = score(&note("C", 4, 1, ""), "120").replace("<divisions>96</divisions>", "<divisions>7</divisions>");
        let projection = project_xml(PathBuf::from("x.musicxml"), &xml);
        assert_eq!(projection.status, ProjectionStatus::ModelInputUnavailable);
        assert!(projection.reason.contains("nonexact-canonical-96-projection"));
    }

    #[test]
    fn musical_fingerprint_ignores_staff_voice_affinity_but_affinity_fingerprint_does_not() {
        let base = score(&note("C", 4, 96, ""), "120");
        let moved = base.replace("<voice>1</voice><staff>1</staff>", "<voice>9</voice><staff>2</staff>");
        let left = project_xml(PathBuf::from("a.musicxml"), &base);
        let right = project_xml(PathBuf::from("b.musicxml"), &moved);
        assert_eq!(left.musical_execution_fingerprint_sha256, right.musical_execution_fingerprint_sha256);
        assert_ne!(left.source_affinity_fingerprint_sha256, right.source_affinity_fingerprint_sha256);
    }

    #[test]
    fn orphan_tie_stop_fails_closed_without_playability_claim() {
        let xml = score(&note("C", 4, 96, "<tie type=\"stop\"/>"), "120");
        let projection = project_xml(PathBuf::from("x.musicxml"), &xml);
        assert_eq!(projection.status, ProjectionStatus::ModelInputUnavailable);
        assert!(projection.reason.contains("orphan-tie-stop"));
        assert!(projection.execution_events.is_empty());
    }
}
