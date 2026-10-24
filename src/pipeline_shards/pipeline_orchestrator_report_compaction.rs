use structural_write_authority::RustigClefSurfaceDecision;
use crate::audit_only::mutation_ownership_evidence::RestArticulationRepairLoopInvocationEvidence;
use crate::audit_only::phrase_boundary_evidence::PhraseAtomicObservation;

struct PhraseMeasureContext {
    part: String,
    measure_ordinal: usize,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy)]
struct PhraseMeasureTiming {
    absolute_start: i128,
    divisions: i64,
    beats: i64,
    beat_type: i64,
    canonical_beat_ticks: i128,
}

type PhraseCanonicalMeasureTiming = (
    i64,
    std::collections::BTreeMap<(String, usize), PhraseMeasureTiming>,
);

fn capture_phrase_boundary_evidence_foundation(source: &source::SourcePlan, xml: &str) {
    const SOURCE_CONTRACT: &str = "canonical_committed_master_musicxml_before_final_render_channel_closure";
    let contexts = phrase_measure_contexts(xml);
    let Ok(score) = crate::musicxml::parse_musicxml(xml) else {
        crate::audit_only::phrase_boundary_evidence::capture_unavailable(
            &source.input_label,
            SOURCE_CONTRACT,
        );
        return;
    };
    let Some((canonical_ticks_per_quarter, measure_timing)) =
        phrase_canonical_measure_timing(&score)
    else {
        crate::audit_only::phrase_boundary_evidence::capture_unavailable(
            &source.input_label,
            SOURCE_CONTRACT,
        );
        return;
    };
    let atomic_notes = parse_atomic_notes(xml);
    let mut context_index = 0usize;
    let mut projection = Vec::with_capacity(atomic_notes.len());

    for (source_ordinal, note) in atomic_notes.into_iter().enumerate() {
        while context_index < contexts.len() && note.note_start >= contexts[context_index].end {
            context_index = context_index.saturating_add(1);
        }
        let Some(context) = contexts.get(context_index) else {
            crate::audit_only::phrase_boundary_evidence::capture_unavailable(
                &source.input_label,
                SOURCE_CONTRACT,
            );
            return;
        };
        if note.note_start < context.start || note.note_start >= context.end {
            crate::audit_only::phrase_boundary_evidence::capture_unavailable(
                &source.input_label,
                SOURCE_CONTRACT,
            );
            return;
        }
        let Some(note_block) = xml.get(note.note_start..note.note_end) else {
            crate::audit_only::phrase_boundary_evidence::capture_unavailable(
                &source.input_label,
                SOURCE_CONTRACT,
            );
            return;
        };
        let Some(timing) = measure_timing.get(&(context.part.clone(), context.measure_ordinal))
        else {
            crate::audit_only::phrase_boundary_evidence::capture_unavailable(
                &source.input_label,
                SOURCE_CONTRACT,
            );
            return;
        };
        let Some(local_start) =
            phrase_canonical_scaled_value(note.start, canonical_ticks_per_quarter, timing.divisions)
        else {
            crate::audit_only::phrase_boundary_evidence::capture_unavailable(
                &source.input_label,
                SOURCE_CONTRACT,
            );
            return;
        };
        let Some(canonical_duration) = phrase_canonical_scaled_value(
            note.duration,
            canonical_ticks_per_quarter,
            timing.divisions,
        ) else {
            crate::audit_only::phrase_boundary_evidence::capture_unavailable(
                &source.input_label,
                SOURCE_CONTRACT,
            );
            return;
        };
        let Some(canonical_start) = timing.absolute_start.checked_add(local_start) else {
            crate::audit_only::phrase_boundary_evidence::capture_unavailable(
                &source.input_label,
                SOURCE_CONTRACT,
            );
            return;
        };
        let tie_start = phrase_note_has_tie_type(note_block, "start");
        let tie_stop = phrase_note_has_tie_type(note_block, "stop");
        projection.push(PhraseAtomicObservation {
            source_ordinal,
            part: context.part.clone(),
            measure_ordinal: context.measure_ordinal,
            staff: note.staff,
            voice: note.voice,
            start: note.start,
            duration: note.duration,
            canonical_start,
            canonical_duration,
            canonical_measure_start: timing.absolute_start,
            canonical_beat_ticks: timing.canonical_beat_ticks,
            measure_beats: timing.beats,
            measure_beat_type: timing.beat_type,
            pitch_midi: note.pitch.map(|pitch| pitch.midi),
            visible: note.visible,
            tie_start,
            tie_stop,
        });
    }

    crate::audit_only::phrase_boundary_evidence::capture(
        &source.input_label,
        SOURCE_CONTRACT,
        canonical_ticks_per_quarter,
        xml,
        projection,
    );
}

