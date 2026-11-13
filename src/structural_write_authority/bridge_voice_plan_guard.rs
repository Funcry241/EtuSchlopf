//! Read-only, exact-input binding of a proposed connection to the already sealed
//! voice plan. This grants no voice edit and predicts no later alignment.
use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BridgeVoiceAdmission { SameLane, SplitLane, Unbound }

#[derive(Debug, Clone)]
struct BridgeHeadContext {
    measure: usize,
    start: i64,
    end: i64,
    staff: String,
    resources: Vec<(i64, i64, String)>,
}

#[derive(Debug, Clone)]
pub(super) struct BoundBridgeVoiceGuard {
    context: BTreeMap<(usize, usize), BridgeHeadContext>,
    post_projection: bool,
    input: String,
    // The keys locate ranges authenticated by the existing canonical bind.
    // They never create source identities or replace missing bindings.
    heads: BTreeMap<(usize, usize), Option<BTreeSet<String>>>,
    labels: BTreeMap<(usize, usize), String>,
    bindings: BTreeMap<(usize, usize), (usize, usize, usize)>,
    // r1484: groups split in the sealed plan may be written provisionally only
    // while the metric owner can bind their complete extension evidence. `None` offers every
    // such group once; `Some` keeps only groups whose evidence was bound.
    provisional: Option<BTreeSet<Vec<(usize, usize)>>>,
}

impl CanonicalFinalChordStructuralPlan {
    pub(crate) fn bridge_voice_admission(&self, input: &str, members: &[(usize, usize)]) -> Option<bool> {
        match self.bridge_voice_guard.as_ref()?.admission(input, members) {
            BridgeVoiceAdmission::SameLane => Some(true),
            BridgeVoiceAdmission::SplitLane => Some(false),
            BridgeVoiceAdmission::Unbound => None,
        }
    }

    /// A split group may be written only as a provisional E050 extension whose
    /// complete evidence is bound in the same call; later withdrawal decides it.
    pub(crate) fn provisional_bridge_offered(&self, members: &[(usize, usize)]) -> bool {
        self.bridge_voice_guard.as_ref().is_some_and(|guard| {
            guard.provisional.as_ref().is_none_or(|bound| bound.contains(members))
        })
    }

    /// Same guard, offering only the provisional groups whose evidence was bound.
    pub(crate) fn with_bound_provisional_bridges(&self, bound: BTreeSet<Vec<(usize, usize)>>) -> Self {
        let mut narrowed = self.clone();
        if let Some(guard) = narrowed.bridge_voice_guard.as_mut() {
            guard.provisional = Some(bound);
        }
        narrowed
    }

    #[cfg(test)]
    pub(crate) fn synthetic_bridge_voice_guard(input: &str) -> Self {
        Self { bridge_voice_guard: Some(BoundBridgeVoiceGuard::synthetic_same_lane(input)), ..Self::default() }
    }

    pub(crate) fn bind_projected_bridge_voice_guard(
        &self,
        source: &FinalPdfSourceOccurrenceContinuation,
        voices: &FrozenSerializedVoicePlan,
        late_plan: Option<&PrimaryBeamVoicePartitionBoundPlan>,
        omissions: &BTreeSet<RenderPrunedFinalChordOmissionKey>,
    ) -> io::Result<Self> {
        verify_final_pdf_source_occurrence_continuation(source.xml(), self, voices, Some(omissions), source)?;
        let rebound = late_plan.map(|plan|
            rebind_primary_beam_voice_partition_plan_to_current_xml(source.xml(), source, plan)).transpose()?;
        let connection_voices = rebound.as_ref().map_or(voices, |plan| plan.repaired_voice_plan());
        let measures = final_chord_parse_binding_measures(source.xml())?;
        let mut by_identity = BTreeMap::<(usize, usize, usize), Vec<&FinalChordTransactionOutputOccurrenceWitness>>::new();
        for row in &source.receipt.occurrences {
            by_identity.entry((row.source_event_id, row.attack_id, row.fragment_id)).or_default().push(row);
        }
        let mut labels = BTreeMap::new();
        let mut bindings = BTreeMap::new();
        let protected = source.receipt.rest_bridge_extension_course.as_ref()
            .map_or_else(BTreeSet::new, |course| course.protected_occurrences());
        for (identity, rows) in by_identity {
            if protected.contains(&identity) { continue; }
            // A split/tied materialization is not silently collapsed into one
            // old physical head. Refuse only groups containing such resources.
            let [row] = rows.as_slice() else { continue; };
            if row.materialization_kind != FinalChordProjectionMaterializationKind::SurvivingSourceNote
                || row.fragment_ordinal != 0 { continue; }
            let note = final_chord_transaction_output_occurrence_note(&measures, row)?;
            let label = connection_voices.label_for_source(row.source_event_id).ok_or_else(||
                io::Error::new(io::ErrorKind::InvalidData, "Projected bridge lost final bound source label"))?;
            let key = (note.range.start, note.range.end);
            if labels.insert(key, label.to_owned()).is_some() || bindings.insert(key, identity).is_some() {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Projected bridge has duplicate physical custody"));
            }
        }
        let mut guarded = self.clone();
        let mut guard = BoundBridgeVoiceGuard::from_bound_labels(source.xml(), &labels)?;
        guard.bindings = bindings;
        guard.post_projection = true;
        guarded.bridge_voice_guard = Some(guard);
        Ok(guarded)
    }

