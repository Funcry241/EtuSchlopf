use super::*;

#[test]
fn coverage_diagnostic_is_silent_for_unchanged_and_covered_transitions() {
    let pre = "<d>0</d>";
    let post = "<d>1</d>";
    assert!(OwnerReceipts::default().coverage_diagnostic(pre, pre).is_none());
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(Some(synthetic_exact_receipt_for_tests(pre, post, 3, 4, "1")));
    assert!(receipts.coverage_diagnostic(pre, post).is_none());
    assert_eq!(receipts.len(), 1);
    let missing = OwnerReceipts::default();
    let first = missing.coverage_diagnostic(pre, post);
    assert!(first.is_some());
    assert_eq!(first, missing.coverage_diagnostic(pre, post));
    assert!(OwnerReceipts::default().coverage_diagnostic("<x/>", "<x/>").is_none());
    assert_eq!(first, missing.coverage_diagnostic(pre, post));
    assert!(receipts.coverage_diagnostic(pre, post).is_none());
}

#[test]
fn coverage_diagnostic_distinguishes_empty_and_partial_coverage() {
    let pre = "<d>0</d>";
    let middle = "<d>1</d>";
    let post = "<d>2</d>";
    let empty = OwnerReceipts::default().coverage_diagnostic(pre, post).expect("missing coverage");
    assert!(empty.contains("receipt_count=0"));
    assert!(empty.contains("last_owner=None"));
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(Some(synthetic_exact_receipt_for_tests(pre, middle, 3, 4, "1")));
    let partial = receipts.coverage_diagnostic(pre, post).expect("uncovered suffix");
    assert!(partial.contains("receipt_count=1"));
    assert!(partial.contains(&format!("receipt_end_fp={:016x}", fingerprint(middle))));
    assert!(partial.contains(&format!("target_fp={:016x}", fingerprint(post))));
    assert!(partial.contains("first_discontinuous_receipt=None"));
    assert!(partial.contains(TEST_EXACT.owner));
}

#[test]
fn coverage_diagnostic_reports_discontinuity_even_when_the_tail_matches() {
    let pre = "<d>0</d>";
    let foreign = "<d>9</d>";
    let post = "<d>1</d>";
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(Some(synthetic_exact_receipt_for_tests(foreign, post, 3, 4, "1")));
    let diagnostic = receipts.coverage_diagnostic(pre, post).expect("foreign pre-state");
    assert!(diagnostic.contains("first_discontinuous_receipt=Some(0)"));
    assert_eq!(receipts.len(), 1);
}

const TEST_GENESIS: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 0,
    owner: "src/test_owner.rs::create_document",
    owner_signature: "fn create_document(source: &str) -> String",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Genesis,
};
const TEST_BLOCK: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 0,
    owner: "src/test_owner.rs::rebuild_document",
    owner_signature: "fn rebuild_document(xml: &mut String)",
    effect: EffectContract::Destructive,
    identity: IdentityContract::BlockDocument,
};
const TEST_EXACT: TransformationDescriptor = TransformationDescriptor {
    population_report_row: 0,
    owner: "src/test_owner.rs::edit_document",
    owner_signature: "fn edit_document(xml: &str) -> String",
    effect: EffectContract::Destructive,
    identity: IdentityContract::Exact,
};

struct TestGenesisOwner;
impl sealed::MintOwner for TestGenesisOwner {}
impl GenesisMintOwner for TestGenesisOwner {
    const DESCRIPTOR: &'static TransformationDescriptor = &TEST_GENESIS;
}
struct OtherGenesisOwner;
impl sealed::MintOwner for OtherGenesisOwner {}
impl GenesisMintOwner for OtherGenesisOwner {
    const DESCRIPTOR: &'static TransformationDescriptor = &TEST_BLOCK;
}
struct TestBlockOwner;
impl sealed::MintOwner for TestBlockOwner {}
impl BlockMintOwner for TestBlockOwner {
    const DESCRIPTOR: &'static TransformationDescriptor = &TEST_BLOCK;
}
struct TestExactOwner;
impl sealed::MintOwner for TestExactOwner {}
impl ExactMintOwner for TestExactOwner {
    const DESCRIPTOR: &'static TransformationDescriptor = &TEST_EXACT;
}
const TEST_PRESERVING: PreservingDescriptor = PreservingDescriptor {
    exclusion_id: "E000",
    owner: "src/test_owner.rs::present_document",
    owner_signature: "fn present_document(xml: &str) -> String",
};
struct TestPreservingOwner;
impl sealed::MintOwner for TestPreservingOwner {}
impl PreservingMintOwner for TestPreservingOwner {
    const DESCRIPTOR: &'static PreservingDescriptor = &TEST_PRESERVING;
}

struct MislabelledExactOwner;
impl sealed::MintOwner for MislabelledExactOwner {}
impl ExactMintOwner for MislabelledExactOwner {
    const DESCRIPTOR: &'static TransformationDescriptor = &TEST_BLOCK;
}


/// Synthetic GENESIS for tests of other modules (test owner, test descriptor).
pub(crate) fn synthetic_genesis_for_tests(post: &str) -> MasterGenesisIdentity {
    genesis(post)
}

/// Synthetic EXACT receipt of one bound edit for tests of other modules.
pub(crate) fn synthetic_exact_receipt_for_tests(
    pre: &str,
    post: &str,
    start: usize,
    end: usize,
    replacement: &str,
) -> ExactTransformationReceipt {
    ExactTransformationReceipt::mint_exact_receipt(
        TestExactOwner,
        pre,
        post,
        vec![ExactEdit::bound(start, end, replacement.to_string())],
    )
    .expect("synthetic exact receipt reproduces its post state")
}

/// r1287 (contract v4 A7): lineage tests commit through the enforced adoption verdict. This
/// EXACT receipt is minted under the frozen HARMONIC_PITCH_SWAP_EXACT descriptor; its single
/// edit is the differing byte range, which must lie inside a bound `<pitch>` element so that
/// the envelope row admits it.
pub(crate) fn permitted_exact_receipt_for_tests(pre: &str, post: &str) -> ExactTransformationReceipt {
    let start = pre.bytes().zip(post.bytes()).take_while(|(a, b)| a == b).count();
    let suffix = pre[start..].bytes().rev().zip(post[start..].bytes().rev()).take_while(|(a, b)| a == b).count();
    ExactTransformationReceipt::mint_exact_receipt(
        ProbeHarmonicPitchSwap,
        pre,
        post,
        vec![ExactEdit::bound(start, pre.len() - suffix, post[start..post.len() - suffix].to_string())],
    )
    .expect("permitted exact receipt reproduces its post state")
}

/// Synthetic path-bound PRESERVING receipt for tests of other modules.
pub(crate) fn synthetic_file_bound_preserving_receipt_for_tests(
    pre: &str,
    post: String,
) -> Option<PreservingTransformationReceipt> {
    PreservingTransformationReceipt::mint_file_bound_preserving_receipt(
        ProbePianistLayout,
        pre,
        post,
    )
}

/// Synthetic score for conservation tests of other modules.
pub(crate) fn synthetic_score_for_tests(measure_body: &str) -> String {
    score(measure_body)
}

fn genesis(post: &str) -> MasterGenesisIdentity {
    MasterGenesisIdentity::mint_genesis(
        TestGenesisOwner,
        "profile-a",
        GenesisInputBinding::input_document("<source/>"),
        post,
    )
}

#[test]
fn identities_are_minted_not_derived_from_content() {
    let first = genesis("<doc>same</doc>");
    let second = genesis("<doc>same</doc>");
    assert_ne!(first.identity, second.identity);
    let cloned = first.clone();
    assert_eq!(cloned.identity, first.identity);
    assert_eq!(format!("{:?}", first.identity), "MintedIdentity(opaque)");
    assert!(format!("{first:?}").contains("identity: MintedIdentity(opaque)"));
}

#[test]
fn genesis_validation_checks_bindings_without_recomputing_identity() {
    let created = genesis("<doc>a</doc>");
    let input = GenesisInputBinding::input_document("<source/>");
    assert!(created
        .validate_genesis_binding::<TestGenesisOwner>("profile-a", &input, "<doc>a</doc>")
        .is_ok());
    assert!(created
        .validate_genesis_binding::<TestGenesisOwner>("profile-b", &input, "<doc>a</doc>")
        .is_err());
    assert!(created
        .validate_genesis_binding::<TestGenesisOwner>("profile-a", &input, "<doc>b</doc>")
        .is_err());
    assert!(created.validate_genesis_output::<OtherGenesisOwner>("<doc>a</doc>").is_err());
}