fn phrase_canonical_measure_timing(
    score: &crate::model::Score,
) -> Option<PhraseCanonicalMeasureTiming> {
    let mut scale = 1_i64;
    let mut measure_count = 0usize;
    for part in &score.parts {
        for measure in &part.measures {
            if measure.divisions <= 0 || measure.beats <= 0 || measure.beat_type <= 0 {
                return None;
            }
            scale = phrase_checked_lcm(scale, measure.divisions)?;
            scale = phrase_checked_lcm(scale, measure.expected.denominator.max(1))?;
            measure_count = measure_count.saturating_add(1);
        }
    }
    if measure_count == 0 {
        return None;
    }

    let mut timing = std::collections::BTreeMap::new();
    for part in &score.parts {
        let mut absolute_start = 0_i128;
        for (index, measure) in part.measures.iter().enumerate() {
            let divisions = measure.divisions.max(1);
            if measure.cursor_min != 0 || measure.expected.compare_to_actual(measure.cursor_max) != 0 {
                return None;
            }
            let expected_numerator = i128::from(measure.expected.numerator)
                .checked_mul(i128::from(scale))?;
            let expected_denominator = i128::from(measure.expected.denominator.max(1))
                .checked_mul(i128::from(divisions))?;
            if expected_denominator <= 0 || expected_numerator % expected_denominator != 0 {
                return None;
            }
            let expected_ticks = expected_numerator / expected_denominator;
            if expected_ticks <= 0 {
                return None;
            }
            let beat_numerator = i128::from(scale).checked_mul(4)?;
            let beat_denominator = i128::from(measure.beat_type.max(1));
            if beat_numerator % beat_denominator != 0 {
                return None;
            }
            let canonical_beat_ticks = beat_numerator / beat_denominator;
            if canonical_beat_ticks <= 0 {
                return None;
            }
            timing.insert(
                (part.id.clone(), index.saturating_add(1)),
                PhraseMeasureTiming {
                    absolute_start,
                    divisions,
                    beats: measure.beats,
                    beat_type: measure.beat_type,
                    canonical_beat_ticks,
                },
            );
            absolute_start = absolute_start.checked_add(expected_ticks)?;
        }
    }
    Some((scale, timing))
}

fn phrase_checked_lcm(left: i64, right: i64) -> Option<i64> {
    let left = left.max(1);
    let right = right.max(1);
    let gcd = phrase_gcd(left, right);
    left.checked_div(gcd)?.checked_mul(right)
}

fn phrase_gcd(mut left: i64, mut right: i64) -> i64 {
    left = left.abs().max(1);
    right = right.abs().max(1);
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

fn phrase_canonical_scaled_value(value: i64, scale: i64, divisions: i64) -> Option<i128> {
    let divisions = divisions.max(1);
    if scale % divisions != 0 {
        return None;
    }
    i128::from(value).checked_mul(i128::from(scale / divisions))
}

fn phrase_measure_contexts(xml: &str) -> Vec<PhraseMeasureContext> {
    let mut contexts = Vec::new();
    let mut part_cursor = 0usize;
    let mut part_ordinal = 0usize;
    while let Some(part_start) = phrase_find_open_tag(xml, part_cursor, "part") {
        let Some(part_open_end_rel) = xml[part_start..].find('>') else {
            break;
        };
        let part_open_end = part_start + part_open_end_rel;
        let Some(part_close_rel) = xml[part_open_end + 1..].find("</part>") else {
            break;
        };
        let part_end = part_open_end + 1 + part_close_rel + "</part>".len();
        part_ordinal = part_ordinal.saturating_add(1);
        let part_open_tag = &xml[part_start..=part_open_end];
        let part = parse_attr_value(part_open_tag, "id")
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| format!("P{part_ordinal}"));
        let mut measure_cursor = part_open_end + 1;
        let mut measure_ordinal = 0usize;
        while let Some(measure_start) = phrase_find_open_tag(xml, measure_cursor, "measure") {
            if measure_start >= part_end {
                break;
            }
            let Some(measure_open_end_rel) = xml[measure_start..part_end].find('>') else {
                break;
            };
            let measure_open_end = measure_start + measure_open_end_rel;
            let Some(measure_close_rel) = xml[measure_open_end + 1..part_end].find("</measure>") else {
                break;
            };
            let measure_end = measure_open_end + 1 + measure_close_rel + "</measure>".len();
            measure_ordinal = measure_ordinal.saturating_add(1);
            contexts.push(PhraseMeasureContext {
                part: part.clone(),
                measure_ordinal,
                start: measure_start,
                end: measure_end,
            });
            measure_cursor = measure_end;
        }
        part_cursor = part_end;
    }
    contexts
}

