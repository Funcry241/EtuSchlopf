#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MaterializingRewriteFamily {
    MicroPitchChordToneMaterializer,
}


#[derive(Debug, Clone, Default)]
struct MaterializedDeltaCounts {
    note_count_before: usize,
    note_count_after: usize,
    pitch: usize,
    onset: usize,
    duration: usize,
    staff: usize,
    voice: usize,
    clef_context: usize,
    visibility: usize,
    visible_before: usize,
    visible_after: usize,
}

impl MaterializedDeltaCounts {
    fn total(&self) -> usize {
        self.pitch + self.onset + self.duration + self.staff + self.voice + self.clef_context
            + self.visibility
            + self.note_count_before.abs_diff(self.note_count_after)
    }

}

#[derive(Debug, Clone)]
struct HarmonicMicroDeltaCost {
    pitch_cost: i64,
    harmonic_cost: i64,
    voice_leading_cost: i64,
    salience_benefit: i64,
    consensus_benefit: i64,
    proof_cost: i64,
    entropy_cost: i64,
    hand_risk: i64,
    register_risk: i64,
}

#[derive(Debug, Clone)]
struct HarmonicMicroDeltaNode {
    node_id: u64,
    event_id: u64,
    family: MaterializingRewriteFamily,
    label: String,
    slot: VisibleChordToneSwapSlot,
    to_midi: i32,
    from_pitch_class: i32,
    to_pitch_class: i32,
    abs_semitone_delta: i32,
    local_context_id: u64,
    rule_signature: u64,
    cost: HarmonicMicroDeltaCost,
    pareto_rank: usize,
    graph_degree: usize,
    total_score: i64,
    patched_xml: String,
    /// Bound inputs of the #8 owner execution that produced `patched_xml`; a
    /// materialized node re-executes the owner so that its own receipt
    /// belongs to exactly that execution (no receipt is shared between nodes).
    swap_candidate: HiddenChordToneRecoveryCandidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HarmonicMicroEdgeKind {
    DuplicateEvent,
    HarmonicPitchClass,
    VoiceLeadingNeighborhood,
    TemporalNeighborhood,
    ProofInterference,
}


#[derive(Debug, Clone)]
struct HarmonicMicroDeltaEdge {
    from_node: u64,
    to_node: u64,
    kind: HarmonicMicroEdgeKind,
    total_signal: i64,
}

#[derive(Debug, Clone, Default)]
struct HarmonicMicroDeltaGraph {
    nodes: Vec<HarmonicMicroDeltaNode>,
}

impl HarmonicMicroDeltaGraph {
    fn ordered_micro_variants(&self) -> Vec<HarmonicMicroDeltaNode> {
        let mut ordered = self.nodes.clone();
        ordered.sort_by(|a, b| {
            a.pareto_rank
                .cmp(&b.pareto_rank)
                .then_with(|| b.total_score.cmp(&a.total_score))
                .then_with(|| a.cost.proof_cost.cmp(&b.cost.proof_cost))
                .then_with(|| a.cost.entropy_cost.cmp(&b.cost.entropy_cost))
                .then_with(|| b.graph_degree.cmp(&a.graph_degree))
                .then_with(|| a.event_id.cmp(&b.event_id))
                .then_with(|| a.node_id.cmp(&b.node_id))
        });
        ordered
    }

}



fn run_certified_materializing_local_rewrite_portfolio(
    best: &mut SolverCandidate,
    options: &CheckOptions,
    next_iteration: &mut usize,
) -> anyhow::Result<()> {
    const MAX_MICRO_CANDIDATES: usize = 6;
    const MAX_ADOPTED_MICRO_DELTAS: usize = 2;

    let mut checked_candidates = 0usize;
    let mut candidate_hashes_seen = BTreeSet::new();

    for portfolio_step in 0..MAX_ADOPTED_MICRO_DELTAS {
        if checked_candidates >= MAX_MICRO_CANDIDATES {
            break;
        }

        let base_xml = best.layout_xml().to_string();
        let graph = build_harmonic_micro_delta_graph(&base_xml, 24);
        let variants = graph.ordered_micro_variants();
        if variants.is_empty() {
            break;
        }

        let noop_raw_rhy_signatures =
            validate_candidate_path_equivalent(&base_xml, options, &BTreeSet::new())
                .map(|validation| validation.raw_rhy_signatures)
                .unwrap_or_default();

        let mut owner_approved_materialized: Vec<(usize, SolverCandidate, usize, i64, i64)> =
            Vec::new();
        let mut produced_candidate_this_round = false;
        let mut guarded_0_1_0_1_seen = false;

        for (idx, node) in variants.into_iter().enumerate() {
            if checked_candidates >= MAX_MICRO_CANDIDATES {
                break;
            }
            let xml = node.patched_xml.clone();
            if !candidate_hashes_seen.insert(stable_text_hash(&xml)) {
                continue;
            }
            produced_candidate_this_round = true;
            checked_candidates += 1;

            let sequence_step = portfolio_step + 1;
            let candidate_label = if portfolio_step > 0 {
                format!("{}_recheck_s{}", node.label, sequence_step)
            } else {
                node.label.clone()
            };
            let delta_counts = materialized_delta_counts(&base_xml, &xml);
            let micro_budget_ok = micro_delta_budget_ok(&delta_counts);
            // Re-execute the node's bound visibility owners (#11 show, #5 hide)
            // on the base state so the materialized candidate carries the
            // receipts of its own execution; the bytes must equal the explored
            // node, otherwise the run aborts.
            let mut rematerialized = base_xml.clone();
            let mut node_receipts = master_state_identity::OwnerReceipts::default();
            let steps = coincident_chord_tone_print_exchange_steps(
                &rematerialized, &node.swap_candidate, &node.slot);
            let mut exchanged = steps.is_some();
            for (start, end, show) in steps.into_iter().flatten() {
                let (changed, receipt) = if show {
                    structural_write_authority::remove_note_hidden_attrs_in_range_authorized(
                        &mut rematerialized, start, end)
                } else {
                    let (changed, receipt) = structural_write_authority::hide_note_print_object_authorized(
                        &mut rematerialized, start, end);
                    (changed.unwrap_or(false), receipt)
                };
                exchanged &= changed && receipt.is_some();
                node_receipts.push_exact(receipt);
            }
            if !exchanged || rematerialized != node.patched_xml {
                master_state_identity::master_sovereignty_violation(
                    "HarmonicMicroDelta: re-execution of the bound visibility owners does not reproduce the explored node bytes",
                );
            }
            let candidate_result = if micro_budget_ok && delta_counts.total() > 0 {
                build_micro_materialized_candidate(
                    *next_iteration,
                    &candidate_label,
                    xml.clone(),
                    node_receipts,
                    best,
                    options,
                    &noop_raw_rhy_signatures,
                )
                .and_then(|(checked, xml_request)| {
                    let mut candidate = best.clone();
                    candidate.iteration = *next_iteration;
                    candidate.profile_name = candidate_label.clone();
                    candidate.final_result = checked;
                    structural_write_authority::apply_block_master_xml_request(
                        &mut candidate.master_state,
                        xml_request,
                    )?;
                    candidate.midi_seed_stats.solver_profile = candidate_label.clone();
                    Ok(candidate)
                })
            } else {
                let xml_request = rejected_micro_materialized_candidate(
                    *next_iteration,
                    &candidate_label,
                    xml.clone(),
                    node_receipts,
                    best,
                );
                let mut candidate = best.clone();
                candidate.iteration = *next_iteration;
                candidate.profile_name = candidate_label.clone();
                structural_write_authority::apply_block_master_xml_request(
                    &mut candidate.master_state,
                    xml_request,
                )?;
                Ok(candidate)
            };

            let candidate = candidate_result.with_context(|| {
                format!(
                    "HarmonicMicroDeltaGraphKernel Kandidat {} fehlgeschlagen",
                    idx + 1
                )
            })?;
            if micro_budget_ok && delta_counts.total() > 0 {
                *next_iteration += 1;
            }

            let final_critical = critical_count(&candidate.final_result);
            let final_clean = micro_budget_ok
                && candidate.final_result.summary.blockers == 0
                && candidate.final_result.summary.warnings == 0
                && final_critical == 0
                && delta_counts.total() > 0;
            if candidate.final_result.summary.blockers == 0
                && candidate.final_result.summary.warnings == 1
                && final_critical == 1
            {
                guarded_0_1_0_1_seen = true;
            }

            let score = materialized_rewrite_selection_score(
                &candidate,
                &delta_counts,
                node.family,
            ) + node.total_score
                - (node.pareto_rank as i64 * 1_000)
                - (portfolio_step as i64 * 15);

            if final_clean && node.pareto_rank == 0 {
                let adoption = decide_hidden_chord_tone_recovery_adoption(
                    HiddenChordToneRecoveryMaterializationProjection::HarmonicPitchSwap,
                    hidden_chord_tone_recovery_decision_facts(
                        &best.final_result,
                        &candidate.final_result,
                        delta_counts.visible_before,
                        delta_counts.visible_after,
                    ),
                );
                if adoption.adopt {
                    owner_approved_materialized.push((
                        idx,
                        candidate,
                        node.pareto_rank,
                        score,
                        node.cost.proof_cost,
                    ));
                }
            }
        }

        owner_approved_materialized.sort_by(|a, b| {
            a.2.cmp(&b.2)
                .then_with(|| b.3.cmp(&a.3))
                .then_with(|| a.4.cmp(&b.4))
                .then_with(|| a.0.cmp(&b.0))
        });

        if let Some((_idx, accepted, _pareto_rank, _score, _proof_cost)) =
            owner_approved_materialized.into_iter().next()
        {
            crate::audit_only::destructive_edit_attribution::observe_selected_state_successor(
                &base_xml,
                accepted.layout_xml(),
            );
            *best = accepted;
            continue;
        }

        if !produced_candidate_this_round || guarded_0_1_0_1_seen {
            break;
        }
        break;
    }

    Ok(())
}

fn micro_delta_budget_ok(delta: &MaterializedDeltaCounts) -> bool {
    delta.note_count_before == delta.note_count_after
        && delta.visible_before == delta.visible_after
        && delta.onset == 0
        && delta.duration == 0
        && delta.pitch <= 1
        && delta.staff <= 1
        && delta.voice <= 1
        && delta.clef_context <= 1
        && delta.total() > 0
        && delta.total() <= 3
}

fn harmonic_micro_event_id(
    cand: &HiddenChordToneRecoveryCandidate,
    slot: &VisibleChordToneSwapSlot,
) -> u64 {
    stable_text_hash(&format!(
        "harmonic-micro-event|staff{}|voice{}|start{}|dur{}|slot-midi{:?}|target-midi{:?}",
        slot.staff,
        slot.voice,
        slot.start_tick,
        slot.duration,
        slot.midi,
        cand.midi
    ))
}

fn harmonic_micro_node_id(
    cand: &HiddenChordToneRecoveryCandidate,
    slot: &VisibleChordToneSwapSlot,
    patched_xml: &str,
) -> u64 {
    stable_text_hash(&format!(
        "harmonic-micro-node|event{}|cand-score{}|patch{}",
        harmonic_micro_event_id(cand, slot),
        cand.score,
        stable_text_hash(patched_xml)
    ))
}

fn harmonic_micro_pitch_class(midi: i32) -> i32 {
    midi.rem_euclid(12)
}

fn local_micro_context_id(staff: usize, voice: &str, start_tick: i64, duration: i64) -> u64 {
    let unit = duration.max(1);
    let bucket = start_tick / (unit * 4);
    stable_text_hash(&format!("micro-context|s{staff}|v{voice}|bucket{bucket}"))
}

fn harmonic_micro_rule_signature(node: &HarmonicMicroDeltaNode) -> u64 {
    stable_text_hash(&format!(
        "micro-rule-sig|staff{}|voice{}|frompc{}|topc{}|delta{}|hand{}|register{}",
        node.slot.staff,
        node.slot.voice,
        node.from_pitch_class,
        node.to_pitch_class,
        node.abs_semitone_delta,
        node.cost.hand_risk / 120,
        node.cost.register_risk / 120
    ))
}


fn pitch_class_harmonic_cost(from_midi: i32, to_midi: i32) -> i64 {
    let interval = (from_midi - to_midi).abs().rem_euclid(12);
    let class_cost = match interval {
        0 => 0,
        3 | 4 | 5 | 7 | 8 | 9 => 120,
        2 | 10 => 220,
        1 | 11 => 320,
        6 => 460,
        _ => 260,
    };
    let octave_cost = ((from_midi - to_midi).abs() / 12) as i64 * 35;
    class_cost + octave_cost
}

fn harmonic_micro_consensus_benefit(cand: &HiddenChordToneRecoveryCandidate) -> i64 {
    match reduction_role_consensus_class(cand) {
        "konsens-muss" => 1_600,
        "konsens-ersatzfigur" => 1_050,
        "konflikt-herabgestuft" => 120,
        "nur Klangfarbe" => 40,
        _ => 0,
    }
}

fn harmonic_micro_voice_leading_cost(
    cand: &HiddenChordToneRecoveryCandidate,
    slot: &VisibleChordToneSwapSlot,
) -> i64 {
    let from = slot.midi.unwrap_or(64);
    let to = cand.midi.unwrap_or(from);
    let pitch_motion = (from - to).abs() as i64;
    let same_scope_bonus = if cand.staff == slot.staff && cand.voice == slot.voice { 0 } else { 400 };
    pitch_motion * 45 + same_scope_bonus
}

fn harmonic_micro_proof_cost(
    cand: &HiddenChordToneRecoveryCandidate,
    slot: &VisibleChordToneSwapSlot,
) -> i64 {
    let pitch_motion = midi_distance(cand.midi, slot.midi) as i64;
    let wide_pitch_penalty = if pitch_motion > 12 { 420 } else { 0 };
    let rhythmic_alignment_penalty = if cand.start_tick % 24 == 0 && cand.duration > 0 { 0 } else { 300 };
    80 + pitch_motion * 18 + wide_pitch_penalty + rhythmic_alignment_penalty
}

fn harmonic_micro_entropy_cost(
    cand: &HiddenChordToneRecoveryCandidate,
    slot: &VisibleChordToneSwapSlot,
) -> i64 {
    let onset_complexity = (cand.start_tick.rem_euclid(96) / 24) * 30;
    let duration_complexity = if cand.duration == slot.duration { 0 } else { 240 };
    let register_symbol_complexity = match cand.midi {
        Some(m) if m >= 96 || m <= 33 => 180,
        Some(m) if m >= 88 || m <= 40 => 80,
        _ => 0,
    };
    onset_complexity + duration_complexity + register_symbol_complexity
}

fn harmonic_micro_hand_risk(
    cand: &HiddenChordToneRecoveryCandidate,
    slot: &VisibleChordToneSwapSlot,
) -> i64 {
    let base = hand_plan_candidate_risk(cand) * 4;
    let from = slot.midi.unwrap_or(64);
    let to = cand.midi.unwrap_or(from);
    let same_staff_large_drop = if cand.staff == 1 && from - to > 12 { 360 } else { 0 };
    let same_staff_large_rise = if cand.staff == 2 && to - from > 12 { 360 } else { 0 };
    base + same_staff_large_drop + same_staff_large_rise
}

fn harmonic_micro_register_risk(cand: &HiddenChordToneRecoveryCandidate) -> i64 {
    match (cand.staff, cand.midi.unwrap_or(64)) {
        (1, m) if !(55..=100).contains(&m) => 420,
        (1, m) if !(60..=96).contains(&m) => 160,
        (2, m) if !(28..=78).contains(&m) => 420,
        (2, m) if !(34..=74).contains(&m) => 160,
        _ => 0,
    }
}

fn harmonic_micro_cost_vector(
    cand: &HiddenChordToneRecoveryCandidate,
    slot: &VisibleChordToneSwapSlot,
) -> HarmonicMicroDeltaCost {
    let from = slot.midi.unwrap_or(64);
    let to = cand.midi.unwrap_or(from);
    HarmonicMicroDeltaCost {
        pitch_cost: (from - to).abs() as i64 * 100,
        harmonic_cost: pitch_class_harmonic_cost(from, to),
        voice_leading_cost: harmonic_micro_voice_leading_cost(cand, slot),
        salience_benefit: consensus_gesture_candidate_weight(cand).max(0),
        consensus_benefit: harmonic_micro_consensus_benefit(cand),
        proof_cost: harmonic_micro_proof_cost(cand, slot),
        entropy_cost: harmonic_micro_entropy_cost(cand, slot),
        hand_risk: harmonic_micro_hand_risk(cand, slot),
        register_risk: harmonic_micro_register_risk(cand),
    }
}

fn harmonic_micro_total_score(cost: &HarmonicMicroDeltaCost, graph_support: i64, abs_semitone_delta: i32) -> i64 {
    cost.salience_benefit
        + cost.consensus_benefit
        + graph_support
        - cost.pitch_cost
        - cost.harmonic_cost
        - cost.voice_leading_cost
        - cost.proof_cost
        - cost.entropy_cost
        - cost.hand_risk
        - cost.register_risk
        - (abs_semitone_delta as i64 * 20)
}

fn harmonic_micro_dominates(a: &HarmonicMicroDeltaNode, b: &HarmonicMicroDeltaNode) -> bool {
    let a_vec = [
        a.cost.pitch_cost,
        a.cost.harmonic_cost,
        a.cost.voice_leading_cost,
        a.cost.proof_cost,
        a.cost.entropy_cost,
        a.cost.hand_risk,
        a.cost.register_risk,
        -a.cost.salience_benefit,
        -a.cost.consensus_benefit,
    ];
    let b_vec = [
        b.cost.pitch_cost,
        b.cost.harmonic_cost,
        b.cost.voice_leading_cost,
        b.cost.proof_cost,
        b.cost.entropy_cost,
        b.cost.hand_risk,
        b.cost.register_risk,
        -b.cost.salience_benefit,
        -b.cost.consensus_benefit,
    ];
    let all_le = a_vec.iter().zip(b_vec.iter()).all(|(x, y)| x <= y);
    let any_lt = a_vec.iter().zip(b_vec.iter()).any(|(x, y)| x < y);
    all_le && any_lt
}

fn assign_harmonic_micro_pareto_front(nodes: &mut [HarmonicMicroDeltaNode]) {
    for i in 0..nodes.len() {
        let mut dominated = false;
        for j in 0..nodes.len() {
            if i != j && harmonic_micro_dominates(&nodes[j], &nodes[i]) {
                dominated = true;
                break;
            }
        }
        nodes[i].pareto_rank = if dominated { 1 } else { 0 };
    }
}

fn pitch_class_affinity(a_pc: i32, b_pc: i32) -> i64 {
    let d = (a_pc - b_pc).rem_euclid(12);
    let interval = d.min(12 - d);
    match interval {
        0 => 1_000,
        3 | 4 => 900,
        5 | 7 => 850,
        2 | 10 => 650,
        1 | 11 => 300,
        6 => 200,
        _ => 500,
    }
}

fn make_harmonic_micro_edge(
    a: &HarmonicMicroDeltaNode,
    b: &HarmonicMicroDeltaNode,
    kind: HarmonicMicroEdgeKind,
    harmonic_affinity: i64,
    voice_affinity: i64,
    temporal_affinity: i64,
    proof_interference_risk: i64,
) -> HarmonicMicroDeltaEdge {
    let total_signal = harmonic_affinity + voice_affinity + temporal_affinity + proof_interference_risk;
    HarmonicMicroDeltaEdge {
        from_node: a.node_id,
        to_node: b.node_id,
        kind,
        total_signal,
    }
}

fn collect_harmonic_micro_edges_for_pair(
    a: &HarmonicMicroDeltaNode,
    b: &HarmonicMicroDeltaNode,
    edges: &mut Vec<HarmonicMicroDeltaEdge>,
) {
    if a.node_id == b.node_id {
        return;
    }
    if a.event_id == b.event_id {
        edges.push(make_harmonic_micro_edge(
            a,
            b,
            HarmonicMicroEdgeKind::DuplicateEvent,
            0,
            0,
            0,
            1_000,
        ));
        return;
    }

    let harmonic = pitch_class_affinity(a.to_pitch_class, b.to_pitch_class);
    if harmonic >= 700 {
        edges.push(make_harmonic_micro_edge(
            a,
            b,
            HarmonicMicroEdgeKind::HarmonicPitchClass,
            harmonic,
            0,
            0,
            0,
        ));
    }

    let same_voice = a.slot.staff == b.slot.staff && a.slot.voice == b.slot.voice;
    let dt = (a.slot.start_tick - b.slot.start_tick).abs();
    let local_window = a.slot.duration.max(b.slot.duration).max(1) * 8;
    if same_voice && dt <= local_window {
        let temporal = 1_000 - ((dt * 1_000) / local_window).min(1_000);
        let voice = 1_000 - ((a.to_midi - b.to_midi).abs() as i64 * 40).min(1_000);
        edges.push(make_harmonic_micro_edge(
            a,
            b,
            HarmonicMicroEdgeKind::VoiceLeadingNeighborhood,
            0,
            voice,
            temporal,
            0,
        ));
    }

    if a.local_context_id == b.local_context_id {
        edges.push(make_harmonic_micro_edge(
            a,
            b,
            HarmonicMicroEdgeKind::TemporalNeighborhood,
            0,
            0,
            800,
            0,
        ));
    }

    if a.rule_signature != 0 && a.rule_signature == b.rule_signature {
        edges.push(make_harmonic_micro_edge(
            a,
            b,
            HarmonicMicroEdgeKind::ProofInterference,
            0,
            0,
            0,
            900,
        ));
    }
}

fn prune_harmonic_micro_edges(
    mut edges: Vec<HarmonicMicroDeltaEdge>,
    max_degree_per_node: usize,
) -> Vec<HarmonicMicroDeltaEdge> {
    edges.sort_by_key(|a| std::cmp::Reverse(a.total_signal));
    let mut degree: BTreeMap<u64, usize> = BTreeMap::new();
    let mut kept = Vec::new();
    for edge in edges {
        let da = *degree.get(&edge.from_node).unwrap_or(&0);
        let db = *degree.get(&edge.to_node).unwrap_or(&0);
        if da >= max_degree_per_node || db >= max_degree_per_node {
            continue;
        }
        degree.insert(edge.from_node, da + 1);
        degree.insert(edge.to_node, db + 1);
        kept.push(edge);
    }
    kept
}

fn graph_support_for_node(node_id: u64, edges: &[HarmonicMicroDeltaEdge]) -> i64 {
    edges
        .iter()
        .filter(|edge| edge.from_node == node_id || edge.to_node == node_id)
        .map(|edge| match edge.kind {
            HarmonicMicroEdgeKind::HarmonicPitchClass => edge.total_signal / 10,
            HarmonicMicroEdgeKind::VoiceLeadingNeighborhood => edge.total_signal / 12,
            HarmonicMicroEdgeKind::TemporalNeighborhood => edge.total_signal / 15,
            HarmonicMicroEdgeKind::ProofInterference => -(edge.total_signal / 8),
            HarmonicMicroEdgeKind::DuplicateEvent => -(edge.total_signal / 4),
        })
        .sum()
}

fn harmonic_micro_node_better(a: &HarmonicMicroDeltaNode, b: &HarmonicMicroDeltaNode) -> bool {
    let a_risk = a.cost.proof_cost
        + a.cost.entropy_cost
        + a.cost.hand_risk
        + a.cost.register_risk
        + a.abs_semitone_delta as i64 * 20;
    let b_risk = b.cost.proof_cost
        + b.cost.entropy_cost
        + b.cost.hand_risk
        + b.cost.register_risk
        + b.abs_semitone_delta as i64 * 20;
    a_risk < b_risk || (a_risk == b_risk && a.node_id < b.node_id)
}

fn dedupe_harmonic_micro_nodes(nodes: Vec<HarmonicMicroDeltaNode>) -> (Vec<HarmonicMicroDeltaNode>, usize) {
    let raw_count = nodes.len();
    let mut best_by_event: BTreeMap<u64, HarmonicMicroDeltaNode> = BTreeMap::new();
    for node in nodes {
        best_by_event
            .entry(node.event_id)
            .and_modify(|old| {
                if harmonic_micro_node_better(&node, old) {
                    *old = node.clone();
                }
            })
            .or_insert(node);
    }
    let deduped = best_by_event.into_values().collect::<Vec<_>>();
    let duplicate_count = raw_count.saturating_sub(deduped.len());
    (deduped, duplicate_count)
}


fn build_harmonic_micro_delta_graph(xml: &str, limit: usize) -> HarmonicMicroDeltaGraph {
    let mut hidden = collect_hidden_chord_tone_recovery_candidates(xml, 4096)
        .into_iter()
        .filter(|cand| cand.midi.is_some() && !cand.pitch_xml.is_empty())
        .collect::<Vec<_>>();
    hidden.sort_by(|a, b| {
        consensus_gesture_candidate_weight(b).cmp(&consensus_gesture_candidate_weight(a))
            .then_with(|| a.start_tick.cmp(&b.start_tick))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.voice.cmp(&b.voice))
            .then_with(|| a.midi.cmp(&b.midi))
    });