#[test]
fn exact_receipt_requires_byte_exact_reproduction() {
    let pre = "<n><v>1</v><s>1</s></n>";
    let post = "<n><v>10</v><s>2</s></n>";
    let edits = vec![ExactEdit::bound(14, 15, "2".to_string()), ExactEdit::bound(6, 7, "10".to_string())];
    let receipt = ExactTransformationReceipt::mint_exact_receipt(TestExactOwner, pre, post, edits)
        .expect("bound edits reproduce the output");
    assert_eq!(receipt.edits.len(), 2);
    assert_eq!(receipt.edits[0].start, 6);

    let outside_change = "<m><v>10</v><s>2</s></n>";
    let edits = vec![ExactEdit::bound(6, 7, "10".to_string()), ExactEdit::bound(14, 15, "2".to_string())];
    assert!(ExactTransformationReceipt::mint_exact_receipt(TestExactOwner, pre, outside_change, edits).is_none());

    let overlapping = vec![ExactEdit::bound(3, 9, "x".to_string()), ExactEdit::bound(6, 12, "y".to_string())];
    assert!(ExactTransformationReceipt::mint_exact_receipt(TestExactOwner, pre, post, overlapping).is_none());

    assert!(ExactTransformationReceipt::mint_exact_receipt(TestExactOwner, pre, pre, Vec::new()).is_none());
}

#[test]
fn exact_receipt_refuses_ambiguous_insertions_and_mislabelled_owners() {
    let pre = "<a></a>";
    let post = "<a>xy</a>";
    let insertions = vec![ExactEdit::bound(3, 3, "x".to_string()), ExactEdit::bound(3, 3, "y".to_string())];
    assert!(ExactTransformationReceipt::mint_exact_receipt(TestExactOwner, pre, post, insertions).is_none());
    let single = vec![ExactEdit::bound(3, 3, "xy".to_string())];
    assert!(ExactTransformationReceipt::mint_exact_receipt(MislabelledExactOwner, pre, post, single.clone()).is_none());
    assert!(ExactTransformationReceipt::mint_exact_receipt(TestExactOwner, pre, post, single).is_some());
}

#[test]
fn replace_then_insert_at_range_end_is_an_exact_script() {
    let pre = "<r><n>p</n><n>q</n></r>";
    let post = "<r><n>R</n>\n<c/><n>q</n></r>";
    let edits = vec![
        ExactEdit::bound(3, 11, "<n>R</n>".to_string()),
        ExactEdit::bound(11, 11, "\n<c/>".to_string()),
    ];
    assert!(ExactTransformationReceipt::mint_exact_receipt(TestExactOwner, pre, post, edits).is_some());
}

fn score(measure_body: &str) -> String {
    format!(
        "<score-partwise><part-list/><part id=\"P1\"><measure><attributes><divisions>2</divisions><key><fifths>0</fifths></key><time><beats>2</beats><beat-type>4</beat-type></time><clef><sign>G</sign></clef></attributes>{measure_body}</measure></part></score-partwise>"
    )
}

const TWO_EIGHTHS: &str = "<note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>eighth</type><staff>1</staff></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>eighth</type><staff>1</staff></note><forward><duration>2</duration></forward>";


#[test]
fn clef_only_attributes_are_conserved_by_the_projection() {
    let pre = "<score-partwise><part id=\"P1\"><measure><attributes><divisions>4</divisions><key><fifths>0</fifths></key><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>16</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
    let post = "<score-partwise><part id=\"P1\"><measure><attributes><divisions>4</divisions><key><fifths>0</fifths></key><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef></attributes><attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>16</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
    assert_eq!(conserved_content_projection(pre), conserved_content_projection(post));

    // Divisions, key and time stay conserved, also inside an added attributes element.
    for conserved_change in [
        "<attributes><divisions>8</divisions></attributes>",
        "<attributes><key><fifths>1</fifths></key></attributes>",
        "<attributes><time><beats>3</beats><beat-type>4</beat-type></time></attributes>",
    ] {
        let changed = pre.replacen("</attributes>", &format!("</attributes>{conserved_change}"), 1);
        assert_ne!(conserved_content_projection(pre), conserved_content_projection(&changed));
    }
}

#[test]
fn preserving_receipt_admits_presentation_only_changes() {
    let pre = score(TWO_EIGHTHS);
    let beamed = score(&TWO_EIGHTHS
        .replacen("<staff>1</staff></note>", "<staff>1</staff><beam number=\"1\">begin</beam></note>", 1)
        .replacen("<staff>1</staff></note><forward>", "<staff>1</staff><beam number=\"1\">end</beam></note><forward>", 1));
    let receipt = PreservingTransformationReceipt::mint_preserving_receipt(TestPreservingOwner, &pre, &beamed)
        .expect("beam-only change is preserving");
    assert!(receipt.validate_preserving_transition::<TestPreservingOwner>(&pre, &beamed).is_ok());
    let clef_inserted = pre.replace("<attributes><divisions>", "<print new-system=\"yes\"/><attributes><divisions>");
    assert!(PreservingTransformationReceipt::mint_preserving_receipt(TestPreservingOwner, &pre, &clef_inserted).is_some());
}

#[test]
fn preserving_receipt_refuses_any_conserved_change() {
    let pre = score(TWO_EIGHTHS);
    for changed in [
        TWO_EIGHTHS.replacen("<step>C</step>", "<step>E</step>", 1),
        TWO_EIGHTHS.replacen("<duration>1</duration>", "<duration>2</duration>", 1),
        TWO_EIGHTHS.replacen("<voice>1</voice>", "<voice>2</voice>", 1),
        TWO_EIGHTHS.replacen("<staff>1</staff>", "<staff>2</staff>", 1),
        TWO_EIGHTHS.replacen("<note>", "<note print-object=\"no\">", 1),
        TWO_EIGHTHS.replacen("<forward><duration>2</duration>", "<forward><duration>1</duration>", 1),
    ] {
        let post = score(&changed);
        assert!(
            PreservingTransformationReceipt::mint_preserving_receipt(TestPreservingOwner, &pre, &post).is_none(),
            "conserved change must be refused"
        );
    }
    assert!(PreservingTransformationReceipt::mint_preserving_receipt(TestPreservingOwner, &pre, "<broken").is_none());
}

const TEST_ADDITIVE: AdditiveDescriptor = AdditiveDescriptor {
    exclusion_id: "E000",
    owner: "src/test_owner.rs::append_direction",
    owner_signature: "fn append_direction(xml: &mut String) -> usize",
    insertion_class: AdditiveInsertionClass::Direction,
};
#[derive(Clone, Copy)]
struct TestAdditiveOwner;
impl sealed::MintOwner for TestAdditiveOwner {}
impl AdditiveMintOwner for TestAdditiveOwner {
    const DESCRIPTOR: &'static AdditiveDescriptor = &TEST_ADDITIVE;
}

// ---------------------------------------------------------------------------
// Inventory-driven owner matrix: one probe per production mint binding. A
// probe carries exactly the production owner's frozen descriptor, so the
// matrix exercises every bound contract (unchanged -> None, changed without
// reproducing geometry -> refused, changed with valid geometry -> receipt).
// ---------------------------------------------------------------------------

macro_rules! descriptor_probe {
    ($probe:ident, $trait:ident, $descriptor_type:ty, $owner:path) => {
        #[derive(Clone, Copy)]
        struct $probe;
        impl sealed::MintOwner for $probe {}
        impl $trait for $probe {
            const DESCRIPTOR: &'static $descriptor_type = <$owner as $trait>::DESCRIPTOR;
        }
    };
}

descriptor_probe!(ProbeVisibleRhythm, BlockMintOwner, TransformationDescriptor, crate::arrangement::VisibleRhythmBlockMint);
descriptor_probe!(ProbeBeatScopedBeaming, BlockMintOwner, TransformationDescriptor, crate::pipeline::BeatScopedBeamingBlockMint);
descriptor_probe!(ProbeClefMeasure, BlockMintOwner, TransformationDescriptor, crate::structural_write_authority::ClefMeasureBlockMint);
descriptor_probe!(ProbeOctaveMeasure, BlockMintOwner, TransformationDescriptor, crate::structural_write_authority::OctaveMeasureBlockMint);
descriptor_probe!(ProbeSafeTransfer, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::SafeTransferExactMint);
descriptor_probe!(ProbeStaffVoiceMove, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::StaffVoiceMoveExactMint);
descriptor_probe!(ProbeRestifyPatch, ExactMintOwner, TransformationDescriptor, crate::pipeline::RestifyPatchExactMint);
descriptor_probe!(ProbeVoiceOverfullCrop, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::VoiceOverfullCropExactMint);
descriptor_probe!(ProbeRestificationBatch, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RestificationBatchExactMint);
descriptor_probe!(ProbeStaffVoiceScalarBatch, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::StaffVoiceScalarBatchExactMint);
descriptor_probe!(ProbeEditorialVoiceRewrite, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::EditorialVoiceCompleteRewriteExactMint);
descriptor_probe!(ProbeNoteVisibility, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::NoteVisibilityExactMint);
descriptor_probe!(ProbeAllowedRangeRemoval, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::AllowedRangeRemovalExactMint);
descriptor_probe!(ProbeHarmonicPitchSwap, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::HarmonicPitchSwapExactMint);
descriptor_probe!(ProbeHiddenAttributeRemoval, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::HiddenAttributeRemovalExactMint);

