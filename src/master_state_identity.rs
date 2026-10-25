//! Master state identity and ordered, sovereign Master lineage.
//!
//! Identity is minted at the creation or transformation event as an opaque,
//! process-local value. It is never derived from XML, fingerprints, locators,
//! profiles or musical coordinates, and validation never recomputes it.
//! Fingerprints exist only for continuity, integrity and binding.
//!
//! A transformation receipt proves one concrete owner execution (one event).
//! Master or Render is not part of the receipt type; the domain is bound only
//! when an adoption consumes the receipt (r1217 neutral receipt names). An
//! unadopted receipt is move-only: it cannot be cloned or shared between
//! variants. Adoption converts it into a clonable `CommittedTransformationEvent`
//! of the lineage, so cloned candidates share committed history, never a
//! still-available receipt.
//!
//! Minting is sealed: every identity is minted through an owner capability type
//! that only its owner module can construct, and the owner type is bound to one
//! frozen population descriptor or frozen PRESERVING/ADDITIVE exclusion contract
//! in this module. Foreign modules can transport an existing receipt but cannot
//! mint a new one.
//!
//! The lineage is `GENESIS -> ordered committed owner events*`. There is no
//! untrusted gap element any more (r1219 Master sovereignty closure): a Master
//! transition that is not continuously covered by verified owner receipts is a
//! fatal sovereignty violation, never a lineage element. Lineage is provenance,
//! never write authority.

use anyhow::ensure;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0100_0000_01b3;

fn fingerprint(text: &str) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Fatal Master sovereignty violation: a productive Master transition without
/// complete, continuous and verified owner-receipt coverage. There is no
/// fallback: the run aborts instead of choosing another candidate or recording
/// an untrusted gap.
#[track_caller]
pub(crate) fn master_sovereignty_violation(detail: &str) -> ! {
    panic!("MASTER SOVEREIGNTY VIOLATION: {detail}")
}

// ---------------------------------------------------------------------------
// Opaque minted identity
// ---------------------------------------------------------------------------

static NEXT_MINTED_IDENTITY: AtomicU64 = AtomicU64::new(1);

/// Opaque identity minted once per creation/transformation event. The raw value
/// is never formatted into any output; `Debug` prints only a fixed marker. It is
/// copyable as a value, but a receipt can only be constructed by a sealed mint.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct MintedIdentity(u64);

impl MintedIdentity {
    fn mint_fresh() -> Self {
        Self(NEXT_MINTED_IDENTITY.fetch_add(1, Ordering::Relaxed))
    }

    fn opaque_value(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for MintedIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("MintedIdentity(opaque)")
    }
}

// ---------------------------------------------------------------------------
// Frozen population descriptors (semantic anchor = owner + signature + contract)
// ---------------------------------------------------------------------------


/// Read-only receipt-origin capability. Its constructor remains private here.
pub(crate) struct SscReceiptOriginAccess { _private: () }
const SSC_ORIGIN_ACCESS: SscReceiptOriginAccess = SscReceiptOriginAccess { _private: () };
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EffectContract {
    Destructive,
    Unknown,
}

impl EffectContract {
    fn contract_label(self) -> &'static str {
        match self {
            Self::Destructive => "DESTRUCTIVE",
            Self::Unknown => "UNKNOWN",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IdentityContract {
    Genesis,
    Exact,
    BlockNote,
    BlockMeasure,
    BlockDocument,
}

impl IdentityContract {
    fn contract_label(self) -> &'static str {
        match self {
            Self::Genesis => "GENESIS",
            Self::Exact => "EXACT",
            Self::BlockNote => "BLOCK_NOTE",
            Self::BlockMeasure => "BLOCK_MEASURE",
            Self::BlockDocument => "BLOCK_DOCUMENT",
        }
    }

    fn is_block(self) -> bool {
        matches!(self, Self::BlockNote | Self::BlockMeasure | Self::BlockDocument)
    }
}

/// Frozen population descriptor. `population_report_row` is reporting metadata
/// only; the semantic anchor is owner, owner signature, effect contract and
/// identity contract.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TransformationDescriptor {
    population_report_row: usize,
    owner: &'static str,
    owner_signature: &'static str,
    effect: EffectContract,
    identity: IdentityContract,
}

impl TransformationDescriptor {
    fn semantic_anchor(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.owner,
            self.owner_signature,
            self.effect.contract_label(),
            self.identity.contract_label()
        )
    }

    fn same_contract(&self, other: &TransformationDescriptor) -> bool {
        self.semantic_anchor() == other.semantic_anchor()
    }
}

const CURSOR_NEUTRAL_RENDER_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 2,
    owner: "src/structural_write_authority/writer_entry_boundary.rs::apply_authorized_serialization_plan",
    owner_signature: "fn apply_authorized_serialization_plan( source_xml: &str, locator: &super::StructuralLocator, plan: &AuthorizedSerializationPlan, source_stage: &str, audit: &mut AuthorizedWriterEntryBoundaryAudit, direction_relocation_plan: &mut Option<ProvenRendererDirectionRelocationPlan>, ) -> String",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const SAME_LANE_RAW_SPLICE_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 67,
    owner: "src/render/silent_voice_visible_measure_coverage_render_writer.rs::materialize_silent_voice_visible_measure_coverage_for_pdf_source",
    owner_signature: "fn materialize_silent_voice_visible_measure_coverage_for_pdf_source( input_musicxml: &std::path::Path, write_enabled: bool, final_chord_structural_plan: &crate::structural_write_authority::CanonicalFinalChordStructuralPlan, frozen_serialized_voice_plan: &crate::structural_write_authority::FrozenSerializedVoicePlan, render_pruned_final_chord_omissions: &std::collections::BTreeSet< crate::structural_write_authority::RenderPrunedFinalChordOmissionKey, >, occurrence_continuation: &crate::structural_write_authority::FinalPdfSourceOccurrenceContinuation, primary_beam_voice_partition_plan: Option<&crate::structural_write_authority::PrimaryBeamVoicePartitionBoundPlan>, ) -> FinalRenderSourceClosure",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const MISSING_REST_RENDER_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 69,
    owner: "src/structural_write_authority/render_measure_articulation_authority.rs::apply_bound_missing_rest_batch_authorized",
    owner_signature: "fn apply_bound_missing_rest_batch_authorized( source: &str, edits: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};



const DIRECTION_RELOCATION_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 0, // Named event, deliberately not a new population row.
    owner: "src/structural_write_authority/renderer_direction_relocation_writer_boundary.rs::compose_authorized_direction_delta_onto_sink",
    owner_signature: "fn compose_authorized_direction_delta_onto_sink( sink_xml: &str, instruction: &RendererDirectionRelocationWriterInstruction, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const MIDI_SEED_GENESIS: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 68,
    owner: "src/midi_seed.rs::midi_to_musicxml_seed_with_profile_and_hotspots",
    owner_signature: "fn midi_to_musicxml_seed_with_profile_and_hotspots( path: &Path, title: &str, profile: CandidateProfile, hotspots: Option<&HotspotPlan>, ) -> anyhow::Result<(String, MidiSeedStats)>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Genesis,
};

const ARRANGEMENT_GENESIS: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 62,
    owner: "src/arrangement.rs::arrange_piano_musicxml_with_profile",
    owner_signature: "fn arrange_piano_musicxml_with_profile(xml: &str, profile: ArrangementProfile) -> ArrangementResult",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Genesis,
};

const VISIBLE_RHYTHM_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 52,
    owner: "src/arrangement.rs::normalize_visible_rhythm_musicxml",
    owner_signature: "fn normalize_visible_rhythm_musicxml(xml: &str) -> ArrangementResult",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockDocument,
};

const BEAT_SCOPED_BEAMING_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 53,
    owner: "src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::enforce_beat_scoped_beaming_and_rest_completion_in_xml",
    owner_signature: "fn enforce_beat_scoped_beaming_and_rest_completion_in_xml(xml: &mut String)",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockDocument,
};

const CLEF_MEASURE_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 49,
    owner: "src/structural_write_authority/render_measure_articulation_authority.rs::replace_bound_clef_measure_authorized",
    owner_signature: "fn replace_bound_clef_measure_authorized( source: &str, start: usize, end: usize, replacement: String, ) -> Option<String>",
    effect: EffectContract::Unknown,
    identity: IdentityContract::BlockMeasure,
};

const OCTAVE_MEASURE_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 50,
    owner: "src/structural_write_authority/render_measure_articulation_authority.rs::replace_bound_octave_measure_authorized",
    owner_signature: "fn replace_bound_octave_measure_authorized( source: &str, start: usize, end: usize, replacement: String, ) -> Option<String>",
    effect: EffectContract::Unknown,
    identity: IdentityContract::BlockMeasure,
};


const SAFE_TRANSFER_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 3,
    owner: "src/structural_write_authority/remaining_atomic_patch_authority.rs::apply_bound_safe_transfer_authorized",
    owner_signature: "fn apply_bound_safe_transfer_authorized( source: &str, note_start: usize, note_end: usize, insert_after_original: usize, restified: String, transferred: String, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const STAFF_VOICE_MOVE_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 4,
    owner: "src/structural_write_authority/remaining_atomic_patch_authority.rs::apply_bound_staff_voice_move_authorized",
    owner_signature: "fn apply_bound_staff_voice_move_authorized( source: &str, voice_value_range: Option<(usize, usize)>, staff_value_range: Option<(usize, usize)>, note_end: usize, target_voice: String, target_staff: String, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const RESTIFY_PATCH_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 63,
    owner: "src/pipeline_shards/staff_voice_atomic_move_patch_core.rs::apply_restify_patch",
    owner_signature: "fn apply_restify_patch(xml: &str, patch: &AtomicPatch) -> String",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const VOICE_OVERFULL_CROP_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 25,
    owner: "src/structural_write_authority/bounded_semantic_rewrite_authority.rs::apply_voice_overfull_crop_edits_authorized",
    owner_signature: "fn apply_voice_overfull_crop_edits_authorized( source: &str, edits: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const RESTIFICATION_BATCH_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 39,
    owner: "src/structural_write_authority/staff_voice_atomic_authority.rs::apply_atomic_restification_batch_authorized",
    owner_signature: "fn apply_atomic_restification_batch_authorized( source: &str, edits: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const STAFF_VOICE_SCALAR_BATCH_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 40,
    owner: "src/structural_write_authority/staff_voice_atomic_authority.rs::apply_atomic_staff_voice_scalar_batch_authorized",
    owner_signature: "fn apply_atomic_staff_voice_scalar_batch_authorized( source: &str, edits: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const EDITORIAL_VOICE_COMPLETE_REWRITE_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 1,
    owner: "src/structural_write_authority/bounded_semantic_rewrite_authority.rs::apply_editorial_voice_complete_rewrite_authorized",
    owner_signature: "fn apply_editorial_voice_complete_rewrite_authorized( source: &str, note_ranges: Vec<(usize, usize)>, insert_position: usize, replacement_rest_note: String, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const NOTE_VISIBILITY_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 5,
    owner: "src/structural_write_authority/remaining_layout_writer_family.rs::hide_note_print_object_authorized",
    owner_signature: "fn hide_note_print_object_authorized( xml: &mut String, note_start: usize, note_end: usize, ) -> Result<bool>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const ALLOWED_RANGE_REMOVAL_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 6,
    owner: "src/structural_write_authority/remaining_layout_writer_family.rs::remove_allowed_xml_ranges_authorized",
    owner_signature: "fn remove_allowed_xml_ranges_authorized( xml: &mut String, ranges: &[(usize, usize)], ) -> Result<usize>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const HARMONIC_PITCH_SWAP_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 8,
    owner: "src/structural_write_authority/thirty_surface_productive_authority.rs::apply_harmonic_pitch_swap_authorized",
    owner_signature: "fn apply_harmonic_pitch_swap_authorized( xml: &mut String, pitch_start: usize, pitch_end: usize, replacement_pitch: &str, ) -> bool",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const HIDDEN_ATTRIBUTE_REMOVAL_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 11,
    owner: "src/structural_write_authority/thirty_surface_productive_authority.rs::remove_note_hidden_attrs_in_range_authorized",
    owner_signature: "fn remove_note_hidden_attrs_in_range_authorized( xml: &mut String, start: usize, end: usize, ) -> bool",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};



const CHORD_GESTURE_DISTRIBUTION_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 26,
    owner: "src/structural_write_authority/final_buffer_surface_authority.rs::apply_chord_gesture_distribution_batch_authorized",
    owner_signature: "fn apply_chord_gesture_distribution_batch_authorized( source: &str, edits: Vec<(usize, usize, String)>, ) -> Option<(String, usize)>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const VISIBILITY_STAFF_DISTRIBUTION_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 27,
    owner: "src/structural_write_authority/final_buffer_surface_authority.rs::apply_visibility_staff_distribution_batch_authorized",
    owner_signature: "fn apply_visibility_staff_distribution_batch_authorized( source: &str, edits: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const MASTER_EXPORT_FIFTHS_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 59,
    owner: "src/structural_write_authority/orchestrator_artifact_sink_authority.rs::write_master_musicxml_output_authorized",
    owner_signature: "fn write_master_musicxml_output_authorized( path: &Path, candidate_xml: &str, functional_respelling: FunctionalMasterRespellingOperation, ) -> io::Result<()>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const FUNCTIONAL_RESPELLING_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 60,
    owner: "src/pipeline_shards/key_signature_enharmonic_report_core.rs::canonical_master_functional_respelling_from_render",
    owner_signature: "fn canonical_master_functional_respelling_from_render( master_xml: &str, render_xml: &str, ) -> Result<String, String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

const FINAL_CHORD_ENCODING_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 65,
    owner: "src/structural_write_authority/serialization_plan.rs::project_final_chord_encoding_plan",
    owner_signature: "fn project_final_chord_encoding_plan( candidate_xml: &str, plan: &FinalChordEncodingPlan, ) -> io::Result<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

/// Frozen PRESERVING exclusion contract (r1204 exclusion table). The anchor is
/// exclusion id + owner + owner signature; the classification is PRESERVING.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PreservingDescriptor {
    exclusion_id: &'static str,
    owner: &'static str,
    owner_signature: &'static str,
}

impl PreservingDescriptor {
    fn semantic_anchor(&self) -> String {
        format!("{}|{}|{}|PRESERVING", self.exclusion_id, self.owner, self.owner_signature)
    }
}

const NOTATION_POLISH_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E051",
    owner: "src/notation.rs::polish_musicxml",
    owner_signature: "fn polish_musicxml(xml: &str) -> anyhow::Result<PolishResult>",
};

const NOTATION_CLEF_ISLAND_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E052",
    owner: "src/notation.rs::decompose_lower_staff_clef_islands",
    owner_signature: "fn decompose_lower_staff_clef_islands(xml: &str) -> anyhow::Result<String>",
};

const PIANIST_LAYOUT_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E056",
    owner: "src/layout.rs::apply_pianist_layout",
    owner_signature: "fn apply_pianist_layout(xml: &str, target_pages: usize) -> LayoutResult",
};

const PARTIAL_BEAT_BEAM_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E053",
    owner: "src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::enforce_partial_beat_post_rest_beams_in_xml",
    owner_signature: "fn enforce_partial_beat_post_rest_beams_in_xml(xml: &mut String) -> PartialBeatPostRestBeamPatchStats",
};

const PARTIAL_BEAT_LANE_TRANSFER_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E057",
    owner: "src/pipeline_shards/partial_beat_post_rest_triple_beam_final_repair.rs::human_bound_cursor_lane_owner_transfer_xml",
    owner_signature: "fn human_bound_cursor_lane_owner_transfer_xml( xml: &str, ) -> (String, PartialBeatLaneSerializationOwnerTransferStats)",
};




const COMMITTED_CROSS_STAFF_BEAM_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E049",
    owner: "src/pipeline_shards/composer_beam_candidate_materialization_core.rs::close_committed_cross_staff_beam_gestures",
    owner_signature: "fn close_committed_cross_staff_beam_gestures( xml: &mut String, ) -> ExternalFeedbackBeamClosureStats",
};

const STRUCTURAL_REST_BRIDGE_VARIANT_A_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E050",
    owner: "src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::apply_structural_rest_bridge_beam_variant_a_render_write",
    owner_signature: "fn apply_structural_rest_bridge_beam_variant_a_render_write( xml: &str, source_label: &str, output_path: &std::path::Path, ) -> (String, SisterFeedbackBeamVariantATinyRenderWriteAudit)",
};

const OCTAVE_WORD_LABEL_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E055",
    owner: "src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::remove_generated_octave_word_labels",
    owner_signature: "fn remove_generated_octave_word_labels(xml: &mut String) -> usize",
};

const VISUAL_LAYOUT_DEFAULTS_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E044",
    owner: "src/structural_write_authority/visual_layout_defaults.rs::authorized_visual_layout_defaults_writer",
    owner_signature: "fn authorized_visual_layout_defaults_writer( xml: &mut String, token: AuthorizedVisualLayoutDefaultsInsertion, ) -> Result<usize>",
};

const LOCAL_STAFF_DISTANCE_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E013",
    owner: "src/structural_write_authority/local_staff_distance_transaction.rs::authorized_local_staff_distance_writer",
    owner_signature: "fn authorized_local_staff_distance_writer( xml: &mut String, token: AuthorizedLocalStaffDistanceTransaction, ) -> Result<usize>",
};

const RUSTIG_CLEF_COURSE_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E024",
    owner: "src/structural_write_authority/rustig_clef_planning.rs::apply_rustig_clef_course_to_buffer",
    owner_signature: "fn apply_rustig_clef_course_to_buffer( xml: &mut String, ) -> Result<(), &'static str>",
};


const LEDGER_BOUNDED_LOWER_STAFF_CLEF_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E054",
    owner: "src/notation.rs::ledger_bounded_lower_staff_clef_readback",
    owner_signature: "fn ledger_bounded_lower_staff_clef_readback( xml: &str, scope: LowerStaffClefReadbackScope, ) -> String",
};

