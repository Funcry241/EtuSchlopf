//! Read-only evidence of the original owner execution, retained only after adoption.
//! No selector, writer, proof-diagnostic adapter or permit enforcement lives here.
use crate::master_state_identity::{
    AdditiveTransformationReceipt, BlockTransformationReceipt, ExactTransformationReceipt,
    MintedIdentity, PreservingTransformationReceipt, SscReceiptOriginAccess,
};
use std::sync::Arc;
#[cfg(not(test))]
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SscEvidenceDomain {
    Master,
    Render,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SscReleasedEventRef<'event> {
    domain: SscEvidenceDomain,
    identity: &'event crate::master_state_identity::MintedIdentity,
}


pub(crate) struct SscOriginReceiptRef<'event> {
    inner: SscOriginReceiptKind<'event>,
}

enum SscOriginReceiptKind<'event> {
    Block(
        &'event crate::master_state_identity::BlockTransformationReceipt
    ),
    Exact(
        &'event crate::master_state_identity::ExactTransformationReceipt
    ),
    Preserving(
        &'event crate::master_state_identity::PreservingTransformationReceipt
    ),
    Additive(
        &'event crate::master_state_identity::AdditiveTransformationReceipt
    ),
}


pub(crate) struct SscEventBinding<'event> {
    released: SscReleasedEventRef<'event>,
    pre_state: &'event str,
    post_state: &'event str,
}

pub(crate) struct SscTransformationEvidence<'event> {
    event: &'event SscEventBinding<'event>,
    receipt: SscOriginReceiptRef<'event>,
    scope: SscFamilyScope,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SscEvidenceIntegrityFailure {
    MissingEvidence,
    DuplicateEvidence,
    ReceiptIdentityMismatch,
    DomainMismatch,
    PreStateUnreadable,
    PostStateUnreadable,
    PreFingerprintMismatch,
    PostFingerprintMismatch,
    BindingMismatch,
    ProofBindingMismatch,
    ProofFamilyMismatch,
    ContradictoryInvariantEvidence,
    IllegalProofCombination,
}


pub(crate) enum SscEvidenceIntegrity<'event> {
    Bound(&'event SscTransformationEvidence<'event>),

    Missing,

    Duplicate {
        records: usize,
    },

    Invalid {
        reason: SscEvidenceIntegrityFailure,
    },
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SscProofFamily {
    F7RestVoiceStrictStaff,
    F5AtomicVoiceStaffVisibility,
    F9AuthoritySerialization,
    Additive,
    Preserving,
    Edit,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SscFamilyScope {
    OutOfScope,
    InScope(SscProofFamily),
}


#[derive(Debug, Clone)]
struct SscConstructiveInvariantCertificate {
    preserved_structure: SscCheckedStructurePreservation,
}

#[derive(Debug, Clone)]
struct SscCheckedStructurePreservation {
    normalized_bytes: usize,
}


#[derive(Debug, Clone)]
struct SscConstructiveInvariantCounterexample {
    kind: SscInvariantFailureKind,
    witness: SscExactDifferenceWitness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SscInvariantFailureKind {
    PreservedProjectionChanged,
    NoteCardinalityChanged,
    ForbiddenFieldChanged,
    RequiredInsertionGeometryBroken,
}

#[derive(Debug, Clone)]
struct SscExactDifferenceWitness {
    pre_range: std::ops::Range<usize>,
    post_range: std::ops::Range<usize>,
    pre_bytes: std::sync::Arc<str>,
    post_bytes: std::sync::Arc<str>,
}


pub(crate) struct SscInvariantProof<'event> {
    event: &'event SscEventBinding<'event>,
    family: SscProofFamily,
    certificate: SscConstructiveInvariantCertificate,
}

pub(crate) struct SscInvariantCounterexample<'event> {
    event: &'event SscEventBinding<'event>,
    family: SscProofFamily,
    counterexample: SscConstructiveInvariantCounterexample,
}

#[derive(Clone, Copy)]
pub(crate) enum SscInvariantAssessment<'event> {
    Unproven,
    Proven(&'event SscInvariantProof<'event>),
    Refuted(&'event SscInvariantCounterexample<'event>),
}


#[derive(Debug, Clone)]
struct SscConstructiveNecessityCertificate {
    defect: SscStructuralDefect,
}


#[derive(Debug, Clone)]
enum SscStructuralDefect {
    InvisibleUncoveredRest {
        note: usize,
    },
    MissingRequestedStaff {
        note: usize,
    },
    MissingRequestedSerialization {
        note: usize,
    },
    MissingFinalBarline {
        boundary: usize,
    },
    BrokenBeamChain {
        note: usize,
    },
}


pub(crate) struct SscNecessityWitness<'event> {
    event: &'event SscEventBinding<'event>,
    family: SscProofFamily,
    certificate: SscConstructiveNecessityCertificate,
}

#[derive(Clone, Copy)]
pub(crate) enum SscNecessityAssessment<'event> {
    Unproven,
    Proven(&'event SscNecessityWitness<'event>),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SscPermitVerdictV2 {
    OutOfScope,
    InvariantUnproven,
    InvariantDenied,
    NecessityUnproven,
    Permit,
}


pub(crate) struct SscClassifiedObservationV2<'event> {
    transformation: &'event SscTransformationEvidence<'event>,
    scope: SscFamilyScope,
    invariant: SscInvariantAssessment<'event>,
    necessity: SscNecessityAssessment<'event>,
    verdict: SscPermitVerdictV2,
    containment: SscContainmentAssessment<'event>,
    containment_verdict: SscContainmentVerdict,
}


pub(crate) enum SscReleasedObservationV2<'event> {
    IntegrityFailure {
        released: SscReleasedEventRef<'event>,
        integrity: SscEvidenceIntegrityFailure,
    },

    Classified(
        SscClassifiedObservationV2<'event>
    ),
}


// Contract v4 A2/A3 containment surface (frozen in r1278).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SscEnvelopeKind {
    Projection,
    Insertion,
    Edit,
    Lane,
    Block,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SscEnvelopeRef {
    descriptor: &'static str,
    kind: SscEnvelopeKind,
}


#[derive(Debug, Clone)]
struct SscConstructiveContainmentCertificate {
    checked_bytes: usize,
}


pub(crate) struct SscContainmentProof<'event> {
    event: &'event SscEventBinding<'event>,
    envelope: SscEnvelopeRef,
    certificate: SscConstructiveContainmentCertificate,
}


pub(crate) struct SscContainmentCounterexample<'event> {
    event: &'event SscEventBinding<'event>,
    envelope: SscEnvelopeRef,
    witness: SscExactDifferenceWitness,
}


#[derive(Clone, Copy)]
pub(crate) enum SscContainmentAssessment<'event> {
    Unproven,
    Contained(&'event SscContainmentProof<'event>),
    Escaped(&'event SscContainmentCounterexample<'event>),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SscContainmentVerdict {
    OutOfScope,
    ContainmentUnproven,
    ContainmentDenied,
    Permit,
}


