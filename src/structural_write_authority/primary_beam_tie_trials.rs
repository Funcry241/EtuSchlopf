//! One in-memory candidate using live primary beam partner channels.
use super::ordered_received_beam_readback::{
    OwnerEdge, OwnerEdgeReadback, closed_group, owner_groups,
};
use super::primary_beam_pair_readback::MasterOccurrences;
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

type FragmentPair = (usize, usize);
pub(super) type EdgeStates = BTreeMap<FragmentPair, OwnerEdgeReadback>;
type Partners = BTreeMap<usize, BTreeSet<usize>>;
type TrialError = (&'static str, String);

#[derive(Debug, Clone, Default)]
pub(super) struct PrimaryBeamTieTrials {
    targets: Vec<FragmentPair>,
    decisions: String,
    candidate: Option<Result<AdmittedTrial, TrialError>>,
}

#[derive(Debug, Clone)]
struct AdmittedTrial {
    replacement_adopted: bool,
    selected: FrozenSerializedVoicePlan,
}

pub(super) fn primary_edges<'a>(
    canonical: &CanonicalFinalChordStructuralPlan,
    capture: &'a ActualSourceBeamBindingObservation,
) -> Result<Vec<OwnerEdge<'a>>, &'static str> {
    let rows = capture.graph_received_occurrences(canonical)?;
    let mut edges = vec![];
    for ((level, group), members) in owner_groups(rows) {
        if level != 1 || !closed_group(&members) {
            continue;
        }
        edges.extend(members.windows(2).map(|pair| OwnerEdge {
            level,
            group,
            group_size: members.len(),
            left: pair[0].0,
            right: pair[1].0,
        }));
    }
    Ok(edges)
}

pub(super) fn partner_classes(
    canonical: &CanonicalFinalChordStructuralPlan,
    capture: Option<&ActualSourceBeamBindingObservation>,
    classes: &BTreeMap<usize, usize>,
) -> io::Result<Partners> {
    let capture = capture.ok_or_else(|| io::Error::other("missing primary beam capture"))?;
    let edges = primary_edges(canonical, capture).map_err(io::Error::other)?;
    let mut partners = Partners::new();
    for edge in edges {
        let (Some(&left), Some(&right)) = (
            classes.get(&edge.left.source),
            classes.get(&edge.right.source),
        ) else {
            return Err(io::Error::other(
                "primary beam endpoint has no assignment class",
            ));
        };
        if left != right {
            partners.entry(left).or_default().insert(right);
            partners.entry(right).or_default().insert(left);
        }
    }
    Ok(partners)
}

pub(super) fn choose_joint_channel(
    class: usize,
    standard: usize,
    ranked: &[(usize, usize)],
    assigned: &BTreeMap<usize, usize>,
    partners: &Partners,
    out: &mut String,
) -> usize {
    let Some(&(best, _)) = ranked.first() else {
        return standard;
    };
    let best_channels: BTreeSet<_> = ranked
        .iter()
        .take_while(|&&(rank, _)| rank == best)
        .map(|&(_, channel)| channel)
        .collect();
    if best_channels.len() < 2 {
        return standard;
    }
    let partner_channels: BTreeSet<_> = partners
        .get(&class)
        .into_iter()
        .flatten()
        .filter_map(|other| assigned.get(other).copied())
        .filter(|channel| best_channels.contains(channel))
        .collect();
    match partner_channels.len() {
        0 => standard,
        1 => {
            let channel = *partner_channels.first().expect("one partner channel");
            if channel != standard {
                let _ = writeln!(
                    out,
                    "PrimaryBeamJointTieDecision: class={class}; best_rank={best}; standard={standard}; chosen={channel}; authority=none"
                );
            }
            channel
        }
        _ => {
            let _ = writeln!(
                out,
                "PrimaryBeamJointTieConflict: class={class}; best_rank={best}; partner_channels={partner_channels:?}; chosen={standard}; authority=none"
            );
            standard
        }
    }
}

