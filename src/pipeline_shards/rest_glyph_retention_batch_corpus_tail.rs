fn compact_current_report_preamble_v0101536(current_preamble: &str) -> String {
    let mut out = current_preamble.trim_end().to_string();
    out.push('\n');
    out.push_static_section_header_v795("\nAKTIVER-REPORT-VERTRAG", "-----------------------");
    out.push_static_named_report_line_v766(
        "Entscheidungsautoritaet",
        " Der formale Ergebniskopf bleibt die einzige Freigabeautoritaet. Spaetere typisierte Projektionen liefern aktuelle Diagnose- und Writerfakten; das Fehlen pensionierter historischer Sektionen erzeugt weder Blocker noch Warnung.",
    );
    out.push_static_named_report_line_v766(
        "Projektion",
        " Der Standardbericht wird direkt aus dem Ergebnispraembel und den deklarierten aktuellen Projektionen aufgebaut. Ein historischer Vollreport wird weder erzeugt noch verworfen oder als Gate-Datenbank geparst.",
    );
    out.push_str(&launcher_version_contract_section());
    out.push('\n');
    out.push_static_named_report_line_v766(
        "Standardausgaben",
        " <Projekt>.musicxml, <Projekt>.render.musicxml, <Projekt>.pdf, <Projekt>.result.txt. MXL bleibt kein Standardoutput.",
    );
    out
}

#[derive(Debug, Clone)]
struct BatchProjectPair {
    project: String,
    wav_path: PathBuf,
    midi_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CorpusExecutionPlan {
    requested_jobs: String,
    logical_threads: usize,
    solver_worker_budget: usize,
    concurrent_projects: usize,
    solver_workers_per_project: usize,
    candidate_budget_workers_per_project: usize,
    renderer_slots: usize,
    reserved_threads: usize,
}

impl CorpusExecutionPlan {
    fn bounded(project_count: usize, parallel: &ParallelPlan) -> Self {
        // Most project stages run on one thread; only the arrangement solver
        // fans out. A third project process is opened once each process still
        // receives six solver workers, so serial stages overlap instead of
        // leaving most logical threads idle. Worker counts change only
        // throughput: per-project budgets come from the separate budget basis
        // and parallel results are reassembled in index order.
        let concurrent_projects = match parallel.workers {
            0..=3 => 1,
            4..=17 => 2,
            _ => 3,
        }
        .min(project_count.max(1));
        let solver_workers_per_project = (parallel.workers / concurrent_projects).max(1);
        let assigned_solver_workers = solver_workers_per_project * concurrent_projects;
        Self {
            requested_jobs: parallel.requested.clone(),
            logical_threads: parallel.logical_threads,
            solver_worker_budget: parallel.workers,
            concurrent_projects,
            solver_workers_per_project,
            candidate_budget_workers_per_project: parallel.candidate_budget_workers,
            // One project process owns at most one renderer process.
            renderer_slots: concurrent_projects,
            reserved_threads: parallel.logical_threads.saturating_sub(assigned_solver_workers),
        }
    }

    fn report_line(&self) -> String {
        format!(
            "KorpusExecutionPlan: RequestedJobs {} | LogicalThreads {} | SolverWorkerBudget {} | ConcurrentProjects {} | SolverWorkersPerProject {} | CandidateBudgetWorkersPerProject {} | RendererSlots {} | ReservedThreads {}",
            self.requested_jobs,
            self.logical_threads,
            self.solver_worker_budget,
            self.concurrent_projects,
            self.solver_workers_per_project,
            self.candidate_budget_workers_per_project,
            self.renderer_slots,
            self.reserved_threads,
        )
    }
}

#[derive(Debug, Clone)]
struct BatchProjectTask {
    project_index: usize,
    project_count: usize,
    project: String,
    wav_path: PathBuf,
    midi_path: PathBuf,
    expected: BatchExpectedOutputs,
}

impl BatchProjectTask {
    fn from_pair(project_index: usize, project_count: usize, pair: BatchProjectPair) -> Self {
        let expected = expected_batch_outputs(&pair.project);
        Self {
            project_index,
            project_count,
            project: pair.project,
            wav_path: pair.wav_path,
            midi_path: pair.midi_path,
            expected,
        }
    }

    fn progress(&self, global_total: usize) -> BatchProjectProgress<'_> {
        BatchProjectProgress {
            project_index: self.project_index,
            project_count: self.project_count,
            global_total,
            file_total: 6,
            project: &self.project,
        }
    }
}

// Scheduling only: byte length is a coarse cost hint, never a musical input.
// Read every hint before moving any task; an unavailable hint keeps the old order.
fn prioritize_batch_tasks_by_midi_size(
    tasks: &mut std::collections::VecDeque<BatchProjectTask>,
    concurrent_projects: usize,
) -> bool {
    if concurrent_projects < 3 || tasks.len() <= 1 {
        return false;
    }
    let sizes = tasks
        .iter()
        .map(|task| {
            std::fs::metadata(&task.midi_path)
                .ok()
                .filter(|metadata| metadata.is_file() && metadata.len() > 0)
                .map(|metadata| metadata.len())
        })
        .collect::<Option<Vec<_>>>();
    let Some(sizes) = sizes else {
        return false;
    };
    let mut sized_tasks = sizes.into_iter().zip(tasks.drain(..)).collect::<Vec<_>>();
    sized_tasks.sort_by(|(left_size, left), (right_size, right)| {
        right_size.cmp(left_size).then_with(|| left.project_index.cmp(&right.project_index))
    });
    tasks.extend(sized_tasks.into_iter().map(|(_, task)| task));
    true
}

#[cfg(test)]
mod corpus_start_order_tests {
    use super::{BatchProjectPair, BatchProjectTask, prioritize_batch_tasks_by_midi_size};
    use std::{collections::{BTreeMap, VecDeque}, path::PathBuf};

    struct Fixture {
        root: PathBuf,
        tasks: VecDeque<BatchProjectTask>,
    }

