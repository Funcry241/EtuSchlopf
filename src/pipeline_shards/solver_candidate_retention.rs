fn solver_pool_source_row(candidate: &SolverCandidate) -> crate::source_selection::SourceSelectionCandidateAuditRow {
        let stats = &candidate.midi_seed_stats;
        crate::source_selection::SourceSelectionCandidateAuditRow {
            candidate_id: stats.source_selection_candidate_id,
            origin_stage: stats.source_selection_origin_stage.clone(),
            profile_name: stats.source_selection_profile_name.clone(),
            span_limit: stats.profile_span_limit,
            jump_limit: stats.profile_jump_limit,
            smooth_jumps: stats.profile_smooth_jumps,
            left_high_soft_limit: stats.profile_left_high_soft_limit,
            right_low_soft_limit: stats.profile_right_low_soft_limit,
            hotspot_source_issue_count: stats.hotspot_plan_audit.source_issue_count,
            hotspot_group_count_before_truncation: stats.hotspot_plan_audit.group_count_before_truncation,
            hotspot_requested_limit: stats.hotspot_plan_audit.requested_limit,
            hotspot_truncated_count: stats.hotspot_plan_audit.truncated_count,
            hotspot_fingerprint_sha256: stats.hotspot_plan_audit.fingerprint_sha256.clone(),
            canonical_source_retention_ppm: stats.source_selection_full_causal.canonical_source_retention_ppm,
            shadow_plan_fingerprint_sha256: stats.source_selection_full_causal.shadow_plan_fingerprint_sha256.clone(),
            evaluation: stats.source_selection_candidate_evaluation.clone(),
        }
}

fn solver_pool_frontier_row(candidate: &SolverCandidate) -> crate::source_selection::SourceSelectionSolverFrontierRow {
        let evaluation = source_selection_candidate_evaluation(
            &candidate.final_result,
            candidate.fidelity.retained_note_percent,
            candidate.arrangement_stats.human_potential_notes_moved,
        );
        crate::source_selection::SourceSelectionSolverFrontierRow {
            solver_iteration: candidate.iteration,
            solver_profile_name: candidate.profile_name.clone(),
            solver_profile_lineage: format!("{:?}", candidate.profile_lineage),
            source_selection_candidate_id: candidate.midi_seed_stats.source_selection_candidate_id,
            evaluation,
            human_potential_notes_moved: candidate.arrangement_stats.human_potential_notes_moved,
            blockers: candidate.final_result.summary.blockers,
            warnings: candidate.final_result.summary.warnings,
            hints: candidate.final_result.summary.hints,
        }
    }

// Storage liveness only. Projection fields do not create candidate identities.
struct RetainedSolverPayload<T, P> {
    projection: P,
    full: Option<Box<T>>,
}
impl<T, P> RetainedSolverPayload<T, P> {
    fn from_parts(payload: T, projection: P, keep_full: bool) -> Self {
        let full = if keep_full { Some(Box::new(payload)) } else { None };
        Self { projection, full }
    }
    fn full(&self) -> &T {
        self.full.as_deref().expect("SubstanceGuard compatibility requires retained payload")
    }
    fn into_full(self) -> T {
        *self.full.expect("SubstanceGuard adoption requires retained payload")
    }
}

struct SolverPoolProjection {
    source: crate::source_selection::SourceSelectionCandidateAuditRow,
    frontier: crate::source_selection::SourceSelectionSolverFrontierRow,
    decision_key: CandidateDecisionKey,
}
type SolverPoolEntry = RetainedSolverPayload<SolverCandidate, SolverPoolProjection>;
impl SolverPoolEntry {
    fn from_candidate(candidate: SolverCandidate, preserve_incumbent: bool) -> Self {
        let keep_full = preserve_incumbent
            || (candidate.final_result.summary.blockers == 0
                && candidate.final_result.summary.warnings == 0);
        let projection = SolverPoolProjection {
            source: solver_pool_source_row(&candidate),
            frontier: solver_pool_frontier_row(&candidate),
            decision_key: candidate_decision_key(&candidate),
        };
        Self::from_parts(candidate, projection, keep_full)
    }
}
struct SolverCandidatePool {
    entries: Vec<SolverPoolEntry>,
}
impl SolverCandidatePool {
    fn from_candidates(candidates: Vec<SolverCandidate>) -> Self {
        let mut pool = Self { entries: Vec::with_capacity(candidates.len()) };
        pool.extend(candidates);
        pool
    }
    fn push(&mut self, candidate: SolverCandidate) {
        self.entries.push(SolverPoolEntry::from_candidate(candidate, false));
    }
    fn push_previous(&mut self, candidate: SolverCandidate) {
        // Conservative lifetime boundary: previously adopted incumbents stay full.
        self.entries.push(SolverPoolEntry::from_candidate(candidate, true));
    }
    fn extend(&mut self, candidates: impl IntoIterator<Item = SolverCandidate>) {
        for candidate in candidates { self.push(candidate); }
    }
    fn iter(&self) -> std::slice::Iter<'_, SolverPoolEntry> { self.entries.iter() }
    fn len(&self) -> usize { self.entries.len() }
    fn remove_full(&mut self, index: usize) -> SolverCandidate {
        self.entries.remove(index).into_full()
    }
}