descriptor_probe!(ProbeChordGestureDistribution, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::ChordGestureDistributionExactMint);
descriptor_probe!(ProbeVisibilityStaffDistribution, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::VisibilityStaffDistributionExactMint);
descriptor_probe!(ProbeMasterExportFifths, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::MasterExportFifthsExactMint);
descriptor_probe!(ProbeFunctionalRespelling, ExactMintOwner, TransformationDescriptor, crate::pipeline::FunctionalRespellingExactMint);
descriptor_probe!(ProbeFinalChordEncoding, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::FinalChordEncodingExactMint);
descriptor_probe!(ProbeNotationPolish, PreservingMintOwner, PreservingDescriptor, crate::pipeline::NotationPolishPreservingMint);
descriptor_probe!(ProbePianistLayout, PreservingMintOwner, PreservingDescriptor, crate::layout::PianistLayoutPreservingMint);
descriptor_probe!(ProbePartialBeatBeam, PreservingMintOwner, PreservingDescriptor, crate::pipeline::PartialBeatBeamPreservingMint);
descriptor_probe!(ProbeOctaveWordLabel, PreservingMintOwner, PreservingDescriptor, crate::pipeline::OctaveWordLabelPreservingMint);
descriptor_probe!(ProbeVisualLayoutDefaults, PreservingMintOwner, PreservingDescriptor, crate::structural_write_authority::VisualLayoutDefaultsPreservingMint);
descriptor_probe!(ProbeLocalStaffDistance, PreservingMintOwner, PreservingDescriptor, crate::structural_write_authority::LocalStaffDistancePreservingMint);
descriptor_probe!(ProbeRustigClefCourse, PreservingMintOwner, PreservingDescriptor, crate::structural_write_authority::RustigClefCoursePreservingMint);
descriptor_probe!(ProbeFinalLayoutPhrase, PreservingMintOwner, PreservingDescriptor, crate::structural_write_authority::FinalLayoutPhrasePreservingMint);
descriptor_probe!(ProbeFinalBarline, AdditiveMintOwner, AdditiveDescriptor, crate::structural_write_authority::FinalBarlineAdditiveMint);
descriptor_probe!(ProbeDirectionInsertion, AdditiveMintOwner, AdditiveDescriptor, crate::structural_write_authority::DirectionInsertionAdditiveMint);

/// Number of production bindings exercised by the matrix (all bindings except
/// the two GENESIS owners, which are covered by the GENESIS tests).
const MATRIX_PROBES: usize = 69;

fn block_owner_matrix<O: BlockMintOwner + Copy>(owner: O) {
    assert!(O::DESCRIPTOR.identity.is_block());
    assert!(BlockTransformationReceipt::mint_block_receipt(owner, "<d>0</d>", "<d>0</d>", None).is_none());
    let receipt = BlockTransformationReceipt::mint_block_receipt(owner, "<d>0</d>", "<d>1</d>", None)
        .expect("changed BLOCK transition mints its receipt");
    assert!(receipt.validate_block_transition::<O>("<d>0</d>", "<d>1</d>").is_ok());
    assert!(receipt.validate_block_transition::<O>("<d>0</d>", "<d>2</d>").is_err());
}

fn exact_owner_matrix<O: ExactMintOwner + Copy>(owner: O) {
    assert_eq!(O::DESCRIPTOR.identity, IdentityContract::Exact);
    let (pre, post) = ("<d>0</d>", "<d>1</d>");
    assert!(ExactTransformationReceipt::mint_exact_receipt(owner, pre, pre, Vec::new()).is_none());
    assert!(ExactTransformationReceipt::mint_exact_receipt(owner, pre, post, Vec::new()).is_none());
    assert!(ExactTransformationReceipt::mint_exact_receipt(
        owner,
        pre,
        post,
        vec![ExactEdit::bound(1, 2, "x".to_string())]
    )
    .is_none());
    let receipt = ExactTransformationReceipt::mint_exact_receipt(
        owner,
        pre,
        post,
        vec![ExactEdit::bound(3, 4, "1".to_string())],
    )
    .expect("reproducing EXACT geometry mints its receipt");
    assert!(receipt.minted_by::<O>());
    assert_eq!(receipt.descriptor_row(), O::DESCRIPTOR.population_report_row);
    assert_eq!(receipt.realize_post_document(pre).as_deref(), Some(post));
    assert!(receipt.realize_post_document(post).is_none());
}

fn preserving_owner_matrix<O: PreservingMintOwner + Copy>(owner: O) {
    let pre = score(TWO_EIGHTHS);
    let presented = pre.replace("<attributes><divisions>", "<print new-page=\"yes\"/><attributes><divisions>");
    let changed = score(&TWO_EIGHTHS.replacen("<step>C</step>", "<step>E</step>", 1));
    assert!(PreservingTransformationReceipt::mint_preserving_receipt(owner, &pre, &pre).is_none());
    assert!(PreservingTransformationReceipt::mint_preserving_receipt(owner, &pre, &changed).is_none());
    let receipt = PreservingTransformationReceipt::mint_preserving_receipt(owner, &pre, &presented)
        .expect("presentation-only transition mints its receipt");
    assert!(receipt.validate_preserving_transition::<O>(&pre, &presented).is_ok());
}

fn additive_owner_matrix<O: AdditiveMintOwner + Copy>(owner: O) {
    let element = O::DESCRIPTOR.insertion_class.element_name();
    let pre = score(TWO_EIGHTHS);
    let at = pre.find("</measure>").expect("measure close");
    let fragment = format!("<{element}/>");
    let mut post = pre.clone();
    post.insert_str(at, &fragment);
    assert!(AdditiveTransformationReceipt::mint_additive_receipt(owner, &pre, &pre, Vec::new()).is_none());
    assert!(AdditiveTransformationReceipt::mint_additive_receipt(
        owner,
        &pre,
        &post,
        vec![ExactEdit::bound(at, at + 1, format!("{fragment}<"))]
    )
    .is_none());
    let foreign = if element == "barline" { "<direction/>" } else { "<barline/>" };
    let mut foreign_post = pre.clone();
    foreign_post.insert_str(at, foreign);
    assert!(AdditiveTransformationReceipt::mint_additive_receipt(
        owner,
        &pre,
        &foreign_post,
        vec![ExactEdit::bound(at, at, foreign.to_string())]
    )
    .is_none());
    let doubled = format!("{fragment}{fragment}");
    let mut doubled_post = pre.clone();
    doubled_post.insert_str(at, &doubled);
    assert!(AdditiveTransformationReceipt::mint_additive_receipt(
        owner,
        &pre,
        &doubled_post,
        vec![ExactEdit::bound(at, at, doubled.clone())]
    )
    .is_none());
    assert!(AdditiveTransformationReceipt::mint_additive_receipt(
        owner,
        &pre,
        &post,
        vec![ExactEdit::bound(at, at, fragment)]
    )
    .is_some());
}

descriptor_probe!(ProbeCursorNeutralRender, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::CursorNeutralRenderExactMint);
descriptor_probe!(ProbeSameLaneRawSplice, ExactMintOwner, TransformationDescriptor, crate::pipeline::SameLaneRawSpliceExactMint);
descriptor_probe!(ProbeMissingRestRender, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::MissingRestRenderExactMint);
descriptor_probe!(ProbeDirectionRelocation, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::DirectionRelocationExactMint);


descriptor_probe!(ProbeLedgerBoundedLowerStaffClefReadbackPreservingMint, PreservingMintOwner, PreservingDescriptor, crate::pipeline::LedgerBoundedLowerStaffClefReadbackPreservingMint);
descriptor_probe!(ProbeNotationClefIslandPreservingMint, PreservingMintOwner, PreservingDescriptor, crate::pipeline::NotationClefIslandPreservingMint);
descriptor_probe!(ProbeCommittedCrossStaffBeamPreservingMint, PreservingMintOwner, PreservingDescriptor, crate::pipeline::CommittedCrossStaffBeamPreservingMint);
descriptor_probe!(ProbeStructuralRestBridgeVariantAPreservingMint, PreservingMintOwner, PreservingDescriptor, crate::pipeline::StructuralRestBridgeVariantAPreservingMint);

