//! Audit-only origin readback for voice timeline collisions.
//!
//! The pipeline already reports adopted states (accepted repair rounds, stage
//! checkpoints, final artifacts) and the exact writer batches that produce
//! states. This module evaluates the voice timeline collision class of
//! docs/selector_derivation_contract_staff_relocation_notation_integrity_error_class.md
//! on every reported state and records, per collision, the first reported state
//! that contains it, the reporting location and, when that state was produced by
//! a registered writer batch, the batch origin. It owns no write path and changes
//! no decision.

use super::voice_run_integrity::{
    measure_event_trace, voice_timeline_collisions, VoiceTimelineCollision,
};
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::sync::Mutex;
use std::thread::ThreadId;

#[derive(Debug, Clone)]
struct FirstSighting {
    observation: usize,
    label: String,
    writer_origin: Option<String>,
}

#[derive(Debug, Clone)]
struct Introducer {
    transition: usize,
    writer_origin: String,
    measure_before: String,
    measure_after: String,
}

#[derive(Debug)]
struct ProjectionEvidence {
    before_xml: String,
    after_xml: String,
    introductions: Vec<(VoiceTimelineCollision, Introducer)>,
}

#[derive(Debug, Clone)]
struct AdoptedProjection {
    collision: VoiceTimelineCollision,
    introduction: Introducer,
    survives: bool,
    checkpoint_trace: String,
    boundary: String,
}

#[derive(Debug, Default)]
struct OriginLedger {
    writer_origins: HashMap<u64, String>,
    introducers: BTreeMap<VoiceTimelineCollision, Introducer>,
    writer_transitions: usize,
    projector_transitions: usize,
    pending_projections: Vec<ProjectionEvidence>,
    adopted_projections: Vec<AdoptedProjection>,
    saw_collision_free_report: bool,
    first_sightings: BTreeMap<VoiceTimelineCollision, FirstSighting>,
    observations: usize,
    unparsed_observations: usize,
}

/// One ledger per session thread (the project thread that called `start`).
/// Writer batches on worker threads join the session of their own thread or,
/// when exactly one session exists, that session.
static LEDGERS: Mutex<Vec<(ThreadId, OriginLedger)>> = Mutex::new(Vec::new());

// The owning session takes precedence; a foreign thread needs one session.
fn ledger_index(
    ledgers: &[(ThreadId, OriginLedger)],
    thread: ThreadId,
    allow_single_foreign: bool,
) -> Option<usize> {
    ledgers.iter().position(|(owner, _)| *owner == thread).or_else(|| {
        (allow_single_foreign && matches!(ledgers, [_])).then_some(0)
    })
}

fn with_ledger(allow_single_foreign: bool, apply: impl FnOnce(&mut OriginLedger)) {
    let thread = std::thread::current().id();
    let Ok(mut ledgers) = LEDGERS.lock() else {
        return;
    };
    if let Some(index) = ledger_index(&ledgers, thread, allow_single_foreign) {
        apply(&mut ledgers[index].1);
    }
}

fn state_key(xml: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    xml.hash(&mut hasher);
    hasher.finish()
}

pub(crate) fn start() {
    let thread = std::thread::current().id();
    if let Ok(mut ledgers) = LEDGERS.lock() {
        ledgers.retain(|(owner, _)| *owner != thread);
        ledgers.push((thread, OriginLedger::default()));
    }
}