    let mut slots = collect_visible_chord_tone_swap_slots(xml);
    slots.sort_by_key(|slot| {
        let midi = slot.midi.unwrap_or(0);
        (slot.start_tick, slot.duration, slot.staff, slot.voice.clone(), midi)
    });

    let mut raw_nodes = Vec::new();
    let mut seen = BTreeSet::new();
    'outer: for cand in hidden.iter().take(128) {
        for slot in slots.iter().filter(|slot| {
            slot.measure == cand.measure
                && slot.start_tick == cand.start_tick
                && slot.duration == cand.duration
                && slot.staff == cand.staff
                && slot.voice == cand.voice
        }) {
            if raw_nodes.len() >= limit {
                break 'outer;
            }
            let Some(from_midi) = slot.midi else { continue; };
            let Some(to_midi) = cand.midi else { continue; };
            let pitch_delta = (from_midi - to_midi).abs();
            if pitch_delta == 0 || pitch_delta > 18 {
                continue;
            }
            let mut patched = xml.to_string();
            // Graph exploration only: this execution's receipts are never adopted.
            let Some(steps) = coincident_chord_tone_print_exchange_steps(xml, cand, slot) else {
                continue;
            };
            let mut exchanged = true;
            for (start, end, show) in steps {
                exchanged &= if show {
                    structural_write_authority::remove_note_hidden_attrs_in_range_authorized(
                        &mut patched, start, end).0
                } else {
                    structural_write_authority::hide_note_print_object_authorized(
                        &mut patched, start, end).0.unwrap_or(false)
                };
            }
            if !exchanged || patched == xml {
                continue;
            }
            if !visibility_exchange_only(xml, &patched, cand.measure) {
                continue;
            }
            if !micro_delta_budget_ok(&materialized_delta_counts(xml, &patched)) {
                continue;
            }
            let patch_hash = stable_text_hash(&patched);
            if !seen.insert(patch_hash) {
                continue;
            }
            let event_id = harmonic_micro_event_id(cand, slot);
            let node_id = harmonic_micro_node_id(cand, slot, &patched);
            let cost = harmonic_micro_cost_vector(cand, slot);
            let from_pitch_class = harmonic_micro_pitch_class(from_midi);
            let to_pitch_class = harmonic_micro_pitch_class(to_midi);
            let local_context_id = local_micro_context_id(slot.staff, &slot.voice, slot.start_tick, slot.duration);
            let mut node = HarmonicMicroDeltaNode {
                node_id,
                event_id,
                family: MaterializingRewriteFamily::MicroPitchChordToneMaterializer,
                label: format!("micro_harmonic_node_{node_id:016x}"),
                slot: slot.clone(),
                to_midi,
                from_pitch_class,
                to_pitch_class,
                abs_semitone_delta: pitch_delta,
                local_context_id,
                rule_signature: 0,
                cost,
                pareto_rank: 0,
                graph_degree: 0,
                total_score: 0,
                patched_xml: patched,
                swap_candidate: cand.clone(),
            };
            node.rule_signature = harmonic_micro_rule_signature(&node);
            raw_nodes.push(node);
        }
    }

    let (mut nodes, _duplicate_event_count) = dedupe_harmonic_micro_nodes(raw_nodes);

    let mut raw_edges = Vec::new();
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            collect_harmonic_micro_edges_for_pair(&nodes[i], &nodes[j], &mut raw_edges);
        }
    }
    let edges = prune_harmonic_micro_edges(raw_edges, 4);

    let mut degree_by_node: BTreeMap<u64, usize> = BTreeMap::new();
    for edge in &edges {
        *degree_by_node.entry(edge.from_node).or_default() += 1;
        *degree_by_node.entry(edge.to_node).or_default() += 1;
    }
    for node in &mut nodes {
        node.graph_degree = *degree_by_node.get(&node.node_id).unwrap_or(&0);
        let graph_support = graph_support_for_node(node.node_id, &edges);
        node.total_score = harmonic_micro_total_score(&node.cost, graph_support, node.abs_semitone_delta);
    }
    assign_harmonic_micro_pareto_front(&mut nodes);
    nodes.sort_by(|a, b| {
        a.pareto_rank
            .cmp(&b.pareto_rank)
            .then_with(|| b.total_score.cmp(&a.total_score))
            .then_with(|| a.cost.proof_cost.cmp(&b.cost.proof_cost))
            .then_with(|| a.event_id.cmp(&b.event_id))
            .then_with(|| a.node_id.cmp(&b.node_id))
    });

    HarmonicMicroDeltaGraph { nodes }
}


