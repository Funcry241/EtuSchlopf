//! Productive phrase-slur materialization inside the already accepted final
//! StructuralWriteAuthority transaction.
//!
//! r146 keeps the writer-root set established by r144 unchanged. All helpers
//! below are read-only over MusicXML and typed phrase intents: they only locate,
//! validate, and return immutable insertion plans. The actual String/XML
//! mutation and persisted render write happen only in the already accepted
//! `materialize_final_render_notation_with_phrase_projection_authorized` root.

use crate::audit_only::phrase_boundary_evidence::PhraseNotationIntentProjection;
use crate::master_state_identity::PreservingTransformationReceipt;
use anyhow::{bail, Context, Result};
use roxmltree::Document;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::horizontal_layout_plan_materialization::materialize_horizontal_layout_file_authorized;
use super::HorizontalLayoutMaterializationDirective;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RenderNoteFingerprint {
    part: String,
    measure_ordinal: usize,
    staff: String,
    voice: String,
    onset: i64,
    duration: i64,
    pitch: Option<i32>,
    visible: bool,
    chord_member: bool,
}

#[derive(Debug, Clone)]
struct RenderNoteEvent {
    fingerprint: RenderNoteFingerprint,
    note_start: usize,
    note_end: usize,
}

/// Existing E007 policy decision, sealed on actual pre-partition occurrences.
/// This value carries the owner's decision; it is never reconstructed from a report.
#[derive(Debug, Clone)]
pub(crate) struct ApprovedPhraseOccurrenceCourse {
    endpoints: Vec<(PhraseNotationIntentProjection,
        super::serialization_plan::PhraseEndpointOccurrence,
        super::serialization_plan::PhraseEndpointOccurrence)>,
}

#[derive(Debug, Clone)]
pub(crate) struct BoundPhraseOccurrenceCourse {
    source: String,
    endpoints: Vec<(PhraseNotationIntentProjection, std::ops::Range<usize>, std::ops::Range<usize>)>,
}

pub(crate) fn approve_phrase_occurrence_course(
    source: &super::FinalPdfSourceOccurrenceContinuation,
    intents: &[PhraseNotationIntentProjection],
) -> Result<ApprovedPhraseOccurrenceCourse> {
    let approved = super::phrase_notation_policy::approve_pending_visual_intents_from_musicxml(source.xml(), intents)?;
    eprintln!("PhraseOccurrenceDecisionCourse: pending={}; approved={:?}; source_fnv64={:016x}; authority=existing_unchanged_phrase_policy",intents.len(),approved.iter().map(|i|(&i.staff,&i.voice,i.start_measure_ordinal,i.start_onset,i.end_measure_ordinal,i.end_onset)).collect::<Vec<_>>(),super::authority_primitives::fingerprint(source.xml()));
    let notes = parse_render_note_events(source.xml());
    let mut endpoints = Vec::new();
    for intent in approved {
        if intent.start_pitches.len() != 1 || intent.end_pitches.len() != 1
            || intent.end_canonical <= intent.start_canonical { continue; }
        let start = resolve_anchor(&notes, &intent.part, &intent.staff, &intent.voice,
            intent.start_measure_ordinal, intent.start_onset, intent.start_pitches[0]);
        let end = resolve_anchor(&notes, &intent.part, &intent.staff, &intent.voice,
            intent.end_measure_ordinal, intent.end_onset, intent.end_pitches[0]);
        let (AnchorResolution::Unique(start), AnchorResolution::Unique(end)) = (start, end) else { continue; };
        let a = &notes[start]; let b = &notes[end];
        endpoints.push((intent, source.bind_phrase_endpoint(a.note_start..a.note_end)?,
            source.bind_phrase_endpoint(b.note_start..b.note_end)?));
    }
    eprintln!("PhraseOccurrenceBoundCourse: endpoints={}; authority=actual_original_occurrence_receipt",endpoints.len());
    Ok(ApprovedPhraseOccurrenceCourse { endpoints })
}

impl ApprovedPhraseOccurrenceCourse {
    pub(crate) fn carry(&self, source: &super::FinalPdfSourceOccurrenceContinuation) -> Result<BoundPhraseOccurrenceCourse> {
        let mut endpoints = Vec::new();
        for (intent, start, end) in &self.endpoints {
            endpoints.push((intent.clone(), source.carry_phrase_endpoint(start)?, source.carry_phrase_endpoint(end)?));
        }
        Ok(BoundPhraseOccurrenceCourse { source: source.xml().to_owned(), endpoints })
    }
}

impl BoundPhraseOccurrenceCourse {
    fn plan_after_layout(&self, source: &str, layout: &str,
        directives: &[HorizontalLayoutMaterializationDirective]) -> Result<Option<PhraseSlurEditPlan>> {
        anyhow::ensure!(source == self.source, "E007 phrase course belongs to another sovereign document");
        let copies = super::horizontal_layout_plan_materialization::observe_original_horizontal_layout_copies(source, directives)?;
        anyhow::ensure!(copies.documents().1 == layout, "E007 phrase course layout differs from actual execution");
        let mut intents = Vec::new(); let mut anchors = Vec::new();
        for (intent, start, end) in &self.endpoints {
            intents.push(intent.clone()); anchors.push((copies.carry(start.clone())?, copies.carry(end.clone())?));
        }
        build_phrase_slur_edit_plan_with_course(layout, &intents, Some(&anchors))
    }
}

#[derive(Debug, Clone)]
struct ResolvedPhraseSlur {
    part: String,
    start_canonical: i128,
    end_canonical: i128,
    start_note_start: usize,
    start_note_end: usize,
    end_note_start: usize,
    end_note_end: usize,
    number: usize,
    crosses_system: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnchorResolution {
    Missing,
    Ambiguous,
    Unique(usize),
}

#[derive(Debug, Clone)]
struct PartRange {
    id: String,
    content_start: usize,
    content_end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PhraseSlurInsertion {
    position: usize,
    text: String,
}

#[derive(Debug, Clone)]
struct PhraseSlurEditPlan {
    insertions: Vec<PhraseSlurInsertion>,
    expected_pairs: BTreeMap<usize, usize>,
    before_note_count: usize,
    before_fingerprint: Vec<RenderNoteFingerprint>,
}

/// Sealed PRESERVING mint capability of frozen exclusion E007. E007 is the
/// semantic owner of the final layout/phrase transaction; the E011 file sink it
/// calls stays an implementation transport and mints nothing.
#[derive(Debug, Clone, Copy)]
pub(crate) struct FinalLayoutPhrasePreservingMint {
    _sealed: (),
}

/// Observation of the original two-phase E007 execution. Only the E007 owner
/// can assemble it: E012's constructive copy relation followed by its actual
/// approved phrase insertion plan. It carries no write or adoption authority.
#[derive(Debug)]
pub(crate) struct FinalLayoutCourseTransport {
    layout: super::horizontal_layout_plan_materialization::OriginalHorizontalLayoutCopies,
    insertions: Vec<PhraseSlurInsertion>,
    after: String,
}

impl FinalLayoutCourseTransport {
    fn observe(
        layout: Result<super::horizontal_layout_plan_materialization::OriginalHorizontalLayoutCopies>,
        insertions: Vec<PhraseSlurInsertion>, after: &str,
    ) -> Result<Self> {
        let layout = layout?;
        let source = layout.documents().1;
        let mut pre_cursor = 0;
        let mut post_cursor = 0;
        for insertion in &insertions {
            let copied = source.get(pre_cursor..insertion.position)
                .ok_or_else(|| anyhow::anyhow!("Invalid original E007 insertion order"))?;
            anyhow::ensure!(after.get(post_cursor..post_cursor + copied.len()) == Some(copied),
                "E007 actual phrase execution changed an original copied segment");
            post_cursor += copied.len();
            anyhow::ensure!(after.get(post_cursor..post_cursor + insertion.text.len())
                == Some(insertion.text.as_str()), "E007 actual phrase insertion differs from its plan");
            post_cursor += insertion.text.len();
            pre_cursor = insertion.position;
        }
        anyhow::ensure!(source.get(pre_cursor..) == after.get(post_cursor..),
            "E007 actual final bytes differ from the original layout and phrase plan");
        Ok(Self { layout, insertions, after: after.to_owned() })
    }