#[test]
fn every_production_owner_binding_is_in_the_receipt_matrix() {
    let source = include_str!("../master_state_identity.rs");
    let bindings = source
        .lines()
        .filter(|line| line.starts_with("bind_mint_owner!("))
        .collect::<Vec<_>>();
    let genesis = bindings
        .iter()
        .filter(|line| line.starts_with("bind_mint_owner!(GenesisMintOwner,"))
        .count();
    assert_eq!(genesis, 2);
    assert_eq!(bindings.len() - genesis, MATRIX_PROBES);
    let this = include_str!("tests.rs");
    let probes = this
        .lines()
        .filter(|line| line.starts_with("descriptor_probe!(Probe"))
        .collect::<Vec<_>>();
    assert_eq!(probes.len(), MATRIX_PROBES);
    for binding in bindings.iter().filter(|line| !line.contains("GenesisMintOwner")) {
        let owner = binding.split(',').nth(1).expect("owner path").trim();
        assert!(
            probes.iter().any(|probe| probe.trim_end_matches(");").ends_with(owner)),
            "production owner {owner} has no probe in the receipt matrix"
        );
    }
}

#[test]
fn block_owner_matrix_covers_every_block_binding() {
    block_owner_matrix(ProbeVisibleRhythm);
    block_owner_matrix(ProbeBeatScopedBeaming);
    block_owner_matrix(ProbeClefMeasure);
    block_owner_matrix(ProbeOctaveMeasure);
}

#[test]
fn exact_owner_matrix_covers_every_exact_binding() {
    exact_owner_matrix(ProbeSafeTransfer);
    exact_owner_matrix(ProbeStaffVoiceMove);
    exact_owner_matrix(ProbeRestifyPatch);
    exact_owner_matrix(ProbeVoiceOverfullCrop);
    exact_owner_matrix(ProbeRestificationBatch);
    exact_owner_matrix(ProbeStaffVoiceScalarBatch);
    exact_owner_matrix(ProbeEditorialVoiceRewrite);
    exact_owner_matrix(ProbeNoteVisibility);
    exact_owner_matrix(ProbeAllowedRangeRemoval);
    exact_owner_matrix(ProbeHarmonicPitchSwap);
    exact_owner_matrix(ProbeHiddenAttributeRemoval);

    exact_owner_matrix(ProbeChordGestureDistribution);
    exact_owner_matrix(ProbeVisibilityStaffDistribution);
    exact_owner_matrix(ProbeMasterExportFifths);
    exact_owner_matrix(ProbeFunctionalRespelling);
    exact_owner_matrix(ProbeFinalChordEncoding);
    exact_owner_matrix(ProbeCursorNeutralRender);
    exact_owner_matrix(ProbeSameLaneRawSplice);
    exact_owner_matrix(ProbeMissingRestRender);
    exact_owner_matrix(ProbeDirectionRelocation);

}

#[test]
fn preserving_owner_matrix_covers_every_preserving_binding() {
    preserving_owner_matrix(ProbeNotationPolish);
    preserving_owner_matrix(ProbePianistLayout);
    preserving_owner_matrix(ProbePartialBeatBeam);
    preserving_owner_matrix(ProbeOctaveWordLabel);
    preserving_owner_matrix(ProbeVisualLayoutDefaults);
    preserving_owner_matrix(ProbeLocalStaffDistance);
    preserving_owner_matrix(ProbeRustigClefCourse);
    preserving_owner_matrix(ProbeFinalLayoutPhrase);
    preserving_owner_matrix(ProbeLedgerBoundedLowerStaffClefReadbackPreservingMint);
    preserving_owner_matrix(ProbeNotationClefIslandPreservingMint);
    preserving_owner_matrix(ProbeCommittedCrossStaffBeamPreservingMint);
    preserving_owner_matrix(ProbeStructuralRestBridgeVariantAPreservingMint);
}

#[test]
fn additive_owner_matrix_covers_every_additive_binding() {
    additive_owner_matrix(ProbeFinalBarline);
    additive_owner_matrix(ProbeDirectionInsertion);
    additive_owner_matrix(TestAdditiveOwner);
}

// ---------------------------------------------------------------------------
// Frozen descriptor tables
// ---------------------------------------------------------------------------

#[test]
fn production_descriptors_match_the_frozen_population() {
    let population = include_str!("../build_gate/destructive_transformation_population_v1.tsv");
    let rows = population
        .lines()
        .skip(1)
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let descriptors: [&TransformationDescriptor; 22] = [
        &MIDI_SEED_GENESIS,
        &ARRANGEMENT_GENESIS,
        &VISIBLE_RHYTHM_BLOCK,
        &BEAT_SCOPED_BEAMING_BLOCK,
        &CLEF_MEASURE_BLOCK,
        &OCTAVE_MEASURE_BLOCK,
        &SAFE_TRANSFER_EXACT,
        &STAFF_VOICE_MOVE_EXACT,
        &RESTIFY_PATCH_EXACT,
        &VOICE_OVERFULL_CROP_EXACT,
        &RESTIFICATION_BATCH_EXACT,
        &STAFF_VOICE_SCALAR_BATCH_EXACT,
        &EDITORIAL_VOICE_COMPLETE_REWRITE_EXACT,
        &NOTE_VISIBILITY_EXACT,
        &ALLOWED_RANGE_REMOVAL_EXACT,
        &HARMONIC_PITCH_SWAP_EXACT,
        &HIDDEN_ATTRIBUTE_REMOVAL_EXACT,
        &CHORD_GESTURE_DISTRIBUTION_EXACT,
        &VISIBILITY_STAFF_DISTRIBUTION_EXACT,
        &MASTER_EXPORT_FIFTHS_EXACT,
        &FUNCTIONAL_RESPELLING_EXACT,
        &FINAL_CHORD_ENCODING_EXACT,
    ];
    let mut seen_rows = std::collections::BTreeSet::new();
    for descriptor in descriptors {
        let row = rows
            .iter()
            .find(|columns| columns[0] == descriptor.population_report_row.to_string())
            .unwrap_or_else(|| panic!("row {} is not in population v1", descriptor.population_report_row));
        assert_eq!(row[1], descriptor.owner);
        assert_eq!(row[2], descriptor.owner_signature);
        assert_eq!(row[3], descriptor.effect.contract_label());
        assert_eq!(row[4], descriptor.identity.contract_label());
        assert!(matches!(row[5], "M" | "B"), "integrated rows act on Master state");
        assert!(seen_rows.insert(descriptor.population_report_row));
    }
    // The nested pair 59 -> 60 binds two distinct EXACT/M descriptors.
    assert_eq!(MASTER_EXPORT_FIFTHS_EXACT.population_report_row, 59);
    assert_eq!(FUNCTIONAL_RESPELLING_EXACT.population_report_row, 60);
    assert!(!MASTER_EXPORT_FIFTHS_EXACT.same_contract(&FUNCTIONAL_RESPELLING_EXACT));
}

fn exclusion_row(exclusion_id: &str) -> Vec<&'static str> {
    include_str!("../build_gate/destructive_transformation_exclusions_v1.tsv")
        .lines()
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .find(|columns| columns[0] == exclusion_id)
        .unwrap_or_else(|| panic!("exclusion {exclusion_id} missing"))
}

#[test]
fn preserving_descriptors_match_the_frozen_exclusions() {
    for descriptor in [
        &NOTATION_POLISH_PRESERVING,
        &PIANIST_LAYOUT_PRESERVING,
        &PARTIAL_BEAT_BEAM_PRESERVING,
        &OCTAVE_WORD_LABEL_PRESERVING,
        &VISUAL_LAYOUT_DEFAULTS_PRESERVING,
        &LOCAL_STAFF_DISTANCE_PRESERVING,
        &RUSTIG_CLEF_COURSE_PRESERVING,
        &FINAL_LAYOUT_PHRASE_PRESERVING,
        &LEDGER_BOUNDED_LOWER_STAFF_CLEF_PRESERVING,
    ] {
        let row = exclusion_row(descriptor.exclusion_id);
        assert_eq!(row[1], descriptor.owner);
        assert_eq!(row[2], descriptor.owner_signature);
        assert_eq!(row[3], "PRESERVING");
    }
}