/// A registered writer batch turned `before_xml` into `after_xml`. On the
/// owning thread, or a worker with one unambiguous session, the collision class
/// is evaluated after a collision-free reported state and only for backup-bearing
/// output. The first introducing batch includes both affected measure traces.
pub(crate) fn observe_writer_transition(origin: &str, before_xml: &str, after_xml: &str) {
    let key = state_key(after_xml);
    let mut inspect_transition = false;
    with_ledger(true, |ledger| {
        inspect_transition = ledger.saw_collision_free_report;
        ledger
            .writer_origins
            .entry(key)
            .or_insert_with(|| origin.to_string());
    });
    if !inspect_transition || !after_xml.contains("<backup") {
        return;
    }
    let Some(after) = voice_timeline_collisions(after_xml) else {
        return;
    };
    let before = if after.is_empty() {
        Vec::new()
    } else {
        voice_timeline_collisions(before_xml).unwrap_or_default()
    };
    with_ledger(true, |ledger| {
        ledger.writer_transitions += 1;
        let transition = ledger.writer_transitions;
        for collision in after {
            if before.contains(&collision) || ledger.introducers.contains_key(&collision) {
                continue;
            }
            let measure_before =
                measure_event_trace(before_xml, collision.part_ordinal, &collision.measure)
                    .unwrap_or_else(|| "unreadable".to_string());
            let measure_after =
                measure_event_trace(after_xml, collision.part_ordinal, &collision.measure)
                    .unwrap_or_else(|| "unreadable".to_string());
            ledger.introducers.insert(
                collision,
                Introducer {
                    transition,
                    writer_origin: origin.to_string(),
                    measure_before,
                    measure_after,
                },
            );
        }
    });
}

/// Direct readback of the FinalChord projector, including its read-only probes.
/// Introducing outputs remain provisional until the FinalNotation caller reports
/// the very same continuation after a successful receipt-backed adoption.
pub(crate) fn observe_final_chord_projection(before_xml: &str, after_xml: &str) {
    if before_xml == after_xml || !after_xml.contains("<backup") {
        return;
    }
    let Some(after) = voice_timeline_collisions(after_xml) else { return; };
    let Some(before) = voice_timeline_collisions(before_xml) else { return; };
    with_ledger(true, |ledger| {
        ledger.projector_transitions += 1;
        let mut introductions = Vec::new();
        for collision in after {
            if before.contains(&collision) { continue; }
            let introduction = Introducer {
                transition: ledger.projector_transitions,
                writer_origin: "src/structural_write_authority/serialization_plan.rs::project_final_chord_encoding_plan".to_string(),
                measure_before: measure_event_trace(before_xml, collision.part_ordinal, &collision.measure)
                    .unwrap_or_else(|| "unreadable".to_string()),
                measure_after: measure_event_trace(after_xml, collision.part_ordinal, &collision.measure)
                    .unwrap_or_else(|| "unreadable".to_string()),
            };
            introductions.push((collision, introduction));
        }
        if !introductions.is_empty() {
            ledger.pending_projections.push(ProjectionEvidence { before_xml: before_xml.to_string(), after_xml: after_xml.to_string(), introductions });
        }
    });
}

/// Called only after the receipt-backed Render checkpoint or Master export succeeded.
/// Exact input and continuation bytes bind the projector observation; checkpoint readback
/// separately records whether each introduced collision survived later rest edits.
pub(crate) fn observe_final_chord_adoption(boundary: &str, before_xml: &str, projected_xml: &str, checkpoint_xml: &str) {
    let mut has_projection = false;
    with_ledger(true, |ledger| {
        has_projection = ledger.pending_projections.iter().any(|proof| proof.before_xml == before_xml && proof.after_xml == projected_xml);
    });
    if !has_projection { return; }
    let Some(collisions) = voice_timeline_collisions(checkpoint_xml) else { return; };
    with_ledger(true, |ledger| {
        let Some(index) = ledger.pending_projections.iter().rposition(|proof| proof.before_xml == before_xml && proof.after_xml == projected_xml) else { return; };
        let proof = ledger.pending_projections.remove(index);
        for (collision, introduction) in proof.introductions {
            let survives = collisions.contains(&collision);
            let checkpoint_trace = measure_event_trace(checkpoint_xml, collision.part_ordinal, &collision.measure)
                .unwrap_or_else(|| "unreadable".to_string());
            if survives {
                ledger.introducers.insert(collision.clone(), introduction.clone());
            }
            ledger.adopted_projections.push(AdoptedProjection { collision, introduction, survives, checkpoint_trace, boundary: boundary.to_string() });
        }
    });
}

/// A state the pipeline reported as adopted or as a stage checkpoint.
pub(crate) fn observe_reported_state(label: &str, xml: &str) {
    let key = state_key(xml);
    let collisions = voice_timeline_collisions(xml);
    with_ledger(true, |ledger| {
        ledger.observations += 1;
        let observation = ledger.observations;
        let Some(collisions) = collisions else {
            ledger.unparsed_observations += 1;
            return;
        };
        ledger.saw_collision_free_report |= collisions.is_empty();
        let writer_origin = ledger.writer_origins.get(&key).cloned();
        for collision in collisions {
            ledger
                .first_sightings
                .entry(collision)
                .or_insert_with(|| FirstSighting {
                    observation,
                    label: label.to_string(),
                    writer_origin: writer_origin.clone(),
                });
        }
    });
}