    pub(crate) fn documents(&self) -> (&str, &str) { (self.layout.documents().0, &self.after) }

    pub(crate) fn carry(&self, range: std::ops::Range<usize>) -> Result<std::ops::Range<usize>> {
        let layout = self.layout.carry(range)?;
        let prefix = self.insertions.iter().filter(|insertion| insertion.position <= layout.start)
            .map(|insertion| insertion.text.len()).sum::<usize>();
        let inside = self.insertions.iter().filter(|insertion|
            insertion.position > layout.start && insertion.position < layout.end)
            .map(|insertion| insertion.text.len()).sum::<usize>();
        Ok(layout.start + prefix..layout.end + prefix + inside)
    }
}

pub(crate) fn materialize_final_render_notation_with_phrase_projection_authorized(
    path: &Path,
    directives: &[HorizontalLayoutMaterializationDirective],
    phrase_notation_intents: &[PhraseNotationIntentProjection],
) -> (Result<usize>, Option<PreservingTransformationReceipt>) {
    // The actual pre bytes are captured before the E011 sink runs, so the
    // PRESERVING receipt binds the real pre -> post transition of this owner.
    let pre_bytes = match load_musicxml_text(path) {
        Ok(pre_bytes) => pre_bytes,
        Err(error) => return (Err(error), None),
    };
    let layout_copies = super::horizontal_layout_plan_materialization::
        observe_original_horizontal_layout_copies(&pre_bytes, directives);
    let mut original_insertions = Vec::new();
    let transaction = (|| -> Result<usize> {
        let changed_slots = materialize_horizontal_layout_file_authorized(path, directives)?;
        if phrase_notation_intents.is_empty() {
            return Ok(changed_slots);
        }

        let source_xml = load_musicxml_text(path)?;
        let approved = super::phrase_notation_policy::approve_pending_visual_intents_from_musicxml(
            &source_xml,
            phrase_notation_intents,
        )?;
        if approved.is_empty() {
            return Ok(changed_slots);
        }
        let Some(plan) = build_phrase_slur_edit_plan(&source_xml, &approved)? else {
            return Ok(changed_slots);
        };

        original_insertions = plan.insertions.clone();

        let mut candidate_xml = source_xml.clone();
        for insertion in plan.insertions.iter().rev() {
            candidate_xml.insert_str(insertion.position, &insertion.text);
        }

        if Document::parse(&candidate_xml).is_err() {
            bail!("phrase-slur render candidate is not parseable MusicXML");
        }
        let after_notes = parse_render_note_events(&candidate_xml);
        if plan.before_note_count != after_notes.len() {
            bail!("phrase-slur render candidate changed note-event cardinality");
        }
        if plan.before_fingerprint != render_note_fingerprint(&after_notes) {
            bail!("phrase-slur render candidate changed sounding/timeline note truth");
        }
        if slur_pair_counts(&candidate_xml) != plan.expected_pairs {
            bail!("phrase-slur render candidate is not exactly pair-balanced by number");
        }

        let writer_trace_before = crate::audit_only::writer_trace::capture_before(path);
        std::fs::write(path, candidate_xml.as_bytes()).with_context(|| {
            format!(
                "phrase-slur verified render candidate could not be persisted: {}",
                path.display()
            )
        })?;

        let persisted_xml = load_musicxml_text(path)?;
        let persisted_notes = parse_render_note_events(&persisted_xml);
        let persisted_green = persisted_xml == candidate_xml
            && Document::parse(&persisted_xml).is_ok()
            && plan.before_note_count == persisted_notes.len()
            && plan.before_fingerprint == render_note_fingerprint(&persisted_notes)
            && slur_pair_counts(&persisted_xml) == plan.expected_pairs;
        if !persisted_green {
            let _ = std::fs::write(path, source_xml.as_bytes());
            bail!("phrase-slur persisted candidate verification failed; source bytes restored");
        }

        crate::audit_only::writer_trace::record_after(
            "phrase_slur_policy_persistence",
            path,
            writer_trace_before.as_deref(),
            &persisted_xml,
        );

        Ok(changed_slots)
    })();
    let changed_slots = match transaction {
        Ok(changed_slots) => changed_slots,
        Err(error) => return (Err(error), None),
    };
    let post_bytes = match load_musicxml_text(path) {
        Ok(post_bytes) => post_bytes,
        Err(error) => return (Err(error), None),
    };
    let course_transport = FinalLayoutCourseTransport::observe(layout_copies, original_insertions, &post_bytes);
    let receipt = PreservingTransformationReceipt::mint_file_bound_preserving_receipt(
        FinalLayoutPhrasePreservingMint { _sealed: () },
        &pre_bytes,
        post_bytes,
    ).map(|receipt| receipt.with_final_layout_course_transport(course_transport));
    (Ok(changed_slots), receipt)
}

#[derive(Debug)]
pub(super) struct PreparedFinalLayoutPhrase {
    pre: String,
    layout_post: String,
    final_post: String,
    changed_slots: usize,
    phrase_write: bool,
    course_transport: Result<FinalLayoutCourseTransport>,
}

impl PreparedFinalLayoutPhrase {
    pub(super) fn final_post(&self) -> &str { &self.final_post }
    pub(super) fn changed_slots(&self) -> usize { self.changed_slots }
    pub(super) fn into_parts(self) -> (String, String, String, usize, bool) {
        (self.pre, self.layout_post, self.final_post, self.changed_slots, self.phrase_write)
    }
}

pub(super) fn prepare_final_render_notation_with_phrase_projection(
    source: &str,
    directives: &[HorizontalLayoutMaterializationDirective],
    phrase_notation_intents: &[PhraseNotationIntentProjection],
) -> (Result<PreparedFinalLayoutPhrase>, Option<PreservingTransformationReceipt>) {
    prepare_final_render_notation_with_phrase_course(source, directives, phrase_notation_intents, None)
}

pub(super) fn prepare_final_render_notation_with_phrase_course(
    source: &str, directives: &[HorizontalLayoutMaterializationDirective],
    phrase_notation_intents: &[PhraseNotationIntentProjection],
    course: Option<&BoundPhraseOccurrenceCourse>,
) -> (Result<PreparedFinalLayoutPhrase>, Option<PreservingTransformationReceipt>) {
    let result = (|| -> Result<PreparedFinalLayoutPhrase> {
        let layout_copies = super::horizontal_layout_plan_materialization::
            observe_original_horizontal_layout_copies(source, directives);
        let mut layout_post = source.to_owned();
        let changed_slots = super::horizontal_layout_plan_materialization::
            materialize_horizontal_layout_plan_authorized(&mut layout_post, directives)?;
        let mut final_post = layout_post.clone();
        let mut phrase_write = false;
        let mut original_insertions = Vec::new();
        if !phrase_notation_intents.is_empty() || course.is_some() {
            let plan = if let Some(course) = course {
                course.plan_after_layout(source, &layout_post, directives)?
            } else {
                let approved = super::phrase_notation_policy::approve_pending_visual_intents_from_musicxml(&layout_post, phrase_notation_intents)?;
                build_phrase_slur_edit_plan(&layout_post, &approved)?
            };
            if let Some(plan) = plan {
                    original_insertions = plan.insertions.clone();
                    let insertions = plan.insertions.iter()
                        .map(|insertion| (insertion.position, insertion.text.clone()))
                        .collect::<Vec<_>>();
                    final_post = super::master_adoption::mutate_final_layout_phrase_insertions(
                        &layout_post, &insertions,
                    )?;
                    // Die vier heutigen E007-Checks, unverÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¤ndert vorgezogen.
                    if Document::parse(&final_post).is_err() {
                        bail!("phrase-slur render candidate is not parseable MusicXML");
                    }
                    let after_notes = parse_render_note_events(&final_post);
                    if plan.before_note_count != after_notes.len() {
                        bail!("phrase-slur render candidate changed note-event cardinality");
                    }
                    if plan.before_fingerprint != render_note_fingerprint(&after_notes) {
                        bail!("phrase-slur render candidate changed sounding/timeline note truth");
                    }
                    if slur_pair_counts(&final_post) != plan.expected_pairs {
                        bail!("phrase-slur render candidate is not exactly pair-balanced by number");
                    }
                    phrase_write = true;
            }
        }
        Ok(PreparedFinalLayoutPhrase {
            course_transport: FinalLayoutCourseTransport::observe(layout_copies, original_insertions, &final_post),
            pre: source.to_owned(), layout_post, final_post, changed_slots, phrase_write,
        })
    })();
    let mut result = result;
    let receipt = match result.as_mut() {
        Ok(prepared) => PreservingTransformationReceipt::mint_file_bound_preserving_receipt(
            FinalLayoutPhrasePreservingMint { _sealed: () },
            source,
            prepared.final_post.clone(),
        ).map(|receipt| receipt.with_final_layout_course_transport(std::mem::replace(
            &mut prepared.course_transport, Err(anyhow::anyhow!("E007 course observation already moved"))))),
        Err(_) => None,
    };
    if let Ok(prepared) = result.as_ref()
        && source != prepared.final_post() && receipt.is_none() {
            return (Err(anyhow::anyhow!("E007 changed prepared post without its own PR")), None);
    }
    (result, receipt)
}

fn load_musicxml_text(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| {
        format!("MusicXML source could not be read: {}", path.display())
    })
}

fn build_phrase_slur_edit_plan(
    source_xml: &str,
    intents: &[PhraseNotationIntentProjection],
) -> Result<Option<PhraseSlurEditPlan>> {
    build_phrase_slur_edit_plan_with_course(source_xml, intents, None)
}

type CarriedPhraseAnchors = (std::ops::Range<usize>, std::ops::Range<usize>);
fn build_phrase_slur_edit_plan_with_course(
    source_xml: &str, intents: &[PhraseNotationIntentProjection],
    carried: Option<&[CarriedPhraseAnchors]>,
) -> Result<Option<PhraseSlurEditPlan>> {
    anyhow::ensure!(carried.is_none_or(|anchors| anchors.len() == intents.len()), "E007 phrase course cardinality drift");
    // Precision first: an already-owned slur surface is not merged in the first
    // productive take. This avoids guessing about ownership or number reuse.
    if intents.is_empty() || source_xml.contains("<slur") {
        return Ok(None);
    }

    let notes = parse_render_note_events(source_xml);
    let system_starts = collect_system_start_ordinals(source_xml);
    let mut resolved = Vec::<ResolvedPhraseSlur>::new();

    for (intent_index, intent) in intents.iter().enumerate() {
        if intent.start_pitches.len() != 1
            || intent.end_pitches.len() != 1
            || intent.end_canonical <= intent.start_canonical
        {
            continue;
        }

        let start = resolve_anchor(
            &notes,
            &intent.part,
            &intent.staff,
            &intent.voice,
            intent.start_measure_ordinal,
            intent.start_onset,
            intent.start_pitches[0],
        );
        let end = resolve_anchor(
            &notes,
            &intent.part,
            &intent.staff,
            &intent.voice,
            intent.end_measure_ordinal,
            intent.end_onset,
            intent.end_pitches[0],
        );
        let (start, end) = if let Some(anchors) = carried {
            let (start_range, end_range) = &anchors[intent_index];
            let resolve_range = |range: &std::ops::Range<usize>| -> Result<AnchorResolution> {
                let matching = notes.iter().enumerate().filter(|(_, note)|
                    note.note_start == range.start && note.note_end == range.end).collect::<Vec<_>>();
                anyhow::ensure!(matching.len() == 1, "E007 carried endpoint is not one actual note");
                Ok(AnchorResolution::Unique(matching[0].0))
            };
            (resolve_range(start_range)?, resolve_range(end_range)?)
        } else { (start, end) };
        let (AnchorResolution::Unique(start_index), AnchorResolution::Unique(end_index)) =
            (start, end)
        else {
            continue;
        };
        let start_note = &notes[start_index];
        let end_note = &notes[end_index];
        if start_note.note_start == end_note.note_start {
            continue;
        }

        resolved.push(ResolvedPhraseSlur {
            part: intent.part.clone(),
            start_canonical: intent.start_canonical,
            end_canonical: intent.end_canonical,
            start_note_start: start_note.note_start,
            start_note_end: start_note.note_end,
            end_note_start: end_note.note_start,
            end_note_end: end_note.note_end,
            number: 0,
            crosses_system: crosses_materialized_system(intent, &system_starts),
        });
    }

    resolved.sort_by(|left, right| {
        (
            &left.part,
            left.start_canonical,
            left.end_canonical,
            left.start_note_start,
        )
            .cmp(&(
                &right.part,
                right.start_canonical,
                right.end_canonical,
                right.start_note_start,
            ))
    });

    let mut active = BTreeMap::<String, Vec<(i128, usize)>>::new();
    let mut numbered = Vec::<ResolvedPhraseSlur>::new();
    // Retain the exact legacy numbering for every previously admitted relation.
    for mut candidate in resolved.iter().filter(|relation| !relation.crosses_system).cloned() {
        let part_active = active.entry(candidate.part.clone()).or_default();
        part_active.retain(|(end, _)| *end > candidate.start_canonical);
        let used = part_active
            .iter()
            .map(|(_, number)| *number)
            .collect::<BTreeSet<_>>();
        let Some(number) = (1..=super::PHRASE_SLUR_NUMBER_LEVEL_MAXIMUM)
            .find(|number| !used.contains(number))
        else {
            continue;
        };
        candidate.number = number;
        part_active.push((candidate.end_canonical, number));
        numbered.push(candidate);
    }

    // A new crossing relation may use only a number that is free over its
    // complete interval. Check future as well as preceding legacy relations,
    // so adding a crossing never removes or renumbers an existing slur.
    for mut candidate in resolved.into_iter().filter(|relation| relation.crosses_system) {
        let Some(number) = (1..=super::PHRASE_SLUR_NUMBER_LEVEL_MAXIMUM).find(|number| {
            !numbered.iter().any(|existing| {
                existing.part == candidate.part && existing.number == *number
                    && existing.start_canonical < candidate.end_canonical
                    && candidate.start_canonical < existing.end_canonical
            })
        }) else {
            continue;
        };
        candidate.number = number;
        numbered.push(candidate);
    }

    if numbered.is_empty() {
        return Ok(None);
    }

    let mut marker_map = BTreeMap::<usize, (usize, Vec<(bool, usize)>)>::new();
    let mut expected_pairs = BTreeMap::<usize, usize>::new();
    for candidate in &numbered {
        marker_map
            .entry(candidate.start_note_start)
            .or_insert_with(|| (candidate.start_note_end, Vec::new()))
            .1
            .push((true, candidate.number));
        marker_map
            .entry(candidate.end_note_start)
            .or_insert_with(|| (candidate.end_note_end, Vec::new()))
            .1
            .push((false, candidate.number));
        *expected_pairs.entry(candidate.number).or_insert(0) += 1;
    }

    let mut insertions = Vec::<PhraseSlurInsertion>::new();
    for (note_start, (note_end, mut markers)) in marker_map {
        markers.sort_by_key(|(start, number)| (if *start { 1usize } else { 0usize }, *number));
        let fragment = source_xml
            .get(note_start..note_end)
            .ok_or_else(|| anyhow::anyhow!("phrase-slur anchor fragment escaped source bounds"))?;
        let insertion = slur_insertion_for_note(note_start, fragment, &markers).ok_or_else(|| {
            anyhow::anyhow!("phrase-slur anchor has no safe notations insertion point")
        })?;
        insertions.push(insertion);
    }
    insertions.sort_by_key(|insertion| insertion.position);

    Ok(Some(PhraseSlurEditPlan {
        insertions,
        expected_pairs,
        before_note_count: notes.len(),
        before_fingerprint: render_note_fingerprint(&notes),
    }))
}

fn resolve_anchor(
    notes: &[RenderNoteEvent],
    part: &str,
    staff: &str,
    voice: &str,
    measure_ordinal: usize,
    onset: i64,
    pitch: i32,
) -> AnchorResolution {
    let matches = notes
        .iter()
        .enumerate()
        .filter(|(_, note)| {
            let fingerprint = &note.fingerprint;
            fingerprint.part == part
                && fingerprint.measure_ordinal == measure_ordinal
                && fingerprint.staff == staff
                && fingerprint.voice == voice
                && fingerprint.onset == onset
                && fingerprint.pitch == Some(pitch)
                && fingerprint.visible
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => AnchorResolution::Missing,
        [index] => AnchorResolution::Unique(*index),
        _ => AnchorResolution::Ambiguous,
    }
}

fn crosses_materialized_system(
    intent: &PhraseNotationIntentProjection,
    system_starts: &BTreeMap<String, BTreeSet<usize>>,
) -> bool {
    if intent.end_measure_ordinal <= intent.start_measure_ordinal {
        return false;
    }
    system_starts.get(&intent.part).is_some_and(|starts| {
        starts
            .range(intent.start_measure_ordinal.saturating_add(1)..=intent.end_measure_ordinal)
            .next()
            .is_some()
    })
}

fn collect_system_start_ordinals(xml: &str) -> BTreeMap<String, BTreeSet<usize>> {
    let mut out = BTreeMap::<String, BTreeSet<usize>>::new();
    for part in part_ranges(xml) {
        let mut starts = BTreeSet::new();
        let mut measure_cursor = part.content_start;
        let mut ordinal = 0usize;
        while let Some(measure_start) =
            find_open_tag(xml, measure_cursor, part.content_end, "measure")
        {
            let Some(open_end) = xml[measure_start..part.content_end]
                .find('>')
                .map(|value| measure_start + value)
            else {
                break;
            };
            let Some(close_rel) = xml[open_end..part.content_end].find("</measure>") else {
                break;
            };
            let measure_end = open_end + close_rel + "</measure>".len();
            ordinal = ordinal.saturating_add(1);
            let body = &xml[open_end + 1..measure_end - "</measure>".len()];
            if body.contains("new-system=\"yes\"")
                || body.contains("new-system='yes'")
                || body.contains("new-page=\"yes\"")
                || body.contains("new-page='yes'")
            {
                starts.insert(ordinal);
            }
            measure_cursor = measure_end;
        }
        out.insert(part.id, starts);
    }
    out
}

fn part_ranges(xml: &str) -> Vec<PartRange> {
    let mut parts = Vec::new();
    let mut cursor = 0usize;
    while let Some(start) = find_open_tag(xml, cursor, xml.len(), "part") {
        let Some(open_end) = xml[start..].find('>').map(|value| start + value) else {
            break;
        };
        let Some(id) = attr_value(&xml[start..=open_end], "id") else {
            cursor = open_end.saturating_add(1);
            continue;
        };
        let Some(close_rel) = xml[open_end + 1..].find("</part>") else {
            break;
        };
        let close_start = open_end + 1 + close_rel;
        parts.push(PartRange {
            id,
            content_start: open_end + 1,
            content_end: close_start,
        });
        cursor = close_start + "</part>".len();
    }
    parts
}

fn parse_render_note_events(xml: &str) -> Vec<RenderNoteEvent> {
    let mut notes = Vec::new();
    for part in part_ranges(xml) {
        let mut measure_cursor = part.content_start;
        let mut measure_ordinal = 0usize;
        while let Some(measure_start) =
            find_open_tag(xml, measure_cursor, part.content_end, "measure")
        {
            let Some(open_end) = xml[measure_start..part.content_end]
                .find('>')
                .map(|value| measure_start + value)
            else {
                break;
            };
            let Some(close_rel) = xml[open_end + 1..part.content_end].find("</measure>") else {
                break;
            };
            let measure_close = open_end + 1 + close_rel;
            measure_ordinal = measure_ordinal.saturating_add(1);
            parse_measure_note_events(
                xml,
                &part.id,
                measure_ordinal,
                open_end + 1,
                measure_close,
                &mut notes,
            );
            measure_cursor = measure_close + "</measure>".len();
        }
    }
    notes
}

fn parse_measure_note_events(
    xml: &str,
    part: &str,
    measure_ordinal: usize,
    start: usize,
    end: usize,
    notes: &mut Vec<RenderNoteEvent>,
) {
    let mut cursor = 0_i64;
    let mut last_non_chord_start = 0_i64;
    let mut pos = start;
    while pos < end {
        let next_note = find_open_tag(xml, pos, end, "note").map(|value| (value, "note"));
        let next_backup = find_open_tag(xml, pos, end, "backup").map(|value| (value, "backup"));
        let next_forward = find_open_tag(xml, pos, end, "forward").map(|value| (value, "forward"));
        let Some((tag_start, tag)) = [next_note, next_backup, next_forward]
            .into_iter()
            .flatten()
            .min_by_key(|(value, _)| *value)
        else {
            break;
        };
        let close_name = format!("</{tag}>");
        let Some(close_rel) = xml[tag_start..end].find(&close_name) else {
            break;
        };
        let block_end = tag_start + close_rel + close_name.len();
        let block = &xml[tag_start..block_end];
        let duration = child_text(block, "duration")
            .and_then(|value| value.trim().parse::<i64>().ok())
            .unwrap_or(0)
            .max(0);
        match tag {
            "backup" => cursor = cursor.saturating_sub(duration),
            "forward" => cursor = cursor.saturating_add(duration),
            "note" => {
                let chord_member = block.contains("<chord");
                let onset = if chord_member {
                    last_non_chord_start
                } else {
                    cursor
                };
                let open_end = block.find('>').unwrap_or(0);
                let visible = !block[..open_end.min(block.len())].contains("print-object=\"no\"")
                    && !block[..open_end.min(block.len())].contains("print-object='no'");
                notes.push(RenderNoteEvent {
                    fingerprint: RenderNoteFingerprint {
                        part: part.to_string(),
                        measure_ordinal,
                        staff: child_text(block, "staff").unwrap_or_else(|| "1".to_string()),
                        voice: child_text(block, "voice").unwrap_or_else(|| "1".to_string()),
                        onset,
                        duration,
                        pitch: pitch_midi(block),
                        visible,
                        chord_member,
                    },
                    note_start: tag_start,
                    note_end: block_end,
                });
                if !chord_member {
                    last_non_chord_start = onset;
                    cursor = cursor.saturating_add(duration);
                }
            }
            _ => {}
        }
        pos = block_end;
    }
}

fn render_note_fingerprint(notes: &[RenderNoteEvent]) -> Vec<RenderNoteFingerprint> {
    notes
        .iter()
        .map(|note| note.fingerprint.clone())
        .collect()
}

fn slur_insertion_for_note(
    note_start: usize,
    fragment: &str,
    markers: &[(bool, usize)],
) -> Option<PhraseSlurInsertion> {
    if fragment.contains("<slur") {
        return None;
    }
    let marker_text = markers
        .iter()
        .map(|(start, number)| {
            format!(
                "          <slur type=\"{}\" number=\"{}\"/>\n",
                if *start { "start" } else { "stop" },
                number,
            )
        })
        .collect::<String>();
    if let Some(position) = fragment.rfind("</notations>") {
        return Some(PhraseSlurInsertion {
            position: note_start + position,
            text: marker_text,
        });
    }
    let position = fragment.rfind("</note>")?;
    Some(PhraseSlurInsertion {
        position: note_start + position,
        text: format!("      <notations>\n{marker_text}      </notations>\n"),
    })
}

fn slur_pair_counts(xml: &str) -> BTreeMap<usize, usize> {
    let mut starts = BTreeMap::<usize, usize>::new();
    let mut stops = BTreeMap::<usize, usize>::new();
    let mut cursor = 0usize;
    while let Some(relative) = xml[cursor..].find("<slur") {
        let tag_start = cursor + relative;
        let Some(tag_end_relative) = xml[tag_start..].find('>') else {
            break;
        };
        let tag_end = tag_start + tag_end_relative;
        let tag = &xml[tag_start..=tag_end];
        let number = attr_value(tag, "number").and_then(|value| value.parse::<usize>().ok());
        let slur_type = attr_value(tag, "type");
        if let (Some(number), Some(slur_type)) = (number, slur_type) {
            match slur_type.as_str() {
                "start" => *starts.entry(number).or_insert(0) += 1,
                "stop" => *stops.entry(number).or_insert(0) += 1,
                _ => {}
            }
        }
        cursor = tag_end.saturating_add(1);
    }

    let mut pairs = BTreeMap::<usize, usize>::new();
    for number in starts.keys().chain(stops.keys()).copied().collect::<BTreeSet<_>>() {
        let start_count = starts.get(&number).copied().unwrap_or(0);
        let stop_count = stops.get(&number).copied().unwrap_or(0);
        if start_count == stop_count && start_count > 0 {
            pairs.insert(number, start_count);
        } else {
            pairs.insert(number, usize::MAX);
        }
    }
    pairs
}

fn pitch_midi(block: &str) -> Option<i32> {
    let step = child_text(block, "step")?;
    let octave = child_text(block, "octave")?.parse::<i32>().ok()?;
    let alter = child_text(block, "alter")
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(0);
    let pitch_class = match step.trim() {
        "C" => 0,
        "D" => 2,
        "E" => 4,
        "F" => 5,
        "G" => 7,
        "A" => 9,
        "B" => 11,
        _ => return None,
    } + alter;
    Some((octave + 1).saturating_mul(12).saturating_add(pitch_class))
}

fn child_text(block: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = block.find(&open)? + open.len();
    let end = start + block[start..].find(&close)?;
    Some(block[start..end].trim().to_string())
}

fn find_open_tag(xml: &str, from: usize, end: usize, tag: &str) -> Option<usize> {
    let needle = format!("<{tag}");
    let mut cursor = from;
    while cursor < end {
        let relative = xml[cursor..end].find(&needle)?;
        let start = cursor + relative;
        let boundary = xml.as_bytes().get(start + needle.len()).copied();
        if boundary.is_some_and(|byte| byte == b'>' || byte == b'/' || byte.is_ascii_whitespace()) {
            return Some(start);
        }
        cursor = start.saturating_add(needle.len());
    }
    None
}

fn attr_value(tag: &str, name: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let needle = format!("{name}={quote}");
        if let Some(start) = tag.find(&needle) {
            let value_start = start + needle.len();
            let value_end = value_start + tag[value_start..].find(quote)?;
            return Some(tag[value_start..value_end].to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn projection(
        start_measure: usize,
        start_onset: i64,
        end_measure: usize,
        end_onset: i64,
    ) -> PhraseNotationIntentProjection {
        PhraseNotationIntentProjection {
            part: "P1".to_string(),
            staff: "1".to_string(),
            voice: "1".to_string(),
            start_measure_ordinal: start_measure,
            start_onset,
            start_canonical: i128::from(start_onset)
                + i128::try_from(start_measure).unwrap_or(0) * 1000,
            start_pitches: vec![60],
            end_measure_ordinal: end_measure,
            end_onset,
            end_canonical: i128::from(end_onset)
                + i128::try_from(end_measure).unwrap_or(0) * 1000,
            end_pitches: vec![62],
            attack_count: 2,
        }
    }

    fn xml(system_break_measure_two: bool) -> String {
        format!(
            r#"<?xml version="1.0"?><score-partwise version="3.1"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="1"><attributes><divisions>4</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff><notations><articulations><tenuto/></articulations></notations></note></measure><measure number="2">{}<note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>"#,
            if system_break_measure_two {
                "<print new-system=\"yes\"/>"
            } else {
                ""
            },
        )
    }

    #[test]
    fn unique_same_system_anchors_plan_balanced_render_only_slur_markup() {
        let source = xml(false);
        let plan = build_phrase_slur_edit_plan(&source, &[projection(1, 0, 1, 4)])
            .expect("plan")
            .expect("productive plan");
        assert_eq!(plan.expected_pairs, BTreeMap::from([(1, 1)]));
        assert_eq!(plan.insertions.len(), 2);
        assert!(plan
            .insertions
            .iter()
            .any(|insertion| insertion.text.contains("type=\"start\" number=\"1\"")));
        assert!(plan
            .insertions
            .iter()
            .any(|insertion| insertion.text.contains("type=\"stop\" number=\"1\"")));
        assert_eq!(plan.before_note_count, 4);
        assert_eq!(plan.before_fingerprint.len(), 4);
    }

    #[test]
    fn final_materialized_system_crossing_is_materialized_without_note_changes() {
        let source = xml(true);
        let plan = build_phrase_slur_edit_plan(&source, &[projection(1, 0, 2, 4)])
            .expect("plan").expect("crossing relation");
        assert_eq!(plan.expected_pairs, BTreeMap::from([(1, 1)]));
        let post = apply_synthetic_slur_plan(&source, &plan);
        assert_eq!(parse_render_note_events(&source).len(), parse_render_note_events(&post).len());
        assert_eq!(render_note_fingerprint(&parse_render_note_events(&post)), plan.before_fingerprint);
    }

    #[test]
    fn multi_pitch_endpoint_is_withheld_as_ambiguous_chord_anchor() {
        let source = xml(false);
        let mut intent = projection(1, 0, 1, 4);
        intent.start_pitches = vec![60, 64];
        let plan = build_phrase_slur_edit_plan(&source, &[intent]).expect("plan");
        assert!(plan.is_none());
    }

    #[test]
    fn existing_slur_ownership_is_withheld_without_merge() {
        let source = xml(false).replace(
            "<notations><articulations>",
            "<notations><slur type=\"start\" number=\"1\"/><articulations>",
        );
        let plan = build_phrase_slur_edit_plan(&source, &[projection(1, 0, 1, 4)])
            .expect("plan");
        assert!(plan.is_none());
    }
fn receipt_directives() -> Vec<HorizontalLayoutMaterializationDirective> {
    vec![HorizontalLayoutMaterializationDirective {
        explicit_width: Some("900".to_owned()),
        system_break: super::super::horizontal_layout_plan_materialization::HorizontalSystemBreakMaterialization::None,
        starts_new_page: false,
    }; 2]
}

#[test]
fn e007_pure_prepare_carries_own_pr_for_layout_and_real_approved_phrase() {
    let source = xml(false);
    let (prepared, receipt) = prepare_final_render_notation_with_phrase_projection(
        &source, &receipt_directives(), &[projection(1, 0, 1, 4)],
    );
    let prepared = prepared.expect("real pure preparation");
    assert!(prepared.phrase_write, "fixture must pass the existing repetition policy");
    assert_eq!(prepared.changed_slots, 2);
    assert_eq!(slur_pair_counts(&prepared.final_post), BTreeMap::from([(1, 1)]));
    let receipt = receipt.expect("real E007 preserving receipt");
    receipt.validate_preserving_transition::<FinalLayoutPhrasePreservingMint>(
        &source, &prepared.final_post,
    ).unwrap();
    assert!(receipt.validate_preserving_transition::<FinalLayoutPhrasePreservingMint>(
        &source.replace("<step>C</step>", "<step>E</step>"), &prepared.final_post,
    ).is_err());
    let mut chain = crate::master_state_identity::RenderTransitionReceipts::new();
    chain.push_preserving(Some(receipt), &prepared.final_post);
    chain.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
}

#[test]
fn original_prepared_and_file_phrase_insertions_carry_encoded_sources_and_noop() {
    use crate::arrangement::OwnedRhythmicContinuation;
    use crate::master_state_identity::OwnerReceipts;
    let state = OwnedRhythmicContinuation::synthetic_source_received_for_test(&[
        (1201, 0, 96, 1, 60), (1202, 96, 288, 1, 62),
        (1203, 384, 96, 1, 60), (1204, 480, 288, 1, 62),
    ], 2).original_final_chord_for_test();
    let intents = [projection(1, 0, 1, 96)];
    let (prepared, receipt) = prepare_final_render_notation_with_phrase_projection(
        state.document(), &receipt_directives(), &intents);
    let prepared = prepared.unwrap();
    assert!(prepared.phrase_write, "must exercise actual inner-note insertions");
    assert_eq!(slur_pair_counts(prepared.final_post()), BTreeMap::from([(1, 1)]));
    let mut owners = OwnerReceipts::default(); owners.push_preserving(receipt);
    let carried = owners.continue_owned_rhythm(&state, prepared.final_post()).unwrap();
    carried.require_complete_course_context(prepared.final_post()).unwrap();
    assert!(carried.same_identities_as(&state));
    assert!(owners.continue_owned_rhythm(&carried, prepared.final_post()).is_err());

    let path = std::env::temp_dir().join(format!("original-phrase-copies-{}.musicxml", std::process::id()));
    std::fs::write(&path, state.document()).unwrap();
    let (written, receipt) = materialize_final_render_notation_with_phrase_projection_authorized(
        &path, &receipt_directives(), &intents);
    assert_eq!(written.unwrap(), 2);
    let (receipt, actual) = receipt.unwrap().split_realized_post_document();
    let actual = actual.unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(actual, prepared.final_post());
    let mut owners = OwnerReceipts::default(); owners.push_preserving(Some(receipt));
    let carried = owners.continue_owned_rhythm(&state, &actual).unwrap();
    assert!(carried.same_identities_as(&state));
    let (noop, receipt) = prepare_final_render_notation_with_phrase_projection(
        carried.document(), &receipt_directives(), &intents);
    let noop = noop.unwrap();
    assert_eq!(noop.final_post(), carried.document()); assert!(receipt.is_none());
    let next = OwnerReceipts::default().continue_owned_rhythm(&carried, noop.final_post()).unwrap();
    assert!(next.same_identities_as(&state));
}

#[test]
fn e007_preparation_error_has_no_ticket_and_no_receipt() {
    let (prepared, receipt) = prepare_final_render_notation_with_phrase_projection(
        &xml(false), &[], &[projection(1, 0, 1, 4)],
    );
    assert!(prepared.is_err());
    assert!(receipt.is_none());
}

#[test]
fn e007_realized_post_equals_unchanged_legacy_e007_endbytes() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!("e007-oracle-{}-{}.musicxml",
        std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
    let source = xml(false);
    let dirs = receipt_directives();
    let intents = [projection(1, 0, 1, 4)];
    let (prepared, prepared_receipt) = prepare_final_render_notation_with_phrase_projection(
        &source, &dirs, &intents,
    );
    let prepared = prepared.unwrap();
    assert!(!path.exists(), "pure prepare must not write a file");
    std::fs::write(&path, &source).unwrap();
    let (legacy_result, legacy_receipt) =
        materialize_final_render_notation_with_phrase_projection_authorized(&path, &dirs, &intents);
    assert_eq!(legacy_result.unwrap(), prepared.changed_slots);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), prepared.final_post);
    for receipt in [prepared_receipt, legacy_receipt] {
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
        probe.push_preserving(receipt, &prepared.final_post);
        probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    }
    std::fs::remove_file(path).unwrap();
}

fn e007_state_fixture(label: &str) -> (
    super::super::RenderSovereignState,
    super::super::MasterAdoptedState,
    std::path::PathBuf,
) {
    use crate::master_state_identity::{MasterLineage, RenderMasterEpoch, RenderTransitionReceipts};
    use crate::master_state_identity::tests::synthetic_genesis_for_tests;
    use super::super::{MasterAdoptedState, RenderSovereignState, RenderSinkKind};
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    crate::master_state_identity::reset_render_lineage_measurement();
    let source = xml(false).replace("<divisions>4</divisions>",
        "<divisions>4</divisions><time><beats>2</beats><beat-type>4</beat-type></time>");
    let master = MasterAdoptedState::adopt_genesis_state(source.clone(),
        MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(&source)));
    let path = std::env::temp_dir().join(format!("r1230-e007-{label}-{}-{}.render.musicxml",
        std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
    let _ = std::fs::remove_file(&path);
    let mut state = RenderSovereignState::begin_project_render(&path, &master);
    let root = state.initial_trial_root(true);
    state.adopt_transition(root, &source, RenderTransitionReceipts::new(),
        RenderSinkKind::PrunedPdfTrial,
        |path, _, post| (std::fs::write(path, post), None)).unwrap();
    state.bind_late_master_epoch(RenderMasterEpoch::after_cursor_lane_before_final_chord(
        master.lineage(), master.document(),
        &crate::cursor_lane_temporal_check::check_cursor_lane_temporal_precondition("MasterXML", master.document()).expect("valid synthetic temporal precondition"), master.document()));
    (state, master, path)
}
fn e007_complete_stages() {
    use crate::master_state_identity::{record_render_stage_boundary, RenderStageBoundary as S};
    for stage in [S::InitialRender,S::RestArticulation,S::MetricBeam,
                  S::FinalNotation,S::PdfSourceSelection,S::Release] {
        record_render_stage_boundary(stage);
    }
}
#[test]
fn real_e007_after_last_58_commits_own_post_and_closes() {
    use crate::master_state_identity::RenderTransitionReceipts;
    let (mut state, _master, path) = e007_state_fixture("after58");
    let h = state.document().unwrap().to_owned();
    let root = state.require_state_root("test historical58");
    state.adopt_final_notation_checkpoint(root, &h, RenderTransitionReceipts::new()).unwrap();
    let changed = super::super::adopt_final_layout_phrase_projection(
        &mut state, &receipt_directives(), &[projection(1,0,1,4)]).unwrap();
    assert_eq!(changed,2);
    let c = state.document().unwrap().to_owned();
    assert_ne!(c,h);
    assert_eq!(state.last_checkpoint_58_recorded_document(),Some(h.as_str()));
    assert!(state.last_checkpoint_58_basis().is_none());
    assert!(state.historical_checkpoint_58_root().is_none());
    assert_eq!(std::fs::read_to_string(&path).unwrap(),c);
    assert_eq!(slur_pair_counts(&c),BTreeMap::from([(1,1)]));
    e007_complete_stages();
    assert!(state.close_render_sovereignty().closed);
    std::fs::remove_file(path).unwrap();
    crate::master_state_identity::reset_render_lineage_measurement();
}
#[test]
fn real_e007_preparation_error_keeps_source_and_is_closure_fatal() {
    let (mut state, _master, path) = e007_state_fixture("prepare-error");
    let a = state.document().unwrap().to_owned();
    assert!(super::super::adopt_final_layout_phrase_projection(
        &mut state, &[], &[projection(1,0,1,4)]).is_err());
    assert_eq!(state.document(),Some(a.as_str()));
    assert_eq!(std::fs::read_to_string(&path).unwrap(),a);
    e007_complete_stages();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        || state.close_render_sovereignty())).is_err());
    std::fs::remove_file(path).unwrap();
    crate::master_state_identity::reset_render_lineage_measurement();
}
#[test]
fn real_e007_stale_artifact_stops_before_preparation_and_write() {
    let (mut state, _master, path) = e007_state_fixture("stale-file");
    let a = state.document().unwrap().to_owned();
    std::fs::write(&path,"stale foreign bytes").unwrap();
    let attempted = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        super::super::adopt_final_layout_phrase_projection(
            &mut state,&receipt_directives(),&[projection(1,0,1,4)])
    }));
    assert!(attempted.is_err());
    assert_eq!(state.document(),Some(a.as_str()));
    assert_eq!(std::fs::read_to_string(&path).unwrap(),"stale foreign bytes");
    e007_complete_stages();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        || state.close_render_sovereignty())).is_err());
    std::fs::remove_file(path).unwrap();
    crate::master_state_identity::reset_render_lineage_measurement();
}


    #[test]
    fn real_e007_transport_failures_never_commit_intermediate_layout() {
        for phase in [1, 2, 3, 4] {
            let (mut state, _master, path) = e007_state_fixture(&format!("fault-{phase}"));
            let a = state.document().unwrap().to_owned();
            super::super::master_adoption::set_final_layout_transport_fault(phase);
            let result = super::super::adopt_final_layout_phrase_projection(
                &mut state, &receipt_directives(), &[projection(1, 0, 1, 4)],
            );
            super::super::master_adoption::set_final_layout_transport_fault(0);
            assert!(result.is_err(), "typed physical phase {phase} must fail");
            assert_eq!(state.document(), Some(a.as_str()));
            e007_complete_stages();
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || state.close_render_sovereignty())).is_err());
            std::fs::remove_file(path).unwrap();
            crate::master_state_identity::reset_render_lineage_measurement();
        }
    }
    #[test]
    fn real_e007_layout_only_then_byte_equal_sink_does_not_add_owner_event() {
        let (mut state, _master, path) = e007_state_fixture("layout-only-noop");
        super::super::adopt_final_layout_phrase_projection(
            &mut state, &receipt_directives(), &[],
        ).unwrap();
        let b = state.document().unwrap().to_owned();
        super::super::adopt_final_layout_phrase_projection(
            &mut state, &receipt_directives(), &[],
        ).unwrap();
        assert_eq!(state.document(), Some(b.as_str()));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), b);
        e007_complete_stages();
        assert!(state.close_render_sovereignty().closed);
        std::fs::remove_file(path).unwrap();
        crate::master_state_identity::reset_render_lineage_measurement();
    }


    fn apply_synthetic_slur_plan(source: &str, plan: &PhraseSlurEditPlan) -> String {
        let mut post = source.to_owned();
        for insertion in plan.insertions.iter().rev() {
            post.insert_str(insertion.position, &insertion.text);
        }
        assert!(Document::parse(&post).is_ok());
        assert_eq!(render_note_fingerprint(&parse_render_note_events(&post)), plan.before_fingerprint);
        assert_eq!(slur_pair_counts(&post), plan.expected_pairs);
        post
    }

    fn synthetic_three_sections() -> String {
        let mut source = xml(true);
        let start = source.rfind("<measure ").unwrap();
        let end = start + source[start..].find("</measure>").unwrap() + "</measure>".len();
        let mut last = source[start..end].to_owned();
        let open_end = last.find('>').unwrap();
        last.replace_range(..=open_end, "<measure number=\"return\">");
        source.insert_str(end, &last);
        source
    }

    #[test]
    fn page_and_multiple_system_boundaries_preserve_the_complete_phrase_relation() {
        for source in [synthetic_three_sections(), synthetic_three_sections().replace("new-system=\"yes\"", "new-page='yes'")] {
            let plan = build_phrase_slur_edit_plan(&source, &[projection(1, 0, 3, 4)]).unwrap().unwrap();
            assert_eq!(plan.expected_pairs, BTreeMap::from([(1, 1)]));
            apply_synthetic_slur_plan(&source, &plan);
        }
    }

    #[test]
    fn crossing_reserves_future_legacy_numbers_without_renumbering_or_losing_pairs() {
        let source = xml(true);
        let retained = projection(2, 0, 2, 4);
        let legacy = build_phrase_slur_edit_plan(&source, std::slice::from_ref(&retained)).unwrap().unwrap();
        let legacy_post = apply_synthetic_slur_plan(&source, &legacy);
        let plan = build_phrase_slur_edit_plan(&source, &[projection(1, 0, 2, 4), retained]).unwrap().unwrap();
        assert_eq!(plan.expected_pairs, BTreeMap::from([(1, 1), (2, 1)]));
        let mut post = apply_synthetic_slur_plan(&source, &plan);
        for kind in ["start", "stop"] {
            post = post.replace(&format!("          <slur type=\"{kind}\" number=\"2\"/>\n"), "");
        }
        post = post.replace("      <notations>\n      </notations>\n", "");
        assert_eq!(post, legacy_post, "only the additional relation and its required empty container may differ");
    }

    #[test]
    fn exhausted_shared_number_capacity_withholds_only_the_new_crossing() {
        let source = xml(true);
        let mut intents = vec![projection(2, 0, 2, 4); super::super::PHRASE_SLUR_NUMBER_LEVEL_MAXIMUM];
        let legacy = build_phrase_slur_edit_plan(&source, &intents).unwrap().unwrap();
        intents.push(projection(1, 0, 2, 4));
        let plan = build_phrase_slur_edit_plan(&source, &intents).unwrap().unwrap();
        assert_eq!(plan.expected_pairs, legacy.expected_pairs);
        assert_eq!(apply_synthetic_slur_plan(&source, &plan), apply_synthetic_slur_plan(&source, &legacy));
    }

    #[test]
    fn crossing_keeps_existing_slur_chord_and_missing_anchor_refusals() {
        let source = xml(true);
        let original = projection(1, 0, 2, 4);
        let mut missing = original.clone(); missing.start_pitches = vec![65];
        let mut start_chord = original.clone(); start_chord.start_pitches.push(64);
        let mut end_chord = original.clone(); end_chord.end_pitches.push(67);
        for intent in [missing, start_chord, end_chord] {
            assert!(build_phrase_slur_edit_plan(&source, &[intent]).unwrap().is_none());
        }
        let occupied = source.replace("<notations><articulations>", "<notations><slur type=\"start\" number=\"1\"/><articulations>");
        assert!(build_phrase_slur_edit_plan(&occupied, std::slice::from_ref(&original)).unwrap().is_none());
        let duplicate = "<note><chord/><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>";
        let ambiguous = source.replacen("</note>", &format!("</note>{duplicate}"), 1);
        assert!(build_phrase_slur_edit_plan(&ambiguous, &[original]).unwrap().is_none());
    }

    #[test]
    fn crossing_decisions_survive_renaming_relocation_and_irrelevant_metadata() {
        let source = xml(true); let original = projection(1, 0, 2, 4);
        let original_plan = build_phrase_slur_edit_plan(&source, std::slice::from_ref(&original)).unwrap().unwrap();
        let renamed = source.replace("P1", "IndependentPart").replace("<staff>1</staff>", "<staff>2</staff>").replace("<voice>1</voice>", "<voice>7</voice>");
        let mut intent = original.clone(); intent.part = "IndependentPart".to_owned(); intent.staff = "2".to_owned(); intent.voice = "7".to_owned();
        let plan = build_phrase_slur_edit_plan(&renamed, &[intent]).unwrap().unwrap();
        assert_eq!(plan.expected_pairs, original_plan.expected_pairs); apply_synthetic_slur_plan(&renamed, &plan);
        let prefix = "<measure number=\"preface\"><note><rest/><duration>4</duration><voice>1</voice><staff>1</staff></note></measure>";
        let mut relocated = source.clone(); relocated.insert_str(part_ranges(&source)[0].content_start, prefix);
        let plan = build_phrase_slur_edit_plan(&relocated, &[projection(2, 0, 3, 4)]).unwrap().unwrap();
        assert_eq!(plan.expected_pairs, original_plan.expected_pairs); apply_synthetic_slur_plan(&relocated, &plan);
        let metadata = source.replace("<part-list>", "<work><work-title>Unrelated title</work-title></work><part-list>");
        let plan = build_phrase_slur_edit_plan(&metadata, &[original]).unwrap().unwrap();
        assert_eq!(plan.expected_pairs, original_plan.expected_pairs); apply_synthetic_slur_plan(&metadata, &plan);
    }

    #[test]
    fn independent_crossing_duplication_keeps_each_original_local_decision() {
        let source = xml(true); let first = projection(1, 0, 2, 4);
        let original = build_phrase_slur_edit_plan(&source, std::slice::from_ref(&first)).unwrap().unwrap();
        let original_post = apply_synthetic_slur_plan(&source, &original);
        let start = source.find("<part id=").unwrap(); let end = source[start..].find("</part>").unwrap()+start+"</part>".len();
        let additional_part = source[start..end].replace("P1", "IndependentPart");
        let mut duplicated = source.clone(); duplicated.insert_str(end, &additional_part);
        duplicated = duplicated.replace("</part-list>", "<score-part id=\"IndependentPart\"><part-name>Independent</part-name></score-part></part-list>");
        let mut second = first.clone(); second.part = "IndependentPart".to_owned();
        let plan = build_phrase_slur_edit_plan(&duplicated, &[first, second]).unwrap().unwrap();
        assert_eq!(plan.expected_pairs, BTreeMap::from([(1, 2)]));
        let post = apply_synthetic_slur_plan(&duplicated, &plan);
        assert!(post.contains(&original_post[start..original_post.find("</part>").unwrap()+"</part>".len()]));
        assert!(post.contains("id=\"IndependentPart\""));
    }

    #[test]
    fn real_e007_owner_carries_crossing_phrase_in_original_receipt_and_exact_file_bytes() {
        let source = synthetic_three_sections();
        let mut intent = projection(1, 4, 2, 0); intent.start_pitches = vec![62]; intent.end_pitches = vec![60];
        let mut directives = vec![HorizontalLayoutMaterializationDirective {
            explicit_width: None,
            system_break: super::super::horizontal_layout_plan_materialization::HorizontalSystemBreakMaterialization::None,
            starts_new_page: false,
        }; 3];
        directives[1].system_break = super::super::horizontal_layout_plan_materialization::HorizontalSystemBreakMaterialization::Enabled;
        let (prepared, receipt) = prepare_final_render_notation_with_phrase_projection(&source, &directives, std::slice::from_ref(&intent));
        let prepared = prepared.unwrap(); assert!(prepared.phrase_write);
        assert_eq!(slur_pair_counts(prepared.final_post()), BTreeMap::from([(1, 1)]));
        receipt.unwrap().validate_preserving_transition::<FinalLayoutPhrasePreservingMint>(&source, prepared.final_post()).unwrap();
        assert_eq!(render_note_fingerprint(&parse_render_note_events(&source)), render_note_fingerprint(&parse_render_note_events(prepared.final_post())));
        let path = std::env::temp_dir().join(format!("synthetic-crossing-owner-{}.render.musicxml",std::process::id()));
        std::fs::write(&path,&source).unwrap();
        let (written, receipt) = materialize_final_render_notation_with_phrase_projection_authorized(&path,&directives,&[intent]);
        assert!(written.is_ok());
        let (receipt, realized) = receipt.unwrap().split_realized_post_document();
        assert_eq!(realized.as_deref(),Some(prepared.final_post()));
        assert_eq!(std::fs::read_to_string(&path).unwrap(),prepared.final_post());
        receipt.validate_preserving_transition::<FinalLayoutPhrasePreservingMint>(&source,prepared.final_post()).unwrap();
        std::fs::remove_file(path).unwrap();
    }

}