    impl Fixture {
        fn new(sizes: &[usize]) -> Self {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let root = std::env::temp_dir().join(format!(
                "etuschlopf-start-order-{}-{}",
                std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            ));
            std::fs::create_dir(&root).unwrap();
            let tasks = sizes.iter().enumerate().map(|(index, size)| {
                // Names deliberately run against the assigned original indices.
                let project = format!("Synthetic Ü project {}", sizes.len() - index);
                let midi_path = root.join(format!("{project}.mid"));
                let wav_path = root.join(format!("{project}.wav"));
                std::fs::write(&midi_path, vec![0; *size]).unwrap();
                BatchProjectTask::from_pair(index + 1, sizes.len(), BatchProjectPair {
                    project, midi_path, wav_path,
                })
            }).collect();
            Self { root, tasks }
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn identities(tasks: &VecDeque<BatchProjectTask>) -> BTreeMap<usize, String> {
        tasks.iter().map(|task| (task.project_index, format!("{task:?}"))).collect()
    }

    #[test]
    fn three_slot_queue_transports_every_task_unchanged_and_breaks_size_ties_by_original_index() {
        let mut fixture = Fixture::new(&[4, 20, 20, 8]);
        let before = identities(&fixture.tasks);
        assert!(prioritize_batch_tasks_by_midi_size(&mut fixture.tasks, 3));
        assert_eq!(fixture.tasks.iter().map(|task| task.project_index).collect::<Vec<_>>(), [2, 3, 4, 1]);
        assert_eq!(fixture.tasks.len(), before.len());
        assert_eq!(identities(&fixture.tasks), before);
        let once = format!("{:?}", fixture.tasks);
        assert!(prioritize_batch_tasks_by_midi_size(&mut fixture.tasks, 3));
        assert_eq!(format!("{:?}", fixture.tasks), once);
    }

    #[test]
    fn one_or_two_slots_keep_the_original_order() {
        for slots in [1, 2] {
            let mut fixture = Fixture::new(&[4, 20, 8]);
            let before = format!("{:?}", fixture.tasks);
            assert!(!prioritize_batch_tasks_by_midi_size(&mut fixture.tasks, slots));
            assert_eq!(format!("{:?}", fixture.tasks), before);
        }
    }

    #[test]
    fn any_unavailable_nonfile_or_empty_hint_keeps_every_task_in_original_order() {
        for case in ["missing", "directory", "empty"] {
            let mut fixture = Fixture::new(&[4, 20, 8]);
            let path = &fixture.tasks[2].midi_path;
            std::fs::remove_file(path).unwrap();
            match case {
                "directory" => std::fs::create_dir(path).unwrap(),
                "empty" => std::fs::write(path, []).unwrap(),
                _ => {},
            }
            let before = format!("{:?}", fixture.tasks);
            assert!(!prioritize_batch_tasks_by_midi_size(&mut fixture.tasks, 3));
            assert_eq!(format!("{:?}", fixture.tasks), before);
        }
    }
}

struct RunningBatchProject {
    task: BatchProjectTask,
    child: std::process::Child,
    page_count_preference_shadow_capture:
        crate::audit_only::page_count_preference_projection::BatchChildPreferenceCapture,
    started: Instant,
    frame: usize,
    observation_context: observation::ObservationSessionContext,
    project_log: crate::audit_only::batch_project_log::BatchProjectLog,
}

fn run_all_batch_orchestrator(corpus_out: &Path, jobs: &str) -> anyhow::Result<i32> {
    let mut runtime_observation = observation::SourceStateOwner::corpus(0);
    let runtime_observation_transport =
        observation_transport::SnapshotDatagramPublisher::local(&runtime_observation);
    macro_rules! observed_corpus_try {
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
    let pairs = observed_corpus_try!(scan_wav_midi_pairs(Path::new(".")));
    if pairs.is_empty() {
        runtime_observation.mark_failed();
        runtime_observation_transport.publish_current(&runtime_observation);
        println!("ETUSCHLOPF-BATCH: keine WAV/MIDI-Paare gefunden. Erwartet: <Projekt>.wav + <Projekt>.mid");
        return Ok(1);
    }

    let batch_started = Instant::now();
    let project_count = pairs.len();
    let parallel = observed_corpus_try!(resolve_parallel_plan(jobs, None));
    let execution_plan = CorpusExecutionPlan::bounded(project_count, &parallel);
    runtime_observation.enter_corpus_stage(
        observation::CorpusStage::PreparingProjects,
        observation::SourceProgress {
            completed: 0,
            total: project_count,
            active: 0,
        },
    );
    runtime_observation_transport.publish_current(&runtime_observation);
    let run_started_unix_seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let corpus_run_id = format!(
        "corpus-{}-{}-{}",
        env!("CARGO_PKG_VERSION"),
        run_started_unix_seconds,
        std::process::id()
    );
    let manifest_path = PathBuf::from(CORPUS_RUN_MANIFEST_FILE);
    let mut corpus_manifest = CorpusRunManifest::started(
        env!("CARGO_PKG_VERSION"),
        corpus_run_id.clone(),
        run_started_unix_seconds,
        pairs.iter().map(|pair| pair.project.clone()).collect(),
    );
    observed_corpus_try!(corpus_manifest
        .write_atomic(&manifest_path)
        .with_context(|| format!("Konnte Korpus-Laufmanifest nicht initialisieren: {}", manifest_path.display())));

    println!("ETUSCHLOPF-BATCH v{}", env!("CARGO_PKG_VERSION"));
    println!("Modus: --all | Projekte: {} | pro Projekt eigene .musicxml/.render.musicxml/.pdf/.result.txt | Korpus-Emergenz am Ende", pairs.len());
    println!("{}", execution_plan.report_line());

    let phases = [
        "Paar prüfen",
        "alte Ausgaben bereinigen",
        "Einzelpipeline starten",
        "Ausgaben prüfen",
        "Result sichern",
        "Korpus-Beobachtung sammeln",
    ];
    let total_units = project_count * phases.len() + 1;
    let mut done_units = 0usize;
    let mut completed_projects = 0usize;
    let mut failed_projects = 0usize;
    let mut missing_outputs = 0usize;
    let mut project_runtime_sum = Duration::from_secs(0);
    let mut longest_project_runtime = Duration::from_secs(0);
    let mut project_errors: Vec<String> = Vec::new();
    let mut observations = Vec::new();
    let current_exe = observed_corpus_try!(
        std::env::current_exe().context("Konnte aktuellen etuschlopf.exe-Pfad nicht bestimmen")
    );
    let mut pending = pairs
        .into_iter()
        .enumerate()
        .map(|(index, pair)| BatchProjectTask::from_pair(index + 1, project_count, pair))
        .collect::<std::collections::VecDeque<_>>();
    let prioritized = prioritize_batch_tasks_by_midi_size(
        &mut pending,
        execution_plan.concurrent_projects,
    );
    println!(
        "KorpusStartOrder: Basis {} | Projects {}",
        if prioritized { "midi_bytes_descending" } else { "natural" },
        pending.iter().map(|task| task.project.as_str()).collect::<Vec<_>>().join(" -> "),
    );
    let mut running: Vec<RunningBatchProject> = Vec::new();

    println!("ETUSCHLOPF-BATCH: {} Projektpaar(e) erkannt. DATEI steht oben, GESAMT bleibt immer als bewegter unterer Balken sichtbar.", pending.len());

    while !pending.is_empty() || !running.is_empty() {
        while running.len() < execution_plan.concurrent_projects {
            let Some(task) = pending.pop_front() else {
                break;
            };
            let project_progress = task.progress(total_units);

            batch_progress(project_progress.snapshot(done_units, 0, completed_projects, phases[0]));
            done_units += 1;

            batch_progress(project_progress.snapshot(done_units, 1, completed_projects, phases[1]));
            observed_corpus_try!(purge_batch_project_outputs(&task.project));
            done_units += 1;

            batch_progress(project_progress.snapshot(done_units, 2, completed_projects, phases[2]));
            let child_observation = runtime_observation.context().child_project(task.project_index);
            // Each project's diagnostic stderr goes to its own log and is appended to the
            // console log as one block after the project ends, so parallel projects can
            // no longer interleave lines. Without a log file the previous inheritance stays.
            let project_log = crate::audit_only::batch_project_log::BatchProjectLog::open(&task.project);
            let child_stderr = project_log.child_stderr();
            let child = std::process::Command::new(&current_exe)
                .arg(&task.project)
                .arg("--jobs")
                .arg(execution_plan.solver_workers_per_project.to_string())
                .args(crate::audit_only::writer_trace::batch_child_trace_arguments())
                .env(
                    observation::SESSION_ENV,
                    child_observation.session_id().as_str(),
                )
                .env(
                    observation::SOURCE_ENV,
                    child_observation.source_id().as_str(),
                )
                .env(
                    observation::PARENT_SOURCE_ENV,
                    child_observation
                        .parent_source_id()
                        .expect("corpus child source always has a parent")
                        .as_str(),
                )
                .stdout(
                    crate::audit_only::page_count_preference_projection::batch_child_stdout(),
                )
                .stderr(child_stderr)
                .spawn()
                .with_context(|| format!("Konnte Einzelpipeline nicht starten: {}", task.project));
            let mut child = observed_corpus_try!(child);
            let page_count_preference_shadow_capture = observed_corpus_try!(
                crate::audit_only::page_count_preference_projection::BatchChildPreferenceCapture::begin(
                    &mut child,
                )
            );
            done_units += 1;
            running.push(RunningBatchProject {
                task,
                child,
                page_count_preference_shadow_capture,
                started: Instant::now(),
                frame: BATCH_PROGRESS_FRAME_COUNTER.fetch_add(1, Ordering::Relaxed),
                observation_context: child_observation.clone(),
                project_log,
            });
            runtime_observation.enter_corpus_stage(
                observation::CorpusStage::RunningProjects,
                observation::SourceProgress {
                    completed: completed_projects,
                    total: project_count,
                    active: running.len(),
                },
            );
            runtime_observation_transport.publish_current(&runtime_observation);
        }

        let mut completed_in_poll = false;
        let mut index = 0usize;
        while index < running.len() {
            let project_name = running[index].task.project.clone();
            let status = observed_corpus_try!(running[index]
                .child
                .try_wait()
                .with_context(|| format!("Konnte Einzelpipeline-Status nicht lesen: {project_name}")));
            let Some(status) = status else {
                index += 1;
                continue;
            };

            completed_in_poll = true;
            let completed = running.remove(index);
            let page_count_preference_shadow_line = observed_corpus_try!(
                completed.page_count_preference_shadow_capture.finish()
            );
            runtime_observation.enter_corpus_stage(
                observation::CorpusStage::CollectingProjectResult,
                observation::SourceProgress {
                    completed: completed_projects,
                    total: project_count,
                    active: running.len(),
                },
            );
            runtime_observation_transport.publish_current(&runtime_observation);
            let project_elapsed = completed.started.elapsed();
            completed.project_log.append_to_console(&completed.task.project);
            eprintln!(
                "KorpusProjectTiming: Project {} | Milliseconds {} | PipelineExit {:?}",
                completed.task.project,
                project_elapsed.as_millis(),
                status.code(),
            );
            project_runtime_sum = project_runtime_sum.saturating_add(project_elapsed);
            longest_project_runtime = longest_project_runtime.max(project_elapsed);
            let project_progress = completed.task.progress(total_units);
            batch_progress_frame(
                project_progress.snapshot(done_units, 3, completed_projects, "Einzelpipeline fertig"),
                completed.frame,
                project_elapsed,
            );

            let mut project_failed = !status.success();
            if !status.success() {
                project_errors.push(format!(
                    "Projekt '{}' meldete Exit-Code {:?}",
                    completed.task.project,
                    status.code()
                ));
            }

            batch_progress(project_progress.snapshot(done_units, 3, completed_projects, phases[3]));
            let mut missing_for_project = Vec::new();
            for path in [
                &completed.task.expected.musicxml,
                &completed.task.expected.render_musicxml,
                &completed.task.expected.pdf,
                &completed.task.expected.result,
            ] {
                if !path.exists() {
                    missing_for_project.push(path.clone());
                    missing_outputs += 1;
                }
            }
            done_units += 1;

            batch_progress(project_progress.snapshot(done_units, 4, completed_projects, phases[4]));
            if !completed.task.expected.result.exists() {
                observed_corpus_try!(write_batch_failure_result(
                    &completed.task.project,
                    &completed.task.wav_path,
                    &completed.task.midi_path,
                    &completed.task.expected,
                    status.code(),
                    &missing_for_project,
                    page_count_preference_shadow_line.as_deref(),
                ));
                project_failed = true;
            } else {
                observed_corpus_try!(append_batch_missing_output_note(
                    &completed.task.expected.result,
                    &missing_for_project,
                    page_count_preference_shadow_line.as_deref(),
                ));
            }
            done_units += 1;

            batch_progress(project_progress.snapshot(done_units, 5, completed_projects, phases[5]));
            if let Ok(text) = std::fs::read_to_string(&completed.task.expected.result) {
                observations.push(observe_project_result(completed.task.expected.result.clone(), &text));
                corpus_manifest.upsert_project(corpus_run_project_evidence(
                    &completed.task.project,
                    &completed.task.expected.result,
                    &text,
                    status.code(),
                    missing_for_project.is_empty(),
                ));
                observed_corpus_try!(corpus_manifest
                    .write_atomic(&manifest_path)
                    .with_context(|| format!("Konnte Korpus-Laufmanifest nach Projekt '{}' nicht schreiben", completed.task.project)));
            }
            done_units += 1;
            completed_projects += 1;
            if project_failed {
                failed_projects += 1;
            }
            etuschlopf::console_status_record::emit_project_finished(
                &completed.observation_context,
                &completed.task.project,
                if project_failed || !missing_for_project.is_empty() {
                    etuschlopf::console_status_record::ProjectOutcome::Failed
                } else {
                    etuschlopf::console_status_record::ProjectOutcome::Ok
                },
                u64::try_from(project_elapsed.as_millis()).unwrap_or(u64::MAX),
            );
            if !pending.is_empty() || !running.is_empty() {
                runtime_observation.enter_corpus_stage(
                    observation::CorpusStage::RunningProjects,
                    observation::SourceProgress {
                        completed: completed_projects,
                        total: project_count,
                        active: running.len(),
                    },
                );
                runtime_observation_transport.publish_current(&runtime_observation);
            }
        }

        if !completed_in_poll {
            if let Some(active) = running.first_mut() {
                active.frame = active.frame.wrapping_add(1);
                batch_progress_frame(
                    active
                        .task
                        .progress(total_units)
                        .snapshot(done_units, 2, completed_projects, "Einzelpipelines laufen"),
                    active.frame,
                    active.started.elapsed(),
                );
            }
            thread::sleep(Duration::from_millis(120));
        }
    }

    runtime_observation.enter_corpus_stage(
        observation::CorpusStage::FinalizingCorpusReport,
        observation::SourceProgress {
            completed: completed_projects,
            total: project_count,
            active: 0,
        },
    );
    runtime_observation_transport.publish_current(&runtime_observation);
    let corpus_progress = BatchProjectProgress {
        project_index: completed_projects,
        project_count: completed_projects,
        global_total: total_units,
        file_total: phases.len(),
        project: "Korpus",
    };
    batch_progress(corpus_progress.snapshot(done_units, phases.len(), completed_projects, "corpus_emergence.result.txt schreiben"));
    let mut corpus_exit = observed_corpus_try!(write_corpus_emergence_report_for_run(
        corpus_out,
        Some(&corpus_run_id),
    ));
    if corpus_exit == 0 {
        let report_evidence = observed_corpus_try!(corpus_run_file_evidence(corpus_out)
            .with_context(|| format!("Konnte Korpusbericht nicht für Manifest binden: {}", corpus_out.display())));
        let completed_unix_seconds = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(run_started_unix_seconds);
        corpus_manifest.finalize(completed_unix_seconds, report_evidence);
        observed_corpus_try!(corpus_manifest
            .write_atomic(&manifest_path)
            .with_context(|| format!("Konnte Korpus-Laufmanifest nicht finalisieren: {}", manifest_path.display())));
        let validation = validate_corpus_run_manifest(Path::new("."), env!("CARGO_PKG_VERSION"));
        if !validation.valid {
            eprintln!("CorpusRunManifest: SelfCheck fehlgeschlagen: {}", validation.reason);
            corpus_exit = 1;
        }
    }
    done_units += 1;

    batch_finish_live_progress();
    println!("\nETUSCHLOPF-BATCH-FERTIG");
    if !project_errors.is_empty() {
        println!("Projektmeldungen:");
        for msg in &project_errors {
            println!("- {msg}");
        }
    }
    println!("Projekte: {} | Beobachtungen: {} | Fehler: {} | fehlende Ausgabedateien: {} | Korpusbericht: {} | Korpus-Exit: {}", completed_projects, observations.len(), failed_projects, missing_outputs, corpus_out.display(), corpus_exit);
    println!(
        "KorpusExecutionTiming: Wall {} | ProjectRuntimeSum {} | LongestProject {} | ConcurrentProjects {}",
        format_duration_short(batch_started.elapsed()),
        format_duration_short(project_runtime_sum),
        format_duration_short(longest_project_runtime),
        execution_plan.concurrent_projects,
    );
    println!("{}", batch_bar_line("GESAMT", done_units, total_units, 28));
    let final_exit = if failed_projects > 0 || missing_outputs > 0 || corpus_exit != 0 {
        1
    } else {
        0
    };
    if final_exit == 0 {
        runtime_observation.mark_completed();
    } else {
        runtime_observation.mark_failed();
    }
    runtime_observation_transport.publish_current(&runtime_observation);
    Ok(final_exit)
}

#[cfg(test)]
mod corpus_execution_plan_tests {
    use super::{CorpusExecutionPlan, ParallelPlan};

    fn parallel_plan(requested: &str, workers: usize, logical_threads: usize) -> ParallelPlan {
        ParallelPlan {
            requested: requested.to_string(),
            workers,
            candidate_budget_workers: workers,
            logical_threads,
            human_potential_budget: 0,
            human_potential_max_moves: 0,
        }
    }

    #[test]
    fn auto_budget_on_twenty_threads_runs_three_projects_with_six_workers_each() {
        let plan = CorpusExecutionPlan::bounded(10, &parallel_plan("auto", 19, 20));
        assert_eq!(plan.concurrent_projects, 3);
        assert_eq!(plan.solver_workers_per_project, 6);
        assert_eq!(plan.candidate_budget_workers_per_project, 19);
        assert_eq!(plan.reserved_threads, 2);
    }

    #[test]
    fn medium_worker_budget_runs_two_projects() {
        let plan = CorpusExecutionPlan::bounded(10, &parallel_plan("11", 11, 12));
        assert_eq!(plan.concurrent_projects, 2);
        assert_eq!(plan.solver_workers_per_project, 5);
        assert_eq!(plan.reserved_threads, 2);
    }

    #[test]
    fn two_projects_never_open_a_third_slot() {
        let plan = CorpusExecutionPlan::bounded(2, &parallel_plan("auto", 19, 20));
        assert_eq!(plan.concurrent_projects, 2);
        assert_eq!(plan.solver_workers_per_project, 9);
    }

    #[test]
    fn small_worker_budget_stays_single_project() {
        let plan = CorpusExecutionPlan::bounded(10, &parallel_plan("3", 3, 20));
        assert_eq!(plan.concurrent_projects, 1);
        assert_eq!(plan.solver_workers_per_project, 3);
    }

    #[test]
    fn single_project_never_opens_a_second_slot() {
        let plan = CorpusExecutionPlan::bounded(1, &parallel_plan("max", 20, 20));
        assert_eq!(plan.concurrent_projects, 1);
        assert_eq!(plan.solver_workers_per_project, 20);
    }
}

#[derive(Debug, Clone)]
struct BatchExpectedOutputs {
    musicxml: PathBuf,
    render_musicxml: PathBuf,
    pdf: PathBuf,
    result: PathBuf,
}

fn expected_batch_outputs(project: &str) -> BatchExpectedOutputs {
    BatchExpectedOutputs {
        musicxml: PathBuf::from(format!("{project}.musicxml")),
        render_musicxml: PathBuf::from(format!("{project}.render.musicxml")),
        pdf: PathBuf::from(format!("{project}.pdf")),
        result: PathBuf::from(format!("{project}.result.txt")),
    }
}

fn scan_wav_midi_pairs(dir: &Path) -> anyhow::Result<Vec<BatchProjectPair>> {
    let mut pairs = Vec::new();
    for entry in std::fs::read_dir(dir).with_context(|| format!("Konnte Ordner nicht lesen: {}", dir.display()))? {
        let entry = entry.context("Konnte Ordnereintrag nicht lesen")?;
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|v| v.to_str()).is_none_or(|v| !v.eq_ignore_ascii_case("wav")) {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|v| v.to_str()).map(str::to_string) else { continue; };
        let midi = dir.join(format!("{stem}.mid"));
        if midi.exists() {
            pairs.push(BatchProjectPair { project: stem, wav_path: path, midi_path: midi });
        }
    }
    pairs.sort_by(|a, b| natural_project_key(&a.project).cmp(&natural_project_key(&b.project)).then_with(|| a.project.cmp(&b.project)));
    Ok(pairs)
}

fn natural_project_key(name: &str) -> Vec<NaturalSortPart> {
    let mut parts = Vec::new();
    let mut buf = String::new();
    let mut in_digits = false;
    for ch in name.chars() {
        let is_digit = ch.is_ascii_digit();
        if !buf.is_empty() && is_digit != in_digits {
            parts.push(natural_part(&buf, in_digits));
            buf.clear();
        }
        buf.push(ch);
        in_digits = is_digit;
    }
    if !buf.is_empty() {
        parts.push(natural_part(&buf, in_digits));
    }
    parts
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum NaturalSortPart {
    Text(String),
    Number(u64),
}

fn natural_part(raw: &str, digits: bool) -> NaturalSortPart {
    if digits {
        NaturalSortPart::Number(raw.parse::<u64>().unwrap_or(0))
    } else {
        NaturalSortPart::Text(raw.to_ascii_lowercase())
    }
}

fn purge_batch_project_outputs(project: &str) -> anyhow::Result<()> {
    let expected = expected_batch_outputs(project);
    for path in [&expected.musicxml, &expected.render_musicxml, &expected.pdf, &expected.result] {
        if path.exists() {
            std::fs::remove_file(path).with_context(|| format!("Konnte alte Batch-Ausgabe nicht entfernen: {}", path.display()))?;
        }
    }
    Ok(())
}

fn write_batch_failure_result(
    project: &str,
    wav_path: &Path,
    midi_path: &Path,
    expected: &BatchExpectedOutputs,
    exit_code: Option<i32>,
    missing_outputs: &[PathBuf],
    page_count_preference_shadow_line: Option<&str>,
) -> anyhow::Result<()> {
    let mut out = String::new();
    out.push_str("ETUSCHLOPF-ERGEBNIS\n");
    out.push_str("==================\n\n");
    out.push_format_args(format_args!("Werkzeug: etuschlopf {}\n", env!("CARGO_PKG_VERSION")));
    out.push_format_args(format_args!("Partitur: {project}\n"));
    out.push_static_named_report_line_v766("Ergebnis", " FEHLER");
    out.push_static_named_report_line_v766("Blocker", " 1 | Warnungen: 0 | Hinweise: 0\n");
    out.push_static_section_header_v795("BATCH-PROJEKT-FEHLER", "--------------------");
    out.push_str("Der --all-Orchestrator hat die Einzelpipeline gestartet, aber keine eigene Projekt-Result-Datei erhalten.\n");
    out.push_format_args(format_args!("ExitCode: {}\n", exit_code.map_or_else(|| "n/a".to_string(), |v| v.to_string())));
    out.push_format_args(format_args!("WAV: {}\n", wav_path.display()));
    out.push_format_args(format_args!("MIDI: {}\n", midi_path.display()));
    out.push_format_args(format_args!("Erwartet: {} | {} | {} | {}\n", expected.musicxml.display(), expected.render_musicxml.display(), expected.pdf.display(), expected.result.display()));
    if !missing_outputs.is_empty() {
        out.push_static_named_report_line_v766("Fehlende Ausgaben", "");
        for path in missing_outputs {
            out.push_format_args(format_args!("- {}\n", path.display()));
        }
    }
    if let Some(shadow_line) = page_count_preference_shadow_line {
        crate::audit_only::page_count_preference_projection::append_section(
            &mut out,
            shadow_line,
            "batch_child_stdout_capture",
        );
    }
    out.push_static_named_report_line_v766("Urteil", " batch_project_pipeline_failed_result_wrapper_created");
    std::fs::write(&expected.result, out).with_context(|| format!("Konnte Batch-Fehlerresultat nicht schreiben: {}", expected.result.display()))
}

fn append_batch_missing_output_note(
    result_path: &Path,
    missing_outputs: &[PathBuf],
    page_count_preference_shadow_line: Option<&str>,
) -> anyhow::Result<()> {
    if missing_outputs.is_empty() && page_count_preference_shadow_line.is_none() {
        return Ok(());
    }
    let mut text = std::fs::read_to_string(result_path).unwrap_or_default();
    if let Some(shadow_line) = page_count_preference_shadow_line
        && !text.contains(
            crate::audit_only::page_count_preference_projection::SECTION,
        ) {
            crate::audit_only::page_count_preference_projection::append_section(
                &mut text,
                shadow_line,
                "batch_child_stdout_capture",
            );
        }
    if !missing_outputs.is_empty() {
        text.push_str("\nBATCH-AUSGABE-GUARD\n-------------------\n");
        text.push_str("Die Einzelpipeline hat eine Result-Datei geschrieben, aber der --all-Orchestrator fand fehlende Ausgabedateien.\n");
        for path in missing_outputs {
            text.push_format_args(format_args!("FEHLT: {}\n", path.display()));
        }
        text.push_static_named_report_line_v766("Urteil", " batch_missing_outputs_detected");
    }
    std::fs::write(result_path, text).with_context(|| format!("Konnte Batch-Guard-Notiz nicht anhängen: {}", result_path.display()))
}

#[derive(Clone, Copy)]
struct BatchProjectProgress<'a> {
    project_index: usize,
    project_count: usize,
    global_total: usize,
    file_total: usize,
    project: &'a str,
}

impl<'a> BatchProjectProgress<'a> {
    fn snapshot(self, global_done: usize, file_done: usize, completed_projects: usize, phase: &'a str) -> BatchProgressSnapshot<'a> {
        BatchProgressSnapshot {
            project_index: self.project_index,
            project_count: self.project_count,
            global_done,
            global_total: self.global_total,
            file_done,
            file_total: self.file_total,
            completed_projects,
            project: self.project,
            phase,
        }
    }
}

#[derive(Clone, Copy)]
struct BatchProgressSnapshot<'a> {
    project_index: usize,
    project_count: usize,
    global_done: usize,
    global_total: usize,
    file_done: usize,
    file_total: usize,
    completed_projects: usize,
    project: &'a str,
    phase: &'a str,
}

fn batch_progress(progress: BatchProgressSnapshot<'_>) {
    let frame = BATCH_PROGRESS_FRAME_COUNTER.fetch_add(1, Ordering::Relaxed);
    batch_progress_frame(progress, frame, Duration::from_secs(0));
    batch_visual_heartbeat(progress, frame);
}

fn batch_progress_frame(
    progress: BatchProgressSnapshot<'_>,
    frame: usize,
    elapsed: Duration,
) {
    let spinner = batch_spinner(frame);
    let file_line = format!(
        "{} {}  Projekt {}/{}: {} | {}{}",
        spinner,
        batch_bar_line_animated("DATEI ", progress.file_done, progress.file_total, 28, frame),
        progress.project_index,
        progress.project_count,
        progress.project,
        progress.phase,
        if elapsed.is_zero() { String::new() } else { format!(" | Laufzeit {}", format_duration_short(elapsed)) }
    );
    let total_spinner = batch_spinner(frame.wrapping_add(1));
    let total_line = format!(
        "{} {}  GESAMT UNTEN | {}/{} Projekte | {}",
        total_spinner,
        batch_bar_line_animated("GESAMT", progress.global_done, progress.global_total, 28, frame.wrapping_mul(2).wrapping_add(7)),
        progress.completed_projects.min(progress.project_count),
        progress.project_count,
        progress.phase
    );

    if batch_live_progress_enabled() {
        let mut active = BATCH_PROGRESS_ACTIVE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if *active {
            print!("\r\x1b[1F\x1b[2K{file_line}\n\r\x1b[2K{total_line}");
        } else {
            print!("{file_line}\n\r{total_line}");
            *active = true;
        }
        let _ = io::stdout().flush();
    } else if frame == 0 || frame.checked_rem(20) == Some(0) {
        println!("{file_line}");
        println!("{total_line}");
    }
}

fn batch_visual_heartbeat(progress: BatchProgressSnapshot<'_>, frame: usize) {
    if !batch_live_progress_enabled() {
        return;
    }
    // Ein sehr kurzer visueller Puls: fachlicher Fortschritt bleibt identisch,
    // aber der untere Gesamtbalken wirkt nicht eingefroren.
    for pulse in 1..=2 {
        thread::sleep(Duration::from_millis(35));
        batch_progress_frame(
            progress,
            frame.wrapping_add(pulse),
            Duration::from_secs(0),
        );
    }
}

fn batch_finish_live_progress() {
    let mut active = BATCH_PROGRESS_ACTIVE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if *active {
        println!();
        *active = false;
    }
}

fn batch_live_progress_enabled() -> bool {
    std::env::var("ETUSCHLOPF_BATCH_LIVE")
        .map(|value| {
            let value = value.trim().to_ascii_lowercase();
            !matches!(value.as_str(), "0" | "false" | "off" | "no" | "plain")
        })
        .unwrap_or(true)
        && io::stdout().is_terminal()
}

fn batch_spinner(frame: usize) -> &'static str {
    const FRAMES: [&str; 4] = ["|", "/", "-", "\\"];
    FRAMES[frame % FRAMES.len()]
}

fn batch_bar_line(label: &str, done: usize, total: usize, width: usize) -> String {
    batch_bar_line_animated(label, done, total, width, 0)
}

fn batch_bar_line_animated(label: &str, done: usize, total: usize, width: usize, frame: usize) -> String {
    let total = total.max(1);
    let done = done.min(total);
    let filled = (done * width + total / 2) / total;
    let pct = (done * 100) / total;
    let mut chars = vec!['.'; width];
    for ch in chars.iter_mut().take(filled.min(width)) {
        *ch = '=';
    }

    if width > 0 {
        if done >= total {
            for ch in chars.iter_mut() {
                *ch = '=';
            }
            // Auch bei 100% noch ein kurzer Abschluss-Glanz, solange der Batch live ist.
            let pulse = frame % width;
            chars[pulse] = '#';
        } else {
            let head = filled.min(width - 1);
            chars[head] = '>';
            // Rein visuelle Bewegung über die ganze Leiste, unabhängig vom echten Prozentwert.
            let pulse = frame % width;
            chars[pulse] = if pulse < filled { '#' } else { 'o' };
        }
    }

    let bar: String = chars.into_iter().collect();
    let bar = colorize(progress_color_code_for_context(done, total, ProgressColorContext::DefaultArtifact), bar);
    format!("{label} [{bar}] {done}/{total} {pct:>3}%")
}



#[derive(Debug, Default)]
struct CorpusProjectObservation {
    project: String,
    report_path: PathBuf,
    result_version: Option<String>,
    result_ok: bool,
    blockers: usize,
    warnings: usize,
    hints: usize,
    retention_percent: Option<usize>,
    final_validator_clean: bool,
    outputs_complete: Option<bool>,
    execution_outcome_observed: Option<bool>,
    feedback_candidate_occurrences: usize,
    feedback_transport_green: bool,
    feedback_proof_green: bool,
    feedback_origin_can_select: bool,
    tonal_boundaries_checked: usize,
}


#[derive(Debug, Clone)]
struct CorpusOutputEvidenceProject {
    outputs_complete: bool,
    execution_outcome_observed: bool,
}

#[derive(Debug, Clone)]
struct BoundCorpusOutputEvidence {
    expected_projects: Vec<String>,
    projects: std::collections::BTreeMap<String, CorpusOutputEvidenceProject>,
}

fn load_bound_corpus_output_evidence(
    run_id_override: Option<&str>,
) -> Option<BoundCorpusOutputEvidence> {
    let manifest = run_id_override.and_then(|run_id| {
        crate::corpus_run_manifest::read_manifest(Path::new(CORPUS_RUN_MANIFEST_FILE))
            .ok()
            .filter(|manifest| {
                manifest.run_id == run_id && manifest.version == env!("CARGO_PKG_VERSION")
            })
    })?;
    let projects = manifest
        .projects
        .into_iter()
        .map(|project| {
            (
                project.project,
                CorpusOutputEvidenceProject {
                    outputs_complete: project.outputs_complete,
                    execution_outcome_observed: project.renderer_outcome_observed,
                },
            )
        })
        .collect();
    Some(BoundCorpusOutputEvidence {
        expected_projects: manifest.expected_projects,
        projects,
    })
}

fn corpus_output_evidence_summary(
    bound: bool,
    outputs_complete_count: usize,
    execution_outcome_observed_count: usize,
    project_count: usize,
) -> String {
    if bound {
        format!(
            "Ausgaben vollstaendig {outputs_complete_count}/{project_count}; Renderer belegt {execution_outcome_observed_count}/{project_count}"
        )
    } else {
        "Ausgaben vollstaendig n/a; Renderer belegt n/a (kein laufgebundenes Manifest)".to_string()
    }
}

fn corpus_project_evidence_table_header() -> &'static str {
    "Projekt | Version | Ergebnis | Blocker | Warnungen | Hinweise | FinalClean | AusgabenVollstaendig | RendererBelegt | CanonicalSourceRetention\n"
}