/// The typed insertion class is derived from the frozen exclusion reason text
/// here, in the test, and never parsed at runtime.
fn insertion_class_from_frozen_reason(reason: &str) -> Option<AdditiveInsertionClass> {
    match reason {
        "final barline only" => Some(AdditiveInsertionClass::FinalBarline),
        "direction insertion only" => Some(AdditiveInsertionClass::Direction),
        _ => None,
    }
}

#[test]
fn additive_descriptors_match_the_frozen_exclusions_and_reasons() {
    for descriptor in [&FINAL_BARLINE_ADDITIVE, &DIRECTION_INSERTION_ADDITIVE] {
        let row = exclusion_row(descriptor.exclusion_id);
        assert_eq!(row[1], descriptor.owner);
        assert_eq!(row[2], descriptor.owner_signature);
        assert_eq!(row[3], "ADDITIVE");
        assert_eq!(insertion_class_from_frozen_reason(row[4]), Some(descriptor.insertion_class));
        assert!(row[4].contains(descriptor.insertion_class.element_name()));
    }
}

// ---------------------------------------------------------------------------
// Lineage commit (through the only commit boundary, the adoption state)
// ---------------------------------------------------------------------------

fn adopted(document: &str) -> crate::structural_write_authority::MasterAdoptedState {
    crate::structural_write_authority::MasterAdoptedState::adopt_genesis_state(
        document.to_string(),
        MasterLineage::lineage_from_genesis(genesis(document)),
    )
}

#[test]
fn committed_receipts_form_a_continuous_lineage() {
    let d0 = "<d><clef><sign>G</sign></clef><pitch>0</pitch></d>";
    let d1 = "<d><clef><sign>F</sign></clef><pitch>0</pitch></d>";
    let d2 = "<d><clef><sign>F</sign></clef><pitch>2</pitch></d>";
    let mut state = adopted(d0);
    let mut receipts = OwnerReceipts::default();
    receipts.push_block(BlockTransformationReceipt::mint_block_receipt(
        ProbeClefMeasure,
        d0,
        d1,
        Some((0, d0.len())),
    ));
    receipts.push_exact(None);
    receipts.push_exact(Some(permitted_exact_receipt_for_tests(d1, d2)));
    assert_eq!(receipts.len(), 2);
    assert!(receipts.coverage_diagnostic(d0, d2).is_none());
    assert_eq!(receipts.len(), 2, "diagnostic must not consume receipts");
    state.adopt_materialized_document(d2.to_string(), receipts, "test");
    let summary = state.lineage().summary(state.document());
    assert_eq!((summary.block_receipts, summary.block_spans_bound, summary.exact_receipts), (1, 1, 1));
    assert!(summary.continuity_holds);
    assert!(!state.lineage().lineage_continuity_holds("<d>3</d>"));
}

#[test]
fn preserving_and_additive_receipts_are_committed_events() {
    let pre = score(TWO_EIGHTHS);
    let presented = pre.replace("<attributes><divisions>", "<print new-page=\"yes\"/><attributes><divisions>");
    let at = presented.find("</measure>").expect("measure close");
    let added = format!("{}{}{}", &presented[..at], "<direction><direction-type><words>x</words></direction-type></direction>", &presented[at..]);
    let mut state = adopted(&pre);
    let mut receipts = OwnerReceipts::default();
    receipts.push_preserving(PreservingTransformationReceipt::mint_preserving_receipt(
        ProbePianistLayout,
        &pre,
        &presented,
    ));
    let additive = AdditiveTransformationReceipt::mint_additive_receipt(
        ProbeDirectionInsertion,
        &presented,
        &added,
        vec![ExactEdit::bound(at, at, "<direction><direction-type><words>x</words></direction-type></direction>".to_string())],
    );
    capture_additive_origin(ProbeDirectionInsertion, &additive, &presented, &added);
    receipts.push_additive(additive);
    state.adopt_materialized_document(added.clone(), receipts, "test");
    let summary = state.lineage().summary(&added);
    assert_eq!((summary.preserving_receipts, summary.additive_receipts, summary.additive_insertions), (1, 1, 1));
    assert!(summary.continuity_holds);
}

#[test]
#[should_panic(expected = "MASTER SOVEREIGNTY VIOLATION")]
fn receipt_with_a_foreign_pre_state_is_fatal() {
    let mut state = adopted("<d>0</d>");
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(Some(synthetic_exact_receipt_for_tests("<d>9</d>", "<d>1</d>", 3, 4, "1")));
    state.adopt_materialized_document("<d>1</d>".to_string(), receipts, "test");
}

#[test]
#[should_panic(expected = "MASTER SOVEREIGNTY VIOLATION")]
fn receipts_in_the_wrong_order_are_fatal() {
    let mut state = adopted("<d>0</d>");
    let first = synthetic_exact_receipt_for_tests("<d>0</d>", "<d>1</d>", 3, 4, "1");
    let second = synthetic_exact_receipt_for_tests("<d>1</d>", "<d>2</d>", 3, 4, "2");
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(Some(second));
    receipts.push_exact(Some(first));
    state.adopt_materialized_document("<d>2</d>".to_string(), receipts, "test");
}

#[test]
#[should_panic(expected = "MASTER SOVEREIGNTY VIOLATION")]
fn receipt_with_a_wrong_descriptor_leaves_the_transition_uncovered() {
    let mut state = adopted("<d>0</d>");
    // A mislabelled owner cannot mint; the uncovered change is fatal.
    let refused = ExactTransformationReceipt::mint_exact_receipt(
        MislabelledExactOwner,
        "<d>0</d>",
        "<d>1</d>",
        vec![ExactEdit::bound(3, 4, "1".to_string())],
    );
    assert!(refused.is_none());
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(refused);
    state.adopt_materialized_document("<d>1</d>".to_string(), receipts, "test");
}

#[test]
#[should_panic(expected = "MASTER SOVEREIGNTY VIOLATION")]
fn partially_covered_transition_is_fatal() {
    let mut state = adopted("<d>0</d>");
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(Some(synthetic_exact_receipt_for_tests("<d>0</d>", "<d>1</d>", 3, 4, "1")));
    state.adopt_materialized_document("<d>2</d>".to_string(), receipts, "test");
}

// ---------------------------------------------------------------------------
// Edit-script composition (#65 two-phase owner, #60 key + respelling)
// ---------------------------------------------------------------------------

fn edit(start: usize, end: usize, replacement: &str) -> ExactEdit {
    ExactEdit::bound(start, end, replacement.to_string())
}

fn two_phase(pre: &str, first: &[ExactEdit], second: &[ExactEdit]) -> (String, Option<Vec<ExactEdit>>) {
    let mut first_sorted = first.to_vec();
    first_sorted.sort_by_key(|edit| (edit.start, edit.end));
    let mut second_sorted = second.to_vec();
    second_sorted.sort_by_key(|edit| (edit.start, edit.end));
    let mid = realize_edit_script(pre, &first_sorted).expect("first script applies");
    let post = realize_edit_script(&mid, &second_sorted).expect("second script applies");
    (post, compose_exact_edit_scripts(pre, first.to_vec(), second.to_vec()))
}

#[test]
fn composition_of_phase_one_only_is_the_phase_one_script() {
    let pre = "<a>0123456789</a>";
    let first = [edit(3, 5, "XY"), edit(8, 8, "+")];
    let (post, composed) = two_phase(pre, &first, &[]);
    let composed = composed.expect("composable");
    assert!(exact_edits_reproduce(pre, &post, &composed));
    assert_eq!(composed, first.to_vec());
}

#[test]
fn composition_of_phase_two_only_is_the_phase_two_script() {
    let pre = "<a>0123456789</a>";
    let second = [edit(4, 6, "")];
    let (post, composed) = two_phase(pre, &[], &second);
    assert_eq!(composed.as_deref(), Some(&second[..]));
    assert!(exact_edits_reproduce(pre, &post, &composed.expect("composable")));
}

#[test]
fn independent_phase_two_edits_are_shifted_back_to_pre_coordinates() {
    let pre = "<a>0123456789</a>";
    let first = [edit(3, 4, "AAAA")];
    // In mid coordinates "9" moved by +3.
    let second = [edit(15, 16, "Z"), edit(9, 10, "")];
    let (post, composed) = two_phase(pre, &first, &second);
    let composed = composed.expect("composable");
    assert!(exact_edits_reproduce(pre, &post, &composed));
    assert_eq!(composed, vec![edit(3, 4, "AAAA"), edit(6, 7, ""), edit(12, 13, "Z")]);
}

