//! Observation of actual canonical beam decisions through an exact current
//! producer/source seam. No selector, allocator, beam inference or write permit.
use super::*;
#[derive(Debug, Clone)]
pub(super) struct BoundOccurrence {
    pub(super) source: usize,
    pub(super) attack: usize,
    pub(super) fragment: usize,
    pub(super) levels: crate::pipeline::ActualBeamLevels,
}
#[derive(Debug, Clone)]
struct BoundChordBeamAttachment {
    member: (usize, usize, usize),
    head: Option<(usize, usize, usize)>,
    head_levels: crate::pipeline::ActualBeamLevels,
    unknown: Option<String>,
}
#[derive(Debug, Clone)]
pub(crate) struct ActualSourceBeamBindingObservation {
    bound: Result<Vec<BoundOccurrence>, String>,
    chord_attachments: Result<Vec<BoundChordBeamAttachment>, String>,
    original: Option<std::sync::Arc<str>>,
    locators: std::sync::Arc<[FinalChordCanonicalSourceLocator]>,
    note_shapes: std::collections::BTreeMap<usize, (String, usize, bool, bool)>,
}
impl Default for ActualSourceBeamBindingObservation {
    fn default() -> Self {
        Self {
            bound: Err("No actual beam owner capture received".into()),
            chord_attachments: Err("No original chord attachment capture received".into()),
            original: None,
            locators: std::sync::Arc::from([]),
            note_shapes: Default::default(),
        }
    }
}
// A chord component carries only a relation to its original beam container.
// Its head's states are never promoted to independently owned member states.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum SharingBeamKind { OwnedDecision, ChordContainer, NonBeamable }
type SharingBeamLevel = (u8, Option<(usize, usize)>, Option<String>);
#[derive(Clone, Copy)]
pub(super) struct SharingBeamWitness<'a> {
    pub(super) source: usize,
    pub(super) attack: usize,
    pub(super) levels: &'a [SharingBeamLevel],
    pub(super) kind: SharingBeamKind,
}
impl ActualSourceBeamBindingObservation {
    pub(super) fn sharing_origin_matches(&self, canonical: &CanonicalFinalChordStructuralPlan) -> bool {
        self.bound.is_ok() && canonical.written_projection.as_ref().is_some_and(|written|
            self.original.as_deref()==Some(written.original_xml.as_ref())
                && self.locators.as_ref()==canonical.source_locators.as_ref())
    }