pub(crate) fn append_report_section(out: &mut String, master: Option<&Path>) {
    let Some(master) = master else {
        return;
    };
    let final_collisions = std::fs::read_to_string(master)
        .ok()
        .and_then(|xml| voice_timeline_collisions(&xml));
    let mut snapshot = (0, 0, BTreeMap::new());
    let mut introducers = BTreeMap::new();
    let mut session_transitions = 0;
    let mut projector_snapshot = (0, 0, Vec::new());
    with_ledger(true, |ledger| {
        snapshot = (
            ledger.observations,
            ledger.unparsed_observations,
            ledger.first_sightings.clone(),
        );
        introducers = ledger.introducers.clone();
        session_transitions = ledger.writer_transitions;
        projector_snapshot = (ledger.projector_transitions, ledger.pending_projections.len(), ledger.adopted_projections.clone());
    });
    let (observations, unparsed, sightings) = snapshot;
    out.push_str("VOICE-RUN-COLLISION-ORIGIN-AUDIT\n");
    out.push_str("================================\n");
    out.push_str("ChangeClass: RefactorOnly; EvidenceSource reported adopted states, stage checkpoints, registered writer batches and direct FinalChord projector transitions; MusicMutation nein; WriteAuthority none.\n");
    let final_count = final_collisions.as_ref().map_or(0, Vec::len);
    let traced = final_collisions.as_ref().map_or(0, |collisions| {
        collisions
            .iter()
            .filter(|collision| sightings.contains_key(*collision))
            .count()
    });
    let _ = writeln!(
        out,
        "Readback: ReportedStates {observations}; UnparsedStates {unparsed}; CollisionsEverSeen {}; FinalMasterCollisions {final_count}; FinalCollisionsTraced {traced}; FinalMasterReadable {}.",
        sightings.len(),
        if final_collisions.is_some() { "ja" } else { "nein" },
    );
    for (collision, sighting) in &sightings {
        let in_final = final_collisions
            .as_ref()
            .is_some_and(|collisions| collisions.contains(collision));
        let _ = writeln!(
            out,
            "CollisionOrigin: Part {}; M{}; Onset {}; Pitch {}; Staff {}; Voice {}; RunVoice {}; InFinalMaster {}; FirstReportedState {}; FirstSeenAt {}; WriterOrigin {}.",
            collision.part_ordinal,
            collision.measure,
            collision.onset,
            collision.pitch,
            collision.staff,
            collision.voice,
            collision.run_voice,
            if in_final { "ja" } else { "nein" },
            sighting.observation,
            sighting.label,
            sighting.writer_origin.as_deref().unwrap_or("not_registered"),
        );
    }
    let _ = writeln!(
        out,
        "IntroducerReadback: SessionWriterTransitions {session_transitions}; CollisionsWithIntroducer {}.",
        introducers.len()
    );
    for (collision, introducer) in &introducers {
        let _ = writeln!(
            out,
            "CollisionIntroducer: Part {}; M{}; Onset {}; Pitch {}; Staff {}; Voice {}; RunVoice {}; Transition {}; WriterOrigin {}; MeasureBefore [{}]; MeasureAfter [{}].",
            collision.part_ordinal,
            collision.measure,
            collision.onset,
            collision.pitch,
            collision.staff,
            collision.voice,
            collision.run_voice,
            introducer.transition,
            introducer.writer_origin,
            introducer.measure_before,
            introducer.measure_after,
        );
    }
    let (projector_transitions, pending, adopted) = projector_snapshot;
    let _ = writeln!(out, "FinalChordProjectorReadback: ParsedTransitions {projector_transitions}; PendingIntroducingOutputs {pending}; AdoptedCollisionEvidence {}.", adopted.len());
    for proof in &adopted {
        let collision = &proof.collision;
        let _ = writeln!(out,
            "FinalChordProjectionAdoption: Part {}; M{}; Onset {}; Pitch {}; Staff {}; Voice {}; RunVoice {}; ProjectorTransition {}; AdoptionBoundary {}; Binding exact_input_and_continuation_bytes; CollisionSurvives {}; MeasureCheckpoint [{}].",
            collision.part_ordinal, collision.measure, collision.onset, collision.pitch,
            collision.staff, collision.voice, collision.run_voice, proof.introduction.transition, proof.boundary,
            if proof.survives { "ja" } else { "nein" }, proof.checkpoint_trace);
    }
    out.push_str("Reading: FirstSeenAt names the code location that reported the first adopted state or checkpoint containing the collision; the producing step lies between that report and the previous one. WriterOrigin is set only when that exact state came from a registered writer batch. Collisions present before the first reported state are attributed to the first report. FinalChordProjectionAdoption proves a direct projector transition was carried by the continuation whose receipt chain was successfully adopted at the named Render or Master boundary; it also reports collision survival in the committed checkpoint. Unadopted projector probes do not become CollisionIntroducer. A surviving adopted projector introduction takes precedence over provisional batch evidence. CollisionIntroducer otherwise names the first registered writer batch in the owning or sole session whose output contains the collision while its input did not; the measure traces list n:onset/duration/staff/voice/pitch with c for chord and h for hidden, b: backup and f: forward.\n\n");
}