#[test]
fn phase_two_edits_inside_a_phase_one_replacement_are_folded() {
    let pre = "<a>0123456789</a>";
    let first = [edit(3, 5, "<n>x</n>")];
    let second = [edit(6, 7, "y"), edit(12, 13, "")];
    let (post, composed) = two_phase(pre, &first, &second);
    let composed = composed.expect("composable");
    assert!(exact_edits_reproduce(pre, &post, &composed));
    assert_eq!(composed, vec![edit(3, 5, "<n>y</n>"), edit(6, 7, "")]);
    // A phase-2 edit touching the end of a phase-1 replacement is folded too.
    let touching = [edit(11, 12, "")];
    let (post, composed) = two_phase(pre, &first, &touching);
    let composed = composed.expect("composable");
    assert!(exact_edits_reproduce(pre, &post, &composed));
    assert_eq!(composed, vec![edit(3, 6, "<n>x</n>")]);
}

#[test]
fn multiple_phase_two_edits_compose_deterministically() {
    let pre = "<m><r/><r/><r/><n/></m>";
    let first = [edit(3, 7, "<rest/>"), edit(15, 19, "<note/>")];
    let mid = realize_edit_script(pre, &first).expect("mid");
    let mut second = Vec::new();
    for (index, _) in mid.match_indices("<r/>") {
        second.push(edit(index, index + 4, "<f/>"));
    }
    assert_eq!(second.len(), 2);
    let (post, composed) = two_phase(pre, &first, &second);
    let composed = composed.expect("composable");
    assert!(exact_edits_reproduce(pre, &post, &composed));
    let (_, again) = two_phase(pre, &first, &second);
    assert_eq!(again.expect("composable"), composed);
}

#[test]
fn no_op_composition_mints_no_receipt() {
    let pre = "<a/>";
    let (post, composed) = two_phase(pre, &[], &[]);
    assert_eq!(post, pre);
    let composed = composed.expect("composable");
    assert!(composed.is_empty());
    assert!(ExactTransformationReceipt::mint_exact_receipt(TestExactOwner, pre, &post, composed).is_none());
}

#[test]
fn impossible_composition_is_refused_and_leaves_the_transition_uncovered() {
    let pre = "<a>0123456789</a>";
    let overlapping = vec![edit(4, 8, "x"), edit(6, 9, "y")];
    assert!(compose_exact_edit_scripts(pre, Vec::new(), overlapping).is_none());
    // Manipulated geometry does not reproduce the post state: no receipt.
    let post = "<a>0X23456789</a>";
    assert!(ExactTransformationReceipt::mint_exact_receipt(TestExactOwner, pre, post, vec![edit(5, 6, "X")]).is_none());
}

#[test]
fn descending_application_script_merges_same_start_edits_in_application_order() {
    // Applied start-descending: (5,5,"b") then (5,5,"a") lands in front, then (2,3,"Z").
    let script = exact_script_from_descending_application(vec![
        (5, 5, "b".to_string()),
        (5, 5, "a".to_string()),
        (2, 3, "Z".to_string()),
    ]);
    assert_eq!(script, vec![edit(2, 3, "Z"), edit(5, 5, "ab")]);
    let pre = "0123456";
    let mut applied = pre.to_string();
    applied.insert(5, 'b');
    applied.insert(5, 'a');
    applied.replace_range(2..3, "Z");
    assert!(exact_edits_reproduce(pre, &applied, &script));
}

#[test]
fn file_bound_preserving_receipt_carries_its_realized_post_bytes() {
    let pre = score(TWO_EIGHTHS);
    let post = pre.replace("<attributes><divisions>", "<print new-system=\"yes\"/><attributes><divisions>");
    let receipt = synthetic_file_bound_preserving_receipt_for_tests(&pre, post.clone())
        .expect("presentation-only file transition");
    let (receipt, document) = receipt.split_realized_post_document();
    assert_eq!(document.as_deref(), Some(post.as_str()));
    assert!(receipt.validate_preserving_transition::<ProbePianistLayout>(&pre, &post).is_ok());
    let (_, again) = receipt.split_realized_post_document();
    assert!(again.is_none());
}

#[test]
fn transported_state_keeps_its_genesis_and_full_lineage() {
    let mut state = adopted("<pitch>0</pitch>");
    let genesis_identity = state.lineage().genesis.identity;
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(Some(permitted_exact_receipt_for_tests("<pitch>0</pitch>", "<pitch>1</pitch>")));
    state.adopt_materialized_document("<pitch>1</pitch>".to_string(), receipts, "test");
    let transported = state.clone();
    assert_eq!(transported.lineage().genesis.identity, genesis_identity);
    assert_eq!(transported.lineage().committed_event_count(), 1);
    assert!(transported.lineage().lineage_continuity_holds(transported.document()));
}

#[test]
fn reload_gap_reason_is_not_constructible() {
    // r1219 retired every gap reason; the release path adopts owner events only.
    let source = include_str!("../master_state_identity.rs");
    for retired in [
        "LineageGapReason",
        "LineagePendingGap",
        "lineage_extend_owner_receipts",
        "lineage_bridge_pending_gap",
    ] {
        assert!(!source.contains(retired), "{retired} must not exist");
    }
    let release = include_str!("../pipeline_shards/pipeline_orchestrator_release_report_exit_stage.rs");
    assert!(!release.contains("Reload"));
    assert!(!release.contains("read_to_string"));
}

/// Synthetic receipt carrying the frozen #60 descriptor, for nested-owner
/// transport tests of #59.
pub(crate) fn synthetic_functional_respelling_receipt_for_tests(
    pre: &str,
    post: &str,
    start: usize,
    end: usize,
    replacement: &str,
) -> ExactTransformationReceipt {
    ExactTransformationReceipt::mint_exact_receipt(
        ProbeFunctionalRespelling,
        pre,
        post,
        vec![ExactEdit::bound(start, end, replacement.to_string())],
    )
    .expect("synthetic #60 receipt reproduces its post state")
}

#[test]
#[should_panic(expected = "MASTER SOVEREIGNTY VIOLATION")]
fn nested_59_60_receipts_in_swapped_order_are_fatal() {
    let mut state = adopted("<k>0</k><n>a</n>");
    let outer = ExactTransformationReceipt::mint_exact_receipt(
        ProbeMasterExportFifths,
        "<k>0</k><n>a</n>",
        "<k>2</k><n>a</n>",
        vec![edit(3, 4, "2")],
    )
    .expect("#59 receipt");
    let inner = synthetic_functional_respelling_receipt_for_tests(
        "<k>2</k><n>a</n>",
        "<k>2</k><n>b</n>",
        11,
        12,
        "b",
    );
    assert_eq!((outer.descriptor_row(), inner.descriptor_row()), (59, 60));
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(Some(inner));
    receipts.push_exact(Some(outer));
    state.adopt_materialized_document("<k>2</k><n>b</n>".to_string(), receipts, "test");
}

#[test]
fn nested_59_60_receipts_in_real_order_are_committed() {
    let (a, b, c) = ("<d><fifths>0</fifths><pitch>a</pitch></d>", "<d><fifths>2</fifths><pitch>a</pitch></d>", "<d><fifths>2</fifths><pitch>b</pitch></d>");
    let mut state = adopted(a);
    let fifths = a.find("0</fifths>").expect("fifths");
    let outer = ExactTransformationReceipt::mint_exact_receipt(ProbeMasterExportFifths, a, b, vec![edit(fifths, fifths + 1, "2")]);
    let pitch = b.find("a</pitch>").expect("pitch");
    let inner = synthetic_functional_respelling_receipt_for_tests(b, c, pitch, pitch + 1, "b");
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(outer);
    receipts.push_exact(Some(inner));
    state.adopt_materialized_document(c.to_string(), receipts, "test");
    let summary = state.lineage().summary(state.document());
    assert_eq!(summary.exact_rows.get(&59), Some(&1));
    assert_eq!(summary.exact_rows.get(&60), Some(&1));
}

