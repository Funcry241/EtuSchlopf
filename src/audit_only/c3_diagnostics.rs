//! Project-local, report-only observations. Never read by musical decisions.
use std::fmt::Write as _;
use std::sync::Mutex;

thread_local! { static SHADOW: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
pub(crate) struct ShadowDiagnosticScope { previous: bool, _same_thread: std::marker::PhantomData<std::rc::Rc<()>> }
pub(crate) fn shadow_scope(shadow: bool) -> ShadowDiagnosticScope {
    let previous = SHADOW.with(|slot| { let previous=slot.get(); slot.set(previous || shadow); previous });
    ShadowDiagnosticScope { previous, _same_thread: std::marker::PhantomData }
}
impl Drop for ShadowDiagnosticScope {
    fn drop(&mut self) { SHADOW.with(|slot|slot.set(self.previous)); }
}

#[derive(Default)]
struct Trace {
    phase: String,
    rows: Vec<String>,
    first_render_difference: bool,
    first_guard_conflict: bool,
    first_shadow_guard_conflict: bool,
}
impl Trace {
    fn row(&mut self, document: &str, staff: &str, status: &str, detail: &str) {
        let phase=if SHADOW.with(|slot|slot.get()) {format!("Shadow/{}",self.phase)} else {self.phase.clone()};
        let row=format!("C3Diagnostic: Phase={:?}; Document={document:?}; Staff={staff:?}; Status={status:?}; Detail={detail:?}.", phase);
        persist_row(&row);
        self.rows.push(row);
    }
    fn append(&self, out: &mut String) {
        let _ = writeln!(out, "C3DiagnosticSchema: 1; Scope=containing_project_result; ComparisonBasis=planning_point_P; ComparisonPolicy={}; CostOrder={}; MusicalAuthority=none.", course::POLICY_ID, course::COST_COMPONENTS.join(","));
        for row in &self.rows { let _=writeln!(out,"{row}"); }
        if self.rows.is_empty() {out.push_str("C3Diagnostic: Status=not_observed; no_decision_inferred.\n");}
    }
}
static TRACE: Mutex<Trace> = Mutex::new(Trace {phase:String::new(),rows:Vec::new(),first_render_difference:false,first_guard_conflict:false,first_shadow_guard_conflict:false});
pub(crate) fn reset() { *TRACE.lock().unwrap_or_else(|e|e.into_inner())=Trace::default(); }
pub(crate) fn phase(phase:&str) {
    let mut trace=TRACE.lock().unwrap_or_else(|e|e.into_inner());
    trace.phase=phase.to_owned();
    trace.row("both","all","entered","");
}
pub(crate) fn record(document:&str,staff:&str,status:&str,detail:&str) {
    TRACE.lock().unwrap_or_else(|e|e.into_inner()).row(document,staff,status,detail);
}
pub(crate) fn begin() {
    phase("entry");
    for document in ["Master","Render"] {
        for staff in ["1","2"] {record(document,staff,"adapter_not_reached","No adapter result observed yet");}
    }
}
pub(crate) fn finish(status:&str,detail:&str) {record("both","all",status,detail);}
pub(crate) fn append_report(out:&mut String) {
    TRACE.lock().unwrap_or_else(|e|e.into_inner()).append(out);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error_phase_and_terminal_status_survive_report_projection() {
        let mut trace=Trace {phase:"octave_anchors".into(),..Trace::default()};
        trace.row("Render","2","unknown","missing visible endpoint\nnext line");
        trace.row("both","all","retained","preparation failed");
        let mut out=String::new();trace.append(&mut out);
        assert!(out.contains("octave_anchors"));assert!(out.contains("retained"));
        assert!(out.contains("endpoint\\nnext line"));assert!(!out.contains("endpoint\nnext line"));
        assert_eq!(out.lines().count(),3);
    }
    #[test]
    fn empty_trace_does_not_claim_a_retained_or_adopted_course() {
        let mut out=String::new();Trace::default().append(&mut out);
        assert!(out.contains("not_observed"));assert!(!out.contains("adopted"));assert!(!out.contains("retained"));
    }
}

/// Compare piecewise-constant courses at the union of their actual time boundaries.
/// A relocated sign can differ even if the sequence of states is identical.
pub(crate) fn first_course_difference<T: Copy + Ord, S: Copy + Eq>(
    expected: &[(T,S)], observed: &[(T,S)],
) -> Option<(T,Option<S>,Option<S>)> {
    let times=expected.iter().chain(observed).map(|(at,_)|*at)
        .collect::<std::collections::BTreeSet<_>>();
    for at in times {
        let expected=expected.iter().rev().find(|(time,_)|*time<=at).map(|(_,state)|*state);
        let observed=observed.iter().rev().find(|(time,_)|*time<=at).map(|(_,state)|*state);
        if expected!=observed {return Some((at,expected,observed));}
    }
    None
}
pub(crate) fn render_checkpoint(staff:&str, differs:bool, detail:&str) {
    let mut trace=TRACE.lock().unwrap_or_else(|e|e.into_inner());
    let first=differs && !trace.first_render_difference;
    trace.first_render_difference |= differs;
    trace.row("Render",staff,if first {"first_course_difference"} else if differs {"course_difference"} else {"course_bound"},detail);
}

// This sink is evidence only. It is inside the bootstrap's already exported
// attempt writeahead directory, so the installed parent can retain it even
// when the candidate is rolled back. It never writes a public result artifact.
#[derive(Default)]
struct EvidenceSession {
    journal: Option<std::fs::File>,
    report: Option<std::path::PathBuf>,
    prior_report: Option<Vec<u8>>,
    directory: Option<std::path::PathBuf>,
}
static EVIDENCE: Mutex<EvidenceSession> = Mutex::new(EvidenceSession {
    journal:None,report:None,prior_report:None,directory:None,
});
pub(crate) fn start_project(report:&std::path::Path) {
    let Some(root)=std::env::var_os("ETUSCHLOPF_TAKE_ATTEMPT_EVIDENCE_DIR") else {return;};
    let directory=std::path::PathBuf::from(root).join(format!("c3-project-{}",std::process::id()));
    if let Err(error)=std::fs::create_dir_all(&directory) {
        eprintln!("C3DiagnosticPersistence: cannot create project evidence: {error}");return;
    }
    let journal=match std::fs::File::create(directory.join("c3.diagnostic.txt")) {
        Ok(file)=>file,Err(error)=>{eprintln!("C3DiagnosticPersistence: cannot open journal: {error}");return;}
    };
    *EVIDENCE.lock().unwrap_or_else(|e|e.into_inner())=EvidenceSession {
        journal:Some(journal),report:Some(report.to_owned()),prior_report:std::fs::read(report).ok(),directory:Some(directory),
    };
    persist_row(&format!("C3DiagnosticEvidence: Schema=1; Report={report:?}; PID={}; Authority=none; State=running.",std::process::id()));
}
fn persist_row(row:&str) {
    use std::io::Write as _;
    let mut evidence=EVIDENCE.lock().unwrap_or_else(|e|e.into_inner());
    if let Some(file)=evidence.journal.as_mut()
        && let Err(error)=writeln!(file,"{row}").and_then(|()|file.flush()) {
        eprintln!("C3DiagnosticPersistence: journal write failed: {error}");
    }
}
/// Drop also runs on early return and unwinding; already flushed rows survive
/// process termination. An absent report stays absent, never fabricated as OK.
pub(crate) struct FailureEvidenceGuard;
impl Drop for FailureEvidenceGuard {
    fn drop(&mut self) {
        let evidence=EVIDENCE.lock().unwrap_or_else(|e|e.into_inner());
        let status=if let (Some(report),Some(directory))=(&evidence.report,&evidence.directory) {
            match std::fs::read(report) {
                Ok(bytes) if evidence.prior_report.as_ref()!=Some(&bytes)=>{
                    match std::fs::write(directory.join("project.result.txt"),bytes) {
                        Ok(())=>"current_report_copied".to_owned(),
                        Err(error)=>format!("report_copy_failed: {error}"),
                    }
                },
                Ok(_)=>"report_unchanged_current_run_not_proven".to_owned(),
                Err(error)=>format!("report_unavailable: {error}"),
            }
        } else {return;};
        drop(evidence);
        persist_row(&format!("C3DiagnosticEvidence: State=process_return; ReportStatus={status:?}; Authority=none."));
    }
}

#[cfg(test)]
mod checkpoint_tests {
    use super::*;
    #[test]
    fn moved_octave_boundary_is_detected_without_any_changed_state_sequence() {
        assert_eq!(first_course_difference(&[(0,0),(8,1),(12,0)],&[(0,0),(9,1),(12,0)]),Some((8,Some(1),Some(0))));
        assert_eq!(first_course_difference(&[(0,0),(8,1)],&[(0,0),(4,0),(8,1)]),None);
        assert_eq!(first_course_difference(&[(0,0)],&[(1,0)]),Some((0,Some(0),None)));
    }
}


pub(crate) fn guard_checkpoint(transition:&str,source:&str) {
    if !crate::pipeline::joint_reading_course_active() {return;}
    let observation=crate::structural_write_authority::clef_surface_materialization::joint_reading_transaction::observe_guard_conflicts(source);
    let mut trace=TRACE.lock().unwrap_or_else(|e|e.into_inner());
    match observation {
        Ok(rows)=>{
            let shadow=SHADOW.with(|slot|slot.get());
            let flag=if shadow {&mut trace.first_shadow_guard_conflict} else {&mut trace.first_guard_conflict};
            let first=!rows.is_empty() && !*flag;
            *flag|=!rows.is_empty();
            let transition=if shadow {format!("Shadow/{transition}")} else {transition.to_owned()};
            trace.row("Render","2",if rows.is_empty(){"guard_clear"}else if first{"first_guard_conflict"}else{"guard_conflict"},
                &format!("Transition={transition:?}; Conflicts={rows:?}; Authority=none"));
        },
        Err(error)=>trace.row("Render","2","guard_observation_unknown",&format!("Transition={transition:?}; Reason={error}")),
    }
}
pub(crate) fn primary_projection_failed(source:&str,error:&str) {
    record("Render","2","primary_clef_projection_failed",error);
    guard_checkpoint("FinalPdfSource/PrimaryClefProjectionFailed",source);
    let mut dump=format!("C3PrimaryProjectionInput: Schema=1; Phase=final_pdf_primary_projection_failure; Authority=none; InputBytes={}; InputSha256={}; Encoding=hex.\n",source.len(),crate::sha256::hex(source.as_bytes()));
    for chunk in source.as_bytes().chunks(64) {
        dump.push_str("InputHex: ");
        for byte in chunk {let _=write!(dump,"{byte:02x}");}
        dump.push('\n');
    }
    let evidence=EVIDENCE.lock().unwrap_or_else(|e|e.into_inner());
    let saved=evidence.directory.as_ref().map(|dir|std::fs::write(dir.join("primary-clef-projection-input.txt"),dump));
    drop(evidence);
    record("Render","2","primary_projection_source_snapshot",&format!("Saved={saved:?}; SnapshotAuthority=none"));
}

#[cfg(test)]
mod shadow_scope_tests {
    use super::*;
    #[test]
    fn diagnostic_scope_marks_rows_and_restores_on_unwind() {
        assert!(!SHADOW.with(|slot|slot.get()));
        let mut trace=Trace::default();
        let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _scope=shadow_scope(true);
            {let _nested=shadow_scope(false);assert!(SHADOW.with(|slot|slot.get()));}
            trace.row("Render","2","guard_conflict","diagnostic only");
            panic!("synthetic Shadow abort");
        }));
        assert!(result.is_err());assert!(!SHADOW.with(|slot|slot.get()));
        trace.row("Render","2","guard_clear","Live");
        assert!(trace.rows[0].contains("Shadow/"));assert!(!trace.rows[1].contains("Shadow/"));
        assert!(!trace.first_guard_conflict);
    }
}