    pub(super) fn sharing_witness(
        &self, canonical: &CanonicalFinalChordStructuralPlan, fragment: usize,
    ) -> Option<SharingBeamWitness<'_>> {
        self.sharing_witness_diagnostic(canonical, fragment).ok()
    }

    pub(super) fn sharing_witness_diagnostic(
        &self, canonical: &CanonicalFinalChordStructuralPlan, fragment: usize,
    ) -> Result<SharingBeamWitness<'_>, &'static str> {
        let original = canonical.written_projection.as_ref().ok_or("missing_canonical_written_projection")?.original_xml.as_ref();
        if self.original.as_deref().ok_or("missing_original_capture_document")? != original || self.locators.as_ref() != canonical.source_locators.as_ref() {
            return Err("original_document_or_locators_changed");
        }
        let owned = self.bound.as_ref().map_err(|_|"missing_original_owned_capture")?;
        if let Some(row) = owned.iter().find(|row| row.fragment == fragment) {
            return Ok(SharingBeamWitness {source:row.source,attack:row.attack,
                levels:&row.levels,kind:SharingBeamKind::OwnedDecision});
        }
        // Applicability is not an absent beam decision. The exact canonical
        // producer supplies an unchanged non-beamable note type with no beam tag.
        let locator=canonical.source_locators.iter().find(|l| l.fragment_id==fragment).ok_or("missing_canonical_fragment")?;
        if self.note_shapes.get(&fragment).is_some_and(|shape|
            shape.3 && matches!(shape.0.as_str(),"quarter"|"half"|"whole"|"breve"|"long"|"maxima")) {
            return Ok(SharingBeamWitness {source:locator.source_event_id,attack:locator.attack_id,
                levels:&[],kind:SharingBeamKind::NonBeamable});
        }
        let attached = self.chord_attachments.as_ref().map_err(|_|"missing_original_attachment_capture")?.iter()
            .find(|row| row.member.2 == fragment && row.unknown.is_none()).ok_or("missing_or_unknown_original_chord_attachment")?;
        let head_id = attached.head.ok_or("missing_original_chord_head")?;
        let member = canonical.source_locators.iter().find(|l|
            (l.source_event_id,l.attack_id,l.fragment_id)==attached.member).ok_or("unbound_original_chord_member")?;
        let head = canonical.source_locators.iter().find(|l|
            (l.source_event_id,l.attack_id,l.fragment_id)==head_id).ok_or("unbound_original_chord_head")?;
        if !member.chord_syntax || member.measure != head.measure || member.onset != head.onset
            || member.duration <= 0 || member.duration != head.duration
            || member.tie_start != head.tie_start || member.tie_stop != head.tie_stop {
            return Err("container_onset_duration_or_ties_changed");
        }
        let action = canonical.actions.iter().find(|a| a.fragment_id==fragment).ok_or("missing_canonical_chord_action")?;
        if action.host_source_event_id != Some(head.source_event_id) {
            return Err("original_chord_host_changed");
        }
        let unchanged_container = match (&action.representation_role, &action.kind) {
            (FinalChordRepresentationRole::IndependentAuxiliaryFragment,
                FinalChordCanonicalActionKind::AuxiliaryLane { canonical_duration, .. }) =>
                member.staff != head.staff && *canonical_duration == member.duration,
            (FinalChordRepresentationRole::RetainedChordMember,
                FinalChordCanonicalActionKind::RetainedChordMember { canonical_duration }) =>
                member.staff == head.staff && *canonical_duration == member.duration
                && canonical.hard_same_edges.iter().any(|edge|
                    ((edge.left_fragment_id == member.fragment_id && edge.right_fragment_id == head.fragment_id)
                        || (edge.right_fragment_id == member.fragment_id && edge.left_fragment_id == head.fragment_id))
                    && member.staff.parse::<u8>().ok() == Some(edge.presentation_staff)),
            _ => false,
        };
        if !unchanged_container {
            return Err("container_representation_or_hard_same_not_preserved");
        }
        let member_shape=self.note_shapes.get(&fragment).ok_or("missing_original_member_note_shape")?;
        let head_shape=self.note_shapes.get(&head.fragment_id).ok_or("missing_original_head_note_shape")?;
        if (!member_shape.2 && action.representation_role != FinalChordRepresentationRole::RetainedChordMember)
            || member_shape.0 != head_shape.0 || member_shape.1 != head_shape.1 {
            return Err("container_type_dots_or_spacing_changed");
        }
        Ok(SharingBeamWitness {source:member.source_event_id,attack:member.attack_id,
            levels:&attached.head_levels,kind:SharingBeamKind::ChordContainer})
    }

    pub(crate) fn append_report_body(&self, out: &mut String) {
        let _ = writeln!(
            out,
            "Authority: observation_only; ChannelAllocationPermit none; MusicMutation none; InferredAbsentDecisions none."
        );
        match &self.bound {
            Err(reason) => {
                let _ = writeln!(
                    out,
                    "ActualCanonicalBeamSourceBinding: Unknown; Reason {reason}"
                );
            }
            Ok(rows) => {
                let groups = rows
                    .iter()
                    .flat_map(|r| {
                        r.levels
                            .iter()
                            .filter_map(|(level, group, _)| group.map(|group| (*level, group)))
                    })
                    .collect::<std::collections::BTreeSet<_>>();
                let negative = rows
                    .iter()
                    .flat_map(|r| &r.levels)
                    .filter(|(_, g, _)| g.is_none())
                    .count();
                let _ = writeln!(
                    out,
                    "ActualCanonicalBeamSourceBinding: captured_scope_bound; OriginalOccurrences {}; PositiveGroups {}; ExplicitNegativeLevelDecisions {}; CompleteScoreCoverage not_claimed; MusicalCompatibility not_claimed.",
                    rows.len(),
                    groups.len(),
                    negative
                );
                for row in rows {
                    let _ = writeln!(
                        out,
                        "ActualCanonicalBeamSourceBindingRow: SourceEventId {} SoundingAttackId {} FragmentId {} Levels {:?}",
                        row.source, row.attack, row.fragment, row.levels
                    );
                }
            }
        }
        match &self.chord_attachments {
            Err(reason) => {
                let _ = writeln!(
                    out,
                    "ActualCanonicalChordBeamAttachments: Unknown; Reason {reason}; IndependentBeamDecisionPermit none."
                );
            }
            Ok(rows) => {
                let known = rows.iter().filter(|row| row.head.is_some()).count();
                let _ = writeln!(
                    out,
                    "ActualCanonicalChordBeamAttachments: SourceBound {}; OriginalHeadBound {}; UnknownHead {}; Authority original_physical_chord_attachment_only; IndependentBeamDecisionPermit none.",
                    rows.len(),
                    known,
                    rows.len() - known
                );
                for row in rows {
                    let _ = writeln!(
                        out,
                        "ActualCanonicalChordBeamAttachmentRow: SourceEventId {} SoundingAttackId {} FragmentId {} OriginalHead {:?} OriginalHeadLevels {:?} Unknown {:?}; IndependentBeamDecisionPermit none.",
                        row.member.0,
                        row.member.1,
                        row.member.2,
                        row.head,
                        row.head_levels,
                        row.unknown
                    );
                }
            }
        }
    }
}
pub(crate) fn observe_actual_source_beam_binding(
    canonical: &CanonicalFinalChordStructuralPlan,
    source_xml: &str,
    capture: Option<&Result<std::sync::Arc<crate::pipeline::ActualCanonicalBeamCapture>, String>>,
) -> ActualSourceBeamBindingObservation {
    let result = (|| -> anyhow::Result<Vec<BoundOccurrence>> {
        anyhow::ensure!(
            canonical.voice_free_evidence_sufficiency_green(),
            "Unbound canonical source plan"
        );
        anyhow::ensure!(
            canonical
                .written_projection
                .as_ref()
                .is_some_and(|evidence| evidence.original_xml.as_ref() == source_xml),
            "Canonical source document is not the original producer seam"
        );
        let capture = capture
            .ok_or_else(|| anyhow::anyhow!("Missing actual beam owner capture"))?
            .as_ref()
            .map_err(|e| anyhow::anyhow!("Actual beam owner capture: {e}"))?;
        let occurrences = capture.occurrences(source_xml)?;
        let measures = final_chord_parse_binding_measures(source_xml)?;
        let notes = measures
            .iter()
            .flat_map(|m| &m.notes)
            .filter(|n| !n.rest_like && n.pitch_midi.is_some())
            .map(|n| ((n.measure, n.raw_note_index), n))
            .collect::<std::collections::BTreeMap<_, _>>();
        let mut by_range = std::collections::BTreeMap::new();
        for locator in canonical.source_locators.iter() {
            let note = notes
                .get(&(locator.measure, locator.raw_note_index))
                .copied()
                .ok_or_else(|| anyhow::anyhow!("Missing current canonical original occurrence"))?;
            anyhow::ensure!(
                note.onset == locator.onset
                    && note.duration == locator.duration
                    && note.staff == locator.staff
                    && note.pitch_identity == locator.pitch_identity,
                "Canonical producer source binding drift"
            );
            anyhow::ensure!(
                by_range
                    .insert((note.range.start, note.range.end), locator)
                    .is_none(),
                "Ambiguous original canonical occurrence"
            );
        }
        anyhow::ensure!(
            by_range.len() == notes.len(),
            "Incomplete current canonical source coverage"
        );
        let mut claimed = std::collections::BTreeSet::new();
        occurrences
            .iter()
            .map(|(range, levels)| {
                let locator = by_range
                    .get(&(range.start, range.end))
                    .copied()
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "Actual beam member has no exact canonical original occurrence"
                        )
                    })?;
                anyhow::ensure!(
                    claimed.insert(locator.fragment_id),
                    "Duplicate canonical beam fragment binding"
                );
                Ok(BoundOccurrence {
                    source: locator.source_event_id,
                    attack: locator.attack_id,
                    fragment: locator.fragment_id,
                    levels: levels.clone(),
                })
            })
            .collect()
    })();
    let attachments = if result.is_ok() {
        bind_original_chord_beam_attachments(canonical, source_xml, capture)
            .map_err(|e| e.to_string())
    } else {
        Err("Canonical source binding is Unknown".into())
    };
    ActualSourceBeamBindingObservation {
        bound: result.map_err(|e| e.to_string()),
        chord_attachments: attachments,
        original: Some(std::sync::Arc::from(source_xml)),
        locators: canonical.source_locators.clone(),
        note_shapes: original_binding_note_shapes(canonical, source_xml),
    }
}