    pub(crate) fn bridge_is_post_projection(&self) -> bool {
        self.bridge_voice_guard.as_ref().is_some_and(|guard| guard.post_projection)
    }

    pub(crate) fn bind_bridge_voice_guard(
        &self,
        input: &str,
        voices: &FrozenSerializedVoicePlan,
        omissions: &BTreeSet<RenderPrunedFinalChordOmissionKey>,
    ) -> io::Result<Self> {
        let bound = bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
            input, self, voices, Some(omissions),
        )?;
        let mut labels = BTreeMap::new();
        let mut bindings = BTreeMap::new();
        let mut identities = BTreeSet::new();
        for row in &bound.input_bindings {
            bindings.insert((row.source_note_start, row.source_note_end),
                (row.source_event_id, row.attack_id, row.fragment_id));
            let label = voices.label_for_source(row.source_event_id)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData,
                    "Bridge voice binding lost sealed source label"))?;
            if !identities.insert((row.source_event_id, row.attack_id, row.fragment_id))
                || labels.insert((row.source_note_start, row.source_note_end), label.to_owned()).is_some()
            {
                return Err(io::Error::new(io::ErrorKind::InvalidData,
                    "Bridge voice binding has duplicate occurrence custody"));
            }
        }
        let mut guarded = self.clone();
        guarded.bridge_voice_guard = Some(BoundBridgeVoiceGuard::from_bound_labels(input, &labels)?);
        guarded.bridge_voice_guard.as_mut().expect("constructed guard").bindings = bindings;
        Ok(guarded)
    }
}

impl BoundBridgeVoiceGuard {
    pub(super) fn occurrence_labels(&self) -> &BTreeMap<(usize, usize), String> { &self.labels }
    pub(super) fn matches_input(&self, input: &str) -> bool { self.input == input }
    pub(super) fn occurrence_bindings(&self) -> &BTreeMap<(usize, usize), (usize, usize, usize)> { &self.bindings }
    fn from_bound_labels(input: &str, labels: &BTreeMap<(usize, usize), String>) -> io::Result<Self> {
        let mut heads = BTreeMap::new();
        let mut context = BTreeMap::new();
        for measure in final_chord_parse_binding_measures(input)? {
            for (index, head) in measure.notes.iter().enumerate() {
                if head.chord_member || head.rest_like || head.pitch_midi.is_none() { continue; }
                let resources = measure.notes.iter().enumerate().filter(|(member_index, member)| {
                    *member_index == index || (member.chord_member && member.anchor_index == Some(index))
                }).map(|(_, note)| note).collect::<Vec<_>>();
                let relation = resources.iter().map(|note| labels.get(&(note.range.start, note.range.end)).cloned())
                    .collect::<Option<BTreeSet<_>>>();
                let key = (head.range.start, head.range.end);
                context.insert(key, BridgeHeadContext {
                    measure: measure.measure, start: head.onset,
                    end: head.onset.saturating_add(head.duration), staff: head.staff.clone(),
                    resources: resources.iter().map(|note| (note.onset, note.onset.saturating_add(note.duration), note.staff.clone())).collect(),
                });
                heads.insert(key, relation);
            }
        }
        Ok(Self { context, post_projection: false, input: input.to_owned(), heads, labels: labels.clone(), bindings: BTreeMap::new(), provisional: None })
    }