#[cfg(test)]
mod phrase_occurrence_course_tests {
    use super::*;

    #[test]
    fn real_original_owner_voice_transport_keeps_approved_phrase_endpoints() {
        use super::super::serialization_plan::*;
        for measure_origin in [17usize, 83] {
            let mut input = crate::pipeline::actual_beam_owner_capture_tests::input()
                .replace("probe", &measure_origin.to_string());
            let document = Document::parse(&input).unwrap();
            let first = document.descendants().find(|node| node.has_tag_name("measure")).unwrap();
            let second = input[first.range()].replace(
                &format!("number=\"{measure_origin}\""),
                &format!("number=\"{}\"", measure_origin + 1));
            input = input.replace("</part>", &(second + "</part>"));
            let (canonical, _voices, source, prepared) =
                CanonicalFinalChordStructuralPlan::original_beam_alignment_test_fixture(&input);
            let notes = parse_render_note_events(source.xml());
            let first = notes.iter().find(|note| note.fingerprint.pitch == Some(72)).unwrap();
            let last = notes.iter().find(|note| note.fingerprint.pitch == Some(76)).unwrap();
            assert_eq!(first.fingerprint.voice, last.fingerprint.voice);
            let intent = PhraseNotationIntentProjection {
                part: first.fingerprint.part.clone(), staff: first.fingerprint.staff.clone(),
                voice: first.fingerprint.voice.clone(), start_measure_ordinal: 1,
                start_onset: first.fingerprint.onset, start_canonical: i128::from(first.fingerprint.onset),
                start_pitches: vec![72], end_measure_ordinal: 1,
                end_onset: last.fingerprint.onset, end_canonical: i128::from(last.fingerprint.onset),
                end_pitches: vec![76], attack_count: 2,
            };
            let approved = approve_phrase_occurrence_course(&source, std::slice::from_ref(&intent)).unwrap();
            let (_, _, independently_projected, _) =
                CanonicalFinalChordStructuralPlan::original_beam_alignment_test_fixture(&input);
            assert_eq!(source.xml(), independently_projected.xml());
            assert!(approved.carry(&independently_projected).is_err(),
                "equal XML and numeric IDs from another original execution cannot carry endpoints");
            assert_eq!(approved.endpoints.len(), 1, "actual unchanged policy must approve the repeated lane");
            let rebound = rebind_primary_beam_voice_partition_plan_to_current_xml(source.xml(), &source, &prepared).unwrap();
            let edits = rebound.edits().iter().map(|edit|
                (edit.voice_text_start(), edit.voice_text_end(), edit.target_label().to_owned())).collect();
            let (post, receipt) = super::super::apply_atomic_staff_voice_scalar_batch_authorized(source.xml(), edits);
            let post = post.unwrap();
            assert_eq!(receipt.unwrap().realize_post_document(source.xml()).as_deref(), Some(post.as_str()));
            let carried = transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
                &source, source.xml(), &post, &canonical, &rebound, Some(&BTreeSet::new())).unwrap();
            assert!(super::super::phrase_notation_policy::approve_pending_visual_intents_from_musicxml(
                &post, std::slice::from_ref(&intent)).unwrap().is_empty(), "technical merging must exercise the old decision loss");
            let bound = approved.carry(&carried).unwrap();
            let directives = vec![HorizontalLayoutMaterializationDirective { explicit_width: Some("900".into()), system_break: super::super::horizontal_layout_plan_materialization::HorizontalSystemBreakMaterialization::None, starts_new_page: false }; 2];
            let (prepared, receipt) = prepare_final_render_notation_with_phrase_course(&post, &directives, &[], Some(&bound));
            let prepared = prepared.unwrap();
            assert!(prepared.phrase_write);
            assert_eq!(slur_pair_counts(prepared.final_post()), BTreeMap::from([(1, 1)]));
            assert_eq!(render_note_fingerprint(&parse_render_note_events(prepared.final_post())),
                render_note_fingerprint(&parse_render_note_events(&post)));
            let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
            receipts.push_preserving(receipt, prepared.final_post());
            receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            let wrong = post.replace("Piano", "Another source");
            let (rejected, receipt) = prepare_final_render_notation_with_phrase_course(&wrong, &directives, &[], Some(&bound));
            assert!(rejected.is_err()); assert!(receipt.is_none());
        }
    }
}