#[derive(Debug, Clone)]
enum CapturedPreState {
    Raw(Arc<str>),
    Deflated(Arc<[u8]>),
}
impl CapturedPreState {
    fn capture(pre: &str) -> Self {
        use std::io::Read;
        if pre.len() > 4096 {
            let mut encoded = Vec::new();
            let mut encoder =
                flate2::read::ZlibEncoder::new(pre.as_bytes(), flate2::Compression::fast());
            if encoder.read_to_end(&mut encoded).is_ok() && encoded.len() < pre.len() {
                return Self::Deflated(Arc::from(encoded));
            }
        }
        Self::Raw(Arc::from(pre))
    }
    fn exact_bytes(&self) -> Option<String> {
        use std::io::Read;
        match self {
            Self::Raw(pre) => Some(pre.to_string()),
            Self::Deflated(bytes) => {
                let mut pre = String::new();
                flate2::read::ZlibDecoder::new(bytes.as_ref())
                    .read_to_string(&mut pre)
                    .ok()?;
                Some(pre)
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum SscOriginScope {
    F7,
    F5,
    F9,
    FinalBarline,
    DirectionInsertion,
    Preserving(SscPreservingOrigin),
    Unproved,
}
/// Necessity contract of a PRESERVING exclusion, bound by its sealed mint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SscPreservingNecessity {
    BeamChainRepair,
    Unbound,
}
/// Origin of one PRESERVING owner execution: its frozen descriptor, the
/// owner's own conserved projection and its necessity contract. Only the
/// sealed receipt origin can bind it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SscPreservingOrigin {
    descriptor: &'static crate::master_state_identity::PreservingDescriptor,
    projection: fn(&str) -> Option<String>,
    necessity: SscPreservingNecessity,
}
impl SscPreservingOrigin {
    pub(crate) fn bind(
        _origin: &SscReceiptOriginAccess,
        descriptor: &'static crate::master_state_identity::PreservingDescriptor,
        projection: fn(&str) -> Option<String>,
        necessity: SscPreservingNecessity,
    ) -> Self {
        Self { descriptor, projection, necessity }
    }
}
#[derive(Debug, Clone)]
pub(crate) struct SscOriginState {
    pre_state: CapturedPreState,
    post_state: CapturedPreState,
    scope: SscOriginScope,
}
impl SscOriginState {
    #[cfg(test)]
    pub(crate) fn captured_pre_for_test(&self) -> Option<String> {
        self.pre_state.exact_bytes()
    }

    pub(crate) fn capture(
        _origin: &SscReceiptOriginAccess,
        pre: &str,
        post: &str,
        scope: SscOriginScope,
    ) -> Self {
        Self {
            pre_state: CapturedPreState::capture(pre),
            post_state: CapturedPreState::capture(post),
            scope,
        }
    }
}

// Compare complete parsed structure, preserving order, content, all attributes
// and timing/cursor elements, except the explicitly named representation fields.
fn structure(xml: &str, scope: SscOriginScope) -> Option<String> {
    structure_skipping(xml, scope, None)
}
fn structure_skipping(
    xml: &str,
    scope: SscOriginScope,
    skip: Option<std::ops::Range<usize>>,
) -> Option<String> {
    let document = roxmltree::Document::parse(xml).ok()?;
    fn visit(
        node: roxmltree::Node<'_, '_>,
        scope: SscOriginScope,
        skip: &Option<std::ops::Range<usize>>,
        out: &mut String,
    ) {
        if skip.as_ref().is_some_and(|range| *range == node.range()) {
            return;
        }
        if node.is_element() {
            let name = node.tag_name().name();
            if matches!(scope, SscOriginScope::F5 | SscOriginScope::F9)
                && matches!(name, "voice" | "staff")
                && node.parent().is_some_and(|n| n.has_tag_name("note"))
            {
                return;
            }
            out.push('<');
            out.push_str(&format!("{:?}:", node.tag_name().namespace()));
            out.push_str(name);
            let rest = node.has_tag_name("note") && node.children().any(|n| n.has_tag_name("rest"));
            let mut attrs: Vec<_> = node
                .attributes()
                .filter(|a| {
                    !(matches!(scope, SscOriginScope::F7)
                        && rest
                        && matches!(a.name(), "print-object" | "print-spacing"))
                })
                .map(|a| (a.namespace(), a.name(), a.value()))
                .collect();
            attrs.sort_unstable();
            for (namespace, name, value) in attrs {
                out.push_str(&format!("|{namespace:?}:{name}={value:?}"));
            }
            out.push('>');
            for child in node.children() {
                visit(child, scope, skip, out);
            }
            out.push_str("</");
            out.push_str(name);
            out.push('>');
        } else if node.is_text()
            && let Some(text) = node.text().filter(|text| !text.trim().is_empty())
        {
            out.push_str(&format!("{text:?}"));
        }
    }
    let mut out = String::new();
    visit(document.root_element(), scope, &skip, &mut out);
    Some(out)
}
fn positive(
    family: SscProofFamily,
    structure: String,
    defect: SscStructuralDefect,
) -> Option<(
    SscProofFamily,
    SscConstructiveInvariantCertificate,
    SscConstructiveNecessityCertificate,
)> {
    Some((
        family,
        SscConstructiveInvariantCertificate {
            preserved_structure: SscCheckedStructurePreservation {
                normalized_bytes: structure.len(),
            },
        },
        SscConstructiveNecessityCertificate { defect },
    ))
}
fn child<'a, 'input>(
    node: roxmltree::Node<'a, 'input>,
    tag: &str,
) -> Option<roxmltree::Node<'a, 'input>> {
    node.children().find(|n| n.has_tag_name(tag))
}
fn scalar(node: roxmltree::Node<'_, '_>, tag: &str) -> Option<String> {
    child(node, tag)?.text().map(|text| text.trim().to_owned())
}
fn prove_origin(
    pre: &str,
    post: &str,
    scope: SscOriginScope,
) -> Option<(
    SscProofFamily,
    SscConstructiveInvariantCertificate,
    SscConstructiveNecessityCertificate,
)> {
    if pre == post || matches!(scope, SscOriginScope::Unproved) {
        return None;
    }
    let before = roxmltree::Document::parse(pre).ok()?;
    let after = roxmltree::Document::parse(post).ok()?;
    if matches!(scope, SscOriginScope::FinalBarline) {
        // The sealed E004 owner has already proved insertion-only geometry and
        // conserved-content subsequence. Independently bind its missing-final
        // boundary defect and exact sole light-heavy barline insertion.
        let boundary = pre.rfind("</measure>")?;
        if before
            .descendants()
            .any(|n| n.has_tag_name("bar-style") && n.text().map(str::trim) == Some("light-heavy"))
        {
            return None;
        }
        let inserted = after
            .descendants()
            .filter(|n| n.has_tag_name("barline"))
            .filter(|n| {
                n.attribute("location") == Some("right")
                    && child(*n, "bar-style").and_then(|n| n.text()).map(str::trim) == Some("light-heavy")
            })
            .collect::<Vec<_>>();
        if inserted.len() != 1 {
            return None;
        }
        let barline = inserted[0];
        let last_measure = after
            .descendants()
            .rfind(|n| n.has_tag_name("measure"))?;
        if barline.parent() != Some(last_measure)
            || last_measure
                .children()
                .rfind(roxmltree::Node::is_element)
                != Some(barline)
        {
            return None;
        }
        let unchanged = structure(pre, SscOriginScope::Unproved)?;
        if unchanged != structure_skipping(post, SscOriginScope::Unproved, Some(barline.range()))? {
            return None;
        }
        return positive(
            SscProofFamily::Additive,
            unchanged,
            SscStructuralDefect::MissingFinalBarline { boundary },
        );
    }
    let unchanged = structure(pre, scope)?;
    if unchanged != structure(post, scope)? {
        return None;
    }
    let before_notes: Vec<_> = before
        .descendants()
        .filter(|n| n.has_tag_name("note"))
        .collect();
    let after_notes: Vec<_> = after
        .descendants()
        .filter(|n| n.has_tag_name("note"))
        .collect();
    if before_notes.len() != after_notes.len() {
        return None;
    }
    match scope {
        SscOriginScope::F5 | SscOriginScope::F9 => {
            // Only missing fields of the existing sealed owner's bound request
            // can witness necessity. Already-present lane changes are Unknown.
            let mut missing = None;
            for (index, (old, new)) in before_notes.into_iter().zip(after_notes).enumerate() {
                for field in ["voice", "staff"] {
                    let left = scalar(old, field);
                    let right = scalar(new, field);
                    if left == right {
                        continue;
                    }
                    if left.is_some()
                        || right.as_deref().is_none_or(|value| {
                            value.parse::<u32>().ok().is_none_or(|value| value == 0)
                        })
                    {
                        return None;
                    }
                    if matches!(scope, SscOriginScope::F5) && field != "staff" {
                        return None;
                    }
                    if old.children().filter(|n| n.has_tag_name(field)).count() != 0
                        || new.children().filter(|n| n.has_tag_name(field)).count() != 1
                    {
                        return None;
                    }
                    missing = Some(index);
                }
            }
            let note = missing?;
            if matches!(scope, SscOriginScope::F5) {
                positive(
                    SscProofFamily::F5AtomicVoiceStaffVisibility,
                    unchanged,
                    SscStructuralDefect::MissingRequestedStaff { note },
                )
            } else {
                positive(
                    SscProofFamily::F9AuthoritySerialization,
                    unchanged,
                    SscStructuralDefect::MissingRequestedSerialization { note },
                )
            }
        }
        SscOriginScope::F7 => {
            let intervals = rest_intervals(&before)?;
            let mut restored = None;
            for (index, (old, new)) in before_notes.into_iter().zip(after_notes).enumerate() {
                let old_visible = old.attribute("print-object") != Some("no");
                let new_visible = new.attribute("print-object") != Some("no");
                if old_visible == new_visible {
                    continue;
                }
                if old_visible || !new_visible || child(old, "rest").is_none() {
                    return None;
                }
                let interval = intervals.get(index)?;
                if interval.end <= interval.start || interval.start < 0 || interval.chord {
                    return None;
                }
                if intervals.iter().enumerate().any(|(other, value)| {
                    other != index
                        && value.measure == interval.measure
                        && value.voice == interval.voice
                        && value.staff == interval.staff
                        && value.visible
                        && value.start < interval.end
                        && interval.start < value.end
                }) {
                    return None;
                }
                restored = Some(index);
            }
            positive(
                SscProofFamily::F7RestVoiceStrictStaff,
                unchanged,
                SscStructuralDefect::InvisibleUncoveredRest { note: restored? },
            )
        }
        _ => None,
    }
}
struct NoteInterval {
    measure: usize,
    start: i64,
    end: i64,
    voice: String,
    staff: String,
    visible: bool,
    chord: bool,
}
fn rest_intervals(document: &roxmltree::Document<'_>) -> Option<Vec<NoteInterval>> {
    let mut result = Vec::new();
    for (measure_index, measure) in document
        .descendants()
        .filter(|n| n.has_tag_name("measure"))
        .enumerate()
    {
        let mut cursor = 0_i64;
        let mut last_onset = 0_i64;
        for node in measure.children().filter(roxmltree::Node::is_element) {
            match node.tag_name().name() {
                "backup" => {
                    cursor = cursor.checked_sub(scalar(node, "duration")?.parse::<i64>().ok()?)?;
                }
                "forward" => {
                    cursor = cursor.checked_add(scalar(node, "duration")?.parse::<i64>().ok()?)?;
                }
                "note" => {
                    if child(node, "grace").is_some() {
                        return None;
                    }
                    let duration = scalar(node, "duration")?.parse::<i64>().ok()?;
                    if duration <= 0 {
                        return None;
                    }
                    let chord = child(node, "chord").is_some();
                    let start = if chord { last_onset } else { cursor };
                    let end = start.checked_add(duration)?;
                    result.push(NoteInterval {
                        measure: measure_index,
                        start,
                        end,
                        voice: scalar(node, "voice")?,
                        staff: scalar(node, "staff")?,
                        visible: node.attribute("print-object") != Some("no"),
                        chord,
                    });
                    if !chord {
                        last_onset = start;
                        cursor = end;
                    }
                }
                _ => {}
            }
        }
    }
    Some(result)
}

// E036 insertion geometry, re-derived from the exact captured bytes only: post
// is pre with exactly one contiguous insertion, and that insertion is
// whitespace, at most one comment and exactly one <direction> element that is
// a direct child of a <measure>. Returns the inserted byte span in post.
enum DirectionGeometry {
    Inserted(std::ops::Range<usize>),
    Broken(std::ops::Range<usize>, std::ops::Range<usize>),
    Unreadable,
}
fn direction_insertion_geometry(pre: &str, post: &str) -> DirectionGeometry {
    let (a, b) = (pre.as_bytes(), post.as_bytes());
    let prefix = a.iter().zip(b).take_while(|(x, y)| x == y).count();
    let suffix = a.iter().rev().zip(b.iter().rev()).take_while(|(x, y)| x == y).count();
    let broken = || {
        let start = prefix.min(a.len()).min(b.len());
        DirectionGeometry::Broken(start..a.len().saturating_sub(suffix).max(start), start..b.len().saturating_sub(suffix).max(start))
    };
    if b.len() <= a.len() || prefix + suffix < a.len() {
        return broken();
    }
    let Ok(document) = roxmltree::Document::parse(post) else {
        return DirectionGeometry::Unreadable;
    };
    let inserted = b.len() - a.len();
    let first = a.len() - suffix.min(a.len());
    for start in first..=prefix.min(a.len()) {
        let span = start..start + inserted;
        if !post.is_char_boundary(span.start) || !post.is_char_boundary(span.end) {
            continue;
        }
        let mut elements = document
            .descendants()
            .filter(|node| node.is_element() && node.range().start >= span.start && node.range().end <= span.end);
        let Some(direction) = elements.next() else { continue };
        let others = document.root().descendants().filter(|node| {
            node.range().start >= span.start
                && node.range().end <= span.end
                && !(node.is_text() && node.text().is_some_and(|text| text.trim().is_empty()))
                && !node.ancestors().skip(1).any(|ancestor| ancestor == direction)
                && *node != direction
        });
        let comments = others.clone().filter(roxmltree::Node::is_comment).count();
        if direction.has_tag_name("direction")
            && direction.parent().is_some_and(|parent| parent.has_tag_name("measure"))
            && comments <= 1
            && others.count() == comments
        {
            return DirectionGeometry::Inserted(span);
        }
    }
    broken()
}
// Canonical structure without the <beam> children of notes; everything else
// (order, content, attributes, timing) must be byte-structurally identical.
fn structure_without_note_beams(xml: &str) -> Option<String> {
    let document = roxmltree::Document::parse(xml).ok()?;
    fn visit(node: roxmltree::Node<'_, '_>, out: &mut String) {
        if node.is_element() {
            if node.has_tag_name("beam") && node.parent().is_some_and(|n| n.has_tag_name("note")) {
                return;
            }
            out.push_str(&format!("<{:?}:{}", node.tag_name().namespace(), node.tag_name().name()));
            let mut attrs: Vec<_> = node.attributes().map(|a| (a.namespace(), a.name(), a.value())).collect();
            attrs.sort_unstable();
            for (namespace, name, value) in attrs {
                out.push_str(&format!("|{namespace:?}:{name}={value:?}"));
            }
            out.push('>');
            for child in node.children() {
                visit(child, out);
            }
            out.push_str(&format!("</{}>", node.tag_name().name()));
        } else if node.is_text()
            && let Some(text) = node.text().filter(|text| !text.trim().is_empty())
        {
            out.push_str(&format!("{text:?}"));
        }
    }
    let mut out = String::new();
    visit(document.root_element(), &mut out);
    Some(out)
}
// One beam chain: (part, measure, voice, beam number, grace).
type BeamChainKey = (usize, usize, String, String, bool);
struct BeamedNote {
    chord: bool,
    keys: Vec<BeamChainKey>,
    beams: Vec<(String, String)>,
}
// MusicXML beam chains are balanced per voice and beam number inside one
// measure: begin opens, continue requires an open chain, end closes it, a
// hook never changes it. Returns all notes and the set of malformed chains.
fn beam_chains(document: &roxmltree::Document<'_>) -> (Vec<BeamedNote>, std::collections::BTreeSet<BeamChainKey>) {
    let mut notes = Vec::new();
    let mut malformed = std::collections::BTreeSet::new();
    for (part_index, part) in document.descendants().filter(|n| n.has_tag_name("part")).enumerate() {
        for (measure_index, measure) in part.children().filter(|n| n.has_tag_name("measure")).enumerate() {
            let mut open = std::collections::BTreeMap::<BeamChainKey, bool>::new();
            for note in measure.children().filter(|n| n.has_tag_name("note")) {
                let voice = scalar(note, "voice").unwrap_or_default();
                let grace = child(note, "grace").is_some();
                let chord = child(note, "chord").is_some();
                let mut beams = note
                    .children()
                    .filter(|n| n.has_tag_name("beam"))
                    .map(|beam| (beam.attribute("number").unwrap_or("1").to_owned(), beam.text().unwrap_or("").trim().to_owned()))
                    .collect::<Vec<_>>();
                let mut keys = Vec::new();
                let mut seen = std::collections::BTreeSet::new();
                for (number, value) in &beams {
                    let key = (part_index, measure_index, voice.clone(), number.clone(), grace);
                    keys.push(key.clone());
                    if chord {
                        continue;
                    }
                    let state = open.entry(key.clone()).or_insert(false);
                    let valid = seen.insert(number.clone())
                        && match value.as_str() {
                            "begin" => !std::mem::replace(state, true),
                            "continue" => *state,
                            "end" => std::mem::replace(state, false),
                            "forward hook" | "backward hook" => true,
                            _ => false,
                        };
                    if !valid {
                        malformed.insert(key);
                    }
                }
                beams.sort();
                notes.push(BeamedNote { chord, keys, beams });
            }
            malformed.extend(open.into_iter().filter(|(_, is_open)| *is_open).map(|(key, _)| key));
        }
    }
    (notes, malformed)
}
// Constructive necessity of a beam-only PRESERVING transition: the pre-state
// has at least one malformed beam chain, the post-state has none, nothing but
// note beams changed, and every changed note beam belongs to a chain that was
// malformed in the pre-state. Witness: the first such repaired note.
fn beam_chain_repair_note(pre: &str, post: &str) -> Option<usize> {
    if structure_without_note_beams(pre)? != structure_without_note_beams(post)? {
        return None;
    }
    let before = roxmltree::Document::parse(pre).ok()?;
    let after = roxmltree::Document::parse(post).ok()?;
    let (old_notes, broken) = beam_chains(&before);
    let (new_notes, still_broken) = beam_chains(&after);
    if broken.is_empty() || !still_broken.is_empty() || old_notes.len() != new_notes.len() {
        return None;
    }
    let mut repaired = None;
    for (index, (old, new)) in old_notes.iter().zip(&new_notes).enumerate() {
        if old.beams == new.beams {
            continue;
        }
        if old.chord || new.chord {
            return None;
        }
        let changed = old.keys.iter().chain(&new.keys).filter(|key| {
            let number = &key.3;
            old.beams.iter().find(|(n, _)| n == number) != new.beams.iter().find(|(n, _)| n == number)
        });
        for key in changed {
            if !broken.contains(key) {
                return None;
            }
        }
        repaired.get_or_insert(index);
    }
    repaired
}
type OriginAssessment<'event> = (
    Option<SscInvariantProof<'event>>,
    Option<SscInvariantCounterexample<'event>>,
    Option<SscNecessityWitness<'event>>,
);
fn difference_witness(pre: &str, post: &str, pre_range: std::ops::Range<usize>, post_range: std::ops::Range<usize>) -> SscExactDifferenceWitness {
    SscExactDifferenceWitness { pre_range, post_range, pre_bytes: Arc::from(pre), post_bytes: Arc::from(post) }
}
// r1284 4b: minimal constructive witness. The common prefix and suffix are trimmed on
// character boundaries, so the bound ranges name exactly the first differing span.
fn minimal_difference(pre: &str, post: &str) -> (usize, usize, usize) {
    let mut start = pre.bytes().zip(post.bytes()).take_while(|(a, b)| a == b).count();
    while !(pre.is_char_boundary(start) && post.is_char_boundary(start)) { start -= 1; }
    let mut suffix = pre.as_bytes()[start..].iter().rev().zip(post.as_bytes()[start..].iter().rev()).take_while(|(a, b)| a == b).count();
    while !(pre.is_char_boundary(pre.len() - suffix) && post.is_char_boundary(post.len() - suffix)) { suffix -= 1; }
    let (mut pre_end, mut post_end) = (pre.len() - suffix, post.len() - suffix);
    // A pure insertion or removal is ambiguous within repeated bytes; among the equivalent
    // ASCII positions take the nearest one that starts at an element tag.
    if (start == pre_end) != (start == post_end) {
        let (text, end) = if start == pre_end { (post.as_bytes(), post_end) } else { (pre.as_bytes(), pre_end) };
        let mut shift = 0;
        while text[start - shift] != b'<' && start > shift && text[start - shift - 1].is_ascii() && text[start - shift - 1] == text[end - shift - 1] { shift += 1; }
        if text[start - shift] != b'<' { shift = 0; }
        start -= shift; pre_end -= shift; post_end -= shift;
    }
    (start, pre_end, post_end)
}
fn minimal_witness(pre: &str, post: &str) -> SscExactDifferenceWitness {
    let (start, pre_end, post_end) = minimal_difference(pre, post);
    difference_witness(pre, post, start..pre_end, start..post_end)
}
// A witness over a derived form names its basis in a fixed first line; the ranges point
// behind it. Raw document witnesses carry no basis line.
fn basis_witness(basis: &str, pre: &str, post: &str) -> SscExactDifferenceWitness {
    minimal_witness(&format!("basis:{basis}\n{pre}"), &format!("basis:{basis}\n{post}"))
}
fn witness_basis(bytes: &str) -> &str {
    bytes.strip_prefix("basis:").and_then(|rest| rest.split_once('\n')).map_or("document", |(basis, _)| basis)
}
fn witness_excerpt(bytes: &str, range: &std::ops::Range<usize>) -> String {
    let mut end = range.end.min(range.start.saturating_add(120));
    while end > range.start && !bytes.is_char_boundary(end) { end -= 1; }
    let excerpt = bytes.get(range.start..end).unwrap_or("");
    if end < range.end { format!("{excerpt}...") } else { excerpt.to_owned() }
}
fn direction_assessment<'event>(event: &'event SscEventBinding<'event>, family: SscProofFamily) -> OriginAssessment<'event> {
    if family != SscProofFamily::Additive {
        return (None, None, None);
    }
    match direction_insertion_geometry(event.pre_state, event.post_state) {
        DirectionGeometry::Inserted(_) => {
            let Some(unchanged) = structure(event.pre_state, SscOriginScope::Unproved) else { return (None, None, None) };
            // Insertion geometry is proven; no direction necessity contract is
            // bound (the tempo obligation is WAV-conditional and WAV is no
            // certificate source), so necessity stays unproven.
            let certificate = SscConstructiveInvariantCertificate { preserved_structure: SscCheckedStructurePreservation { normalized_bytes: unchanged.len() } };
            (Some(SscInvariantProof { event, family, certificate }), None, None)
        }
        DirectionGeometry::Broken(pre_range, post_range) => {
            let witness = difference_witness(event.pre_state, event.post_state, pre_range, post_range);
            let counterexample = SscConstructiveInvariantCounterexample { kind: SscInvariantFailureKind::RequiredInsertionGeometryBroken, witness };
            (None, Some(SscInvariantCounterexample { event, family, counterexample }), None)
        }
        DirectionGeometry::Unreadable => (None, None, None),
    }
}
fn preserving_assessment<'event>(event: &'event SscEventBinding<'event>, origin: SscPreservingOrigin, family: SscProofFamily) -> OriginAssessment<'event> {
    if family != SscProofFamily::Preserving {
        return (None, None, None);
    }
    let (Some(before), Some(after)) = ((origin.projection)(event.pre_state), (origin.projection)(event.post_state)) else {
        return (None, None, None);
    };
    if before != after {
        let witness = basis_witness("owner_projection", &before, &after);
        let counterexample = SscConstructiveInvariantCounterexample { kind: SscInvariantFailureKind::PreservedProjectionChanged, witness };
        return (None, Some(SscInvariantCounterexample { event, family, counterexample }), None);
    }
    let certificate = SscConstructiveInvariantCertificate { preserved_structure: SscCheckedStructurePreservation { normalized_bytes: before.len() } };
    let necessity = match origin.necessity {
        SscPreservingNecessity::BeamChainRepair => beam_chain_repair_note(event.pre_state, event.post_state)
            .map(|note| SscNecessityWitness { event, family, certificate: SscConstructiveNecessityCertificate { defect: SscStructuralDefect::BrokenBeamChain { note } } }),
        SscPreservingNecessity::Unbound => None,
    };
    (Some(SscInvariantProof { event, family, certificate }), None, necessity)
}
// A sealed origin scope is evidence only for the receipt kind and descriptor
// that bound it at mint.
fn scope_bound_to_receipt(receipt: &SscOriginReceiptKind<'_>, scope: SscOriginScope) -> bool {
    use crate::master_state_identity::AdditiveInsertionClass;
    match (scope, receipt) {
        (SscOriginScope::Preserving(origin), SscOriginReceiptKind::Preserving(receipt)) => {
            *origin.descriptor == *crate::master_state_identity::ssc_preserving_descriptor(receipt)
        }
        (SscOriginScope::DirectionInsertion, SscOriginReceiptKind::Additive(receipt)) => {
            crate::master_state_identity::ssc_additive_insertion_class(receipt) == AdditiveInsertionClass::Direction
        }
        (SscOriginScope::FinalBarline, SscOriginReceiptKind::Additive(receipt)) => {
            crate::master_state_identity::ssc_additive_insertion_class(receipt) == AdditiveInsertionClass::FinalBarline
        }
        (SscOriginScope::Preserving(_) | SscOriginScope::DirectionInsertion | SscOriginScope::FinalBarline, _) => false,
        _ => true,
    }
}
// Measurement label only; never an input to a certificate.
fn scope_label(scope: SscOriginScope, pre: &str, post: &str) -> Option<&'static str> {
    match scope {
        SscOriginScope::Preserving(origin) => Some(match origin.necessity {
            SscPreservingNecessity::BeamChainRepair => "PRESERVING_BEAM_CHAIN",
            SscPreservingNecessity::Unbound => "PRESERVING_UNBOUND",
        }),
        SscOriginScope::DirectionInsertion => Some(match direction_insertion_geometry(pre, post) {
            DirectionGeometry::Inserted(span) => {
                let fragment = &post[span];
                if fragment.contains("<dynamics") {
                    "DIRECTION_DYNAMICS"
                } else if fragment.contains("<sound") || fragment.contains("<metronome") {
                    "DIRECTION_TEMPO"
                } else if fragment.contains("<words") {
                    "DIRECTION_WORDS"
                } else {
                    "DIRECTION_OTHER"
                }
            }
            _ => "DIRECTION_OTHER",
        }),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Contract v4 A3: containment against the frozen envelope table. The table is the
// parent-admitted upper bound; this checker may be narrower, never wider.
struct EnvelopeRow {
    envelope: SscEnvelopeRef,
    elements: Vec<&'static str>,
    fields: Vec<(&'static str, char, &'static str)>,
    operations: Vec<&'static str>,
}
fn envelope_kind(name: &str) -> Option<SscEnvelopeKind> {
    match name {
        "Projection" => Some(SscEnvelopeKind::Projection),
        "Insertion" => Some(SscEnvelopeKind::Insertion),
        "Edit" => Some(SscEnvelopeKind::Edit),
        "Lane" => Some(SscEnvelopeKind::Lane),
        "Block" => Some(SscEnvelopeKind::Block),
        _ => None,
    }
}
fn envelope_row(descriptor: &str) -> Option<EnvelopeRow> {
    let table = include_str!("build_gate/ssc_containment_envelopes_v1.tsv");
    table.lines().filter_map(|line| line.strip_prefix("row=")).find_map(|row| {
        let columns = row.split('|').map(str::trim).collect::<Vec<_>>();
        if columns.len() != 9 || columns[0] != descriptor {
            return None;
        }
        let list = |value: &'static str| value.split(',').map(str::trim).filter(|item| !item.is_empty() && *item != "-").collect::<Vec<_>>();
        let fields = list(columns[4])
            .into_iter()
            .filter_map(|field| {
                field
                    .split_once('.')
                    .map(|(host, name)| (host, '.', name))
                    .or_else(|| field.split_once('@').map(|(host, name)| (host, '@', name)))
            })
            .collect();
        Some(EnvelopeRow {
            envelope: SscEnvelopeRef { descriptor: columns[0], kind: envelope_kind(columns[2])? },
            elements: list(columns[3]),
            fields,
            operations: list(columns[5]),
        })
    })
}
// The frozen descriptor behind a sealed origin scope (bound at mint).
fn envelope_descriptor(scope: SscOriginScope) -> Option<&'static str> {
    match scope {
        SscOriginScope::F5 => Some("STAFF_VOICE_MOVE_EXACT"),
        SscOriginScope::F9 => Some("CURSOR_NEUTRAL_RENDER_EXACT"),
        SscOriginScope::F7 => Some("RENDER_ROW_44_EXACT"),
        SscOriginScope::FinalBarline => Some("FINAL_BARLINE_ADDITIVE"),
        SscOriginScope::DirectionInsertion => Some("DIRECTION_INSERTION_ADDITIVE"),
        SscOriginScope::Preserving(origin) => crate::master_state_identity::ssc_preserving_descriptor_name(origin.descriptor),
        SscOriginScope::Unproved => None,
    }
}
// Receipt-kind and sealed descriptor decide new families; captured Unproved is
// deliberately not rewritten at mint (the frozen #57 owner remains unchanged).
fn receipt_envelope_descriptor(receipt: &SscOriginReceiptKind<'_>, scope: SscOriginScope) -> Option<&'static str> {
    match receipt {
        SscOriginReceiptKind::Exact(receipt) if matches!(scope, SscOriginScope::Unproved) =>
            crate::master_state_identity::ssc_exact_descriptor_name(crate::master_state_identity::ssc_exact_descriptor(receipt)),
        SscOriginReceiptKind::Block(receipt) if matches!(scope, SscOriginScope::Unproved) =>
            crate::master_state_identity::ssc_block_descriptor_name(crate::master_state_identity::ssc_block_descriptor(receipt)),
        _ => envelope_descriptor(scope),
    }
}
fn receipt_proof_scope(receipt: &SscOriginReceiptKind<'_>, scope: SscOriginScope) -> SscFamilyScope {
    if !matches!(scope, SscOriginScope::Unproved) { return proof_scope(scope); }
    match receipt {
        SscOriginReceiptKind::Block(_) => SscFamilyScope::InScope(SscProofFamily::Block),
        SscOriginReceiptKind::Exact(_) => {
            match receipt_envelope_descriptor(receipt, scope).and_then(envelope_row) {
                Some(row) if row.envelope.kind == SscEnvelopeKind::Lane => SscFamilyScope::OutOfScope,
                _ => SscFamilyScope::InScope(SscProofFamily::Edit),
            }
        }
        _ => SscFamilyScope::OutOfScope,
    }
}
// Replays the original immutable script, checking every unchanged byte segment.
// No fingerprint or structural normalization substitutes for locality.
fn edit_script_locality(pre: &str, post: &str, edits: &[(usize, usize, &str)]) -> bool {
    if edits.is_empty() { return false; }
    let (mut pre_cursor, mut post_cursor) = (0usize, 0usize);
    let mut previous = None;
    for &(start, end, replacement) in edits {
        if start > end || start < pre_cursor || previous == Some((start, start)) && start == end { return false; }
        let Some(unchanged) = pre.get(pre_cursor..start) else { return false };
        let Some(unchanged_end) = post_cursor.checked_add(unchanged.len()) else { return false };
        let Some(replacement_end) = unchanged_end.checked_add(replacement.len()) else { return false };
        if pre.get(start..end).is_none() || post.get(post_cursor..unchanged_end) != Some(unchanged)
            || post.get(unchanged_end..replacement_end) != Some(replacement) { return false; }
        previous = Some((start, end)); pre_cursor = end; post_cursor = replacement_end;
    }
    pre.get(pre_cursor..).is_some() && pre.get(pre_cursor..) == post.get(post_cursor..)
}
fn bound_span_locality(pre: &str, post: &str, span: Option<(usize, usize)>) -> bool {
    let Some((start, end)) = span else { return true };
    if start > end { return false; }
    let (Some(prefix), Some(suffix)) = (pre.get(..start), pre.get(end..)) else { return false };
    let Some(post_end) = post.len().checked_sub(suffix.len()) else { return false };
    post_end >= start && post.get(..start) == Some(prefix) && post.get(post_end..) == Some(suffix)
}
fn receipt_locality_assessment<'event>(event: &'event SscEventBinding<'event>, receipt: &SscOriginReceiptKind<'_>, family: SscProofFamily) -> OriginAssessment<'event> {
    let valid = match (family, receipt) {
        (SscProofFamily::Edit, SscOriginReceiptKind::Exact(receipt)) => edit_script_locality(event.pre_state, event.post_state, &crate::master_state_identity::ssc_exact_edit_spans(receipt)),
        (SscProofFamily::Block, SscOriginReceiptKind::Block(receipt)) => bound_span_locality(event.pre_state, event.post_state, crate::master_state_identity::ssc_block_bound_span(receipt)),
        _ => return (None, None, None),
    };
    if valid {
        let certificate = SscConstructiveInvariantCertificate { preserved_structure: SscCheckedStructurePreservation { normalized_bytes: event.pre_state.len() } };
        (Some(SscInvariantProof { event, family, certificate }), None, None)
    } else {
        let witness = minimal_witness(event.pre_state, event.post_state);
        let counterexample = SscConstructiveInvariantCounterexample { kind: SscInvariantFailureKind::ForbiddenFieldChanged, witness };
        (None, Some(SscInvariantCounterexample { event, family, counterexample }), None)
    }
}
// Canonical structure with every envelope-declared mutable part removed. Whole
// elements that may change are skipped; without `change` they are kept aside so that
// only declared insertions or removals remain possible.
struct ContainmentForm {
    canonical: String,
    whole: Vec<(String, String)>,
}
#[cfg(test)]
fn containment_form(xml: &str, row: &EnvelopeRow) -> Option<ContainmentForm> {
    let document = roxmltree::Document::parse(xml).ok()?;
    Some(containment_form_of(&document, row))
}
// r1290: same canonical bytes as before, written into one buffer instead of one allocation
// per element, attribute and text node; the parsed document is shared with the edit check.
fn containment_form_of(document: &roxmltree::Document<'_>, row: &EnvelopeRow) -> ContainmentForm {
    use std::fmt::Write;
    fn serialize(node: roxmltree::Node<'_, '_>, out: &mut String) {
        if node.is_element() {
            let name = node.tag_name().name();
            out.push('<');
            out.push_str(name);
            let mut attrs: Vec<_> = node.attributes().map(|a| (a.name(), a.value())).collect();
            attrs.sort_unstable();
            for (attr, value) in attrs {
                let _ = write!(out, "|{attr}={value:?}");
            }
            out.push('>');
            for child in node.children() {
                serialize(child, out);
            }
            out.push_str("</");
            out.push_str(name);
            out.push('>');
        } else if node.is_text()
            && let Some(text) = node.text().filter(|text| !text.trim().is_empty())
        {
            let _ = write!(out, "{text:?}");
        }
    }
    fn visit(node: roxmltree::Node<'_, '_>, row: &EnvelopeRow, form: &mut ContainmentForm) {
        if node.is_element() {
            let name = node.tag_name().name();
            let parent = node.parent().filter(roxmltree::Node::is_element).map(|p| p.tag_name().name());
            let hosted = parent.is_some_and(|parent| row.fields.iter().any(|(host, kind, field)| *host == parent && *kind == '.' && *field == name));
            let has_fields = row.fields.iter().any(|(host, _, _)| *host == name);
            let lane_scoped = row.envelope.kind != SscEnvelopeKind::Lane || parent == Some("note");
            let whole = row.elements.contains(&name) && !has_fields && lane_scoped;
            if hosted || whole {
                if !row.operations.contains(&"change") {
                    let mut text = String::new();
                    serialize(node, &mut text);
                    form.whole.push((name.to_owned(), text));
                }
                return;
            }
            form.canonical.push('<');
            form.canonical.push_str(name);
            let mut attrs: Vec<_> = node
                .attributes()
                .filter(|a| !row.fields.iter().any(|(host, kind, field)| *host == name && *kind == '@' && *field == a.name()))
                .map(|a| (a.name(), a.value()))
                .collect();
            attrs.sort_unstable();
            for (attr, value) in attrs {
                let _ = write!(form.canonical, "|{attr}={value:?}");
            }
            form.canonical.push('>');
            for child in node.children() {
                visit(child, row, form);
            }
            form.canonical.push_str("</");
            form.canonical.push_str(name);
            form.canonical.push('>');
        } else if node.is_text()
            && let Some(text) = node.text().filter(|text| !text.trim().is_empty())
        {
            let _ = write!(form.canonical, "{text:?}");
        }
    }
    let mut form = ContainmentForm { canonical: String::new(), whole: Vec::new() };
    visit(document.root_element(), row, &mut form);
    form
}
fn is_subsequence(short: &[(String, String)], long: &[(String, String)]) -> bool {
    let mut rest = long.iter();
    short.iter().all(|item| rest.by_ref().any(|candidate| candidate == item))
}
enum Containment {
    Contained(usize),
    Escaped(SscExactDifferenceWitness),
}
#[cfg(test)]
fn containment(pre: &str, post: &str, row: &EnvelopeRow) -> Containment {
    let (Some(before), Some(after)) = (containment_form(pre, row), containment_form(post, row)) else {
        panic!("test XML must be readable");
    };
    containment_of_forms(before, after, row)
}
fn containment_of_forms(before: ContainmentForm, after: ContainmentForm, row: &EnvelopeRow) -> Containment {
    let insert = row.operations.contains(&"insert");
    let remove = row.operations.contains(&"remove");
    let whole_ok = before.whole == after.whole
        || (insert && !remove && is_subsequence(&before.whole, &after.whole))
        || (remove && !insert && is_subsequence(&after.whole, &before.whole))
        || (insert && remove);
    if before.canonical == after.canonical && whole_ok {
        return Containment::Contained(before.canonical.len());
    }
    if before.canonical != after.canonical {
        return Containment::Escaped(basis_witness("canonical", &before.canonical, &after.canonical));
    }
    // Canonically equal: the escape is the whole-element sequence itself, so the witness
    // shows the first undeclared changed, inserted or removed whole element.
    let sequence = |form: &ContainmentForm| form.whole.iter().map(|(_, text)| text.as_str()).collect::<Vec<_>>().join("\n");
    Containment::Escaped(basis_witness("whole_elements", &sequence(&before), &sequence(&after)))
}
// Insertion invariant of the final barline owner (E004): post is pre plus exactly one
// right light-heavy barline as the last element of the last measure, nothing else.
fn final_barline_geometry(pre: &str, post: &str) -> Option<bool> {
    let after = roxmltree::Document::parse(post).ok()?;
    let inserted = after
        .descendants()
        .filter(|n| n.has_tag_name("barline") && n.attribute("location") == Some("right"))
        .filter(|n| child(*n, "bar-style").and_then(|n| n.text()).map(str::trim) == Some("light-heavy"))
        .collect::<Vec<_>>();
    let before = roxmltree::Document::parse(pre).ok()?;
    let existing = before
        .descendants()
        .filter(|n| n.has_tag_name("barline") && n.attribute("location") == Some("right"))
        .filter(|n| child(*n, "bar-style").and_then(|n| n.text()).map(str::trim) == Some("light-heavy"))
        .count();
    if inserted.len() != existing + 1 {
        return Some(false);
    }
    let Some(last_measure) = after.descendants().rfind(|n| n.has_tag_name("measure")) else { return Some(false) };
    let Some(barline) = inserted.into_iter().find(|n| n.parent() == Some(last_measure)) else { return Some(false) };
    if last_measure.children().rfind(roxmltree::Node::is_element) != Some(barline) {
        return Some(false);
    }
    Some(structure(pre, SscOriginScope::Unproved)? == structure_skipping(post, SscOriginScope::Unproved, Some(barline.range()))?)
}
// An Edit script may surround unchanged wrapper bytes. Its actual changed byte
// subranges must be covered by the declared element/field intervals in both states.
#[cfg(test)]
fn mutable_intervals(xml: &str, row: &EnvelopeRow) -> Option<Vec<std::ops::Range<usize>>> {
    let document = roxmltree::Document::parse(xml).ok()?;
    Some(mutable_intervals_of(&document, row))
}
fn mutable_intervals_of(document: &roxmltree::Document<'_>, row: &EnvelopeRow) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    for node in document.descendants().filter(roxmltree::Node::is_element) {
        let name = node.tag_name().name();
        let has_fields = row.fields.iter().any(|(host, _, _)| *host == name);
        let hosted = node.parent().is_some_and(|parent| parent.is_element() && row.fields.iter().any(|(host, kind, field)| *host == parent.tag_name().name() && *kind == '.' && *field == name));
        if hosted || row.elements.contains(&name) && !has_fields { ranges.push(node.range()); }
        for attribute in node.attributes() {
            if row.fields.iter().any(|(host, kind, field)| *host == name && *kind == '@' && *field == attribute.name()) { ranges.push(attribute.range()); }
        }
    }
    ranges.sort_by_key(|range| (range.start, range.end));
    ranges
}
fn range_is_declared(xml: &str, changed: std::ops::Range<usize>, ranges: &[std::ops::Range<usize>]) -> bool {
    if changed.is_empty() { return true; }
    let mut cursor = changed.start;
    let mut covered = false;
    for range in ranges {
        if range.end <= cursor || range.start >= changed.end { continue; }
        if range.start > cursor && !xml.as_bytes()[cursor..range.start].iter().all(u8::is_ascii_whitespace) { return false; }
        covered = true; cursor = range.end.min(changed.end);
        if cursor == changed.end { return true; }
    }
    covered && xml.as_bytes()[cursor..changed.end].iter().all(u8::is_ascii_whitespace)
}
// Some(None): every changed subrange is declared; Some(Some(..)): the first undeclared
// changed subrange in pre and post document coordinates.
type UndeclaredEdit = Option<(std::ops::Range<usize>, std::ops::Range<usize>)>;
#[cfg(test)]
fn undeclared_edit(pre: &str, post: &str, edits: &[(usize, usize, &str)], row: &EnvelopeRow) -> Option<UndeclaredEdit> {
    let before = mutable_intervals(pre, row)?;
    let after = mutable_intervals(post, row)?;
    undeclared_edit_within(pre, post, edits, &before, &after)
}
fn undeclared_edit_within(pre: &str, post: &str, edits: &[(usize, usize, &str)], before: &[std::ops::Range<usize>], after: &[std::ops::Range<usize>]) -> Option<UndeclaredEdit> {
    let (mut pre_cursor, mut post_cursor) = (0usize, 0usize);
    for &(start, end, replacement) in edits {
        let original = pre.get(start..end)?;
        let post_start = post_cursor.checked_add(start.checked_sub(pre_cursor)?)?;
        let prefix = original.bytes().zip(replacement.bytes()).take_while(|(a,b)| a == b).count();
        let suffix = original.as_bytes()[prefix..].iter().rev().zip(replacement.as_bytes()[prefix..].iter().rev()).take_while(|(a,b)| a == b).count();
        let (pre_changed, post_changed) = (start + prefix..end - suffix, post_start + prefix..post_start + replacement.len() - suffix);
        if !range_is_declared(pre, pre_changed.clone(), before)
            || !range_is_declared(post, post_changed.clone(), after) { return Some(Some((pre_changed, post_changed))); }
        pre_cursor = end; post_cursor = post_start + replacement.len();
    }
    Some(None)
}
// r1290: pre and post are parsed once; the edit check and the canonical containment form
// share the two documents. Unparsable bytes stay ContainmentUnproven, as before.
fn receipt_containment_assessment<'event>(event: &'event SscEventBinding<'event>, row: Option<EnvelopeRow>, receipt: &SscOriginReceiptKind<'_>) -> (Option<SscContainmentProof<'event>>, Option<SscContainmentCounterexample<'event>>) {
    let Some(row) = row else { return (None, None) };
    let (Ok(pre), Ok(post)) = (roxmltree::Document::parse(event.pre_state), roxmltree::Document::parse(event.post_state)) else {
        return (None, None);
    };
    if let SscOriginReceiptKind::Exact(receipt) = receipt
        && row.envelope.kind == SscEnvelopeKind::Edit
    {
        let (before, after) = (mutable_intervals_of(&pre, &row), mutable_intervals_of(&post, &row));
        match undeclared_edit_within(event.pre_state, event.post_state, &crate::master_state_identity::ssc_exact_edit_spans(receipt), &before, &after) {
            Some(Some((pre_range, post_range))) => {
                let witness = difference_witness(event.pre_state, event.post_state, pre_range, post_range);
                return (None, Some(SscContainmentCounterexample { event, envelope: row.envelope, witness }));
            }
            None => return (None, None),
            Some(None) => {},
        }
    }
    match containment_of_forms(containment_form_of(&pre, &row), containment_form_of(&post, &row), &row) {
        Containment::Contained(checked_bytes) => (Some(SscContainmentProof { event, envelope: row.envelope, certificate: SscConstructiveContainmentCertificate { checked_bytes } }), None),
        Containment::Escaped(witness) => (None, Some(SscContainmentCounterexample { event, envelope: row.envelope, witness })),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SscAdoptionDomain {
    Master,
    Render,
}
enum AdoptedReceipt {
    Block(BlockTransformationReceipt),
    Exact(ExactTransformationReceipt),
    Preserving(PreservingTransformationReceipt),
    Additive(AdditiveTransformationReceipt),
}
struct AdoptedEvidence {
    receipt: AdoptedReceipt,
    domain: SscAdoptionDomain,
    additive_origin: Option<SscOriginState>,
}
impl AdoptedReceipt {
    fn identity(&self) -> &MintedIdentity {
        match self {
            Self::Block(receipt) => receipt.ssc_bound_origin().0,
            Self::Exact(receipt) => receipt.ssc_bound_origin().0,
            Self::Preserving(receipt) => receipt.ssc_bound_origin().0,
            Self::Additive(receipt) => crate::master_state_identity::ssc_additive_identity(receipt),
        }
    }
    fn borrowed<'a>(
        &'a self,
        additive_origin: Option<&'a SscOriginState>,
    ) -> Option<(
        SscOriginReceiptKind<'a>,
        &'a MintedIdentity,
        &'a SscOriginState,
    )> {
        match self {
            Self::Block(receipt) => {
                let (identity, origin) = receipt.ssc_bound_origin();
                Some((SscOriginReceiptKind::Block(receipt), identity, origin))
            }
            Self::Exact(receipt) => {
                let (identity, origin) = receipt.ssc_bound_origin();
                Some((SscOriginReceiptKind::Exact(receipt), identity, origin))
            }
            Self::Preserving(receipt) => {
                let (identity, origin) = receipt.ssc_bound_origin();
                Some((SscOriginReceiptKind::Preserving(receipt), identity, origin))
            }
            Self::Additive(receipt) => Some((
                SscOriginReceiptKind::Additive(receipt),
                crate::master_state_identity::ssc_additive_identity(receipt),
                additive_origin?,
            )),
        }
    }
}
#[cfg(not(test))]
static ADOPTED_EVIDENCE: Mutex<Vec<AdoptedEvidence>> = Mutex::new(Vec::new());
#[cfg(test)]
thread_local! { static ADOPTED_EVIDENCE: std::cell::RefCell<Vec<AdoptedEvidence>> = const { std::cell::RefCell::new(Vec::new()) }; }
fn with_records(operation: impl FnOnce(&mut Vec<AdoptedEvidence>)) {
    #[cfg(not(test))]
    operation(
        &mut ADOPTED_EVIDENCE
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()),
    );
    #[cfg(test)]
    ADOPTED_EVIDENCE.with_borrow_mut(operation);
}
pub(crate) fn reset_origin_measurement(_origin: &SscReceiptOriginAccess) {
    with_records(Vec::clear);
    with_pending(Vec::clear);
    with_scopes(|scopes| *scopes = (None, None));
}
// The existing released lineage supplies opaque event identities. This only
// sets the released denominator without deleting committed candidate evidence.
pub(crate) fn retain_released_master_evidence(
    _origin: &SscReceiptOriginAccess,
    identities: &[MintedIdentity],
) {
    with_scopes(|scopes| scopes.0 = Some(identities.to_vec()));
}
macro_rules! record_commit {
    ($function:ident, $type:ty, $variant:ident) => {
        pub(crate) fn $function(
            _origin: &SscReceiptOriginAccess,
            receipt: $type,
            domain: SscAdoptionDomain,
        ) {
            with_records(|records| {
                records.push(AdoptedEvidence {
                    receipt: AdoptedReceipt::$variant(receipt),
                    domain,
                    additive_origin: None,
                })
            });
        }
    };
}
record_commit!(record_block_commit, BlockTransformationReceipt, Block);
record_commit!(record_exact_commit, ExactTransformationReceipt, Exact);
record_commit!(
    record_preserving_commit,
    PreservingTransformationReceipt,
    Preserving
);
#[cfg(not(test))]
static PENDING_ADDITIVE: Mutex<Vec<(MintedIdentity, SscOriginState)>> = Mutex::new(Vec::new());
#[cfg(test)]
thread_local! { static PENDING_ADDITIVE: std::cell::RefCell<Vec<(MintedIdentity, SscOriginState)>> = const { std::cell::RefCell::new(Vec::new()) }; }
fn with_pending(operation: impl FnOnce(&mut Vec<(MintedIdentity, SscOriginState)>)) {
    #[cfg(not(test))]
    operation(
        &mut PENDING_ADDITIVE
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()),
    );
    #[cfg(test)]
    PENDING_ADDITIVE.with_borrow_mut(operation);
}
/// Borrow the original owner's capture without consuming its pending evidence.
/// The sealed access capability and receipt identity bind this read to origin.
pub(crate) fn read_additive_origin(
    _origin: &SscReceiptOriginAccess,
    receipt: &AdditiveTransformationReceipt,
) -> Option<(String, String)> {
    let identity = crate::master_state_identity::ssc_additive_identity(receipt);
    let mut captured = None;
    with_pending(|pending| {
        captured = pending.iter().find(|(existing, _)| existing == identity)
            .map(|(_, origin)| origin.clone());
    });
    let captured = captured?;
    Some((captured.pre_state.exact_bytes()?, captured.post_state.exact_bytes()?))
}