#[test]
fn gate_c_master_lineage_has_a_single_commit_boundary() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut stack = vec![root.clone()];
    let mut offenders = Vec::new();
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("source dir") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                if !path.ends_with("build_gate") {
                    stack.push(path);
                }
                continue;
            }
            if path.extension().and_then(|value| value.to_str()) != Some("rs") {
                continue;
            }
            let relative = path.strip_prefix(&root).expect("relative").to_string_lossy().replace('\\', "/");
            if relative == "master_state_identity/tests.rs" {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("source");
            let compact = text.split_whitespace().collect::<String>();
            if compact.contains(".commit_owner_receipts(")
                && relative != "structural_write_authority/master_adoption/adopted_state.rs"
            {
                offenders.push(format!("{relative}: lineage commit outside the adoption boundary"));
            }
            if compact.contains("MasterAdoptionCommitBoundary{_sealed:()}")
                && relative != "structural_write_authority/master_adoption/adopted_state.rs"
            {
                offenders.push(format!("{relative}: commit capability constructed outside the boundary"));
            }
            for retired in ["GapReason", "PendingGap", "lineage_extend_owner_receipts", "MasterXmlReplacementRequest::bind("] {
                if compact.contains(retired) {
                    offenders.push(format!("{relative}: retired {retired}"));
                }
            }
            if compact.contains("current_thread_render_checkpoint_xml().or_else(current_thread_render_artifact_xml)")
                && !compact.contains("fnbind_tonal_frame_fact_from_render_snapshot(")
            {
                offenders.push(format!("{relative}: thread-local Render->Master backflow"));
            }
        }
    }
    assert!(offenders.is_empty(), "{offenders:#?}");
}

#[test]
fn descending_application_script_folds_same_start_replacements_exactly() {
    let pre = "0123456789";
    // Earlier (3,6,"abcd") then later (3,5,"Z") on the mutated buffer.
    let mut applied = pre.to_string();
    applied.replace_range(3..6, "abcd");
    applied.replace_range(3..5, "Z");
    let script = exact_script_from_descending_application(vec![
        (3, 6, "abcd".to_string()),
        (3, 5, "Z".to_string()),
    ]);
    assert!(exact_edits_reproduce(pre, &applied, &script));
    // A later edit that consumes the earlier replacement and further pre bytes.
    let mut applied = pre.to_string();
    applied.replace_range(3..4, "x");
    applied.replace_range(3..6, "YY");
    let script = exact_script_from_descending_application(vec![
        (3, 4, "x".to_string()),
        (3, 6, "YY".to_string()),
    ]);
    assert_eq!(script, vec![ExactEdit::bound(3, 6, "YY".to_string())]);
    assert!(exact_edits_reproduce(pre, &applied, &script));
}

#[test]
fn render_boundary_descriptors_preserve_the_accepted_contracts() {
    let population = include_str!("../build_gate/ssc_population_prerequisites_r1250_v1.txt");
    let installed = include_str!("../build_gate/destructive_transformation_population_v1.tsv");
    for descriptor in [&CURSOR_NEUTRAL_RENDER_EXACT, &SAME_LANE_RAW_SPLICE_EXACT, &MISSING_REST_RENDER_EXACT] {
        let row = population.lines().map(|line| line.split('\t').collect::<Vec<_>>())
            .find(|row| row[0] == descriptor.population_report_row.to_string()).unwrap();
        assert_eq!(row[1], descriptor.owner);
        assert_eq!(row[2], descriptor.owner_signature);
        assert_eq!(row[3], descriptor.effect.contract_label());
        assert_eq!(row[4], descriptor.identity.contract_label());
        assert_eq!(row[5], "R");
        if descriptor.population_report_row == 67 { assert_eq!(row[9], "DEFERRED_RENDER_PREREQUISITE"); }
        let current = installed.lines().map(|line| line.split('\t').collect::<Vec<_>>())
            .find(|row| row[0] == descriptor.population_report_row.to_string()).unwrap();
        assert_eq!(&current[..9], &row[..9]);
        assert_eq!(current[10], row[10]);
        if descriptor.population_report_row == 67 { assert_eq!(current[9], "READY"); }
    }
    let direction = include_str!("../build_gate/render_event_boundaries_v1.tsv").lines()
        .map(|line| line.split('\t').collect::<Vec<_>>()).find(|row| row[0] == "DirectionRelocation").unwrap();
    assert_eq!(DIRECTION_RELOCATION_EXACT.owner, direction[2]);
    assert_eq!(DIRECTION_RELOCATION_EXACT.owner_signature, direction[3]);
    assert_eq!(direction[6], "last #58 committed render state");
    assert!(!CURSOR_NEUTRAL_RENDER_EXACT.same_contract(&DIRECTION_RELOCATION_EXACT));
}


#[test]
fn normalized_exact_receipt_rejects_coordinate_payload_owner_and_pre_tampering() {
    use crate::structural_write_authority::apply_backup_anchor_rewrite_authorized as owner;
    let pre = "é<backup><duration>4</duration></backup>終";
    let start = "é".len();
    let end = pre.find('終').unwrap();
    let replacement = "<backup><duration>8</duration></backup>";
    let issue = || owner(pre, vec![(start, end, replacement.to_string())]);
    let (post, receipt) = issue();
    let receipt = receipt.unwrap();
    assert_eq!(receipt.realize_post_document(pre), post);
    assert!(receipt.descriptor.same_contract(&RENDER_ROW_41_EXACT));
    assert!(!receipt.descriptor.same_contract(&RENDER_ROW_42_EXACT));
    assert!(receipt.realize_post_document("wrong pre").is_none());
    for change in 0..3 {
        let (_, receipt) = issue();
        let mut receipt = receipt.unwrap();
        match change {
            0 => receipt.edits[0].start += 1,
            1 => receipt.edits[0].end -= 1,
            _ => receipt.edits[0].replacement.push('x'),
        }
        assert!(receipt.realize_post_document(pre).is_none());
    }
}



descriptor_probe!(ProbeNormalizedRow7, BlockMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow7BlockMint);
descriptor_probe!(ProbeNormalizedRow17, BlockMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow17BlockMint);
descriptor_probe!(ProbeNormalizedRow19, BlockMintOwner, TransformationDescriptor, crate::pipeline::RenderRow19BlockMint);
descriptor_probe!(ProbeNormalizedRow20, BlockMintOwner, TransformationDescriptor, crate::pipeline::RenderRow20BlockMint);
descriptor_probe!(ProbeNormalizedRow21, BlockMintOwner, TransformationDescriptor, crate::pipeline::RenderRow21BlockMint);
descriptor_probe!(ProbeNormalizedRow22, ExactMintOwner, TransformationDescriptor, crate::pipeline::RenderRow22ExactMint);
descriptor_probe!(ProbeNormalizedRow30, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow30ExactMint);
descriptor_probe!(ProbeNormalizedRow31, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow31ExactMint);
descriptor_probe!(ProbeNormalizedRow32, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow32ExactMint);
descriptor_probe!(ProbeNormalizedRow33, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow33ExactMint);
descriptor_probe!(ProbeNormalizedRow34, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow34ExactMint);
descriptor_probe!(ProbeNormalizedRow35, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow35ExactMint);
descriptor_probe!(ProbeNormalizedRow36, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow36ExactMint);
descriptor_probe!(ProbeNormalizedRow37, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow37ExactMint);
descriptor_probe!(ProbeNormalizedRow38, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow38ExactMint);
descriptor_probe!(ProbeNormalizedRow41, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::VisibleLane41RenderExactMint);
descriptor_probe!(ProbeNormalizedRow42, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::VisibleLane42RenderExactMint);
descriptor_probe!(ProbeNormalizedRow43, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::VisibleLane43RenderExactMint);
descriptor_probe!(ProbeNormalizedRow44, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::VisibleLane44RenderExactMint);
descriptor_probe!(ProbeNormalizedRow45, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::VisibleLane45RenderExactMint);
descriptor_probe!(ProbeNormalizedRow46, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::VisibleLane46RenderExactMint);
descriptor_probe!(ProbeNormalizedRow47, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::VisibleLane47RenderExactMint);
descriptor_probe!(ProbeNormalizedRow48, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::VisibleLane48RenderExactMint);
descriptor_probe!(ProbeNormalizedRow51, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow51ExactMint);
descriptor_probe!(ProbeNormalizedRow54, BlockMintOwner, TransformationDescriptor, crate::pipeline::RenderRow54BlockMint);
descriptor_probe!(ProbeNormalizedRow55, BlockMintOwner, TransformationDescriptor, crate::pipeline::RenderRow55BlockMint);
descriptor_probe!(ProbeNormalizedRow57, BlockMintOwner, TransformationDescriptor, crate::pipeline::RenderRow57BlockMint);
descriptor_probe!(ProbeNormalizedRow58, ExactMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow58ExactMint);
descriptor_probe!(ProbeNormalizedRow61, BlockMintOwner, TransformationDescriptor, crate::structural_write_authority::RenderRow61BlockMint);
descriptor_probe!(ProbeNormalizedRow64, ExactMintOwner, TransformationDescriptor, crate::pipeline::RenderRow64ExactMint);
descriptor_probe!(ProbeNormalizedRow66, BlockMintOwner, TransformationDescriptor, crate::pipeline::RenderRow66BlockMint);
#[test]
fn newly_normalized_owner_matrix_has_full_positive_negative_coverage() {
block_owner_matrix(ProbeNormalizedRow7);
block_owner_matrix(ProbeNormalizedRow17);
block_owner_matrix(ProbeNormalizedRow19);
block_owner_matrix(ProbeNormalizedRow20);
block_owner_matrix(ProbeNormalizedRow21);
exact_owner_matrix(ProbeNormalizedRow22);
exact_owner_matrix(ProbeNormalizedRow30);
exact_owner_matrix(ProbeNormalizedRow31);
exact_owner_matrix(ProbeNormalizedRow32);
exact_owner_matrix(ProbeNormalizedRow33);
exact_owner_matrix(ProbeNormalizedRow34);
exact_owner_matrix(ProbeNormalizedRow35);
exact_owner_matrix(ProbeNormalizedRow36);
exact_owner_matrix(ProbeNormalizedRow37);
exact_owner_matrix(ProbeNormalizedRow38);
exact_owner_matrix(ProbeNormalizedRow41);
exact_owner_matrix(ProbeNormalizedRow42);
exact_owner_matrix(ProbeNormalizedRow43);
exact_owner_matrix(ProbeNormalizedRow44);
exact_owner_matrix(ProbeNormalizedRow45);
exact_owner_matrix(ProbeNormalizedRow46);
exact_owner_matrix(ProbeNormalizedRow47);
exact_owner_matrix(ProbeNormalizedRow48);
exact_owner_matrix(ProbeNormalizedRow51);
block_owner_matrix(ProbeNormalizedRow54);
block_owner_matrix(ProbeNormalizedRow55);
block_owner_matrix(ProbeNormalizedRow57);
exact_owner_matrix(ProbeNormalizedRow58);
block_owner_matrix(ProbeNormalizedRow61);
exact_owner_matrix(ProbeNormalizedRow64);
block_owner_matrix(ProbeNormalizedRow66);
}