fn build_micro_materialized_candidate(
    _iteration: usize,
    patch_name: &str,
    xml: String,
    owner_receipts: master_state_identity::OwnerReceipts,
    base: &SolverCandidate,
    options: &CheckOptions,
    noop_raw_rhy_signatures: &BTreeSet<String>,
) -> anyhow::Result<(CheckReport, structural_write_authority::MasterXmlReplacementRequest)> {
    let validation = validate_candidate_path_equivalent(&xml, options, noop_raw_rhy_signatures)
        .with_context(|| format!("HarmonicMicroDeltaGraphKernel {patch_name} konnte nicht geprüft werden"))?;
    let request = structural_write_authority::MasterXmlReplacementRequest::bind_with_owner_receipts(
        base.layout_xml(),
        xml,
        owner_receipts,
    );
    Ok((validation.normalized_report, request))
}

fn rejected_micro_materialized_candidate(
    _iteration: usize,
    _patch_name: &str,
    xml: String,
    owner_receipts: master_state_identity::OwnerReceipts,
    base: &SolverCandidate,
) -> structural_write_authority::MasterXmlReplacementRequest {
    structural_write_authority::MasterXmlReplacementRequest::bind_with_owner_receipts(
        base.layout_xml(),
        xml,
        owner_receipts,
    )
}