// Whole-domain counterfactuals at the existing planning-point observation.
// This observer has no path back into selection, preparation or adoption.
use etuschlopf::joint_reading_course as course;

#[derive(Debug, PartialEq, Eq)]
struct ExcursionComparison {
    start: usize,
    end: usize,
    replacement: course::State,
    selected_cost: course::Cost,
    alternative: Result<(course::Cost, course::LedgerMetrics), course::Error>,
}

/// For every selected run, inspect its next return to the same combined state.
/// Only the intervening excursion is replaced; the full incoming/outgoing path,
/// original incumbent, allowed states, group edges and time model stay bound.
fn compare_course_excursions(
    domain: &course::Domain,
    path: &[course::State],
) -> Result<Vec<ExcursionComparison>, course::Error> {
    let selected_cost = course::evaluate_course(domain, path)?;
    let runs = path.iter().enumerate().filter_map(|(index, state)|
        (index == 0 || path[index - 1] != *state).then_some((index, *state)))
        .collect::<Vec<_>>();
    let mut rows = Vec::new();
    for (index, (_, state)) in runs.iter().enumerate() {
        let Some(&(end, _)) = runs.iter().skip(index + 1).find(|(_, next)| next == state) else { continue; };
        let start = runs[index + 1].0;
        let mut alternative = path.to_vec();
        alternative[start..end].fill(*state);
        let evaluated = course::evaluate_course(domain, &alternative)
            .and_then(|cost| course::ledger_metrics(domain, &alternative).map(|metrics| (cost, metrics)));
        rows.push(ExcursionComparison { start, end, replacement: *state, selected_cost, alternative: evaluated });
    }
    Ok(rows)
}