const FINAL_LAYOUT_PHRASE_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E007",
    owner: "src/structural_write_authority/final_notation_phrase_projection_transaction.rs::materialize_final_render_notation_with_phrase_projection_authorized",
    owner_signature: "fn materialize_final_render_notation_with_phrase_projection_authorized( path: &Path, directives: &[HorizontalLayoutMaterializationDirective], phrase_notation_intents: &[PhraseNotationIntentProjection], ) -> Result<usize>",
};

/// Frozen constant name of a PRESERVING descriptor, the key of its contract v4
/// envelope row. Pure lookup over the frozen descriptors; no runtime input.
pub(crate) fn ssc_preserving_descriptor_name(descriptor: &PreservingDescriptor) -> Option<&'static str> {
    [
        (&NOTATION_POLISH_PRESERVING, "NOTATION_POLISH_PRESERVING"),
        (&NOTATION_CLEF_ISLAND_PRESERVING, "NOTATION_CLEF_ISLAND_PRESERVING"),
        (&PIANIST_LAYOUT_PRESERVING, "PIANIST_LAYOUT_PRESERVING"),
        (&PARTIAL_BEAT_BEAM_PRESERVING, "PARTIAL_BEAT_BEAM_PRESERVING"),
        (&PARTIAL_BEAT_LANE_TRANSFER_PRESERVING, "PARTIAL_BEAT_LANE_TRANSFER_PRESERVING"),
        (&COMMITTED_CROSS_STAFF_BEAM_PRESERVING, "COMMITTED_CROSS_STAFF_BEAM_PRESERVING"),
        (&STRUCTURAL_REST_BRIDGE_VARIANT_A_PRESERVING, "STRUCTURAL_REST_BRIDGE_VARIANT_A_PRESERVING"),
        (&OCTAVE_WORD_LABEL_PRESERVING, "OCTAVE_WORD_LABEL_PRESERVING"),
        (&VISUAL_LAYOUT_DEFAULTS_PRESERVING, "VISUAL_LAYOUT_DEFAULTS_PRESERVING"),
        (&LOCAL_STAFF_DISTANCE_PRESERVING, "LOCAL_STAFF_DISTANCE_PRESERVING"),
        (&RUSTIG_CLEF_COURSE_PRESERVING, "RUSTIG_CLEF_COURSE_PRESERVING"),
        (&LEDGER_BOUNDED_LOWER_STAFF_CLEF_PRESERVING, "LEDGER_BOUNDED_LOWER_STAFF_CLEF_PRESERVING"),
        (&FINAL_LAYOUT_PHRASE_PRESERVING, "FINAL_LAYOUT_PHRASE_PRESERVING"),
    ]
    .into_iter()
    .find(|(frozen, _)| *frozen == descriptor)
    .map(|(_, name)| name)
}

/// Typed insertion class of a frozen ADDITIVE exclusion. It is derived from the
/// frozen exclusion reason by the descriptor test, never parsed at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AdditiveInsertionClass {
    /// `final barline only`: exactly one `<barline>` element.
    FinalBarline,
    /// `direction insertion only`: exactly one `<direction>` element.
    Direction,

    
    TieFragment,
}

impl AdditiveInsertionClass {
    fn element_name(self) -> &'static str {
        match self {
            Self::FinalBarline => "barline",
            Self::Direction => "direction",
            Self::TieFragment => "__owner_local_fragment_only",
        }
    }
}

/// Frozen ADDITIVE exclusion contract: exclusion id + owner + owner signature
/// plus the typed insertion class bound to the frozen reason.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AdditiveDescriptor {
    exclusion_id: &'static str,
    owner: &'static str,
    owner_signature: &'static str,
    insertion_class: AdditiveInsertionClass,
}

const FINAL_BARLINE_ADDITIVE: AdditiveDescriptor = AdditiveDescriptor {
    exclusion_id: "E004",
    owner: "src/structural_write_authority/final_barline.rs::authorized_final_barline_writer",
    owner_signature: "fn authorized_final_barline_writer( xml: &mut String, token: AuthorizedFinalBarlineInsertion, ) -> Result<usize>",
    insertion_class: AdditiveInsertionClass::FinalBarline,
};

const DIRECTION_INSERTION_ADDITIVE: AdditiveDescriptor = AdditiveDescriptor {
    exclusion_id: "E036",
    owner: "src/structural_write_authority/thirty_surface_productive_authority.rs::insert_direction_into_measure_authorized",
    owner_signature: "fn insert_direction_into_measure_authorized( xml: &mut String, locator: &StructuralMeasureLocator, direction: &str, ) -> usize",
    insertion_class: AdditiveInsertionClass::Direction,
};

// ---------------------------------------------------------------------------

const RENDER_ROW_7_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 7,
    owner: "src/structural_write_authority/retention_chord_authority.rs::ensure_bound_chord_marker_authorized",
    owner_signature: "fn ensure_bound_chord_marker_authorized(source: &str) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockNote,
};
const RENDER_ROW_17_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 17,
    owner: "src/structural_write_authority/thirty_surface_productive_authority.rs::set_rest_note_duration_type_authorized",
    owner_signature: "fn set_rest_note_duration_type_authorized( source: &str, duration: i32, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockNote,
};
const RENDER_ROW_19_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 19,
    owner: "src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::beat_scoped_mark_note_hidden",
    owner_signature: "fn beat_scoped_mark_note_hidden(note_text: &str) -> String",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockNote,
};
const RENDER_ROW_20_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 20,
    owner: "src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs::hide_note_fragment_print_object_no",
    owner_signature: "fn hide_note_fragment_print_object_no(fragment: &str) -> String",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockNote,
};
const RENDER_ROW_21_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 21,
    owner: "src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs::collapse_hidden_timeline_rests_to_forward_in_measure",
    owner_signature: "fn collapse_hidden_timeline_rests_to_forward_in_measure(measure: &str) -> (String, RenderXmlHiddenRestVirtualStats)",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockMeasure,
};
const RENDER_ROW_22_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 22,
    owner: "src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs::prune_safe_hidden_inner_chord_tones_in_measure",
    owner_signature: "fn prune_safe_hidden_inner_chord_tones_in_measure(measure: &str) -> (String, RenderXmlPruneStats)",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_30_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 30,
    owner: "src/structural_write_authority/rest_voice_family_authority.rs::apply_final_rest_glyph_height_authorized",
    owner_signature: "fn apply_final_rest_glyph_height_authorized( source: &str, replacements: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_31_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 31,
    owner: "src/structural_write_authority/rest_voice_family_authority.rs::apply_final_visible_rest_role_rewrite_authorized",
    owner_signature: "fn apply_final_visible_rest_role_rewrite_authorized( source: &str, replacements: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_32_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 32,
    owner: "src/structural_write_authority/rest_voice_family_authority.rs::apply_layer_balanced_rest_consolidation_authorized",
    owner_signature: "fn apply_layer_balanced_rest_consolidation_authorized( source: &str, replacements: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_33_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 33,
    owner: "src/structural_write_authority/rest_voice_family_authority.rs::apply_lower_staff_rest_chain_compaction_authorized",
    owner_signature: "fn apply_lower_staff_rest_chain_compaction_authorized( source: &str, replacements: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_34_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 34,
    owner: "src/structural_write_authority/rest_voice_family_authority.rs::apply_pianistic_rest_consolidation_authorized",
    owner_signature: "fn apply_pianistic_rest_consolidation_authorized( source: &str, replacements: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_35_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 35,
    owner: "src/structural_write_authority/rest_voice_family_authority.rs::apply_rest_only_staff_readability_authorized",
    owner_signature: "fn apply_rest_only_staff_readability_authorized( source: &str, replacements: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_36_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 36,
    owner: "src/structural_write_authority/rest_voice_family_authority.rs::apply_secondary_reanchored_rest_visibility_authorized",
    owner_signature: "fn apply_secondary_reanchored_rest_visibility_authorized( source: &str, replacements: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_37_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 37,
    owner: "src/structural_write_authority/rest_voice_family_authority.rs::apply_single_rest_consolidation_authorized",
    owner_signature: "fn apply_single_rest_consolidation_authorized( source: &str, replacements: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_38_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 38,
    owner: "src/structural_write_authority/rest_voice_family_authority.rs::apply_strict_staff_gap_materialization_authorized",
    owner_signature: "fn apply_strict_staff_gap_materialization_authorized( source: &str, replacements: Vec<(usize, usize, String)>, ) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_41_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 41,
    owner: "src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_backup_anchor_rewrite_authorized",
    owner_signature: "macro:fn apply_backup_anchor_rewrite_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_42_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 42,
    owner: "src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_duplicate_same_pitch_suppression_authorized",
    owner_signature: "macro:fn apply_duplicate_same_pitch_suppression_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_43_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 43,
    owner: "src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_empty_voice_rest_suppression_authorized",
    owner_signature: "macro:fn apply_empty_voice_rest_suppression_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_44_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 44,
    owner: "src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_hidden_rest_visibility_rewrite_authorized",
    owner_signature: "macro:fn apply_hidden_rest_visibility_rewrite_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_45_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 45,
    owner: "src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_lower_visible_rest_suppression_authorized",
    owner_signature: "macro:fn apply_lower_visible_rest_suppression_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_46_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 46,
    owner: "src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_rest_beat_materialization_authorized",
    owner_signature: "macro:fn apply_rest_beat_materialization_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_47_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 47,
    owner: "src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_upper_duplicate_rest_suppression_authorized",
    owner_signature: "macro:fn apply_upper_duplicate_rest_suppression_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_48_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 48,
    owner: "src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_upper_initial_rest_coalescing_authorized",
    owner_signature: "macro:fn apply_upper_initial_rest_coalescing_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> Option<String>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_51_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 51,
    owner: "src/structural_write_authority/strict_staff_visible_lane_authority.rs::insert_missing_staff_edge_rest_authorized",
    owner_signature: "fn insert_missing_staff_edge_rest_authorized( source: &str, insertions: Vec<(usize, String)>, ) -> Option<String>",
    effect: EffectContract::Unknown,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_54_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 54,
    owner: "src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::close_beat_scoped_rest_completion_crossing_rests_in_xml",
    owner_signature: "fn close_beat_scoped_rest_completion_crossing_rests_in_xml( xml: &str, ) -> (String, usize)",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockDocument,
};
const RENDER_ROW_55_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 55,
    owner: "src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs::collapse_hidden_timeline_rests_to_forward",
    owner_signature: "fn collapse_hidden_timeline_rests_to_forward(xml: &str) -> (String, RenderXmlHiddenRestVirtualStats)",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockDocument,
};
const RENDER_ROW_57_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 57,
    owner: "src/pipeline_shards/human_pdf_pianistic_cross_staff_ledger_tail.rs::build_render_xml_print_cleanup_one_safe_visual_trial",
    owner_signature: "fn build_render_xml_print_cleanup_one_safe_visual_trial( xml: &str, source_label: &str, output_path: &Path, ) -> anyhow::Result<RenderXmlPrintCleanupOneSafeVisualTrial>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockDocument,
};
const RENDER_ROW_58_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 58,
    owner: "src/structural_write_authority/orchestrator_artifact_sink_authority.rs::write_render_checkpoint",
    owner_signature: "fn write_render_checkpoint( path: &Path, candidate_xml: &str, stage: RenderCheckpointStage, ) -> io::Result<()>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_61_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 61,
    owner: "src/structural_write_authority/clef_surface_materialization.rs::write_clef_surface_materialization_authorized",
    owner_signature: "fn write_clef_surface_materialization_authorized( output_path: &Path, candidate: VerifiedClefSurfaceMaterialization, ) -> io::Result<()>",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockMeasure,
};
const RENDER_ROW_64_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 64,
    owner: "src/pipeline_shards/retention_composer_limit_core.rs::apply_retention_first_p2_assertive_return_write_explicit_gate",
    owner_signature: "fn apply_retention_first_p2_assertive_return_write_explicit_gate( master_xml: &str, render_xml: &str, ) -> (String, RetentionFirstP2AssertiveReturnWriteAudit)",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};
const RENDER_ROW_66_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 66,
    owner: "src/pipeline_shards/key_signature_enharmonic_report_core.rs::apply_key_signature_enharmonic_spelling_render_write",
    owner_signature: "fn apply_key_signature_enharmonic_spelling_render_write( xml: &str, source_label: &str, output_path: &Path, ) -> (String, KeySignatureEnharmonicSpellingRenderWriteAudit)",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockNote,
};
// Sealed owner capabilities
// ---------------------------------------------------------------------------

mod sealed {
    pub trait MintOwner {}
}

/// Owner capability for minting a GENESIS identity. Only this module can bind
/// an owner type to a descriptor; only the owner module can construct the type.
pub(crate) trait GenesisMintOwner: sealed::MintOwner {
    const DESCRIPTOR: &'static TransformationDescriptor;
}

/// Owner capability for minting a BLOCK receipt.
pub(crate) trait BlockMintOwner: sealed::MintOwner {
    const DESCRIPTOR: &'static TransformationDescriptor;
}

/// Owner capability for minting an EXACT receipt.
pub(crate) trait ExactMintOwner: sealed::MintOwner {
    const DESCRIPTOR: &'static TransformationDescriptor;
}

/// Owner capability for minting a PRESERVING receipt of an exclusion owner.
pub(crate) trait PreservingMintOwner: sealed::MintOwner {
    const DESCRIPTOR: &'static PreservingDescriptor;

    fn conserved_projection(xml: &str) -> Option<String> {
        conserved_content_projection(xml)
    }
}

/// Owner capability for minting an ADDITIVE receipt of an exclusion owner.
pub(crate) trait AdditiveMintOwner: sealed::MintOwner {
    const DESCRIPTOR: &'static AdditiveDescriptor;
}

macro_rules! bind_mint_owner {
    ($trait:ident, $owner:path, $descriptor_type:ty, $descriptor:expr) => {
        impl sealed::MintOwner for $owner {}
        impl $trait for $owner {
            const DESCRIPTOR: &'static $descriptor_type = &$descriptor;
        }
    };
}

bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::CursorNeutralRenderExactMint, TransformationDescriptor, CURSOR_NEUTRAL_RENDER_EXACT);
bind_mint_owner!(ExactMintOwner, crate::pipeline::SameLaneRawSpliceExactMint, TransformationDescriptor, SAME_LANE_RAW_SPLICE_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::MissingRestRenderExactMint, TransformationDescriptor, MISSING_REST_RENDER_EXACT);

bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::DirectionRelocationExactMint, TransformationDescriptor, DIRECTION_RELOCATION_EXACT);
bind_mint_owner!(GenesisMintOwner, crate::midi_seed::MidiSeedGenesisMint, TransformationDescriptor, MIDI_SEED_GENESIS);
bind_mint_owner!(GenesisMintOwner, crate::arrangement::ArrangementGenesisMint, TransformationDescriptor, ARRANGEMENT_GENESIS);
bind_mint_owner!(BlockMintOwner, crate::arrangement::VisibleRhythmBlockMint, TransformationDescriptor, VISIBLE_RHYTHM_BLOCK);
bind_mint_owner!(BlockMintOwner, crate::pipeline::BeatScopedBeamingBlockMint, TransformationDescriptor, BEAT_SCOPED_BEAMING_BLOCK);
bind_mint_owner!(BlockMintOwner, crate::structural_write_authority::ClefMeasureBlockMint, TransformationDescriptor, CLEF_MEASURE_BLOCK);
bind_mint_owner!(BlockMintOwner, crate::structural_write_authority::OctaveMeasureBlockMint, TransformationDescriptor, OCTAVE_MEASURE_BLOCK);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::SafeTransferExactMint, TransformationDescriptor, SAFE_TRANSFER_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::StaffVoiceMoveExactMint, TransformationDescriptor, STAFF_VOICE_MOVE_EXACT);
bind_mint_owner!(ExactMintOwner, crate::pipeline::RestifyPatchExactMint, TransformationDescriptor, RESTIFY_PATCH_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::VoiceOverfullCropExactMint, TransformationDescriptor, VOICE_OVERFULL_CROP_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RestificationBatchExactMint, TransformationDescriptor, RESTIFICATION_BATCH_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::StaffVoiceScalarBatchExactMint, TransformationDescriptor, STAFF_VOICE_SCALAR_BATCH_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::EditorialVoiceCompleteRewriteExactMint, TransformationDescriptor, EDITORIAL_VOICE_COMPLETE_REWRITE_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::NoteVisibilityExactMint, TransformationDescriptor, NOTE_VISIBILITY_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::AllowedRangeRemovalExactMint, TransformationDescriptor, ALLOWED_RANGE_REMOVAL_EXACT);
#[cfg(test)]
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::HarmonicPitchSwapExactMint, TransformationDescriptor, HARMONIC_PITCH_SWAP_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::HiddenAttributeRemovalExactMint, TransformationDescriptor, HIDDEN_ATTRIBUTE_REMOVAL_EXACT);

bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::ChordGestureDistributionExactMint, TransformationDescriptor, CHORD_GESTURE_DISTRIBUTION_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::VisibilityStaffDistributionExactMint, TransformationDescriptor, VISIBILITY_STAFF_DISTRIBUTION_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::MasterExportFifthsExactMint, TransformationDescriptor, MASTER_EXPORT_FIFTHS_EXACT);
bind_mint_owner!(ExactMintOwner, crate::pipeline::FunctionalRespellingExactMint, TransformationDescriptor, FUNCTIONAL_RESPELLING_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::FinalChordEncodingExactMint, TransformationDescriptor, FINAL_CHORD_ENCODING_EXACT);
bind_mint_owner!(PreservingMintOwner, crate::pipeline::LedgerBoundedLowerStaffClefReadbackPreservingMint, PreservingDescriptor, LEDGER_BOUNDED_LOWER_STAFF_CLEF_PRESERVING);
bind_mint_owner!(PreservingMintOwner, crate::pipeline::NotationPolishPreservingMint, PreservingDescriptor, NOTATION_POLISH_PRESERVING);
bind_mint_owner!(PreservingMintOwner, crate::pipeline::NotationClefIslandPreservingMint, PreservingDescriptor, NOTATION_CLEF_ISLAND_PRESERVING);
bind_mint_owner!(PreservingMintOwner, crate::layout::PianistLayoutPreservingMint, PreservingDescriptor, PIANIST_LAYOUT_PRESERVING);
bind_mint_owner!(PreservingMintOwner, crate::pipeline::PartialBeatBeamPreservingMint, PreservingDescriptor, PARTIAL_BEAT_BEAM_PRESERVING);
impl sealed::MintOwner for crate::pipeline::PartialBeatLaneTransferPreservingMint {}
impl PreservingMintOwner for crate::pipeline::PartialBeatLaneTransferPreservingMint {
    const DESCRIPTOR: &'static PreservingDescriptor = &PARTIAL_BEAT_LANE_TRANSFER_PRESERVING;