#[cfg(test)]
mod tests {
    use super::{append_report_section, ledger_index, observe_reported_state, observe_writer_transition, start, with_ledger, OriginLedger};

    fn score(body: &str) -> String {
        let number = 2;
        format!(
            "<score-partwise><part id=\"P1\"><measure number=\"{number}\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{body}</measure></part></score-partwise>"
        )
    }

    #[test]
    fn first_sighting_and_writer_origin_are_recorded() {
        start();
        let clean = score(
            "<note><pitch><step>E</step><octave>5</octave></pitch><duration>16</duration><voice>1</voice><staff>1</staff></note><backup><duration>16</duration></backup><note><pitch><step>C</step><octave>3</octave></pitch><duration>16</duration><voice>2</voice><staff>2</staff></note>",
        );
        let colliding = score(
            "<note><pitch><step>E</step><octave>5</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><note><pitch><step>F</step><octave>4</octave></pitch><duration>4</duration><voice>2</voice><staff>2</staff></note><note><pitch><step>E</step><octave>5</octave></pitch><duration>8</duration><voice>1</voice><staff>1</staff></note><backup><duration>16</duration></backup><note><pitch><step>C</step><octave>3</octave></pitch><duration>16</duration><voice>2</voice><staff>2</staff></note>",
        );
        observe_writer_transition("before_clean", &clean, &format!("{colliding}\n"));
        with_ledger(false, |ledger| {
            assert_eq!(ledger.writer_transitions, 0);
            assert!(ledger.introducers.is_empty());
        });
        observe_reported_state("first", &clean);
        observe_writer_transition("without_backup", &clean, &score(""));
        with_ledger(false, |ledger| assert_eq!(ledger.writer_transitions, 0));
        observe_writer_transition("writer.rs:7", &clean, &colliding);
        observe_reported_state("second", &colliding);
        observe_reported_state("third", &colliding);
        let dir = std::env::temp_dir().join(format!("etuschlopf_vrco_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("dir");
        let master = dir.join("master.musicxml");
        std::fs::write(&master, &colliding).expect("master");
        let mut out = String::new();
        append_report_section(&mut out, Some(&master));
        std::fs::remove_dir_all(&dir).expect("cleanup");
        assert!(out.contains("ReportedStates 3"));
        assert!(out.contains("FinalMasterCollisions 1; FinalCollisionsTraced 1"));
        assert!(out.contains("InFinalMaster ja; FirstReportedState 2; FirstSeenAt second; WriterOrigin writer.rs:7."));
        assert!(out.contains("CollisionsWithIntroducer 1"));
        assert!(out.contains("Transition 1; WriterOrigin writer.rs:7; MeasureBefore ["));
    }

    #[test]
    fn foreign_thread_requires_one_session_and_owner_takes_precedence() {
        let owner = std::thread::current().id();
        let foreign = std::thread::spawn(|| std::thread::current().id()).join().expect("foreign thread");
        let second_owner = std::thread::spawn(|| std::thread::current().id()).join().expect("second owner");
        let mut ledgers = Vec::new();
        assert_eq!(ledger_index(&ledgers, foreign, true), None);
        ledgers.push((owner, OriginLedger::default()));
        assert_eq!(ledger_index(&ledgers, owner, false), Some(0));
        assert_eq!(ledger_index(&ledgers, foreign, false), None);
        assert_eq!(ledger_index(&ledgers, foreign, true), Some(0));
        ledgers.push((second_owner, OriginLedger::default()));
        assert_eq!(ledger_index(&ledgers, foreign, true), None);
        assert_eq!(ledger_index(&ledgers, owner, true), Some(0));
        assert_eq!(ledger_index(&ledgers, second_owner, true), Some(1));
    }

    fn projection_fixture() -> (String, String) {
        let head = "<note><pitch><step>E</step><octave>5</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>";
        let tail = "<note><pitch><step>E</step><octave>5</octave></pitch><duration>8</duration><voice>1</voice><staff>1</staff></note><backup><duration>16</duration></backup><note><pitch><step>C</step><octave>3</octave></pitch><duration>16</duration><voice>2</voice><staff>2</staff></note>";
        let body = |voice| format!("{head}<note><pitch><step>F</step><octave>4</octave></pitch><duration>4</duration><voice>{voice}</voice><staff>2</staff></note>{tail}");
        (score(&body(1)), score(&body(2)))
    }

    #[test]
    fn projector_probe_requires_exact_continuation_and_successful_adoption() {
        start();
        let (before, after) = projection_fixture();
        super::observe_final_chord_projection(&before, &after);
        with_ledger(false, |ledger| {
            assert_eq!(ledger.projector_transitions, 1);
            assert_eq!(ledger.pending_projections.len(), 1);
            assert!(ledger.introducers.is_empty());
            assert!(ledger.adopted_projections.is_empty());
        });
        super::observe_final_chord_adoption("FinalNotationReceiptChainCommitted", &before, &format!("{after}\n"), &after);
        super::observe_final_chord_adoption("FinalNotationReceiptChainCommitted", &after, &after, &after);
        with_ledger(false, |ledger| assert!(ledger.adopted_projections.is_empty()));
        super::observe_final_chord_adoption("FinalNotationReceiptChainCommitted", &before, &after, &after);
        with_ledger(false, |ledger| {
            assert_eq!(ledger.introducers.len(), 1);
            assert!(ledger.pending_projections.is_empty());
            assert_eq!(ledger.adopted_projections.len(), 1);
            let proof = &ledger.adopted_projections[0];
            assert!(proof.survives);
            assert!(proof.introduction.measure_before.contains("/1/F4"));
            assert!(proof.introduction.measure_after.contains("/2/F4"));
            assert_eq!(proof.checkpoint_trace, proof.introduction.measure_after);
        });
    }

    #[test]
    fn committed_checkpoint_records_disappeared_collision_without_introducer() {
        start();
        let (before, after) = projection_fixture();
        super::observe_final_chord_projection(&before, &after);
        super::observe_final_chord_adoption("ReleasedMasterFinalExportReceiptChainCommitted", &before, &after, &before);
        with_ledger(false, |ledger| {
            assert!(ledger.introducers.is_empty());
            assert_eq!(ledger.adopted_projections.len(), 1);
            assert!(!ledger.adopted_projections[0].survives);
            assert_eq!(ledger.adopted_projections[0].boundary, "ReleasedMasterFinalExportReceiptChainCommitted");
        });
    }

    #[test]
    fn projector_cannot_attribute_unreadable_input_or_existing_collision() {
        start();
        let (before, after) = projection_fixture();
        super::observe_final_chord_projection("unreadable", &after);
        super::observe_final_chord_projection(&after, &format!("{after}\n"));
        super::observe_final_chord_projection(&before, &format!("{before}\n"));
        with_ledger(false, |ledger| {
            assert_eq!(ledger.projector_transitions, 2);
            assert!(ledger.pending_projections.is_empty());
            assert!(ledger.introducers.is_empty());
        });
    }

}