fn ssc_completed_report() -> String {
    let report = "TYPED-LOWER-CLEF-SURFACE-MATERIALIZATION\n----------------------------------------\nStatus: present.\nSILENT-VOICE-COVERAGE-WRITE-OUTCOME\n--------------------------------\nStatus: present.\nRendererOutcome: observed_from_existing_writer_audit.\n";
    crate::final_report::DraftReport::new(report.to_owned())
        .compact(str::to_owned)
        .finalize()
        .expect("final report")
        .as_str()
        .to_owned()
}

#[test]
fn ssc_unproved_master_event_is_refused_before_any_commit() {
    // r1287 (contract v4 A7): an event without a provable envelope (test descriptor) is
    // ContainmentUnproven and refused before commit; document and lineage stay unchanged,
    // and the receipt committed first in the same transition is not committed either.
    reset_master_lineage_measurement();
    let (pre, middle, post) = ("<d><pitch>0</pitch></d>", "<d><pitch>1</pitch></d>", "<d><pitch>2</pitch></d>");
    let mut state = adopted(pre);
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut receipts = OwnerReceipts::default();
        receipts.push_exact(Some(permitted_exact_receipt_for_tests(pre, middle)));
        receipts.push_exact(Some(synthetic_exact_receipt_for_tests(middle, post, 10, 11, "2")));
        state.adopt_materialized_document(post.to_owned(), receipts, "test unknown family");
    }))
    .expect_err("unproven adoption must be refused");
    let message = refusal.downcast_ref::<String>().cloned().unwrap_or_default();
    assert!(message.contains("MASTER SOVEREIGNTY VIOLATION") && message.contains("SSC adoption refused") && message.contains("unproven=1"), "{message}");
    assert_eq!(state.lineage().committed_event_count(), 0);
    assert_eq!(state.document(), pre);
}
#[test]
fn ssc_discarded_owner_receipts_never_count_as_adopted_events() {
    reset_master_lineage_measurement();
    let _discarded = synthetic_exact_receipt_for_tests("<d>0</d>", "<d>1</d>", 3, 4, "1");
    let report = ssc_completed_report();
    assert!(!report.contains("SSC-SHADOW-PERMIT"));
    assert!(report.contains("ReleaseScopeMissing: 2"), "{report}");
    assert!(report.contains("ReleasedOwnerEvents: 0"), "{report}");
}
pub(crate) fn committed_identity_matches_for_test(lineage: &MasterLineage, identity: MintedIdentity) -> bool { lineage.events.last().is_some_and(|event| event.identity == identity) }

#[test]
fn block_receipt_of_a_descriptor_without_frozen_envelope_row_is_refused_before_commit() {
    // r1287 (contract v4 A7): no frozen envelope row means ContainmentUnproven; the Block
    // receipt is valid as a receipt but its adoption is refused.
    reset_master_lineage_measurement();
    let (pre, post) = ("<d><clef><sign>G</sign></clef></d>", "<d><clef><sign>F</sign></clef></d>");
    let mut state = adopted(pre);
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut receipts = OwnerReceipts::default();
        receipts.push_block(BlockTransformationReceipt::mint_block_receipt(TestBlockOwner, pre, post, Some((0, pre.len()))));
        state.adopt_materialized_document(post.to_owned(), receipts, "test block without row");
    }))
    .expect_err("adoption without frozen envelope row must be refused");
    let message = refusal.downcast_ref::<String>().cloned().unwrap_or_default();
    assert!(message.contains("SSC adoption refused") && message.contains("permitted=0"), "{message}");
    assert_eq!(state.lineage().committed_event_count(), 0);
    assert_eq!(state.document(), pre);
}

#[test]
fn ssc_released_identity_excludes_committed_losing_candidate_with_same_bytes() {
    reset_master_lineage_measurement();
    let pre = "<pitch>0</pitch>";
    let post = "<pitch>1</pitch>";
    let mut losing = adopted(pre);
    let mut released = adopted(pre);
    for state in [&mut losing, &mut released] {
        let mut receipts = OwnerReceipts::default();
        receipts.push_exact(Some(permitted_exact_receipt_for_tests(pre, post)));
        state.adopt_materialized_document(post.to_owned(), receipts, "test independent candidate");
    }
    assert_ne!(losing.lineage().events[0].identity, released.lineage().events[0].identity);
    record_final_master_lineage(released.lineage(), released.document());
    let report = ssc_completed_report();
    for line in ["MasterOwnerEvents: 1", "ClassifiedOwnerEvents: 1", "V4Permitted: 1", "MissingEvidence: 0", "V4UnreleasedCommits: checked=1; permitted=1; denied=0; unproven=0; out_of_scope=0; integrity=0"] {
        assert!(report.contains(line), "{line}: {report}");
    }
    let shared = released.lineage().clone();
    record_final_master_lineage(&shared, released.document());
    assert_eq!(ssc_completed_report(), report);
    assert_eq!(losing.document(), post);
    assert_eq!(released.document(), post);
}

#[test]
fn ssc_additive_origin_rejects_foreign_pre_state_without_changing_receipt() {
    // r1287 (contract v4 A7): an origin captured from foreign bytes is never bound, so the
    // receipt has no evidence (MissingEvidence) and its adoption is refused before commit.
    reset_master_lineage_measurement();
    let pre = score(TWO_EIGHTHS);
    let at = pre.find("</measure>").expect("measure close");
    let post = pre.replacen("</measure>", "<direction/></measure>", 1);
    let receipt = AdditiveTransformationReceipt::mint_additive_receipt(
        TestAdditiveOwner, &pre, &post, vec![ExactEdit::bound(at, at, "<direction/>".to_owned())],
    );
    assert!(receipt.is_some(), "original receipt");
    capture_additive_origin(TestAdditiveOwner, &receipt, &pre, "<foreign/>");
    capture_additive_origin(TestAdditiveOwner, &receipt, "<foreign/>", &post);
    let mut state = adopted(&pre);
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut receipts = OwnerReceipts::default();
        receipts.push_additive(receipt);
        state.adopt_materialized_document(post.clone(), receipts, "test missing capture");
    }))
    .expect_err("adoption without bound evidence must be refused");
    let message = refusal.downcast_ref::<String>().cloned().unwrap_or_default();
    assert!(message.contains("SSC adoption refused") && message.contains("MissingEvidence"), "{message}");
    assert_eq!(state.lineage().committed_event_count(), 0);
    assert_eq!(state.document(), pre);
}


pub(crate) fn forget_additive_origin_for_tests(receipt: &AdditiveTransformationReceipt) {
    crate::ssc_evidence_proof::release_unadopted_additive_origin(&SSC_ORIGIN_ACCESS, &receipt.identity);
}