fn append_corpus_output_evidence_data(
    out: &mut String,
    bound: bool,
    outputs_complete_count: usize,
    execution_outcome_observed_count: usize,
) {
    if bound {
        out.push_format_args(format_args!(
            "outputs_complete_projects={outputs_complete_count}\n"
        ));
        out.push_format_args(format_args!(
            "renderer_outcome_observed_projects={execution_outcome_observed_count}\n"
        ));
    } else {
        out.push_str("outputs_complete_projects=n/a\n");
        out.push_str("renderer_outcome_observed_projects=n/a\n");
    }
}


fn write_corpus_emergence_report(out_path: &Path) -> anyhow::Result<i32> {
    write_corpus_emergence_report_for_run(out_path, None)
}

fn write_corpus_emergence_report_for_run(
    out_path: &Path,
    run_id_override: Option<&str>,
) -> anyhow::Result<i32> {
    let generated_unix_seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let corpus_run_id = run_id_override
        .map_or_else(|| format!("corpus-{}-{generated_unix_seconds}", env!("CARGO_PKG_VERSION")), str::to_string);

    let mut observations = Vec::new();
    for entry in std::fs::read_dir(".").context("Konnte aktuellen Ordner nicht lesen")? {
        let entry = entry.context("Konnte Ordnereintrag nicht lesen")?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if !name.ends_with(".result.txt") || name == "corpus_emergence.result.txt" {
            continue;
        }
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(err) => {
                eprintln!("CorpusEmergence: ueberspringe {}: {}", path.display(), err);
                continue;
            }
        };
        observations.push(observe_project_result(path, &text));
    }
    observations.sort_by(|left, right| {
        natural_project_key(&left.project).cmp(&natural_project_key(&right.project))
    });

    let bound_evidence = load_bound_corpus_output_evidence(run_id_override);
    if let Some(evidence) = bound_evidence.as_ref() {
        observations.retain(|observation| evidence.expected_projects.contains(&observation.project));
        for observation in &mut observations {
            if let Some(project) = evidence.projects.get(&observation.project) {
                observation.outputs_complete = Some(project.outputs_complete);
                observation.execution_outcome_observed = Some(project.execution_outcome_observed);
            }
        }
    }

    let mut out = String::new();
    out.push_str("ETUSCHLOPF-CORPUS-EMERGENCE\n");
    out.push_str("===========================\n\n");
    out.push_format_args(format_args!("Werkzeug: etuschlopf {}\n", env!("CARGO_PKG_VERSION")));
    out.push_format_args(format_args!("GeneratedUnixSeconds: {generated_unix_seconds}\n"));
    out.push_format_args(format_args!("RunId: {corpus_run_id}\n"));
    out.push_format_args(format_args!(
        "CorpusRunManifest: {}\n",
        if run_id_override.is_some() {
            CORPUS_RUN_MANIFEST_FILE
        } else {
            "none_standalone_report"
        }
    ));
    out.push_format_args(format_args!(
        "EvidenceContract: {}\n",
        if run_id_override.is_some() {
            "hash_bound_corpus_run_manifest_v1"
        } else {
            "standalone_report_without_batch_manifest"
        }
    ));
    out.push_static_named_report_line_v766(
        "Modus",
        " aktuelle formale Korpusprojektion aus projektbezogenen *.result.txt-Dateien",
    );
    out.push_static_named_report_line_v766("Musikmutation", " nein; dieser Bericht schreibt keine Musik");
    out.push_static_named_report_line_v766("Regeländerung", " nein");
    out.push_static_named_report_line_v766(
        "Prioritaetsvertrag",
        " keine Etuede, kein Takt, kein Writer und keine Passreihenfolge werden aus historischen Auditdaten abgeleitet.\n",
    );

    if observations.is_empty() {
        out.push_str("KOMPAKT\n-------\n");
        out.push_str("Keine projektbezogenen .result.txt-Dateien gefunden. Starte zuerst run.cmd --all im Ordner mit WAV/MIDI-Paaren.\n");
        std::fs::write(out_path, out)
            .with_context(|| format!("Konnte Korpusbericht nicht schreiben: {}", out_path.display()))?;
        println!("CorpusEmergence: keine Result-Dateien gefunden -> {}", out_path.display());
        return Ok(1);
    }

    let project_count = observations.len();
    let ok_count = observations.iter().filter(|observation| observation.result_ok).count();
    let blocker_projects = observations.iter().filter(|observation| observation.blockers > 0).count();
    let warning_projects = observations.iter().filter(|observation| observation.warnings > 0).count();
    let hint_projects = observations.iter().filter(|observation| observation.hints > 0).count();
    let final_clean_count = observations
        .iter()
        .filter(|observation| observation.final_validator_clean)
        .count();
    let outputs_complete_count = observations
        .iter()
        .filter(|observation| observation.outputs_complete == Some(true))
        .count();
    let execution_outcome_observed_count = observations
        .iter()
        .filter(|observation| observation.execution_outcome_observed == Some(true))
        .count();
    let feedback_candidate_occurrences = observations
        .iter()
        .map(|observation| observation.feedback_candidate_occurrences)
        .sum::<usize>();
    let feedback_transport_green_projects = observations
        .iter()
        .filter(|observation| observation.feedback_transport_green)
        .count();
    let feedback_proof_green_projects = observations
        .iter()
        .filter(|observation| observation.feedback_proof_green)
        .count();
    let feedback_origin_select_projects = observations
        .iter()
        .filter(|observation| observation.feedback_origin_can_select)
        .count();
    let tonal_boundaries_checked = observations
        .iter()
        .map(|observation| observation.tonal_boundaries_checked)
        .sum::<usize>();
    let current_version_project_count = observations
        .iter()
        .filter(|observation| observation.result_version.as_deref() == Some(env!("CARGO_PKG_VERSION")))
        .count();
    let retention_values = observations
        .iter()
        .filter_map(|observation| observation.retention_percent)
        .collect::<Vec<_>>();
    let avg_retention = if retention_values.is_empty() {
        None
    } else {
        Some(retention_values.iter().sum::<usize>() / retention_values.len())
    };
    let expected_project_count = bound_evidence
        .as_ref()
        .map_or(0, |manifest| manifest.expected_projects.len());
    let missing_projects = bound_evidence.as_ref().map_or_else(Vec::new, |evidence| {
        missing_expected_projects(&evidence.expected_projects, &observations)
    });
    let full_corpus = expected_project_count > 0
        && project_count == expected_project_count
        && current_version_project_count == expected_project_count
        && missing_projects.is_empty();
    let projects = observations
        .iter()
        .map(|observation| observation.project.clone())
        .collect::<Vec<_>>()
        .join(", ");
    let missing_project_list = if missing_projects.is_empty() {
        "none".to_string()
    } else {
        join_project_list(&missing_projects)
    };
    let selection_reason = if full_corpus {
        "fresh_full_expected_project_corpus_current_facts_only_automatic_target_selection_retired"
    } else {
        "corpus_incomplete_or_version_mixed_no_priority"
    };

    out.push_static_section_header_v795("\nCORPUS-RUN-METADATA", "-------------------");
    out.push_format_args(format_args!("RunId: {corpus_run_id}\n"));
    out.push_format_args(format_args!("GeneratedUnixSeconds: {generated_unix_seconds}\n"));
    out.push_format_args(format_args!("CorpusRunManifest: {}\n", if run_id_override.is_some() { CORPUS_RUN_MANIFEST_FILE } else { "none_standalone_report" }));
    out.push_format_args(format_args!("EvidenceContract: {}\n", if run_id_override.is_some() { "hash_bound_corpus_run_manifest_v1" } else { "standalone_report_without_batch_manifest" }));
    out.push_format_args(format_args!("FullCorpus: {}\n", yes_no(full_corpus)));
    out.push_format_args(format_args!("ProjectCount: {project_count}\n"));
    out.push_format_args(format_args!("CurrentVersionProjectCount: {current_version_project_count}\n"));
    out.push_format_args(format_args!("Projects: {projects}\n"));
    out.push_format_args(format_args!("ExpectedProjectCount: {expected_project_count}\n"));
    out.push_format_args(format_args!("MissingProjects: {missing_project_list}\n"));
    out.push_str("SelectionEligible: nein\n");
    out.push_format_args(format_args!("SelectionReason: {selection_reason}\n"));
    out.push_str("AutomaticTargetSelection: nein\n");
    out.push_str("ActiveRepairTarget: none\n");

    out.push_static_section_header_v795("\nKOMPAKT", "-------");
    let output_evidence_summary = corpus_output_evidence_summary(
        bound_evidence.is_some(),
        outputs_complete_count,
        execution_outcome_observed_count,
        project_count,
    );
    out.push_format_args(format_args!(
        "Aktueller formaler Stand: Projekte {project_count}; OK {ok_count}; Blockerprojekte {blocker_projects}; Warnprojekte {warning_projects}; Hinweisprojekte {hint_projects}; final-clean {final_clean_count}; {output_evidence_summary}; durchschnittlicher CanonicalSourceRetention {}%.\n",
        avg_retention
            .map_or_else(|| "n/a".to_string(), |value| value.to_string())
    ));
    out.push_str("Keine musikalische Prioritaet wird aus frueheren Auditzaehlern, Taktadressen, mathematischen Passordnungen oder Visual-Hotspot-Rankings erzeugt.\n");

    out.push_static_section_header_v795("\nAKTUELLE-PROJEKTERGEBNISSE", "---------------------------");
    out.push_str(corpus_project_evidence_table_header());
    for observation in &observations {
        out.push_format_args(format_args!(
            "{} | {} | {} | {} | {} | {} | {} | {} | {} | {}\n",
            observation.project,
            observation.result_version.as_deref().unwrap_or("n/a"),
            if observation.result_ok { "OK" } else { "FEHLER" },
            observation.blockers,
            observation.warnings,
            observation.hints,
            yes_no(observation.final_validator_clean),
            observation.outputs_complete.map_or("n/a", yes_no),
            observation.execution_outcome_observed.map_or("n/a", yes_no),
            observation
                .retention_percent
                .map_or_else(|| "n/a".to_string(), |value| format!("{value}%"))
        ));
    }

    out.push_static_section_header_v795("\nAKTUELLE-SICHERHEITSEVIDENZ", "---------------------------");
    out.push_format_args(format_args!("FeedbackCandidateOccurrences: {feedback_candidate_occurrences}\n"));
    out.push_format_args(format_args!("FeedbackTransportGreenProjects: {feedback_transport_green_projects}/{project_count}\n"));
    out.push_format_args(format_args!("FeedbackProofGreenProjects: {feedback_proof_green_projects}/{project_count}\n"));
    out.push_format_args(format_args!("FeedbackOriginCanSelectProjects: {feedback_origin_select_projects}\n"));
    out.push_format_args(format_args!("TonalEvidenceBoundariesChecked: {tonal_boundaries_checked}\n"));
    out.push_str("Entscheidung: Diese Werte pruefen Evidenztransport und Nichtbeeinflussung; sie waehlen keine musikalische Arbeit aus.\n");

    out.push_static_section_header_v795("\nHISTORISCHE-HYPOTHESEN-RETIREMENT", "---------------------------------");
    out.push_str("HistoricalMeasureHypothesesAuthoritative: nein\n");
    out.push_str("HistoricalPassOrderAuthoritative: nein\n");
    out.push_str("HistoricalAuditCountersSelectWork: nein\n");
    out.push_str("VisualHotspotAutoSelection: nein\n");
    out.push_str("MusicalPriorityRequiresFreshEvidence: ja\n");
    out.push_str("FreshEvidenceSources: aktuelles PDF, aktuelle MusicXML-Artefakte, konkreter Nutzerbefund oder frisch reproduzierte Validatorverletzung\n");
    out.push_str("Entscheidung: Der Korpusnachweis bestaetigt den aktuellen formalen Zustand, erzeugt aber keinen Reparaturauftrag.\n");

    out.push_static_section_header_v795("\nCORPUS-EMERGENCE-DATA", "---------------------");
    let report_files = observations
        .iter()
        .map(|observation| observation.report_path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    out.push_format_args(format_args!("report_files={report_files}\n"));
    out.push_format_args(format_args!("projects={project_count}\n"));
    out.push_format_args(format_args!("current_version_projects={current_version_project_count}\n"));
    out.push_format_args(format_args!("ok_projects={ok_count}\n"));
    out.push_format_args(format_args!("blocker_projects={blocker_projects}\n"));
    out.push_format_args(format_args!("warning_projects={warning_projects}\n"));
    out.push_format_args(format_args!("hint_projects={hint_projects}\n"));
    out.push_format_args(format_args!("final_clean_projects={final_clean_count}\n"));
    append_corpus_output_evidence_data(
        &mut out,
        bound_evidence.is_some(),
        outputs_complete_count,
        execution_outcome_observed_count,
    );
    out.push_format_args(format_args!("feedback_candidate_occurrences={feedback_candidate_occurrences}\n"));
    out.push_format_args(format_args!("feedback_transport_green_projects={feedback_transport_green_projects}\n"));
    out.push_format_args(format_args!("feedback_proof_green_projects={feedback_proof_green_projects}\n"));
    out.push_format_args(format_args!("feedback_origin_can_select_projects={feedback_origin_select_projects}\n"));
    out.push_format_args(format_args!("tonal_evidence_boundaries_checked={tonal_boundaries_checked}\n"));
    out.push_format_args(format_args!(
        "avg_retention_percent={}\n",
        avg_retention
            .map_or_else(|| "n/a".to_string(), |value| value.to_string())
    ));
    out.push_format_args(format_args!("missing_projects={missing_project_list}\n"));
    out.push_str("active_repair_target=none\n");
    out.push_str("historical_measure_hypotheses_authoritative=no\n");
    out.push_str("historical_pass_order_authoritative=no\n");
    out.push_str("historical_audit_counters_select_work=no\n");
    out.push_str("visual_hotspot_auto_selection=no\n");
    out.push_str("musical_priority_requires_fresh_evidence=yes\n");
    out.push_str("music_mutation=no\n");
    out.push_str("verdict=current_corpus_facts_only_no_automatic_musical_priority\n");

    std::fs::write(out_path, out)
        .with_context(|| format!("Konnte Korpusbericht nicht schreiben: {}", out_path.display()))?;
    println!(
        "CorpusEmergence: {} Projektbericht(e) formal ausgewertet -> {}",
        project_count,
        out_path.display()
    );
    if feedback_transport_green_projects != project_count {
        eprintln!("CorpusEmergence: Feedback-Evidenztransport unvollstaendig; Korpuslauf bleibt fail-closed.");
        return Ok(1);
    }
    Ok(0)
}










