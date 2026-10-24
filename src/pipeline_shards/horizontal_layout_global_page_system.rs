include!("renderer_exact_partition_search.rs");
// Joint global page+system reflow candidate and cutover validation.
//
// Candidate construction remains side-effect free. The selected PageCount
// preference candidate is already bound to final-notation demand, accepted
// capacity, the structural system-length bound and the observed vertical
// systems-per-page ceiling. This module converts that selected candidate into
// the single joint Page/System plan and validates its promotion into the
// existing HorizontalLayoutPlan authority.

#[derive(Debug, Clone, PartialEq, Eq)]
struct GlobalHorizontalPageSystemPlan {
    starts_new_system: Vec<bool>,
    starts_new_page: Vec<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GlobalHorizontalSystemLoadEvidence {
    start_measure: usize,
    end_measure: usize,
    load: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct GlobalHorizontalPageSystemPlanAudit {
    slots_compared: usize,
    preferred_pages: usize,
    selected_pages: usize,
    feasible_page_counts: Vec<usize>,
    authoritative_systems: usize,
    selected_systems: usize,
    authoritative_singletons: usize,
    selected_singletons: usize,
    system_break_changes: usize,
    page_break_changes: usize,
    added_system_breaks: usize,
    removed_system_breaks: usize,
    added_page_breaks: usize,
    removed_page_breaks: usize,
    authoritative_min_page_slots: usize,
    authoritative_max_page_slots: usize,
    selected_min_page_slots: usize,
    selected_max_page_slots: usize,
    authoritative_min_systems_per_page: usize,
    authoritative_max_systems_per_page: usize,
    selected_min_systems_per_page: usize,
    selected_max_systems_per_page: usize,
    max_authoritative_system_measures: usize,
    max_allowed_system_measures: usize,
    max_selected_system_measures: usize,
    vertical_max_systems_per_page: usize,
    accepted_system_capacity: usize,
    max_selected_system_load: usize,
    selected_system_load_evidence: Vec<GlobalHorizontalSystemLoadEvidence>,
    selected_measure_demand_evidence: Vec<usize>,
    selected_measure_clearance_edge_evidence: Vec<usize>,
    forced_over_capacity_singletons: usize,
    selected_nonterminal_thin_pages: usize,
    selected_avoidable_thin_pages: usize,
}

const FINAL_SELECTED_PLAN_DEMAND_MESSAGE_PREFIX: &str =
    "HorizontalLayoutFinalSelectedPlanDemandBinding:";

impl GlobalHorizontalPageSystemPlanAudit {
    fn selected_system_load_projection(&self) -> String {
        if self.selected_system_load_evidence.is_empty() {
            return "none".to_string();
        }
        self.selected_system_load_evidence
            .iter()
            .map(|row| {
                if row.start_measure == row.end_measure {
                    format!("{}={}", row.start_measure, row.load)
                } else {
                    format!("{}-{}={}", row.start_measure, row.end_measure, row.load)
                }
            })
            .collect::<Vec<_>>()
            .join(",")
    }

    fn selected_measure_demand_projection(&self) -> String {
        if self.selected_measure_demand_evidence.is_empty() {
            return "none".to_string();
        }
        self.selected_measure_demand_evidence
            .iter()
            .enumerate()
            .map(|(index, demand)| format!("{}={}", index + 1, demand))
            .collect::<Vec<_>>()
            .join(",")
    }

    fn final_selected_plan_demand_binding_projection(
        &self,
        plan: &HorizontalLayoutPlan,
    ) -> anyhow::Result<String> {
        if self.accepted_system_capacity == 0
            || self.selected_measure_demand_evidence.is_empty()
            || self.selected_measure_demand_evidence.len() != plan.slots.len()
            || self.selected_measure_clearance_edge_evidence.len() != plan.slots.len()
        {
            anyhow::bail!(
                "HorizontalLayoutFinalSelectedPlanDemandBinding: Planner-Demands, Capacity und finaler Plan besitzen keine exakte Slotbindung"
            );
        }

        let mut system_starts = vec![0usize];
        for (index, slot) in plan.slots.iter().enumerate().skip(1) {
            if slot.directives.starts_new_page || slot.directives.starts_new_system {
                system_starts.push(index);
            }
        }

        let mut selected_system_loads = Vec::with_capacity(system_starts.len());
        for (system_index, start) in system_starts.iter().copied().enumerate() {
            let end = system_starts
                .get(system_index + 1)
                .copied()
                .unwrap_or(plan.slots.len());
            if start >= end {
                anyhow::bail!(
                    "HorizontalLayoutFinalSelectedPlanDemandBinding: ungueltige finale Systempartition"
                );
            }
            let load = self.selected_measure_demand_evidence[start..end]
                .iter()
                .copied()
                .fold(0usize, usize::saturating_add);
            if load > self.accepted_system_capacity {
                anyhow::bail!(
                    "HorizontalLayoutFinalSelectedPlanDemandBinding: finales System {}-{} ueberschreitet gebundene Planner-Capacity {} mit Last {}",
                    start + 1,
                    end,
                    self.accepted_system_capacity,
                    load
                );
            }
            let range = if start + 1 == end {
                format!("{}", start + 1)
            } else {
                format!("{}-{}", start + 1, end)
            };
            selected_system_loads.push(format!("{range}={load}"));
        }

        Ok(format!(
            "{} AcceptedSystemCapacity {} | SelectedSystemLoads {} | SelectedMeasureDemands {} | Source existing_joint_planner_measure_demands_recomposed_to_final_selected_plan | MeasureDemandRecompose exact | ModelRecompute nein | ThresholdChange nein | PolicyWrite nein",
            FINAL_SELECTED_PLAN_DEMAND_MESSAGE_PREFIX,
            self.accepted_system_capacity,
            selected_system_loads.join(","),
            self.selected_measure_demand_projection(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JointPageSystemCandidate {
    page_system_lengths: Vec<Vec<usize>>,
    page_slot_lengths: Vec<usize>,
    systems: usize,
    singletons: usize,
    forced_over_capacity_singletons: usize,
    system_count_balance_penalty: u128,
    page_slot_balance_penalty: u128,
    slack_square_sum: u128,
    max_system_load: usize,
}

impl JointPageSystemCandidate {
    fn empty() -> Self {
        Self {
            page_system_lengths: Vec::new(),
            page_slot_lengths: Vec::new(),
            systems: 0,
            singletons: 0,
            forced_over_capacity_singletons: 0,
            system_count_balance_penalty: 0,
            page_slot_balance_penalty: 0,
            slack_square_sum: 0,
            max_system_load: 0,
        }
    }

    fn is_better_than(&self, other: &Self) -> bool {
        (
            self.systems,
            self.singletons,
            self.forced_over_capacity_singletons,
            self.system_count_balance_penalty,
            self.page_slot_balance_penalty,
            self.slack_square_sum,
            &self.page_slot_lengths,
            &self.page_system_lengths,
        ) < (
            other.systems,
            other.singletons,
            other.forced_over_capacity_singletons,
            other.system_count_balance_penalty,
            other.page_slot_balance_penalty,
            other.slack_square_sum,
            &other.page_slot_lengths,
            &other.page_system_lengths,
        )
    }

    fn pages(&self) -> usize {
        self.page_slot_lengths.len()
    }

    fn page_count_preference_candidate(&self) -> PageCountPreferenceCandidate {
        PageCountPreferenceCandidate {
            pages: self.pages(),
            page_slot_lengths: self.page_slot_lengths.clone(),
            page_system_lengths: self.page_system_lengths.clone(),
            systems: self.systems,
            singletons: self.singletons,
            forced_over_capacity_singletons: self.forced_over_capacity_singletons,
            system_count_balance_penalty: self.system_count_balance_penalty,
            page_slot_balance_penalty: self.page_slot_balance_penalty,
            slack_square_sum: self.slack_square_sum,
            max_system_load: self.max_system_load,
        }
    }

    fn is_preference_better_than(&self, other: &Self, preferred_pages: usize) -> bool {
        self.page_count_preference_candidate().is_preference_better_than(
            &other.page_count_preference_candidate(),
            preferred_pages,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JointPageSystemSegmentPartition {
    system_lengths: Vec<usize>,
    systems: usize,
    singletons: usize,
    forced_over_capacity_singletons: usize,
    slack_square_sum: u128,
    max_system_load: usize,
}

impl JointPageSystemSegmentPartition {
    fn empty() -> Self {
        Self {
            system_lengths: Vec::new(),
            systems: 0,
            singletons: 0,
            forced_over_capacity_singletons: 0,
            slack_square_sum: 0,
            max_system_load: 0,
        }
    }

    fn is_better_than(&self, other: &Self) -> bool {
        (
            self.systems,
            self.singletons,
            self.forced_over_capacity_singletons,
            self.slack_square_sum,
            &self.system_lengths,
        ) < (
            other.systems,
            other.singletons,
            other.forced_over_capacity_singletons,
            other.slack_square_sum,
            &other.system_lengths,
        )
    }
}

fn joint_page_system_segment_partition(
    demands: &[usize],
    capacity: usize,
    max_system_measures: usize,
) -> anyhow::Result<JointPageSystemSegmentPartition> {
    if demands.is_empty() || max_system_measures == 0 {
        anyhow::bail!("GlobalHorizontalPageSystemPlan: ungueltige Segment-Partition");
    }

    let mut suffix: Vec<Option<JointPageSystemSegmentPartition>> = vec![None; demands.len() + 1];
    suffix[demands.len()] = Some(JointPageSystemSegmentPartition::empty());

    for start in (0..demands.len()).rev() {
        let max_len = max_system_measures.min(demands.len() - start);
        let mut best: Option<JointPageSystemSegmentPartition> = None;
        let mut load = 0usize;

        for len in 1..=max_len {
            load = load.saturating_add(demands[start + len - 1]);
            if len > 1 && load > capacity {
                continue;
            }
            let Some(tail) = suffix[start + len].as_ref() else {
                continue;
            };

            let slack = capacity.saturating_sub(load) as u128;
            let mut system_lengths = Vec::with_capacity(tail.system_lengths.len() + 1);
            system_lengths.push(len);
            system_lengths.extend(tail.system_lengths.iter().copied());
            let candidate = JointPageSystemSegmentPartition {
                system_lengths,
                systems: tail.systems + 1,
                singletons: tail.singletons + usize::from(len == 1),
                forced_over_capacity_singletons: tail.forced_over_capacity_singletons
                    + usize::from(len == 1 && load > capacity),
                slack_square_sum: tail
                    .slack_square_sum
                    .saturating_add(slack.saturating_mul(slack)),
                max_system_load: tail.max_system_load.max(load),
            };
            if best
                .as_ref()
                .is_none_or(|current| candidate.is_better_than(current))
            {
                best = Some(candidate);
            }
        }
        suffix[start] = best;
    }

    suffix[0].clone().ok_or_else(|| {
        anyhow::anyhow!(
            "GlobalHorizontalPageSystemPlan: keine zulaessige Systempartition fuer Page-Segment"
        )
    })
}

struct GlobalHorizontalAuthoritativePageShape {
    page_starts: Vec<usize>,
    page_system_lengths: Vec<Vec<usize>>,
    min_page_slots: usize,
    max_page_slots: usize,
    min_systems_per_page: usize,
    max_systems_per_page: usize,
    max_system_measures: usize,
}

fn authoritative_page_shape(
    plan: &HorizontalLayoutPlan,
) -> anyhow::Result<GlobalHorizontalAuthoritativePageShape> {
    let page_starts = global_reflow_page_starts(plan);
    if page_starts.is_empty() {
        anyhow::bail!("GlobalHorizontalPageSystemPlan: keine autoritativen Seiten");
    }

    let mut page_system_lengths = Vec::with_capacity(page_starts.len());
    let mut min_page_slots = usize::MAX;
    let mut max_page_slots = 0usize;
    let mut min_systems_per_page = usize::MAX;
    let mut max_systems_per_page = 0usize;
    let mut max_system_measures = 0usize;

    for (page_index, page_start) in page_starts.iter().copied().enumerate() {
        let page_end = page_starts
            .get(page_index + 1)
            .copied()
            .unwrap_or(plan.slots.len());
        if page_start >= page_end {
            anyhow::bail!("GlobalHorizontalPageSystemPlan: ungueltige autoritative Page-Grenze");
        }
        let lengths = global_reflow_baseline_system_lengths_for_page(plan, page_start, page_end);
        min_page_slots = min_page_slots.min(page_end - page_start);
        max_page_slots = max_page_slots.max(page_end - page_start);
        min_systems_per_page = min_systems_per_page.min(lengths.len());
        max_systems_per_page = max_systems_per_page.max(lengths.len());
        max_system_measures = max_system_measures.max(lengths.iter().copied().max().unwrap_or(0));
        page_system_lengths.push(lengths);
    }

    Ok(GlobalHorizontalAuthoritativePageShape {
        page_starts,
        page_system_lengths,
        min_page_slots,
        max_page_slots,
        min_systems_per_page,
        max_systems_per_page,
        max_system_measures: max_system_measures.max(1),
    })
}


fn build_global_horizontal_page_system_plan(
    final_notation_xml: &str,
    authoritative_plan: &HorizontalLayoutPlan,
    preference_audit: &GlobalHorizontalPageCountPreferenceAudit,
) -> anyhow::Result<(
    GlobalHorizontalPageSystemPlan,
    GlobalHorizontalPageSystemPlanAudit,
)> {
    let facts = HorizontalLayoutFactSnapshot::capture_final_notation(final_notation_xml)?;
    let slots = authoritative_plan.slots.len();
    if slots == 0
        || facts.slots.len() != slots
        || preference_audit.slots_compared != slots
    {
        anyhow::bail!(
            "GlobalHorizontalPageSystemPlan: finale Facts, autoritativer Plan und Preference-Frontier besitzen keine exakte Slotbindung"
        );
    }

    let selected = preference_audit
        .selected_preference_candidate
        .as_ref()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "GlobalHorizontalPageSystemPlan: keine autorisierbare PageCount-Preference-Auswahl"
            )
        })?;
    if selected.pages == 0
        || selected.pages > preference_audit.preferred_pages
        || selected.pages < preference_audit.first_tested_pages
        || selected.page_slot_lengths.len() != selected.pages
        || selected.page_system_lengths.len() != selected.pages
        || selected.forced_over_capacity_singletons != 0
        || !preference_audit
            .feasible_candidates
            .iter()
            .any(|candidate| candidate == selected)
    {
        anyhow::bail!(
            "GlobalHorizontalPageSystemPlan: Preference-Auswahl ist nicht exakt an die harte FullFrontier gebunden"
        );
    }

    let demands = facts
        .slots
        .iter()
        .map(global_reflow_measure_demand)
        .collect::<Vec<_>>();
    let authoritative_shape = authoritative_page_shape(authoritative_plan)?;
    let authoritative_systems = authoritative_shape
        .page_system_lengths
        .iter()
        .map(Vec::len)
        .sum::<usize>();
    let authoritative_singletons = authoritative_shape
        .page_system_lengths
        .iter()
        .flat_map(|lengths| lengths.iter())
        .filter(|length| **length == 1)
        .count();

    let GlobalHorizontalAuthoritativePageShape {
        page_starts: authoritative_page_starts,
        min_page_slots: authoritative_min_page_slots,
        max_page_slots: authoritative_max_page_slots,
        min_systems_per_page: authoritative_min_systems_per_page,
        max_systems_per_page: authoritative_max_systems_per_page,
        max_system_measures: max_authoritative_system_measures,
        ..
    } = authoritative_shape;

    let mut selected_plan = GlobalHorizontalPageSystemPlan {
        starts_new_system: vec![false; slots],
        starts_new_page: vec![false; slots],
    };
    let mut cursor = 0usize;
    let mut recomputed_systems = 0usize;
    let mut recomputed_singletons = 0usize;
    let mut recomputed_max_system_load = 0usize;
    let mut recomputed_max_system_measures = 0usize;
    let mut selected_system_load_evidence = Vec::new();

    for (page_index, system_lengths) in selected.page_system_lengths.iter().enumerate() {
        let page_slots = selected.page_slot_lengths[page_index];
        if page_slots == 0 || system_lengths.is_empty() {
            anyhow::bail!(
                "GlobalHorizontalPageSystemPlan: ausgewaehlte Page/System-Partition enthaelt leere Segmente"
            );
        }
        if page_index > 0 {
            if cursor >= slots {
                anyhow::bail!(
                    "GlobalHorizontalPageSystemPlan: Page-Start liegt ausserhalb der Slotbindung"
                );
            }
            selected_plan.starts_new_page[cursor] = true;
        }

        let page_end = cursor.saturating_add(page_slots);
        if page_end > slots {
            anyhow::bail!(
                "GlobalHorizontalPageSystemPlan: Page-Partition ueberschreitet die Slotbindung"
            );
        }
        let loads =
            global_reflow_system_loads(&demands[cursor..page_end], system_lengths);
        if loads.len() != system_lengths.len() {
            anyhow::bail!(
                "GlobalHorizontalPageSystemPlan: System-Load-Readback driftet von der ausgewaehlten Partition"
            );
        }

        let mut system_cursor = cursor;
        for (system_index, length) in system_lengths.iter().copied().enumerate() {
            if length == 0
                || length > preference_audit.max_system_measures
                || system_cursor.saturating_add(length) > page_end
            {
                anyhow::bail!(
                    "GlobalHorizontalPageSystemPlan: ausgewaehlte Systempartition verletzt den strukturellen Systemlaengenvertrag"
                );
            }
            let load = loads[system_index];
            if load > preference_audit.accepted_system_capacity {
                anyhow::bail!(
                    "GlobalHorizontalPageSystemPlan: ausgewaehlte Systemlast {} ueberschreitet akzeptierte Kapazitaet {}",
                    load,
                    preference_audit.accepted_system_capacity
                );
            }
            if system_index > 0 {
                selected_plan.starts_new_system[system_cursor] = true;
            }
            recomputed_systems = recomputed_systems.saturating_add(1);
            recomputed_singletons =
                recomputed_singletons.saturating_add(usize::from(length == 1));
            recomputed_max_system_load = recomputed_max_system_load.max(load);
            recomputed_max_system_measures = recomputed_max_system_measures.max(length);
            selected_system_load_evidence.push(GlobalHorizontalSystemLoadEvidence {
                start_measure: system_cursor.saturating_add(1),
                end_measure: system_cursor.saturating_add(length),
                load,
            });
            system_cursor = system_cursor.saturating_add(length);
        }
        if system_cursor != page_end {
            anyhow::bail!(
                "GlobalHorizontalPageSystemPlan: Systempartition deckt Page-Slots nicht exakt ab"
            );
        }
        cursor = page_end;
    }
    if cursor != slots
        || recomputed_systems != selected.systems
        || recomputed_singletons != selected.singletons
        || recomputed_max_system_load != selected.max_system_load
        || selected_system_load_evidence.len() != selected.systems
        || selected.max_system_load > preference_audit.accepted_system_capacity
        || selected.max_systems_per_page() > preference_audit.vertical_max_systems_per_page
        || recomputed_max_system_measures > preference_audit.max_system_measures
        || selected.systems > authoritative_systems
        || selected.singletons > authoritative_singletons
    {
        anyhow::bail!(
            "GlobalHorizontalPageSystemPlan: Preference-Auswahl verletzt harte Capacity/Shape-Invarianten"
        );
    }

    let mut audit = GlobalHorizontalPageSystemPlanAudit {
        slots_compared: slots,
        preferred_pages: preference_audit.preferred_pages,
        selected_pages: selected.pages,
        feasible_page_counts: preference_audit.feasible_page_counts.clone(),
        authoritative_systems,
        selected_systems: selected.systems,
        authoritative_singletons,
        selected_singletons: selected.singletons,
        authoritative_min_page_slots,
        authoritative_max_page_slots,
        selected_min_page_slots: selected.min_page_slots(),
        selected_max_page_slots: selected.max_page_slots(),
        authoritative_min_systems_per_page,
        authoritative_max_systems_per_page,
        selected_min_systems_per_page: selected.min_systems_per_page(),
        selected_max_systems_per_page: selected.max_systems_per_page(),
        max_authoritative_system_measures,
        max_allowed_system_measures: preference_audit.max_system_measures,
        max_selected_system_measures: recomputed_max_system_measures,
        vertical_max_systems_per_page: preference_audit.vertical_max_systems_per_page,
        accepted_system_capacity: preference_audit.accepted_system_capacity,
        max_selected_system_load: recomputed_max_system_load,
        selected_system_load_evidence,
        selected_measure_demand_evidence: demands,
        selected_measure_clearance_edge_evidence: facts
            .slots
            .iter()
            .map(|slot| slot.following_accidental_clearance_edges)
            .collect(),
        forced_over_capacity_singletons: selected.forced_over_capacity_singletons,
        selected_nonterminal_thin_pages: selected.nonterminal_thin_pages(),
        selected_avoidable_thin_pages: selected.avoidable_thin_pages(),
        ..GlobalHorizontalPageSystemPlanAudit::default()
    };

    for slot_index in 1..slots {
        let old_page = authoritative_plan.slots[slot_index].directives.starts_new_page;
        let new_page = selected_plan.starts_new_page[slot_index];
        if old_page != new_page {
            audit.page_break_changes += 1;
            if new_page {
                audit.added_page_breaks += 1;
            } else {
                audit.removed_page_breaks += 1;
            }
        }
        let old_system = authoritative_plan.slots[slot_index].directives.starts_new_system;
        let new_system = selected_plan.starts_new_system[slot_index];
        if old_system != new_system {
            audit.system_break_changes += 1;
            if new_system {
                audit.added_system_breaks += 1;
            } else {
                audit.removed_system_breaks += 1;
            }
        }
        if new_page && new_system {
            anyhow::bail!(
                "GlobalHorizontalPageSystemPlan: Page- und Systembreak duerfen nicht denselben Slot besitzen"
            );
        }
    }

    if (authoritative_page_starts.len() != preference_audit.preferred_pages
        && authoritative_page_starts.len() != selected.pages)
        || audit.selected_max_systems_per_page > audit.vertical_max_systems_per_page
        || audit.max_selected_system_measures > audit.max_allowed_system_measures
        || audit.max_selected_system_load > audit.accepted_system_capacity
        || audit.forced_over_capacity_singletons != 0
    {
        anyhow::bail!(
            "GlobalHorizontalPageSystemPlan: ausgewaehlter Joint-Plan verletzt die Preference-Frontier-Hard-Invarianten"
        );
    }

    Ok((selected_plan, audit))
}


fn validate_global_horizontal_page_system_reflow_cutover(
    authoritative_plan: &HorizontalLayoutPlan,
    reflow_plan: &GlobalHorizontalPageSystemPlan,
    audit: &GlobalHorizontalPageSystemPlanAudit,
) -> anyhow::Result<()> {
    let slots = authoritative_plan.slots.len();
    if slots == 0
        || reflow_plan.starts_new_system.len() != slots
        || reflow_plan.starts_new_page.len() != slots
        || audit.slots_compared != slots
    {
        anyhow::bail!(
            "GlobalHorizontalPageSystemReflowCutover: Slotbindung zwischen autoritativem Plan, Candidate und Audit ist unvollstaendig"
        );
    }
    if audit.preferred_pages == 0
        || audit.selected_pages == 0
        || audit.selected_pages > audit.preferred_pages
        || audit.feasible_page_counts.is_empty()
        || !audit.feasible_page_counts.contains(&audit.selected_pages)
        || audit
            .feasible_page_counts
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
        || audit
            .feasible_page_counts
            .iter()
            .any(|pages| *pages == 0 || *pages > audit.preferred_pages)
        || audit.selected_systems > audit.authoritative_systems
        || audit.selected_singletons > audit.authoritative_singletons
        || audit.forced_over_capacity_singletons != 0
        || audit.max_selected_system_measures > audit.max_allowed_system_measures
        || audit.max_selected_system_load > audit.accepted_system_capacity
        || audit.selected_max_systems_per_page > audit.vertical_max_systems_per_page
    {
        anyhow::bail!(
            "GlobalHorizontalPageSystemReflowCutover: Candidate verletzt die gebundenen Preference-Frontier-Hard-Invarianten"
        );
    }

    let candidate_pages = 1usize.saturating_add(
        reflow_plan
            .starts_new_page
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, starts)| **starts)
            .count(),
    );
    if candidate_pages != audit.selected_pages {
        anyhow::bail!(
            "GlobalHorizontalPageSystemReflowCutover: Candidate-Seitenzahl {} driftet von ausgewaehlter Preference-Seitenzahl {}",
            candidate_pages,
            audit.selected_pages
        );
    }

    let mut system_break_changes = 0usize;
    let mut page_break_changes = 0usize;
    let mut added_system_breaks = 0usize;
    let mut removed_system_breaks = 0usize;
    let mut added_page_breaks = 0usize;
    let mut removed_page_breaks = 0usize;

    for slot_index in 0..slots {
        let new_page = reflow_plan.starts_new_page[slot_index];
        let new_system = reflow_plan.starts_new_system[slot_index];
        if new_page && new_system {
            anyhow::bail!(
                "GlobalHorizontalPageSystemReflowCutover: Page- und Systembreak duerfen nicht denselben Slot besitzen"
            );
        }
        if slot_index == 0 && (new_page || new_system) {
            anyhow::bail!(
                "GlobalHorizontalPageSystemReflowCutover: erster struktureller Slot darf keinen expliziten Break tragen"
            );
        }

        let old_page = authoritative_plan.slots[slot_index]
            .directives
            .starts_new_page;
        if old_page != new_page {
            page_break_changes += 1;
            if new_page {
                added_page_breaks += 1;
            } else {
                removed_page_breaks += 1;
            }
        }

        let old_system = authoritative_plan.slots[slot_index]
            .directives
            .starts_new_system;
        if old_system != new_system {
            system_break_changes += 1;
            if new_system {
                added_system_breaks += 1;
            } else {
                removed_system_breaks += 1;
            }
        }
    }

    if system_break_changes != audit.system_break_changes
        || page_break_changes != audit.page_break_changes
        || added_system_breaks != audit.added_system_breaks
        || removed_system_breaks != audit.removed_system_breaks
        || added_page_breaks != audit.added_page_breaks
        || removed_page_breaks != audit.removed_page_breaks
    {
        anyhow::bail!(
            "GlobalHorizontalPageSystemReflowCutover: Candidate-Diff stimmt nicht exakt mit der gebundenen Preference-Evidence ueberein"
        );
    }

    Ok(())
}


impl HorizontalLayoutPlan {
    fn with_global_page_system_reflow(
        &self,
        reflow_plan: &GlobalHorizontalPageSystemPlan,
        audit: &GlobalHorizontalPageSystemPlanAudit,
    ) -> anyhow::Result<Self> {
        validate_global_horizontal_page_system_reflow_cutover(self, reflow_plan, audit)?;

        let mut promoted = self.clone();
        for (slot_index, slot) in promoted.slots.iter_mut().enumerate() {
            slot.directives.starts_new_page = reflow_plan.starts_new_page[slot_index];
            slot.directives.starts_new_system = reflow_plan.starts_new_system[slot_index];
            slot.page_owner = Some(HorizontalLayoutDecisionOwner::GlobalPageSystemReflow);
            slot.system_owner = Some(HorizontalLayoutDecisionOwner::GlobalPageSystemReflow);
        }

        if !promoted.provenance_complete() {
            anyhow::bail!(
                "GlobalHorizontalPageSystemReflowCutover: promovierter Plan besitzt unvollstaendige Provenienz"
            );
        }
        Ok(promoted)
    }
}

fn renderer_clearance_system_has_structural_slack(
    clearance_edges: &[usize],
    system_length: usize,
    max_system_measures: usize,
) -> bool {
    if clearance_edges.iter().all(|edges| *edges == 0) || max_system_measures <= 1 {
        return true;
    }
    system_length < max_system_measures
}

fn renderer_feasibility_constraint_segment_partition(
    demands: &[usize],
    clearance_edges: &[usize],
    global_start_measure: usize,
    capacity: usize,
    max_system_measures: usize,
    no_goods: &std::collections::BTreeSet<(usize, usize)>,
) -> anyhow::Result<JointPageSystemSegmentPartition> {
    if demands.is_empty()
        || demands.len() != clearance_edges.len()
        || max_system_measures == 0
        || global_start_measure == 0
    {
        anyhow::bail!(
            "RendererFeasibilityBoundedJointPlanCutover: ungueltige Segment-Basis"
        );
    }

    let mut suffix: Vec<Option<JointPageSystemSegmentPartition>> =
        vec![None; demands.len() + 1];
    suffix[demands.len()] = Some(JointPageSystemSegmentPartition::empty());

    for start in (0..demands.len()).rev() {
        let max_len = max_system_measures.min(demands.len() - start);
        let mut best: Option<JointPageSystemSegmentPartition> = None;
        let mut load = 0usize;

        for len in 1..=max_len {
            load = load.saturating_add(demands[start + len - 1]);
            if len > 1 && load > capacity {
                continue;
            }
            if !renderer_clearance_system_has_structural_slack(
                &clearance_edges[start..start + len],
                len,
                max_system_measures,
            ) {
                continue;
            }
            let start_measure = global_start_measure.saturating_add(start);
            let end_measure = start_measure.saturating_add(len).saturating_sub(1);
            if no_goods.contains(&(start_measure, end_measure)) {
                continue;
            }
            let Some(tail) = suffix[start + len].as_ref() else {
                continue;
            };
            let slack = capacity.saturating_sub(load) as u128;
            let mut system_lengths = Vec::with_capacity(tail.system_lengths.len() + 1);
            system_lengths.push(len);
            system_lengths.extend(tail.system_lengths.iter().copied());
            let candidate = JointPageSystemSegmentPartition {
                system_lengths,
                systems: tail.systems + 1,
                singletons: tail.singletons + usize::from(len == 1),
                forced_over_capacity_singletons: tail.forced_over_capacity_singletons
                    + usize::from(len == 1 && load > capacity),
                slack_square_sum: tail
                    .slack_square_sum
                    .saturating_add(slack.saturating_mul(slack)),
                max_system_load: tail.max_system_load.max(load),
            };
            if best
                .as_ref()
                .is_none_or(|current| candidate.is_better_than(current))
            {
                best = Some(candidate);
            }
        }
        suffix[start] = best;
    }

    suffix[0].clone().ok_or_else(|| {
        anyhow::anyhow!(
            "RendererFeasibilityBoundedJointPlanCutover: keine zulaessige Segmentpartition"
        )
    })
}

fn build_renderer_feasibility_constraint_candidate(
    measure_demands: &[usize],
    measure_clearance_edges: &[usize],
    selected_pages: usize,
    system_constraints: (usize, usize),
    vertical_max_systems_per_page: usize,
    reference_systems: usize,
    no_goods: &std::collections::BTreeSet<(usize, usize)>,
) -> anyhow::Result<JointPageSystemCandidate> {
    let (accepted_system_capacity, max_system_measures) = system_constraints;
    if measure_demands.is_empty()
        || measure_demands.len() != measure_clearance_edges.len()
        || selected_pages == 0
        || accepted_system_capacity == 0
        || max_system_measures == 0
        || vertical_max_systems_per_page == 0
        || reference_systems == 0
    {
        anyhow::bail!(
            "RendererFeasibilityBoundedJointPlanCutover: ungueltige Candidate-Basis"
        );
    }

    let slots = measure_demands.len();
    let mut dp: Vec<Vec<Option<JointPageSystemCandidate>>> =
        vec![vec![None; slots + 1]; selected_pages + 1];
    dp[0][0] = Some(JointPageSystemCandidate::empty());

    for page_index in 0..selected_pages {
        for start in 0..=slots {
            let Some(prefix) = dp[page_index][start].clone() else {
                continue;
            };
            let remaining_pages_after = selected_pages - page_index - 1;
            let min_end = start.saturating_add(1);
            let max_end = slots.saturating_sub(remaining_pages_after);
            if min_end > max_end {
                continue;
            }
            for end in min_end..=max_end {
                let segment = renderer_feasibility_constraint_segment_partition(
                    &measure_demands[start..end],
                    &measure_clearance_edges[start..end],
                    start + 1,
                    accepted_system_capacity,
                    max_system_measures,
                    no_goods,
                )?;
                if segment.systems > vertical_max_systems_per_page {
                    continue;
                }

                let mut candidate = prefix.clone();
                candidate.systems = candidate.systems.saturating_add(segment.systems);
                candidate.singletons = candidate.singletons.saturating_add(segment.singletons);
                candidate.forced_over_capacity_singletons = candidate
                    .forced_over_capacity_singletons
                    .saturating_add(segment.forced_over_capacity_singletons);
                let system_delta = segment
                    .systems
                    .saturating_mul(selected_pages)
                    .abs_diff(reference_systems) as u128;
                candidate.system_count_balance_penalty = candidate
                    .system_count_balance_penalty
                    .saturating_add(system_delta.saturating_mul(system_delta));
                let page_slots = end - start;
                let slot_delta = page_slots
                    .saturating_mul(selected_pages)
                    .abs_diff(slots) as u128;
                candidate.page_slot_balance_penalty = candidate
                    .page_slot_balance_penalty
                    .saturating_add(slot_delta.saturating_mul(slot_delta));
                candidate.slack_square_sum = candidate
                    .slack_square_sum
                    .saturating_add(segment.slack_square_sum);
                candidate.max_system_load = candidate.max_system_load.max(segment.max_system_load);
                candidate.page_slot_lengths.push(page_slots);
                candidate.page_system_lengths.push(segment.system_lengths);

                let replace = dp[page_index + 1][end]
                    .as_ref()
                    .is_none_or(|current| candidate.is_better_than(current));
                if replace {
                    dp[page_index + 1][end] = Some(candidate);
                }
            }
        }
    }

    dp[selected_pages][slots].clone().ok_or_else(|| {
        anyhow::anyhow!(
            "RendererFeasibilityBoundedJointPlanCutover: keine zulaessige gemeinsame Page/System-Partition"
        )
    })
}

struct RendererFeasibilityFrontierRequest<'a> {
    measure_demands: &'a [usize],
    measure_clearance_edges: &'a [usize],
    feasible_page_counts: &'a [usize],
    preferred_pages: usize,
    accepted_system_capacity: usize,
    max_system_measures: usize,
    vertical_max_systems_per_page: usize,
    reference_systems: usize,
    no_goods: &'a std::collections::BTreeSet<(usize, usize)>,
    rejected_partitions: &'a RendererRejectedPartitions,
}

fn build_renderer_feasibility_frontier_candidate(
    request: &RendererFeasibilityFrontierRequest<'_>,
) -> anyhow::Result<JointPageSystemCandidate> {
    let measure_demands = request.measure_demands;
    let measure_clearance_edges = request.measure_clearance_edges;
    let feasible_page_counts = request.feasible_page_counts;
    let preferred_pages = request.preferred_pages;
    let accepted_system_capacity = request.accepted_system_capacity;
    let max_system_measures = request.max_system_measures;
    let vertical_max_systems_per_page = request.vertical_max_systems_per_page;
    let reference_systems = request.reference_systems;
    let no_goods = request.no_goods;
    if measure_demands.is_empty()
        || measure_demands.len() != measure_clearance_edges.len()
        || feasible_page_counts.is_empty()
        || preferred_pages == 0
        || accepted_system_capacity == 0
        || max_system_measures == 0
        || vertical_max_systems_per_page == 0
        || reference_systems == 0
        || feasible_page_counts
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
        || feasible_page_counts
            .iter()
            .any(|pages| *pages == 0 || *pages > preferred_pages)
    {
        anyhow::bail!(
            "RendererFeasibilityFrontierCutover: ungueltige gebundene PageCount-Frontier"
        );
    }

    let mut best: Option<JointPageSystemCandidate> = None;
    for pages in feasible_page_counts.iter().copied() {
        let candidate_result = if request.rejected_partitions.is_empty() {
            build_renderer_feasibility_constraint_candidate(
            measure_demands,
            measure_clearance_edges,
            pages,
            (accepted_system_capacity, max_system_measures),
            vertical_max_systems_per_page,
            reference_systems,
            no_goods,
        )
        } else { renderer_exact_partition_candidate(request, pages) };
        let candidate = match candidate_result {
            Ok(candidate) => candidate,
            Err(error)
                if error.to_string()
                    == "RendererFeasibilityBoundedJointPlanCutover: keine zulaessige gemeinsame Page/System-Partition" =>
            {
                continue;
            }
            Err(error) => return Err(error),
        };
        if best
            .as_ref()
            .is_none_or(|current| candidate.is_preference_better_than(current, preferred_pages))
        {
            best = Some(candidate);
        }
    }

    best.ok_or_else(|| {
        anyhow::anyhow!(
            "RendererFeasibilityFrontierCutover: keine zulaessige gemeinsame Page/System-Partition ueber gebundene PageCount-Frontier {:?}",
            feasible_page_counts
        )
    })
}

#[derive(Debug, Clone)]
enum RendererFeasibilityOracleOutcome {
    NotRequired,
    Confirmed {
        plan: HorizontalLayoutPlan,
        rounds: usize,
        no_goods: std::collections::BTreeSet<(usize, usize)>,
        source_fingerprint: u64,
    },
    Rejected {
        reason: String,
    },
}

fn renderer_feasibility_source_fingerprint(xml: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in xml.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn select_renderer_feasibility_productive_plan(
    initial_plan: &HorizontalLayoutPlan,
    outcome: &RendererFeasibilityOracleOutcome,
    committed_source_xml: &str,
) -> anyhow::Result<HorizontalLayoutPlan> {
    match outcome {
        RendererFeasibilityOracleOutcome::NotRequired => Ok(initial_plan.clone()),
        RendererFeasibilityOracleOutcome::Confirmed {
            plan,
            source_fingerprint,
            ..
        } => {
            let current_fingerprint =
                renderer_feasibility_source_fingerprint(committed_source_xml);
            if current_fingerprint != *source_fingerprint || !plan.provenance_complete() {
                anyhow::bail!(
                    "RendererFeasibilityProductiveAdoption: bestaetigter Plan ist nicht mehr exakt an die committed Pre-Layout-Quelle gebunden"
                );
            }
            Ok(plan.clone())
        }
        RendererFeasibilityOracleOutcome::Rejected { reason } => {
            anyhow::bail!(
                "RendererFeasibilityProductiveAdoption: Renderer-Oracle nicht bestaetigt: {}",
                reason
            )
        }
    }
}

fn promote_renderer_feasibility_constraint_candidate(
    initial_plan: &HorizontalLayoutPlan,
    candidate: &JointPageSystemCandidate,
    basis: &GlobalHorizontalPageSystemPlanAudit,
    no_goods: &std::collections::BTreeSet<(usize, usize)>,
) -> anyhow::Result<HorizontalLayoutPlan> {
    let slots = initial_plan.slots.len();
    let candidate_pages = candidate.pages();
    if slots == 0
        || candidate_pages == 0
        || candidate.page_system_lengths.len() != candidate_pages
        || !basis.feasible_page_counts.contains(&candidate_pages)
        || candidate.page_slot_lengths.iter().copied().sum::<usize>() != slots
        || basis.selected_measure_clearance_edge_evidence.len() != slots
        || candidate.forced_over_capacity_singletons != 0
        || candidate.max_system_load > basis.accepted_system_capacity
        || candidate
            .page_system_lengths
            .iter()
            .map(Vec::len)
            .max()
            .unwrap_or(0)
            > basis.vertical_max_systems_per_page
        || candidate
            .page_system_lengths
            .iter()
            .flat_map(|systems| systems.iter())
            .copied()
            .max()
            .unwrap_or(0)
            > basis.max_allowed_system_measures
    {
        anyhow::bail!(
            "RendererFeasibilityBoundedJointPlanCutover: Candidate verletzt gebundene Capacity/Shape-Invarianten"
        );
    }

    let mut promoted = initial_plan.clone();
    for slot in &mut promoted.slots {
        slot.directives.starts_new_page = false;
        slot.directives.starts_new_system = false;
        slot.page_owner = None;
        slot.system_owner = None;
    }

    let mut cursor = 0usize;
    let mut systems = 0usize;
    let mut singletons = 0usize;
    let mut max_load = 0usize;
    for (page_index, system_lengths) in candidate.page_system_lengths.iter().enumerate() {
        let page_slots = candidate.page_slot_lengths[page_index];
        if page_slots == 0 || system_lengths.is_empty() {
            anyhow::bail!(
                "RendererFeasibilityBoundedJointPlanCutover: Candidate enthaelt leere Page/System-Segmente"
            );
        }
        if page_index > 0 {
            if cursor >= slots {
                anyhow::bail!(
                    "RendererFeasibilityBoundedJointPlanCutover: Page-Start ausserhalb Slotbindung"
                );
            }
            promoted.slots[cursor].directives.starts_new_page = true;
            promoted.slots[cursor].page_owner =
                Some(HorizontalLayoutDecisionOwner::RendererFeasibility);
        }
        let page_end = cursor.saturating_add(page_slots);
        if page_end > slots {
            anyhow::bail!(
                "RendererFeasibilityBoundedJointPlanCutover: Page-Partition ueberschreitet Slotbindung"
            );
        }
        let mut system_cursor = cursor;
        for (system_index, length) in system_lengths.iter().copied().enumerate() {
            if length == 0 || system_cursor.saturating_add(length) > page_end {
                anyhow::bail!(
                    "RendererFeasibilityBoundedJointPlanCutover: ungueltige Systempartition"
                );
            }
            let start_measure = system_cursor.saturating_add(1);
            let end_measure = system_cursor.saturating_add(length);
            if !renderer_clearance_system_has_structural_slack(
                &basis.selected_measure_clearance_edge_evidence[start_measure - 1..end_measure],
                length,
                basis.max_allowed_system_measures,
            ) {
                anyhow::bail!(
                    "RendererFeasibilityBoundedJointPlanCutover: Candidate-System {}-{} verbraucht trotz finaler Clearance-Kante die maximale Systemlaenge",
                    start_measure,
                    end_measure
                );
            }
            if no_goods.contains(&(start_measure, end_measure)) {
                anyhow::bail!(
                    "RendererFeasibilityBoundedJointPlanCutover: Candidate wiederholt beobachtetes Renderer-No-Good {}-{}",
                    start_measure,
                    end_measure
                );
            }
            let load = basis.selected_measure_demand_evidence[start_measure - 1..end_measure]
                .iter()
                .copied()
                .fold(0usize, usize::saturating_add);
            if length > 1 && load > basis.accepted_system_capacity {
                anyhow::bail!(
                    "RendererFeasibilityBoundedJointPlanCutover: Candidate-Systemlast {} ueberschreitet Capacity {}",
                    load,
                    basis.accepted_system_capacity
                );
            }
            if system_index > 0 {
                promoted.slots[system_cursor].directives.starts_new_system = true;
                promoted.slots[system_cursor].system_owner =
                    Some(HorizontalLayoutDecisionOwner::RendererFeasibility);
            }
            systems = systems.saturating_add(1);
            singletons = singletons.saturating_add(usize::from(length == 1));
            max_load = max_load.max(load);
            system_cursor = system_cursor.saturating_add(length);
        }
        if system_cursor != page_end {
            anyhow::bail!(
                "RendererFeasibilityBoundedJointPlanCutover: Systempartition deckt Page nicht exakt ab"
            );
        }
        cursor = page_end;
    }

    if cursor != slots
        || systems != candidate.systems
        || singletons != candidate.singletons
        || max_load != candidate.max_system_load
        || !promoted.provenance_complete()
    {
        anyhow::bail!(
            "RendererFeasibilityBoundedJointPlanCutover: promovierter Candidate driftet von gebundener Evidence"
        );
    }
    Ok(promoted)
}

#[cfg(test)]
mod horizontal_layout_global_page_system_tests {
    use super::{
        build_renderer_feasibility_constraint_candidate,
        build_renderer_feasibility_frontier_candidate,
        joint_page_system_segment_partition,
        RendererFeasibilityFrontierRequest, RendererRejectedPartitions, FINAL_SELECTED_PLAN_DEMAND_MESSAGE_PREFIX,
    };

    #[test]
    fn final_selected_plan_demand_message_leaves_console_owner_prefix_to_console_progress() {
        assert_eq!(
            FINAL_SELECTED_PLAN_DEMAND_MESSAGE_PREFIX,
            "HorizontalLayoutFinalSelectedPlanDemandBinding:"
        );
        assert!(!FINAL_SELECTED_PLAN_DEMAND_MESSAGE_PREFIX.starts_with("ETUSCHLOPF:"));
    }

    #[test]
    fn six_slots_can_form_three_two_slot_systems_without_singleton() {
        let selected = joint_page_system_segment_partition(&[150; 6], 504, 2)
            .expect("joint page/system partition");
        assert_eq!(selected.system_lengths, vec![2, 2, 2]);
        assert_eq!(selected.singletons, 0);
    }

    #[test]
    fn renderer_no_good_can_add_one_system_without_singletons() {
        let no_goods = std::collections::BTreeSet::from([(1usize, 3usize)]);
        let selected = build_renderer_feasibility_constraint_candidate(
            &[100; 6],
            &[0; 6],
            1,
            (300, 3),
            3,
            2,
            &no_goods,
        )
        .expect("renderer-feasibility candidate");
        assert_eq!(selected.systems, 3);
        assert_eq!(selected.singletons, 0);
        assert_eq!(selected.page_system_lengths, vec![vec![2, 2, 2]]);
    }

    #[test]
    fn renderer_frontier_can_leave_initial_page_count_after_no_goods() {
        let no_goods = std::collections::BTreeSet::from([
            (1usize, 2usize),
            (2usize, 3usize),
            (3usize, 4usize),
            (4usize, 5usize),
            (5usize, 6usize),
        ]);
        let selected = build_renderer_feasibility_frontier_candidate(
            &RendererFeasibilityFrontierRequest {
                measure_demands: &[100; 6],
                measure_clearance_edges: &[0; 6],
                feasible_page_counts: &[2, 3],
                preferred_pages: 3,
                accepted_system_capacity: 200,
                max_system_measures: 2,
                vertical_max_systems_per_page: 2,
                reference_systems: 3,
                no_goods: &no_goods,
                rejected_partitions: &RendererRejectedPartitions::new(),
            },
        )
        .expect("frontier renderer-feasibility candidate");
        assert_eq!(selected.pages(), 3);
        assert_eq!(selected.systems, 6);
        assert_eq!(
            selected.page_system_lengths,
            vec![vec![1, 1], vec![1, 1], vec![1, 1]]
        );
    }

    #[test]
    fn renderer_frontier_keeps_preference_order_when_multiple_page_counts_remain() {
        let no_goods = std::collections::BTreeSet::new();
        let selected = build_renderer_feasibility_frontier_candidate(
            &RendererFeasibilityFrontierRequest {
                measure_demands: &[100; 6],
                measure_clearance_edges: &[0; 6],
                feasible_page_counts: &[2, 3],
                preferred_pages: 3,
                accepted_system_capacity: 300,
                max_system_measures: 3,
                vertical_max_systems_per_page: 3,
                reference_systems: 2,
                no_goods: &no_goods,
                rejected_partitions: &RendererRejectedPartitions::new(),
            },
        )
        .expect("frontier preference candidate");
        assert_eq!(selected.pages(), 2);
        assert_eq!(selected.systems, 2);
        assert_eq!(selected.singletons, 0);
    }

    #[test]
    fn renderer_clearance_edge_reserves_one_structural_system_slot() {
        let no_goods = std::collections::BTreeSet::new();
        let selected = build_renderer_feasibility_constraint_candidate(
            &[100; 6],
            &[0, 1, 0, 0, 0, 0],
            1,
            (300, 3),
            3,
            2,
            &no_goods,
        )
        .expect("clearance-structural-slack candidate");
        assert_eq!(selected.systems, 3);
        assert_eq!(selected.page_system_lengths, vec![vec![2, 2, 2]]);
    }

    #[test]
    fn system_length_bound_is_hard_even_when_four_slots_fit_capacity() {
        let selected = joint_page_system_segment_partition(&[100; 6], 504, 3)
            .expect("bounded joint page/system partition");
        assert!(selected.system_lengths.iter().all(|length| *length <= 3));
    }
}