    fn admission(&self, input: &str, members: &[(usize, usize)]) -> BridgeVoiceAdmission {
        if input != self.input || members.is_empty()
            || members.iter().copied().collect::<BTreeSet<_>>().len() != members.len()
        { return BridgeVoiceAdmission::Unbound; }
        let mut labels = BTreeSet::new();
        for key in members {
            let Some(Some(resource_labels)) = self.heads.get(key) else { return BridgeVoiceAdmission::Unbound; };
            labels.extend(resource_labels.iter().cloned());
        }
        if self.post_projection && labels.len() == 1 {
            let selected = members.iter().map(|key| self.context.get(key)).collect::<Option<Vec<_>>>();
            let Some(selected) = selected else { return BridgeVoiceAdmission::Unbound; };
            let Some(first) = selected.first() else { return BridgeVoiceAdmission::Unbound; };
            if selected.iter().any(|head| head.measure != first.measure || head.staff != first.staff) {
                return BridgeVoiceAdmission::Unbound;
            }
            let start = selected.iter().map(|head| head.start).min().expect("nonempty selected heads");
            let end = selected.iter().map(|head| head.end).max().expect("nonempty selected heads");
            for (key, context) in &self.context {
                if members.contains(key) || context.measure != first.measure
                    || !context.resources.iter().any(|(onset, stop, staff)| staff == &first.staff && *onset < end && start < *stop) {
                    continue;
                }
                match self.heads.get(key) {
                    Some(Some(other)) if other.is_disjoint(&labels) => {},
                    Some(Some(_)) => return BridgeVoiceAdmission::SplitLane,
                    _ => return BridgeVoiceAdmission::Unbound,
                }
            }
        }
        match labels.len() {
            1 => BridgeVoiceAdmission::SameLane,
            0 => BridgeVoiceAdmission::Unbound,
            _ => BridgeVoiceAdmission::SplitLane,
        }
    }

    #[cfg(test)]
    fn synthetic_same_lane(input: &str) -> Self {
        // These older materializer fixtures intentionally assume admission;
        // they do not exercise canonical binding or require numeric labels.
        let document = roxmltree::Document::parse(input).expect("synthetic XML");
        let heads = document.descendants().filter(|node| node.has_tag_name("note"))
            .filter(|node| node.children().any(|child| child.has_tag_name("pitch"))
                && !node.children().any(|child| child.has_tag_name("chord")))
            .map(|node| {
                let range = node.range();
                ((range.start, range.end), Some(["fixture-lane".to_owned()].into_iter().collect()))
            }).collect();
        Self { context: BTreeMap::new(), post_projection: false, input: input.to_owned(), heads, labels: BTreeMap::new(), bindings: BTreeMap::new(), provisional: None }
    }
}

#[cfg(test)]
impl CanonicalFinalChordStructuralPlan {
    /// Fixture producer: IDs and the sealed lane decision are explicitly authored
    /// by the test, then exercised through the production canonical binder.
    fn synthetic_bridge_fixture_parts(input: &str, labels: &[&str]) -> io::Result<(Self, FrozenSerializedVoicePlan)> {
        let mut canonical = Self { authority_bound: true, ..Self::default() };
        let mut voices = FrozenSerializedVoicePlan { authority_bound: true, ..FrozenSerializedVoicePlan::default() };
        let mut locators = Vec::new();
        for measure in final_chord_parse_binding_measures(input)? {
            for note in measure.notes.iter().filter(|note| !note.rest_like && note.pitch_midi.is_some()) {
                let id = locators.len() + 1;
                if let Some(label) = labels.get(id - 1) { voices.source_to_label.insert(id, (*label).to_owned()); }
                locators.push(FinalChordCanonicalSourceLocator {
                    fragment_id: id, source_event_id: id, attack_id: id,
                    measure: measure.measure, raw_note_index: note.raw_note_index,
                    onset: note.onset, duration: note.duration,
                    pitch_identity: note.pitch_identity.clone(), pitch_midi: note.pitch_midi.expect("pitched fixture"),
                    staff: note.staff.clone(), chord_syntax: note.chord_member,
                    tie_start: note.tie_start, tie_stop: note.tie_stop,
                });
            }
        }
        canonical.source_locators = std::sync::Arc::from(locators);
        Ok((canonical, voices))
    }

    pub(crate) fn synthetic_bridge_guard_with_labels(input: &str, labels: &[&str]) -> io::Result<Self> {
        let (canonical, voices) = Self::synthetic_bridge_fixture_parts(input, labels)?;
        canonical.bind_bridge_voice_guard(input, &voices, &BTreeSet::new())
    }