impl CanonicalFinalChordStructuralPlan {
    pub(crate) fn observe_primary_beam_tie_trials(
        &mut self,
        attacks: &[FrozenVoiceAttackEvidenceRow],
        capture: &ActualSourceBeamBindingObservation,
        preferences: &[LegacyVoiceLabelPreferenceRow],
        gaps: &[crate::midi_seed::SourceVoiceGap],
        targets: Vec<FragmentPair>,
    ) {
        let mut decisions = String::new();
        let candidate = (!targets.is_empty()).then(|| {
            let channels = build_frozen_serialization_voice_channels_inner(
                self,
                attacks,
                Some(capture),
                preferences,
                true,
                &BTreeSet::new(),
                (None, AllocationObservation::JointTies(&mut decisions)),
            )
            .map_err(|error| {
                (
                    "initial_assignment_or_partition_admission",
                    error.to_string(),
                )
            })?;
            let mut baseline = bind_musicxml_voice_labels(&channels, preferences)
                .map_err(|error| ("baseline_label_binding", error.to_string()))?;
            channels
                .require_consumer_admission(self, &baseline)
                .map_err(|error| ("baseline_consumer_admission", error.to_string()))?;
            baseline.bind_source_voice_gaps(gaps.to_vec());
            let mut replacement_report = String::new();
            let replacement = channels.checked_written_only_replacement(
                &mut replacement_report,
                self,
                attacks,
                capture,
                preferences,
                &baseline,
            );
            let replacement_adopted = replacement.is_some();
            let (selected_channels, mut selected) = replacement.unwrap_or((channels, baseline));
            selected_channels
                .require_consumer_admission(self, &selected)
                .map_err(|error| ("selected_consumer_admission", error.to_string()))?;
            selected.bind_source_voice_gaps(gaps.to_vec());
            Ok(AdmittedTrial {
                replacement_adopted,
                selected,
            })
        });
        self.primary_beam_tie_trials = std::sync::Arc::new(PrimaryBeamTieTrials {
            targets,
            decisions,
            candidate,
        });
    }
}

pub(super) fn master_edge_states(
    plan: &FinalChordEncodingPlan,
    (before, after, receipt): (&str, &str, Option<&ExactTransformationReceipt>),
    edges: &[OwnerEdge<'_>],
    locators: &BTreeMap<usize, &FinalChordCanonicalSourceLocator>,
) -> Result<EdgeStates, &'static str> {
    let occurrences = MasterOccurrences::bind(plan, before, after, receipt)?;
    Ok(edges
        .iter()
        .map(|edge| {
            (
                (edge.left.fragment, edge.right.fragment),
                occurrences.read_edge(after, edge, locators),
            )
        })
        .collect())
}

pub(super) fn lost_present_edges(reference: &EdgeStates, trial: &EdgeStates) -> Vec<FragmentPair> {
    reference
        .iter()
        .filter_map(|(&pair, &state)| {
            (state == OwnerEdgeReadback::Present
                && trial.get(&pair) != Some(&OwnerEdgeReadback::Present))
            .then_some(pair)
        })
        .collect()
}

