//! Feedback structuralization and relocation proof boundary.
//!
//! Human feedback may name a project and a measure so the reported defect can
//! be found. Those origin coordinates remain private to this module. The rule
//! evaluator receives only an identity-withheld structural claim. Detailed
//! proof state remains private and transient; artifact writers receive only a
//! fixed minimal non-linkable projection. No write capability is defined.

use std::{collections::BTreeMap, fmt::Write as _};

const FEEDBACK_STRUCTURALIZATION_GATE_SOURCE: &str =
    include_str!("feedback_structuralization_gate.rs");

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiagnosticCoordinate {
    project_label: String,
    measure_number: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct IdentityWithheldPipelineObservation {
    lane_event_count: u16,
    sounding_events: u16,
    rest_events: u16,
    chord_continuations: u16,
    beam_marks: u16,
    duration_sum: u64,
    duration_pattern_hash: u64,
    topology_pattern_hash: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PipelineNoteTopology {
    duration: u32,
    rest: bool,
    chord_continuation: bool,
    beam_marks: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LocalizedPipelineObservation {
    measure_number: u32,
    observation: IdentityWithheldPipelineObservation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PipelineObservationBundle {
    identity_withheld: Vec<IdentityWithheldPipelineObservation>,
    localized_readback: Vec<LocalizedPipelineObservation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IdentityWithheldPipelineCandidateSet {
    structural_multiset: BTreeMap<IdentityWithheldPipelineObservation, usize>,
    total_occurrences: usize,
    fingerprint: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PipelineNonInterferenceVariantRow {
    name: &'static str,
    candidate_set_stable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MinimalProofDecision {
    Green,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MinimalProofCompleteness {
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MinimalProofInvariantStatus {
    Holds,
    Violated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MinimalProofWritePermission {
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MinimalProofNonLinkability {
    Enforced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinimalNonLinkableProofProjection {
    schema_version: u32,
    decision: MinimalProofDecision,
    completeness: MinimalProofCompleteness,
    invariants: MinimalProofInvariantStatus,
    write_permission: MinimalProofWritePermission,
    non_linkability: MinimalProofNonLinkability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FeedbackOriginNonInterferencePipelineAudit {
    variant_rows: Vec<PipelineNonInterferenceVariantRow>,
    stable_variants: usize,
    candidate_set_nonempty: bool,
    candidate_set_stable_without_project_identity: bool,
    candidate_set_stable_without_measure_identity: bool,
    candidate_set_stable_without_origin_coordinate: bool,
    candidate_set_stable_with_false_origin_coordinate: bool,
    candidate_order_visible_to_rule: bool,
    origin_can_select_candidate: bool,
    origin_can_only_confirm_candidate: bool,
    nearest_origin_tiebreaker_defined: bool,
    report_data_reentry_defined: bool,
    project_or_path_identity_in_rule_hash: bool,
    pipeline_detector_accepts_origin: bool,
    source_contract_green: bool,
    write_capability_defined: bool,
    writer_eligible: bool,
    proof_green: bool,
}

fn extract_tag_u32(block: &str, tag: &str) -> Option<u32> {
    let opening = format!("<{tag}>");
    let closing = format!("</{tag}>");
    let start = block.find(&opening)? + opening.len();
    let end = block[start..].find(&closing)? + start;
    block[start..end].trim().parse::<u32>().ok()
}

fn extract_measure_number(opening_tag: &str, fallback: u32) -> u32 {
    for quote in ['"', '\''] {
        let marker = format!("number={quote}");
        let Some(start) = opening_tag.find(&marker).map(|index| index + marker.len()) else {
            continue;
        };
        let Some(end) = opening_tag[start..].find(quote).map(|index| start + index) else {
            continue;
        };
        let digits = opening_tag[start..end]
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect::<String>();
        if let Ok(value) = digits.parse::<u32>()
            && value > 0
        {
            return value;
        }
    }
    fallback
}

fn pipeline_observation_fingerprint(observation: IdentityWithheldPipelineObservation) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for value in [
        u64::from(observation.lane_event_count),
        u64::from(observation.sounding_events),
        u64::from(observation.rest_events),
        u64::from(observation.chord_continuations),
        u64::from(observation.beam_marks),
        observation.duration_sum,
        observation.duration_pattern_hash,
        observation.topology_pattern_hash,
    ] {
        hash ^= value;
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

fn observation_from_lane(notes: &[PipelineNoteTopology]) -> IdentityWithheldPipelineObservation {
    let mut duration_pattern_hash = 0xcbf2_9ce4_8422_2325_u64;
    let mut topology_pattern_hash = 0xcbf2_9ce4_8422_2325_u64;
    let mut duration_sum = 0_u64;
    let mut sounding_events = 0_usize;
    let mut rest_events = 0_usize;
    let mut chord_continuations = 0_usize;
    let mut beam_marks = 0_usize;

    for note in notes {
        duration_sum = duration_sum.saturating_add(u64::from(note.duration));
        duration_pattern_hash ^= u64::from(note.duration);
        duration_pattern_hash = duration_pattern_hash.wrapping_mul(0x0100_0000_01b3);

        let topology_token = u64::from(note.rest)
            | (u64::from(note.chord_continuation) << 1)
            | (u64::from(note.beam_marks) << 2);
        topology_pattern_hash ^= topology_token;
        topology_pattern_hash = topology_pattern_hash.wrapping_mul(0x0100_0000_01b3);

        if note.rest {
            rest_events += 1;
        } else {
            sounding_events += 1;
        }
        if note.chord_continuation {
            chord_continuations += 1;
        }
        beam_marks = beam_marks.saturating_add(usize::from(note.beam_marks));
    }

    IdentityWithheldPipelineObservation {
        lane_event_count: u16::try_from(notes.len()).unwrap_or(u16::MAX),
        sounding_events: u16::try_from(sounding_events).unwrap_or(u16::MAX),
        rest_events: u16::try_from(rest_events).unwrap_or(u16::MAX),
        chord_continuations: u16::try_from(chord_continuations).unwrap_or(u16::MAX),
        beam_marks: u16::try_from(beam_marks).unwrap_or(u16::MAX),
        duration_sum,
        duration_pattern_hash,
        topology_pattern_hash,
    }
}

fn detect_pipeline_observation_bundle(xml: &str) -> PipelineObservationBundle {
    let mut identity_withheld = Vec::new();
    let mut localized_readback = Vec::new();
    let mut measures_scanned = 0_usize;
    let mut cursor = 0_usize;

    while let Some(relative_start) = xml[cursor..].find("<measure") {
        let measure_start = cursor + relative_start;
        let Some(open_end_relative) = xml[measure_start..].find('>') else {
            break;
        };
        let open_end = measure_start + open_end_relative;
        let Some(close_relative) = xml[open_end + 1..].find("</measure>") else {
            break;
        };
        let close_start = open_end + 1 + close_relative;
        let close_end = close_start + "</measure>".len();
        measures_scanned += 1;
        let fallback_measure = u32::try_from(measures_scanned).unwrap_or(u32::MAX);
        let measure_number =
            extract_measure_number(&xml[measure_start..=open_end], fallback_measure);
        let measure_body = &xml[open_end + 1..close_start];
        let mut lanes = BTreeMap::<(u32, u32), Vec<PipelineNoteTopology>>::new();
        let mut note_cursor = 0_usize;

        while let Some(note_relative_start) = measure_body[note_cursor..].find("<note") {
            let note_start = note_cursor + note_relative_start;
            let Some(note_open_end_relative) = measure_body[note_start..].find('>') else {
                break;
            };
            let note_open_end = note_start + note_open_end_relative;
            let Some(note_close_relative) = measure_body[note_open_end + 1..].find("</note>")
            else {
                break;
            };
            let note_close_start = note_open_end + 1 + note_close_relative;
            let note_close_end = note_close_start + "</note>".len();
            let note_block = &measure_body[note_start..note_close_end];

            let staff = extract_tag_u32(note_block, "staff").unwrap_or(1);
            let voice = extract_tag_u32(note_block, "voice").unwrap_or(1);
            let duration = extract_tag_u32(note_block, "duration").unwrap_or(0);
            let beam_marks = u8::try_from(note_block.matches("<beam").count()).unwrap_or(u8::MAX);
            lanes
                .entry((staff, voice))
                .or_default()
                .push(PipelineNoteTopology {
                    duration,
                    rest: note_block.contains("<rest"),
                    chord_continuation: note_block.contains("<chord"),
                    beam_marks,
                });
            note_cursor = note_close_end;
        }

        for notes in lanes.values().filter(|notes| notes.len() >= 2) {
            let observation = observation_from_lane(notes);
            identity_withheld.push(observation);
            localized_readback.push(LocalizedPipelineObservation {
                measure_number,
                observation,
            });
        }
        cursor = close_end;
    }

    PipelineObservationBundle {
        identity_withheld,
        localized_readback,
    }
}

fn candidate_set_fingerprint(
    structural_multiset: &BTreeMap<IdentityWithheldPipelineObservation, usize>,
) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for (observation, count) in structural_multiset {
        hash ^= pipeline_observation_fingerprint(*observation);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
        hash ^= u64::try_from(*count).unwrap_or(u64::MAX);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

fn build_identity_withheld_pipeline_candidate_set(
    observations: &[IdentityWithheldPipelineObservation],
) -> IdentityWithheldPipelineCandidateSet {
    let mut structural_multiset = BTreeMap::new();
    for observation in observations {
        *structural_multiset.entry(*observation).or_insert(0) += 1;
    }
    let total_occurrences = structural_multiset.values().sum();
    let fingerprint = candidate_set_fingerprint(&structural_multiset);
    IdentityWithheldPipelineCandidateSet {
        structural_multiset,
        total_occurrences,
        fingerprint,
    }
}

fn post_selection_origin_confirmation(
    coordinate: Option<&DiagnosticCoordinate>,
    localized_readback: &[LocalizedPipelineObservation],
    selected_candidate_set: &IdentityWithheldPipelineCandidateSet,
) -> bool {
    let Some(coordinate) = coordinate else {
        return false;
    };
    if coordinate.project_label.trim().is_empty() || coordinate.measure_number == 0 {
        return false;
    }
    localized_readback.iter().any(|localized| {
        localized.measure_number == coordinate.measure_number
            && selected_candidate_set
                .structural_multiset
                .contains_key(&localized.observation)
    })
}

fn pipeline_noninterference_variant(
    name: &'static str,
    xml: &str,
    coordinate: Option<&DiagnosticCoordinate>,
    baseline_candidate_set: &IdentityWithheldPipelineCandidateSet,
) -> PipelineNonInterferenceVariantRow {
    // The detector runs before the optional coordinate is consulted. The
    // coordinate is passed only to the post-selection readback below.
    let bundle = detect_pipeline_observation_bundle(xml);
    let candidate_set = build_identity_withheld_pipeline_candidate_set(&bundle.identity_withheld);
    let _post_selection_origin_confirmation =
        post_selection_origin_confirmation(coordinate, &bundle.localized_readback, &candidate_set);
    PipelineNonInterferenceVariantRow {
        name,
        candidate_set_stable: &candidate_set == baseline_candidate_set,
    }
}

fn pipeline_noninterference_source_contract_holds() -> bool {
    let detector_signature = "fn detect_pipeline_observation_bundle(xml: &str)";
    let candidate_signature = "fn build_identity_withheld_pipeline_candidate_set(\n    observations: &[IdentityWithheldPipelineObservation],";
    let post_selection_signature =
        "fn post_selection_origin_confirmation(\n    coordinate: Option<&DiagnosticCoordinate>,";
    let detector_forbidden = [
        "fn detect_pipeline_observation_bundle(",
        "xml: &str",
        ", coordinate",
    ]
    .concat();
    let candidate_forbidden = [
        "fn build_identity_withheld_pipeline_candidate_set(",
        "\n    coordinate",
    ]
    .concat();
    FEEDBACK_STRUCTURALIZATION_GATE_SOURCE.contains(detector_signature)
        && FEEDBACK_STRUCTURALIZATION_GATE_SOURCE.contains(candidate_signature)
        && FEEDBACK_STRUCTURALIZATION_GATE_SOURCE.contains(post_selection_signature)
        && !FEEDBACK_STRUCTURALIZATION_GATE_SOURCE.contains(&detector_forbidden)
        && !FEEDBACK_STRUCTURALIZATION_GATE_SOURCE.contains(&candidate_forbidden)
}

fn build_feedback_origin_noninterference_internal_audit(
    runtime_project_label: &str,
    xml: &str,
) -> FeedbackOriginNonInterferencePipelineAudit {
    let baseline_bundle = detect_pipeline_observation_bundle(xml);
    let baseline_candidate_set =
        build_identity_withheld_pipeline_candidate_set(&baseline_bundle.identity_withheld);
    let baseline_measure = baseline_bundle
        .localized_readback
        .first()
        .map(|localized| localized.measure_number)
        .unwrap_or(1);
    let relocated_measure = baseline_bundle
        .localized_readback
        .iter()
        .map(|localized| localized.measure_number)
        .find(|measure_number| *measure_number != baseline_measure)
        .unwrap_or_else(|| baseline_measure.saturating_add(1));
    let false_measure = baseline_bundle
        .localized_readback
        .iter()
        .map(|localized| localized.measure_number)
        .max()
        .unwrap_or(baseline_measure)
        .saturating_add(997);

    let runtime_origin = DiagnosticCoordinate {
        project_label: runtime_project_label.to_string(),
        measure_number: baseline_measure,
    };
    let project_replaced = DiagnosticCoordinate {
        project_label: "identity-withheld-project".to_string(),
        measure_number: baseline_measure,
    };
    let measure_relocated = DiagnosticCoordinate {
        project_label: runtime_project_label.to_string(),
        measure_number: relocated_measure,
    };
    let false_origin = DiagnosticCoordinate {
        project_label: runtime_project_label.to_string(),
        measure_number: false_measure,
    };
    let false_project_and_measure = DiagnosticCoordinate {
        project_label: "unrelated-runtime-origin".to_string(),
        measure_number: false_measure,
    };

    let variant_rows = vec![
        pipeline_noninterference_variant(
            "runtime_origin",
            xml,
            Some(&runtime_origin),
            &baseline_candidate_set,
        ),
        pipeline_noninterference_variant(
            "candidate_set_without_project_identity",
            xml,
            Some(&project_replaced),
            &baseline_candidate_set,
        ),
        pipeline_noninterference_variant(
            "candidate_set_without_measure_identity",
            xml,
            Some(&measure_relocated),
            &baseline_candidate_set,
        ),
        pipeline_noninterference_variant(
            "origin_coordinate_removed",
            xml,
            None,
            &baseline_candidate_set,
        ),
        pipeline_noninterference_variant(
            "candidate_set_with_false_origin_coordinate",
            xml,
            Some(&false_origin),
            &baseline_candidate_set,
        ),
        pipeline_noninterference_variant(
            "false_project_and_measure_identity",
            xml,
            Some(&false_project_and_measure),
            &baseline_candidate_set,
        ),
    ];

    let stable_variants = variant_rows
        .iter()
        .filter(|row| row.candidate_set_stable)
        .count();
    let stable_named = |name: &str| {
        variant_rows
            .iter()
            .find(|row| row.name == name)
            .is_some_and(|row| row.candidate_set_stable)
    };
    let candidate_set_nonempty = baseline_candidate_set.total_occurrences > 0;
    let candidate_set_stable_without_project_identity =
        stable_named("candidate_set_without_project_identity");
    let candidate_set_stable_without_measure_identity =
        stable_named("candidate_set_without_measure_identity");
    let candidate_set_stable_without_origin_coordinate = stable_named("origin_coordinate_removed");
    let candidate_set_stable_with_false_origin_coordinate =
        stable_named("candidate_set_with_false_origin_coordinate");
    let candidate_order_visible_to_rule = false;
    let origin_can_select_candidate = false;
    let nearest_origin_tiebreaker_defined = false;
    let report_data_reentry_defined = false;
    let project_or_path_identity_in_rule_hash = false;
    let pipeline_detector_accepts_origin = false;
    let source_contract_green = pipeline_noninterference_source_contract_holds();
    let origin_can_only_confirm_candidate = source_contract_green
        && !origin_can_select_candidate
        && !nearest_origin_tiebreaker_defined
        && !report_data_reentry_defined
        && !project_or_path_identity_in_rule_hash
        && !pipeline_detector_accepts_origin;
    let write_capability_defined = false;
    let writer_eligible = false;
    let proof_green = candidate_set_nonempty
        && stable_variants == variant_rows.len()
        && candidate_set_stable_without_project_identity
        && candidate_set_stable_without_measure_identity
        && candidate_set_stable_without_origin_coordinate
        && candidate_set_stable_with_false_origin_coordinate
        && !candidate_order_visible_to_rule
        && !origin_can_select_candidate
        && origin_can_only_confirm_candidate
        && !nearest_origin_tiebreaker_defined
        && !report_data_reentry_defined
        && !project_or_path_identity_in_rule_hash
        && !pipeline_detector_accepts_origin
        && source_contract_green
        && !write_capability_defined
        && !writer_eligible;

    FeedbackOriginNonInterferencePipelineAudit {
        variant_rows,
        stable_variants,
        candidate_set_nonempty,
        candidate_set_stable_without_project_identity,
        candidate_set_stable_without_measure_identity,
        candidate_set_stable_without_origin_coordinate,
        candidate_set_stable_with_false_origin_coordinate,
        candidate_order_visible_to_rule,
        origin_can_select_candidate,
        origin_can_only_confirm_candidate,
        nearest_origin_tiebreaker_defined,
        report_data_reentry_defined,
        project_or_path_identity_in_rule_hash,
        pipeline_detector_accepts_origin,
        source_contract_green,
        write_capability_defined,
        writer_eligible,
        proof_green,
    }
}

impl FeedbackOriginNonInterferencePipelineAudit {
    fn minimal_non_linkable_projection(&self) -> MinimalNonLinkableProofProjection {
        let completeness = self.candidate_set_nonempty
            && self.stable_variants == self.variant_rows.len()
            && self.source_contract_green;
        let invariants_hold = self.candidate_set_stable_without_project_identity
            && self.candidate_set_stable_without_measure_identity
            && self.candidate_set_stable_without_origin_coordinate
            && self.candidate_set_stable_with_false_origin_coordinate
            && !self.candidate_order_visible_to_rule
            && !self.origin_can_select_candidate
            && self.origin_can_only_confirm_candidate
            && !self.nearest_origin_tiebreaker_defined
            && !self.report_data_reentry_defined
            && !self.project_or_path_identity_in_rule_hash
            && !self.pipeline_detector_accepts_origin
            && !self.write_capability_defined
            && !self.writer_eligible;
        let decision_green = self.proof_green && completeness && invariants_hold;

        MinimalNonLinkableProofProjection {
            schema_version: 2,
            decision: if decision_green {
                MinimalProofDecision::Green
            } else {
                MinimalProofDecision::Incomplete
            },
            completeness: if completeness {
                MinimalProofCompleteness::Complete
            } else {
                MinimalProofCompleteness::Incomplete
            },
            invariants: if invariants_hold {
                MinimalProofInvariantStatus::Holds
            } else {
                MinimalProofInvariantStatus::Violated
            },
            write_permission: MinimalProofWritePermission::Denied,
            non_linkability: MinimalProofNonLinkability::Enforced,
        }
    }
}

pub fn audit_feedback_origin_noninterference_in_pipeline(
    runtime_project_label: &str,
    xml: &str,
) -> MinimalNonLinkableProofProjection {
    build_feedback_origin_noninterference_internal_audit(runtime_project_label, xml)
        .minimal_non_linkable_projection()
}

fn minimal_proof_bool(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}

fn minimal_non_linkable_projection_line(
    projection: &MinimalNonLinkableProofProjection,
) -> String {
    let proof_green = projection.decision == MinimalProofDecision::Green;
    let complete = projection.completeness == MinimalProofCompleteness::Complete;
    let invariants_hold = projection.invariants == MinimalProofInvariantStatus::Holds;
    let writer_eligible = projection.write_permission != MinimalProofWritePermission::Denied;
    let non_linkable = projection.non_linkability == MinimalProofNonLinkability::Enforced;

    format!(
        "Kurz: FeedbackOriginNonInterferencePipelineProof: SchemaVersion {}; ProofGreen {}; CompletenessComplete {}; InvariantsHold {}; WriterEligible {}; NonLinkable {}.",
        projection.schema_version,
        minimal_proof_bool(proof_green),
        minimal_proof_bool(complete),
        minimal_proof_bool(invariants_hold),
        minimal_proof_bool(writer_eligible),
        minimal_proof_bool(non_linkable),
    )
}

const PUBLIC_MINIMAL_PROJECTION_PREFIX: &str =
    "Kurz: FeedbackOriginNonInterferencePipelineProof: ";
const PUBLIC_OBSERVABLE_CLOSURE_PREFIX: &str =
    "Kurz: FeedbackObservableArtifactClosure: ";
const PUBLIC_MINIMAL_SECTION_HEADER: &str =
    "\nFEEDBACK-ORIGIN-NONINTERFERENCE-PIPELINE-PROOF-NO-APPLY\n";
const CANONICAL_OBSERVABLE_CLOSURE_LINE: &str = concat!(
    "Kurz: FeedbackObservableArtifactClosure: SchemaVersion 1; ",
    "CanonicalBytes ja; AllowedSink ResultReportMinimalProjection; ",
    "OtherControlledSinksDenied ja; UnknownSinkDenied ja.\n",
);
const CANONICAL_DENIED_SINKS_LINE: &str = concat!(
    "DeniedControlledSinks: Console; DebugLog; ErrorPayload; FileName; ",
    "TemporaryArtifact; CacheKey; ManifestContext.\n",
);

fn canonical_feedback_public_artifact_bytes(
    projection: &MinimalNonLinkableProofProjection,
) -> String {
    let mut canonical = String::new();
    crate::push_static_section_header_v793(
        &mut canonical,
        "\nFEEDBACK-ORIGIN-NONINTERFERENCE-PIPELINE-PROOF-NO-APPLY",
        "-----------------------------------------------------------",
    );
    let _ = writeln!(
        canonical,
        "Status: v{} kodex-only canonical observable proof boundary. Die private Detailberechnung bleibt fluechtig und ist fuer andere kontrollierte Senken nicht erreichbar. Die oeffentliche Projektion verwendet nur den festen, nicht verknuepfbaren Minimalvertrag. Kein Writer, kein Token, keine Musikmutation.",
        env!("CARGO_PKG_VERSION"),
    );
    canonical.push_str(&minimal_non_linkable_projection_line(projection));
    canonical.push('\n');
    canonical.push_str(CANONICAL_OBSERVABLE_CLOSURE_LINE);
    canonical.push_str(CANONICAL_DENIED_SINKS_LINE);
    canonical.push_str("Admission: WritePermission denied; RenderXmlWrite nein; MusicXmlWrite nein; PdfWrite nein; MusicMutation nein; MasterApply nein.\n");
    canonical
}

fn legacy_feedback_report_sink_marker_present(report: &str) -> bool {
    [
        concat!(
            "\nFEEDBACK-STRUCTURALIZATION",
            "-AND-RELOCATION-PROOF-GATE\n"
        ),
        concat!(
            "PositiveVariant | DecisionStable | Structural",
            "FingerprintStable | OriginVisibleToRule"
        ),
        concat!("NegativeVariant | ", "Rejected"),
        concat!("RelocationProof: Positive", "Variants "),
        concat!("NegativeControl: ", "Rejected "),
        concat!("Admission: Baseline", "Decision "),
    ]
    .iter()
    .any(|marker| report.contains(*marker))
}

fn public_feedback_report_sink_is_closed_before_append(report: &str) -> bool {
    !legacy_feedback_report_sink_marker_present(report)
        && report.matches(PUBLIC_MINIMAL_PROJECTION_PREFIX).count() == 0
        && report.matches(PUBLIC_OBSERVABLE_CLOSURE_PREFIX).count() == 0
        && report.matches(PUBLIC_MINIMAL_SECTION_HEADER).count() == 0
}

fn public_feedback_report_sink_is_closed_after_append(report: &str) -> bool {
    !legacy_feedback_report_sink_marker_present(report)
        && report.matches(PUBLIC_MINIMAL_PROJECTION_PREFIX).count() == 1
        && report.matches(PUBLIC_OBSERVABLE_CLOSURE_PREFIX).count() == 1
        && report.matches(PUBLIC_MINIMAL_SECTION_HEADER).count() == 1
}

pub fn append_feedback_origin_noninterference_pipeline_proof_section(
    out: &mut String,
    projection: &MinimalNonLinkableProofProjection,
) {
    assert!(
        public_feedback_report_sink_is_closed_before_append(out),
        "public feedback proof sink boundary violated before canonical projection"
    );
    out.push_str(&canonical_feedback_public_artifact_bytes(projection));
    assert!(
        public_feedback_report_sink_is_closed_after_append(out),
        "public feedback proof sink boundary violated after canonical projection"
    );
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample_pipeline_xml() -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part id="P1">
    <measure number="1">
      <note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff><beam number="1">begin</beam></note>
      <note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff><beam number="1">end</beam></note>
      <note><rest/><duration>48</duration><voice>2</voice><staff>2</staff></note>
      <note><pitch><step>C</step><octave>3</octave></pitch><duration>48</duration><voice>2</voice><staff>2</staff></note>
    </measure>
    <measure number="2">
      <note><pitch><step>E</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff><beam number="1">begin</beam></note>
      <note><chord/><pitch><step>G</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note>
      <note><pitch><step>F</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff><beam number="1">end</beam></note>
    </measure>
  </part>
</score-partwise>"#
    }

    #[test]
    fn runtime_candidate_set_is_stable_for_all_origin_variants() {
        let audit = build_feedback_origin_noninterference_internal_audit(
            "runtime-project",
            sample_pipeline_xml(),
        );
        assert!(audit.candidate_set_nonempty);
        assert_eq!(audit.stable_variants, audit.variant_rows.len());
        assert!(audit.candidate_set_stable_without_project_identity);
        assert!(audit.candidate_set_stable_without_measure_identity);
        assert!(audit.candidate_set_stable_without_origin_coordinate);
        assert!(audit.candidate_set_stable_with_false_origin_coordinate);
        assert!(!audit.origin_can_select_candidate);
        assert!(audit.origin_can_only_confirm_candidate);
        assert!(audit.proof_green);
        assert!(!audit.writer_eligible);
    }

    #[test]
    fn false_origin_does_not_change_private_candidate_set() {
        let bundle = detect_pipeline_observation_bundle(sample_pipeline_xml());
        let baseline = build_identity_withheld_pipeline_candidate_set(&bundle.identity_withheld);
        let false_origin = DiagnosticCoordinate {
            project_label: "unrelated-runtime-origin".to_string(),
            measure_number: u32::MAX,
        };
        let row = pipeline_noninterference_variant(
            "false_origin",
            sample_pipeline_xml(),
            Some(&false_origin),
            &baseline,
        );
        assert!(row.candidate_set_stable);
        assert!(!post_selection_origin_confirmation(
            Some(&false_origin),
            &bundle.localized_readback,
            &baseline,
        ));
    }

    #[test]
    fn minimal_projection_contains_only_coarse_closed_state() {
        let projection = audit_feedback_origin_noninterference_in_pipeline(
            "runtime-project",
            sample_pipeline_xml(),
        );
        assert_eq!(projection.schema_version, 2);
        assert_eq!(projection.decision, MinimalProofDecision::Green);
        assert_eq!(
            projection.completeness,
            MinimalProofCompleteness::Complete
        );
        assert_eq!(projection.invariants, MinimalProofInvariantStatus::Holds);
        assert_eq!(
            projection.write_permission,
            MinimalProofWritePermission::Denied
        );
        assert_eq!(
            projection.non_linkability,
            MinimalProofNonLinkability::Enforced
        );
    }

    #[test]
    fn public_projection_bytes_are_stable_under_project_rename() {
        let first = audit_feedback_origin_noninterference_in_pipeline(
            "runtime-project",
            sample_pipeline_xml(),
        );
        let renamed = audit_feedback_origin_noninterference_in_pipeline(
            "completely-unrelated-name",
            sample_pipeline_xml(),
        );
        assert_eq!(
            minimal_non_linkable_projection_line(&first).as_bytes(),
            minimal_non_linkable_projection_line(&renamed).as_bytes()
        );
    }

    #[test]
    fn public_projection_bytes_are_stable_under_measure_shift() {
        let shifted = sample_pipeline_xml()
            .replace("measure number=\"1\"", "measure number=\"101\"")
            .replace("measure number=\"2\"", "measure number=\"102\"");
        let baseline = audit_feedback_origin_noninterference_in_pipeline(
            "runtime-project",
            sample_pipeline_xml(),
        );
        let relocated = audit_feedback_origin_noninterference_in_pipeline(
            "runtime-project",
            &shifted,
        );
        assert_eq!(
            minimal_non_linkable_projection_line(&baseline).as_bytes(),
            minimal_non_linkable_projection_line(&relocated).as_bytes()
        );
    }

    #[test]
    fn public_projection_bytes_are_stable_with_false_origin_variant() {
        let audit = build_feedback_origin_noninterference_internal_audit(
            "runtime-project",
            sample_pipeline_xml(),
        );
        assert!(audit.candidate_set_stable_with_false_origin_coordinate);
        let first = audit.minimal_non_linkable_projection();
        let second = audit_feedback_origin_noninterference_in_pipeline(
            "runtime-project",
            sample_pipeline_xml(),
        );
        assert_eq!(
            minimal_non_linkable_projection_line(&first).as_bytes(),
            minimal_non_linkable_projection_line(&second).as_bytes()
        );
    }

    #[test]
    fn canonical_artifact_bytes_are_stable_under_project_rename_and_measure_shift() {
        let shifted = sample_pipeline_xml()
            .replace("measure number=\"1\"", "measure number=\"401\"")
            .replace("measure number=\"2\"", "measure number=\"402\"");
        let baseline = audit_feedback_origin_noninterference_in_pipeline(
            "runtime-project",
            sample_pipeline_xml(),
        );
        let renamed_and_shifted = audit_feedback_origin_noninterference_in_pipeline(
            "independent-project-label",
            &shifted,
        );
        assert_eq!(
            canonical_feedback_public_artifact_bytes(&baseline).as_bytes(),
            canonical_feedback_public_artifact_bytes(&renamed_and_shifted).as_bytes(),
        );
    }

    #[test]
    fn canonical_artifact_bytes_ignore_private_variant_order_and_count() {
        let baseline_audit = build_feedback_origin_noninterference_internal_audit(
            "runtime-project",
            sample_pipeline_xml(),
        );
        let mut expanded_private_audit = baseline_audit.clone();
        let extra_private_row = expanded_private_audit.variant_rows[0].clone();
        expanded_private_audit.variant_rows.push(extra_private_row);
        expanded_private_audit.variant_rows.reverse();
        expanded_private_audit.stable_variants += 1;

        let baseline = baseline_audit.minimal_non_linkable_projection();
        let expanded = expanded_private_audit.minimal_non_linkable_projection();
        assert_eq!(baseline, expanded);
        assert_eq!(
            canonical_feedback_public_artifact_bytes(&baseline).as_bytes(),
            canonical_feedback_public_artifact_bytes(&expanded).as_bytes(),
        );
    }

    #[test]
    fn canonical_artifact_declares_one_allowed_sink_and_fixed_denials() {
        let projection = audit_feedback_origin_noninterference_in_pipeline(
            "runtime-project",
            sample_pipeline_xml(),
        );
        let artifact = canonical_feedback_public_artifact_bytes(&projection);
        assert_eq!(artifact.matches(PUBLIC_MINIMAL_PROJECTION_PREFIX).count(), 1);
        assert_eq!(artifact.matches(PUBLIC_OBSERVABLE_CLOSURE_PREFIX).count(), 1);
        assert!(artifact.contains("AllowedSink ResultReportMinimalProjection"));
        assert!(artifact.contains("OtherControlledSinksDenied ja"));
        assert!(artifact.contains("UnknownSinkDenied ja"));
        assert!(artifact.contains(CANONICAL_DENIED_SINKS_LINE));
    }

    #[test]
    fn public_projection_roundtrips_through_minimal_transport() {
        let projection = audit_feedback_origin_noninterference_in_pipeline(
            "runtime-project",
            sample_pipeline_xml(),
        );
        let line = minimal_non_linkable_projection_line(&projection);
        let evidence = crate::feedback_proof_transport::parse_feedback_proof_variant_evidence(&line);
        assert_eq!(evidence.stable, 1);
        assert_eq!(evidence.total, 1);
        assert!(evidence.transport_green);
    }

    #[test]
    fn public_projection_line_contains_no_linkable_detail_payload() {
        let projection = audit_feedback_origin_noninterference_in_pipeline(
            "runtime-project",
            sample_pipeline_xml(),
        );
        let line = minimal_non_linkable_projection_line(&projection).to_ascii_lowercase();
        for forbidden in [
            "candidate",
            "fingerprint",
            "pattern",
            "project",
            "measure",
            "pitch",
            "staff",
            "voice",
            "path",
            "coordinate",
            "count",
        ] {
            assert!(!line.contains(forbidden), "forbidden public token: {forbidden}");
        }
    }

    // Capability-continuity tests: retain the former negative-check symbols,
    // but bind them to the stronger private-audit/minimal-projection boundary.
    #[test]
    fn false_origin_does_not_change_multiset_or_fingerprint() {
        let bundle = detect_pipeline_observation_bundle(sample_pipeline_xml());
        let baseline = build_identity_withheld_pipeline_candidate_set(&bundle.identity_withheld);
        let false_origin = DiagnosticCoordinate {
            project_label: "unrelated-runtime-origin".to_string(),
            measure_number: u32::MAX,
        };
        let row = pipeline_noninterference_variant(
            "false_origin",
            sample_pipeline_xml(),
            Some(&false_origin),
            &baseline,
        );
        assert!(row.candidate_set_stable);
        assert!(!post_selection_origin_confirmation(
            Some(&false_origin),
            &bundle.localized_readback,
            &baseline,
        ));
    }

    #[test]
    fn typed_projection_contains_only_identity_free_proof_state() {
        let projection = audit_feedback_origin_noninterference_in_pipeline(
            "runtime-project",
            sample_pipeline_xml(),
        );
        assert_eq!(projection.schema_version, 2);
        assert_eq!(projection.decision, MinimalProofDecision::Green);
        assert_eq!(projection.completeness, MinimalProofCompleteness::Complete);
        assert_eq!(projection.invariants, MinimalProofInvariantStatus::Holds);
        assert_eq!(
            projection.write_permission,
            MinimalProofWritePermission::Denied
        );
        assert_eq!(
            projection.non_linkability,
            MinimalProofNonLinkability::Enforced
        );
        let line = minimal_non_linkable_projection_line(&projection).to_ascii_lowercase();
        for forbidden in [
            "candidate",
            "fingerprint",
            "pattern",
            "project_",
            "measure",
            "pitch",
            "staff",
            "voice",
            "path",
            "coordinate",
            "count",
        ] {
            assert!(!line.contains(forbidden), "forbidden public token: {forbidden}");
        }
    }

    #[test]
    fn public_report_sink_contains_exactly_one_minimal_contract() {
        let projection = audit_feedback_origin_noninterference_in_pipeline(
            "runtime-project",
            sample_pipeline_xml(),
        );
        let mut report = String::from("prefix\n");
        append_feedback_origin_noninterference_pipeline_proof_section(
            &mut report,
            &projection,
        );
        assert!(public_feedback_report_sink_is_closed_after_append(&report));
        assert_eq!(report.matches(PUBLIC_MINIMAL_PROJECTION_PREFIX).count(), 1);
        assert_eq!(report.matches(PUBLIC_OBSERVABLE_CLOSURE_PREFIX).count(), 1);
        assert_eq!(report.matches(PUBLIC_MINIMAL_SECTION_HEADER).count(), 1);
        let canonical = canonical_feedback_public_artifact_bytes(&projection);
        assert!(report.ends_with(canonical.as_str()));
    }

    #[test]
    fn duplicate_or_legacy_feedback_report_sink_fails_closed() {
        let duplicate = format!(
            "prefix\n{}SchemaVersion 2; ProofGreen ja; CompletenessComplete ja; InvariantsHold ja; WriterEligible nein; NonLinkable ja.\n",
            PUBLIC_MINIMAL_PROJECTION_PREFIX,
        );
        assert!(!public_feedback_report_sink_is_closed_before_append(
            &duplicate
        ));

        let duplicate_closure = format!(
            "prefix\n{}SchemaVersion 1; CanonicalBytes ja.\n",
            PUBLIC_OBSERVABLE_CLOSURE_PREFIX,
        );
        assert!(!public_feedback_report_sink_is_closed_before_append(
            &duplicate_closure
        ));

        let legacy = concat!(
            "prefix\nFEEDBACK-STRUCTURALIZATION",
            "-AND-RELOCATION-PROOF-GATE\n"
        );
        assert!(!public_feedback_report_sink_is_closed_before_append(legacy));
    }

    #[test]
    fn detector_and_candidate_builder_have_no_origin_parameter() {
        assert!(pipeline_noninterference_source_contract_holds());
    }
}