pub(crate) fn record_course_excursions(
    document: &str,
    staff: &str,
    source: &str,
    domain: &course::Domain,
    outcome: &course::Outcome,
    anchors: &[(usize, (i128, i128))],
) {
    let source_sha256 = crate::sha256::hex(source.as_bytes());
    let binding = format!("SourceSha256={source_sha256}; ComparisonBasis=planning_point_P; FullDomain=true; LaterMaterialization=not_attempted; MusicalAuthority=none");
    if !outcome.complete || anchors.len() != domain.layers.len() {
        record(document, staff, "excursion_comparison_unavailable", &format!("{binding}; Reason=incomplete search or anchor binding"));
        return;
    }
    // Complete same-epoch evidence also supports later one-off questions outside
    // the built-in excursion family, without installing piece-specific probes.
    // Debug grammar is bound to this exact source revision, not a public API.
    record(document, staff, "course_domain_snapshot", &format!("{binding}; SnapshotSchema=1; KernelDomain={domain:?}; SelectedPath={:?}; Anchors={anchors:?}; SelectedCost={:?}",
        outcome.selected, outcome.selected_cost));
    let rows = match compare_course_excursions(domain, &outcome.selected) {
        Ok(rows) if rows.iter().all(|row| row.selected_cost == outcome.selected_cost) => rows,
        Ok(_) => {
            record(document, staff, "excursion_comparison_unavailable", &format!("{binding}; Reason=selected cost binding differs"));
            return;
        },
        Err(error) => {
            record(document, staff, "excursion_comparison_unavailable", &format!("{binding}; Reason={error:?}"));
            return;
        },
    };
    record(document, staff, "excursion_comparison_observed", &format!("{binding}; ReturningExcursions={}", rows.len()));
    let components = course::COST_COMPONENTS;
    for row in rows {
        let location = format!("{binding}; Start={:?}; EndExclusive={:?}; ReplacementState={}; SelectedCost={:?}",
            anchors[row.start], anchors[row.end], row.replacement.index(), row.selected_cost);
        match row.alternative {
            Err(error) => record(document, staff, "excursion_graph_rejected", &format!("{location}; GraphError={error:?}; NoMaterializerConclusion=true")),
            Ok((cost, metrics)) => {
                let first = row.selected_cost.0.iter().zip(cost.0).position(|(selected, alternative)| *selected != alternative);
                let difference = first.map(|index| format!("{}; SelectedComponent={}; AlternativeComponent={}",
                    components[index], row.selected_cost.0[index], cost.0[index])).unwrap_or_else(|| "none".to_owned());
                let order = match cost.cmp(&row.selected_cost) {
                    std::cmp::Ordering::Less => "primary_better_counterexample",
                    std::cmp::Ordering::Equal => "primary_equal_tie_break_not_evaluated",
                    std::cmp::Ordering::Greater => "primary_worse",
                };
                record(document, staff, "excursion_graph_admissible", &format!("{location}; AlternativeCost={cost:?}; FirstDifference={difference}; PrimaryOrder={order}; MaximumLedgers={}; VisibleHeads={}; LedgerHistogram={:?}",
                    metrics.maximum, metrics.visible_heads, metrics.histogram));
            },
        }
    }
}