pub(crate) fn register_additive_origin(
    _origin: &SscReceiptOriginAccess,
    identity: &MintedIdentity,
    origin: SscOriginState,
) {
    with_pending(|pending| {
        if !pending.iter().any(|(existing, _)| existing == identity) {
            pending.push((*identity, origin));
        }
    });
}
pub(crate) fn record_additive_commit(
    _origin: &SscReceiptOriginAccess,
    receipt: AdditiveTransformationReceipt,
    domain: SscAdoptionDomain,
) {
    let identity = crate::master_state_identity::ssc_additive_identity(&receipt);
    let mut additive_origin = None;
    with_pending(|pending| {
        if let Some(index) = pending
            .iter()
            .position(|(existing, _)| existing == identity)
        {
            additive_origin = Some(pending.swap_remove(index).1);
        }
    });
    with_records(|records| {
        records.push(AdoptedEvidence {
            receipt: AdoptedReceipt::Additive(receipt),
            domain,
            additive_origin,
        })
    });
}

// Contract v4 A7 (r1287): borrowed receipt view at a wired adoption boundary. It carries no
// authority; it only lets the adoption verdict read the receipt-bound captures before commit.
pub(crate) enum AdoptionReceiptView<'a> {
    Block(&'a BlockTransformationReceipt),
    Exact(&'a ExactTransformationReceipt),
    Preserving(&'a PreservingTransformationReceipt),
    Additive(&'a AdditiveTransformationReceipt),
}
// Contract v4 A7 (r1287): the adoption verdict. It runs the report's own classifier on the
// sealed receipt captures before the receipt is committed. Only Permit admits the adoption;
// every other verdict, integrity failure or replay refuses it with the constructive witness.
// No cache: every call classifies its receipt afresh.
pub(crate) fn ssc_adoption_verdict(_origin: &SscReceiptOriginAccess, receipt: AdoptionReceiptView<'_>, domain: SscAdoptionDomain) -> Result<(), String> {
    let identity = *match &receipt {
        AdoptionReceiptView::Block(receipt) => receipt.ssc_bound_origin().0,
        AdoptionReceiptView::Exact(receipt) => receipt.ssc_bound_origin().0,
        AdoptionReceiptView::Preserving(receipt) => receipt.ssc_bound_origin().0,
        AdoptionReceiptView::Additive(receipt) => crate::master_state_identity::ssc_additive_identity(receipt),
    };
    let mut replayed = false;
    with_records(|records| replayed = records.iter().any(|record| *record.receipt.identity() == identity));
    if replayed {
        return Err("SSC adoption refused: integrity=DuplicateEvidence (receipt identity already adopted)".to_owned());
    }
    let released = SscReleasedEventRef { domain: match domain { SscAdoptionDomain::Master => SscEvidenceDomain::Master, SscAdoptionDomain::Render => SscEvidenceDomain::Render }, identity: &identity };
    let mut counts = ObservationCounts::default();
    match receipt {
        AdoptionReceiptView::Block(receipt) => { let (bound, origin) = receipt.ssc_bound_origin(); classify_bound_origin(released, SscOriginReceiptKind::Block(receipt), bound, origin, &mut counts); }
        AdoptionReceiptView::Exact(receipt) => { let (bound, origin) = receipt.ssc_bound_origin(); classify_bound_origin(released, SscOriginReceiptKind::Exact(receipt), bound, origin, &mut counts); }
        AdoptionReceiptView::Preserving(receipt) => { let (bound, origin) = receipt.ssc_bound_origin(); classify_bound_origin(released, SscOriginReceiptKind::Preserving(receipt), bound, origin, &mut counts); }
        AdoptionReceiptView::Additive(receipt) => with_pending(|pending| match pending.iter().find(|(pending, _)| *pending == identity) {
            Some((_, origin)) => classify_bound_origin(released, SscOriginReceiptKind::Additive(receipt), &identity, origin, &mut counts),
            None => counts.observe(SscReleasedObservationV2::IntegrityFailure { released, integrity: SscEvidenceIntegrityFailure::MissingEvidence }),
        }),
    }
    adoption_decision(&counts)
}
// The A7 decision over the classification of exactly one receipt: Permit and nothing else.
fn adoption_decision(counts: &ObservationCounts) -> Result<(), String> {
    let [out_of_scope, unproven, denied, permitted] = counts.containment;
    if permitted == 1 && counts.missing + counts.duplicate + counts.invalid == 0 {
        return Ok(());
    }
    let integrity = counts.integrity.keys().cloned().collect::<Vec<_>>().join(",");
    let witness = counts.containment_witnesses.first().map_or(String::new(), |line| format!("; {}", line.trim_end()));
    Err(format!("SSC adoption refused: permitted={permitted}; denied={denied}; unproven={unproven}; out_of_scope={out_of_scope}; integrity={integrity}{witness}"))
}

pub(crate) fn observe_ssc_event_v2<'event>(
    transformation: &'event SscTransformationEvidence<'event>,
    invariant: SscInvariantAssessment<'event>,
    necessity: SscNecessityAssessment<'event>,
    containment: SscContainmentAssessment<'event>,
) -> Result<
    SscClassifiedObservationV2<'event>,
    SscEvidenceIntegrityFailure,
> {
    let scope = transformation.scope;
    if let SscFamilyScope::InScope(family @ (SscProofFamily::Edit | SscProofFamily::Block)) = scope {
        let descriptor = match (family, &transformation.receipt.inner) {
            (SscProofFamily::Edit, SscOriginReceiptKind::Exact(receipt)) => crate::master_state_identity::ssc_exact_descriptor_name(crate::master_state_identity::ssc_exact_descriptor(receipt)),
            (SscProofFamily::Block, SscOriginReceiptKind::Block(receipt)) => crate::master_state_identity::ssc_block_descriptor_name(crate::master_state_identity::ssc_block_descriptor(receipt)),
            _ => return Err(SscEvidenceIntegrityFailure::ProofFamilyMismatch),
        };
        let envelope = match containment {
            SscContainmentAssessment::Contained(proof) => Some(proof.envelope),
            SscContainmentAssessment::Escaped(counterexample) => Some(counterexample.envelope),
            SscContainmentAssessment::Unproven => None,
        };
        if let Some(envelope) = envelope {
            let expected = if family == SscProofFamily::Edit { SscEnvelopeKind::Edit } else { SscEnvelopeKind::Block };
            if descriptor != Some(envelope.descriptor) || envelope.kind != expected {
                return Err(SscEvidenceIntegrityFailure::ProofFamilyMismatch);
            }
        }
    }
    let same_event = |event: &SscEventBinding<'event>| {
        std::ptr::eq(transformation.event, event)
    };
    // V2 classification stays as the necessity measurement (contract v4 A2).
    let verdict = match (scope, invariant, necessity) {
        (SscFamilyScope::OutOfScope, SscInvariantAssessment::Unproven, SscNecessityAssessment::Unproven) => SscPermitVerdictV2::OutOfScope,
        (SscFamilyScope::OutOfScope, _, _) => return Err(SscEvidenceIntegrityFailure::IllegalProofCombination),
        (SscFamilyScope::InScope(_), SscInvariantAssessment::Unproven, SscNecessityAssessment::Unproven) => SscPermitVerdictV2::InvariantUnproven,
        (SscFamilyScope::InScope(_), SscInvariantAssessment::Unproven | SscInvariantAssessment::Refuted(_), SscNecessityAssessment::Proven(_)) => {
            return Err(SscEvidenceIntegrityFailure::IllegalProofCombination)
        }
        (SscFamilyScope::InScope(family), SscInvariantAssessment::Refuted(counterexample), SscNecessityAssessment::Unproven) => {
            if !same_event(counterexample.event) {
                return Err(SscEvidenceIntegrityFailure::ProofBindingMismatch);
            }
            if counterexample.family != family {
                return Err(SscEvidenceIntegrityFailure::ProofFamilyMismatch);
            }
            SscPermitVerdictV2::InvariantDenied
        }
        (SscFamilyScope::InScope(family), SscInvariantAssessment::Proven(proof), SscNecessityAssessment::Unproven) => {
            if !same_event(proof.event) {
                return Err(SscEvidenceIntegrityFailure::ProofBindingMismatch);
            }
            if proof.family != family {
                return Err(SscEvidenceIntegrityFailure::ProofFamilyMismatch);
            }
            SscPermitVerdictV2::NecessityUnproven
        }
        (SscFamilyScope::InScope(family), SscInvariantAssessment::Proven(proof), SscNecessityAssessment::Proven(witness)) => {
            if !same_event(proof.event) || !same_event(witness.event) {
                return Err(SscEvidenceIntegrityFailure::ProofBindingMismatch);
            }
            if proof.family != family || witness.family != family {
                return Err(SscEvidenceIntegrityFailure::ProofFamilyMismatch);
            }
            SscPermitVerdictV2::Permit
        }
    };
    // Keep the legacy V2 coverage/necessity measurement unchanged. New families
    // close V4 coverage only; their proofs above still undergo full event/family binding.
    let verdict = if matches!(scope, SscFamilyScope::InScope(SscProofFamily::Edit | SscProofFamily::Block)) { SscPermitVerdictV2::OutOfScope } else { verdict };
    // Contract v4 A2: Permit means proven invariant plus proven containment in the
    // descriptor's frozen envelope; necessity never decides it.
    let containment_verdict = match (scope, containment) {
        (SscFamilyScope::OutOfScope, SscContainmentAssessment::Unproven) => SscContainmentVerdict::OutOfScope,
        (SscFamilyScope::OutOfScope, _) => return Err(SscEvidenceIntegrityFailure::IllegalProofCombination),
        (SscFamilyScope::InScope(_), SscContainmentAssessment::Escaped(counterexample)) => {
            if !same_event(counterexample.event) {
                return Err(SscEvidenceIntegrityFailure::ProofBindingMismatch);
            }
            SscContainmentVerdict::ContainmentDenied
        }
        (SscFamilyScope::InScope(_), SscContainmentAssessment::Contained(proof)) => {
            if !same_event(proof.event) {
                return Err(SscEvidenceIntegrityFailure::ProofBindingMismatch);
            }
            match invariant {
                SscInvariantAssessment::Proven(_) => SscContainmentVerdict::Permit,
                SscInvariantAssessment::Refuted(_) => SscContainmentVerdict::ContainmentDenied,
                SscInvariantAssessment::Unproven => SscContainmentVerdict::ContainmentUnproven,
            }
        }
        (SscFamilyScope::InScope(_), SscContainmentAssessment::Unproven) => match invariant {
            SscInvariantAssessment::Refuted(_) => SscContainmentVerdict::ContainmentDenied,
            _ => SscContainmentVerdict::ContainmentUnproven,
        },
    };
    Ok(SscClassifiedObservationV2 {
        transformation,
        scope,
        invariant,
        necessity,
        verdict,
        containment,
        containment_verdict,
    })
}


// Released domains are independent; identity values are never decoded or reminted.
type ReleasedScopes = (Option<Vec<MintedIdentity>>, Option<Vec<MintedIdentity>>);
#[cfg(not(test))]
static RELEASED_SCOPES: Mutex<ReleasedScopes> = Mutex::new((None, None));
#[cfg(test)]
thread_local! { static RELEASED_SCOPES: std::cell::RefCell<ReleasedScopes> = const { std::cell::RefCell::new((None, None)) }; }
fn with_scopes(operation: impl FnOnce(&mut ReleasedScopes)) {
    #[cfg(not(test))]
    operation(&mut RELEASED_SCOPES.lock().unwrap_or_else(|poisoned| poisoned.into_inner()));
    #[cfg(test)]
    RELEASED_SCOPES.with_borrow_mut(operation);
}
pub(crate) fn retain_released_render_evidence(_origin: &SscReceiptOriginAccess, identities: Option<&[MintedIdentity]>) {
    with_scopes(|scopes| scopes.1 = identities.map(<[MintedIdentity]>::to_vec));
}
fn proof_scope(scope: SscOriginScope) -> SscFamilyScope {
    match scope {
        SscOriginScope::F7 => SscFamilyScope::InScope(SscProofFamily::F7RestVoiceStrictStaff),
        SscOriginScope::F5 => SscFamilyScope::InScope(SscProofFamily::F5AtomicVoiceStaffVisibility),
        SscOriginScope::F9 => SscFamilyScope::InScope(SscProofFamily::F9AuthoritySerialization),
        SscOriginScope::FinalBarline | SscOriginScope::DirectionInsertion => SscFamilyScope::InScope(SscProofFamily::Additive),
        SscOriginScope::Preserving(_) => SscFamilyScope::InScope(SscProofFamily::Preserving),
        SscOriginScope::Unproved => SscFamilyScope::OutOfScope,
    }
}
impl SscOriginReceiptKind<'_> {
    fn bound_identity(&self) -> &MintedIdentity {
        match self {
            Self::Block(receipt) => receipt.ssc_bound_origin().0,
            Self::Exact(receipt) => receipt.ssc_bound_origin().0,
            Self::Preserving(receipt) => receipt.ssc_bound_origin().0,
            Self::Additive(receipt) => crate::master_state_identity::ssc_additive_identity(receipt),
        }
    }
    fn fingerprints(&self, pre: &str, post: &str) -> (bool, bool) {
        match self {
            Self::Block(receipt) => crate::master_state_identity::ssc_block_origin_fingerprints(receipt, pre, post),
            Self::Exact(receipt) => crate::master_state_identity::ssc_exact_origin_fingerprints(receipt, pre, post),
            Self::Preserving(receipt) => crate::master_state_identity::ssc_preserving_origin_fingerprints(receipt, pre, post),
            Self::Additive(receipt) => crate::master_state_identity::ssc_additive_origin_fingerprints(receipt, pre, post),
        }
    }
}
#[derive(Default)]
struct ObservationCounts {
    released: usize,
    master: usize,
    render: usize,
    missing: usize,
    duplicate: usize,
    invalid: usize,
    out_of_scope: usize,
    invariant_unproven: usize,
    invariant_denied: usize,
    necessity_unproven: usize,
    permit: usize,
    integrity: std::collections::BTreeMap<String, usize>,
    // Contract v4 verdicts: [OutOfScope, ContainmentUnproven, ContainmentDenied, Permit].
    containment: [usize; 4],
    containment_kinds: std::collections::BTreeMap<&'static str, [usize; 4]>,
    containment_denied: std::collections::BTreeMap<&'static str, usize>,
    containment_witnesses: Vec<String>,
    content_permits: usize,
    families: std::collections::BTreeMap<&'static str, [usize; 4]>,
    scopes: std::collections::BTreeMap<&'static str, [usize; 4]>,
}
impl ObservationCounts {
    fn fail(&mut self, reason: SscEvidenceIntegrityFailure) {
        match reason {
            SscEvidenceIntegrityFailure::MissingEvidence => self.missing += 1,
            SscEvidenceIntegrityFailure::DuplicateEvidence => self.duplicate += 1,
            _ => self.invalid += 1,
        }
        *self.integrity.entry(format!("{reason:?}")).or_default() += 1;
    }
    fn observe(&mut self, observation: SscReleasedObservationV2<'_>) {
        match observation {
            SscReleasedObservationV2::IntegrityFailure { released, integrity } => {
                let _bound_domain = released.domain;
                let _opaque_identity = released.identity;
                self.fail(integrity);
            }
            SscReleasedObservationV2::Classified(observation) => {
                // Access every constructive component without exposing it to writers.
                let binding = observation.transformation.event;
                let _exact_origin = (binding.released.domain, binding.released.identity, binding.pre_state, binding.post_state);
                let _original_receipt = observation.transformation.receipt.inner.bound_identity();
                if let SscFamilyScope::InScope(family) = observation.scope {
                    let label = match family { SscProofFamily::F7RestVoiceStrictStaff => "F7", SscProofFamily::F5AtomicVoiceStaffVisibility => "F5", SscProofFamily::F9AuthoritySerialization => "F9", SscProofFamily::Additive => "ADDITIVE", SscProofFamily::Preserving => "PRESERVING", SscProofFamily::Edit => "EDIT", SscProofFamily::Block => "BLOCK" };
                    let slot = match observation.verdict { SscPermitVerdictV2::InvariantUnproven => Some(0), SscPermitVerdictV2::InvariantDenied => Some(1), SscPermitVerdictV2::NecessityUnproven => Some(2), SscPermitVerdictV2::Permit => Some(3), SscPermitVerdictV2::OutOfScope => None };
                    if let Some(slot) = slot { self.families.entry(label).or_default()[slot] += 1; }
                }
                match observation.invariant {
                    SscInvariantAssessment::Unproven => {},
                    SscInvariantAssessment::Proven(proof) => { let _checked = proof.certificate.preserved_structure.normalized_bytes; },
                    SscInvariantAssessment::Refuted(counterexample) => {
                        let exact = &counterexample.counterexample;
                        let _witness = (exact.kind, &exact.witness.pre_range, &exact.witness.post_range, &exact.witness.pre_bytes, &exact.witness.post_bytes);
                    },
                }
                if let SscNecessityAssessment::Proven(witness) = observation.necessity {
                    let _defect = match witness.certificate.defect {
                        SscStructuralDefect::InvisibleUncoveredRest { note }
                        | SscStructuralDefect::MissingRequestedStaff { note }
                        | SscStructuralDefect::MissingRequestedSerialization { note } => note,
                        SscStructuralDefect::MissingFinalBarline { boundary } => boundary,
                        SscStructuralDefect::BrokenBeamChain { note } => note,
                    };
                }
                let slot = match observation.containment_verdict {
                    SscContainmentVerdict::OutOfScope => 0,
                    SscContainmentVerdict::ContainmentUnproven => 1,
                    SscContainmentVerdict::ContainmentDenied => 2,
                    SscContainmentVerdict::Permit => 3,
                };
                self.containment[slot] += 1;
                if observation.containment_verdict == SscContainmentVerdict::ContainmentDenied {
                    let witness = match (observation.containment, observation.invariant) {
                        (SscContainmentAssessment::Escaped(counterexample), _) => Some((counterexample.envelope.descriptor, "envelope_escape", &counterexample.witness)),
                        (SscContainmentAssessment::Contained(proof), SscInvariantAssessment::Refuted(counterexample)) => Some((proof.envelope.descriptor, "locality_or_invariant", &counterexample.counterexample.witness)),
                        (_, SscInvariantAssessment::Refuted(counterexample)) => Some((receipt_envelope_descriptor(&observation.transformation.receipt.inner, SscOriginScope::Unproved).unwrap_or("UNRESOLVED_DESCRIPTOR"), "locality_or_invariant", &counterexample.counterexample.witness)),
                        _ => None,
                    };
                    if let Some((descriptor, reason, witness)) = witness {
                        self.containment_witnesses.push(format!(
                            "V4DeniedWitness: descriptor={descriptor}; reason={reason}; basis={}; pre_range={:?}; post_range={:?}; pre_bound_bytes={}; post_bound_bytes={}; pre_excerpt={:?}; post_excerpt={:?}\n",
                            witness_basis(&witness.pre_bytes), witness.pre_range, witness.post_range, witness.pre_bytes.len(), witness.post_bytes.len(),
                            witness_excerpt(&witness.pre_bytes, &witness.pre_range), witness_excerpt(&witness.post_bytes, &witness.post_range),
                        ));
                    }
                }
                if observation.containment_verdict == SscContainmentVerdict::Permit
                    && let SscContainmentAssessment::Contained(proof) = observation.containment
                    && include_str!("build_gate/ssc_containment_envelopes_v1.tsv").lines().filter_map(|line| line.strip_prefix("row=")).any(|row| {
                        let fields = row.split('|').collect::<Vec<_>>(); fields.len() == 9 && fields[0] == proof.envelope.descriptor && fields[7] == "yes"
                    })
                { self.content_permits += 1; }

                let envelope = match observation.containment {
                    SscContainmentAssessment::Unproven => None,
                    SscContainmentAssessment::Contained(proof) => {
                        let _checked = (proof.event.released.identity, proof.certificate.checked_bytes);
                        Some(proof.envelope)
                    }
                    SscContainmentAssessment::Escaped(counterexample) => {
                        let exact = &counterexample.witness;
                        let _witness = (counterexample.event.released.identity, &exact.pre_range, &exact.post_range, &exact.pre_bytes, &exact.post_bytes);
                        Some(counterexample.envelope)
                    }
                };
                if let Some(envelope) = envelope {
                    let kind = match envelope.kind {
                        SscEnvelopeKind::Projection => "PROJECTION",
                        SscEnvelopeKind::Insertion => "INSERTION",
                        SscEnvelopeKind::Edit => "EDIT",
                        SscEnvelopeKind::Lane => "LANE",
                        SscEnvelopeKind::Block => "BLOCK",
                    };
                    self.containment_kinds.entry(kind).or_default()[slot] += 1;
                    if slot == 2 {
                        *self.containment_denied.entry(envelope.descriptor).or_default() += 1;
                    }
                }
                match observation.verdict {
                    SscPermitVerdictV2::OutOfScope => self.out_of_scope += 1,
                    SscPermitVerdictV2::InvariantUnproven => self.invariant_unproven += 1,
                    SscPermitVerdictV2::InvariantDenied => self.invariant_denied += 1,
                    SscPermitVerdictV2::NecessityUnproven => self.necessity_unproven += 1,
                    SscPermitVerdictV2::Permit => self.permit += 1,
                }
            }
        }
    }
    fn classified(&self) -> usize {
        self.out_of_scope + self.invariant_unproven + self.invariant_denied + self.necessity_unproven + self.permit
    }
}
fn invariant_from_exact_origin<'event>(event: &'event SscEventBinding<'event>, origin: &SscOriginState, family: SscProofFamily) -> (Option<SscInvariantProof<'event>>, Option<SscInvariantCounterexample<'event>>, Option<SscNecessityWitness<'event>>) {
    match origin.scope {
        SscOriginScope::DirectionInsertion => return direction_assessment(event, family),
        SscOriginScope::Preserving(preserving) => return preserving_assessment(event, preserving, family),
        _ => {}
    }
    // Fresh constructive issuance from the receipt-bound bytes, not V1 verdict reuse.
    if let Some((checked_family, certificate, necessity)) = prove_origin(event.pre_state, event.post_state, origin.scope) {
        if checked_family != family { return (None, None, None); }
        return (Some(SscInvariantProof { event, family, certificate }), None, Some(SscNecessityWitness { event, family, certificate: necessity }));
    }
    if matches!(origin.scope, SscOriginScope::FinalBarline) && family == SscProofFamily::Additive {
        // Contract v4: the insertion invariant stands on its own; necessity is measured.
        return match final_barline_geometry(event.pre_state, event.post_state) {
            Some(true) => {
                let certificate = SscConstructiveInvariantCertificate { preserved_structure: SscCheckedStructurePreservation { normalized_bytes: event.pre_state.len() } };
                (Some(SscInvariantProof { event, family, certificate }), None, None)
            }
            Some(false) => {
                let witness = minimal_witness(event.pre_state, event.post_state);
                let counterexample = SscConstructiveInvariantCounterexample { kind: SscInvariantFailureKind::RequiredInsertionGeometryBroken, witness };
                (None, Some(SscInvariantCounterexample { event, family, counterexample }), None)
            }
            None => (None, None, None),
        };
    }
    let before = structure(event.pre_state, origin.scope);
    let after = structure(event.post_state, origin.scope);
    let (Some(before), Some(after)) = (before, after) else { return (None, None, None); };
    if matches!(family, SscProofFamily::Additive) && event.pre_state != event.post_state {
        // An absent necessity certificate is not a refutation of insertion geometry.
        return (None, None, None);
    }
    if before == after && !matches!(family, SscProofFamily::Additive) {
        return (Some(SscInvariantProof { event, family, certificate: SscConstructiveInvariantCertificate { preserved_structure: SscCheckedStructurePreservation { normalized_bytes: before.len() } } }), None, None);
    }
    let note_count = |xml| roxmltree::Document::parse(xml).ok().map(|document| document.descendants().filter(|node| node.has_tag_name("note")).count());
    let kind = if matches!(family, SscProofFamily::Additive) {
        SscInvariantFailureKind::RequiredInsertionGeometryBroken
    } else if note_count(event.pre_state) != note_count(event.post_state) {
        SscInvariantFailureKind::NoteCardinalityChanged
    } else if matches!(family, SscProofFamily::F5AtomicVoiceStaffVisibility | SscProofFamily::F9AuthoritySerialization) {
        SscInvariantFailureKind::ForbiddenFieldChanged
    } else {
        SscInvariantFailureKind::PreservedProjectionChanged
    };
    let counterexample = SscInvariantCounterexample { event, family, counterexample: SscConstructiveInvariantCounterexample { kind, witness: minimal_witness(event.pre_state, event.post_state) } };
    (None, Some(counterexample), None)
}
fn classify_record(released: SscReleasedEventRef<'_>, record: &AdoptedEvidence, counts: &mut ObservationCounts) {
    let Some((receipt, identity, origin)) = record.receipt.borrowed(record.additive_origin.as_ref()) else {
        counts.observe(SscReleasedObservationV2::IntegrityFailure { released, integrity: SscEvidenceIntegrityFailure::MissingEvidence }); return;
    };
    classify_bound_origin(released, receipt, identity, origin, counts);
}
fn classify_bound_origin(released: SscReleasedEventRef<'_>, receipt: SscOriginReceiptKind<'_>, identity: &MintedIdentity, origin: &SscOriginState, counts: &mut ObservationCounts) {
    let failed = |counts: &mut ObservationCounts, integrity| counts.observe(SscReleasedObservationV2::IntegrityFailure { released, integrity });
    if identity != released.identity { failed(counts, SscEvidenceIntegrityFailure::ReceiptIdentityMismatch); return; }
    if !scope_bound_to_receipt(&receipt, origin.scope) { failed(counts, SscEvidenceIntegrityFailure::ProofFamilyMismatch); return; }
    let Some(pre) = origin.pre_state.exact_bytes() else { failed(counts, SscEvidenceIntegrityFailure::PreStateUnreadable); return; };
    let Some(post) = origin.post_state.exact_bytes() else { failed(counts, SscEvidenceIntegrityFailure::PostStateUnreadable); return; };
    let (pre_matches, post_matches) = receipt.fingerprints(&pre, &post);
    if !pre_matches { failed(counts, SscEvidenceIntegrityFailure::PreFingerprintMismatch); return; }
    if !post_matches { failed(counts, SscEvidenceIntegrityFailure::PostFingerprintMismatch); return; }
    let event = SscEventBinding { released, pre_state: &pre, post_state: &post };
    let transformation = SscTransformationEvidence { event: &event, scope: receipt_proof_scope(&receipt, origin.scope), receipt: SscOriginReceiptRef { inner: receipt } };
    let integrity = SscEvidenceIntegrity::Bound(&transformation);
    let SscEvidenceIntegrity::Bound(bound) = integrity else { unreachable!() };
    let (proof, counterexample, witness) = match bound.scope {
        SscFamilyScope::OutOfScope => (None, None, None),
        SscFamilyScope::InScope(family @ (SscProofFamily::Edit | SscProofFamily::Block)) => receipt_locality_assessment(&event, &bound.receipt.inner, family),
        SscFamilyScope::InScope(family) => invariant_from_exact_origin(&event, origin, family),
    };
    if proof.is_some() && counterexample.is_some() { failed(counts, SscEvidenceIntegrityFailure::ContradictoryInvariantEvidence); return; }
    let invariant = match (&proof, &counterexample) {
        (Some(proof), None) => SscInvariantAssessment::Proven(proof),
        (None, Some(counterexample)) => SscInvariantAssessment::Refuted(counterexample),
        _ => SscInvariantAssessment::Unproven,
    };
    let necessity = witness.as_ref().map_or(SscNecessityAssessment::Unproven, SscNecessityAssessment::Proven);
    let label = scope_label(origin.scope, &pre, &post);
    let row = match bound.scope {
        SscFamilyScope::InScope(_) => receipt_envelope_descriptor(&bound.receipt.inner, origin.scope).and_then(envelope_row),
        SscFamilyScope::OutOfScope => None,
    };
    let (contained, escaped) = receipt_containment_assessment(&event, row, &bound.receipt.inner);
    let containment = match (&contained, &escaped) {
        (Some(proof), _) => SscContainmentAssessment::Contained(proof),
        (None, Some(counterexample)) => SscContainmentAssessment::Escaped(counterexample),
        (None, None) => SscContainmentAssessment::Unproven,
    };
    match observe_ssc_event_v2(bound, invariant, necessity, containment) {
        Ok(classified) => {
            let slot = match classified.verdict { SscPermitVerdictV2::InvariantUnproven => Some(0), SscPermitVerdictV2::InvariantDenied => Some(1), SscPermitVerdictV2::NecessityUnproven => Some(2), SscPermitVerdictV2::Permit => Some(3), SscPermitVerdictV2::OutOfScope => None };
            if let (Some(label), Some(slot)) = (label, slot) { counts.scopes.entry(label).or_default()[slot] += 1; }
            counts.observe(SscReleasedObservationV2::Classified(classified));
        }
        Err(integrity) => failed(counts, integrity),
    }
}
fn count_released(scopes: &ReleasedScopes, records: &[&AdoptedEvidence]) -> ObservationCounts {
    let mut counts = ObservationCounts::default();
    for (domain, identities) in [(SscAdoptionDomain::Master, &scopes.0), (SscAdoptionDomain::Render, &scopes.1)] {
        let Some(identities) = identities else { continue; };
        let mut seen = Vec::new();
        for identity in identities {
            if seen.contains(identity) { continue; }
            seen.push(*identity);
            counts.released += 1;
            match domain { SscAdoptionDomain::Master => counts.master += 1, SscAdoptionDomain::Render => counts.render += 1 };
            let released = SscReleasedEventRef { domain: match domain { SscAdoptionDomain::Master => SscEvidenceDomain::Master, SscAdoptionDomain::Render => SscEvidenceDomain::Render }, identity };
            let matching: Vec<_> = records.iter().filter(|record| record.domain == domain && record.receipt.identity() == identity).collect();
            let integrity = match matching.len() {
                0 if records.iter().any(|record| record.receipt.identity() == identity) => SscEvidenceIntegrity::Invalid { reason: SscEvidenceIntegrityFailure::DomainMismatch },
                0 => SscEvidenceIntegrity::Missing,
                1 => { classify_record(released, matching[0], &mut counts); continue; },
                records => SscEvidenceIntegrity::Duplicate { records },
            };
            let failure = match integrity {
                SscEvidenceIntegrity::Missing => SscEvidenceIntegrityFailure::MissingEvidence,
                SscEvidenceIntegrity::Duplicate { records } => { assert!(records > 1); SscEvidenceIntegrityFailure::DuplicateEvidence },
                SscEvidenceIntegrity::Invalid { reason } => reason,
                SscEvidenceIntegrity::Bound(_) => SscEvidenceIntegrityFailure::BindingMismatch,
            };
            counts.observe(SscReleasedObservationV2::IntegrityFailure { released, integrity: failure });
        }
    }
    counts
}
// Every unreleased record is one checked commit. Duplicate identities are
// integrity failures for each affected record, never silently deduplicated.
// r1289: records come into existence only through the receipt commit, which only the two
// wired A7 boundaries call after `ssc_adoption_verdict` returned Permit. An unreleased
// record is therefore counted as admitted by that verdict and not classified a second time.
#[derive(Default)]
struct UnreleasedCounts {
    checked: usize,
    observations: ObservationCounts,
}
fn count_unreleased(scopes: &ReleasedScopes, records: &[&AdoptedEvidence]) -> UnreleasedCounts {
    let mut counts = UnreleasedCounts::default();
    let master = scopes.0.as_ref().into_iter().flatten().collect::<std::collections::BTreeSet<_>>();
    let render = scopes.1.as_ref().into_iter().flatten().collect::<std::collections::BTreeSet<_>>();
    let unreleased = records.iter().copied().filter(|record| {
        let own_scope = match record.domain { SscAdoptionDomain::Master => &master, SscAdoptionDomain::Render => &render };
        !own_scope.contains(record.receipt.identity())
    }).collect::<Vec<_>>();
    let mut multiplicities = std::collections::BTreeMap::new();
    for record in &unreleased {
        *multiplicities.entry(record.receipt.identity()).or_insert(0_usize) += 1;
    }
    for record in unreleased {
        counts.checked += 1;
        let identity = record.receipt.identity();
        let other_scope = match record.domain {
            SscAdoptionDomain::Master => &render,
            SscAdoptionDomain::Render => &master,
        };
        if multiplicities.get(identity).copied().unwrap_or_default() > 1 {
            counts.observations.fail(SscEvidenceIntegrityFailure::DuplicateEvidence);
        } else if other_scope.contains(identity) {
            counts.observations.fail(SscEvidenceIntegrityFailure::DomainMismatch);
        } else {
            counts.observations.containment[3] += 1;
        }
    }
    counts
}
fn append_unreleased_coverage(scopes: &ReleasedScopes, released: &ObservationCounts, counts: &UnreleasedCounts, out: &mut String) {
    let observed = &counts.observations;
    let [out_of_scope, unproven, denied, permitted] = observed.containment;
    let integrity = observed.missing + observed.duplicate + observed.invalid;
    out.push_str(&format!("V4UnreleasedCommits: checked={}; permitted={permitted}; denied={denied}; unproven={unproven}; out_of_scope={out_of_scope}; integrity={integrity}\nV4UnreleasedBasis: adoption_verdict_at_commit\n", counts.checked));
    let all_permitted = scopes.0.is_some() && scopes.1.is_some()
        && released.missing + released.duplicate + released.invalid == 0
        && released.containment[3] == released.released
        && integrity == 0 && permitted == counts.checked;
    out.push_str(&format!("V4AllCommitsPermitted: {}\n", if all_permitted { "yes" } else { "no" }));
}
pub(crate) fn append_capture_coverage(_access: &crate::final_report::SscObservationReadAccess, out: &mut String) {
    let mut scopes = (None, None);
    with_scopes(|released| scopes = released.clone());
    let mut counts = ObservationCounts::default();
    let mut unreleased = UnreleasedCounts::default();
    with_records(|records| {
        let records = records.iter().collect::<Vec<_>>();
        counts = count_released(&scopes, &records);
        unreleased = count_unreleased(&scopes, &records);
    });
    let missing_scope = usize::from(scopes.0.is_none()) + usize::from(scopes.1.is_none());
    let balance = counts.released == counts.missing + counts.duplicate + counts.invalid + counts.classified();
    out.push_str(&format!("ReleaseScopeMissing: {missing_scope}\nMasterReleaseScope: {}\nRenderReleaseScope: {}\nReleasedOwnerEvents: {}\nMasterOwnerEvents: {}\nRenderOwnerEvents: {}\nMissingEvidence: {}\nDuplicateEvidenceIdentities: {}\nInvalidEvidenceIdentities: {}\nOutOfScope: {}\nInvariantUnproven: {}\nInvariantDenied: {}\nNecessityUnproven: {}\nPermit: {}\nClassifiedOwnerEvents: {}\nReleasedBalance: {}\nEnforcementReadiness: active_at_adoption\n", if scopes.0.is_some() { "released" } else { "missing" }, if scopes.1.is_some() { "released" } else { "missing" }, counts.released, counts.master, counts.render, counts.missing, counts.duplicate, counts.invalid, counts.out_of_scope, counts.invariant_unproven, counts.invariant_denied, counts.necessity_unproven, counts.permit, counts.classified(), if balance { "complete" } else { "invalid" }));
    for (reason, count) in &counts.integrity { out.push_str(&format!("Integrity{reason}: {count}\n")); }
    for family in ["F7", "F5", "F9", "ADDITIVE", "PRESERVING"] {
        let classes = counts.families.get(family).copied().unwrap_or_default();
        for (label, count) in ["InvariantUnproven", "InvariantDenied", "NecessityUnproven", "Permit"].into_iter().zip(classes) {
            out.push_str(&format!("Family{family}{label}: {count}\n"));
        }
    }
    // Contract v4 A2 verdicts (r1287: enforced at the A7 adoption boundaries). The V2 lines
    // above remain the necessity measurement.
    let [outside, unproven, denied, permitted] = counts.containment;
    out.push_str(&format!("V4VerdictMeaning: permit_is_integrity_plus_containment; necessity_is_measurement_only\nV4OutOfScope: {outside}\nV4ContainmentUnproven: {unproven}\nV4ContainmentDenied: {denied}\nV4Permitted: {permitted}\nV4Balance: {}\n", if outside + unproven + denied + permitted == counts.classified() { "complete" } else { "invalid" }));
    for kind in ["PROJECTION", "INSERTION", "EDIT", "LANE", "BLOCK"] {
        let [_, unproven, denied, permitted] = counts.containment_kinds.get(kind).copied().unwrap_or_default();
        out.push_str(&format!("V4Envelope{kind}: unproven={unproven}; denied={denied}; permitted={permitted}\n"));
    }
    out.push_str(&format!("V4ContentChangingPermitted: {}\n", counts.content_permits));
    for witness in &counts.containment_witnesses { out.push_str(witness); }
    for (descriptor, count) in &counts.containment_denied {
        out.push_str(&format!("V4DeniedDescriptor{descriptor}: {count}\n"));
    }
    for scope in ["PRESERVING_BEAM_CHAIN", "PRESERVING_UNBOUND", "DIRECTION_TEMPO", "DIRECTION_WORDS", "DIRECTION_DYNAMICS", "DIRECTION_OTHER"] {
        let classes = counts.scopes.get(scope).copied().unwrap_or_default();
        for (label, count) in ["InvariantUnproven", "InvariantDenied", "NecessityUnproven", "Permit"].into_iter().zip(classes) {
            out.push_str(&format!("Scope{scope}{label}: {count}\n"));
        }
    }
    append_unreleased_coverage(&scopes, &counts, &unreleased, out);
}