    pub(crate) fn synthetic_projected_bridge_fixture(input: &str, labels: &[&str])
        -> io::Result<(Self, FrozenSerializedVoicePlan, FinalPdfSourceOccurrenceContinuation)> {
        let (canonical, mut voices) = Self::synthetic_bridge_fixture_parts(input, labels)?;
        // Explicit fixture presentation roles, authored before projection. This
        // helper never derives productive roles from an exported label.
        for locator in canonical.source_locators.iter() {
            let role = if locator.staff == "1" { SerializedVoicePresentationRole::UpperPrimary }
                else { SerializedVoicePresentationRole::LowerPrimary };
            voices.source_to_role.insert(locator.source_event_id, role);
            voices.label_to_role.insert(voices.source_to_label[&locator.source_event_id].clone(), role);
        }
        let projected = project_canonical_final_chord_render_occurrence_continuation(
            input, &canonical, &voices, &BTreeSet::new())?;
        let course = FinalPdfSourceOccurrenceContinuation::from_original_transport(
            projected.projected.xml, projected.projected.receipt)?;
        Ok((canonical, voices, course))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn score(chord: bool, measure: &str) -> String {
        let chord = if chord { "<note><chord/><pitch><step>E</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>" } else { "" };
        format!("<score-partwise><part id=\"P1\"><measure number=\"{measure}\"><attributes><divisions>96</divisions></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>{chord}<note><rest/><duration>48</duration><voice>2</voice><staff>2</staff></note><note><pitch><step>G</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note></measure></part></score-partwise>")
    }
    fn heads(xml: &str) -> Vec<(usize, usize)> {
        final_chord_parse_binding_measures(xml).unwrap().iter().flat_map(|m| &m.notes)
            .filter(|n| !n.rest_like && !n.chord_member && n.pitch_midi.is_some())
            .map(|n| (n.range.start, n.range.end)).collect()
    }
    #[test]
    fn sealed_lanes_override_equal_input_labels() {
        let xml = score(false, "11");
        let guard = CanonicalFinalChordStructuralPlan::synthetic_bridge_guard_with_labels(&xml, &["A", "B"]).unwrap();
        assert_eq!(guard.bridge_voice_admission(&xml, &heads(&xml)), Some(false));
    }
    #[test]
    fn equal_sealed_lanes_admit_without_writing_labels() {
        let xml = score(false, "11");
        let guard = CanonicalFinalChordStructuralPlan::synthetic_bridge_guard_with_labels(&xml, &["other", "other"]).unwrap();
        assert_eq!(guard.bridge_voice_admission(&xml, &heads(&xml)), Some(true));
        assert!(xml.contains("<voice>2</voice>"));
    }
    #[test]
    fn absent_sealed_source_is_not_replaced_by_input_voice() {
        let xml = score(false, "11");
        assert!(CanonicalFinalChordStructuralPlan::synthetic_bridge_guard_with_labels(&xml, &["A"]).is_err());
    }
    #[test]
    fn default_plan_cannot_authorize_a_bridge() {
        let xml = score(false, "11");
        assert_eq!(CanonicalFinalChordStructuralPlan::default().bridge_voice_admission(&xml, &heads(&xml)), None);
    }
    #[test]
    fn stale_input_is_unbound_even_for_same_ranges() {
        let xml = score(false, "11");
        let guard = CanonicalFinalChordStructuralPlan::synthetic_bridge_guard_with_labels(&xml, &["A", "A"]).unwrap();
        assert_eq!(guard.bridge_voice_admission(&score(false, "12"), &heads(&xml)), None);
    }
    #[test]
    fn missing_range_empty_group_and_duplicate_members_are_unbound() {
        let xml = score(false, "11");
        let guard = CanonicalFinalChordStructuralPlan::synthetic_bridge_guard_with_labels(&xml, &["A", "A"]).unwrap();
        let h = heads(&xml);
        for members in [vec![], vec![(0, 1)], vec![h[0], h[0]]] {
            assert_eq!(guard.bridge_voice_admission(&xml, &members), None);
        }
    }
    #[test]
    fn attached_chord_member_with_distinct_sealed_lane_refuses_whole_group() {
        let xml = score(true, "11");
        let guard = CanonicalFinalChordStructuralPlan::synthetic_bridge_guard_with_labels(&xml, &["A", "B", "A"]).unwrap();
        assert_eq!(guard.bridge_voice_admission(&xml, &heads(&xml)), Some(false));
    }
    #[test]
    fn complete_chord_with_shared_lane_is_admissible() {
        let xml = score(true, "11");
        let guard = CanonicalFinalChordStructuralPlan::synthetic_bridge_guard_with_labels(&xml, &["A", "A", "A"]).unwrap();
        assert_eq!(guard.bridge_voice_admission(&xml, &heads(&xml)), Some(true));
    }
    #[test]
    fn label_permutation_and_member_order_do_not_change_admission() {
        let xml = score(false, "11");
        for labels in [["1", "1"], ["lane-z", "lane-z"], ["7", "2"]] {
            let guard = CanonicalFinalChordStructuralPlan::synthetic_bridge_guard_with_labels(&xml, &labels).unwrap();
            let mut h = heads(&xml);let before=guard.bridge_voice_admission(&xml, &h);h.reverse();
            assert_eq!(before, guard.bridge_voice_admission(&xml, &h));
            assert_eq!(before, Some(labels[0] == labels[1]));
        }
    }
    #[test]
    fn unsealed_canonical_plan_is_rejected_by_production_constructor() {
        let xml = score(false, "11");
        assert!(CanonicalFinalChordStructuralPlan::default().bind_bridge_voice_guard(&xml,
            &FrozenSerializedVoicePlan::default(), &BTreeSet::new()).is_err());
    }
    #[test]
    fn projected_guard_uses_bound_final_assignment_without_rebinding_occurrence_identity() {
        let ordinal=(usize::from(u8::MAX)%13).to_string();
        let input=score(false,&ordinal);
        let (canonical,voices,course)=CanonicalFinalChordStructuralPlan::synthetic_projected_bridge_fixture(&input,&["2","2"]).unwrap();
        let mut final_voices=voices.clone();
        let source=canonical.source_locators.last().unwrap().source_event_id;
        final_voices.source_to_label.insert(source,"4".to_owned());
        final_voices.label_to_role.insert("4".to_owned(),SerializedVoicePresentationRole::LowerSecondary);
        // An explicitly supplied fixture assignment; no production allocator or
        // role inference is replaced by this negative admission probe.
        let plan=PrimaryBeamVoicePartitionBoundPlan {
            source_fingerprint_fnv64:fingerprint(course.xml()), repaired_voice_plan:final_voices,
            edits:Vec::new(),selected_members:Vec::new(),scope_candidates:1,
            original_group_alignment:Vec::new(), bridge_alignment_proofs:Vec::new(),
        };
        let ordinary=canonical.bind_projected_bridge_voice_guard(&course,&voices,None,&BTreeSet::new()).unwrap();
        let future=canonical.bind_projected_bridge_voice_guard(&course,&voices,Some(&plan),&BTreeSet::new()).unwrap();
        assert_eq!(ordinary.bridge_voice_admission(course.xml(),&heads(course.xml())),Some(true));
        assert_eq!(future.bridge_voice_admission(course.xml(),&heads(course.xml())),Some(false));
        assert_eq!(ordinary.bridge_voice_guard.as_ref().unwrap().bindings,future.bridge_voice_guard.as_ref().unwrap().bindings);

        let extra = "<backup><duration>96</duration></backup><forward><duration>48</duration></forward><note><pitch><step>D</step><octave>3</octave></pitch><duration>24</duration><voice>4</voice><type>16th</type><staff>2</staff></note><forward><duration>24</duration></forward>";
        let input=input.replace("</measure>",&format!("{extra}</measure>"));
        let (canonical,voices,course)=CanonicalFinalChordStructuralPlan::synthetic_projected_bridge_fixture(&input,&["2","2","4"]).unwrap();
        let mut final_voices=voices.clone();
        final_voices.source_to_label.insert(canonical.source_locators.last().unwrap().source_event_id,"2".to_owned());
        let intrusion=PrimaryBeamVoicePartitionBoundPlan {
            source_fingerprint_fnv64:fingerprint(course.xml()),repaired_voice_plan:final_voices,
            edits:Vec::new(),selected_members:Vec::new(),scope_candidates:1,
            original_group_alignment:Vec::new(), bridge_alignment_proofs:Vec::new(),
        };
        let members=heads(course.xml()).into_iter().take(2).collect::<Vec<_>>();
        let ordinary=canonical.bind_projected_bridge_voice_guard(&course,&voices,None,&BTreeSet::new()).unwrap();
        let future=canonical.bind_projected_bridge_voice_guard(&course,&voices,Some(&intrusion),&BTreeSet::new()).unwrap();
        assert_eq!(ordinary.bridge_voice_admission(course.xml(),&members),Some(true));
        assert_eq!(future.bridge_voice_admission(course.xml(),&members),Some(false));
    }

}