fn phrase_find_open_tag(xml: &str, mut cursor: usize, name: &str) -> Option<usize> {
    let needle = format!("<{name}");
    while let Some(relative) = xml.get(cursor..)?.find(&needle) {
        let start = cursor + relative;
        let after = start + needle.len();
        let accepted = xml
            .as_bytes()
            .get(after)
            .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b'>');
        if accepted {
            return Some(start);
        }
        cursor = after;
    }
    None
}

fn phrase_note_has_tie_type(note_block: &str, tie_type: &str) -> bool {
    ["tie", "tied"].into_iter().any(|tag_name| {
        let mut cursor = 0usize;
        while let Some(start) = phrase_find_open_tag(note_block, cursor, tag_name) {
            let Some(open_end_rel) = note_block[start..].find('>') else {
                return false;
            };
            let open_end = start + open_end_rel;
            if parse_attr_value(&note_block[start..=open_end], "type").as_deref() == Some(tie_type) {
                return true;
            }
            cursor = open_end.saturating_add(1);
        }
        false
    })
}

// r1291: writes the wall time of one pipeline stage to stderr (console log only) and returns a
// fresh clock; it changes no stage, result or report.
fn pipeline_stage_timing(project: &str, stage: &str, started: std::time::Instant) -> std::time::Instant {
    eprintln!("ETUSCHLOPF: PhaseTiming: project={project}; stage={stage}; ms={}", started.elapsed().as_millis());
    std::time::Instant::now()
}