#[cfg(test)]
mod solver_pool_retention_tests {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Facts { warnings: usize, blockers: usize, hints: usize, audit: String }
    fn key(warnings: usize) -> CandidateDecisionKey {
        CandidateDecisionKey { blockers: 0, critical_count: 0, max_critical_hardness: 0,
            warnings, hints: 0, retention_rank: std::cmp::Reverse(100), human_potential_notes_moved: 0 }
    }
    fn facts(warnings: usize, audit: &str) -> Facts {
        Facts { warnings, blockers: 0, hints: 0, audit: audit.to_string() }
    }
    fn entry(document: &str, facts: Facts) -> RetainedSolverPayload<String, Facts> {
        let keep = facts.blockers == 0 && facts.warnings == 0;
        RetainedSolverPayload::from_parts(document.to_string(), facts, keep)
    }
    fn eval(index: usize, entry: &RetainedSolverPayload<String, Facts>) -> SubstanceGuardEval {
        let compatible=entry.projection.blockers == 0 && entry.projection.warnings == 0;
        let visible=if compatible { visible_pitched_note_count(entry.full()) } else { 0 };
        SubstanceGuardEval { index, compatible, visible, hints: entry.projection.hints,
            decision_key: key(entry.projection.warnings) }
    }
    const TWO_NOTES: &str = "<measure><note><pitch><step>C</step><octave>4</octave></pitch></note><note><pitch><step>C</step><octave>4</octave></pitch></note></measure>";
    const THREE_NOTES: &str = "<measure><note><pitch><step>C</step><octave>4</octave></pitch></note><note><pitch><step>C</step><octave>4</octave></pitch></note><note><pitch><step>C</step><octave>4</octave></pitch></note></measure>";
    #[test]
    fn compatible_candidate_is_adoptable_with_exact_full_document() {
        let pool=[entry(TWO_NOTES,facts(0,"first")),entry(THREE_NOTES,facts(0,"second"))];
        let choice=substance_guard_choice(pool.iter().enumerate().map(|(i,e)|eval(i,e)),1,0,key(0));
        assert_eq!(choice,Some((1,3)));
        let [_,selected]=pool;
        assert_eq!(selected.into_full(),THREE_NOTES);
    }
    #[test]
    fn warning_candidate_that_would_visually_win_is_excluded_and_keeps_rows() {
        let rejected=entry(THREE_NOTES,facts(1,"exact rejected row"));
        assert!(rejected.full.is_none());
        assert_eq!(rejected.projection,facts(1,"exact rejected row"));
        // Even a hypothetical high visible count cannot override incompatibility.
        let outcome=substance_guard_choice([SubstanceGuardEval { index:0,compatible:false,
            visible:999,hints:0,decision_key:key(1) }],1,0,key(0));
        assert_eq!(outcome,None);
    }
    #[test]
    fn first_equal_compatible_candidate_wins_in_original_order() {
        let pool=[entry(TWO_NOTES,facts(0,"alpha")),entry(TWO_NOTES,facts(0,"beta"))];
        assert_eq!(substance_guard_choice(pool.iter().enumerate().map(|(i,e)|eval(i,e)),1,0,key(0)),Some((0,2)));
    }
    #[test]
    fn equal_visible_count_does_not_trigger_an_adoption() {
        assert_eq!(substance_guard_choice([eval(0,&entry(TWO_NOTES,facts(0,"row")))],2,0,key(0)),None);
    }
    #[test]
    fn full_and_projected_entries_preserve_interleaved_row_order() {
        let pool=[entry(TWO_NOTES,facts(1,"row0")),entry(TWO_NOTES,facts(0,"row1")),entry(THREE_NOTES,facts(1,"row2"))];
        let rows=pool.iter().map(|e|e.projection.audit.as_str()).collect::<Vec<_>>();
        assert_eq!(rows,["row0","row1","row2"]);
        assert_eq!(substance_guard_choice(pool.iter().enumerate().map(|(i,e)|eval(i,e)),1,0,key(0)),Some((1,2)));
    }
    #[test]
    fn independent_excluded_duplication_does_not_change_existing_choice() {
        let base=[eval(0,&entry(TWO_NOTES,facts(0,"eligible"))),eval(1,&entry(THREE_NOTES,facts(1,"excluded")))];
        let duplicated=[base[0].clone(),base[1].clone(),SubstanceGuardEval { index:2,..base[1].clone() }];
        assert_eq!(substance_guard_choice(base,1,0,key(0)),substance_guard_choice(duplicated,1,0,key(0)));
    }
    #[test]
    fn previous_incumbent_can_remain_full_despite_warning() {
        let previous=RetainedSolverPayload::from_parts(THREE_NOTES.to_string(),facts(1,"previous"),true);
        assert_eq!(previous.full(),THREE_NOTES);
        assert!(!eval(0,&previous).compatible);
    }
}