fn materialized_delta_counts(before: &str, after: &str) -> MaterializedDeltaCounts {
    let before_notes = parse_atomic_notes(before);
    let after_notes = parse_atomic_notes(after);
    let mut counts = MaterializedDeltaCounts {
        note_count_before: before_notes.len(),
        note_count_after: after_notes.len(),
        visible_before: before_notes.iter().filter(|n| n.visible && n.pitch.is_some()).count(),
        visible_after: after_notes.iter().filter(|n| n.visible && n.pitch.is_some()).count(),
        clef_context: count_tag_occurrences(before, "clef").abs_diff(count_tag_occurrences(after, "clef")),
        ..Default::default()
    };
    for (a, b) in before_notes.iter().zip(after_notes.iter()) {
        if a.pitch.as_ref().map(|p| p.midi) != b.pitch.as_ref().map(|p| p.midi) {
            counts.pitch += 1;
        }
        if a.start != b.start {
            counts.onset += 1;
        }
        if a.duration != b.duration {
            counts.duration += 1;
        }
        if a.staff != b.staff {
            counts.staff += 1;
        }
        if a.voice != b.voice {
            counts.voice += 1;
        }
        if a.visible != b.visible {
            counts.visibility += 1;
        }
    }
    counts
}

/// The harmonic micro-delta may only exchange which of two coincident chord
/// tones is printed: same notes in the same order with unchanged pitch, onset,
/// duration, staff and voice; exactly one tone shown and one concealed; the
/// printed rhythm of the measure unchanged; and no other structural entry
/// (note, backup or forward) differing except the hidden flag of those two notes.
fn visibility_exchange_only(before: &str, after: &str, measure: usize) -> bool {
    let printed_rhythm = visible_measure_rhythm_invariant_signature(before, measure);
    let printed_rhythm_after = visible_measure_rhythm_invariant_signature(after, measure);
    let (before_structure, after_structure) =
        (full_pitch_agnostic_structure_signature(before), full_pitch_agnostic_structure_signature(after));
    let unhidden = |entry: &String| entry.replace("|h1|", "|h0|");
    let structure_changes = before_structure.iter().zip(&after_structure)
        .filter(|(b, a)| b != a).collect::<Vec<_>>();
    let (before, after) = (parse_atomic_notes(before), parse_atomic_notes(after));
    printed_rhythm.is_some()
        && printed_rhythm == printed_rhythm_after
        && before_structure.len() == after_structure.len()
        && structure_changes.len() == 2
        && structure_changes.iter().all(|(b, a)| unhidden(b) == unhidden(a))
        && before.len() == after.len()
        && before.iter().zip(&after).all(|(a, b)| {
            a.pitch.as_ref().map(|p| p.midi) == b.pitch.as_ref().map(|p| p.midi)
                && (a.start, a.duration, &a.staff, &a.voice) == (b.start, b.duration, &b.staff, &b.voice)
        })
        && before.iter().zip(&after).filter(|(a, b)| a.visible && !b.visible).count() == 1
        && before.iter().zip(&after).filter(|(a, b)| !a.visible && b.visible).count() == 1
}

fn materialized_rewrite_selection_score(
    candidate: &SolverCandidate,
    delta: &MaterializedDeltaCounts,
    family: MaterializingRewriteFamily,
) -> i64 {
    let family_bonus = match family {
        MaterializingRewriteFamily::MicroPitchChordToneMaterializer => 500,
    };
    let visible_gain = delta.visible_after as i64 - delta.visible_before as i64;
    family_bonus
        + (delta.staff as i64 * 70)
        + (delta.voice as i64 * 50)
        + (delta.pitch as i64 * 30)
        + visible_gain * 40
        - candidate.final_result.summary.blockers as i64 * 20_000
        - candidate.final_result.summary.warnings as i64 * 8_000
        - candidate.final_result.summary.hints as i64 * 1_000
        - critical_count(&candidate.final_result) as i64 * 12_000
}

fn count_tag_occurrences(xml: &str, tag: &str) -> usize {
    let needle = format!("<{tag}");
    xml.match_indices(&needle).count()
}

fn stable_text_hash(text: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}