fn original_binding_note_shapes(
    canonical: &CanonicalFinalChordStructuralPlan, original: &str,
) -> std::collections::BTreeMap<usize, (String, usize, bool, bool)> {
    let Ok(measures)=final_chord_parse_binding_measures(original) else {return Default::default()};
    let locators=canonical.source_locators.iter().map(|l|((l.measure,l.raw_note_index),l.fragment_id))
        .collect::<std::collections::BTreeMap<_,_>>();
    measures.iter().flat_map(|m|m.notes.iter().map(move|n|(m.measure,n))).filter_map(|(measure,note)| {
        let fragment=*locators.get(&(measure,note.raw_note_index))?;
        let document=roxmltree::Document::parse(original.get(note.range.clone())?).ok()?;
        let node=document.root_element();
        let note_type=node.children().find(|n|n.has_tag_name("type"))?.text()?.to_string();
        Some((fragment,(note_type,node.children().filter(|n|n.has_tag_name("dot")).count(),
            node.attribute("print-spacing")!=Some("no"),
            !node.children().any(|n|n.has_tag_name("beam")))))
    }).collect()
}

fn bind_original_chord_beam_attachments(
    canonical: &CanonicalFinalChordStructuralPlan,
    source_xml: &str,
    capture: Option<&Result<std::sync::Arc<crate::pipeline::ActualCanonicalBeamCapture>, String>>,
) -> anyhow::Result<Vec<BoundChordBeamAttachment>> {
    let capture = capture
        .ok_or_else(|| anyhow::anyhow!("Missing original chord capture"))?
        .as_ref()
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let occurrences = capture.occurrences(source_xml)?;
    let measures = final_chord_parse_binding_measures(source_xml)?;
    let locators = canonical
        .source_locators
        .iter()
        .map(|row| ((row.measure, row.raw_note_index), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut by_range = std::collections::BTreeMap::new();
    for measure in &measures {
        for note in &measure.notes {
            if let Some(locator) = locators.get(&(note.measure, note.raw_note_index)) {
                anyhow::ensure!(
                    by_range
                        .insert((note.range.start, note.range.end), *locator)
                        .is_none(),
                    "Duplicate physical chord source"
                );
            }
        }
    }
    let identity = |range: &std::ops::Range<usize>| -> anyhow::Result<(usize, usize, usize)> {
        let row = by_range.get(&(range.start, range.end)).ok_or_else(|| {
            anyhow::anyhow!("Original chord attachment has no exact canonical source occurrence")
        })?;
        Ok((row.source_event_id, row.attack_id, row.fragment_id))
    };
    capture
        .chord_attachments(source_xml)?
        .iter()
        .map(|row| {
            let (head, head_levels) = match row.head {
                Some(index) => {
                    let (range, levels) = occurrences
                        .get(index)
                        .ok_or_else(|| anyhow::anyhow!("Foreign original chord head index"))?;
                    (Some(identity(range)?), levels.clone())
                }
                None => (None, Vec::new()),
            };
            Ok(BoundChordBeamAttachment {
                member: identity(&row.member)?,
                head,
                head_levels,
                unknown: row.unknown.clone(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn captured() -> (
        String,
        std::sync::Arc<crate::pipeline::ActualCanonicalBeamCapture>,
    ) {
        let scope = 83usize.to_string();
        let input =
            crate::pipeline::actual_beam_owner_capture_tests::input().replace("probe", &scope);
        let (after, _, capture) = crate::pipeline::actual_beam_owner_capture_tests::execute(&input);
        (after, capture)
    }

    #[test]
    fn actual_owner_scope_binds_to_current_canonical_source_identities() {
        let (xml, capture) = captured();
        let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical, _) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
        let observation = observe_actual_source_beam_binding(&canonical, &xml, Some(&Ok(capture)));
        let rows = observation.bound.unwrap();
        assert_eq!(rows.len(), 4);
        let expected = evidence
            .iter()
            .map(|r| (r.source_event_id, r.attack_id, r.fragment_id))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            rows.iter()
                .map(|r| (r.source, r.attack, r.fragment))
                .collect::<std::collections::BTreeSet<_>>(),
            expected
        );
    }

    #[test]
    fn equal_note_geometry_does_not_bind_a_different_producer_document() {
        let (xml, capture) = captured();
        let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (mut canonical, _) =
            plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
        canonical.written_projection.as_mut().unwrap().original_xml =
            std::sync::Arc::from(xml.replace("version=\"4.0\"", "version=\"3.1\""));
        assert!(
            observe_actual_source_beam_binding(&canonical, &xml, Some(&Ok(capture)))
                .bound
                .is_err()
        );
    }

    #[test]
    fn opaque_source_bijection_changes_only_carried_identifiers() {
        let (xml, capture) = captured();
        let mut evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
        for row in &mut evidence {
            row.source_event_id += 131;
            row.attack_id += 233;
            row.fragment_id += 379;
        }
        let (canonical, _) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
        let rows = observe_actual_source_beam_binding(&canonical, &xml, Some(&Ok(capture)))
            .bound
            .unwrap();
        assert_eq!(rows.len(), 4);
        assert!(
            rows.iter()
                .all(|r| r.source >= 131 && r.attack >= 233 && r.fragment >= 379)
        );
    }
    #[test]
    fn an_original_chord_attachment_binds_both_existing_canonical_identities() {
        let before = crate::pipeline::actual_beam_owner_capture_tests::input_with_original_chord()
            .replace("probe", &83usize.to_string());
        let (xml, _, capture) = crate::pipeline::actual_beam_owner_capture_tests::execute(&before);
        let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical, _) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
        let observation = observe_actual_source_beam_binding(&canonical, &xml, Some(&Ok(capture)));
        assert_eq!(observation.bound.as_ref().unwrap().len(), 4);
        let rows = observation.chord_attachments.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        assert_ne!(rows[0].member, rows[0].head.unwrap());
        assert!(rows[0].unknown.is_none());
        assert_eq!(rows[0].head_levels.len(), 4);
        let mut report = String::new();
        observation.append_report_body(&mut report);
        let mut ledger =
            crate::final_report::DraftReport::new(String::new()).compact(ToOwned::to_owned);
        ledger
            .project_owned_section(
                "actual_canonical_beam_source_binding",
                "ACTUAL-CANONICAL-BEAM-SOURCE-BINDING",
                "------------------------------------",
                |out| out.push_str(&report),
            )
            .unwrap();
        let text = ledger.render_snapshot();
        assert!(text.contains("SourceBound 1; OriginalHeadBound 1; UnknownHead 0"));
        assert!(text.contains("IndependentBeamDecisionPermit none"));
    }
    fn split_fixture(duration: i64) -> (CanonicalFinalChordStructuralPlan, ActualSourceBeamBindingObservation, usize) {
        split_fixture_with_type(duration,"16th")
    }
    fn split_fixture_with_type(duration: i64, member_type: &str) -> (CanonicalFinalChordStructuralPlan, ActualSourceBeamBindingObservation, usize) {
        let before=crate::pipeline::actual_beam_owner_capture_tests::input_with_original_chord()
            .replace("probe",&83usize.to_string())
            .replace("<chord/><pitch><step>A</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff>",
                &format!("<chord/><pitch><step>A</step><octave>5</octave></pitch><duration>{duration}</duration><voice>3</voice><type>{member_type}</type><staff>2</staff>"));
        let (xml,_,capture)=crate::pipeline::actual_beam_owner_capture_tests::execute(&before);
        let evidence=final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical,_)=plan_canonical_final_chord_structure_once(&xml,&evidence).unwrap();
        let observation=observe_actual_source_beam_binding(&canonical,&xml,Some(&Ok(capture)));
        let fragment=observation.chord_attachments.as_ref().unwrap()[0].member.2;
        (canonical,observation,fragment)
    }
    #[test]
    fn unchanged_staff_split_has_only_a_container_membership_witness() {
        let (canonical,observation,fragment)=split_fixture(24);
        let witness=observation.sharing_witness(&canonical,fragment).unwrap();
        assert!(witness.kind==SharingBeamKind::ChordContainer);
        assert_eq!(witness.levels.len(),4);
        assert!(witness.levels.iter().any(|(_,group,_)|group.is_some()));
        assert!(!observation.bound.as_ref().unwrap().iter().any(|r|r.fragment==fragment));
    }
    #[test]
    fn extended_member_cannot_use_the_original_head_container_for_sharing() {
        let (canonical,observation,fragment)=split_fixture(48);
        assert!(observation.sharing_witness(&canonical,fragment).is_none());
    }
    #[test]
    fn a_different_projection_document_cannot_reuse_a_membership_witness() {
        let (mut canonical,observation,fragment)=split_fixture(24);
        canonical.written_projection.as_mut().unwrap().original_xml=std::sync::Arc::from("foreign");
        assert!(observation.sharing_witness(&canonical,fragment).is_none());
    }
    #[test]
    fn a_changed_locator_cannot_reuse_a_membership_witness() {
        let (mut canonical,observation,fragment)=split_fixture(24);
        let mut locators=canonical.source_locators.to_vec();locators[0].attack_id+=133;
        canonical.source_locators=locators.into();
        assert!(observation.sharing_witness(&canonical,fragment).is_none());
    }
    #[test]
    fn unknown_attachment_never_supplies_container_membership() {
        let (canonical,mut observation,fragment)=split_fixture(24);
        observation.chord_attachments.as_mut().unwrap()[0].unknown=Some("missing original head".into());
        assert!(observation.sharing_witness(&canonical,fragment).is_none());
    }
    #[test]
    fn canonical_owned_head_keeps_its_independent_decision_kind() {
        let (canonical,observation,_)=split_fixture(24);
        let row=&observation.bound.as_ref().unwrap()[0];
        assert!(observation.sharing_witness(&canonical,row.fragment).unwrap().kind==SharingBeamKind::OwnedDecision);
    }

    #[test]
    fn complete_channel_search_uses_the_original_staff_split_container() {
        let source=crate::pipeline::actual_beam_owner_capture_tests::input_with_original_chord()
            .replace("probe",&83usize.to_string())
            .replace("<chord/><pitch><step>A</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff>",
                "<chord/><pitch><step>A</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>2</staff>");
        let lower=["C","D","E","F"].into_iter().map(|step|format!(
            "<note><pitch><step>{step}</step><octave>3</octave></pitch><duration>24</duration><voice>4</voice><type>16th</type><staff>2</staff></note>"
        )).collect::<String>();
        let source=source.replace("</measure>",&format!("<backup><duration>384</duration></backup>{lower}<note><rest/><duration>288</duration><voice>4</voice><type>half</type><dot/><staff>2</staff></note></measure>"));
        let (xml,_,capture)=crate::pipeline::actual_beam_owner_capture_tests::execute(&source);
        let evidence=final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical,_)=plan_canonical_final_chord_structure_once(&xml,&evidence).unwrap();
        let observation=observe_actual_source_beam_binding(&canonical,&xml,Some(&Ok(capture)));
        let member=observation.chord_attachments.as_ref().unwrap()[0].member.0;
        let upper=canonical.source_locators.iter().find(|l|l.staff=="1").unwrap().source_event_id;
        let classes=canonical.source_locators.iter().map(|l|(l.source_event_id,
            if l.staff=="1" {upper} else {l.source_event_id})).collect::<std::collections::BTreeMap<_,_>>();
        let source_roles=canonical.source_locators.iter().map(|l|(l.source_event_id,
            if l.staff=="1" {SerializedVoicePresentationRole::UpperPrimary}
            else if l.source_event_id==member {SerializedVoicePresentationRole::LowerAuxiliary}
            else {SerializedVoicePresentationRole::LowerSecondary})).collect::<std::collections::BTreeMap<_,_>>();
        let order=classes.values().copied().collect::<std::collections::BTreeSet<_>>().into_iter().collect::<Vec<_>>();
        let partition=order.iter().enumerate().map(|(i,c)|(*c,i+17)).collect::<std::collections::BTreeMap<_,_>>();
        let class_role=order.iter().map(|c|(*c,source_roles[c])).collect::<std::collections::BTreeMap<_,_>>();
        let roles=order.iter().map(|c|(partition[c],class_role[c])).collect::<std::collections::BTreeMap<_,_>>();
        let model=channel_consumer_model(canonical.written_projection.as_ref().unwrap(),&classes).unwrap();
        assert!(channel_consumer_overflow(&model,&partition,&roles).is_err());
        let context=cross_role_channel_sharing::SharingContext::new(&canonical,&observation,&classes,&source_roles,&[]);
        let eligible=context.eligible_classes(); assert_eq!(eligible.len(),order.len());
        let empty=std::collections::BTreeSet::new();
        let input=ChannelSearchInput{model:&model,class_order:&order,class_role:&class_role,
            semantic_different:&empty,written_different:&empty};
        let missing_observation=ActualSourceBeamBindingObservation::default();
        let no_origin=cross_role_channel_sharing::SharingContext::new(&canonical,&missing_observation,&classes,&source_roles,&[]);
        assert!(no_origin.search(&input,&partition,&roles,&eligible).is_none());
        assert_eq!(no_origin.diagnostic_count("missing_exact_original_capture_origin"),1);
        let conflicts=order.iter().enumerate().flat_map(|(i,a)|order[i+1..].iter()
            .map(move |b|((*a).min(*b),(*a).max(*b)))).collect::<std::collections::BTreeSet<_>>();
        let blocked=ChannelSearchInput{model:&model,class_order:&order,class_role:&class_role,
            semantic_different:&conflicts,written_different:&empty};
        assert!(context.search(&blocked,&partition,&roles,&eligible).is_none());
        assert!(context.diagnostic_count("guest_host_semantic_or_written_conflict")>0);
        let mut foreign_classes=classes.clone();
        foreign_classes.insert(classes.keys().copied().max().unwrap()+1,order[0]);
        let missing=cross_role_channel_sharing::SharingContext::new(&canonical,&observation,&foreign_classes,&source_roles,&[]);
        assert!(missing.search(&input,&partition,&roles,&eligible).is_none());
        assert_eq!(missing.diagnostic_count("baseline_projection_failed"),1);
        let (after,after_roles,_,_)=context.search(&input,&partition,&roles,&eligible).unwrap();
        assert!(context.diagnostic_count("consumer_admissible_partition_checked")>0);
        assert_eq!(context.diagnostic_count("guest_host_semantic_or_written_conflict"),0);
        assert!(channel_consumer_overflow(&model,&after,&after_roles).is_ok());
        assert_eq!(after[&upper],partition[&upper]);
        assert_ne!(after[&member],partition[&member]);
    }

    #[test]
    fn complete_channel_search_admits_hosts_with_retained_original_chords() {
        let source=crate::pipeline::actual_beam_owner_capture_tests::input_with_original_chord()
            .replace("probe",&83usize.to_string())
            .replace("<chord/><pitch><step>A</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff>",
                "<chord/><pitch><step>A</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>2</staff>");
        let lower=["C","D","E","F"].into_iter().map(|step|format!(
            "<note><pitch><step>{step}</step><octave>3</octave></pitch><duration>24</duration><voice>4</voice><type>16th</type><staff>2</staff></note><note><chord/><pitch><step>A</step><octave>3</octave></pitch><duration>24</duration><voice>4</voice><type>16th</type><staff>2</staff></note>"
        )).collect::<String>();
        let source=source.replace("</measure>",&format!("<backup><duration>384</duration></backup>{lower}<note><rest/><duration>288</duration><voice>4</voice><type>half</type><dot/><staff>2</staff></note></measure>"));
        let (xml,_,capture)=crate::pipeline::actual_beam_owner_capture_tests::execute(&source);
        let evidence=final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical,_)=plan_canonical_final_chord_structure_once(&xml,&evidence).unwrap();
        let observation=observe_actual_source_beam_binding(&canonical,&xml,Some(&Ok(capture)));
        let member=observation.chord_attachments.as_ref().unwrap()[0].member.0;
        let upper=canonical.source_locators.iter().find(|l|l.staff=="1").unwrap().source_event_id;
        let classes=canonical.source_locators.iter().map(|l|(l.source_event_id,
            if l.staff=="1" {upper} else {canonical.actions.iter().find(|a|a.fragment_id==l.fragment_id
                && a.representation_role==FinalChordRepresentationRole::RetainedChordMember)
                .and_then(|a|a.host_source_event_id).unwrap_or(l.source_event_id)})).collect::<std::collections::BTreeMap<_,_>>();
        let source_roles=canonical.source_locators.iter().map(|l|(l.source_event_id,
            if l.staff=="1" {SerializedVoicePresentationRole::UpperPrimary}
            else if l.source_event_id==member {SerializedVoicePresentationRole::LowerAuxiliary}
            else {SerializedVoicePresentationRole::LowerSecondary})).collect::<std::collections::BTreeMap<_,_>>();
        let order=classes.values().copied().collect::<std::collections::BTreeSet<_>>().into_iter().collect::<Vec<_>>();
        let partition=order.iter().enumerate().map(|(i,c)|(*c,i+17)).collect::<std::collections::BTreeMap<_,_>>();
        let class_role=order.iter().map(|c|(*c,source_roles[c])).collect::<std::collections::BTreeMap<_,_>>();
        let roles=order.iter().map(|c|(partition[c],class_role[c])).collect::<std::collections::BTreeMap<_,_>>();
        let model=channel_consumer_model(canonical.written_projection.as_ref().unwrap(),&classes).unwrap();
        assert!(channel_consumer_overflow(&model,&partition,&roles).is_err());
        let context=cross_role_channel_sharing::SharingContext::new(&canonical,&observation,&classes,&source_roles,&[]);
        let eligible=context.eligible_classes(); assert_eq!(eligible.len(),order.len());
        assert_eq!(canonical.actions.iter().filter(|a|a.representation_role==FinalChordRepresentationRole::RetainedChordMember).count(),4);
        let empty=std::collections::BTreeSet::new();
        let input=ChannelSearchInput{model:&model,class_order:&order,class_role:&class_role,
            semantic_different:&empty,written_different:&empty};
        let (after,after_roles,_,_)=context.search(&input,&partition,&roles,&eligible).unwrap();
        assert!(channel_consumer_overflow(&model,&after,&after_roles).is_ok());
        assert_eq!(after[&upper],partition[&upper]);
        assert_ne!(after[&member],partition[&member]);
    }

    #[test]
    fn non_beamable_source_applicability_is_distinct_from_an_unknown_head_decision() {
        let (mut canonical,mut observation,fragment)=split_fixture_with_type(96,"quarter");
        let attached=observation.chord_attachments.as_mut().unwrap().iter_mut().find(|r|r.member.2==fragment).unwrap();
        attached.head=None; attached.unknown=Some("no independently owned head decision".into());
        let witness=observation.sharing_witness(&canonical,fragment).unwrap();
        assert!(witness.kind==SharingBeamKind::NonBeamable); assert!(witness.levels.is_empty());
        observation.note_shapes.get_mut(&fragment).unwrap().3=false;
        assert!(observation.sharing_witness(&canonical,fragment).is_none());
        canonical.written_projection.as_mut().unwrap().original_xml=std::sync::Arc::from("foreign");
        assert!(observation.sharing_witness(&canonical,fragment).is_none());
    }

    fn retained_container_fixture() -> (CanonicalFinalChordStructuralPlan, ActualSourceBeamBindingObservation, usize) {
        let before=crate::pipeline::actual_beam_owner_capture_tests::input_with_original_chord()
            .replace("probe",&83usize.to_string());
        let (xml,_,capture)=crate::pipeline::actual_beam_owner_capture_tests::execute(&before);
        let evidence=final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical,_)=plan_canonical_final_chord_structure_once(&xml,&evidence).unwrap();
        let observation=observe_actual_source_beam_binding(&canonical,&xml,Some(&Ok(capture)));
        let fragment=observation.chord_attachments.as_ref().unwrap()[0].member.2;
        (canonical,observation,fragment)
    }
    #[test]
    fn retained_original_chord_uses_container_evidence_for_its_complete_hard_same_class() {
        let (canonical,observation,fragment)=retained_container_fixture();
        let attachment=&observation.chord_attachments.as_ref().unwrap()[0];
        let head=attachment.head.unwrap();
        let row=observation.sharing_witness(&canonical,fragment).unwrap();
        assert!(row.kind==SharingBeamKind::ChordContainer);
        assert!(!observation.bound.as_ref().unwrap().iter().any(|r|r.fragment==fragment));
        let classes=canonical.source_locators.iter().map(|l|(l.source_event_id,
            if l.fragment_id==fragment {head.0} else {l.source_event_id}))
            .collect::<std::collections::BTreeMap<_,_>>();
        let roles=canonical.source_locators.iter().map(|l|(l.source_event_id,
            SerializedVoicePresentationRole::UpperAuxiliary)).collect::<std::collections::BTreeMap<_,_>>();
        let context=cross_role_channel_sharing::SharingContext::new(&canonical,&observation,&classes,&roles,&[]);
        assert!(context.eligible_classes().contains(&head.0));
    }
    #[test]
    fn retained_container_requires_the_existing_hard_same_edge() {
        let (mut canonical,observation,fragment)=retained_container_fixture();
        canonical.hard_same_edges=std::sync::Arc::from([]);
        assert!(observation.sharing_witness(&canonical,fragment).is_none());
        assert_eq!(observation.sharing_witness_diagnostic(&canonical,fragment).err(),Some("container_representation_or_hard_same_not_preserved"));
    }
    #[test]
    fn retained_container_rejects_changed_duration_or_representation() {
        let (mut canonical,observation,fragment)=retained_container_fixture();
        let mut actions=canonical.actions.to_vec();
        let action=actions.iter_mut().find(|a|a.fragment_id==fragment).unwrap();
        action.kind=FinalChordCanonicalActionKind::RetainedChordMember{canonical_duration:action.locator.duration+1};
        canonical.actions=actions.into();
        assert!(observation.sharing_witness(&canonical,fragment).is_none());
        let mut actions=canonical.actions.to_vec();
        let action=actions.iter_mut().find(|a|a.fragment_id==fragment).unwrap();
        action.kind=FinalChordCanonicalActionKind::AuxiliaryLane{serialization_run_id:0,canonical_duration:action.locator.duration};
        action.representation_role=FinalChordRepresentationRole::IndependentAuxiliaryFragment;
        canonical.actions=actions.into();
        assert!(observation.sharing_witness(&canonical,fragment).is_none());
    }

    // Synthetic owner execution, canonical planner, actual output receipt, label binder
    // and search. Only a deliberately separated host-pair conflict relation is supplied.
    fn context_preservation_search_fixture(
        staff: u8, guest_role: SerializedVoicePresentationRole,
        host_role: SerializedVoicePresentationRole, separate_measure: bool,
        unowned: bool, written_member_beam: bool, protect_guest_rest: bool,
    ) -> (bool, usize) {
        let other=3-staff;
        let original_member="<chord/><pitch><step>A</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff>";
        let marked=if written_member_beam {"<beam number=\"1\">begin</beam>"} else {""};
        let member=format!("<chord/><pitch><step>A</step><octave>5</octave></pitch><duration>24</duration><voice>{other}</voice><type>16th</type><staff>{staff}</staff>{marked}");
        let mut source=crate::pipeline::actual_beam_owner_capture_tests::input_with_original_chord()
            .replace("probe",&83usize.to_string()).replace(original_member,"MEMBER_TOKEN")
            .replace("<staff>1</staff>",&format!("<staff>{other}</staff>"))
            .replace("<voice>3</voice>",&format!("<voice>{other}</voice>"))
            .replace("MEMBER_TOKEN",&member);
        if protect_guest_rest {
            source=source.replacen(&format!("<voice>{other}</voice>"),&format!("<voice>{staff}</voice>"),1);
        }
        let heads=["C","D","E","F"].into_iter().map(|step|format!(
            "<note><pitch><step>{step}</step><octave>3</octave></pitch><duration>96</duration><voice>4</voice><type>quarter</type><staff>{staff}</staff></note>"
        )).collect::<String>();
        let tail=String::new();
        source=if separate_measure {
            source.replace("</part>",&format!("<measure number=\"{}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{heads}{tail}</measure></part>",83usize.saturating_add(source.matches("</measure>").count())))
        } else {
            source.replace("</measure>",&format!("<backup><duration>{}</duration></backup>{heads}{tail}</measure>",384-if unowned {3*24} else {0}))
        };
        if unowned {
            for step in ["D","E","F"] {
                source=source.replace(&format!("<note><pitch><step>{step}</step><octave>5</octave></pitch><duration>24</duration><voice>{other}</voice><type>16th</type><staff>{other}</staff></note>"),"");
            }
        }
        let (xml,_,capture)=crate::pipeline::actual_beam_owner_capture_tests::execute(&source);
        let evidence=final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical,_)=plan_canonical_final_chord_structure_once(&xml,&evidence).unwrap();
        let observation=observe_actual_source_beam_binding(&canonical,&xml,Some(&Ok(capture)));
        let member=observation.chord_attachments.as_ref().unwrap()[0].member.0;
        if unowned {
            assert!(observation.chord_attachments.as_ref().unwrap()[0].unknown.is_some());
            assert!(observation.sharing_witness(&canonical,observation.chord_attachments.as_ref().unwrap()[0].member.2).is_none());
        }
        let main=canonical.source_locators.iter().find(|l|l.staff==other.to_string()).unwrap().source_event_id;
        let classes=canonical.source_locators.iter().map(|l|(l.source_event_id,
            if l.staff==other.to_string() {main} else {l.source_event_id})).collect::<std::collections::BTreeMap<_,_>>();
        let source_roles=canonical.source_locators.iter().map(|l|(l.source_event_id,
            if l.staff==other.to_string() {if other==1 {SerializedVoicePresentationRole::UpperPrimary} else {SerializedVoicePresentationRole::LowerPrimary}}
            else if l.source_event_id==member {guest_role} else {host_role})).collect::<std::collections::BTreeMap<_,_>>();
        let order=classes.values().copied().collect::<std::collections::BTreeSet<_>>().into_iter().collect::<Vec<_>>();
        let partition=order.iter().enumerate().map(|(i,c)|(*c,i+17)).collect::<std::collections::BTreeMap<_,_>>();
        let class_role=order.iter().map(|c|(*c,source_roles[c])).collect::<std::collections::BTreeMap<_,_>>();
        let roles=order.iter().map(|c|(partition[c],class_role[c])).collect::<std::collections::BTreeMap<_,_>>();
        let legacy=canonical.source_locators.iter().map(|l|LegacyVoiceLabelPreferenceRow{
            source_event_id:l.source_event_id,label:if l.source_event_id==member {"9000".into()} else {format!("100{}",classes[&l.source_event_id])},
        }).collect::<Vec<_>>();
        let model=channel_consumer_model(canonical.written_projection.as_ref().unwrap(),&classes).unwrap();
        assert!(channel_consumer_overflow(&model,&partition,&roles).is_err());
        let context=cross_role_channel_sharing::SharingContext::new(&canonical,&observation,&classes,&source_roles,&legacy);
        let eligible=context.eligible_classes();
        let hosts=order.iter().copied().filter(|c|*c!=main && *c!=member).collect::<Vec<_>>();
        assert_eq!(hosts.len(),4);
        let conflicts=hosts.iter().enumerate().flat_map(|(i,a)|hosts[i+1..].iter()
            .map(move|b|((*a).min(*b),(*a).max(*b)))).collect::<std::collections::BTreeSet<_>>();
        let empty=std::collections::BTreeSet::new();
        let input=ChannelSearchInput{model:&model,class_order:&order,class_role:&class_role,
            semantic_different:&conflicts,written_different:&empty};
        let result=context.search(&input,&partition,&roles,&eligible);
        if let Some((after,after_roles,_,_))=&result {
            assert_eq!(context.diagnostic_count("distributed_candidate_phase_started"),0);
            assert!(channel_consumer_overflow(&model,after,after_roles).is_ok());
            assert_eq!(after[&main],partition[&main]);
            assert!(order.iter().any(|class|after[class]!=partition[class]));
            assert!(after_roles.len()<roles.len());
        }
        (result.is_some(),context.diagnostic_count("unowned_occurrence_exact_measure_context_preserved"))
    }
    #[test]
    fn exact_unknown_context_preservation_admits_a_disjoint_measure_without_inventing_a_decision() {
        let (admitted,preserved)=context_preservation_search_fixture(2,SerializedVoicePresentationRole::LowerAuxiliary,
            SerializedVoicePresentationRole::LowerSecondary,true,true,false,false);
        assert!(admitted); assert!(preserved>0);
    }
    #[test]
    fn unknown_context_with_new_same_measure_partners_is_refused() {
        assert!(!context_preservation_search_fixture(2,SerializedVoicePresentationRole::LowerAuxiliary,
            SerializedVoicePresentationRole::LowerSecondary,false,true,false,false).0);
    }
    #[test]
    fn unknown_written_beam_marks_never_use_context_preservation() {
        assert!(!context_preservation_search_fixture(2,SerializedVoicePresentationRole::LowerAuxiliary,
            SerializedVoicePresentationRole::LowerSecondary,true,true,true,false).0);
    }
    #[test]
    fn complete_same_role_channel_sharing_is_not_discarded_before_compatibility() {
        assert!(context_preservation_search_fixture(2,SerializedVoicePresentationRole::LowerSecondary,
            SerializedVoicePresentationRole::LowerSecondary,false,false,false,false).0);
    }
    #[test]
    fn upper_pitched_role_sharing_preserves_actual_rest_role_inputs() {
        assert!(context_preservation_search_fixture(1,SerializedVoicePresentationRole::UpperPrimary,
            SerializedVoicePresentationRole::UpperSecondary,true,false,false,false).0);
    }
    #[test]
    fn lower_primary_reader_predicate_remains_a_binding_obligation() {
        assert!(!context_preservation_search_fixture(2,SerializedVoicePresentationRole::LowerPrimary,
            SerializedVoicePresentationRole::LowerSecondary,true,false,false,false).0);
    }
    #[test]
    fn an_actual_rest_on_the_guest_label_keeps_that_channel_protected() {
        assert!(!context_preservation_search_fixture(1,SerializedVoicePresentationRole::UpperPrimary,
            SerializedVoicePresentationRole::UpperSecondary,true,false,false,true).0);
    }
    #[test]
    fn retained_nonspacing_member_keeps_only_its_original_container_relation() {
        let before=crate::pipeline::actual_beam_owner_capture_tests::input_with_original_chord()
            .replace("probe",&83usize.to_string()).replace("<note><chord/>","<note print-spacing=\"no\"><chord/>");
        let (xml,_,capture)=crate::pipeline::actual_beam_owner_capture_tests::execute(&before);
        let evidence=final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical,_)=plan_canonical_final_chord_structure_once(&xml,&evidence).unwrap();
        let observation=observe_actual_source_beam_binding(&canonical,&xml,Some(&Ok(capture)));
        let fragment=observation.chord_attachments.as_ref().unwrap()[0].member.2;
        let witness=observation.sharing_witness(&canonical,fragment).unwrap();
        assert!(witness.kind==SharingBeamKind::ChordContainer);
        assert!(!observation.bound.as_ref().unwrap().iter().any(|r|r.fragment==fragment));
    }

    fn distributed_channel_fixture(block_one_group: bool, copies: usize) -> bool {
        let source=crate::pipeline::actual_beam_owner_capture_tests::input_with_original_chord()
            .replace("probe",&83usize.to_string()).replace("<staff>1</staff>","<staff>2</staff>")
            .replace("<voice>3</voice>","<voice>2</voice>");
        let attrs="<attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>";
        let hosts=["C","D","E","F"].into_iter().map(|step|format!(
            "<note><pitch><step>{step}</step><octave>3</octave></pitch><duration>96</duration><voice>4</voice><type>quarter</type><staff>1</staff></note>"
        )).collect::<String>();
        let mut extra=format!("<measure number=\"{}\">{attrs}{hosts}</measure>",83usize+source.matches("</measure>").count());
        for step in (0..copies).flat_map(|_|["G","A"]) {
            extra.push_str(&format!("<measure number=\"{}\">{attrs}<note><pitch><step>{step}</step><octave>3</octave></pitch><duration>384</duration><voice>5</voice><type>whole</type><staff>1</staff></note></measure>",83usize+source.matches("</measure>").count()+extra.matches("</measure>").count()));
        }
        let source=source.replace("</part>",&format!("{extra}</part>"));
        let (xml,_,capture)=crate::pipeline::actual_beam_owner_capture_tests::execute(&source);
        let evidence=final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical,_)=plan_canonical_final_chord_structure_once(&xml,&evidence).unwrap();
        let observation=observe_actual_source_beam_binding(&canonical,&xml,Some(&Ok(capture)));
        let main=canonical.source_locators.iter().find(|l|l.staff=="2").unwrap().source_event_id;
        let classes=canonical.source_locators.iter().map(|l|(l.source_event_id,if l.staff=="2" {main} else {l.source_event_id})).collect::<std::collections::BTreeMap<_,_>>();
        let mut upper=classes.values().copied().filter(|c|*c!=main).collect::<Vec<_>>();upper.sort();upper.dedup();assert_eq!(upper.len(),4+2*copies);
        let hosts=&upper[..4];let guests=&upper[4..];
        let source_roles=classes.iter().map(|(s,c)|(*s,if *c==main {SerializedVoicePresentationRole::LowerPrimary} else {SerializedVoicePresentationRole::UpperSecondary})).collect::<std::collections::BTreeMap<_,_>>();
        let order=std::iter::once(main).chain(upper.iter().copied()).collect::<Vec<_>>();
        let partition=order.iter().map(|c|(*c,if guests.contains(c) {997} else {*c+101})).collect::<std::collections::BTreeMap<_,_>>();
        let class_role=order.iter().map(|c|(*c,source_roles[c])).collect::<std::collections::BTreeMap<_,_>>();
        let roles=order.iter().map(|c|(partition[c],class_role[c])).collect::<std::collections::BTreeMap<_,_>>();
        let mut conflicts=std::collections::BTreeSet::new();
        for (i,a) in hosts.iter().enumerate() {for b in &hosts[i+1..] {conflicts.insert(((*a).min(*b),(*a).max(*b)));}}
        for (index,guest) in guests.iter().enumerate() {for (host_index,host) in hosts.iter().enumerate() {
            if host_index!=index%2 || (block_one_group && index==1) {conflicts.insert(((*guest).min(*host),(*guest).max(*host)));}
        }}
        let model=channel_consumer_model(canonical.written_projection.as_ref().unwrap(),&classes).unwrap();
        assert!(channel_consumer_overflow(&model,&partition,&roles).is_err());
        let empty=std::collections::BTreeSet::new();
        let input=ChannelSearchInput{model:&model,class_order:&order,class_role:&class_role,semantic_different:&conflicts,written_different:&empty};
        let legacy=classes.iter().map(|(source,class)|LegacyVoiceLabelPreferenceRow{source_event_id:*source,label:if guests.contains(class) {"9000".into()} else {format!("100{class}")}}).collect::<Vec<_>>();
        let context=cross_role_channel_sharing::SharingContext::new(&canonical,&observation,&classes,&source_roles,&legacy);
        let result=context.search(&input,&partition,&roles,&context.eligible_classes());
        if let Some((after,after_roles,_,baseline_voices))=&result {
            let final_channels=FrozenSerializationVoiceChannelPlan {
                authority_bound:true,
                source_to_channel:classes.iter().map(|(&source,class)|(source,after[class])).collect(),
                source_to_role:source_roles.clone(), channel_to_role:after_roles.clone(),
                channel_count:after_roles.len(),semantic_fingerprint_fnv64:canonical.semantic_fingerprint_fnv64,
                consumer_allocation:ChannelConsumerAllocation::Undetermined("synthetic returned sharing plan"),
                label_continuation:Some(baseline_voices.clone()),
            };
            let final_voices=bind_musicxml_voice_labels(&final_channels,&legacy).unwrap();
            for (&source,class) in &classes {
                if !guests.contains(class) {
                    assert_eq!(final_voices.label_for_source(source),baseline_voices.label_for_source(source));
                }
                assert_eq!(final_voices.role_for_source(source),baseline_voices.role_for_source(source));
            }
            let bound=bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
                &xml,&canonical,&final_voices,None).unwrap();
            let emitted=project_canonical_bound_final_chord_encoding_plan_for_transaction_output_alpha(
                &xml,&canonical,&bound).unwrap();
            verify_canonical_final_chord_transaction_local_output_alpha(
                &emitted.xml,&canonical,&final_voices,None,&emitted.receipt,
                bound.plan.source_fingerprint,bound.plan.source_len).unwrap();
            assert!(channel_consumer_overflow_with_labels(&model,after,after_roles,Some(&final_voices)).is_ok());
            for (index,guest) in guests.iter().enumerate() {assert_eq!(after[guest],partition[&hosts[index%2]]);}
            assert_eq!(after[&main],partition[&main]);assert!(!after_roles.contains_key(&997));
            assert_eq!(context.diagnostic_count("whole_channel_distributed_attempted"),1);
            assert_eq!(context.diagnostic_count("distributed_candidate_phase_started"),1);
            assert!(context.diagnostic_count("distributed_component_host_predicate_evaluated")<=guests.len()*hosts.len());
        }
        result.is_some()
    }
    #[test]
    fn complete_measure_components_can_evacuate_one_channel_to_distinct_hosts() {
        assert!(distributed_channel_fixture(false,1));
    }
    #[test]
    fn distributed_channel_search_refuses_a_component_with_no_conflict_free_host() {
        assert!(!distributed_channel_fixture(true,1));
    }

    #[test]
    fn distributed_guest_majorities_preserve_productive_host_labels() {
        assert!(distributed_channel_fixture(false,3));
    }

    #[test]
    fn many_independent_components_use_one_linear_host_selection_pass() {
        assert!(distributed_channel_fixture(false,24));
    }

}