// Stable path anchor for the productive pipeline inventory. The executable flow lives in
// `real_main`; phase and support implementations stay private through lexical `include!` files.
fn real_main() -> anyhow::Result<i32> {
    crate::audit_only::phrase_boundary_evidence::clear();
    master_state_identity::reset_master_lineage_measurement();
    master_state_identity::reset_render_lineage_measurement();
    let cli = Cli::normalized_from_env();
    let _ =
        crate::audit_only::writer_trace::cleanup_clef_ab_probe_artifacts(&std::env::current_dir()?);
    if cli.all {
        return run_all_batch_orchestrator(&cli.corpus_out, &cli.jobs);
    }
    if cli.corpus_emergence {
        return write_corpus_emergence_report(&cli.corpus_out);
    }

    let project_label = cli
        .input
        .as_ref()
        .and_then(|input| input.file_name())
        .map(|label| label.to_string_lossy().into_owned());
    // r1291: wall time per project stage on stderr only (console log).
    let timing_label = project_label.clone().unwrap_or_default();
    let mut stage_clock = std::time::Instant::now();
    let mut runtime_observation = observation::SourceStateOwner::project(project_label);
    let runtime_observation_transport =
        observation_transport::SnapshotDatagramPublisher::local(&runtime_observation);
    etuschlopf::console_status_record::emit_project_stage(&runtime_observation);
    macro_rules! observed_project_try {
        ($expression:expr) => {
            match $expression {
                Ok(value) => value,
                Err(error) => {
                    runtime_observation.mark_failed();
                    runtime_observation_transport.publish_current(&runtime_observation);
                    return Err(error);
                }
            }
        };
    }

    let ProjectRunPreparationStageOutcome {
        source,
        writer_trace_session: _writer_trace_session,
        reference,
        options,
        parallel,
        midi_path,
        midi_projection,
        source_event_causal_projection,
    } = observed_project_try!(run_project_run_preparation_stage(
        ProjectRunPreparationStageInputs { cli: &cli }
    ));
    stage_clock = pipeline_stage_timing(&timing_label, "ProjectRunPreparation", stage_clock);

    runtime_observation.enter_project_stage(observation::ProjectStage::ArrangementSolver);
    runtime_observation_transport.publish_current(&runtime_observation);
    etuschlopf::console_status_record::emit_project_stage(&runtime_observation);
    let ArrangementSolverStageOutcome {
        best,
        next_iteration,
    } = observed_project_try!(run_arrangement_solver_stage(ArrangementSolverStageInputs {
        cli: &cli,
        source: &source,
        reference: &reference,
        options: &options,
        parallel: &parallel,
        midi_path: &midi_path,
    }));
    stage_clock = pipeline_stage_timing(&timing_label, "ArrangementSolver", stage_clock);

    runtime_observation.enter_project_stage(observation::ProjectStage::MasterNotationAndSalience);
    runtime_observation_transport.publish_current(&runtime_observation);
    etuschlopf::console_status_record::emit_project_stage(&runtime_observation);
    let MasterNotationAndSalienceStageOutcome { best } =
        observed_project_try!(run_master_notation_and_salience_stage(
            MasterNotationAndSalienceStageInputs {
                best,
                next_iteration,
                options: &options,
            }
        ));
    stage_clock = pipeline_stage_timing(&timing_label, "MasterNotationAndSalience", stage_clock);

    runtime_observation.enter_project_stage(observation::ProjectStage::InitialRenderTransaction);
    runtime_observation_transport.publish_current(&runtime_observation);
    etuschlopf::console_status_record::emit_project_stage(&runtime_observation);
    let InitialRenderTransactionStageOutcome {
        best,
        transaction: initial_render,
        mut render_state,
    } = observed_project_try!(run_initial_render_transaction_stage(
        InitialRenderTransactionStageInputs {
            best,
            source: &source,
            reference: &reference,
        }
    ));
    stage_clock = pipeline_stage_timing(&timing_label, "InitialRenderTransaction", stage_clock);


    runtime_observation.enter_project_stage(observation::ProjectStage::MetricBeamAndFinalNotation);
    runtime_observation_transport.publish_current(&runtime_observation);
    etuschlopf::console_status_record::emit_project_stage(&runtime_observation);
    let metric_beam_and_final_notation = observed_project_try!(
        run_metric_beam_and_final_notation_stage(MetricBeamAndFinalNotationStageInputs {
            initial_render: &initial_render,
            render_state: &mut render_state,
            source: &source,
            reference,
            no_pdf: cli.no_pdf,
            final_blockers: best.final_result.summary.blockers,
            horizontal_layout_seed: &best.horizontal_layout_seed,
        })
    );
    stage_clock = pipeline_stage_timing(&timing_label, "MetricBeamAndFinalNotation", stage_clock);

    runtime_observation.enter_project_stage(observation::ProjectStage::ReleaseReportAndExit);
    runtime_observation_transport.publish_current(&runtime_observation);
    etuschlopf::console_status_record::emit_project_stage(&runtime_observation);
    let exit_code = observed_project_try!(run_release_report_and_exit_stage(
        ReleaseReportAndExitStageInputs {
            cli: &cli,
            source: &source,
            best,
            initial_render,
            render_state,
            metric_beam_and_final_notation,
            midi_projection,
            source_event_causal_projection,
        }
    ));
    let _ = pipeline_stage_timing(&timing_label, "ReleaseReportAndExit", stage_clock);
    if exit_code == 0 {
        runtime_observation.mark_completed();
    } else {
        runtime_observation.mark_failed();
    }
    runtime_observation_transport.publish_current(&runtime_observation);
    Ok(exit_code)
}

include!("pipeline_orchestrator_release_report_exit_stage.rs");

include!("pipeline_orchestrator_project_setup_stages.rs");

include!("pipeline_orchestrator_initial_render_stage.rs");