    fn conserved_projection(xml: &str) -> Option<String> {
        crate::pipeline::e057_existing_event_multiset_projection(xml)
    }
}

bind_mint_owner!(PreservingMintOwner, crate::pipeline::CommittedCrossStaffBeamPreservingMint, PreservingDescriptor, COMMITTED_CROSS_STAFF_BEAM_PRESERVING);
bind_mint_owner!(PreservingMintOwner, crate::pipeline::StructuralRestBridgeVariantAPreservingMint, PreservingDescriptor, STRUCTURAL_REST_BRIDGE_VARIANT_A_PRESERVING);
bind_mint_owner!(PreservingMintOwner, crate::pipeline::OctaveWordLabelPreservingMint, PreservingDescriptor, OCTAVE_WORD_LABEL_PRESERVING);
bind_mint_owner!(PreservingMintOwner, crate::structural_write_authority::VisualLayoutDefaultsPreservingMint, PreservingDescriptor, VISUAL_LAYOUT_DEFAULTS_PRESERVING);
bind_mint_owner!(PreservingMintOwner, crate::structural_write_authority::LocalStaffDistancePreservingMint, PreservingDescriptor, LOCAL_STAFF_DISTANCE_PRESERVING);
bind_mint_owner!(PreservingMintOwner, crate::structural_write_authority::RustigClefCoursePreservingMint, PreservingDescriptor, RUSTIG_CLEF_COURSE_PRESERVING);
bind_mint_owner!(PreservingMintOwner, crate::structural_write_authority::FinalLayoutPhrasePreservingMint, PreservingDescriptor, FINAL_LAYOUT_PHRASE_PRESERVING);
bind_mint_owner!(AdditiveMintOwner, crate::structural_write_authority::FinalBarlineAdditiveMint, AdditiveDescriptor, FINAL_BARLINE_ADDITIVE);
bind_mint_owner!(AdditiveMintOwner, crate::structural_write_authority::DirectionInsertionAdditiveMint, AdditiveDescriptor, DIRECTION_INSERTION_ADDITIVE);


bind_mint_owner!(BlockMintOwner, crate::structural_write_authority::RenderRow7BlockMint, TransformationDescriptor, RENDER_ROW_7_BLOCK);
bind_mint_owner!(BlockMintOwner, crate::structural_write_authority::RenderRow17BlockMint, TransformationDescriptor, RENDER_ROW_17_BLOCK);
bind_mint_owner!(BlockMintOwner, crate::pipeline::RenderRow19BlockMint, TransformationDescriptor, RENDER_ROW_19_BLOCK);
bind_mint_owner!(BlockMintOwner, crate::pipeline::RenderRow20BlockMint, TransformationDescriptor, RENDER_ROW_20_BLOCK);
bind_mint_owner!(BlockMintOwner, crate::pipeline::RenderRow21BlockMint, TransformationDescriptor, RENDER_ROW_21_BLOCK);
bind_mint_owner!(ExactMintOwner, crate::pipeline::RenderRow22ExactMint, TransformationDescriptor, RENDER_ROW_22_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RenderRow30ExactMint, TransformationDescriptor, RENDER_ROW_30_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RenderRow31ExactMint, TransformationDescriptor, RENDER_ROW_31_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RenderRow32ExactMint, TransformationDescriptor, RENDER_ROW_32_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RenderRow33ExactMint, TransformationDescriptor, RENDER_ROW_33_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RenderRow34ExactMint, TransformationDescriptor, RENDER_ROW_34_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RenderRow35ExactMint, TransformationDescriptor, RENDER_ROW_35_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RenderRow36ExactMint, TransformationDescriptor, RENDER_ROW_36_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RenderRow37ExactMint, TransformationDescriptor, RENDER_ROW_37_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RenderRow38ExactMint, TransformationDescriptor, RENDER_ROW_38_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::VisibleLane41RenderExactMint, TransformationDescriptor, RENDER_ROW_41_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::VisibleLane42RenderExactMint, TransformationDescriptor, RENDER_ROW_42_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::VisibleLane43RenderExactMint, TransformationDescriptor, RENDER_ROW_43_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::VisibleLane44RenderExactMint, TransformationDescriptor, RENDER_ROW_44_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::VisibleLane45RenderExactMint, TransformationDescriptor, RENDER_ROW_45_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::VisibleLane46RenderExactMint, TransformationDescriptor, RENDER_ROW_46_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::VisibleLane47RenderExactMint, TransformationDescriptor, RENDER_ROW_47_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::VisibleLane48RenderExactMint, TransformationDescriptor, RENDER_ROW_48_EXACT);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RenderRow51ExactMint, TransformationDescriptor, RENDER_ROW_51_EXACT);
bind_mint_owner!(BlockMintOwner, crate::pipeline::RenderRow54BlockMint, TransformationDescriptor, RENDER_ROW_54_BLOCK);
bind_mint_owner!(BlockMintOwner, crate::pipeline::RenderRow55BlockMint, TransformationDescriptor, RENDER_ROW_55_BLOCK);
bind_mint_owner!(BlockMintOwner, crate::pipeline::RenderRow57BlockMint, TransformationDescriptor, RENDER_ROW_57_BLOCK);
bind_mint_owner!(ExactMintOwner, crate::structural_write_authority::RenderRow58ExactMint, TransformationDescriptor, RENDER_ROW_58_EXACT);
bind_mint_owner!(BlockMintOwner, crate::structural_write_authority::RenderRow61BlockMint, TransformationDescriptor, RENDER_ROW_61_BLOCK);
bind_mint_owner!(ExactMintOwner, crate::pipeline::RenderRow64ExactMint, TransformationDescriptor, RENDER_ROW_64_EXACT);
bind_mint_owner!(BlockMintOwner, crate::pipeline::RenderRow66BlockMint, TransformationDescriptor, RENDER_ROW_66_BLOCK);

// ---------------------------------------------------------------------------
// Process-local measurement (projection only, never authority)
// ---------------------------------------------------------------------------

static GENESIS_MINTED: AtomicUsize = AtomicUsize::new(0);
static BLOCK_RECEIPTS_MINTED: AtomicUsize = AtomicUsize::new(0);
static EXACT_RECEIPTS_MINTED: AtomicUsize = AtomicUsize::new(0);
static PRESERVING_RECEIPTS_MINTED: AtomicUsize = AtomicUsize::new(0);
static ADDITIVE_RECEIPTS_MINTED: AtomicUsize = AtomicUsize::new(0);
static OWNER_RECEIPT_REFUSALS: AtomicUsize = AtomicUsize::new(0);
static LINEAGES_STARTED: AtomicUsize = AtomicUsize::new(0);
static RECEIPTS_COMMITTED: AtomicUsize = AtomicUsize::new(0);
static MASTER_ADOPTIONS: AtomicUsize = AtomicUsize::new(0);
static FINAL_MASTER_LINEAGE: Mutex<Option<FinalMasterLineageSummary>> = Mutex::new(None);
static MASTER_ARTIFACT_WITNESS: Mutex<Option<MasterArtifactWitness>> = Mutex::new(None);

fn count(counter: &AtomicUsize) {
    counter.fetch_add(1, Ordering::Relaxed);
}

/// A changed transition whose owner verifier refused the receipt. On a Master
/// adoption path this becomes a fatal coverage violation at the adoption
/// boundary; owners that also run on non-Master documents only count it here.
fn count_owner_receipt_refusal() {
    count(&OWNER_RECEIPT_REFUSALS);
}

// ---------------------------------------------------------------------------
// GENESIS
// ---------------------------------------------------------------------------

/// Binding of the input a GENESIS creator consumed. Binding only, never identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GenesisInputBinding {
    ProductiveSourcePlanSha256(String),
    InputDocumentFingerprint(u64),
}