#[cfg(test)]
mod truth_closure_tests {
    use super::*;
    fn record() -> AdoptedEvidence {
        let receipt = crate::master_state_identity::tests::synthetic_exact_receipt_for_tests("<d>0</d>", "<d>1</d>", 3, 4, "1");
        AdoptedEvidence { receipt: AdoptedReceipt::Exact(receipt), domain: SscAdoptionDomain::Master, additive_origin: None }
    }
    #[test]
    fn unreleased_duplicates_count_every_record_as_integrity_without_changing_released() {
        let record = record();
        let scopes = (Some(Vec::new()), Some(Vec::new()));
        let counts = count_unreleased(&scopes, &[&record, &record]);
        assert_eq!((counts.checked, counts.observations.duplicate, counts.observations.classified()), (2, 2, 0));
        let released = count_released(&scopes, &[&record, &record]);
        assert_eq!((released.released, released.duplicate), (0, 0));
        let mut report = String::new();
        append_unreleased_coverage(&scopes, &released, &counts, &mut report);
        assert!(report.contains("V4UnreleasedCommits: checked=2; permitted=0; denied=0; unproven=0; out_of_scope=0; integrity=2\n"));
        assert!(report.contains("V4AllCommitsPermitted: no\n"));
    }
    #[test]
    fn unreleased_scope_is_domain_bound_and_missing_released_prevents_all_permitted() {
        let record = record();
        let identity = *record.receipt.identity();
        let scopes = (Some(Vec::new()), Some(vec![identity]));
        let counts = count_unreleased(&scopes, &[&record]);
        assert_eq!((counts.checked, counts.observations.invalid), (1, 1));
        assert_eq!(counts.observations.integrity.get("DomainMismatch"), Some(&1));
        let released = count_released(&scopes, &[&record]);
        assert_eq!((released.released, released.invalid), (1, 1));
        let missing = count_released(&(Some(vec![identity]), Some(Vec::new())), &[]);
        let mut report = String::new();
        append_unreleased_coverage(&scopes, &missing, &UnreleasedCounts::default(), &mut report);
        assert!(report.contains("V4AllCommitsPermitted: no\n"));
        report.clear();
        append_unreleased_coverage(&(None, None), &ObservationCounts::default(), &UnreleasedCounts::default(), &mut report);
        assert!(report.contains("V4AllCommitsPermitted: no\n"));
    }
    #[test]
    fn unreleased_records_are_counted_as_admitted_at_commit_and_never_reclassified() {
        // r1289: a record exists only after the A7 adoption verdict admitted it; the report
        // does not classify it again, even if its content would not be provable today.
        let record = record();
        let scopes = (Some(Vec::new()), Some(Vec::new()));
        let released = count_released(&scopes, &[&record]);
        let counts = count_unreleased(&scopes, &[&record]);
        assert_eq!((counts.checked, counts.observations.containment, counts.observations.classified()), (1, [0, 0, 0, 1], 0));
        let mut report = String::new();
        append_unreleased_coverage(&scopes, &released, &counts, &mut report);
        assert!(report.contains("V4UnreleasedCommits: checked=1; permitted=1; denied=0; unproven=0; out_of_scope=0; integrity=0\nV4UnreleasedBasis: adoption_verdict_at_commit\n"), "{report}");
        assert!(report.contains("V4AllCommitsPermitted: yes\n"), "{report}");
        let (receipt, identity, origin) = record.receipt.borrowed(None).unwrap();
        let mut corrupt = origin.clone();
        corrupt.pre_state = CapturedPreState::Deflated(Arc::from([0_u8, 1, 2]));
        let mut observations = ObservationCounts::default();
        classify_bound_origin(SscReleasedEventRef { domain: SscEvidenceDomain::Master, identity }, receipt, identity, &corrupt, &mut observations);
        assert_eq!(observations.integrity.get("PreStateUnreadable"), Some(&1));
    }
    #[test]
    fn released_denominator_survives_missing_record_and_excludes_orphans() {
        let record = record();
        let identity = *record.receipt.identity();
        let released = (Some(vec![identity]), Some(Vec::new()));
        let missing = count_released(&released, &[]);
        assert_eq!((missing.released, missing.missing, missing.classified()), (1, 1, 0));
        let orphan = count_released(&(Some(Vec::new()), Some(Vec::new())), &[&record]);
        assert_eq!((orphan.released, orphan.classified()), (0, 0));
    }
    #[test]
    fn no_release_scope_has_no_candidate_fallback() {
        let record = record();
        let missing = count_released(&(None, None), &[&record]);
        assert_eq!((missing.released, missing.classified()), (0, 0));
    }
    #[test]
    fn duplicate_records_are_one_released_integrity_class() {
        let record = record();
        let identity = *record.receipt.identity();
        let duplicate = count_released(&(Some(vec![identity, identity]), Some(Vec::new())), &[&record, &record]);
        assert_eq!((duplicate.released, duplicate.duplicate, duplicate.classified()), (1, 1, 0));
    }
    #[test]
    fn domains_are_not_interchangeable_and_render_scope_is_independent() {
        let mut record = record();
        let identity = *record.receipt.identity();
        let wrong = count_released(&(Some(Vec::new()), Some(vec![identity])), &[&record]);
        assert_eq!((wrong.released, wrong.invalid, wrong.classified()), (1, 1, 0));
        assert_eq!(wrong.integrity.get("DomainMismatch"), Some(&1));
        record.domain = SscAdoptionDomain::Render;
        let correct = count_released(&(Some(Vec::new()), Some(vec![identity])), &[&record]);
        assert_eq!((correct.master, correct.render, correct.out_of_scope), (0, 1, 1));
    }
    #[test]
    fn unreadable_and_changed_origin_states_are_explicit_integrity_classes() {
        let record = record();
        let identity = *record.receipt.identity();
        for (which, expected) in [(0, "PreStateUnreadable"), (1, "PostStateUnreadable"), (2, "PreFingerprintMismatch"), (3, "PostFingerprintMismatch")] {
            let (receipt, original_identity, origin) = record.receipt.borrowed(None).unwrap();
            let mut corrupt = origin.clone();
            match which {
                0 => corrupt.pre_state = CapturedPreState::Deflated(Arc::from([0_u8, 1, 2])),
                1 => corrupt.post_state = CapturedPreState::Deflated(Arc::from([0_u8, 1, 2])),
                2 => corrupt.pre_state = CapturedPreState::Raw(Arc::from("<foreign/>")),
                _ => corrupt.post_state = CapturedPreState::Raw(Arc::from("<foreign/>")),
            }
            let released = SscReleasedEventRef { domain: SscEvidenceDomain::Master, identity: &identity };
            let mut counts = ObservationCounts::default();
            classify_bound_origin(released, receipt, original_identity, &corrupt, &mut counts);
            assert_eq!((counts.invalid, counts.classified()), (1, 0));
            assert_eq!(counts.integrity.get(expected), Some(&1));
        }
    }
    #[test]
    fn exact_invariant_refutation_is_distinct_from_missing_necessity() {
        let record = record(); let identity = *record.receipt.identity();
        let before = "<score><note><pitch>C</pitch></note></score>";
        let after = "<score><note><pitch>D</pitch></note></score>";
        let event = SscEventBinding { released: SscReleasedEventRef { domain: SscEvidenceDomain::Master, identity: &identity }, pre_state: before, post_state: after };
        let origin = SscOriginState { pre_state: CapturedPreState::capture(before), post_state: CapturedPreState::capture(after), scope: SscOriginScope::F5 };
        let (proof, counterexample, necessity) = invariant_from_exact_origin(&event, &origin, SscProofFamily::F5AtomicVoiceStaffVisibility);
        assert!(proof.is_none() && counterexample.is_some() && necessity.is_none());
        let exact = &counterexample.as_ref().unwrap().counterexample.witness;
        assert_eq!((&*exact.pre_bytes, &*exact.post_bytes), (before, after));
        let unchanged = SscEventBinding { pre_state: before, post_state: before, ..event };
        let (proof, counterexample, necessity) = invariant_from_exact_origin(&unchanged, &origin, SscProofFamily::F5AtomicVoiceStaffVisibility);
        assert!(proof.is_some() && counterexample.is_none() && necessity.is_none());
    }
    #[test]
    fn observer_rejects_foreign_proof_and_out_of_scope_proofs() {
        let record = record(); let identity = *record.receipt.identity();
        let event = SscEventBinding { released: SscReleasedEventRef { domain: SscEvidenceDomain::Master, identity: &identity }, pre_state: "<d/>", post_state: "<d/>" };
        let other = SscEventBinding { released: event.released, pre_state: event.pre_state, post_state: event.post_state };
        let family = SscProofFamily::F5AtomicVoiceStaffVisibility;
        let proof = SscInvariantProof { event: &other, family, certificate: SscConstructiveInvariantCertificate { preserved_structure: SscCheckedStructurePreservation { normalized_bytes: 4 } } };
        let (receipt, _, _) = record.receipt.borrowed(None).unwrap();
        let transformation = SscTransformationEvidence { event: &event, receipt: SscOriginReceiptRef { inner: receipt }, scope: SscFamilyScope::InScope(family) };
        assert!(matches!(observe_ssc_event_v2(&transformation, SscInvariantAssessment::Proven(&proof), SscNecessityAssessment::Unproven, SscContainmentAssessment::Unproven), Err(SscEvidenceIntegrityFailure::ProofBindingMismatch)));
        let outside = SscTransformationEvidence { scope: SscFamilyScope::OutOfScope, ..transformation };
        assert!(matches!(observe_ssc_event_v2(&outside, SscInvariantAssessment::Proven(&proof), SscNecessityAssessment::Unproven, SscContainmentAssessment::Unproven), Err(SscEvidenceIntegrityFailure::IllegalProofCombination)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn score(note: &str) -> String {
        format!(
            "<score-partwise><part id=\"P\"><measure><attributes><divisions>1</divisions></attributes>{note}</measure></part></score-partwise>"
        )
    }
    const NOTE: &str = "<note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>";
    #[test]
    fn missing_requested_staff_has_structural_certificate() {
        let pre = score(NOTE);
        let post = pre.replace("</note>", "<staff>2</staff></note>");
        assert!(matches!(
            prove_origin(&pre, &post, SscOriginScope::F5),
            Some((SscProofFamily::F5AtomicVoiceStaffVisibility, _, _))
        ));
    }
    #[test]
    fn existing_staff_move_is_not_rebranded_as_necessity() {
        let pre = score(&NOTE.replace("</note>", "<staff>1</staff></note>"));
        let post = pre.replace("<staff>1</staff>", "<staff>2</staff>");
        assert!(prove_origin(&pre, &post, SscOriginScope::F5).is_none());
    }
    #[test]
    fn serialized_missing_voice_is_distinct_from_lane_reassignment() {
        let pre = score(&NOTE.replace("<voice>1</voice>", ""));
        let post = score(NOTE);
        assert!(matches!(
            prove_origin(&pre, &post, SscOriginScope::F9),
            Some((SscProofFamily::F9AuthoritySerialization, _, _))
        ));
        assert!(
            prove_origin(
                &post,
                &post.replace("<voice>1</voice>", "<voice>2</voice>"),
                SscOriginScope::F9
            )
            .is_none()
        );
    }
    #[test]
    fn changed_pitch_duration_cursor_and_namespace_never_pass_lane_proof() {
        let pre = score(NOTE);
        let valid = pre.replace("</note>", "<staff>2</staff></note>");
        for post in [
            valid.replace("<step>C</step>", "<step>D</step>"),
            valid.replace("<duration>4</duration>", "<duration>3</duration>"),
            valid.replace(
                "</measure>",
                "<backup><duration>1</duration></backup></measure>",
            ),
            valid.replace("<score-partwise>", "<score-partwise xmlns=\"foreign\">"),
        ] {
            assert!(prove_origin(&pre, &post, SscOriginScope::F5).is_none());
        }
    }
    #[test]
    fn unproved_scope_is_unknown_even_for_a_structurally_valid_change() {
        let pre = score(NOTE);
        let post = pre.replace("</note>", "<staff>2</staff></note>");
        assert!(prove_origin(&pre, &post, SscOriginScope::Unproved).is_none());
    }
    #[test]
    fn hidden_uncovered_rest_has_a_voice_staff_bound_witness() {
        let rest = "<note print-object=\"no\"><rest/><duration>4</duration><voice>1</voice><staff>1</staff></note>";
        let pre = score(rest);
        let post = pre.replace(" print-object=\"no\"", "");
        assert!(matches!(
            prove_origin(&pre, &post, SscOriginScope::F7),
            Some((SscProofFamily::F7RestVoiceStrictStaff, _, _))
        ));
    }
    #[test]
    fn overlapping_visible_note_or_rest_is_not_a_coverage_defect() {
        let rest = "<note print-object=\"no\"><rest/><duration>4</duration><voice>1</voice><staff>1</staff></note>";
        for other in [
            NOTE.replace("</note>", "<staff>1</staff></note>"),
            rest.replace(" print-object=\"no\"", ""),
        ] {
            let pre = score(&format!(
                "{rest}<backup><duration>4</duration></backup>{other}"
            ));
            let post = pre.replacen(" print-object=\"no\"", "", 1);
            assert!(prove_origin(&pre, &post, SscOriginScope::F7).is_none());
        }
    }
    #[test]
    fn rest_removal_timing_voice_and_chord_changes_remain_unknown() {
        let rest = "<note print-object=\"no\"><rest/><duration>4</duration><voice>1</voice><staff>1</staff></note>";
        let pre = score(rest);
        let post = pre.replace(" print-object=\"no\"", "");
        for changed in [
            post.replace("<duration>4</duration>", "<duration>3</duration>"),
            post.replace("<voice>1</voice>", "<voice>2</voice>"),
            post.replace("<rest/>", "<chord/><rest/>"),
        ] {
            assert!(prove_origin(&pre, &changed, SscOriginScope::F7).is_none());
        }
    }
    #[test]
    fn final_barline_insertion_is_positive_only_for_the_sealed_scope() {
        let pre = score(NOTE);
        let post = pre.replace(
            "</measure>",
            "<barline location=\"right\"><bar-style>light-heavy</bar-style></barline></measure>",
        );
        assert!(matches!(
            prove_origin(&pre, &post, SscOriginScope::FinalBarline),
            Some((SscProofFamily::Additive, _, _))
        ));
        assert!(prove_origin(&pre, &post, SscOriginScope::Unproved).is_none());
        assert!(
            prove_origin(
                &pre,
                &post.replace("<step>C</step>", "<step>D</step>"),
                SscOriginScope::FinalBarline
            )
            .is_none()
        );
    }
    #[test]
    fn malformed_unchanged_or_duplicate_lane_fields_cannot_issue_certificates() {
        assert!(prove_origin("broken", "other", SscOriginScope::F5).is_none());
        let pre = score(NOTE);
        assert!(prove_origin(&pre, &pre, SscOriginScope::F5).is_none());
        let post = pre.replace("</note>", "<staff>2</staff><staff>2</staff></note>");
        assert!(prove_origin(&pre, &post, SscOriginScope::F5).is_none());
    }
    #[test]
    fn duplication_and_unrelated_metadata_do_not_change_local_certificate() {
        let pre = score(NOTE);
        let post = pre.replace("</note>", "<staff>2</staff></note>");
        for (before, after) in [
            (pre.clone(), post.clone()),
            (
                pre.replace("id=\"P\"", "id=\"other\""),
                post.replace("id=\"P\"", "id=\"other\""),
            ),
            (pre.clone(), post.clone()),
        ] {
            assert!(prove_origin(&before, &after, SscOriginScope::F5).is_some());
        }
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;
    #[test]
    fn ssc_compressed_snapshot_is_lossless_with_multibyte_original_bytes() {
        let pre = format!("<score>{}</score>", "Äß🎵<note/>".repeat(2500));
        let captured = CapturedPreState::capture(&pre);
        assert!(matches!(captured, CapturedPreState::Deflated(_)));
        assert_eq!(captured.exact_bytes().as_deref(), Some(pre.as_str()));
    }
    #[test]
    fn ssc_invalid_compressed_capture_is_not_reconstructed_from_a_fingerprint() {
        let captured = CapturedPreState::Deflated(Arc::from([1_u8, 2, 3]));
        assert!(captured.exact_bytes().is_none());
    }
}

pub(crate) fn release_unadopted_additive_origin(
    _origin: &SscReceiptOriginAccess,
    identity: &MintedIdentity,
) {
    fn remove(pending: &mut Vec<(MintedIdentity, SscOriginState)>, identity: &MintedIdentity) {
        if let Some(index) = pending
            .iter()
            .position(|(existing, _)| existing == identity)
        {
            pending.swap_remove(index);
        }
    }
    #[cfg(not(test))]
    with_pending(|pending| remove(pending, identity));
    // Test TLS stores may be destroyed before adopted receipts at thread exit.
    #[cfg(test)]
    let _ = PENDING_ADDITIVE.try_with(|slot| {
        if let Ok(mut pending) = slot.try_borrow_mut() {
            remove(&mut pending, identity);
        }
    });
}



// Test-only probe for the PRESERVING/E036 proof bundle. Real receipts are
// produced by their owners in structural_write_authority tests and handed over
// by value; this probe only replays the private classification with one named
// corruption. It returns plain counts, never a proof carrier.
#[cfg(test)]
pub(crate) enum SscBundleProbeReceipt {
    Preserving(PreservingTransformationReceipt),
    Additive(AdditiveTransformationReceipt),
}
#[cfg(test)]
pub(crate) enum SscBundleProbeCase {
    Released,
    Missing,
    Duplicate,
    WrongDomain,
    ForeignIdentity,
    StalePre(String),
    WrongPost(String),
    CorruptPre,
    CorruptPost,
    ForeignScope,
    ChangedProjection(String),
}
#[cfg(test)]
impl SscBundleProbeReceipt {
    fn adopted(self) -> AdoptedEvidence {
        match self {
            Self::Preserving(receipt) => AdoptedEvidence { receipt: AdoptedReceipt::Preserving(receipt), domain: SscAdoptionDomain::Master, additive_origin: None },
            Self::Additive(receipt) => {
                let identity = *crate::master_state_identity::ssc_additive_identity(&receipt);
                let mut origin = None;
                with_pending(|pending| origin = pending.iter().find(|(existing, _)| *existing == identity).map(|(_, origin)| origin.clone()));
                AdoptedEvidence { receipt: AdoptedReceipt::Additive(receipt), domain: SscAdoptionDomain::Master, additive_origin: origin }
            }
        }
    }
}
#[cfg(test)]
pub(crate) fn ssc_bundle_probe_for_test(
    receipt: SscBundleProbeReceipt,
    foreign: SscBundleProbeReceipt,
    case: SscBundleProbeCase,
) -> std::collections::BTreeMap<String, usize> {
    let record = receipt.adopted();
    let foreign = foreign.adopted();
    let identity = *record.receipt.identity();
    let mut counts = match &case {
        SscBundleProbeCase::Released => count_released(&(Some(vec![identity]), Some(Vec::new())), &[&record]),
        SscBundleProbeCase::Missing => count_released(&(Some(vec![identity]), Some(Vec::new())), &[]),
        SscBundleProbeCase::Duplicate => count_released(&(Some(vec![identity]), Some(Vec::new())), &[&record, &record]),
        SscBundleProbeCase::WrongDomain => count_released(&(Some(Vec::new()), Some(vec![identity])), &[&record]),
        _ => ObservationCounts::default(),
    };
    let Some((kind, own_identity, origin)) = record.receipt.borrowed(record.additive_origin.as_ref()) else {
        return std::collections::BTreeMap::from([("NoOrigin".to_owned(), 1)]);
    };
    let released = SscReleasedEventRef { domain: SscEvidenceDomain::Master, identity: &identity };
    let mut corrupt = origin.clone();
    match case {
        SscBundleProbeCase::ForeignIdentity => {
            let foreign_released = SscReleasedEventRef { domain: SscEvidenceDomain::Master, identity: foreign.receipt.identity() };
            classify_bound_origin(foreign_released, kind, own_identity, origin, &mut counts);
        }
        SscBundleProbeCase::ForeignScope => {
            let (_, _, foreign_origin) = foreign.receipt.borrowed(foreign.additive_origin.as_ref()).expect("foreign origin");
            classify_bound_origin(released, kind, own_identity, foreign_origin, &mut counts);
        }
        SscBundleProbeCase::StalePre(bytes) | SscBundleProbeCase::WrongPost(bytes) | SscBundleProbeCase::ChangedProjection(bytes) if bytes.is_empty() => {}
        SscBundleProbeCase::StalePre(bytes) => {
            corrupt.pre_state = CapturedPreState::Raw(Arc::from(bytes.as_str()));
            classify_bound_origin(released, kind, own_identity, &corrupt, &mut counts);
        }
        SscBundleProbeCase::WrongPost(bytes) => {
            corrupt.post_state = CapturedPreState::Raw(Arc::from(bytes.as_str()));
            classify_bound_origin(released, kind, own_identity, &corrupt, &mut counts);
        }
        SscBundleProbeCase::CorruptPre => {
            corrupt.pre_state = CapturedPreState::Deflated(Arc::from([9_u8, 9, 9]));
            classify_bound_origin(released, kind, own_identity, &corrupt, &mut counts);
        }
        SscBundleProbeCase::CorruptPost => {
            corrupt.post_state = CapturedPreState::Deflated(Arc::from([9_u8, 9, 9]));
            classify_bound_origin(released, kind, own_identity, &corrupt, &mut counts);
        }
        SscBundleProbeCase::ChangedProjection(post) => {
            // Bypasses the fingerprint barrier on purpose: it isolates the
            // constructive invariant refutation for the exact captured pre.
            let pre = origin.pre_state.exact_bytes().expect("captured pre");
            let event = SscEventBinding { released, pre_state: &pre, post_state: &post };
            let scope = proof_scope(origin.scope);
            if let SscFamilyScope::InScope(family) = scope {
                let (proof, counterexample, necessity) = invariant_from_exact_origin(&event, origin, family);
                let transformation = SscTransformationEvidence { event: &event, receipt: SscOriginReceiptRef { inner: kind }, scope };
                let invariant = match (&proof, &counterexample) {
                    (Some(proof), None) => SscInvariantAssessment::Proven(proof),
                    (None, Some(counterexample)) => SscInvariantAssessment::Refuted(counterexample),
                    _ => SscInvariantAssessment::Unproven,
                };
                let necessity = necessity.as_ref().map_or(SscNecessityAssessment::Unproven, SscNecessityAssessment::Proven);
                match observe_ssc_event_v2(&transformation, invariant, necessity, SscContainmentAssessment::Unproven) {
                    Ok(classified) => counts.observe(SscReleasedObservationV2::Classified(classified)),
                    Err(integrity) => counts.fail(integrity),
                }
            }
        }
        _ => {}
    }
    let mut out = counts.integrity.clone();
    // r1287: the A7 adoption decision over exactly this classification.
    out.insert("AdoptionPermitted".to_owned(), usize::from(adoption_decision(&counts).is_ok()));
    for (label, value) in [
        ("Released", counts.released), ("Missing", counts.missing), ("Duplicate", counts.duplicate), ("Invalid", counts.invalid),
        ("OutOfScope", counts.out_of_scope), ("InvariantUnproven", counts.invariant_unproven), ("InvariantDenied", counts.invariant_denied),
        ("NecessityUnproven", counts.necessity_unproven), ("Permit", counts.permit), ("Classified", counts.classified()),
        ("V4OutOfScope", counts.containment[0]), ("V4ContainmentUnproven", counts.containment[1]),
        ("V4ContainmentDenied", counts.containment[2]), ("V4Permitted", counts.containment[3]),
    ] {
        out.insert(label.to_owned(), value);
    }
    out
}

// Pure byte-level classes of the bundle; no owner or writer is reachable here.
#[cfg(test)]
mod proof_bundle_class_tests {
    use super::*;
    fn score(notes: &str) -> String {
        format!("<score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>P</part-name></score-part></part-list><part id=\"P1\"><measure><attributes><divisions>2</divisions></attributes>{notes}</measure></part></score-partwise>")
    }
    fn eighth(step: &str, beam: Option<&str>) -> String {
        let beam = beam.map_or_else(String::new, |value| format!("<beam number=\"1\">{value}</beam>"));
        format!("<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>eighth</type><staff>1</staff>{beam}</note>")
    }
    fn four(beams: [Option<&str>; 4]) -> String {
        score(&["C", "D", "E", "F"].iter().zip(beams).map(|(step, beam)| eighth(step, beam)).collect::<String>())
    }
    #[test]
    fn beam_chain_necessity_requires_a_real_defect_and_full_attribution() {
        let broken = four([Some("begin"), Some("continue"), None, None]);
        let closed = four([Some("begin"), Some("continue"), Some("continue"), Some("end")]);
        assert_eq!(beam_chain_repair_note(&broken, &closed), Some(2));
        let well_formed = four([Some("begin"), Some("end"), Some("begin"), Some("end")]);
        assert_eq!(beam_chain_repair_note(&well_formed, &closed), None, "no defect, no necessity");
        let still_broken = four([Some("begin"), Some("continue"), Some("continue"), None]);
        assert_eq!(beam_chain_repair_note(&broken, &still_broken), None, "unresolved defect");
        assert_eq!(beam_chain_repair_note(&broken, &closed.replacen("<step>F</step>", "<step>G</step>", 1)), None, "non-beam change");
        let orphan_end = four([Some("begin"), Some("end"), Some("end"), None]);
        let wider = four([Some("begin"), Some("continue"), Some("end"), None]);
        assert_eq!(beam_chain_repair_note(&orphan_end, &wider), Some(1), "change inside the malformed sequence");
        let second_voice = |beams: [Option<&str>; 2]| ["E", "F"].iter().zip(beams).map(|(step, beam)| eighth(step, beam).replace("<voice>1</voice>", "<voice>2</voice>")).collect::<String>();
        let pre = score(&format!("{}{}<backup><duration>2</duration></backup>{}", eighth("C", Some("begin")), eighth("D", None), second_voice([Some("begin"), Some("end")])));
        let post = score(&format!("{}{}<backup><duration>2</duration></backup>{}", eighth("C", Some("begin")), eighth("D", Some("end")), second_voice([None, None])));
        assert_eq!(beam_chain_repair_note(&pre, &post), None, "a change in a well-formed foreign sequence is not attributed");
        let chord = |beam: Option<&str>| eighth("G", beam).replace("<pitch>", "<chord/><pitch>");
        let pre = score(&format!("{}{}{}", eighth("C", Some("begin")), chord(None), eighth("D", None)));
        let post = score(&format!("{}{}{}", eighth("C", Some("begin")), chord(Some("begin")), eighth("D", Some("end"))));
        assert_eq!(beam_chain_repair_note(&pre, &post), None, "chord-member beam changes are not attributed");
        assert_eq!(beam_chain_repair_note("broken", &closed), None);
    }
    #[test]
    fn direction_geometry_accepts_only_one_measure_bound_direction_insertion() {
        let pre = four([None, None, None, None]);
        let at = pre.find("<note>").unwrap();
        let insert = |offset: usize, fragment: &str| format!("{}{fragment}{}", &pre[..offset], &pre[offset..]);
        let direction = "\n<direction><direction-type><words>x</words></direction-type></direction>";
        assert!(matches!(direction_insertion_geometry(&pre, &insert(at, direction)), DirectionGeometry::Inserted(_)));
        assert!(matches!(direction_insertion_geometry(&pre, &insert(at, &format!("\n<!-- m -->{direction}"))), DirectionGeometry::Inserted(_)));
        let part_list = pre.find("<part-list>").unwrap();
        for broken in [
            insert(at, &format!("{direction}{direction}")),
            insert(at, &format!("\n<!-- a --><!-- b -->{direction}")),
            insert(at, "\n<barline><bar-style>light-heavy</bar-style></barline>"),
            insert(part_list, direction),
            insert(at, direction).replacen("<step>C</step>", "<step>D</step>", 1),
            pre.replacen("<note>", "<note print-object=\"no\">", 1),
        ] {
            assert!(matches!(direction_insertion_geometry(&pre, &broken), DirectionGeometry::Broken(..)), "{broken}");
        }
        assert!(matches!(direction_insertion_geometry(&pre, "<unterminated"), DirectionGeometry::Unreadable | DirectionGeometry::Broken(..)));
    }
}

// Contract v4 A2/A3: containment against the frozen envelope rows; pure byte classes.
#[cfg(test)]
mod containment_v4_tests {
    use super::*;
    fn score(measure: &str) -> String {
        format!("<score-partwise><part id=\"P\"><measure><attributes><divisions>2</divisions></attributes>{measure}</measure></part></score-partwise>")
    }
    fn note(step: &str, voice: &str, beam: Option<&str>) -> String {
        let beam = beam.map_or(String::new(), |value| format!("<beam number=\"1\">{value}</beam>"));
        format!("<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>1</duration><voice>{voice}</voice><type>eighth</type>{beam}</note>")
    }
    fn row(descriptor: &str) -> EnvelopeRow {
        envelope_row(descriptor).unwrap_or_else(|| panic!("frozen envelope row {descriptor}"))
    }
    fn contained(pre: &str, post: &str, descriptor: &str) -> bool {
        matches!(containment(pre, post, &row(descriptor)), Containment::Contained(_))
    }
    fn escaped(pre: &str, post: &str, descriptor: &str) -> bool {
        matches!(containment(pre, post, &row(descriptor)), Containment::Escaped(_))
    }
    #[test]
    fn every_bound_scope_has_its_frozen_row_and_kind() {
        for (scope, kind) in [
            (SscOriginScope::F5, SscEnvelopeKind::Lane),
            (SscOriginScope::F9, SscEnvelopeKind::Lane),
            (SscOriginScope::F7, SscEnvelopeKind::Lane),
            (SscOriginScope::FinalBarline, SscEnvelopeKind::Insertion),
            (SscOriginScope::DirectionInsertion, SscEnvelopeKind::Insertion),
        ] {
            let descriptor = envelope_descriptor(scope).unwrap();
            assert_eq!(row(descriptor).envelope.kind, kind, "{descriptor}");
        }
        assert!(envelope_descriptor(SscOriginScope::Unproved).is_none());
        let preserving = include_str!("build_gate/ssc_containment_envelopes_v1.tsv").lines().filter(|line| line.contains("|Preserving|Projection|")).count();
        assert_eq!(preserving, 13);
        assert!(envelope_row("UNKNOWN_DESCRIPTOR").is_none());
    }
    #[test]
    fn beam_owner_is_contained_in_beams_and_escapes_on_pitch() {
        let pre = score(&format!("{}{}", note("C", "1", None), note("D", "1", None)));
        let beamed = score(&format!("{}{}", note("C", "1", Some("begin")), note("D", "1", Some("end"))));
        assert!(contained(&pre, &beamed, "COMMITTED_CROSS_STAFF_BEAM_PRESERVING"));
        let respelled = score(&format!("{}{}", note("C", "1", Some("begin")), note("E", "1", Some("end"))));
        assert!(escaped(&pre, &respelled, "COMMITTED_CROSS_STAFF_BEAM_PRESERVING"));
        let revoiced = score(&format!("{}{}", note("C", "2", Some("begin")), note("D", "1", Some("end"))));
        assert!(escaped(&pre, &revoiced, "COMMITTED_CROSS_STAFF_BEAM_PRESERVING"));
    }
    #[test]
    fn lane_owner_changes_note_lanes_only() {
        let pre = score(&note("C", "1", None));
        let moved = score(&note("C", "2", None));
        assert!(contained(&pre, &moved, "STAFF_VOICE_MOVE_EXACT"));
        let with_staff = score(&note("C", "1", None).replace("</voice>", "</voice><staff>2</staff>"));
        assert!(contained(&pre, &with_staff, "STAFF_VOICE_MOVE_EXACT"));
        // A voice outside a note (forward) is not a lane field of the sealed owner.
        let forward = score(&format!("<forward><duration>1</duration><voice>1</voice></forward>{}", note("C", "1", None)));
        let forward_moved = forward.replacen("<voice>1</voice></forward>", "<voice>2</voice></forward>", 1);
        assert!(escaped(&forward, &forward_moved, "STAFF_VOICE_MOVE_EXACT"));
        let pitched = score(&note("D", "2", None));
        assert!(escaped(&pre, &pitched, "STAFF_VOICE_MOVE_EXACT"));
        let hidden = score("<note print-object=\"no\"><rest/><duration>2</duration><voice>1</voice></note>");
        let shown = score("<note><rest/><duration>2</duration><voice>1</voice></note>");
        assert!(contained(&hidden, &shown, "RENDER_ROW_44_EXACT"));
        assert!(escaped(&hidden, &shown.replace("<duration>2", "<duration>1"), "RENDER_ROW_44_EXACT"));
    }
    #[test]
    fn insertion_rows_admit_only_new_elements_of_their_class() {
        let pre = score(&note("C", "1", None));
        let direction = "<direction placement=\"below\"><direction-type><dynamics><p/></dynamics></direction-type></direction>";
        let inserted = score(&format!("{direction}{}", note("C", "1", None)));
        assert!(contained(&pre, &inserted, "DIRECTION_INSERTION_ADDITIVE"));
        // Insert-only: changing an existing direction is not an insertion.
        let changed = inserted.replace("<p/>", "<f/>");
        assert!(escaped(&inserted, &changed, "DIRECTION_INSERTION_ADDITIVE"));
        let removed = pre.clone();
        assert!(escaped(&inserted, &removed, "DIRECTION_INSERTION_ADDITIVE"));
        let barline = "<barline location=\"right\"><bar-style>light-heavy</bar-style></barline>";
        let finished = pre.replace("</measure>", &format!("{barline}</measure>"));
        assert!(contained(&pre, &finished, "FINAL_BARLINE_ADDITIVE"));
        assert_eq!(final_barline_geometry(&pre, &finished), Some(true));
        let early = pre.replace("<attributes>", &format!("{barline}<attributes>"));
        assert_eq!(final_barline_geometry(&pre, &early), Some(false));
        assert!(escaped(&pre, &finished.replace("<step>C", "<step>D"), "FINAL_BARLINE_ADDITIVE"));
    }
    #[test]
    fn layout_and_clef_owners_stay_inside_their_purpose() {
        let pre = score(&note("C", "1", None));
        let printed = pre.replace("<measure>", "<measure width=\"120\"><print new-system=\"yes\"/>");
        assert!(contained(&pre, &printed, "PIANIST_LAYOUT_PRESERVING"));
        assert!(escaped(&pre, &printed.replace("<step>C", "<step>D"), "PIANIST_LAYOUT_PRESERVING"));
        let clefed = pre.replace("</attributes>", "<clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>");
        assert!(contained(&pre, &clefed, "RUSTIG_CLEF_COURSE_PRESERVING"));
        assert!(escaped(&pre, &printed, "RUSTIG_CLEF_COURSE_PRESERVING"));
    }
    fn observe_with(scope: SscFamilyScope, invariant: Option<bool>, containment: Option<bool>) -> Result<SscContainmentVerdict, SscEvidenceIntegrityFailure> {
        let receipt = crate::master_state_identity::tests::synthetic_exact_receipt_for_tests("<d>0</d>", "<d>1</d>", 3, 4, "1");
        let record = AdoptedEvidence { receipt: AdoptedReceipt::Exact(receipt), domain: SscAdoptionDomain::Master, additive_origin: None };
        let identity = *record.receipt.identity();
        let event = SscEventBinding { released: SscReleasedEventRef { domain: SscEvidenceDomain::Master, identity: &identity }, pre_state: "<d/>", post_state: "<d/>" };
        let family = SscProofFamily::F5AtomicVoiceStaffVisibility;
        let (receipt, _, _) = record.receipt.borrowed(None).unwrap();
        let transformation = SscTransformationEvidence { event: &event, receipt: SscOriginReceiptRef { inner: receipt }, scope };
        let proof = SscInvariantProof { event: &event, family, certificate: SscConstructiveInvariantCertificate { preserved_structure: SscCheckedStructurePreservation { normalized_bytes: 4 } } };
        let refuted = SscInvariantCounterexample { event: &event, family, counterexample: SscConstructiveInvariantCounterexample { kind: SscInvariantFailureKind::ForbiddenFieldChanged, witness: difference_witness("<d/>", "<d/>", 0..0, 0..0) } };
        let envelope = SscEnvelopeRef { descriptor: "STAFF_VOICE_MOVE_EXACT", kind: SscEnvelopeKind::Lane };
        let inside = SscContainmentProof { event: &event, envelope, certificate: SscConstructiveContainmentCertificate { checked_bytes: 4 } };
        let outside = SscContainmentCounterexample { event: &event, envelope, witness: difference_witness("<a/>", "<b/>", 1..2, 1..2) };
        let invariant = match invariant { None => SscInvariantAssessment::Unproven, Some(true) => SscInvariantAssessment::Proven(&proof), Some(false) => SscInvariantAssessment::Refuted(&refuted) };
        let containment = match containment { None => SscContainmentAssessment::Unproven, Some(true) => SscContainmentAssessment::Contained(&inside), Some(false) => SscContainmentAssessment::Escaped(&outside) };
        observe_ssc_event_v2(&transformation, invariant, SscNecessityAssessment::Unproven, containment).map(|classified| classified.containment_verdict)
    }
    #[test]
    fn permit_needs_invariant_and_containment_and_never_necessity() {
        let inside = SscFamilyScope::InScope(SscProofFamily::F5AtomicVoiceStaffVisibility);
        assert_eq!(observe_with(inside, Some(true), Some(true)), Ok(SscContainmentVerdict::Permit));
        assert_eq!(observe_with(inside, Some(true), Some(false)), Ok(SscContainmentVerdict::ContainmentDenied));
        assert_eq!(observe_with(inside, Some(false), Some(true)), Ok(SscContainmentVerdict::ContainmentDenied));
        assert_eq!(observe_with(inside, None, Some(true)), Ok(SscContainmentVerdict::ContainmentUnproven));
        assert_eq!(observe_with(inside, Some(true), None), Ok(SscContainmentVerdict::ContainmentUnproven));
        assert_eq!(observe_with(inside, Some(false), None), Ok(SscContainmentVerdict::ContainmentDenied));
        assert_eq!(observe_with(SscFamilyScope::OutOfScope, None, None), Ok(SscContainmentVerdict::OutOfScope));
        assert_eq!(observe_with(SscFamilyScope::OutOfScope, None, Some(true)), Err(SscEvidenceIntegrityFailure::IllegalProofCombination));
    }
    #[test]
    fn a_containment_proof_of_another_event_is_a_binding_failure() {
        let receipt = crate::master_state_identity::tests::synthetic_exact_receipt_for_tests("<d>0</d>", "<d>1</d>", 3, 4, "1");
        let record = AdoptedEvidence { receipt: AdoptedReceipt::Exact(receipt), domain: SscAdoptionDomain::Master, additive_origin: None };
        let identity = *record.receipt.identity();
        let event = SscEventBinding { released: SscReleasedEventRef { domain: SscEvidenceDomain::Master, identity: &identity }, pre_state: "<d/>", post_state: "<d/>" };
        let other = SscEventBinding { released: event.released, pre_state: event.pre_state, post_state: event.post_state };
        let family = SscProofFamily::F5AtomicVoiceStaffVisibility;
        let (receipt, _, _) = record.receipt.borrowed(None).unwrap();
        let transformation = SscTransformationEvidence { event: &event, receipt: SscOriginReceiptRef { inner: receipt }, scope: SscFamilyScope::InScope(family) };
        let proof = SscInvariantProof { event: &event, family, certificate: SscConstructiveInvariantCertificate { preserved_structure: SscCheckedStructurePreservation { normalized_bytes: 4 } } };
        let foreign = SscContainmentProof { event: &other, envelope: SscEnvelopeRef { descriptor: "STAFF_VOICE_MOVE_EXACT", kind: SscEnvelopeKind::Lane }, certificate: SscConstructiveContainmentCertificate { checked_bytes: 4 } };
        assert!(matches!(
            observe_ssc_event_v2(&transformation, SscInvariantAssessment::Proven(&proof), SscNecessityAssessment::Unproven, SscContainmentAssessment::Contained(&foreign)),
            Err(SscEvidenceIntegrityFailure::ProofBindingMismatch)
        ));
    }
}

#[cfg(test)]
mod edit_block_containment_tests {
    use super::*;
    #[test]
    fn edit_replay_and_declared_ranges_are_independent_obligations() {
        let pre = "<score><note><pitch><step>C</step></pitch></note><metadata>x</metadata></score>";
        let start = pre.find('C').unwrap();
        let post = pre.replacen('C', "D", 1);
        let edits = [(start, start + 1, "D")];
        let row = envelope_row("HARMONIC_PITCH_SWAP_EXACT").unwrap();
        assert!(edit_script_locality(pre, &post, &edits));
        assert_eq!(undeclared_edit(pre, &post, &edits, &row), Some(None));
        assert!(matches!(containment(pre, &post, &row), Containment::Contained(_)));
        let outside = post.replace("metadata>x", "metadata>y");
        assert!(!edit_script_locality(pre, &outside, &edits));
        let wrong = [(start, start + 1, "E")];
        assert!(!edit_script_locality(pre, &post, &wrong));
        let location = pre.find(">x<").unwrap() + 1;
        let escaped = pre.replacen(">x<", ">y<", 1);
        let valid_script = [(location, location + 1, "y")];
        assert!(edit_script_locality(pre, &escaped, &valid_script));
        assert_eq!(undeclared_edit(pre, &escaped, &valid_script, &row), Some(Some((location..location + 1, location..location + 1))));
        assert!(matches!(containment(pre, &escaped, &row), Containment::Escaped(_)));
    }
    #[test]
    fn edit_canonical_equality_cannot_hide_an_outside_comment_or_space_edit() {
        let pre = "<score><!--x--><note><pitch><step>C</step></pitch></note> </score>";
        let row = envelope_row("HARMONIC_PITCH_SWAP_EXACT").unwrap();
        for (location, replacement) in [(pre.find("x--").unwrap(), "y"), (pre.rfind(" </").unwrap(), "\t")] {
            let mut post = pre.to_owned(); post.replace_range(location..location + 1, replacement);
            let edits = [(location, location + 1, replacement)];
            assert!(edit_script_locality(pre, &post, &edits));
            assert!(matches!(containment(pre, &post, &row), Containment::Contained(_)));
            assert!(matches!(undeclared_edit(pre, &post, &edits, &row), Some(Some(_))));
        }
    }
    #[test]
    fn block_locality_enforces_prefix_suffix_even_when_containment_holds() {
        let pre = "<score><measure><attributes><clef><sign>G</sign></clef></attributes></measure><metadata>x</metadata></score>";
        let start = pre.find("<measure>").unwrap(); let end = pre.find("</measure>").unwrap() + "</measure>".len();
        let post = pre.replace("<sign>G", "<sign>F");
        let row = envelope_row("CLEF_MEASURE_BLOCK").unwrap();
        assert!(bound_span_locality(pre, &post, Some((start, end))));
        assert!(matches!(containment(pre, &post, &row), Containment::Contained(_)));
        let escaped = post.replace("metadata>x", "metadata>y");
        assert!(bound_span_locality(pre, &escaped, None));
        assert!(!bound_span_locality(pre, &escaped, Some((start, end))));
        assert!(matches!(containment(pre, &escaped, &row), Containment::Escaped(_)));
        let outside_format = post.replace("</score>", " </score>");
        assert!(matches!(containment(pre, &outside_format, &row), Containment::Contained(_)));
        assert!(!bound_span_locality(pre, &outside_format, Some((start, end))));
        assert!(!bound_span_locality(pre, &post, Some((end, start))));
        assert!(!bound_span_locality(pre, &post, Some((start, usize::MAX))));
    }
    #[test]
    fn foreign_descriptor_and_block_family_on_exact_are_binding_failures() {
        let receipt = crate::master_state_identity::tests::synthetic_exact_receipt_for_tests("<d>0</d>", "<d>1</d>", 3, 4, "1");
        let identity = *receipt.ssc_bound_origin().0;
        let event = SscEventBinding { released: SscReleasedEventRef { domain: SscEvidenceDomain::Master, identity: &identity }, pre_state: "<d>0</d>", post_state: "<d>1</d>" };
        for family in [SscProofFamily::Edit, SscProofFamily::Block] {
            let transformation = SscTransformationEvidence { event: &event, receipt: SscOriginReceiptRef { inner: SscOriginReceiptKind::Exact(&receipt) }, scope: SscFamilyScope::InScope(family) };
            let proof = SscContainmentProof { event: &event, envelope: envelope_row("NOTE_VISIBILITY_EXACT").unwrap().envelope, certificate: SscConstructiveContainmentCertificate { checked_bytes: 0 } };
            assert!(matches!(observe_ssc_event_v2(&transformation, SscInvariantAssessment::Unproven, SscNecessityAssessment::Unproven, SscContainmentAssessment::Contained(&proof)), Err(SscEvidenceIntegrityFailure::ProofFamilyMismatch)));
        }
    }
}
#[cfg(test)]
mod governed_4b_witness_tests {
    // r1284 4b: the seven rows are frozen here as literal r1283 and r1284 rows, so these
    // tests never depend on the installed table; each mechanism comes from the A5 analysis.
    use super::*;
    fn literal(descriptor: &'static str, kind: SscEnvelopeKind, elements: &[&'static str], fields: &[(&'static str, char, &'static str)], operations: &[&'static str]) -> EnvelopeRow {
        EnvelopeRow { envelope: SscEnvelopeRef { descriptor, kind }, elements: elements.to_vec(), fields: fields.to_vec(), operations: operations.to_vec() }
    }
    fn escaped(pre: &str, post: &str, row: &EnvelopeRow) -> Option<SscExactDifferenceWitness> {
        match containment(pre, post, row) { Containment::Escaped(witness) => Some(witness), _ => None }
    }
    fn contained(pre: &str, post: &str, row: &EnvelopeRow) -> bool {
        matches!(containment(pre, post, row), Containment::Contained(_))
    }
    fn shown(witness: &SscExactDifferenceWitness) -> (&str, String, String) {
        (witness_basis(&witness.pre_bytes), witness_excerpt(&witness.pre_bytes, &witness.pre_range), witness_excerpt(&witness.post_bytes, &witness.post_range))
    }
    fn score(measure: &str) -> String {
        format!("<score-partwise><part id=\"P\"><measure>{measure}</measure></part></score-partwise>")
    }
    const NOTE: &str = "<note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>";

    #[test]
    fn minimal_witness_names_only_the_differing_span_on_char_boundaries() {
        let witness = minimal_witness("<a>xäy</a>", "<a>xöy</a>");
        assert_eq!((witness.pre_range.clone(), witness.post_range.clone()), (4..6, 4..6));
        assert_eq!(shown(&witness), ("document", "ä".to_owned(), "ö".to_owned()));
        let insertion = minimal_witness("<a></a>", "<a><b/></a>");
        assert_eq!((insertion.pre_range.clone(), shown(&insertion).2), (3..3, "<b/>".to_owned()));
        let same = minimal_witness("<a/>", "<a/>");
        assert!(same.pre_range.is_empty() && same.post_range.is_empty());
        let long = "x".repeat(300);
        let witness = minimal_witness("<a></a>", &format!("<a>{long}</a>"));
        assert_eq!(shown(&witness).2, format!("{}...", "x".repeat(120)));
    }

    #[test]
    fn note_visibility_print_spacing_is_named_and_then_declared() {
        let pre = score(NOTE);
        let post = pre.replacen("<note>", "<note print-object=\"no\" print-spacing=\"no\">", 1);
        let old = literal("NOTE_VISIBILITY_EXACT", SscEnvelopeKind::Edit, &["note"], &[("note", '@', "print-object")], &["change", "insert"]);
        let new = literal("NOTE_VISIBILITY_EXACT", SscEnvelopeKind::Edit, &["note"], &[("note", '@', "print-object"), ("note", '@', "print-spacing")], &["change"]);
        let witness = escaped(&pre, &post, &old).unwrap();
        assert_eq!(shown(&witness), ("canonical", String::new(), "|print-spacing=\"no\"".to_owned()));
        let start = pre.find("<note>").unwrap();
        let edits = [(start, start + "<note>".len(), "<note print-object=\"no\" print-spacing=\"no\">")];
        assert!(edit_script_locality(&pre, &post, &edits));
        assert!(matches!(undeclared_edit(&pre, &post, &edits, &old), Some(Some(_))));
        assert_eq!(undeclared_edit(&pre, &post, &edits, &new), Some(None));
        assert!(contained(&pre, &post, &new));
        assert!(escaped(&pre, &post.replacen("<step>C", "<step>D", 1), &new).is_some());
    }

    #[test]
    fn allowed_range_canonically_empty_notation_case_has_a_whole_element_witness() {
        let pre = score(&NOTE.replace("</voice>", "</voice><notations><slur type=\"start\" number=\"1\"/></notations>"));
        let post = pre.replacen("<slur type=\"start\" number=\"1\"/>", "", 1);
        let old = literal("ALLOWED_RANGE_REMOVAL_EXACT", SscEnvelopeKind::Edit, &["notations", "slur", "tie", "tied"], &[], &["remove"]);
        let new = literal("ALLOWED_RANGE_REMOVAL_EXACT", SscEnvelopeKind::Edit, &["notations", "slur", "tie", "tied", "attributes"], &[], &["change", "remove"]);
        let witness = escaped(&pre, &post, &old).unwrap();
        let (basis, before, after) = shown(&witness);
        assert_eq!(basis, "whole_elements");
        assert!(before.starts_with("<slur|number=\"1\"|type=\"start\">") && after.is_empty(), "{before:?} {after:?}");
        assert!(contained(&pre, &post, &new));
        assert!(escaped(&pre, &post.replacen("<step>C", "<step>D", 1), &new).is_some());
    }

    #[test]
    fn allowed_range_safe_lower_staff_clef_removal_is_named_and_then_declared() {
        let clef = "<attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>";
        let pre = score(&format!("{clef}{NOTE}"));
        let post = pre.replacen(clef, "", 1);
        let old = literal("ALLOWED_RANGE_REMOVAL_EXACT", SscEnvelopeKind::Edit, &["notations", "slur", "tie", "tied"], &[], &["remove"]);
        let new = literal("ALLOWED_RANGE_REMOVAL_EXACT", SscEnvelopeKind::Edit, &["notations", "slur", "tie", "tied", "attributes"], &[], &["change", "remove"]);
        let witness = escaped(&pre, &post, &old).unwrap();
        let (basis, before, after) = shown(&witness);
        assert_eq!((basis, after.as_str()), ("canonical", ""));
        assert!(before.starts_with("<attributes><clef|number=\"2\">"), "{before:?}");
        assert!(contained(&pre, &post, &new));
    }

    #[test]
    fn rest_display_placement_tag_form_is_named_and_then_declared() {
        let rest = "<note><rest measure=\"yes\"/><duration>4</duration><voice>1</voice></note>";
        let pre = score(rest);
        let post = pre.replacen("<rest measure=\"yes\"/>", "<rest><display-step>B</display-step><display-octave>4</display-octave></rest>", 1);
        let old = literal("RENDER_ROW_30_EXACT", SscEnvelopeKind::Edit, &["rest", "note"], &[("rest", '.', "display-step"), ("rest", '.', "display-octave"), ("note", '@', "default-y")], &["change"]);
        let new = literal("RENDER_ROW_30_EXACT", SscEnvelopeKind::Edit, &["rest", "note", "display-step", "display-octave"], &[("note", '@', "default-y")], &["change"]);
        let witness = escaped(&pre, &post, &old).unwrap();
        assert_eq!(shown(&witness), ("canonical", "|measure=\"yes\"".to_owned(), String::new()));
        let start = pre.find("<rest").unwrap();
        let edits = [(start, start + "<rest measure=\"yes\"/>".len(), "<rest><display-step>B</display-step><display-octave>4</display-octave></rest>")];
        assert!(edit_script_locality(&pre, &post, &edits));
        assert!(matches!(undeclared_edit(&pre, &post, &edits, &old), Some(Some(_))));
        assert_eq!(undeclared_edit(&pre, &post, &edits, &new), Some(None));
        assert!(contained(&pre, &post, &new));
        assert!(escaped(&pre, &post.replacen("<duration>4", "<duration>2", 1), &new).is_some());
    }

    #[test]
    fn missing_staff_edge_forward_insertion_is_named_and_then_declared() {
        let pre = score(NOTE);
        let payload = "<backup><duration>4</duration></backup><forward><duration>2</duration></forward><note><rest/><duration>2</duration><voice>2</voice></note>";
        let post = pre.replacen("</note>", &format!("</note>{payload}"), 1);
        let old = literal("RENDER_ROW_51_EXACT", SscEnvelopeKind::Edit, &["note", "backup"], &[], &["insert"]);
        let new = literal("RENDER_ROW_51_EXACT", SscEnvelopeKind::Edit, &["note", "backup", "forward"], &[], &["insert"]);
        let witness = escaped(&pre, &post, &old).unwrap();
        let (basis, before, after) = shown(&witness);
        assert_eq!((basis, before.as_str()), ("canonical", ""));
        assert!(after.starts_with("<forward><duration>"), "{after:?}");
        assert!(contained(&pre, &post, &new));
        let removed = pre.replacen(NOTE, "", 1);
        let witness = escaped(&pre, &removed, &new).unwrap();
        assert_eq!(shown(&witness).0, "whole_elements");
    }

    #[test]
    fn final_layout_ottava_recue_carrier_removal_is_named_and_then_declared() {
        let direction = "<direction><direction-type><words>8va</words></direction-type><direction-type><octave-shift type=\"continue\" number=\"1\"/></direction-type><staff>1</staff></direction>";
        let pre = score(&format!("{direction}{NOTE}"));
        let post = pre.replacen("<direction-type><words>8va</words></direction-type>", "", 1);
        let old = literal("FINAL_LAYOUT_PHRASE_PRESERVING", SscEnvelopeKind::Projection, &["notations", "slur", "print", "system-layout", "measure", "note"], &[("measure", '@', "width"), ("note", '@', "default-x")], &["change", "insert", "remove"]);
        let new = literal("FINAL_LAYOUT_PHRASE_PRESERVING", SscEnvelopeKind::Projection, &["notations", "slur", "print", "system-layout", "measure", "note", "direction-type", "words"], &[("measure", '@', "width"), ("note", '@', "default-x")], &["change", "insert", "remove"]);
        let witness = escaped(&pre, &post, &old).unwrap();
        // Exactly one generated carrier: the words and one direction-type wrapper.
        assert_eq!(shown(&witness), ("canonical", "<words>\"8va\"</words></direction-type><direction-type>".to_owned(), String::new()));
        assert!(contained(&pre, &post, &new));
        assert!(escaped(&pre, &post.replacen("<staff>1", "<staff>2", 1), &new).is_some());
    }

    #[test]
    fn tonal_key_container_insertion_is_named_and_then_declared() {
        let pre = score(NOTE);
        let post = pre.replacen("<note>", "<attributes><key><fifths>-3</fifths></key></attributes><note>", 1).replacen("<step>C</step>", "<step>B</step><alter>1</alter>", 1);
        let respell = post.replacen("<step>B</step><alter>1</alter>", "<step>C</step>", 1);
        let old_master = literal("FUNCTIONAL_RESPELLING_EXACT", SscEnvelopeKind::Edit, &["pitch", "accidental"], &[], &["change", "insert", "remove"]);
        let new_master = literal("FUNCTIONAL_RESPELLING_EXACT", SscEnvelopeKind::Edit, &["pitch", "accidental", "attributes", "key", "fifths"], &[], &["change", "insert", "remove"]);
        let old_render = literal("RENDER_ROW_66_BLOCK", SscEnvelopeKind::Block, &["pitch", "accidental", "key", "fifths", "mode", "cancel"], &[], &["change", "insert", "remove"]);
        let new_render = literal("RENDER_ROW_66_BLOCK", SscEnvelopeKind::Block, &["pitch", "accidental", "key", "fifths", "mode", "cancel", "attributes"], &[], &["change", "insert", "remove"]);
        let master = escaped(&pre, &post, &old_master).unwrap();
        assert_eq!(shown(&master), ("canonical", String::new(), "<attributes><key><fifths>\"-3\"</fifths></key></attributes>".to_owned()));
        // The A5 analysis measured +25 canonical bytes for #66: exactly the empty wrapper.
        let render = escaped(&pre, &respell, &old_render).unwrap();
        assert_eq!(shown(&render), ("canonical", String::new(), "<attributes></attributes>".to_owned()));
        assert_eq!(render.post_range.len(), 25);
        assert!(contained(&pre, &post, &new_master) && contained(&pre, &respell, &new_render));
        assert!(escaped(&pre, &post.replacen("<duration>4", "<duration>2", 1), &new_master).is_some());
    }

    #[test]
    fn denied_report_line_carries_basis_bounded_ranges_and_excerpts() {
        let receipt = crate::master_state_identity::tests::synthetic_exact_receipt_for_tests("<d>0</d>", "<d>1</d>", 3, 4, "1");
        let record = AdoptedEvidence { receipt: AdoptedReceipt::Exact(receipt), domain: SscAdoptionDomain::Master, additive_origin: None };
        let identity = *record.receipt.identity();
        let event = SscEventBinding { released: SscReleasedEventRef { domain: SscEvidenceDomain::Master, identity: &identity }, pre_state: "<d/>", post_state: "<d/>" };
        let family = SscProofFamily::F5AtomicVoiceStaffVisibility;
        let (receipt, _, _) = record.receipt.borrowed(None).unwrap();
        let transformation = SscTransformationEvidence { event: &event, receipt: SscOriginReceiptRef { inner: receipt }, scope: SscFamilyScope::InScope(family) };
        let proof = SscInvariantProof { event: &event, family, certificate: SscConstructiveInvariantCertificate { preserved_structure: SscCheckedStructurePreservation { normalized_bytes: 4 } } };
        let envelope = SscEnvelopeRef { descriptor: "STAFF_VOICE_MOVE_EXACT", kind: SscEnvelopeKind::Lane };
        let outside = SscContainmentCounterexample { event: &event, envelope, witness: basis_witness("whole_elements", "<notations><slur/></notations>", "<notations></notations>") };
        let classified = observe_ssc_event_v2(&transformation, SscInvariantAssessment::Proven(&proof), SscNecessityAssessment::Unproven, SscContainmentAssessment::Escaped(&outside)).unwrap();
        assert_eq!(classified.containment_verdict, SscContainmentVerdict::ContainmentDenied);
        let mut counts = ObservationCounts::default();
        counts.observe(SscReleasedObservationV2::Classified(classified));
        assert_eq!(counts.containment_witnesses, vec![
            "V4DeniedWitness: descriptor=STAFF_VOICE_MOVE_EXACT; reason=envelope_escape; basis=whole_elements; pre_range=32..39; post_range=32..32; pre_bound_bytes=51; post_bound_bytes=44; pre_excerpt=\"<slur/>\"; post_excerpt=\"\"\n".to_owned(),
        ]);
    }
}
#[cfg(test)]
pub(crate) fn ssc_edit_scope_on_block_probe_for_test(receipt: &BlockTransformationReceipt) -> bool {
    let identity = receipt.ssc_bound_origin().0;
    let event = SscEventBinding { released: SscReleasedEventRef { domain: SscEvidenceDomain::Master, identity }, pre_state: "", post_state: "" };
    let transformation = SscTransformationEvidence { event: &event, receipt: SscOriginReceiptRef { inner: SscOriginReceiptKind::Block(receipt) }, scope: SscFamilyScope::InScope(SscProofFamily::Edit) };
    matches!(observe_ssc_event_v2(&transformation, SscInvariantAssessment::Unproven, SscNecessityAssessment::Unproven, SscContainmentAssessment::Unproven), Err(SscEvidenceIntegrityFailure::ProofFamilyMismatch))
}