include!("pipeline_orchestrator_metric_final_notation_stage.rs");

include!("pipeline_orchestrator_final_notation_support.rs");

include!("pipeline_orchestrator_rest_articulation_render_stage.rs");

include!("pipeline_orchestrator_release_support.rs");

include!("pipeline_orchestrator_arrangement_solver_stage.rs");

include!("pipeline_orchestrator_clef_layout_support.rs");

#[cfg(test)]
mod phrase_boundary_evidence_adapter_tests {
    use super::{
        phrase_canonical_measure_timing, phrase_canonical_scaled_value, phrase_find_open_tag,
        phrase_measure_contexts, phrase_note_has_tie_type,
    };

    #[test]
    fn part_locator_ignores_part_list_and_score_part_tags() {
        let xml = "<score-partwise><part-list><score-part id=\"P1\"/></part-list><part id=\"P1\"><measure number=\"\
1\"></measure></part></score-partwise>";
        let part_start = phrase_find_open_tag(xml, 0, "part").expect("score part");
        assert_eq!(&xml[part_start..part_start + 5], "<part");
        assert!(xml[part_start..].starts_with("<part id=\"P1\""));
    }

    #[test]
    fn measure_contexts_keep_part_identity_and_reset_measure_ordinal_per_part() {
        let xml = r#"<score-partwise><part-list></part-list><part id="P1"><measure number="A"><note></note></measure><measure number="B"><note></note></measure></part><part id="P2"><measure number="X"><note></note></measure></part></score-partwise>"#;
        let contexts = phrase_measure_contexts(xml);
        assert_eq!(contexts.len(), 3);
        assert_eq!(contexts[0].part, "P1");
        assert_eq!(contexts[0].measure_ordinal, 1);
        assert_eq!(contexts[1].part, "P1");
        assert_eq!(contexts[1].measure_ordinal, 2);
        assert_eq!(contexts[2].part, "P2");
        assert_eq!(contexts[2].measure_ordinal, 1);
    }

    #[test]
    fn tie_type_reader_distinguishes_start_stop_and_tied_notation() {
        let start = r#"<note><tie type="start"/><notations><tied type="start"/></notations></note>"#;
        let stop = r#"<note><tie type='stop'/><notations><tied type='stop'/></notations></note>"#;
        assert!(phrase_note_has_tie_type(start, "start"));
        assert!(!phrase_note_has_tie_type(start, "stop"));
        assert!(phrase_note_has_tie_type(stop, "stop"));
        assert!(!phrase_note_has_tie_type(stop, "start"));
    }

    #[test]
    fn canonical_measure_timing_reuses_parsed_metric_owner_across_division_change() {
        let xml = "<score-partwise><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><rest/><duration>384</duration><voice>1</voice><staff>1</staff></note></measure><measure number=\"\
2\"><attributes><divisions>48</divisions><time><beats>3</beats><beat-type>4</beat-type></time></attributes><note><rest/><duration>144</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        let score = crate::musicxml::parse_musicxml(xml).expect("canonical score");
        let (scale, timing) = phrase_canonical_measure_timing(&score).expect("metric timing");
        assert_eq!(scale, 96);
        assert_eq!(timing[&("P1".to_string(), 1)].absolute_start, 0);
        assert_eq!(timing[&("P1".to_string(), 2)].absolute_start, 384);
        assert_eq!(timing[&("P1".to_string(), 1)].canonical_beat_ticks, 96);
        assert_eq!(timing[&("P1".to_string(), 2)].canonical_beat_ticks, 96);
        assert_eq!(timing[&("P1".to_string(), 2)].beats, 3);
        assert_eq!(timing[&("P1".to_string(), 2)].beat_type, 4);
        assert_eq!(
            phrase_canonical_scaled_value(
                24,
                scale,
                timing[&("P1".to_string(), 2)].divisions,
            ),
            Some(48),
        );
    }

    #[test]
    fn canonical_measure_timing_fails_closed_for_incomplete_measure_span() {
        let xml = "<score-partwise><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><rest/><duration>192</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        let score = crate::musicxml::parse_musicxml(xml).expect("canonical score");
        assert!(phrase_canonical_measure_timing(&score).is_none());
    }

}