#[cfg(test)]
mod excursion_tests {
    use super::*;
    fn state(index: u8) -> course::State { course::State::new(index).unwrap() }
    fn domain(pattern: &[u8], low: i32) -> course::Domain {
        course::Domain {
            entry: course::Entry::Initial, exit: course::Exit::End,
            exit_allowed: [true; 10], exit_group_crossing: false,
            layers: pattern.iter().map(|index| course::Layer {
                duration: course::Time::new(1, 1).unwrap(), incumbent: state(*index),
                heads: vec![course::Head { diatonic: if *index == 5 { low } else { 35 }, visible: true }],
                allowed: vec![state(0), state(5)], group_crossing: false, incoming: [[true; 10]; 10],
            }).collect(),
        }
    }
    #[test]
    fn admissible_stable_course_wins_on_sign_count_before_three_ledger_heads() {
        let domain = domain(&[0, 5, 0], 24);
        let outcome = course::solve(&domain, None).unwrap();
        assert_eq!(outcome.selected, vec![state(0), state(0), state(0)]);
        let before = outcome.clone();
        let rows = compare_course_excursions(&domain, &[state(0), state(5), state(0)]).unwrap();
        assert_eq!(rows.len(), 1);
        let (alternative, metrics) = rows[0].alternative.as_ref().unwrap();
        assert_eq!(metrics.maximum, 3);
        assert_eq!(alternative.0[10], rows[0].selected_cost.0[10] + 1);
        assert_eq!(alternative.0[9] + 2, rows[0].selected_cost.0[9]);
        assert!(*alternative < rows[0].selected_cost);
        assert_eq!(outcome, before);
    }
    #[test]
    fn protected_heads_and_fixed_states_are_not_turned_into_cost_losses() {
        let guarded = domain(&[0, 5, 0], 21);
        let path = vec![state(0), state(5), state(0)];
        assert!(compare_course_excursions(&guarded, &path).unwrap()[0].alternative.is_err());
        let mut fixed = domain(&[0, 5, 0], 24);
        fixed.layers[1].allowed = vec![state(5)];
        assert!(compare_course_excursions(&fixed, &path).unwrap()[0].alternative.is_err());
    }
    #[test]
    fn returning_excursions_are_all_reported_without_a_global_winner() {
        let one = domain(&[0, 5, 0], 24);
        let rows = compare_course_excursions(&one, &[state(0), state(5), state(0)]).unwrap();
        let two = domain(&[0, 5, 0, 5, 0], 24);
        let path = [state(0), state(5), state(0), state(5), state(0)];
        let duplicated = compare_course_excursions(&two, &path).unwrap();
        assert_eq!(rows.iter().filter(|row| row.replacement == state(0)).count(), 1);
        assert_eq!(duplicated.iter().filter(|row| row.replacement == state(0)).count(), 2);
        assert_eq!(duplicated.len(), 3);
        assert_eq!(course::solve(&two, None).unwrap().selected, vec![state(0); path.len()]);
    }
    #[test]
    fn complete_boundaries_and_nonreturning_courses_remain_explicit() {
        let plain = domain(&[0, 0], 24);
        assert!(compare_course_excursions(&plain, &[state(0), state(0)]).unwrap().is_empty());
        let path = [state(0), state(5), state(0)];
        let mut timed = domain(&[0, 5, 0], 24);
        timed.entry = course::Entry::Carry { state: state(0), elapsed: course::Time::new(1, 2).unwrap() };
        timed.exit = course::Exit::Carry { state: state(0), remaining: course::Time::new(1, 2).unwrap() };
        assert!(compare_course_excursions(&timed, &path).unwrap()[0].alternative.is_ok());
        assert!(compare_course_excursions(&timed, &path[..2]).is_err());
    }
    #[test]
    fn irrelevant_prefix_relocation_preserves_the_cost_comparison() {
        let original = domain(&[0, 5, 0], 24);
        let moved = domain(&[0, 0, 5, 0], 24);
        let before = compare_course_excursions(&original, &[state(0), state(5), state(0)]).unwrap();
        let after = compare_course_excursions(&moved, &[state(0), state(0), state(5), state(0)]).unwrap();
        assert_eq!(before[0].selected_cost, after[0].selected_cost);
        assert_eq!(before[0].alternative.as_ref().unwrap().0, after[0].alternative.as_ref().unwrap().0);
        assert_eq!(before[0].start + 1, after[0].start);
        assert_eq!(before[0].end + 1, after[0].end);
    }
}