impl FinalChordEncodingPlan {
    pub(super) fn append_primary_beam_tie_trials(
        &self,
        out: &mut String,
        canonical: &CanonicalFinalChordStructuralPlan,
        capture: &ActualSourceBeamBindingObservation,
        reference_execution: (&str, &str, Option<&ExactTransformationReceipt>),
    ) {
        use OwnerEdgeReadback::{Present, SplitVoice, Unknown};
        let trial = &canonical.primary_beam_tie_trials;
        let _ = writeln!(
            out,
            "PrimaryBeamJointTieTrial: attempts={}; reference_ties={}; seam=master_before_countervoice_stems; authority=none",
            usize::from(trial.candidate.is_some()),
            trial.targets.len()
        );
        let Some(candidate) = &trial.candidate else {
            return;
        };
        out.push_str(&trial.decisions);
        let (selected, reference, states) = match (|| {
            let selected = candidate
                .as_ref()
                .map_err(|(stage, reason)| (*stage, reason.clone()))?;
            let edges = primary_edges(canonical, capture)
                .map_err(|reason| ("reference_capture", reason.to_owned()))?;
            let locators = canonical
                .source_locators
                .iter()
                .map(|row| (row.fragment_id, row))
                .collect();
            let reference = master_edge_states(self, reference_execution, &edges, &locators)
                .map_err(|reason| ("reference_master", reason.to_owned()))?;
            let before = reference_execution.0;
            let alpha =
                audit_musicxml_voice_label_binding_alpha(before, canonical, &selected.selected)
                    .map_err(|error| ("master_label_alpha", error.to_string()))?;
            if !alpha.green() {
                return Err((
                    "master_label_alpha",
                    format!(
                        "semantic_role_delta={}; paired_binding_failures={}; stale_binding_false_accepts={}",
                        alpha.semantic_role_deltas(),
                        alpha.paired_binding_failures(),
                        alpha.stale_binding_false_accepts()
                    ),
                ));
            }
            let plan =
                bind_canonical_final_chord_encoding_plan(before, canonical, &selected.selected)
                    .map_err(|error| ("master_plan_binding", error.to_string()))?;
            let (xml, receipt) = project_final_chord_encoding_plan(before, &plan);
            let xml = xml.map_err(|error| ("master_projection", error.to_string()))?;
            let states =
                master_edge_states(&plan, (before, &xml, receipt.as_ref()), &edges, &locators)
                    .map_err(|reason| ("master_readback", reason.to_owned()))?;
            Ok((selected, reference, states))
        })() {
            Ok(result) => result,
            Err((stage, reason)) => {
                let _ = writeln!(
                    out,
                    "PrimaryBeamJointTieResult: candidate_admitted={}; passed=false; reference_retained=true; stopped_at={stage}; reason={reason}; authority=none",
                    candidate.is_ok()
                );
                return;
            }
        };
        let lost = lost_present_edges(&reference, &states);
        let baseline_present = reference.values().filter(|&&v| v == Present).count();
        let trial_present = states.values().filter(|&&v| v == Present).count();
        let reference_complete = reference.values().all(|&v| v != Unknown);
        let mut target_count = 0;
        let mut recovered = 0;
        let mut targets_bound = true;
        for &pair in &trial.targets {
            let old = reference.get(&pair).copied();
            let new = states.get(&pair).copied();
            targets_bound &= matches!(old, Some(Present | SplitVoice));
            if old == Some(SplitVoice) {
                target_count += 1;
                recovered += usize::from(new == Some(Present));
            }
            let _ = writeln!(
                out,
                "PrimaryBeamJointTieTarget: fragments={pair:?}; reference={old:?}; master={new:?}; authority=none"
            );
        }
        for pair in &lost {
            let _ = writeln!(
                out,
                "PrimaryBeamJointTieLostEdge: fragments={pair:?}; master={:?}; authority=none",
                states.get(pair)
            );
        }
        let passed =
            reference_complete && targets_bound && recovered == target_count && lost.is_empty();
        let _ = writeln!(
            out,
            "PrimaryBeamJointTieResult: candidate_admitted=true; replacement_adopted={}; passed={passed}; reference_complete={reference_complete}; preserved={}/{}; recovered_targets={recovered}/{target_count}; master_present={trial_present}/{}; authority=none",
            selected.replacement_adopted,
            baseline_present - lost.len(),
            baseline_present,
            reference.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_balken_does_not_offset_a_lost_reference_edge() {
        use OwnerEdgeReadback::{Present, SplitVoice};
        let reference = [((1, 2), Present), ((2, 3), SplitVoice)].into();
        let trial = [((1, 2), SplitVoice), ((2, 3), Present)].into();
        assert_eq!(lost_present_edges(&reference, &trial), [(1, 2)]);
    }

    #[test]
    fn partner_preference_uses_distinct_best_channels_in_the_live_assignment() {
        let partners = [(4, [5, 6].into())].into();
        let mut assigned = [(5, 2), (6, 2)].into();
        let mut report = String::new();
        let ranked = [(0, 1), (0, 2), (0, 2), (0, 3), (1, 4)];
        assert_eq!(
            choose_joint_channel(4, 1, &ranked, &assigned, &partners, &mut report),
            2
        );
        assigned.insert(6, 3);
        assert_eq!(
            choose_joint_channel(4, 1, &ranked, &assigned, &partners, &mut report),
            1
        );
        assert!(report.contains("partner_channels={2, 3}"));
        assigned.insert(5, 4);
        assert_eq!(
            choose_joint_channel(4, 1, &ranked, &assigned, &partners, &mut report),
            3
        );
        assigned.insert(6, 4);
        assert_eq!(
            choose_joint_channel(4, 1, &ranked, &assigned, &partners, &mut report),
            1
        );
    }
}