impl GenesisInputBinding {
    pub(crate) fn input_document(xml: &str) -> Self {
        Self::InputDocumentFingerprint(fingerprint(xml))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MasterGenesisIdentity {
    identity: MintedIdentity,
    descriptor: &'static TransformationDescriptor,
    parameter_digest: u64,
    input: GenesisInputBinding,
    post_fingerprint: u64,
}

impl MasterGenesisIdentity {
    /// Mints a new GENESIS at the creation event of its owner.
    pub(crate) fn mint_genesis<O: GenesisMintOwner>(
        _owner: O,
        parameter_contract: &str,
        input: GenesisInputBinding,
        post_xml: &str,
    ) -> Self {
        count(&GENESIS_MINTED);
        Self {
            identity: MintedIdentity::mint_fresh(),
            descriptor: O::DESCRIPTOR,
            parameter_digest: fingerprint(parameter_contract),
            input,
            post_fingerprint: fingerprint(post_xml),
        }
    }

    /// Validates bindings of an existing GENESIS. The identity is never recomputed.
    pub(crate) fn validate_genesis_binding<O: GenesisMintOwner>(
        &self,
        parameter_contract: &str,
        input: &GenesisInputBinding,
        post_xml: &str,
    ) -> anyhow::Result<()> {
        ensure!(
            self.descriptor.same_contract(O::DESCRIPTOR),
            "MasterGenesisIdentity: owner contract drift"
        );
        ensure!(
            self.parameter_digest == fingerprint(parameter_contract),
            "MasterGenesisIdentity: parameter contract drift"
        );
        ensure!(&self.input == input, "MasterGenesisIdentity: input binding drift");
        ensure!(
            self.post_fingerprint == fingerprint(post_xml),
            "MasterGenesisIdentity: post fingerprint drift"
        );
        Ok(())
    }

    /// Validates the owner contract and the bound output only.
    pub(crate) fn validate_genesis_output<O: GenesisMintOwner>(
        &self,
        post_xml: &str,
    ) -> anyhow::Result<()> {
        ensure!(
            self.descriptor.same_contract(O::DESCRIPTOR),
            "MasterGenesisIdentity: owner contract drift"
        );
        ensure!(
            self.post_fingerprint == fingerprint(post_xml),
            "MasterGenesisIdentity: post fingerprint drift"
        );
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// BLOCK
// ---------------------------------------------------------------------------

/// One execution of a BLOCK transformation: the frozen owner is responsible for
/// exactly this pre->post transition; no atomic edit claim is made. Move-only.
#[derive(Debug)]
pub(crate) struct BlockTransformationReceipt {
    ssc_origin: Box<crate::ssc_evidence_proof::SscOriginState>,
    identity: MintedIdentity,
    descriptor: &'static TransformationDescriptor,
    bound_span: Option<(usize, usize)>,
    pre_fingerprint: u64,
    post_fingerprint: u64,
    /// Post bytes realized inside a path-bound owner's own write transaction,
    /// carried move-only to the Render adoption boundary. `None` for buffer
    /// owners, whose post state is supplied by the adopting transition.
    realized_post_document: Option<String>,
}

fn ssc_exact_scope(descriptor: &TransformationDescriptor) -> crate::ssc_evidence_proof::SscOriginScope {
    use crate::ssc_evidence_proof::SscOriginScope;
    if descriptor.same_contract(&STAFF_VOICE_MOVE_EXACT) { SscOriginScope::F5 }
    else if descriptor.same_contract(&CURSOR_NEUTRAL_RENDER_EXACT) { SscOriginScope::F9 }
    else if descriptor.same_contract(&RENDER_ROW_44_EXACT) { SscOriginScope::F7 }
    else { SscOriginScope::Unproved }
}

impl BlockTransformationReceipt {
    pub(crate) fn ssc_bound_origin(&self) -> (&MintedIdentity, &crate::ssc_evidence_proof::SscOriginState) { (&self.identity, &self.ssc_origin) }
    /// Mints a BLOCK receipt at the owner's execution. An unchanged transition is
    /// no event and yields `None`. `bound_span` is the pre-state byte range the
    /// owner was bound to, when the owner has one.
    pub(crate) fn mint_block_receipt<O: BlockMintOwner>(
        _owner: O,
        pre_xml: &str,
        post_xml: &str,
        bound_span: Option<(usize, usize)>,
    ) -> Option<Self> {
        if pre_xml == post_xml {
            return None;
        }
        if !O::DESCRIPTOR.identity.is_block() {
            count_owner_receipt_refusal();
            return None;
        }
        count(&BLOCK_RECEIPTS_MINTED);
        Some(Self {
            ssc_origin: Box::new(crate::ssc_evidence_proof::SscOriginState::capture(&SSC_ORIGIN_ACCESS, pre_xml, post_xml, crate::ssc_evidence_proof::SscOriginScope::Unproved)),
            identity: MintedIdentity::mint_fresh(),
            descriptor: O::DESCRIPTOR,
            bound_span,
            pre_fingerprint: fingerprint(pre_xml),
            post_fingerprint: fingerprint(post_xml),
            realized_post_document: None,
        })
    }

    /// BLOCK mint of a path-bound owner that persists its own output: the
    /// receipt carries the post bytes the owner wrote, bound to its post
    /// fingerprint, so the adoption needs no artifact reimport.
    pub(crate) fn mint_file_bound_block_receipt<O: BlockMintOwner>(
        owner: O,
        pre_xml: &str,
        post_xml: String,
        bound_span: Option<(usize, usize)>,
    ) -> Option<Self> {
        let mut receipt = Self::mint_block_receipt(owner, pre_xml, &post_xml, bound_span)?;
        receipt.realized_post_document = Some(post_xml);
        Some(receipt)
    }

    /// Splits the realized post bytes of a path-bound owner from its receipt.
    pub(crate) fn split_realized_post_document(mut self) -> (Self, Option<String>) {
        let document = self
            .realized_post_document
            .take()
            .filter(|document| fingerprint(document) == self.post_fingerprint);
        (self, document)
    }

    /// Validates owner contract and pre/post binding. Never recomputes identity.
    pub(crate) fn validate_block_transition<O: BlockMintOwner>(
        &self,
        pre_xml: &str,
        post_xml: &str,
    ) -> anyhow::Result<()> {
        ensure!(
            self.descriptor.same_contract(O::DESCRIPTOR),
            "BlockTransformationReceipt: owner contract drift"
        );
        ensure!(
            self.descriptor.identity.is_block(),
            "BlockTransformationReceipt: descriptor is not a BLOCK contract"
        );
        ensure!(
            self.pre_fingerprint == fingerprint(pre_xml),
            "BlockTransformationReceipt: pre fingerprint drift"
        );
        ensure!(
            self.post_fingerprint == fingerprint(post_xml),
            "BlockTransformationReceipt: post fingerprint drift"
        );
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// EXACT
// ---------------------------------------------------------------------------

/// One edit bound at the owner against the unmodified pre-state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExactEdit {
    start: usize,
    end: usize,
    replacement: String,
}

impl ExactEdit {
    pub(crate) fn bound(start: usize, end: usize, replacement: String) -> Self {
        Self {
            start,
            end,
            replacement,
        }
    }
}

/// Neutral emitter geometry. Original binding and receipt execution, never
/// output values, supply provenance. This payload has no writing capability.
#[derive(Debug, Clone, Hash)]
pub(crate) struct FinalChordCourseRow {
    pub(crate) input: std::ops::Range<usize>,
    pub(crate) outputs: Vec<(std::ops::Range<usize>, bool)>,
    pub(crate) ordinals: Vec<usize>,
    pub(crate) source: Option<usize>,
    pub(crate) tied_chain: bool,
    pub(crate) auxiliary: bool,
    pub(crate) original_offset: i64,
}
impl FinalChordCourseRow {
    pub(crate) fn input(&self) -> &std::ops::Range<usize> { &self.input }
    pub(crate) fn outputs(&self) -> &[(std::ops::Range<usize>, bool)] { &self.outputs }
    pub(crate) fn emission_ordinals(&self) -> &[usize] { &self.ordinals }
    pub(crate) fn source(&self) -> Option<usize> { self.source }
    pub(crate) fn tied_chain(&self) -> bool { self.tied_chain }
    pub(crate) fn auxiliary(&self) -> bool { self.auxiliary }
    pub(crate) fn original_offset(&self) -> i64 { self.original_offset }
}
#[derive(Debug, Clone)]
pub(crate) struct FinalChordCourseCapture {
    execution: MintedIdentity,
    before: String,
    after: String,
    rows: Vec<FinalChordCourseRow>,
    row_seal: u64,
}

/// One execution of an EXACT transformation with its byte-verified edit script.
/// Move-only.
#[derive(Debug)]
pub(crate) struct ExactTransformationReceipt {
    ssc_origin: Box<crate::ssc_evidence_proof::SscOriginState>,
    identity: MintedIdentity,
    descriptor: &'static TransformationDescriptor,
    edits: Vec<ExactEdit>,
    pre_fingerprint: u64,
    post_fingerprint: u64,
    physical_births: Option<crate::arrangement::physical_births::PhysicalRhythmicBirths>,
    staff_transition: Option<crate::arrangement::physical_births::StaffPresentationTransition>,
    final_chord_capture: Option<Result<Box<FinalChordCourseCapture>, String>>,
}

impl ExactTransformationReceipt {
    pub(crate) fn ssc_bound_origin(&self) -> (&MintedIdentity, &crate::ssc_evidence_proof::SscOriginState) { (&self.identity, &self.ssc_origin) }
    /// Mints an EXACT receipt only if the bound edits, applied to `pre_xml`
    /// in pre-state coordinates, reproduce `post_xml` byte-exactly and no byte
    /// outside the bound ranges changed. An unchanged transition is no event
    /// (`None`). A changed transition the edits do not reproduce is an owner
    /// refusal (`None`, counted); a Master adoption of it is fatal.
    pub(crate) fn mint_exact_receipt<O: ExactMintOwner>(
        _owner: O,
        pre_xml: &str,
        post_xml: &str,
        mut edits: Vec<ExactEdit>,
    ) -> Option<Self> {
        if pre_xml == post_xml {
            return None;
        }
        edits.sort_by_key(|edit| (edit.start, edit.end));
        if O::DESCRIPTOR.identity != IdentityContract::Exact
            || !exact_edits_reproduce(pre_xml, post_xml, &edits)
        {
            count_owner_receipt_refusal();
            // Diagnostic only: the owner still refuses this changed transition.
            // No receipt, origin, permission or fallback is created by this output.
            let reason = if O::DESCRIPTOR.identity != IdentityContract::Exact {
                "descriptor_not_exact"
            } else {
                "edit_script_does_not_reproduce"
            };
            let spans = edits
                .iter()
                .take(8)
                .map(|edit| (edit.start, edit.end, edit.replacement.len()))
                .collect::<Vec<_>>();
            let _ = std::io::Write::write_fmt(
                &mut std::io::stderr().lock(),
                format_args!(
                    "ETUSCHLOPF: ExactReceiptRefusal: owner={}; row={}; reason={reason}; pre_fp={:016x}; post_fp={:016x}; pre_bytes={}; post_bytes={}; edit_count={}; first_bound_spans={spans:?}\n",
                    O::DESCRIPTOR.owner,
                    O::DESCRIPTOR.population_report_row,
                    fingerprint(pre_xml),
                    fingerprint(post_xml),
                    pre_xml.len(),
                    post_xml.len(),
                    edits.len(),
                ),
            );
            return None;
        }
        count(&EXACT_RECEIPTS_MINTED);
        Some(Self {
            ssc_origin: Box::new(crate::ssc_evidence_proof::SscOriginState::capture(&SSC_ORIGIN_ACCESS, pre_xml, post_xml, ssc_exact_scope(O::DESCRIPTOR))),
            identity: MintedIdentity::mint_fresh(),
            descriptor: O::DESCRIPTOR,
            edits,
            pre_fingerprint: fingerprint(pre_xml),
            post_fingerprint: fingerprint(post_xml),
            physical_births: None,
            staff_transition: None,
            final_chord_capture: None,
        })
    }

    #[cfg(test)]
    pub(crate) fn descriptor_row(&self) -> usize {
        self.descriptor.population_report_row
    }

    /// Realizes the post state of this EXACT execution from its bound pre
    /// state by applying the receipt's own edit script. Fail-closed: `None`
    /// unless `pre_xml` is the bound pre state and the result is the bound
    /// post state.
    pub(crate) fn realize_post_document(&self, pre_xml: &str) -> Option<String> {
        self.operation_readback(pre_xml).ok().map(|readback| readback.post_document)
    }

    pub(crate) fn attach_physical_births(&mut self, births: crate::arrangement::physical_births::PhysicalRhythmicBirths) -> anyhow::Result<()> {
        ensure!(self.physical_births.is_none(), "Physical birth capture already attached");
        births.validate_exact_execution(self)?;
        self.physical_births = Some(births);
        Ok(())
    }

    pub(crate) fn attach_staff_transition(&mut self,
        transition: crate::arrangement::physical_births::StaffPresentationTransition) -> anyhow::Result<()> {
        ensure!(self.staff_transition.is_none(), "Staff transition already attached");
        transition.validate_exact_execution(self)?;
        self.staff_transition = Some(transition);
        Ok(())
    }

    pub(crate) fn record_final_chord_capture(&mut self,
        capture: anyhow::Result<FinalChordCourseCapture>) {
        assert!(self.final_chord_capture.is_none(), "Original FinalChord capture reused");
        self.final_chord_capture = Some(capture.map(Box::new).map_err(|error| error.to_string()));
    }

    pub(crate) fn final_chord_capture(&self)
        -> Option<&Result<Box<FinalChordCourseCapture>, String>> {
        self.final_chord_capture.as_ref()
    }

    pub(crate) fn staff_transition(&self) -> Option<&crate::arrangement::physical_births::StaffPresentationTransition> {
        self.staff_transition.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn take_staff_transition(&mut self) -> Option<crate::arrangement::physical_births::StaffPresentationTransition> {
        self.staff_transition.take()
    }

    pub(crate) fn take_physical_births(&mut self) -> Option<crate::arrangement::physical_births::PhysicalRhythmicBirths> { self.physical_births.take() }

    /// Read-only execution evidence from the original owner script. A carried
    /// range supplies no origin, conservation verdict or adoption permission.
    pub(crate) fn operation_readback<'a>(&'a self, pre_xml: &'a str) -> anyhow::Result<ExactOperationReadback<'a>> {
        if fingerprint(pre_xml) != self.pre_fingerprint {
            anyhow::bail!("Exact operation readback: foreign pre-state");
        }
        let post_document = realize_edit_script(pre_xml, &self.edits)
            .ok_or_else(|| anyhow::anyhow!("Exact operation readback: invalid owner script"))?;
        ensure!(fingerprint(&post_document) == self.post_fingerprint,
            "Exact operation readback: foreign post-state");
        let readback = ExactOperationReadback {
            pre_document: pre_xml, post_document, edits: &self.edits,
        };
        // Replay's unchanged spans also exercise the same transport primitive
        // used for caller-bound occurrences. This verifies bytes, never identity.
        let mut cursor = 0;
        for edit in &self.edits {
            readback.verify_unchanged_span(pre_xml, cursor..edit.start)?;
            cursor = edit.end;
        }
        readback.verify_unchanged_span(pre_xml, cursor..pre_xml.len())?;
        Ok(readback)
    }

    /// Whether this receipt was minted by the owner bound to `O`.
    pub(crate) fn minted_by<O: ExactMintOwner>(&self) -> bool {
        self.descriptor.same_contract(O::DESCRIPTOR)
    }
}

/// Borrows the original receipt; it cannot mint, clone or replace its authority.
/// The output covers only supplied pre-existing ranges. Inserted occurrences
/// need their actual creator's birth evidence; removal needs discharge evidence.
#[derive(Debug)]
pub(crate) struct ExactOperationReadback<'a> {
    pre_document: &'a str,
    post_document: String,
    edits: &'a [ExactEdit],
}

impl ExactOperationReadback<'_> {
    pub(crate) fn post_document(&self) -> &str { &self.post_document }

    fn verify_unchanged_span(&self, before: &str, range: std::ops::Range<usize>) -> anyhow::Result<()> {
        if range.is_empty() { return Ok(()); }
        let after = self.carry_range(range.clone())?;
        ensure!(before.get(range) == self.post_document().get(after),
            "Exact operation readback: unedited bytes drifted");
        Ok(())
    }

    pub(crate) fn carry_range(&self, range: std::ops::Range<usize>) -> anyhow::Result<std::ops::Range<usize>> {
        ensure!(range.start < range.end && self.pre_document.get(range.clone()).is_some(),
            "Exact operation readback: invalid source range");
        let mut before_removed = 0usize;
        let mut before_inserted = 0usize;
        let mut inside_removed = 0usize;
        let mut inside_inserted = 0usize;
        for edit in self.edits {
            if edit.end <= range.start {
                before_removed += edit.end - edit.start;
                before_inserted += edit.replacement.len();
            } else if edit.start >= range.end {
                break;
            } else {
                ensure!(edit.start >= range.start && edit.end <= range.end,
                    "Exact operation readback: operation crosses occurrence boundary");
                inside_removed += edit.end - edit.start;
                inside_inserted += edit.replacement.len();
            }
        }
        let start = range.start - before_removed + before_inserted;
        let end = range.end - before_removed + before_inserted - inside_removed + inside_inserted;
        ensure!(start < end && self.post_document.get(start..end).is_some(),
            "Exact operation readback: removed or invalid occurrence requires separate accounting");
        Ok(start..end)
    }
}

#[cfg(test)]
#[path = "../tests/unit/exact_owner_operation_readback.rs"]
mod exact_owner_operation_readback_tests;

/// Byte-exact verification without materializing a new document: every
/// unedited segment of `pre_xml` must reappear unchanged in `post_xml`, every
/// replacement must appear at its shifted position, and the lengths must close.
fn exact_edits_reproduce(pre_xml: &str, post_xml: &str, edits: &[ExactEdit]) -> bool {
    if edits.is_empty() {
        return false;
    }
    let mut pre_cursor = 0_usize;
    let mut post_cursor = 0_usize;
    let mut previous: Option<&ExactEdit> = None;
    for edit in edits {
        if edit.start > edit.end || edit.end > pre_xml.len() || edit.start < pre_cursor {
            return false;
        }
        if let Some(previous) = previous
            && previous.start == edit.start
            && previous.end == previous.start
            && edit.end == edit.start
        {
            // Two insertions at the same point have no unique order.
            return false;
        }
        let Some(unchanged) = pre_xml.get(pre_cursor..edit.start) else {
            return false;
        };
        let unchanged_end = post_cursor + unchanged.len();
        if post_xml.get(post_cursor..unchanged_end) != Some(unchanged) {
            return false;
        }
        let replacement_end = unchanged_end + edit.replacement.len();
        if post_xml.get(unchanged_end..replacement_end) != Some(edit.replacement.as_str()) {
            return false;
        }
        if pre_xml.get(edit.start..edit.end).is_none() {
            return false;
        }
        pre_cursor = edit.end;
        post_cursor = replacement_end;
        previous = Some(edit);
    }
    pre_xml.get(pre_cursor..).is_some() && pre_xml.get(pre_cursor..) == post_xml.get(post_cursor..)
}

/// Evaluates an ascending, non-overlapping byte edit script against `source`.
/// A pure function of its inputs: it decides nothing and mutates no document;
/// it only evaluates a script that an owner bound at origin.
fn realize_edit_script(source: &str, edits: &[ExactEdit]) -> Option<String> {
    let mut out = String::with_capacity(source.len());
    let mut cursor = 0_usize;
    for edit in edits {
        if edit.start < cursor || edit.start > edit.end {
            return None;
        }
        out.push_str(source.get(cursor..edit.start)?);
        out.push_str(&edit.replacement);
        source.get(edit.start..edit.end)?;
        cursor = edit.end;
    }
    out.push_str(source.get(cursor..)?);
    Some(out)
}

/// Canonical pre-coordinate script of bound edits that an owner applies in the
/// given order to its progressively mutated buffer, where the application
/// order is start-descending so every coordinate stays a pre-state coordinate.
/// Consecutive edits sharing one start are merged in application order: each
/// later edit at that point lands in front of the earlier ones. This is a pure
/// function of the bound edits, never a post-state diff.
pub(crate) fn exact_script_from_descending_application(
    edits_in_application_order: Vec<(usize, usize, String)>,
) -> Vec<ExactEdit> {
    let mut merged = Vec::<ExactEdit>::with_capacity(edits_in_application_order.len());
    for (start, end, replacement) in edits_in_application_order {
        if let Some(last) = merged.last_mut()
            && last.start == start
        {
            // The later edit replaces `end - start` bytes of the already
            // mutated buffer at `start`: first the earlier replacement, then
            // pre bytes after the earlier edit's end.
            let consumed = end - start;
            match last.replacement.get(consumed..) {
                Some(rest) => last.replacement = format!("{replacement}{rest}"),
                None => {
                    last.end += consumed.saturating_sub(last.replacement.len());
                    last.replacement = replacement;
                }
            }
            continue;
        }
        merged.push(ExactEdit {
            start,
            end,
            replacement,
        });
    }
    merged.reverse();
    merged
}

/// Composes two bound edit scripts of one owner execution into one script in
/// the coordinates of the original pre-state. `first` is bound against
/// `pre_xml`; `second` is bound against the intermediate document that `first`
/// produced. Edits of `second` that lie in unchanged text are shifted back;
/// edits that touch or overlap a replacement of `first` are folded into one
/// combined edit. No post-state diff is used: the geometry comes only from the
/// two bound scripts. Returns `None` if a script is not ascending and
/// non-overlapping.
pub(crate) fn compose_exact_edit_scripts(
    pre_xml: &str,
    mut first: Vec<ExactEdit>,
    mut second: Vec<ExactEdit>,
) -> Option<Vec<ExactEdit>> {
    first.sort_by_key(|edit| (edit.start, edit.end));
    second.sort_by_key(|edit| (edit.start, edit.end));
    if first
        .windows(2)
        .any(|pair| pair[1].start < pair[0].end)
    {
        return None;
    }
    if second
        .windows(2)
        .any(|pair| pair[1].start < pair[0].end)
    {
        return None;
    }
    let mid_xml = realize_edit_script(pre_xml, &first)?;
    // Mid-coordinate regions of the first script's replacements.
    let mut regions = Vec::<(usize, usize, usize)>::with_capacity(first.len()); // (mid_start, mid_end, first_index)
    let mut delta: isize = 0;
    for (index, edit) in first.iter().enumerate() {
        let mid_start = usize::try_from(edit.start as isize + delta).ok()?;
        let mid_end = mid_start + edit.replacement.len();
        regions.push((mid_start, mid_end, index));
        delta += edit.replacement.len() as isize - (edit.end - edit.start) as isize;
    }
    // Maps a mid position that is not strictly inside a first-script region
    // back to pre coordinates.
    let mid_to_pre = |position: usize| -> Option<usize> {
        let mut shift: isize = 0;
        for (mid_start, mid_end, index) in &regions {
            let edit = &first[*index];
            if position < *mid_start {
                break;
            }
            if position == *mid_start {
                return Some(edit.start);
            }
            if position < *mid_end {
                return None;
            }
            if position == *mid_end {
                return Some(edit.end);
            }
            shift += edit.replacement.len() as isize - (edit.end - edit.start) as isize;
        }
        usize::try_from(position as isize - shift).ok()
    };
    // Cluster: every second edit together with all first regions it touches.
    #[derive(Clone, Copy)]
    struct Cluster {
        mid_start: usize,
        mid_end: usize,
    }
    let mut intervals = Vec::<(usize, usize, bool)>::new(); // (start, end, is_second)
    for (mid_start, mid_end, _) in &regions {
        intervals.push((*mid_start, *mid_end, false));
    }
    for edit in &second {
        intervals.push((edit.start, edit.end, true));
    }
    intervals.sort_by_key(|interval| (interval.0, interval.1));
    let mut clusters = Vec::<(Cluster, bool)>::new(); // (span, contains a second edit)
    for (start, end, is_second) in intervals {
        if let Some((cluster, has_second)) = clusters.last_mut()
            && start <= cluster.mid_end
        {
            cluster.mid_end = cluster.mid_end.max(end);
            *has_second |= is_second;
            continue;
        }
        clusters.push((
            Cluster {
                mid_start: start,
                mid_end: end,
            },
            is_second,
        ));
    }
    let mut composed = Vec::<ExactEdit>::new();
    for (cluster, has_second) in clusters {
        // Pre-coordinate span: the union of the first edits inside the cluster
        // and the mapped boundaries of unchanged text.
        let mut pre_start = None;
        let mut pre_end = None;
        for (mid_start, mid_end, index) in &regions {
            if *mid_end < cluster.mid_start || *mid_start > cluster.mid_end {
                continue;
            }
            let edit = &first[*index];
            pre_start = Some(pre_start.map_or(edit.start, |value: usize| value.min(edit.start)));
            pre_end = Some(pre_end.map_or(edit.end, |value: usize| value.max(edit.end)));
        }
        let mapped_start = mid_to_pre(cluster.mid_start)?;
        let mapped_end = mid_to_pre(cluster.mid_end)?;
        let pre_start = pre_start.map_or(mapped_start, |value| value.min(mapped_start));
        let pre_end = pre_end.map_or(mapped_end, |value| value.max(mapped_end));
        let mut replacement = mid_xml.get(cluster.mid_start..cluster.mid_end)?.to_string();
        if has_second {
            let local_edits = second
                .iter()
                .filter(|edit| edit.start >= cluster.mid_start && edit.end <= cluster.mid_end)
                .map(|edit| ExactEdit {
                    start: edit.start - cluster.mid_start,
                    end: edit.end - cluster.mid_start,
                    replacement: edit.replacement.clone(),
                })
                .collect::<Vec<_>>();
            replacement = realize_edit_script(&replacement, &local_edits)?;
        }
        composed.push(ExactEdit {
            start: pre_start,
            end: pre_end,
            replacement,
        });
    }
    Some(composed)
}

// ---------------------------------------------------------------------------
// PRESERVING
// ---------------------------------------------------------------------------

/// One original owner has one typed continuation observation. Keep the
/// alternative behind the receipt's existing pointer, rather than enlarging
/// every owner token and its recoverable error path.
#[derive(Debug)]
enum RecordedPreservingCourseTransport {
    Representation(crate::arrangement::representation_transport::RhythmicRepresentationTransport),
    FinalLayout(Result<crate::structural_write_authority::FinalLayoutCourseTransport, String>),
}

/// One execution of a PRESERVING exclusion owner whose conservation contract
/// was verified at the owner: the musical content projection of the pre-state
/// equals that of the post-state. It is not a BLOCK substitute: it exists only
/// when the verifier proves that nothing in the conserved quantities changed.
/// Move-only.
#[derive(Debug)]
pub(crate) struct PreservingTransformationReceipt {
    ssc_origin: Box<crate::ssc_evidence_proof::SscOriginState>,
    identity: MintedIdentity,
    descriptor: &'static PreservingDescriptor,
    conserved_fingerprint: u64,
    pre_fingerprint: u64,
    post_fingerprint: u64,
    /// Post bytes realized inside a path-bound owner's own transaction (the
    /// owner's verified readback of its single write), carried move-only to the
    /// Master adoption boundary. `None` for buffer owners, whose post state is
    /// the adopting buffer itself.
    realized_post_document: Option<String>,
    /// Operation journal recorded by the minting owner from its own token
    /// (E044/E013 layout writes only). Observation for course custody; never
    /// read by the verifier, the commit or the adoption verdict.
    layout_journal: Option<Box<crate::arrangement::representation_transport::RepresentationStep>>,
    recorded_representation: Option<Box<RecordedPreservingCourseTransport>>,
}

/// Read-only original E007 course, recorded only when its real receipt commits.
/// Private fields prevent source/readback code from manufacturing an observation.
#[derive(Clone)]
pub(crate) struct CommittedFinalLayoutObservation {
    identity: MintedIdentity,
    domain: crate::ssc_evidence_proof::SscAdoptionDomain,
    before: String,
    after: String,
}
impl CommittedFinalLayoutObservation {
    pub(crate) fn documents(&self) -> (&str, &str) { (&self.before, &self.after) }
}
static COMMITTED_FINAL_LAYOUT_OBSERVATIONS: Mutex<Vec<CommittedFinalLayoutObservation>> = Mutex::new(Vec::new());
pub(crate) fn clear_final_layout_observations() {
    COMMITTED_FINAL_LAYOUT_OBSERVATIONS.lock().unwrap_or_else(|e|e.into_inner()).clear();
}
pub(crate) fn committed_final_layout_observation(post: &str, domain: crate::ssc_evidence_proof::SscAdoptionDomain) -> Option<CommittedFinalLayoutObservation> {
    COMMITTED_FINAL_LAYOUT_OBSERVATIONS.lock().unwrap_or_else(|e|e.into_inner()).iter().rev()
        .find(|record| record.after==post && record.domain==domain).cloned()
}

impl PreservingTransformationReceipt {
    pub(crate) fn with_final_layout_course_transport(mut self,
        transport: anyhow::Result<crate::structural_write_authority::FinalLayoutCourseTransport>) -> Self {
        self.recorded_representation = Some(Box::new(RecordedPreservingCourseTransport::FinalLayout(
            transport.map_err(|error| error.to_string()))));
        self
    }

    fn final_layout_course_transport(&self) -> anyhow::Result<Option<&crate::structural_write_authority::FinalLayoutCourseTransport>> {
        let Some(RecordedPreservingCourseTransport::FinalLayout(transport)) = self.recorded_representation.as_deref()
            else { return Ok(None); };
        let transport = transport.as_ref().map_err(|error| anyhow::anyhow!("Original E007 copy observation: {error}"))?;
        let (before, after) = transport.documents();
        self.validate_preserving_transition::<crate::structural_write_authority::FinalLayoutPhrasePreservingMint>(before, after)?;
        Ok(Some(transport))
    }

    fn recorded_representation(&self) -> Option<&crate::arrangement::representation_transport::RhythmicRepresentationTransport> {
        match self.recorded_representation.as_deref() {
            Some(RecordedPreservingCourseTransport::Representation(transport)) => Some(transport),
            _ => None,
        }
    }
    /// Attaches the owner's own operation journal to its freshly minted receipt.
    pub(crate) fn with_layout_journal(mut self,
        journal: Option<crate::arrangement::representation_transport::RepresentationStep>) -> Self {
        self.layout_journal = journal.map(Box::new);
        self
    }

    /// Binds an owner-recorded layout journal to this original receipt of one of
    /// the two frozen single-write PRESERVING layout owners (E044 defaults
    /// insertion, E013 local staff distance). Reads fingerprints only.
    pub(crate) fn binds_layout_journal(&self, before: &str, after: &str) -> anyhow::Result<()> {
        ensure!(self.descriptor == &VISUAL_LAYOUT_DEFAULTS_PRESERVING
            || self.descriptor == &LOCAL_STAFF_DISTANCE_PRESERVING,
            "Unsupported preserving course continuation owner");
        ensure!(self.realized_post_document.is_none(), "Path-bound preserving owner is not a buffer write");
        ensure!(self.pre_fingerprint == fingerprint(before) && self.post_fingerprint == fingerprint(after),
            "Layout journal does not bind the original preserving receipt");
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn without_layout_journal_for_test(mut self) -> Self {
        self.layout_journal = None;
        self
    }

    /// Original owner journal travels with its original receipt. It creates no
    /// occurrence identity and supplies no adoption or write permission.
    pub(crate) fn with_recorded_representation(mut self,
        transport: crate::arrangement::representation_transport::RhythmicRepresentationTransport) -> Self {
        self.recorded_representation = Some(Box::new(RecordedPreservingCourseTransport::Representation(transport)));
        self
    }

    pub(crate) fn binds_recorded_representation(&self, before: &str, after: &str) -> anyhow::Result<()> {
        ensure!([NOTATION_POLISH_PRESERVING.owner, PIANIST_LAYOUT_PRESERVING.owner,
            PARTIAL_BEAT_BEAM_PRESERVING.owner, LEDGER_BOUNDED_LOWER_STAFF_CLEF_PRESERVING.owner,
            RUSTIG_CLEF_COURSE_PRESERVING.owner]
            .contains(&self.descriptor.owner), "Unsupported recorded preserving owner");
        ensure!(self.pre_fingerprint == fingerprint(before) && self.post_fingerprint == fingerprint(after),
            "Recorded representation does not bind the original preserving receipt");
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn has_recorded_representation_for_test(&self) -> bool {
        self.recorded_representation().is_some()
    }

    #[cfg(test)]
    pub(crate) fn without_recorded_representation_for_test(mut self) -> Self {
        self.recorded_representation = None; self
    }

    pub(crate) fn ssc_bound_origin(&self) -> (&MintedIdentity, &crate::ssc_evidence_proof::SscOriginState) { (&self.identity, &self.ssc_origin) }
    /// Mints a PRESERVING receipt only if both documents parse and their
    /// conserved content projections are identical. An unchanged transition is
    /// no event (`None`); a refused conservation proof is counted and a Master
    /// adoption of it is fatal.
    pub(crate) fn mint_preserving_receipt<O: PreservingMintOwner>(
        _owner: O,
        pre_xml: &str,
        post_xml: &str,
    ) -> Option<Self> {
        if pre_xml == post_xml {
            return None;
        }
        let conserved = O::conserved_projection(pre_xml);
        if conserved.is_none() || conserved != O::conserved_projection(post_xml) {
            count_owner_receipt_refusal();
            return None;
        }
        count(&PRESERVING_RECEIPTS_MINTED);
        Some(Self {
            ssc_origin: Box::new(crate::ssc_evidence_proof::SscOriginState::capture(&SSC_ORIGIN_ACCESS, pre_xml, post_xml, crate::ssc_evidence_proof::SscOriginScope::Preserving(ssc_preserving_origin::<O>()))),
            identity: MintedIdentity::mint_fresh(),
            descriptor: O::DESCRIPTOR,
            conserved_fingerprint: fingerprint(&conserved?),
            pre_fingerprint: fingerprint(pre_xml),
            post_fingerprint: fingerprint(post_xml),
            realized_post_document: None,
            layout_journal: None,
            recorded_representation: None,
        })
    }

    /// PRESERVING mint of a path-bound owner (its frozen signature takes a file
    /// path, not a buffer). The owner captured `pre_xml` before its write and
    /// `post_xml` is its own verified readback inside the same transaction; the
    /// receipt carries those post bytes move-only so the Master adoption needs
    /// no later file reimport.
    pub(crate) fn mint_file_bound_preserving_receipt<O: PreservingMintOwner>(
        owner: O,
        pre_xml: &str,
        post_xml: String,
    ) -> Option<Self> {
        let mut receipt = Self::mint_preserving_receipt(owner, pre_xml, &post_xml)?;
        receipt.realized_post_document = Some(post_xml);
        Some(receipt)
    }

    /// Splits the realized post bytes of a path-bound owner from its receipt.
    /// The returned bytes are bound to the receipt's post fingerprint.
    pub(crate) fn split_realized_post_document(mut self) -> (Self, Option<String>) {
        let document = self
            .realized_post_document
            .take()
            .filter(|document| fingerprint(document) == self.post_fingerprint);
        (self, document)
    }

    /// Validates owner contract, pre/post binding and the conserved content.
    pub(crate) fn validate_preserving_transition<O: PreservingMintOwner>(
        &self,
        pre_xml: &str,
        post_xml: &str,
    ) -> anyhow::Result<()> {
        ensure!(
            self.descriptor.semantic_anchor() == O::DESCRIPTOR.semantic_anchor(),
            "PreservingTransformationReceipt: owner contract drift"
        );
        ensure!(
            self.pre_fingerprint == fingerprint(pre_xml)
                && self.post_fingerprint == fingerprint(post_xml),
            "PreservingTransformationReceipt: pre/post fingerprint drift"
        );
        let conserved = O::conserved_projection(post_xml)
            .map(|projection| fingerprint(&projection));
        ensure!(
            conserved == Some(self.conserved_fingerprint),
            "PreservingTransformationReceipt: conserved content drift"
        );
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// ADDITIVE
// ---------------------------------------------------------------------------

/// One execution of a frozen ADDITIVE exclusion owner: insertions only, each a
/// fragment of exactly the descriptor's element class; every pre byte survives
/// unchanged; the pre conserved projection survives as a subsequence. Move-only.
#[derive(Debug)]
pub(crate) struct AdditiveTransformationReceipt {
    identity: MintedIdentity,
    descriptor: &'static AdditiveDescriptor,
    insertions: usize,
    pre_fingerprint: u64,
    post_fingerprint: u64,
}

impl AdditiveTransformationReceipt {
    /// Mints an ADDITIVE receipt only if every bound edit is an insertion
    /// (`start == end`), the insertions reproduce the post-state byte-exactly,
    /// each inserted fragment is exactly one element of the descriptor's
    /// insertion class (optionally preceded by one XML comment and
    /// whitespace), and the pre conserved projection is a subsequence of the
    /// post projection.
    pub(crate) fn mint_additive_receipt<O: AdditiveMintOwner>(
        _owner: O,
        pre_xml: &str,
        post_xml: &str,
        mut insertions: Vec<ExactEdit>,
    ) -> Option<Self> {
        if pre_xml == post_xml {
            return None;
        }
        insertions.sort_by_key(|edit| (edit.start, edit.end));
        let class = O::DESCRIPTOR.insertion_class;
        let verified = insertions.iter().all(|edit| edit.start == edit.end)
            && insertions
                .iter()
                .all(|edit| additive_fragment_matches_class(&edit.replacement, class))
            && exact_edits_reproduce(pre_xml, post_xml, &insertions)
            && conserved_projection_is_subsequence(pre_xml, post_xml);
        if !verified {
            count_owner_receipt_refusal();
            return None;
        }
        count(&ADDITIVE_RECEIPTS_MINTED);
        Some(Self {
            identity: MintedIdentity::mint_fresh(),
            descriptor: O::DESCRIPTOR,
            insertions: insertions.len(),
            pre_fingerprint: fingerprint(pre_xml),
            post_fingerprint: fingerprint(post_xml),
        })
    }
}

fn additive_fragment_matches_class(fragment: &str, class: AdditiveInsertionClass) -> bool {
    let wrapped = format!("<additive-insertion-root>{fragment}</additive-insertion-root>");
    let Ok(document) = roxmltree::Document::parse(&wrapped) else {
        return false;
    };
    let root = document.root_element();
    let mut elements = 0_usize;
    for child in root.children() {
        if child.is_element() {
            if child.tag_name().name() != class.element_name() {
                return false;
            }
            elements += 1;
        } else if child.is_text() {
            if !child.text().unwrap_or("").trim().is_empty() {
                return false;
            }
        } else if !child.is_comment() {
            return false;
        }
    }
    elements == 1
}

fn conserved_projection_is_subsequence(pre_xml: &str, post_xml: &str) -> bool {
    let (Some(pre), Some(post)) = (
        conserved_content_projection(pre_xml),
        conserved_content_projection(post_xml),
    ) else {
        return false;
    };
    let mut post_lines = post.lines();
    pre.lines()
        .all(|line| post_lines.by_ref().any(|candidate| candidate == line))
}

/// Canonical projection of the conserved musical content of a MusicXML
/// document, in document order: parts and measures; per note pitch or rest,
/// duration, chord, grace, cue, voice, staff, tie types and visibility;
/// backup and forward durations; divisions, key and time; barlines.
/// Beams, stems, clefs, notations other than ties, directions, print and
/// layout elements are presentation and may change under a PRESERVING contract.
fn conserved_content_projection(xml: &str) -> Option<String> {
    let document = roxmltree::Document::parse(xml).ok()?;
    let child_text = |node: roxmltree::Node<'_, '_>, name: &str| -> String {
        node.children()
            .find(|child| child.has_tag_name(name))
            .and_then(|child| child.text())
            .map_or_else(String::new, |text| text.trim().to_string())
    };
    let has_child = |node: roxmltree::Node<'_, '_>, name: &str| {
        node.children().any(|child| child.has_tag_name(name))
    };
    let mut out = String::new();
    for part in document.descendants().filter(|node| node.has_tag_name("part")) {
        out.push_str(&format!("P|{}\n", part.attribute("id").unwrap_or("")));
        for measure in part.children().filter(|node| node.has_tag_name("measure")) {
            out.push_str("M\n");
            for element in measure.children().filter(roxmltree::Node::is_element) {
                match element.tag_name().name() {
                    "note" => {
                        let content = element
                            .children()
                            .find(|child| child.has_tag_name("pitch"))
                            .map_or_else(
                                || {
                                    if has_child(element, "rest") {
                                        "rest".to_string()
                                    } else {
                                        "unpitched".to_string()
                                    }
                                },
                                |pitch| {
                                    format!(
                                        "{}{}/{}",
                                        child_text(pitch, "step"),
                                        child_text(pitch, "octave"),
                                        child_text(pitch, "alter")
                                    )
                                },
                            );
                        let mut ties = element
                            .children()
                            .filter(|child| child.has_tag_name("tie"))
                            .map(|tie| tie.attribute("type").unwrap_or("").to_string())
                            .collect::<Vec<_>>();
                        ties.sort();
                        out.push_str(&format!(
                            "N|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                            content,
                            child_text(element, "duration"),
                            has_child(element, "chord"),
                            has_child(element, "grace"),
                            has_child(element, "cue"),
                            child_text(element, "voice"),
                            child_text(element, "staff"),
                            ties.join(","),
                            element.attribute("print-object").unwrap_or("yes"),
                        ));
                    }
                    "backup" => out.push_str(&format!("B|{}\n", child_text(element, "duration"))),
                    "forward" => out.push_str(&format!(
                        "F|{}|{}|{}\n",
                        child_text(element, "duration"),
                        child_text(element, "voice"),
                        child_text(element, "staff")
                    )),
                    "attributes" => {
                        // Only conserved attributes are projected; an attributes
                        // element that carries only clefs or staff details is
                        // presentation and leaves no trace.
                        if has_child(element, "divisions") {
                            out.push_str(&format!("D|{}\n", child_text(element, "divisions")));
                        }
                        for key in element.children().filter(|child| child.has_tag_name("key")) {
                            out.push_str(&format!("K|{}\n", child_text(key, "fifths")));
                        }
                        for time in element.children().filter(|child| child.has_tag_name("time")) {
                            out.push_str(&format!(
                                "T|{}/{}\n",
                                child_text(time, "beats"),
                                child_text(time, "beat-type")
                            ));
                        }
                    }
                    "barline" => {
                        let repeat = element
                            .children()
                            .find(|child| child.has_tag_name("repeat"))
                            .and_then(|repeat| repeat.attribute("direction"))
                            .unwrap_or("");
                        out.push_str(&format!(
                            "L|{}|{}|{}\n",
                            element.attribute("location").unwrap_or("right"),
                            child_text(element, "bar-style"),
                            repeat
                        ));
                    }
                    _ => {}
                }
            }
        }
    }
    Some(out)
}

// ---------------------------------------------------------------------------
// Typed owner receipts in transport. Move-only: a still-unadopted receipt can
// never be cloned or shared between variants.
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum TransformationReceipt {
    Block(BlockTransformationReceipt),
    Exact(ExactTransformationReceipt),
    Preserving(PreservingTransformationReceipt),
    Additive(AdditiveTransformationReceipt),
}

impl TransformationReceipt {
    fn pre_fingerprint(&self) -> u64 {
        match self {
            Self::Block(receipt) => receipt.pre_fingerprint,
            Self::Exact(receipt) => receipt.pre_fingerprint,
            Self::Preserving(receipt) => receipt.pre_fingerprint,
            Self::Additive(receipt) => receipt.pre_fingerprint,
        }
    }

    fn post_fingerprint(&self) -> u64 {
        match self {
            Self::Block(receipt) => receipt.post_fingerprint,
            Self::Exact(receipt) => receipt.post_fingerprint,
            Self::Preserving(receipt) => receipt.post_fingerprint,
            Self::Additive(receipt) => receipt.post_fingerprint,
        }
    }

    fn identity(&self) -> MintedIdentity {
        match self {
            Self::Block(receipt) => receipt.identity,
            Self::Exact(receipt) => receipt.identity,
            Self::Preserving(receipt) => receipt.identity,
            Self::Additive(receipt) => receipt.identity,
        }
    }

    /// Read-only view for the A7 adoption verdict at a wired boundary (r1287).
    fn adoption_view(&self) -> crate::ssc_evidence_proof::AdoptionReceiptView<'_> {
        match self {
            Self::Block(receipt) => crate::ssc_evidence_proof::AdoptionReceiptView::Block(receipt),
            Self::Exact(receipt) => crate::ssc_evidence_proof::AdoptionReceiptView::Exact(receipt),
            Self::Preserving(receipt) => crate::ssc_evidence_proof::AdoptionReceiptView::Preserving(receipt),
            Self::Additive(receipt) => crate::ssc_evidence_proof::AdoptionReceiptView::Additive(receipt),
        }
    }

    fn commit(self, domain: crate::ssc_evidence_proof::SscAdoptionDomain) -> CommittedTransformationEvent {
        let pre_fingerprint = self.pre_fingerprint();
        let post_fingerprint = self.post_fingerprint();
        let identity = self.identity();
        let contract = match &self {
            Self::Block(receipt) => CommittedContract::Block {
                row: receipt.descriptor.population_report_row,
                span_bound: receipt.bound_span.is_some(),
            },
            Self::Exact(receipt) => CommittedContract::Exact {
                row: receipt.descriptor.population_report_row,
                edits: receipt.edits.len(),
            },
            Self::Preserving(receipt) => CommittedContract::Preserving {
                exclusion: receipt.descriptor.exclusion_id,
            },
            Self::Additive(receipt) => CommittedContract::Additive {
                exclusion: receipt.descriptor.exclusion_id,
                insertions: receipt.insertions,
            },
        };
        let event = CommittedTransformationEvent {
            identity,
            contract,
            pre_fingerprint,
            post_fingerprint,
        };
        // This is the original receipt-consuming commit after the unchanged
        // SSC verdict. Carry its sealed E007 observation as data, not a domain
        // callback or a reconstructed origin, and never alter the adoption.
        if let Self::Preserving(receipt) = &self
            && let Ok(Some(course)) = receipt.final_layout_course_transport() {
            let (before, after) = course.documents();
            let mut observations = COMMITTED_FINAL_LAYOUT_OBSERVATIONS.lock().unwrap_or_else(|e|e.into_inner());
            if !observations.iter().any(|record|record.identity==identity && record.domain==domain) {
                observations.push(CommittedFinalLayoutObservation {identity, domain, before:before.to_owned(), after:after.to_owned()});
            }
        }
        match self {
            Self::Block(receipt) => crate::ssc_evidence_proof::record_block_commit(&SSC_ORIGIN_ACCESS, receipt, domain),
            Self::Exact(receipt) => crate::ssc_evidence_proof::record_exact_commit(&SSC_ORIGIN_ACCESS, receipt, domain),
            Self::Preserving(receipt) => crate::ssc_evidence_proof::record_preserving_commit(&SSC_ORIGIN_ACCESS, receipt, domain),
            Self::Additive(receipt) => crate::ssc_evidence_proof::record_additive_commit(&SSC_ORIGIN_ACCESS, receipt, domain),
        }
        event
    }
}

/// Receipts returned by owners for one transition, in application order.
/// Move-only.
#[derive(Debug, Default)]
pub(crate) struct OwnerReceipts {
    receipts: Vec<TransformationReceipt>,
    physical_block_births: Vec<(MintedIdentity, crate::arrangement::physical_births::PhysicalRhythmicBirths)>,
    physical_block_capture_errors: Vec<(MintedIdentity, String)>,
    recorded_block_transports: Vec<(MintedIdentity, Result<crate::arrangement::representation_transport::RhythmicRepresentationTransport, String>)>,
    actual_beam_captures: Vec<(MintedIdentity, Result<std::sync::Arc<crate::pipeline::ActualCanonicalBeamCapture>, String>)>,
}

/// Move-only companion; the pinned original BLOCK receipt is unchanged.
#[derive(Debug)]
pub(crate) struct CapturedBlockReceiptInput {
    receipt: Option<BlockTransformationReceipt>,
    beam: Option<Result<std::sync::Arc<crate::pipeline::ActualCanonicalBeamCapture>,String>>,
}
impl From<Option<BlockTransformationReceipt>> for CapturedBlockReceiptInput {
    fn from(receipt: Option<BlockTransformationReceipt>) -> Self {Self{receipt,beam:None}}
}
impl CapturedBlockReceiptInput {
    pub(crate) fn observed_beam(receipt: BlockTransformationReceipt,
        capture: anyhow::Result<crate::pipeline::ActualCanonicalBeamCapture>) -> Self {
        let beam=capture.and_then(|capture| {capture.binds_original_receipt(&receipt)?;Ok(std::sync::Arc::new(capture))})
            .map_err(|e|e.to_string());
        Self{receipt:Some(receipt),beam:Some(beam)}
    }
    #[cfg(test)]
    pub(crate) fn into_parts_for_test(self) -> (Option<BlockTransformationReceipt>,Option<Result<std::sync::Arc<crate::pipeline::ActualCanonicalBeamCapture>,String>>) {
        (self.receipt,self.beam)
    }
}

/// Original BLOCK receipts travel with their emitter journal without changing
/// the frozen caller or mint layout. Deref exposes the original receipt slice.
#[derive(Debug, Default)]
pub(crate) struct RecordedClefReceipts {
    receipts: Vec<BlockTransformationReceipt>,
    transport: Option<Result<crate::arrangement::representation_transport::RhythmicRepresentationTransport, String>>,
}
impl std::ops::Deref for RecordedClefReceipts {
    type Target = Vec<BlockTransformationReceipt>;
    fn deref(&self) -> &Self::Target { &self.receipts }
}
impl RecordedClefReceipts {
    pub(crate) fn new(receipts: Vec<BlockTransformationReceipt>,
        transport: &anyhow::Result<crate::arrangement::representation_transport::RhythmicRepresentationTransport>) -> Self {
        Self { receipts, transport: Some(match transport {
            Ok(value) => Ok(value.clone()), Err(error) => Err(error.to_string()),
        }) }
    }
    #[cfg(test)]
    pub(crate) fn without_capture_for_test(mut self) -> Self {
        self.transport = None; self
    }
}
impl From<Vec<BlockTransformationReceipt>> for OwnerReceipts {
    fn from(receipts: Vec<BlockTransformationReceipt>) -> Self {
        let mut owners = Self::default();
        for receipt in receipts { owners.push_block(Some(receipt)); }
        owners
    }
}
impl From<RecordedClefReceipts> for OwnerReceipts {
    fn from(bundle: RecordedClefReceipts) -> Self {
        let mut owners = Self::default();
        let transport = bundle.transport.unwrap_or_else(|| Err("Missing original clef journal".into()))
            .map_err(|error| anyhow::anyhow!(error));
        owners.push_clef_blocks_with_transport(bundle.receipts, transport);
        owners
    }
}

impl OwnerReceipts {
    /// Read-only custody continuation through original receipts. Failure means
    /// Unknown context, never a replacement receipt or an adoption escape.
    pub(crate) fn continue_owned_rhythm(&self, initial: &crate::arrangement::OwnedRhythmicContinuation,
        after: &str) -> anyhow::Result<crate::arrangement::OwnedRhythmicContinuation> {
        let mut state = initial.clone();
        for (index, receipt) in self.receipts.iter().enumerate() {
            let (row, owner) = match receipt {
                TransformationReceipt::Exact(r) => (r.descriptor.population_report_row, r.descriptor.owner),
                TransformationReceipt::Block(r) => (r.descriptor.population_report_row, r.descriptor.owner),
                TransformationReceipt::Preserving(r) => (0, r.descriptor.owner),
                TransformationReceipt::Additive(r) => (0, r.descriptor.owner),
            };
            let continuation: anyhow::Result<_> = (|| Ok(match receipt {
                TransformationReceipt::Exact(receipt) => {
                    ensure!(receipt.physical_births.is_some()
                        || !matches!(receipt.descriptor.population_report_row, 3 | 26 | 39 | 63),
                        "Missing actual creator context capture");
                    state.continue_exact(receipt, receipt.physical_births.as_ref())?
                },
                TransformationReceipt::Block(receipt) => {
                    if let Some((_, transport)) = self.recorded_block_transports.iter()
                        .find(|(identity, _)| identity == &receipt.identity) {
                        return state.continue_recorded_block(transport.as_ref()
                            .map_err(|error| anyhow::anyhow!("Original notation capture: {error}"))?);
                    }
                    let capture = self.physical_block_births.iter().find(|(identity, _)| identity == &receipt.identity)
                        .map(|(_, capture)| capture).ok_or_else(|| {
                            self.physical_block_capture_errors.iter().find(|(identity,_)| identity==&receipt.identity)
                                .map_or_else(|| anyhow::anyhow!("Unsupported course continuation BLOCK owner"),
                                    |(_,error)| anyhow::anyhow!("Original BLOCK course capture refused: {error}"))
                        })?;
                    capture.block_post_document(receipt, state.document())?;
                    state.continue_visible(capture)?
                },
                TransformationReceipt::Additive(receipt) => state.continue_additive(receipt)?,
                TransformationReceipt::Preserving(receipt) => {
                    if let Some(transport) = receipt.final_layout_course_transport()? {
                        return state.continue_final_layout(transport);
                    }
                    if let Some(transport) = receipt.recorded_representation() {
                        return state.continue_preserving_representation(receipt, transport);
                    }
                    let (_, transport) = self.recorded_block_transports.iter()
                        .find(|(identity, _)| identity == &receipt.identity)
                        .ok_or_else(|| anyhow::anyhow!("Unsupported course continuation owner boundary"))?;
                    state.continue_recorded_block(transport.as_ref()
                        .map_err(|error| anyhow::anyhow!("Original layout capture: {error}"))?)?
                },
            }))();
            state = continuation.map_err(|error| anyhow::anyhow!(
                "OwnerStep={} PopulationRow={} Owner={owner}: {error}", index + 1, row))?;
        }
        state.validate_binding(after)?;
        Ok(state)
    }

    pub(crate) fn push_visible_block_with_births(&mut self, receipt: Option<BlockTransformationReceipt>, births: Option<crate::arrangement::physical_births::PhysicalRhythmicBirths>, before: &str) -> anyhow::Result<()> {
        if let Some(receipt) = receipt {
            let births = births.ok_or_else(|| anyhow::anyhow!("Missing actual normalizer capture"))?;
            births.block_post_document(&receipt, before)?;
            self.physical_block_births.push((receipt.identity, births));
            self.push_block(Some(receipt));
        }
        Ok(())
    }

    /// Observation failure stays attached to the original receipt, never vetoes
    /// or substitutes for the unchanged SSC adoption.
    pub(crate) fn push_octave_block_with_transport(&mut self, receipt: Option<BlockTransformationReceipt>,
        step: anyhow::Result<crate::arrangement::representation_transport::RepresentationStep>) {
        if let Some(receipt) = receipt {
            let transport = step.and_then(|step|
                crate::arrangement::representation_transport::RhythmicRepresentationTransport::octave_notation(step, &receipt));
            self.recorded_block_transports.push((receipt.identity, transport.map_err(|error| error.to_string())));
            self.push_block(Some(receipt));
        }
    }

    pub(crate) fn push_clef_blocks_with_transport(&mut self, receipts: Vec<BlockTransformationReceipt>,
        transport: anyhow::Result<crate::arrangement::representation_transport::RhythmicRepresentationTransport>) {
        match transport.and_then(|transport| transport.clef_receipt_steps(&receipts)) {
            Ok(transports) => for (receipt, transport) in receipts.into_iter().zip(transports) {
                self.recorded_block_transports.push((receipt.identity, Ok(transport)));
                self.push_block(Some(receipt));
            },
            Err(error) => for receipt in receipts {
                self.recorded_block_transports.push((receipt.identity, Err(error.to_string())));
                self.push_block(Some(receipt));
            },
        }
    }
    pub(crate) fn physical_birth_readback(&self, before: &str, after: &str) -> anyhow::Result<Vec<crate::arrangement::physical_births::PhysicalRhythmicBirths>> {
        let mut document = before.to_owned(); let mut births = Vec::new();
        for receipt in &self.receipts {
            document = match receipt {
                TransformationReceipt::Exact(receipt) => {
                    let next = receipt.operation_readback(&document)?;
                    if let Some(capture) = &receipt.physical_births {
                        capture.validate_binding(&document, next.post_document())?;
                        births.push(capture.clone());
                    } else {
                        ensure!(!matches!(receipt.descriptor.population_report_row, 3 | 39 | 63), "Missing physical creator capture");
                    }
                    next.post_document
                },
                TransformationReceipt::Block(receipt) => {
                    let capture = self.physical_block_births.iter().find(|(identity, _)| identity == &receipt.identity)
                        .map(|(_, capture)| capture).ok_or_else(|| anyhow::anyhow!("Unsupported physical birth boundary"))?;
                    let next = capture.block_post_document(receipt, &document)?;
                    births.push(capture.clone()); next
                },
                _ => anyhow::bail!("Unsupported physical birth boundary"),
            };
        }
        ensure!(document == after, "Physical birth readback does not reach the requested state");
        Ok(births)
    }
    /// Read-only failure context. This neither validates adoption nor consumes
    /// receipts; the sovereign adoption boundary remains the only verdict.
    /// A covered continuous byte transition has no diagnostic output.
    pub(crate) fn coverage_diagnostic(&self, before: &str, after: &str) -> Option<String> {
        let before_fingerprint = fingerprint(before);
        let after_fingerprint = fingerprint(after);
        let mut head = before_fingerprint;
        let mut first_discontinuous_receipt = None;
        for (index, receipt) in self.receipts.iter().enumerate() {
            if receipt.pre_fingerprint() != head && first_discontinuous_receipt.is_none() {
                first_discontinuous_receipt = Some(index);
            }
            head = receipt.post_fingerprint();
        }
        if first_discontinuous_receipt.is_none() && head == after_fingerprint {
            return None;
        }
        let last_owner = self.receipts.last().map(|receipt| match receipt {
            TransformationReceipt::Block(receipt) => receipt.descriptor.owner,
            TransformationReceipt::Exact(receipt) => receipt.descriptor.owner,
            TransformationReceipt::Preserving(receipt) => receipt.descriptor.owner,
            TransformationReceipt::Additive(receipt) => receipt.descriptor.owner,
        });
        Some(format!(
            "receipt_count={}; base_fp={before_fingerprint:016x}; receipt_end_fp={head:016x}; target_fp={after_fingerprint:016x}; base_bytes={}; target_bytes={}; first_discontinuous_receipt={first_discontinuous_receipt:?}; last_owner={last_owner:?}",
            self.receipts.len(),
            before.len(),
            after.len(),
        ))
    }

    pub(crate) fn push_exact(&mut self, receipt: Option<ExactTransformationReceipt>) {
        self.receipts.extend(receipt.map(TransformationReceipt::Exact));
    }

    pub(crate) fn push_block(&mut self, receipt: impl Into<CapturedBlockReceiptInput>) {
        let input=receipt.into();
        if let Some(receipt)=input.receipt {
            if let Some(capture)=&input.beam {
                let carry=capture.as_ref().map_err(|error| format!("Original beam capture: {error}"))
                    .and_then(|capture| capture.course_carry_births(&receipt)
                        .map_err(|error| format!("Original complete-note carry: {error}")));
                match carry {
                    Ok(births)=>self.physical_block_births.push((receipt.identity,births)),
                    Err(error)=>self.physical_block_capture_errors.push((receipt.identity,error)),
                }
            }
            if let Some(capture)=input.beam {self.actual_beam_captures.push((receipt.identity,capture));}
            self.receipts.push(TransformationReceipt::Block(receipt));
        }
    }

    pub(crate) fn push_preserving(&mut self, receipt: Option<PreservingTransformationReceipt>) {
        let Some(mut receipt) = receipt else { return; };
        // An owner-recorded layout journal travels keyed by the original receipt
        // identity; its refusal stays observational and never blocks the commit.
        if let Some(journal) = receipt.layout_journal.take() {
            let transport = crate::arrangement::representation_transport::RhythmicRepresentationTransport::preserving_layout(*journal, &receipt);
            self.recorded_block_transports.push((receipt.identity, transport.map_err(|error| error.to_string())));
        }
        self.receipts.push(TransformationReceipt::Preserving(receipt));
    }

    pub(crate) fn push_additive(&mut self, receipt: Option<AdditiveTransformationReceipt>) {
        self.receipts.extend(receipt.map(TransformationReceipt::Additive));
    }

    pub(crate) fn append(&mut self, mut other: OwnerReceipts) {
        self.receipts.append(&mut other.receipts);
        self.physical_block_births.append(&mut other.physical_block_births);
        self.physical_block_capture_errors.append(&mut other.physical_block_capture_errors);
        self.recorded_block_transports.append(&mut other.recorded_block_transports);
        self.actual_beam_captures.append(&mut other.actual_beam_captures);
    }

    pub(crate) fn push_blocks(&mut self, receipts: impl Into<OwnerReceipts>) {
        self.append(receipts.into());
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.receipts.len()
    }
}

// ---------------------------------------------------------------------------
// Lineage of committed events
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum CommittedContract {
    Block { row: usize, span_bound: bool },
    Exact { row: usize, edits: usize },
    Preserving { exclusion: &'static str },
    Additive { exclusion: &'static str, insertions: usize },
}

/// One transformation event already consumed by a Master adoption. Clonable
/// committed history; it grants nothing and cannot be adopted again.
#[derive(Debug, Clone)]
pub(crate) struct CommittedTransformationEvent {
    identity: MintedIdentity,
    contract: CommittedContract,
    pre_fingerprint: u64,
    post_fingerprint: u64,
}

/// Capability of the Master adoption boundary to commit receipts into a
/// lineage. Its constructor lives only in the adoption module, so no pipeline
/// stage, wrapper or helper can extend a productive lineage directly.
pub(crate) trait LineageCommitBoundary: sealed::MintOwner {}

impl sealed::MintOwner for crate::structural_write_authority::MasterAdoptionCommitBoundary {}
impl LineageCommitBoundary for crate::structural_write_authority::MasterAdoptionCommitBoundary {}

/// `GENESIS -> ordered committed owner events*`. Provenance only.
#[derive(Debug, Clone)]
pub(crate) struct MasterLineage {
    genesis: MasterGenesisIdentity,
    events: Vec<CommittedTransformationEvent>,
}

impl MasterLineage {
    pub(crate) fn lineage_from_genesis(genesis: MasterGenesisIdentity) -> Self {
        count(&LINEAGES_STARTED);
        Self {
            genesis,
            events: Vec::new(),
        }
    }

    fn head_fingerprint(&self) -> u64 {
        self.events
            .last()
            .map_or(self.genesis.post_fingerprint, |event| event.post_fingerprint)
    }

    /// Whether the lineage head binds exactly `current_xml`.
    pub(crate) fn lineage_head_binds(&self, current_xml: &str) -> bool {
        self.head_fingerprint() == fingerprint(current_xml)
    }

    /// Commits owner receipts in application order and requires that they
    /// close continuously at `state_after`. Any discontinuity, duplicate
    /// identity or uncovered remainder is a fatal sovereignty violation. Only
    /// the Master adoption boundary can call this.
    pub(crate) fn commit_owner_receipts<B: LineageCommitBoundary>(
        &mut self,
        _boundary: &B,
        owner_receipts: OwnerReceipts,
        state_after: &str,
        transition: &str,
    ) {
        // Contract v4 A7 (r1287): every receipt of the transition is checked (continuity,
        // identity and the SSC adoption verdict) before any receipt is committed, so a
        // refusal never leaves a partial commit and never falls back to another candidate.
        let mut head = self.head_fingerprint();
        let mut admitted = Vec::new();
        for receipt in &owner_receipts.receipts {
            if receipt.pre_fingerprint() != head {
                master_sovereignty_violation(&format!(
                    "{transition}: receipt pre-state is not the lineage head (discontinuous or out-of-order receipt)"
                ));
            }
            let identity = receipt.identity();
            if self.genesis.identity == identity
                || self.events.iter().any(|event| event.identity == identity)
                || admitted.contains(&identity)
            {
                master_sovereignty_violation(&format!(
                    "{transition}: receipt identity already committed"
                ));
            }
            if let Err(refusal) = crate::ssc_evidence_proof::ssc_adoption_verdict(
                &SSC_ORIGIN_ACCESS,
                receipt.adoption_view(),
                crate::ssc_evidence_proof::SscAdoptionDomain::Master,
            ) {
                master_sovereignty_violation(&format!("{transition}: {refusal}"));
            }
            admitted.push(identity);
            head = receipt.post_fingerprint();
        }
        if head != fingerprint(state_after) {
            master_sovereignty_violation(&format!(
                "{transition}: owner receipts do not cover the adopted Master transition"
            ));
        }
        for receipt in owner_receipts.receipts {
            count(&RECEIPTS_COMMITTED);
            self.events.push(receipt.commit(crate::ssc_evidence_proof::SscAdoptionDomain::Master));
        }
    }

    /// Mechanical continuity check: GENESIS post = first pre, event N post =
    /// event N+1 pre, last post = current state, identities unique.
    pub(crate) fn lineage_continuity_holds(&self, current_xml: &str) -> bool {
        let mut head = self.genesis.post_fingerprint;
        let mut seen = std::collections::BTreeSet::new();
        seen.insert(self.genesis.identity.opaque_value());
        for event in &self.events {
            if event.pre_fingerprint != head || !seen.insert(event.identity.opaque_value()) {
                return false;
            }
            head = event.post_fingerprint;
        }
        head == fingerprint(current_xml)
    }

    #[cfg(test)]
    pub(crate) fn committed_event_count(&self) -> usize {
        self.events.len()
    }

    fn summary(&self, current_xml: &str) -> FinalMasterLineageSummary {
        let mut summary = FinalMasterLineageSummary {
            genesis_row: self.genesis.descriptor.population_report_row,
            genesis_owner: self.genesis.descriptor.owner,
            continuity_holds: self.lineage_continuity_holds(current_xml),
            ..FinalMasterLineageSummary::default()
        };
        for event in &self.events {
            match event.contract {
                CommittedContract::Block { row, span_bound } => {
                    summary.block_receipts += 1;
                    summary.block_spans_bound += usize::from(span_bound);
                    *summary.block_rows.entry(row).or_default() += 1;
                }
                CommittedContract::Exact { row, edits } => {
                    summary.exact_receipts += 1;
                    summary.exact_edits += edits;
                    *summary.exact_rows.entry(row).or_default() += 1;
                }
                CommittedContract::Preserving { exclusion } => {
                    summary.preserving_receipts += 1;
                    *summary.preserving_contracts.entry(exclusion).or_default() += 1;
                }
                CommittedContract::Additive {
                    exclusion,
                    insertions,
                } => {
                    summary.additive_receipts += 1;
                    summary.additive_insertions += insertions;
                    *summary.additive_contracts.entry(exclusion).or_default() += 1;
                }
            }
        }
        summary
    }
}

// ---------------------------------------------------------------------------
// Final projection (report only)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
struct FinalMasterLineageSummary {
    genesis_row: usize,
    genesis_owner: &'static str,
    block_receipts: usize,
    block_spans_bound: usize,
    exact_receipts: usize,
    exact_edits: usize,
    preserving_receipts: usize,
    additive_receipts: usize,
    additive_insertions: usize,
    continuity_holds: bool,
    block_rows: BTreeMap<usize, usize>,
    exact_rows: BTreeMap<usize, usize>,
    preserving_contracts: BTreeMap<&'static str, usize>,
    additive_contracts: BTreeMap<&'static str, usize>,
}

#[derive(Debug, Clone, Copy)]
struct MasterArtifactWitness {
    artifact_matches_sovereign_state: bool,
}

/// Counts one completed Master adoption (projection only).
pub(crate) fn count_master_adoption() {
    count(&MASTER_ADOPTIONS);
}

/// Resets the process-local measurement at the start of one project run.
pub(crate) fn reset_master_lineage_measurement() {
    crate::ssc_evidence_proof::reset_origin_measurement(&SSC_ORIGIN_ACCESS);
    for counter in [
        &GENESIS_MINTED,
        &BLOCK_RECEIPTS_MINTED,
        &EXACT_RECEIPTS_MINTED,
        &PRESERVING_RECEIPTS_MINTED,
        &ADDITIVE_RECEIPTS_MINTED,
        &OWNER_RECEIPT_REFUSALS,
        &LINEAGES_STARTED,
        &RECEIPTS_COMMITTED,
        &MASTER_ADOPTIONS,
    ] {
        counter.store(0, Ordering::Relaxed);
    }
    *FINAL_MASTER_LINEAGE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
    *MASTER_ARTIFACT_WITNESS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
}

/// Records the projection of the released Master lineage for the report.
pub(crate) fn record_final_master_lineage(lineage: &MasterLineage, current_xml: &str) {
    let identities = lineage.events.iter().map(|event| event.identity).collect::<Vec<_>>();
    crate::ssc_evidence_proof::retain_released_master_evidence(&SSC_ORIGIN_ACCESS, &identities);
    let summary = lineage.summary(current_xml);
    *FINAL_MASTER_LINEAGE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(summary);
}

/// Records the byte-exact artifact readback witness of the released Master.
pub(crate) fn record_master_artifact_witness(artifact_matches_sovereign_state: bool) {
    *MASTER_ARTIFACT_WITNESS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(MasterArtifactWitness {
        artifact_matches_sovereign_state,
    });
}

fn format_row_counts(rows: &BTreeMap<usize, usize>) -> String {
    if rows.is_empty() {
        return "none".to_string();
    }
    rows.iter()
        .map(|(row, count)| format!("#{row}x{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_contract_counts(contracts: &BTreeMap<&'static str, usize>) -> String {
    if contracts.is_empty() {
        return "none".to_string();
    }
    contracts
        .iter()
        .map(|(contract, count)| format!("{contract}x{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

/// Appends the Master lineage projection. Counts only; no identity value, no
/// authority.
pub(crate) fn append_master_lineage_report(out: &mut String) {
    let load = |counter: &AtomicUsize| counter.load(Ordering::Relaxed);
    out.push_str("MasterLineageContract: identity is minted at the creation or transformation event as an opaque process-local value and is never derived from XML, fingerprints, locators, profiles or musical coordinates; fingerprints bind continuity only; minting is sealed to owner capability types bound to frozen population descriptors or frozen PRESERVING/ADDITIVE exclusion contracts; a receipt proves one owner event and is domain-neutral until a Master adoption consumes it; unadopted receipts are move-only; a PRESERVING receipt exists only when the conserved content projection (parts, measures, pitch or rest, duration, chord, grace, cue, voice, staff, ties, visibility, backup, forward, divisions, key, time, barlines) is identical before and after; an ADDITIVE receipt exists only for byte-exact insertions of the frozen element class that keep the conserved projection as a subsequence; there is no untrusted gap element: a Master transition without complete continuous receipt coverage is a fatal sovereignty violation; lineage is provenance, never write authority.\n");
    out.push_str(&format!(
        "MasterLineageMints: genesis {}; block_receipts {}; exact_receipts {}; preserving_receipts {}; additive_receipts {}; owner_receipt_refusals_all_domains {}; receipts_committed_to_master {}; master_adoptions {}.\n",
        load(&GENESIS_MINTED),
        load(&BLOCK_RECEIPTS_MINTED),
        load(&EXACT_RECEIPTS_MINTED),
        load(&PRESERVING_RECEIPTS_MINTED),
        load(&ADDITIVE_RECEIPTS_MINTED),
        load(&OWNER_RECEIPT_REFUSALS),
        load(&RECEIPTS_COMMITTED),
        load(&MASTER_ADOPTIONS),
    ));
    out.push_str(&format!(
        "MasterLineageCandidates: lineages_started_with_valid_genesis {} (MIDI-only candidates and arrangement variants); candidates_without_genesis 0 (type-enforced: MasterAdoptedState is constructible only from a minted GENESIS lineage whose head binds the adopted document).\n",
        load(&LINEAGES_STARTED),
    ));
    let slot = FINAL_MASTER_LINEAGE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let witness = *MASTER_ARTIFACT_WITNESS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match slot.as_ref() {
        Some(summary) => {
            out.push_str(&format!(
                "MasterLineageReleased: genesis #{} {}; block_receipts {} ({}); block_spans_bound {}; exact_receipts {} ({}); exact_edits {}; preserving_receipts {} ({}); additive_receipts {} ({}); additive_insertions {}; continuity {}.\n",
                summary.genesis_row,
                summary.genesis_owner,
                summary.block_receipts,
                format_row_counts(&summary.block_rows),
                summary.block_spans_bound,
                summary.exact_receipts,
                format_row_counts(&summary.exact_rows),
                summary.exact_edits,
                summary.preserving_receipts,
                format_contract_counts(&summary.preserving_contracts),
                summary.additive_receipts,
                format_contract_counts(&summary.additive_contracts),
                summary.additive_insertions,
                if summary.continuity_holds { "holds" } else { "violated" },
            ));
            let continuity_violations = usize::from(!summary.continuity_holds);
            let artifact_mismatch = usize::from(
                witness.is_none_or(|witness| !witness.artifact_matches_sovereign_state),
            );
            let closed = continuity_violations == 0 && artifact_mismatch == 0;
            out.push_str(&format!(
                "MasterSovereigntyClosure: MasterSovereigntyClosed: {closed}; MasterLineagePending: 0 (no gap element exists); AdoptionWithoutGenesis: 0 (type-enforced); ContentChangingMasterAdoptionWithoutIdentity: 0 (fatal at the adoption boundary); MasterContinuityViolations: {continuity_violations}; MasterReceiptCoverageFailures: 0 (fatal at the adoption boundary); MasterOwnerWriteWithoutReceipt: 0 (fatal at the adoption boundary); MasterCandidateFallbackAfterReceiptFailure: 0 (no fallback path: violation aborts the run); MasterArtifactReadbackMismatch: {artifact_mismatch}; LegacyMasterReloadCount: 0 (reload path removed).\n"
            ));
            if closed {
                out.push_str("MASTER SOVEREIGNTY CLOSED\n");
            }
        }
        None => out.push_str("MasterLineageReleased: not recorded.\n"),
    }
}




const E032_FRAGMENT_ADDITIVE: AdditiveDescriptor = AdditiveDescriptor {
    exclusion_id: "E032",
    owner: "src/structural_write_authority/thirty_surface_productive_authority.rs::add_tie_markup_authorized",
    owner_signature: "fn add_tie_markup_authorized(source: &str, start: bool, stop: bool) -> String",
    insertion_class: AdditiveInsertionClass::TieFragment,
};
impl AdditiveTransformationReceipt {
    pub(crate) fn mint_tie_fragment_receipt(
        _owner: crate::structural_write_authority::TieFragmentAdditiveMint,
        pre_xml: &str,
        post_xml: &str,
        insertions: Vec<ExactEdit>,
    ) -> Option<Self> {
        if pre_xml == post_xml {
            return None;
        }
        let mut insertions = insertions;
        insertions.sort_by_key(|edit| (edit.start, edit.end));
        if !fragment_edits_match(AdditiveInsertionClass::TieFragment, pre_xml, &insertions)
            || !exact_edits_reproduce(pre_xml, post_xml, &insertions)
        {
            count_owner_receipt_refusal();
            return None;
        }
        count(&ADDITIVE_RECEIPTS_MINTED);
        let receipt = Self {
            identity: MintedIdentity::mint_fresh(),
            descriptor: &E032_FRAGMENT_ADDITIVE,
            insertions: insertions.len(),
            pre_fingerprint: fingerprint(pre_xml),
            post_fingerprint: fingerprint(post_xml),
        };
        assert_eq!(
            realize_bound_additive_fragment_receipt(
                &receipt,
                receipt.descriptor.insertion_class,
                pre_xml,
                &insertions
            )
            .as_deref(),
            Some(post_xml),
            "fragment receipt replay failed"
        );
        ssc_capture_fragment_origin(&receipt, pre_xml, post_xml);
        Some(receipt)
    }
}

// Dropping an unadopted receipt drops only its private observational snapshot.
impl Drop for AdditiveTransformationReceipt {
    fn drop(&mut self) {
        crate::ssc_evidence_proof::release_unadopted_additive_origin(&SSC_ORIGIN_ACCESS, &self.identity);
    }
}

// Private observation beside the frozen ADDITIVE mint/verifier. Never mints an identity.
pub(crate) fn ssc_additive_identity(receipt: &AdditiveTransformationReceipt) -> &MintedIdentity { &receipt.identity }
pub(crate) fn capture_additive_origin<O: AdditiveMintOwner>(
    _owner: O, receipt: &Option<AdditiveTransformationReceipt>, pre: &str, post: &str,
) {
    if let Some(receipt) = receipt.as_ref().filter(|receipt| receipt.descriptor == O::DESCRIPTOR && receipt.pre_fingerprint == fingerprint(pre) && receipt.post_fingerprint == fingerprint(post)) {
        let scope = if O::DESCRIPTOR == &FINAL_BARLINE_ADDITIVE { crate::ssc_evidence_proof::SscOriginScope::FinalBarline } else if O::DESCRIPTOR == &DIRECTION_INSERTION_ADDITIVE { crate::ssc_evidence_proof::SscOriginScope::DirectionInsertion } else { crate::ssc_evidence_proof::SscOriginScope::Unproved };
        let origin = crate::ssc_evidence_proof::SscOriginState::capture(&SSC_ORIGIN_ACCESS, pre, post, scope);
        crate::ssc_evidence_proof::register_additive_origin(&SSC_ORIGIN_ACCESS, &receipt.identity, origin);
    }
}
// SSC scope of a PRESERVING owner, bound at its sealed mint: the owner's own
// conserved projection and, only for the frozen beam exclusions, the
// beam-chain repair necessity contract. Every other exclusion stays unbound.
fn ssc_preserving_origin<O: PreservingMintOwner>() -> crate::ssc_evidence_proof::SscPreservingOrigin {
    use crate::ssc_evidence_proof::SscPreservingNecessity;
    let beam_owners = [
        &COMMITTED_CROSS_STAFF_BEAM_PRESERVING,
        &STRUCTURAL_REST_BRIDGE_VARIANT_A_PRESERVING,
        &NOTATION_POLISH_PRESERVING,
        &PARTIAL_BEAT_BEAM_PRESERVING,
    ];
    let necessity = if beam_owners.iter().any(|descriptor| **descriptor == *O::DESCRIPTOR) {
        SscPreservingNecessity::BeamChainRepair
    } else {
        SscPreservingNecessity::Unbound
    };
    crate::ssc_evidence_proof::SscPreservingOrigin::bind(&SSC_ORIGIN_ACCESS, O::DESCRIPTOR, O::conserved_projection, necessity)
}
pub(crate) fn ssc_preserving_descriptor(receipt: &PreservingTransformationReceipt) -> &'static PreservingDescriptor {
    receipt.descriptor
}
pub(crate) fn ssc_additive_insertion_class(receipt: &AdditiveTransformationReceipt) -> AdditiveInsertionClass {
    receipt.descriptor.insertion_class
}
fn ssc_capture_fragment_origin(receipt: &AdditiveTransformationReceipt, pre: &str, post: &str) {
    if receipt.pre_fingerprint != fingerprint(pre) || receipt.post_fingerprint != fingerprint(post) {
        return;
    }
    let origin = crate::ssc_evidence_proof::SscOriginState::capture(&SSC_ORIGIN_ACCESS, pre, post, crate::ssc_evidence_proof::SscOriginScope::Unproved);
    crate::ssc_evidence_proof::register_additive_origin(&SSC_ORIGIN_ACCESS, &receipt.identity, origin);
}

// Read-only proof and replay from the owner's original prebound script. These
// helpers do not mint identities and add no generic ADDITIVE construction path.
fn fragment_edits_match(class: AdditiveInsertionClass, pre: &str, edits: &[ExactEdit]) -> bool {
    if edits.iter().any(|edit| edit.start != edit.end) {
        return false;
    }
    match class {
        AdditiveInsertionClass::TieFragment => [false, true].into_iter().any(|start| {
            [false, true].into_iter().any(|stop| {
                crate::structural_write_authority::bound_tie_fragment_insertions(pre, start, stop)
                    == edits
            })
        }),
        _ => false,
    }
}
pub(crate) fn realize_bound_additive_fragment_receipt(
    receipt: &AdditiveTransformationReceipt,
    class: AdditiveInsertionClass,
    pre: &str,
    edits: &[ExactEdit],
) -> Option<String> {
    if receipt.descriptor.insertion_class != class
        || receipt.pre_fingerprint != fingerprint(pre)
        || receipt.insertions != edits.len()
        || !fragment_edits_match(class, pre, edits)
    {
        return None;
    }
    let post = realize_edit_script(pre, edits)?;
    (receipt.post_fingerprint == fingerprint(&post)).then_some(post)
}

pub(crate) fn ssc_block_origin_fingerprints(receipt: &BlockTransformationReceipt, pre: &str, post: &str) -> (bool, bool) {
    (receipt.pre_fingerprint == fingerprint(pre), receipt.post_fingerprint == fingerprint(post))
}

pub(crate) fn ssc_exact_origin_fingerprints(receipt: &ExactTransformationReceipt, pre: &str, post: &str) -> (bool, bool) {
    (receipt.pre_fingerprint == fingerprint(pre), receipt.post_fingerprint == fingerprint(post))
}

pub(crate) fn ssc_preserving_origin_fingerprints(receipt: &PreservingTransformationReceipt, pre: &str, post: &str) -> (bool, bool) {
    (receipt.pre_fingerprint == fingerprint(pre), receipt.post_fingerprint == fingerprint(post))
}

pub(crate) fn ssc_additive_origin_fingerprints(receipt: &AdditiveTransformationReceipt, pre: &str, post: &str) -> (bool, bool) {
    (receipt.pre_fingerprint == fingerprint(pre), receipt.post_fingerprint == fingerprint(post))
}

// SSC v4 Take 3: read-only access to the original sealed owner contracts.
pub(crate) fn ssc_exact_descriptor(receipt: &ExactTransformationReceipt) -> &'static TransformationDescriptor { receipt.descriptor }
pub(crate) fn ssc_exact_descriptor_name(descriptor: &TransformationDescriptor) -> Option<&'static str> {
    [
        (&CURSOR_NEUTRAL_RENDER_EXACT, "CURSOR_NEUTRAL_RENDER_EXACT"),
        (&SAME_LANE_RAW_SPLICE_EXACT, "SAME_LANE_RAW_SPLICE_EXACT"),
        (&MISSING_REST_RENDER_EXACT, "MISSING_REST_RENDER_EXACT"),
        (&DIRECTION_RELOCATION_EXACT, "DIRECTION_RELOCATION_EXACT"),
        (&SAFE_TRANSFER_EXACT, "SAFE_TRANSFER_EXACT"),
        (&STAFF_VOICE_MOVE_EXACT, "STAFF_VOICE_MOVE_EXACT"),
        (&RESTIFY_PATCH_EXACT, "RESTIFY_PATCH_EXACT"),
        (&VOICE_OVERFULL_CROP_EXACT, "VOICE_OVERFULL_CROP_EXACT"),
        (&RESTIFICATION_BATCH_EXACT, "RESTIFICATION_BATCH_EXACT"),
        (&STAFF_VOICE_SCALAR_BATCH_EXACT, "STAFF_VOICE_SCALAR_BATCH_EXACT"),
        (&EDITORIAL_VOICE_COMPLETE_REWRITE_EXACT, "EDITORIAL_VOICE_COMPLETE_REWRITE_EXACT"),
        (&NOTE_VISIBILITY_EXACT, "NOTE_VISIBILITY_EXACT"),
        (&ALLOWED_RANGE_REMOVAL_EXACT, "ALLOWED_RANGE_REMOVAL_EXACT"),
        (&HARMONIC_PITCH_SWAP_EXACT, "HARMONIC_PITCH_SWAP_EXACT"),
        (&HIDDEN_ATTRIBUTE_REMOVAL_EXACT, "HIDDEN_ATTRIBUTE_REMOVAL_EXACT"),
        (&CHORD_GESTURE_DISTRIBUTION_EXACT, "CHORD_GESTURE_DISTRIBUTION_EXACT"),
        (&VISIBILITY_STAFF_DISTRIBUTION_EXACT, "VISIBILITY_STAFF_DISTRIBUTION_EXACT"),
        (&MASTER_EXPORT_FIFTHS_EXACT, "MASTER_EXPORT_FIFTHS_EXACT"),
        (&FUNCTIONAL_RESPELLING_EXACT, "FUNCTIONAL_RESPELLING_EXACT"),
        (&FINAL_CHORD_ENCODING_EXACT, "FINAL_CHORD_ENCODING_EXACT"),
        (&RENDER_ROW_22_EXACT, "RENDER_ROW_22_EXACT"),
        (&RENDER_ROW_30_EXACT, "RENDER_ROW_30_EXACT"),
        (&RENDER_ROW_31_EXACT, "RENDER_ROW_31_EXACT"),
        (&RENDER_ROW_32_EXACT, "RENDER_ROW_32_EXACT"),
        (&RENDER_ROW_33_EXACT, "RENDER_ROW_33_EXACT"),
        (&RENDER_ROW_34_EXACT, "RENDER_ROW_34_EXACT"),
        (&RENDER_ROW_35_EXACT, "RENDER_ROW_35_EXACT"),
        (&RENDER_ROW_36_EXACT, "RENDER_ROW_36_EXACT"),
        (&RENDER_ROW_37_EXACT, "RENDER_ROW_37_EXACT"),
        (&RENDER_ROW_38_EXACT, "RENDER_ROW_38_EXACT"),
        (&RENDER_ROW_41_EXACT, "RENDER_ROW_41_EXACT"),
        (&RENDER_ROW_42_EXACT, "RENDER_ROW_42_EXACT"),
        (&RENDER_ROW_43_EXACT, "RENDER_ROW_43_EXACT"),
        (&RENDER_ROW_44_EXACT, "RENDER_ROW_44_EXACT"),
        (&RENDER_ROW_45_EXACT, "RENDER_ROW_45_EXACT"),
        (&RENDER_ROW_46_EXACT, "RENDER_ROW_46_EXACT"),
        (&RENDER_ROW_47_EXACT, "RENDER_ROW_47_EXACT"),
        (&RENDER_ROW_48_EXACT, "RENDER_ROW_48_EXACT"),
        (&RENDER_ROW_51_EXACT, "RENDER_ROW_51_EXACT"),
        (&RENDER_ROW_58_EXACT, "RENDER_ROW_58_EXACT"),
        (&RENDER_ROW_64_EXACT, "RENDER_ROW_64_EXACT"),
    ].into_iter().find(|(frozen, _)| frozen.same_contract(descriptor)).map(|(_, name)| name)
}
pub(crate) fn ssc_block_descriptor(receipt: &BlockTransformationReceipt) -> &'static TransformationDescriptor { receipt.descriptor }
pub(crate) fn ssc_block_descriptor_name(descriptor: &TransformationDescriptor) -> Option<&'static str> {
    [
        (&VISIBLE_RHYTHM_BLOCK, "VISIBLE_RHYTHM_BLOCK"),
        (&BEAT_SCOPED_BEAMING_BLOCK, "BEAT_SCOPED_BEAMING_BLOCK"),
        (&CLEF_MEASURE_BLOCK, "CLEF_MEASURE_BLOCK"),
        (&OCTAVE_MEASURE_BLOCK, "OCTAVE_MEASURE_BLOCK"),
        (&RENDER_ROW_7_BLOCK, "RENDER_ROW_7_BLOCK"),
        (&RENDER_ROW_17_BLOCK, "RENDER_ROW_17_BLOCK"),
        (&RENDER_ROW_19_BLOCK, "RENDER_ROW_19_BLOCK"),
        (&RENDER_ROW_20_BLOCK, "RENDER_ROW_20_BLOCK"),
        (&RENDER_ROW_21_BLOCK, "RENDER_ROW_21_BLOCK"),
        (&RENDER_ROW_54_BLOCK, "RENDER_ROW_54_BLOCK"),
        (&RENDER_ROW_55_BLOCK, "RENDER_ROW_55_BLOCK"),
        (&RENDER_ROW_57_BLOCK, "RENDER_ROW_57_BLOCK"),
        (&RENDER_ROW_61_BLOCK, "RENDER_ROW_61_BLOCK"),
        (&RENDER_ROW_66_BLOCK, "RENDER_ROW_66_BLOCK"),
    ].into_iter().find(|(frozen, _)| frozen.same_contract(descriptor)).map(|(_, name)| name)
}
pub(crate) fn ssc_block_bound_span(receipt: &BlockTransformationReceipt) -> Option<(usize, usize)> { receipt.bound_span }
pub(crate) fn ssc_exact_edit_spans(receipt: &ExactTransformationReceipt) -> Vec<(usize, usize, &str)> { receipt.edits.iter().map(|edit| (edit.start, edit.end, edit.replacement.as_str())).collect() }

mod render_lineage;

mod original_respelling_chains;
pub(crate) use original_respelling_chains::OriginalFunctionalRespellingChains;

fn final_chord_row_seal(rows: &[FinalChordCourseRow]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    rows.hash(&mut hasher);
    hasher.finish()
}

impl FinalChordCourseCapture {
    pub(crate) fn recorded(_: &crate::structural_write_authority::FinalChordEncodingExactMint,
        receipt: &ExactTransformationReceipt, before: &str, after: &str,
        rows: Vec<FinalChordCourseRow>) -> anyhow::Result<Self> {
        let capture = Self { execution: *receipt.ssc_bound_origin().0,
            before: before.to_owned(), after: after.to_owned(), row_seal: final_chord_row_seal(&rows), rows };
        capture.validate_execution(receipt, before)?;
        Ok(capture)
    }
    pub(crate) fn validate_execution(&self, receipt: &ExactTransformationReceipt,
        before: &str) -> anyhow::Result<&str> {
        ensure!(receipt.minted_by::<crate::structural_write_authority::FinalChordEncodingExactMint>()
            && *receipt.ssc_bound_origin().0 == self.execution, "Foreign original FinalChord execution");
        ensure!(final_chord_row_seal(&self.rows) == self.row_seal, "Original FinalChord emission map changed");
        ensure!(before == self.before && receipt.operation_readback(before)?.post_document() == self.after,
            "Foreign original FinalChord endpoints");
        Ok(&self.after)
    }
    pub(crate) fn rows(&self) -> &[FinalChordCourseRow] { &self.rows }
    #[cfg(test)]
    pub(crate) fn altered_rows_for_test(&self, rows: Vec<FinalChordCourseRow>) -> Self {
        let mut copy = self.clone(); copy.rows = rows; copy
    }

}


/// Original bytes of the two frozen note-free ADDITIVE owners. No new mint,
/// verifier, receipt or adoption authority is introduced by this observation.
pub(crate) fn additive_original_documents(receipt: &AdditiveTransformationReceipt) -> anyhow::Result<(String, String)> {
    ensure!(receipt.descriptor == &FINAL_BARLINE_ADDITIVE
        || receipt.descriptor == &DIRECTION_INSERTION_ADDITIVE,
        "Unsupported additive course continuation owner");
    let (pre, post) = crate::ssc_evidence_proof::read_additive_origin(&SSC_ORIGIN_ACCESS, receipt)
        .ok_or_else(|| anyhow::anyhow!("Missing original additive capture"))?;
    ensure!(receipt.pre_fingerprint == fingerprint(&pre)
        && receipt.post_fingerprint == fingerprint(&post), "Original additive capture binding mismatch");
    Ok((pre, post))
}

impl ExactOperationReadback<'_> {
    /// Notation-only endpoint cleanup inside one carried occurrence: every
    /// original edit touching `range` lies inside it and deletes only `<tied>`
    /// or `<slur>` endpoint tags or an empty `<notations>` wrapper. A sounding
    /// `<tie>` never qualifies. Reads the original edit script; never XML geometry.
    pub(crate) fn notation_endpoint_cleanup_within(&self, range: &std::ops::Range<usize>) -> bool {
        let mut inside = self.edits.iter()
            .filter(|edit| edit.start < range.end && edit.end > range.start).peekable();
        inside.peek().is_some() && inside.all(|edit| edit.start >= range.start && edit.end <= range.end
            && edit.replacement.is_empty()
            && self.pre_document.get(edit.start..edit.end).is_some_and(is_notation_endpoint_fragment))
    }
}

fn is_notation_endpoint_fragment(fragment: &str) -> bool {
    let wrapped = format!("<cleanup>{fragment}</cleanup>");
    let Ok(doc) = roxmltree::Document::parse(&wrapped) else { return false; };
    let root = doc.root_element();
    root.children().any(|node| node.is_element()) && root.children().all(|node| {
        if node.is_text() { return node.text().is_some_and(|text| text.trim().is_empty()); }
        if !node.is_element() || node.tag_name().namespace().is_some() { return false; }
        match node.tag_name().name() {
            "tied" | "slur" => node.children().next().is_none()
                && wrapped[node.range()].ends_with("/>"),
            "notations" => node.attributes().len() == 0
                && node.children().all(|child| child.is_text()
                    && child.text().is_some_and(|text| text.trim().is_empty())),
            _ => false,
        }
    })
}

pub(crate) use render_lineage::{
    append_render_lineage_report, count_render_closure_failure, count_render_declined_sink_write,
    count_render_unchanged_sink_write, record_final_render_lineage, record_render_artifact_read,
    record_render_stage_boundary, render_sovereignty_violation, reset_render_lineage_measurement,
    RenderArtifactReadPurpose, RenderClosureMetric, RenderClosureVerdict, RenderDiscardReason,
    RenderLineage, RenderLineagePosition, RenderMasterEpoch, RenderMasterEpochBinding,
    RenderOriginFork, RenderOriginId, RenderSovereignIdentity, RenderStageBoundary,
    RenderTransitionBasis, RenderTransitionReceipts,
    VerifiedRenderTransition,
};

#[cfg(test)]
pub(crate) mod tests;

impl OwnerReceipts {
    /// Read-only custody census of one Master transition for stderr. Never read
    /// by the lineage commit, an owner or any later stage; failures stay text.
    /// Returns `None` for a transition whose notes changed only in text and whose
    /// receipts are all kinds the course continuation already carries.
    pub(crate) fn custody_census_line(&self, before: &str, after: &str, transition: &str) -> Option<String> {
        let owners = self.receipts.iter().map(|receipt| {
            let (kind, row, owner, births, staff) = match receipt {
                TransformationReceipt::Exact(r) => ("EXACT", r.descriptor.population_report_row,
                    r.descriptor.owner, r.physical_births.is_some(), r.staff_transition.is_some()),
                TransformationReceipt::Block(r) => ("BLOCK", r.descriptor.population_report_row,
                    r.descriptor.owner, self.physical_block_births.iter().any(|(id, _)| *id == r.identity), false),
                TransformationReceipt::Preserving(r) => ("PRESERVING", 0, r.descriptor.owner, false, false),
                TransformationReceipt::Additive(r) => ("ADDITIVE", 0, r.descriptor.owner, false, false),
            };
            format!("{kind}:{row}:{owner}:births={births}:staff_capture={staff}")
        }).collect::<Vec<_>>();
        let uncarried_kind = self.receipts.iter().any(|receipt| !self.census_kind_is_carried(receipt));
        let (counts, physical_change) = match crate::arrangement::OwnedRhythmicContinuation::custody_census(before, after) {
            Ok(c) => (format!("unchanged={}; text_only={}; staff_moved={}; disposition_changed={}; removed={}; added={}",
                c.unchanged, c.text_only, c.staff_moved, c.disposition_changed, c.removed, c.added),
                c.staff_moved + c.disposition_changed + c.removed + c.added > 0),
            Err(error) => (format!("inventory_unreadable={:?}", error.to_string()), true),
        };
        (physical_change || uncarried_kind).then(|| format!(
            "ETUSCHLOPF: CustodyCensus: transition={transition:?}; receipts={}; {counts}; owners={owners:?}; projection_only=true",
            self.receipts.len()))
    }

    /// Receipt kinds the existing course continuation already carries when the
    /// notes change only in text: EXACT, BLOCK with its own birth capture and the
    /// three PRESERVING owners with a representation transport.
    fn census_kind_is_carried(&self, receipt: &TransformationReceipt) -> bool {
        match receipt {
            TransformationReceipt::Exact(_) => true,
            TransformationReceipt::Block(r) => self.physical_block_births.iter().any(|(id, _)| *id == r.identity),
            TransformationReceipt::Preserving(r) => [NOTATION_POLISH_PRESERVING.owner,
                NOTATION_CLEF_ISLAND_PRESERVING.owner, PIANIST_LAYOUT_PRESERVING.owner].contains(&r.descriptor.owner),
            TransformationReceipt::Additive(_) => false,
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/notation_endpoint_cleanup_readback.rs"]
mod notation_endpoint_cleanup_readback_tests;


impl OwnerReceipts {
    /// Read-only companion continuation, evaluated before the unchanged sovereign
    /// commit and installed only after that commit returns successfully.
    pub(crate) fn observe_actual_beam_continuation(&self,
        previous: Option<&Result<std::sync::Arc<crate::pipeline::ActualCanonicalBeamCapture>, String>>,
    ) -> Option<Result<std::sync::Arc<crate::pipeline::ActualCanonicalBeamCapture>, String>> {
        let mut observed = previous.cloned();
        for receipt in &self.receipts {
            if let Some((_, capture)) = self.actual_beam_captures.iter().find(|(identity, _)| *identity == receipt.identity()) {
                observed = Some(capture.clone());
                continue;
            }
            let Some(Ok(capture)) = observed.as_ref() else { continue; };
            let continued = match receipt {
                TransformationReceipt::Exact(receipt) => capture.carry_exact(receipt),
                TransformationReceipt::Preserving(receipt) => receipt.recorded_representation()
                    .ok_or_else(|| anyhow::anyhow!("Missing original preserving operation journal"))
                    .and_then(|transport| capture.carry_recorded(transport)),
                TransformationReceipt::Block(receipt) => self.recorded_block_transports.iter()
                    .find(|(identity, _)| *identity == receipt.identity)
                    .ok_or_else(|| anyhow::anyhow!("Missing original block operation journal"))
                    .and_then(|(_, transport)| transport.as_ref().map_err(|error| anyhow::anyhow!("{error}")))
                    .and_then(|transport| capture.carry_recorded(transport)),
                TransformationReceipt::Additive(_) => Err(anyhow::anyhow!("No original insertion journal carried for beam continuation")),
            };
            observed = Some(continued.map(std::sync::Arc::new).map_err(|error| error.to_string()));
        }
        observed
    }
}
