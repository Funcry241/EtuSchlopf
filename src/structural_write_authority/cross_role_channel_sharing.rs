//! Capacity repair through whole-channel reuse. Canonical decisions come only
//! from the original captured owner; projected XML supplies compatibility checks.
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use super::ordered_received_beam_readback::{self, OwnerEdge, OwnerEdgeReadback};

type Partition = BTreeMap<usize, usize>;
type Roles = BTreeMap<usize, SerializedVoicePresentationRole>;
type Placement = (Partition, Roles, Vec<usize>, FrozenSerializedVoicePlan);
type OccurrenceKey = (usize, u8, usize);
type BeamEdge = (u8, OccurrenceKey, OccurrenceKey);
type SeenPartitions = BTreeSet<(Partition, Roles)>;

// A reuse boundary is one immutable canonical plan and one actual owner capture.
// Hashes are diagnostic only. Projection hits require equality of every field of
// the bound voice plan. Eviction changes recomputation, never a decision.
pub(super) struct ProjectionReuse<'a> {
    canonical: &'a CanonicalFinalChordStructuralPlan,
    observation: &'a ActualSourceBeamBindingObservation,
    ordered_edges: Result<Vec<OwnerEdge<'a>>, &'static str>,
    projections: RefCell<std::collections::VecDeque<Rc<SharingProjection>>>,
    witnesses: RefCell<BTreeMap<usize, Result<actual_source_beam_binding::SharingBeamWitness<'a>, &'static str>>>,
    binding_source: std::cell::OnceCell<FinalChordBindingSource<'a>>,
    projection_hits: Cell<usize>,
    projection_misses: Cell<usize>,
    witness_hits: Cell<usize>,
    witness_misses: Cell<usize>,
    edge_loss_detail_emitted: Cell<bool>,
}

impl<'a> ProjectionReuse<'a> {
    pub(super) fn component_overlaps_replaced_entry_rest(
        &self,
        voices: &FrozenSerializedVoicePlan,
        sources: &BTreeSet<usize>,
    ) -> io::Result<bool> {
        let before = self.project(voices.clone())?;
        Ok(before.entry_rests.values().any(|rest| rest.note_index.is_none()
            && before.notes.values().any(|note|
                sources.contains(&note.source) && rest.overlaps(note))))
    }

    // Performance-only LRU bound. All evicted values are recomputed by the same
    // projector; this does not bound traversal or candidate admission.
    const RETAINED_PROJECTIONS: usize = 64;

    pub(super) fn new(canonical: &'a CanonicalFinalChordStructuralPlan,
        observation: &'a ActualSourceBeamBindingObservation) -> Self {
        Self { canonical, observation, ordered_edges: ordered_received_beam_readback::owner_edges(canonical, observation), projections: RefCell::default(), witnesses: RefCell::default(),
            binding_source: std::cell::OnceCell::new(),
            projection_hits: Cell::new(0), projection_misses: Cell::new(0),
            witness_hits: Cell::new(0), witness_misses: Cell::new(0),
            edge_loss_detail_emitted: Cell::new(false) }
    }

    fn matches(&self, canonical: &CanonicalFinalChordStructuralPlan,
        observation: &ActualSourceBeamBindingObservation) -> bool {
        std::ptr::eq(self.canonical, canonical) && std::ptr::eq(self.observation, observation)
    }

    fn witness(&self, fragment: usize)
        -> Result<actual_source_beam_binding::SharingBeamWitness<'a>, &'static str> {
        if let Some(row) = self.witnesses.borrow().get(&fragment).copied() {
            self.witness_hits.set(self.witness_hits.get() + 1);
            return row;
        }
        self.witness_misses.set(self.witness_misses.get() + 1);
        let row = self.observation.sharing_witness_diagnostic(self.canonical, fragment);
        self.witnesses.borrow_mut().insert(fragment, row);
        row
    }

    fn project(&self, voices: FrozenSerializedVoicePlan)
        -> io::Result<Rc<SharingProjection>> {
        let edges = self.ordered_edges.as_deref().map_err(|reason| sharing_error(reason))?;
        let mut entries = self.projections.borrow_mut();
        if let Some(index) = entries.iter().position(|entry| entry.voices == voices) {
            self.projection_hits.set(self.projection_hits.get() + 1);
            let result = entries.remove(index).expect("located projection");
            entries.push_back(Rc::clone(&result));
            return Ok(result);
        }
        drop(entries);
        self.projection_misses.set(self.projection_misses.get() + 1);
        if self.binding_source.get().is_none() {
            let xml = &self.canonical.written_projection.as_ref()
                .ok_or_else(|| sharing_error("Missing source producer"))?.original_xml;
            let source = FinalChordBindingSource::parse(xml)?;
            let _ = self.binding_source.set(source);
        }
        let result = Rc::new(SharingContext::project_voices_from_source(
            self.canonical, voices, Some(edges), self.binding_source.get().expect("parsed source"),
        )?);
        let mut entries = self.projections.borrow_mut();
        if entries.len() == Self::RETAINED_PROJECTIONS { entries.pop_front(); }
        entries.push_back(Rc::clone(&result));
        Ok(result)
    }

    pub(super) fn append_diagnostics(&self, out: &mut String) {
        let row = format!("OriginalNotationAllocationReuse: projection_hits={}; projection_misses={}; witness_hits={}; witness_misses={}; retained_projections={}; equality=complete_bound_voice_plan; lifetime=immutable_owner_scope; authority=none",
            self.projection_hits.get(), self.projection_misses.get(), self.witness_hits.get(),
            self.witness_misses.get(), self.projections.borrow().len());
        let _ = writeln!(out, "{row}");
        eprintln!("{row}");
    }
}

pub(super) struct SharingContext<'a, 'input, 'reuse> {
    canonical: &'a CanonicalFinalChordStructuralPlan,
    observation: &'a ActualSourceBeamBindingObservation,
    classes: &'input Partition,
    source_roles: &'input Roles,
    legacy: &'input [LegacyVoiceLabelPreferenceRow],
    diagnostics: RefCell<SharingDiagnostics>,
    local_reuse: ProjectionReuse<'a>,
    shared_reuse: Option<&'reuse ProjectionReuse<'a>>,
}

// Observations are never queried by admission or traversal decisions.
#[derive(Default)]
struct SharingDiagnostics {
    counters: BTreeMap<&'static str, usize>,
    first_details: BTreeMap<&'static str, String>,
    channel_rows: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SharingNote {
    source: usize,
    attack: usize,
    measure: usize,
    staff: String,
    onset: i64,
    duration: i64,
    ordinal: usize,
    beat_unit: i64,
    beam_levels: u8,
    label: String,
    non_voice_bytes: String,
    states: BTreeMap<u8, String>,
    chord_head: Option<OccurrenceKey>,
    non_pitched_chord_head: Option<(usize, usize)>,
}

// Connected components of the class/measure incidence graph restricted to the
// remaining guest classes. This computes the same least fixed point as repeated
// whole-score scans, while visiting each incident edge only once per component.
struct MeasureIncidence {
    by_class: BTreeMap<usize, BTreeSet<usize>>,
    by_measure: BTreeMap<usize, BTreeSet<usize>>,
}
impl MeasureIncidence {
    fn new(baseline: &SharingProjection, classes: &Partition) -> Self {
        let mut value = Self { by_class: BTreeMap::new(), by_measure: BTreeMap::new() };
        for note in baseline.notes.values() {
            let class = classes[&note.source];
            value.by_class.entry(class).or_default().insert(note.measure);
            value.by_measure.entry(note.measure).or_default().insert(class);
        }
        value
    }
    fn groups(&self, members: &[usize]) -> Vec<Vec<usize>> {
        let mut remaining = members.iter().copied().collect::<BTreeSet<_>>();
        let mut out = Vec::new();
        while let Some(first) = remaining.first().copied() {
            let mut group = BTreeSet::from([first]);
            let mut pending = vec![first];
            let mut visited_measures = BTreeSet::new();
            while let Some(class) = pending.pop() {
                for measure in self.by_class.get(&class).into_iter().flatten() {
                    if !visited_measures.insert(*measure) { continue; }
                    for neighbour in &self.by_measure[measure] {
                        if remaining.contains(neighbour) && group.insert(*neighbour) {
                            pending.push(*neighbour);
                        }
                    }
                }
            }
            for class in &group { remaining.remove(class); }
            out.push(members.iter().copied().filter(|class| group.contains(class)).collect());
        }
        out
    }
}

struct SharingTraversal<'a, 'b> {
    distributed_candidates: bool,
    input: &'a ChannelSearchInput<'b>,
    initial: &'a Partition,
    baseline: &'a SharingProjection,
    measure_incidence: &'a MeasureIncidence,
    refused_compatibility: &'a RefCell<SeenPartitions>,
}

struct ChannelEvacuation<'a> {
    guest: usize,
    hosts: &'a [usize],
    groups: &'a [Vec<usize>],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoundEntryRest {
    interval: (usize, i64, i64, String),
    label: String,
    raw_index: usize,
    note_index: Option<usize>,
}

impl BoundEntryRest {
    fn overlaps(&self, note: &SharingNote) -> bool {
        let (measure, start, duration, staff) = &self.interval;
        note.measure == *measure && note.staff == *staff && note.label == self.label
            && note.onset < start.saturating_add(*duration)
            && note.onset.saturating_add(note.duration) > *start
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SharingProjection {
    voices: FrozenSerializedVoicePlan,
    notes: BTreeMap<OccurrenceKey, SharingNote>,
    non_pitched: Vec<String>,
    entry_rests: BTreeMap<usize, BoundEntryRest>,
    cursor_channels: Vec<(usize, usize)>,
    protected_channels: BTreeSet<usize>,
    protected_resources: BTreeMap<usize, BTreeSet<&'static str>>,
    non_pitched_shadow_roles: Vec<Option<bool>>,
    non_pitched_note_positions: Vec<(usize,i64,i64,String)>,
    edges: BTreeSet<BeamEdge>,
    owner_edges: Vec<OwnerEdgeReadback>,
}

impl<'a, 'input, 'reuse> SharingContext<'a, 'input, 'reuse> {
    pub(super) fn new(
        canonical: &'a CanonicalFinalChordStructuralPlan,
        observation: &'a ActualSourceBeamBindingObservation,
        classes: &'input Partition,
        source_roles: &'input Roles,
        legacy: &'input [LegacyVoiceLabelPreferenceRow],
    ) -> Self {
        Self {
            canonical,
            observation,
            classes,
            source_roles,
            legacy,
            diagnostics: RefCell::default(),
            local_reuse: ProjectionReuse::new(canonical, observation),
            shared_reuse: None,
        }
    }

    pub(super) fn with_reuse(mut self, reuse: Option<&'reuse ProjectionReuse<'a>>) -> Self {
        self.shared_reuse = reuse.filter(|reuse| reuse.matches(self.canonical, self.observation));
        self
    }

    fn reuse(&self) -> &ProjectionReuse<'a> {
        self.shared_reuse.unwrap_or(&self.local_reuse)
    }

    fn observe(&self, reason: &'static str) {
        let mut diagnostics=self.diagnostics.borrow_mut();
        let count=diagnostics.counters.entry(reason).or_default();
        *count=count.saturating_add(1);
    }

    fn refuse(&self, reason: &'static str) -> bool {
        self.observe(reason);
        false
    }

    fn refuse_detail(&self, reason: &'static str, detail: String) -> bool {
        self.diagnostics.borrow_mut().first_details.entry(reason).or_insert(detail);
        self.refuse(reason)
    }

    // Read-only explanation of the already decided refusal. The returned text
    // is emitted only; no traversal, compatibility or writer consumes it.
    fn beam_edge_loss_lines(&self, baseline: &SharingProjection, after: &SharingProjection,
        initial: &Partition, proposed: &Partition, roles: &Roles) -> Vec<String> {
        let lost=baseline.edges.difference(&after.edges).copied().collect::<Vec<_>>();
        let evaluation=self.diagnostic_observed_edge_loss_evaluations();
        let partition_fingerprint=fingerprint(&format!("{proposed:?};{roles:?}"));
        let prefix=format!("FrozenChannelSharingDiagnostic: scope={:016x}; evaluation={evaluation}; partition_fnv64={partition_fingerprint:016x}",self.canonical.semantic_fingerprint_fnv64);
        let moved=proposed.iter().filter(|(class,channel)|initial.get(class)!=Some(channel))
            .map(|(class,channel)|(*class,initial.get(class).copied(),*channel)).collect::<Vec<_>>();
        let mut lines=vec![format!("{prefix}; phase=written_edge_loss_summary; lost_edges={}; baseline_edges={}; proposed_edges={}; moved_class_channels={moved:?}; proposed_roles={roles:?}; authority=none",lost.len(),baseline.edges.len(),after.edges.len())];
        for (level,left,right) in &lost {
            lines.push(format!("{prefix}; phase=written_edge_loss_edge; level={level}; left={left:?}; right={right:?}; left_moved={:?}; right_moved={:?}; authority=none",
                baseline.notes.get(left).and_then(|note|self.classes.get(&note.source)).map(|class|initial.get(class)!=proposed.get(class)),
                baseline.notes.get(right).and_then(|note|self.classes.get(&note.source)).map(|class|initial.get(class)!=proposed.get(class))));
        }
        for (name,projection) in [("baseline",baseline),("proposed",after)] {
            let lanes=lost.iter().flat_map(|(_,left,right)|[*left,*right])
                .filter_map(|key|projection.notes.get(&key))
                .map(|note|(note.measure,note.staff.clone(),note.label.clone())).collect::<BTreeSet<_>>();
            for (measure,staff,label) in lanes {
                let mut ordered=projection.notes.iter().filter(|(_,note)|note.measure==measure
                    && note.staff==staff && note.label==label).collect::<Vec<_>>();
                // Complete note order, including members attached to a head.
                // written_connections consumes each physical head once.
                ordered.sort_by_key(|(_,note)|(note.onset,note.ordinal));
                lines.push(format!("{prefix}; phase=written_edge_loss_lane; projection={name}; measure={measure}; staff={staff:?}; label={label:?}; note_count={}; order=onset_then_pitched_ordinal; authority=none",ordered.len()));
                for (position,(key,note)) in ordered.into_iter().enumerate() {
                    let witness=self.reuse().witness(key.0)
                        .map(|row|(row.source,row.attack,match row.kind {
                            actual_source_beam_binding::SharingBeamKind::OwnedDecision=>"OwnedDecision",
                            actual_source_beam_binding::SharingBeamKind::ChordContainer=>"ChordContainer",
                            actual_source_beam_binding::SharingBeamKind::NonBeamable=>"NonBeamable",
                        },row.levels.to_vec()));
                    let original_locator=self.canonical.source_locators.iter().find(|row|row.fragment_id==key.0);
                    let representation=self.canonical.actions.iter().find(|row|row.fragment_id==key.0)
                        .map(|row|(row.host_source_event_id,&row.representation_role,&row.kind));
                    lines.push(format!("{prefix}; phase=written_edge_loss_note; projection={name}; measure={measure}; staff={staff:?}; label={label:?}; position={position}; occurrence={key:?}; source={}; attack={}; onset={}; pitched_ordinal={}; beam_levels={}; states={:?}; original_witness={witness:?}; original_locator={original_locator:?}; canonical_representation={representation:?}; non_voice_bytes={:?}; chord_head={:?}; authority=none",
                        note.source,note.attack,note.onset,note.ordinal,note.beam_levels,note.states,note.non_voice_bytes,note.chord_head));
                }
            }
        }
        lines
    }

    fn diagnostic_observed_edge_loss_evaluations(&self) -> usize {
        self.diagnostics.borrow().counters.get("existing_written_beam_edge_lost")
            .copied().unwrap_or(0).saturating_add(1)
    }

    fn observe_channels(&self, initial: &Partition, roles: &Roles,
        eligible: &BTreeSet<usize>, baseline: &SharingProjection) {
        let mut members=BTreeMap::<usize,BTreeSet<usize>>::new();
        for (class,channel) in initial { members.entry(*channel).or_default().insert(*class); }
        for (channel,classes) in members {
            let mut failures=BTreeMap::<&'static str,usize>::new();
            let mut first=BTreeMap::<&'static str,(usize,Option<(usize,usize)>)>::new();
            for class in classes.iter().filter(|class|!eligible.contains(class)) {
                let failure=self.canonical.source_locators.iter()
                    .filter(|l|self.classes.get(&l.source_event_id)==Some(class))
                    .find_map(|l|self.reuse().witness(l.fragment_id)
                        .err().map(|reason|(reason,l.source_event_id,l.fragment_id)));
                let (reason,identity)=failure.map(|(reason,source,fragment)|(reason,Some((source,fragment))))
                    .unwrap_or(("class_identity_or_all_level_evidence_invalid",None));
                *failures.entry(reason).or_default()+=1;
                first.entry(reason).or_insert((*class,identity));
            }
            self.diagnostics.borrow_mut().channel_rows.push(format!(
                "channel={channel}; role={:?}; classes={}; eligible_classes={}; protected={}; protected_resources={:?}; class_refusals={failures:?}; first_class_source_fragment={first:?}",
                roles.get(&channel),classes.len(),classes.iter().filter(|c|eligible.contains(c)).count(),
                baseline.protected_channels.contains(&channel),baseline.protected_resources.get(&channel)));
        }
    }

    fn emit_diagnostics(&self, admitted: bool) {
        let diagnostic=self.diagnostics.borrow();
        let scope=self.canonical.semantic_fingerprint_fnv64;
        eprintln!("FrozenChannelSharingDiagnostic: scope={scope:016x}; phase=search_summary; admitted={admitted}; counter_unit=predicate_evaluations; counters={:?}; authority=none",diagnostic.counters);
        for row in &diagnostic.channel_rows {
            eprintln!("FrozenChannelSharingDiagnostic: scope={scope:016x}; phase=initial_channel; {row}; authority=none");
        }
        for (reason,detail) in &diagnostic.first_details {
            eprintln!("FrozenChannelSharingDiagnostic: scope={scope:016x}; phase=first_refusal; reason={reason}; detail={detail:?}; authority=none");
        }
    }

    #[cfg(test)]
    pub(super) fn diagnostic_count(&self, reason: &str) -> usize {
        self.diagnostics.borrow().counters.get(reason).copied().unwrap_or(0)
    }

    fn row(&self, fragment: usize) -> Option<actual_source_beam_binding::SharingBeamWitness<'_>> {
        self.reuse().witness(fragment).ok()
    }

    pub(super) fn eligible_classes(&self) -> BTreeSet<usize> {
        let mut members = BTreeMap::<usize, Vec<&FinalChordCanonicalSourceLocator>>::new();
        for locator in self.canonical.source_locators.iter() {
            if let Some(class) = self.classes.get(&locator.source_event_id) {
                members.entry(*class).or_default().push(locator);
            }
        }
        members
            .into_iter()
            .filter_map(|(class, locators)| {
                locators
                    .iter()
                    .all(|locator| {
                        self.row(locator.fragment_id).is_some_and(|row| {
                            row.source == locator.source_event_id
                                && row.attack == locator.attack_id
                                && (row.kind==actual_source_beam_binding::SharingBeamKind::NonBeamable
                                    || row.levels.len()==4)
                                && row.levels.iter().enumerate().all(
                                    |(index, (level, group, state))| {
                                        usize::from(*level) == index + 1
                                            && group.is_some() == state.is_some()
                                    },
                                )
                        })
                    })
                    .then_some(class)
            })
            .collect()
    }

    fn bound_voices(&self, by_class: &Partition, roles: &Roles,
        continuation: Option<&FrozenSerializedVoicePlan>) -> io::Result<FrozenSerializedVoicePlan> {
        let _timing = crate::audit_only::voice_allocation_timing::Timer::probe("BoundVoices");
        let source_to_channel = self
            .classes
            .iter()
            .map(|(source, class)| {
                by_class
                    .get(class)
                    .copied()
                    .map(|channel| (*source, channel))
                    .ok_or_else(|| sharing_error("Missing whole-class channel"))
            })
            .collect::<io::Result<BTreeMap<_, _>>>()?;
        let channels = FrozenSerializationVoiceChannelPlan {
            authority_bound: true,
            source_to_channel,
            source_to_role: self.source_roles.clone(),
            channel_to_role: roles.clone(),
            channel_count: roles.len(),
            semantic_fingerprint_fnv64: self.canonical.semantic_fingerprint_fnv64,
            consumer_allocation: ChannelConsumerAllocation::Undetermined("read-only sharing probe"),
            label_continuation: continuation.cloned(),
        };
        bind_musicxml_voice_labels(&channels, self.legacy)
    }

    fn projection(&self, by_class: &Partition, roles: &Roles,
        continuation: Option<&FrozenSerializedVoicePlan>) -> io::Result<Rc<SharingProjection>> {
        let voices = self.bound_voices(by_class, roles, continuation)?;
        self.reuse().project(voices)
    }

    #[cfg(test)]
    fn projection_for_voices(
        canonical: &CanonicalFinalChordStructuralPlan,
        voices: FrozenSerializedVoicePlan,
    ) -> io::Result<SharingProjection> {
        Self::project_voices(canonical, voices, None)
    }

    fn project_voices(
        canonical: &CanonicalFinalChordStructuralPlan,
        voices: FrozenSerializedVoicePlan,
        owner_edges: Option<&[OwnerEdge<'_>]>,
    ) -> io::Result<SharingProjection> {
        let xml = &canonical.written_projection.as_ref()
            .ok_or_else(|| sharing_error("Missing source producer"))?.original_xml;
        let source = FinalChordBindingSource::parse(xml)?;
        Self::project_voices_from_source(canonical, voices, owner_edges, &source)
    }

    fn project_voices_from_source(
        canonical: &CanonicalFinalChordStructuralPlan,
        voices: FrozenSerializedVoicePlan,
        owner_edges: Option<&[OwnerEdge<'_>]>,
        parsed: &FinalChordBindingSource<'_>,
    ) -> io::Result<SharingProjection> {
        let _timing = crate::audit_only::voice_allocation_timing::Timer::probe("CanonicalProjection");
        let source = parsed.xml;
        let bound = bind_canonical_final_chord_encoding_plan_from_parsed_source(
            parsed, canonical, &voices, None,
        )?;
        let projected =
            project_canonical_bound_final_chord_encoding_plan_for_transaction_output_alpha(
                source,
                canonical,
                &bound,
            )?;
        let measures = final_chord_parse_binding_measures(&projected.xml)?;
        let mut notes_by_ordinal = BTreeMap::new();
        let mut beat_units = BTreeMap::new();
        for measure in &measures {
            beat_units.insert(measure.measure, measure.divisions.max(1));
            for (ordinal, note) in measure
                .notes
                .iter()
                .filter(|note| !note.rest_like && note.pitch_midi.is_some())
                .enumerate()
            {
                if notes_by_ordinal
                    .insert((measure.measure, ordinal), note)
                    .is_some()
                {
                    return Err(sharing_error("Ambiguous emitted occurrence"));
                }
            }
        }
        let mut notes = BTreeMap::new();
        let mut claimed = BTreeSet::new();
        let mut keys_by_ordinal = BTreeMap::new();
        for witness in &projected.receipt.occurrences {
            let ordinal = (witness.measure, witness.pitched_ordinal);
            let note = notes_by_ordinal
                .get(&ordinal)
                .ok_or_else(|| sharing_error("Missing emitted occurrence"))?;
            let kind = match witness.materialization_kind {
                FinalChordProjectionMaterializationKind::SurvivingSourceNote => 0,
                FinalChordProjectionMaterializationKind::TiedAnchorChain => 1,
                FinalChordProjectionMaterializationKind::AuxiliaryLane => 2,
            };
            let key = (witness.fragment_id, kind, witness.fragment_ordinal);
            let raw = &projected.xml[note.range.clone()];
            let document =
                roxmltree::Document::parse(raw).map_err(|e| sharing_error(&e.to_string()))?;
            let beam_levels = match document
                .root_element()
                .children()
                .find(|n| n.has_tag_name("type"))
                .and_then(|n| n.text())
            {
                Some("eighth") => 1,
                Some("16th") => 2,
                Some("32nd") => 3,
                Some("64th") => 4,
                Some("128th") | Some("256th") | Some("512th") | Some("1024th") => 5,
                Some("quarter" | "half" | "whole" | "breve" | "long" | "maxima") => 0,
                Some(_) | None => 5,
            };
            let mut states = BTreeMap::new();
            for beam in document
                .root_element()
                .children()
                .filter(|n| n.has_tag_name("beam"))
            {
                let level = beam
                    .attribute("number")
                    .unwrap_or("1")
                    .parse::<u8>()
                    .map_err(|e| sharing_error(&e.to_string()))?;
                if states
                    .insert(level, beam.text().unwrap_or_default().trim().to_owned())
                    .is_some()
                {
                    return Err(sharing_error("Ambiguous written beam level"));
                }
            }
            let neutral=voice_neutral_bytes(raw,document.root_element());
            if !claimed.insert(ordinal)
                || notes
                    .insert(
                        key,
                        SharingNote {
                            source: witness.source_event_id,
                            attack: witness.attack_id,
                            measure: witness.measure,
                            staff: note.staff.clone(),
                            onset: note.onset,
                            duration: note.duration,
                            ordinal: witness.pitched_ordinal,
                            beat_unit: beat_units[&witness.measure],
                            beam_levels,
                            label: note.voice.clone(),
                            non_voice_bytes: neutral,
                            states,
                            chord_head: None,
                            non_pitched_chord_head: None,
                        },
                    )
                    .is_some()
            {
                return Err(sharing_error("Duplicate emitted identity"));
            }
            keys_by_ordinal.insert(ordinal, key);
        }
        if claimed.len() != notes_by_ordinal.len() {
            return Err(sharing_error("Incomplete emitted identity coverage"));
        }
        // The parser's physical anchor index and the exact producer receipt
        // identify the emitted head. No pitch/onset match mints this relation.
        for measure in &measures {
            let keys = measure.notes.iter().filter(|note| !note.rest_like && note.pitch_midi.is_some())
                .enumerate().map(|(ordinal, note)| (note.raw_note_index,
                    keys_by_ordinal[&(measure.measure, ordinal)])).collect::<BTreeMap<_, _>>();
            for note in measure.notes.iter().filter(|note| note.chord_member && !note.rest_like && note.pitch_midi.is_some()) {
                let anchor = note.anchor_index.and_then(|index| measure.notes.get(index))
                    .ok_or_else(|| sharing_error("Unbound written chord head"))?;
                let key = keys[&note.raw_note_index];
                let member = notes.get_mut(&key).ok_or_else(|| sharing_error("Missing chord member"))?;
                if anchor.rest_like || anchor.pitch_midi.is_none() {
                    // A preserved non-pitched carrier is not a pitched beam
                    // container. Bind its physical position only for equality;
                    // this supplies no owner, cohort or inherited connection.
                    member.non_pitched_chord_head = Some((measure.measure, anchor.raw_note_index));
                } else {
                    member.chord_head = Some(*keys.get(&anchor.raw_note_index)
                        .ok_or_else(|| sharing_error("Unbound pitched chord head"))?);
                }
            }
        }
        let document = roxmltree::Document::parse(&projected.xml)
            .map_err(|e| sharing_error(&e.to_string()))?;
        // Source cursor obligations are not generated alignment operations.
        // If this staff has any original forward (or an unstated staff), keep
        // its forward-carrying channels protected. Otherwise every projected
        // forward on it was emitted by this exact canonical producer.
        let non_pitched_note_positions=measures.iter().flat_map(|m|
            m.notes.iter().filter(|n|n.rest_like || n.pitch_midi.is_none())
                .map(move |n|(m.measure,n.onset,n.duration,n.staff.clone()))).collect();
        let source_cursor_staffs=original_cursor_staffs(source)?;
        let mut non_pitched = Vec::new();
        let mut non_pitched_shadow_roles = Vec::new();
        let mut protected_channels = BTreeSet::new();
        let mut protected_resources = BTreeMap::<usize,BTreeSet<&'static str>>::new();
        let mut cursor_channels = Vec::new();
        let mut raw_indices = BTreeMap::new();
        let mut note_indices = BTreeMap::new();
        for node in document.descendants().filter(|node| {
            node.is_element()
                && (node.has_tag_name("forward")
                    || node.has_tag_name("backup")
                    || (node.has_tag_name("note")
                        && !node.children().any(|c| c.has_tag_name("pitch"))))
        }) {
            let raw_index = non_pitched.len();
            raw_indices.insert(node.range().start, raw_index);
            let generated_alignment=node.has_tag_name("forward")
                && node.children().find(|n|n.has_tag_name("staff")).and_then(|n|n.text())
                    .is_some_and(|staff| generated_alignment_staff(Some(staff),&source_cursor_staffs));
            if generated_alignment {
                let raw=&projected.xml[node.range()];
                let document=roxmltree::Document::parse(raw).map_err(|e|sharing_error(&e.to_string()))?;
                non_pitched.push(voice_neutral_bytes(raw,document.root_element()));
            } else {
                non_pitched.push(projected.xml[node.range()].to_owned());
            }
            let voice = node
                .children()
                .find(|c| c.has_tag_name("voice"))
                .and_then(|n| n.text());
            if node.has_tag_name("note") {
                note_indices.insert(node.range().start, non_pitched_shadow_roles.len());
                non_pitched_shadow_roles.push(voices.role_for_label(voice.unwrap_or("1"))
                    .map(|role|role.shadow_rest_primary_exempt()));
            }
            if !generated_alignment
                && let Some(label) = voice.or_else(|| node.has_tag_name("note").then_some("1"))
                && let Some(channel) = voices.channel_for_label(label)
            {
                protected_channels.insert(channel);
                if node.has_tag_name("forward") || node.has_tag_name("backup") {
                    cursor_channels.push((raw_index, channel));
                }
                protected_resources.entry(channel).or_default().insert(if node.has_tag_name("forward") {"forward"} else if node.has_tag_name("backup") {"backup"} else {"non_pitched_note"});
            }
        }
        let entry_rests = bound_entry_rests(
            parsed, &projected, &document, &raw_indices, &note_indices,
        )?;
        let edges = written_connections(&notes);
        let owner_edges = owner_edges.map_or_else(Vec::new, |edges|
            ordered_received_beam_readback::read_projected_owner_edges(
                &projected.xml, &measures, &projected.receipt.occurrences, edges));

        Ok(SharingProjection {
            voices,
            notes,
            non_pitched,
            entry_rests,
            cursor_channels,
            protected_channels,
            protected_resources,
            non_pitched_shadow_roles,
            non_pitched_note_positions,
            edges,
            owner_edges,
        })
    }

    fn unchanged_measure_beam_context(
        baseline: &SharingProjection, after: &SharingProjection, key: &OccurrenceKey,
    ) -> bool {
        let (Some(before),Some(current))=(baseline.notes.get(key),after.notes.get(key)) else {return false};
        // Unknown written beam marks cannot supply a new or transported decision.
        if !before.states.is_empty() || !current.states.is_empty()
            || before.beam_levels>4 || current.beam_levels>4 {
            return false;
        }
        let context=|projection: &SharingProjection, note: &SharingNote| {
            let mut rows=projection.notes.iter().filter(|(_,n)|
                n.measure==note.measure && n.label==note.label).collect::<Vec<_>>();
            rows.sort_by_key(|(_,n)|(n.onset,n.ordinal));
            rows.into_iter().map(|(key,n)|(*key,n.staff.clone(),n.onset,n.beat_unit,n.non_voice_bytes.clone()))
                .collect::<Vec<_>>()
        };
        context(baseline,before)==context(after,current)
    }

    fn proves_added_written_pair(&self, (level, left, right): &BeamEdge) -> bool {
        let (Some(a), Some(b)) = (self.row(left.0), self.row(right.0)) else {
            return self.refuse("missing_new_beam_edge_witness");
        };
        let cohort = |row: actual_source_beam_binding::SharingBeamWitness<'_>| {
            row.levels
                .iter()
                .find(|(l, _, _)| l == level)
                .and_then(|(_, group, _)| *group)
        };
        if cohort(a).is_none() || cohort(a) != cohort(b) {
            return self.refuse_detail("new_beam_edge_cohort_mismatch",format!("left={left:?}; right={right:?}; level={level}; original_left={:?}; original_right={:?}",cohort(a),cohort(b)));
        }
        true
    }

    fn compatible(
        &self,
        baseline: &SharingProjection,
        baseline_channels: &Partition,
        by_class: &Partition,
        roles: &Roles,
    ) -> bool {
        let _timing = crate::audit_only::voice_allocation_timing::Timer::probe("Compatibility");
        let voices = match self.bound_voices(by_class, roles, Some(&baseline.voices)) {
            Ok(voices) => voices,
            Err(error) => return self.refuse_detail("proposed_projection_failed", error.to_string()),
        };
        // Contrapositive of existing preservation obligations, not a new
        // admission rule. A passing proposal must keep these occurrence keys,
        // staffs, onsets, chord relations and all non-voice bytes. Consequently
        // its physical head labels must retain every existing written edge.
        if let Some(proof) = unavoidable_written_edge_loss(baseline, &voices) {
            self.observe("written_edge_loss_preflight");
            return self.refuse_detail("existing_written_beam_edge_lost", format!("preflight={proof:?}; premise=existing_occurrence_and_non_voice_preservation"));
        }
        let moved = self
            .classes
            .iter()
            .filter_map(|(source, class)| {
                (baseline_channels.get(class) != by_class.get(class)).then_some(*source)
            })
            .collect::<BTreeSet<_>>();
        // All accepted outputs preserve non-voice bytes and physical heads.
        // Under that mandatory premise, label substitution determines written
        // edges whenever physical-head onset order is unambiguous. A changed
        // relation between unmoved sources therefore cannot pass the full guard.
        if let Some(expected) = preserved_written_connections(baseline, &voices) {
            if let Some(edge) = first_unmoved_connection_change(baseline, &expected, &moved) {
                self.observe("unmoved_connection_change_preflight");
                return self.refuse_detail("unmoved_beam_connections_changed", format!(
                    "preflight_edge={edge:?}; premise=existing_occurrence_and_non_voice_preservation"));
            }
            for edge in expected.difference(&baseline.edges) {
                if !self.proves_added_written_pair(edge) {
                    self.observe("new_connection_owner_proof_preflight");
                    return false;
                }
            }
        }
        let after = match self.reuse().project(voices) {
            Ok(after) => after,
            Err(error) => return self.refuse_detail("proposed_projection_failed", error.to_string()),
        };
        // Observe the old uncontinued vote without using it for admission.
        if let Ok(rebound) = self.bound_voices(by_class, roles, None) {
            for source in self.classes.keys() {
                let left=baseline.voices.label_for_source(*source);
                let right=rebound.label_for_source(*source);
                let label_changed=!moved.contains(source) && left!=right;
                let role_changed=baseline.voices.role_for_source(*source)!=rebound.role_for_source(*source);
                let reader_changed=left.and_then(|label|baseline.voices.role_for_label(label))
                    .map(SerializedVoicePresentationRole::lower_dual_edge_primary)
                    !=right.and_then(|label|rebound.role_for_label(label))
                    .map(SerializedVoicePresentationRole::lower_dual_edge_primary);
                if label_changed || role_changed || reader_changed {
                    self.refuse_detail("legacy_rebinding_first_guard_mismatch",format!(
                        "source={source}; unmoved_label_changed={label_changed}; source_role_changed={role_changed}; lower_primary_reader_changed={reader_changed}; before_label={left:?}; rebound_label={right:?}; before_role={:?}; rebound_role={:?}",
                        baseline.voices.role_for_source(*source),rebound.role_for_source(*source)));
                    break;
                }
            }
        }
        let hosts = moved
            .iter()
            .filter_map(|source| by_class.get(&self.classes[source]).copied())
            .collect::<BTreeSet<_>>();
        let touched = self
            .classes
            .iter()
            .filter_map(|(source, class)| {
                (moved.contains(source)
                    || by_class
                        .get(class)
                        .is_some_and(|channel| hosts.contains(channel)))
                .then_some(*source)
            })
            .collect::<BTreeSet<_>>();
        if baseline.non_pitched != after.non_pitched
            || baseline.non_pitched_shadow_roles != after.non_pitched_shadow_roles
            || baseline.non_pitched_note_positions != after.non_pitched_note_positions
            || baseline.notes.keys().ne(after.notes.keys())
        {
            return self.refuse("non_pitched_bytes_or_occurrence_keys_changed");
        }
        for (source, class) in self.classes {
            if (moved.contains(source)
                && baseline_channels
                    .get(class)
                    .is_some_and(|ch| baseline.protected_channels.contains(ch)))
                || (by_class
                    .get(class)
                    .is_some_and(|ch| hosts.contains(ch) && after.protected_channels.contains(ch)))
            {
                return self.refuse("protected_guest_or_host_channel");
            }
            let (Some(before_label), Some(after_label)) = (
                baseline.voices.label_for_source(*source),
                after.voices.label_for_source(*source),
            ) else {
                return self.refuse("missing_proposed_or_baseline_label");
            };
            let (Some(before_role), Some(after_role)) = (
                baseline.voices.role_for_label(before_label),
                after.voices.role_for_label(after_label),
            ) else {
                return self.refuse("missing_proposed_or_baseline_label_role");
            };
            let changes = [
                ("unmoved_label_changed", !moved.contains(source) && before_label != after_label),
                ("source_role_changed", baseline.voices.role_for_source(*source) != after.voices.role_for_source(*source)),
                ("lower_primary_reader_predicate_changed", before_role.lower_dual_edge_primary() != after_role.lower_dual_edge_primary()),
            ];
            if changes.iter().any(|(_, changed)| *changed) {
                for (reason, changed) in changes {
                    if changed { self.refuse_detail(reason, format!(
                        "source={source}; before_label={before_label:?}; after_label={after_label:?}; before_source_role={:?}; after_source_role={:?}; before_label_role={before_role:?}; after_label_role={after_role:?}",
                        baseline.voices.role_for_source(*source), after.voices.role_for_source(*source))); }
                }
                return self.refuse("unmoved_label_source_role_or_reader_predicate_changed");
            }
        }
        for (key, before) in &baseline.notes {
            let current = &after.notes[key];
            if before.source != current.source
                || before.attack != current.attack
                || before.non_voice_bytes != current.non_voice_bytes
                || before.measure != current.measure || before.staff != current.staff
                || before.onset != current.onset || before.beat_unit != current.beat_unit
                || before.chord_head != current.chord_head
                || before.non_pitched_chord_head != current.non_pitched_chord_head
            {
                return self.refuse("occurrence_identity_or_non_voice_bytes_changed");
            }
            if touched.contains(&current.source) {
                let Some(row) = self.row(key.0) else {
                    if Self::unchanged_measure_beam_context(baseline,&after,key) {
                        self.observe("unowned_occurrence_exact_measure_context_preserved");
                        continue;
                    }
                    return self.refuse_detail("missing_witness_and_measure_context_changed",format!("occurrence={key:?}; source={}",current.source));
                };
                if row.source != current.source
                    || row.attack != current.attack
                    || current.beam_levels > 4
                    || current.states.keys().any(|level| *level > 4 || *level == 0)
                {
                    return self.refuse("touched_occurrence_identity_or_beam_level_invalid");
                }
                if row.kind==actual_source_beam_binding::SharingBeamKind::NonBeamable
                    && (current.beam_levels!=0 || !current.states.is_empty()) { return self.refuse("non_beamable_applicability_changed"); }
                if row.kind==actual_source_beam_binding::SharingBeamKind::OwnedDecision {
                    for (level, _, state) in row.levels {
                        if current.states.get(level) != state.as_ref() { return self.refuse_detail("original_owned_beam_state_changed",format!("fragment={}; source={}; level={level}; original={state:?}; proposed={:?}",key.0,current.source,current.states.get(level))); }
                    }
                }
            }
        }
        if !baseline.edges.is_subset(&after.edges) {
            // Keep the first complete explanation in this immutable batch.
            // Every rejection still increments the existing search counters.
            if !self.reuse().edge_loss_detail_emitted.replace(true) {
                for line in self.beam_edge_loss_lines(baseline, &after, baseline_channels, by_class, roles) {
                    eprintln!("{line}");
                }
            }
            return self.refuse("existing_written_beam_edge_lost");
        }
        let after_notes = after.notes.iter().collect::<Vec<_>>();
        for (index, (left_key, left)) in after_notes.iter().enumerate() {
            for (right_key, right) in &after_notes[index + 1..] {
                if left.measure != right.measure
                    || left.staff != right.staff
                    || left.label != right.label
                    || left.onset.div_euclid(left.beat_unit)
                        != right.onset.div_euclid(right.beat_unit)
                    || baseline.notes[*left_key].label == baseline.notes[*right_key].label
                    || (!moved.contains(&left.source) && !moved.contains(&right.source))
                {
                    continue;
                }
                for level in 1..=left.beam_levels.min(right.beam_levels) {
                    let (Some(a), Some(b)) = (self.row(left_key.0), self.row(right_key.0)) else {
                        return self.refuse("missing_new_shared_beat_pair_witness");
                    };
                    let group = |row: actual_source_beam_binding::SharingBeamWitness<'_>| {
                        row.levels
                            .iter()
                            .find(|(l, _, _)| *l == level)
                            .and_then(|(_, g, _)| *g)
                    };
                    if group(a).is_none() || group(a) != group(b) {
                        return self.refuse_detail("new_shared_beat_pair_cohort_mismatch",format!("left={left_key:?}; right={right_key:?}; level={level}; original_left={:?}; original_right={:?}",group(a),group(b)));
                    }
                }
            }
        }
        let unchanged_edges = |p: &SharingProjection| {
            p.edges
                .iter()
                .copied()
                .filter(|(_, a, b)| {
                    !moved.contains(&p.notes[a].source) && !moved.contains(&p.notes[b].source)
                })
                .collect::<BTreeSet<_>>()
        };
        if unchanged_edges(baseline) != unchanged_edges(&after) {
            return self.refuse("unmoved_beam_connections_changed");
        }
        let prove_pair = |edge: &BeamEdge| self.proves_added_written_pair(edge);
        // The paired readbacks use the same source-owned ordered edge list.
        // An after-state of Present proves its exact two endpoints and strand.
        match check_beam_transition(baseline, &after, prove_pair, |_, old, new|
            old == OwnerEdgeReadback::SplitVoice && new == OwnerEdgeReadback::Present)
        {
            Ok(()) => true,
            Err(failure) => self.refuse(failure.reason()),
        }
    }

    pub(super) fn check_overflow_recolouring(
        &self,
        input: &ChannelSearchInput<'_>,
        baseline: &Partition,
        baseline_roles: &Roles,
        candidate: &Partition,
        candidate_roles: &Roles,
    ) -> io::Result<()> {
        let before = self.bound_voices(baseline, baseline_roles, None)?;
        let after = self.bound_voices(candidate, candidate_roles, None)?;
        input.check_partition(candidate, candidate_roles, Some(&after))
            .map_err(|_| sharing_error("bound overflow candidate is incompatible or unreserved"))?;
        check_written_only_projection(
            None, self.canonical, &before, &after, self.observation,
        )
    }

    pub(super) fn search(
        &self, input: &ChannelSearchInput<'_>, initial: &Partition,
        roles: &Roles, eligible: &BTreeSet<usize>,
    ) -> Option<Placement> {
        let _timing = crate::audit_only::voice_allocation_timing::Timer::span("CrossRoleSearch");
        *self.diagnostics.borrow_mut()=SharingDiagnostics::default();
        let result=self.search_inner(input,initial,roles,eligible);
        self.emit_diagnostics(result.is_some());
        result
    }

    fn search_inner(
        &self,
        input: &ChannelSearchInput<'_>,
        initial: &Partition,
        roles: &Roles,
        eligible: &BTreeSet<usize>,
    ) -> Option<Placement> {
        if !self.observation.sharing_origin_matches(self.canonical) {
            self.observe("missing_exact_original_capture_origin");
            return None;
        }
        let baseline = match self.projection(initial, roles, None) {
            Ok(baseline) => baseline,
            Err(error) => {
                self.refuse_detail("baseline_projection_failed",error.to_string());
                return None;
            }
        };
        self.observe_channels(initial,roles,eligible,&baseline);
        // Preserve the complete accepted whole-channel search before trying
        // the additional deterministic candidate family. Separate memoization
        // is required because the second pass has additional successors.
        let measure_incidence = MeasureIncidence::new(&baseline, self.classes);
        // The predicate has identical inputs in both traversal phases. Only its
        // false result is retained; successors are still traversed in full.
        let refused_compatibility = RefCell::new(SeenPartitions::new());
        for distributed_candidates in [false,true] {
            self.observe(if distributed_candidates {"distributed_candidate_phase_started"} else {"whole_only_phase_started"});
            let mut seen=BTreeSet::new();
            if let Some(result)=self.visit(
                &SharingTraversal{input,initial,baseline:&baseline,distributed_candidates,
                    measure_incidence: &measure_incidence, refused_compatibility: &refused_compatibility},
                initial,roles,&mut seen,
            ) {return Some(result);}
        }
        None
    }

    // Components use exact emitted receipt identities. A whole HARD SAME class
    // and every connected measure input remain together, including cross-staff
    // notes and classes continuing into another measure. No beam consumer input
    // is split merely to find a host.
    #[cfg(test)]
    fn evacuation_groups(baseline: &SharingProjection, members: &[usize], classes: &Partition) -> Vec<Vec<usize>> {
        let mut remaining=members.iter().copied().collect::<BTreeSet<_>>();
        let mut out=Vec::new();
        while let Some(first)=remaining.first().copied() {
            let mut group=BTreeSet::from([first]);
            loop {
                let measures=baseline.notes.values().filter(|n|group.contains(&classes[&n.source]))
                    .map(|n|n.measure).collect::<BTreeSet<_>>();
                let neighbours=baseline.notes.values().filter(|n|measures.contains(&n.measure))
                    .map(|n|classes[&n.source]).filter(|class|remaining.contains(class))
                    .collect::<BTreeSet<_>>();
                let count=group.len();group.extend(neighbours);
                if count==group.len() {break;}
            }
            for class in &group {remaining.remove(class);}
            out.push(members.iter().copied().filter(|c|group.contains(c)).collect());
        }
        out
    }

    fn evacuate_groups(
        &self, traversal: &SharingTraversal<'_, '_>, evacuation: &ChannelEvacuation<'_>,
        current: &Partition, roles: &Roles, seen: &mut SeenPartitions,
    ) -> Option<Placement> {
        // Preserve the accepted deterministic candidate, then vary each whole
        // component's host independently. This finite candidate family is
        // polynomial in components and hosts, not their Cartesian product.
        // Refusal of this family does not prove that no partition exists.
        let first = self.place_evacuation_groups(traversal, evacuation, current, roles, None)?;
        let mut next_roles = roles.clone();
        next_roles.remove(&evacuation.guest);
        self.observe("whole_channel_distributed_attempted");
        if let Some(result) = self.visit(traversal, &first, &next_roles, seen) { return Some(result); }
        for (index, group) in evacuation.groups.iter().enumerate() {
            for &host in evacuation.hosts {
                if group.iter().all(|class| first.get(class) == Some(&host)) { continue; }
                let Some(next) = self.place_evacuation_groups(
                    traversal, evacuation, current, roles, Some((index, host)),
                ) else { continue; };
                self.observe("whole_channel_distributed_alternative_attempted");
                if let Some(result) = self.visit(traversal, &next, &next_roles, seen) { return Some(result); }
            }
        }
        None
    }

    fn place_evacuation_groups(
        &self, traversal: &SharingTraversal<'_, '_>, evacuation: &ChannelEvacuation<'_>,
        current: &Partition, roles: &Roles, alternate: Option<(usize, usize)>,
    ) -> Option<Partition> {
        let mut next = current.clone();
        for (index, group) in evacuation.groups.iter().enumerate() {
            let host = evacuation.hosts.iter().find(|host| {
                self.observe("distributed_component_host_predicate_evaluated");
                alternate.is_none_or(|(changed, selected)| changed != index || **host == selected)
                    && group.iter().all(|a| traversal.input.class_role[a].lower_dual_edge_primary()
                        == roles[*host].lower_dual_edge_primary()
                        && !next.iter().any(|(b, ch)| ch == *host && traversal.input.conflicts(*a, *b)))
            })?;
            for class in group { next.insert(*class, *host); }
        }
        Some(next)
    }

    fn visit(
        &self,
        traversal: &SharingTraversal<'_, '_>,
        current: &Partition,
        roles: &Roles,
        seen: &mut SeenPartitions,
    ) -> Option<Placement> {
        let _timing = crate::audit_only::voice_allocation_timing::Timer::probe("Visit");
        crate::audit_only::voice_allocation_timing::pulse();
        let SharingTraversal {
            input,
            initial,
            baseline,
            distributed_candidates,
            measure_incidence,
            refused_compatibility,
        } = *traversal;
        if !seen.insert((current.clone(), roles.clone())) {
            self.observe("partition_already_visited");
            return None;
        }
        self.observe("partition_visited");
        let voices = match self.bound_voices(current, roles, Some(&baseline.voices)) {
            Ok(voices) => voices,
            Err(error) => { self.refuse_detail("proposed_label_continuation_failed", error.to_string()); return None; }
        };
        let failing = match input.check_partition(current, roles, Some(&voices)) {
            Ok(()) if { self.observe("consumer_admissible_partition_checked"); {
                let key = (current.clone(), roles.clone());
                if refused_compatibility.borrow().contains(&key) {
                    self.observe("compatibility_refusal_reused");
                    false
                } else {
                    let compatible = self.compatible(baseline, initial, current, roles);
                    if !compatible { refused_compatibility.borrow_mut().insert(key); }
                    compatible
                }
            } } => {
                let moved = input
                    .class_order
                    .iter()
                    .copied()
                    .filter(|class| current.get(class) != initial.get(class))
                    .collect();
                return Some((current.clone(), roles.clone(), moved, baseline.voices.clone()));
            }
            Ok(()) => { self.observe("consumer_admissible_partition_refused"); roles.values().copied().map(channel_role_staff).collect() },
            Err(ChannelPartitionRefusal::Capacity(failing)) => {
                self.observe("consumer_overflow_partition");
                failing
            }
            Err(ChannelPartitionRefusal::Conflict) => {
                self.observe("consumer_partition_merges_incompatible_classes");
                return None;
            }
        };
        let mut members = BTreeMap::<usize, Vec<usize>>::new();
        for class in input.class_order {
            members
                .entry(*current.get(class).or_else(|| {self.observe("missing_current_class_channel");None})?)
                .or_default()
                .push(*class);
        }
        let first = |channel: &usize| {
            input
                .class_order
                .iter()
                .position(|class| current.get(class) == Some(channel))
                .unwrap()
        };
        let complete = |channel: &usize| {
            members[channel].iter().all(|class| {
                match initial.get(class) {
                    Some(original) if !baseline.protected_channels.contains(original) => true,
                    Some(_) => self.refuse("whole_channel_has_protected_original_resource"),
                    None => self.refuse("whole_channel_missing_original_assignment"),
                }
            })
        };
        let load = |channel: &usize| {
            input
                .model
                .notes
                .iter()
                .filter(|n| {
                    n.chordrest
                        && n.class
                            .is_some_and(|class| current.get(&class) == Some(channel))
                })
                .count()
        };
        let mut guests = roles
            .keys()
            .copied()
            .filter(|ch| complete(ch) && (failing.contains(&channel_role_staff(roles[ch]))
                || self.refuse("guest_staff_not_overflowed")))
            .collect::<Vec<_>>();
        guests.sort_by_key(|ch| (load(ch), first(ch)));
        for guest in &guests {
            let guest=*guest;
            let mut hosts = roles
                .keys()
                .copied()
                .filter(|ch| {
                    *ch != guest
                        && complete(ch)
                        && (channel_role_staff(roles[ch]) == channel_role_staff(roles[&guest])
                            || self.refuse("host_on_other_staff"))
                })
                .collect::<Vec<_>>();
            hosts.sort_by_key(|ch| (roles[ch].sort_rank(), first(ch)));
            for host in hosts {
                if members[&guest]
                    .iter()
                    .any(|a| members[&host].iter().any(|b| input.conflicts(*a, *b)))
                {
                    self.observe("guest_host_semantic_or_written_conflict");
                    continue;
                }
                if members[&guest].iter().chain(&members[&host]).any(|class| {
                    input.class_role[class].lower_dual_edge_primary() != roles[&host].lower_dual_edge_primary()
                }) { self.observe("guest_host_role_reader_predicate_conflict"); continue; }
                self.observe("whole_channel_merge_attempted");
                let mut next = current.clone();
                for class in &members[&guest] {
                    next.insert(*class, host);
                }
                let mut next_roles = roles.clone();
                next_roles.remove(&guest);
                if let Some(solution) = self.visit(traversal, &next, &next_roles, seen) {
                    return Some(solution);
                }
            }
        }
        if !distributed_candidates {self.observe("partition_children_exhausted");return None;}
        // Second pass only: construct complete placements of whole components.
        for guest in guests {
            let mut groups=measure_incidence.groups(&members[&guest]);
            if groups.len()<2 {continue;}
            let mut hosts=roles.keys().copied().filter(|ch|*ch!=guest && complete(ch)
                && channel_role_staff(roles[ch])==channel_role_staff(roles[&guest])).collect::<Vec<_>>();
            hosts.sort_by_key(|ch|(roles[ch].sort_rank(),first(ch)));
            if hosts.len()<2 {continue;}
            let options=|group: &Vec<usize>|hosts.iter().filter(|host|
                group.iter().all(|a|input.class_role[a].lower_dual_edge_primary()==roles[*host].lower_dual_edge_primary()
                    && !current.iter().any(|(b,ch)|ch==*host && input.conflicts(*a,*b)))).count();
            if groups.iter().any(|group|options(group)==0) {continue;}
            groups.sort_by_key(options);
            if let Some(solution)=self.evacuate_groups(traversal,
                &ChannelEvacuation{guest,hosts:&hosts,groups:&groups},current,roles,seen) {
                return Some(solution);
            }
        }
        self.observe("partition_children_exhausted");
        None
    }
}

/// Read back both canonical byte projections using the same occurrence receipt
/// and written beam parser that the channel search consumes.
pub(super) fn check_written_only_projection(
    mut report: Option<&mut String>,
    canonical: &CanonicalFinalChordStructuralPlan,
    production: &FrozenSerializedVoicePlan,
    probe: &FrozenSerializedVoicePlan,
    original_beams: &ActualSourceBeamBindingObservation,
) -> io::Result<()> {
    let edges = ordered_received_beam_readback::owner_edges(canonical, original_beams);
    let before = SharingContext::project_voices(canonical, production.clone(), edges.as_deref().ok())?;
    let after = SharingContext::project_voices(canonical, probe.clone(), edges.as_deref().ok())?;
    if let Some(out) = report.as_deref_mut() {
        append_voice_probe_comparison(out, canonical.semantic_fingerprint_fnv64, &before, &after);
    }
    let edges = match edges {
        Ok(edges) => edges,
        Err(reason) => {
            if let Some(out) = report {
                let _ = writeln!(out, "WrittenOnlyOwnerBeamEdges: result=Unknown; reason={reason}; authority=none");
            }
            return Err(sharing_error(reason));
        }
    };
    if let Some(out) = report.as_deref_mut() {
        ordered_received_beam_readback::append_owner_edge_comparison(out, &edges, &before.owner_edges, &after.owner_edges);
    }
    let closure = certify_closed_owner_beats(&before, &after, &edges, original_beams);
    validate_written_only_projections(&before, &after, &closure, report.as_deref_mut(), |fragment, note| {
        original_beams.sharing_witness(canonical, fragment).is_some_and(|witness|
            witness.source == note.source && witness.attack == note.attack
                && witness.kind == actual_source_beam_binding::SharingBeamKind::NonBeamable)
    })?;
    if let Some(out) = report && !closure.added_pairs.is_empty() {
        let _ = writeln!(out, "WrittenOnlyOwnerBeatClosure: certified_pairs={}; certified_moves={}; scope=complete_written_beat",
            closure.added_pairs.len(), closure.moved.len());
    }
    Ok(())
}

/// Certify transport of one sealed source-closed original component.
/// Independent notation origin supplies line identity; exact owner and written
/// readback supplies conservation. No unrelated source moves and no edge is added.
pub(super) fn check_original_component_written_reuse(
    canonical: &CanonicalFinalChordStructuralPlan,
    production: &FrozenSerializedVoicePlan,
    proposed: &FrozenSerializedVoicePlan,
    component_sources: &BTreeSet<usize>,
    reuse: &ProjectionReuse<'_>,
) -> io::Result<()> {
    if !canonical.original_beam_voice_groups.iter().any(|group| group.allocation_sources() == *component_sources)
        || component_sources.is_empty()
        || !std::ptr::eq(reuse.canonical, canonical)
        || production.source_to_role != proposed.source_to_role
        || production.source_to_label.keys().ne(proposed.source_to_label.keys())
    { return Err(sharing_error("original_written_reuse_foreign_or_changed_source_roles")); }
    let changes = production.source_to_label.iter().filter_map(|(source, old)| {
        let new = &proposed.source_to_label[source];
        (old != new).then_some((old.as_str(), new.as_str()))
    }).collect::<BTreeSet<_>>();
    if changes.len() != 1 { return Err(sharing_error("notation_reuse_not_one_component_transport")); }
    let &(guest, host) = changes.first().expect("one label substitution");
    let destination_sources = production.source_to_label.iter().filter_map(|(source, label)|
        (label == host).then_some(*source)).collect();
    if !production.source_to_label.values().any(|label| label == host)
        || !canonical.proves_original_notation_voice_for_component_reuse(component_sources, &destination_sources)
        || production.source_to_label.iter().any(|(source, old)|
            proposed.source_to_label[source] != if component_sources.contains(source) { host } else { old.as_str() })
        || component_sources.iter().any(|source| production.label_for_source(*source) != Some(guest))
    { return Err(sharing_error("original_written_reuse_missing_complete_original_line")); }
    let before = reuse.project(production.clone())?;
    let after = reuse.project(proposed.clone())?;
    if before.notes.keys().ne(after.notes.keys()) || before.non_pitched != after.non_pitched
        || before.non_pitched_note_positions != after.non_pitched_note_positions
        || before.non_pitched_shadow_roles != after.non_pitched_shadow_roles
        || before.edges != after.edges || before.owner_edges != after.owner_edges
    { return Err(sharing_error("original_written_reuse_changed_occurrences_rests_or_beams")); }
    let mut rests = Vec::new();
    for raw in &after.non_pitched {
        let doc = roxmltree::Document::parse(raw).map_err(|_| sharing_error("original_written_reuse_unreadable_rest"))?;
        let node = doc.root_element();
        if !node.has_tag_name("note") { continue; }
        let label = match node.children().find(|child| child.has_tag_name("voice")) {
            Some(voice) => voice.text().map(str::trim).filter(|text| !text.is_empty())
                .ok_or_else(|| sharing_error("original_written_reuse_missing_rest_voice"))?,
            None => "1",
        };
        rests.push(label.to_owned());
    }
    if rests.len() != after.non_pitched_note_positions.len() {
        return Err(sharing_error("original_written_reuse_unknown_rest_positions"));
    }
    for (key, old) in &before.notes {
        let new = &after.notes[key];
        if old.source != new.source || old.attack != new.attack || old.measure != new.measure
            || old.staff != new.staff || old.onset != new.onset || old.duration != new.duration
            || old.beat_unit != new.beat_unit || old.chord_head != new.chord_head
            || old.non_pitched_chord_head != new.non_pitched_chord_head
            || old.non_voice_bytes != new.non_voice_bytes
            || production.role_for_label(&old.label).is_none()
            || production.role_for_label(&old.label) != proposed.role_for_label(&new.label)
        { return Err(sharing_error("original_written_reuse_changed_written_content_or_role")); }
        if old.label == new.label { continue; }
        let doc = roxmltree::Document::parse(&old.non_voice_bytes)
            .map_err(|_| sharing_error("original_written_reuse_unreadable_note"))?;
        if old.chord_head.is_some() || old.non_pitched_chord_head.is_some()
            || before.notes.values().any(|note| note.chord_head == Some(*key))
            || doc.descendants().any(|node| node.has_tag_name("tie") || node.has_tag_name("tied") || node.has_tag_name("grace"))
        { return Err(sharing_error("original_written_reuse_chord_tie_or_grace_requires_proof")); }
        let end = new.onset.checked_add(new.duration).filter(|end| *end >= new.onset)
            .ok_or_else(|| sharing_error("original_written_reuse_invalid_written_interval"))?;
        let written = WrittenInterval { measure: new.measure, start: new.onset, end };
        if after.notes.iter().any(|(other_key, other)| key != other_key
            && new.measure == other.measure && new.staff == other.staff && new.label == other.label
            && other.onset.checked_add(other.duration).filter(|end| *end >= other.onset)
                .map(|end| WrittenInterval { measure: other.measure, start: other.onset, end })
                .is_none_or(|other_written| written.overlaps(other_written)))
            || after.non_pitched_note_positions.iter().zip(&rests).any(|((measure, onset, duration, staff), label)|
                *measure == new.measure && *staff == new.staff && *label == new.label
                    && onset.checked_add(*duration).filter(|end| *end >= *onset)
                        .map(|end| WrittenInterval { measure: *measure, start: *onset, end })
                        .is_none_or(|rest_written| written.overlaps(rest_written)))
        { return Err(sharing_error("original_written_reuse_written_occupancy_collision")); }
        for (projection, label) in [(&before, &old.label), (&after, &new.label)] {
            if projection.voices.channel_for_label(label)
                .and_then(|channel| projection.protected_resources.get(&channel))
                .is_some_and(|resources| resources.contains("forward") || resources.contains("backup"))
            { return Err(sharing_error("original_written_reuse_unresolved_cursor_channel")); }
        }
    }
    Ok(())
}

/// A different sufficient proof from whole-beat channel reuse. The sealed
/// source component supplies the notation equality; both exact projections
/// still decide physical conservation and every existing beam relation.
// Match the owner's exact source continuation before comparing representations.
fn bound_entry_rests(
    parsed: &FinalChordBindingSource<'_>,
    projected: &CanonicalProjectedFinalChord,
    document: &roxmltree::Document<'_>,
    raw_indices: &BTreeMap<usize, usize>,
    note_indices: &BTreeMap<usize, usize>,
) -> io::Result<BTreeMap<usize, BoundEntryRest>> {
    let source = parsed.xml;
    let mut by_offset = BTreeMap::new();
    for row in &projected.non_pitched_continuations {
        by_offset.entry(row.source_note_start).and_modify(|entry| *entry = None)
            .or_insert(Some(row));
    }
    let outputs = document.descendants().filter(|node| node.has_tag_name("note") || node.has_tag_name("forward"))
        .map(|node| (node.range().start, node)).collect::<BTreeMap<_, _>>();
    let mut result = BTreeMap::new();
    for measure in &parsed.measures {
        for pair in measure.notes.windows(2) {
            let [rest, head] = pair else { continue; };
            if !rest.rest_like || rest.chord_member || rest.onset != 0 || rest.duration <= 0
                || head.rest_like || head.chord_member || head.pitch_midi.is_none()
                || head.onset != rest.duration || head.staff != rest.staff || head.voice != rest.voice
                || !final_chord_note_fragment_is_visible(&source[rest.range.clone()])
                || !final_chord_note_fragment_is_visible(&source[head.range.clone()])
                || source[rest.range.end..head.range.start].contains("<backup")
            { continue; }
            let Some(Some(row)) = by_offset.get(&rest.range.start) else { continue; };
            if row.fragment_ordinal != 0 || row.source_note_end != rest.range.end { continue; }
            let Some(node) = outputs.get(&row.projected_start) else { continue; };
            let child = |name| final_chord_direct_child_text(*node, name);
            if child("duration").and_then(|d| d.parse::<i64>().ok()) != Some(rest.duration)
                || child("voice").as_deref() != Some(rest.voice.as_str())
                || child("staff").as_deref() != Some(rest.staff.as_str())
            { continue; }
            let note_index = match row.carrier {
                FinalChordNonPitchedCarrier::Note
                    if node.has_tag_name("note") && projected.xml.get(node.range()) == source.get(rest.range.clone()) =>
                    note_indices.get(&row.projected_start).copied(),
                FinalChordNonPitchedCarrier::Forward if node.has_tag_name("forward") => None,
                _ => continue,
            };
            let Some(&raw_index) = raw_indices.get(&row.projected_start) else { continue; };
            result.insert(rest.range.start, BoundEntryRest {
                interval: (rest.measure, rest.onset, rest.duration, rest.staff.clone()),
                label: rest.voice.clone(), raw_index, note_index,
            });
        }
    }
    Ok(result)
}

#[derive(Default)]
struct RestRestoration {
    before_raw: BTreeSet<usize>,
    after_raw: BTreeSet<usize>,
    after_notes: BTreeSet<usize>,
}

fn prove_entry_rest_restoration(
    before: &SharingProjection,
    after: &SharingProjection,
    sources: &BTreeSet<usize>,
) -> io::Result<RestRestoration> {
    let invalid = || sharing_error("original_component_rest_restoration_unproved");
    let mut proof = RestRestoration::default();
    for (origin, old) in &before.entry_rests {
        let Some(new) = after.entry_rests.get(origin) else { continue; };
        let (None, Some(index)) = (old.note_index, new.note_index) else { continue; };
        if old.interval != new.interval || old.label != new.label
            || after.non_pitched_note_positions.get(index) != Some(&new.interval)
        { continue; }
        if after.notes.values().any(|note| old.overlaps(note)) || !before.notes.values().any(|note|
            sources.contains(&note.source) && old.overlaps(note))
        { continue; }
        proof.before_raw.insert(old.raw_index);
        proof.after_raw.insert(new.raw_index);
        proof.after_notes.insert(index);
    }
    let same_raw = before.non_pitched.iter().enumerate().filter(|(i, _)| !proof.before_raw.contains(i))
        .map(|(_, raw)| raw).eq(after.non_pitched.iter().enumerate()
            .filter(|(i, _)| !proof.after_raw.contains(i)).map(|(_, raw)| raw));
    if proof.after_notes.is_empty() {
        return Err(sharing_error("original_component_no_entry_rest_restored"));
    }
    if !same_raw
        || !before.non_pitched_note_positions.iter().eq(after.non_pitched_note_positions.iter().enumerate()
            .filter(|(i, _)| !proof.after_notes.contains(i)).map(|(_, row)| row))
        || !before.non_pitched_shadow_roles.iter().eq(after.non_pitched_shadow_roles.iter().enumerate()
            .filter(|(i, _)| !proof.after_notes.contains(i)).map(|(_, row)| row))
    { return Err(invalid()); }
    Ok(proof)
}

pub(super) fn check_original_notation_component_projection(
    canonical: &CanonicalFinalChordStructuralPlan,
    production: &FrozenSerializedVoicePlan,
    proposed: &FrozenSerializedVoicePlan,
    original: &ActualSourceBeamBindingObservation,
    sources: &BTreeSet<usize>,
    reuse: &ProjectionReuse<'_>,
) -> io::Result<()> {
    verify_original_component_projection(canonical, production, proposed, original, sources, reuse, false)
}

pub(super) fn check_original_component_and_rest_projection(
    canonical: &CanonicalFinalChordStructuralPlan,
    production: &FrozenSerializedVoicePlan,
    proposed: &FrozenSerializedVoicePlan,
    original: &ActualSourceBeamBindingObservation,
    sources: &BTreeSet<usize>,
    reuse: &ProjectionReuse<'_>,
) -> io::Result<()> {
    verify_original_component_projection(canonical, production, proposed, original, sources, reuse, true)
}

fn verify_original_component_projection(
    canonical: &CanonicalFinalChordStructuralPlan,
    production: &FrozenSerializedVoicePlan,
    proposed: &FrozenSerializedVoicePlan,
    original: &ActualSourceBeamBindingObservation,
    sources: &BTreeSet<usize>,
    reuse: &ProjectionReuse<'_>,
    restore_entry_rest: bool,
) -> io::Result<()> {
    let mut attested = BTreeSet::new();
    for group in &canonical.original_beam_voice_groups {
        let members = group.allocation_sources();
        if members.is_empty() || !members.is_subset(sources) { continue; }
        let labels = members.iter().map(|source| proposed.label_for_source(*source))
            .collect::<BTreeSet<_>>();
        if labels.len() != 1 || labels.contains(&None) {
            return Err(sharing_error("original_component_not_assigned_as_one_line"));
        }
        attested.extend(members);
    }
    if &attested != sources {
        return Err(sharing_error("original_component_has_no_complete_sealed_notation_origin"));
    }
    let edges = ordered_received_beam_readback::owner_edges(canonical, original)
        .map_err(sharing_error)?;
    let project = |voices: FrozenSerializedVoicePlan| {
        if reuse.matches(canonical, original) { reuse.project(voices) }
        else { SharingContext::project_voices(canonical, voices, Some(&edges)).map(Rc::new) }
    };
    let before = project(production.clone())?;
    let after = project(proposed.clone())?;
    if before.notes.keys().ne(after.notes.keys()) {
        return Err(sharing_error("original_component_occurrence_resources_changed"));
    }
    let restored = if restore_entry_rest {
        prove_entry_rest_restoration(&before, &after, sources)?
    } else {
        if before.non_pitched != after.non_pitched
            || before.non_pitched_note_positions != after.non_pitched_note_positions
            || before.non_pitched_shadow_roles != after.non_pitched_shadow_roles
        { return Err(sharing_error("original_component_occurrence_or_rest_resources_changed")); }
        RestRestoration::default()
    };
    // Exact source rests are compatibility obligations, not an authority to
    // reconstruct musical line identity. Unknown source cursor use refuses.
    let mut rest_labels = Vec::new();
    for raw in &before.non_pitched {
        let doc = roxmltree::Document::parse(raw)
            .map_err(|_| sharing_error("original_component_unreadable_cursor_resource"))?;
        let node = doc.root_element();
        if !node.has_tag_name("note") { continue; }
        let label = match node.children().find(|child| child.has_tag_name("voice")) {
            Some(voice) => voice.text().map(str::trim).filter(|text| !text.is_empty())
                .ok_or_else(|| sharing_error("original_component_unresolved_rest_voice"))?,
            None => "1",
        };
        rest_labels.push(label.to_owned());
    }
    if rest_labels.len() != before.non_pitched_note_positions.len() {
        return Err(sharing_error("original_component_unresolved_rest_positions"));
    }
    let mut rest_intervals = BTreeMap::<(usize, String, String), Vec<(i64, i64)>>::new();
    for ((measure, onset, duration, staff), label) in before.non_pitched_note_positions.iter().zip(rest_labels) {
        let end = onset.checked_add(*duration).filter(|end| *end >= *onset)
            .ok_or_else(|| sharing_error("original_component_invalid_rest_interval"))?;
        rest_intervals.entry((*measure, staff.clone(), label)).or_default().push((*onset, end));
    }
    let mut changed = false;
    for (key, old) in &before.notes {
        let new = &after.notes[key];
        if old.source != new.source || old.attack != new.attack || old.measure != new.measure
            || old.staff != new.staff || old.onset != new.onset || old.duration != new.duration
            || old.beat_unit != new.beat_unit || old.chord_head != new.chord_head
            || old.non_pitched_chord_head != new.non_pitched_chord_head
            || old.non_voice_bytes != new.non_voice_bytes
        { return Err(sharing_error("original_component_non_voice_content_changed")); }
        let role_changed = production.source_to_role.get(&old.source) != proposed.source_to_role.get(&old.source)
            || production.role_for_label(&old.label) != proposed.role_for_label(&new.label);
        if old.label != new.label || role_changed {
            if !sources.contains(&old.source) || old.chord_head.is_some() || old.non_pitched_chord_head.is_some() {
                return Err(sharing_error("original_component_changed_unproved_source"));
            }
            let same_staff_role = |role: Option<&SerializedVoicePresentationRole>| match role {
                Some(SerializedVoicePresentationRole::UpperPrimary | SerializedVoicePresentationRole::UpperSecondary) => old.staff == "1",
                Some(SerializedVoicePresentationRole::LowerPrimary | SerializedVoicePresentationRole::LowerSecondary) => old.staff == "2",
                _ => false,
            };
            if !same_staff_role(production.source_to_role.get(&old.source))
                || !same_staff_role(proposed.source_to_role.get(&old.source))
            { return Err(sharing_error("original_component_changed_staff_or_auxiliary_role")); }
            if old.label != new.label {
                let end = new.onset.checked_add(new.duration)
                    .ok_or_else(|| sharing_error("original_component_invalid_note_interval"))?;
                if rest_intervals.get(&(new.measure, new.staff.clone(), new.label.clone()))
                    .is_some_and(|intervals| intervals.iter().any(|&(start, stop)| new.onset < stop && start < end))
                { return Err(sharing_error("original_component_source_rest_collision")); }
                for (projection, label, ignored) in [
                    (&before, &old.label, &restored.before_raw),
                    (&after, &new.label, &restored.after_raw),
                ] {
                    if projection.voices.channel_for_label(label).is_some_and(|channel|
                        projection.cursor_channels.iter().any(|(index, owner)|
                            channel == *owner && !ignored.contains(index)))
                    { return Err(sharing_error("original_component_unresolved_source_cursor_channel")); }
                }
            }
            let doc = roxmltree::Document::parse(&old.non_voice_bytes)
                .map_err(|_| sharing_error("original_component_unreadable_note"))?;
            if doc.descendants().any(|node| node.has_tag_name("tie") || node.has_tag_name("tied") || node.has_tag_name("grace")) {
                return Err(sharing_error("original_component_tie_or_grace_requires_further_proof"));
            }
            changed = true;
        }
    }
    let mut improved = false;
    let original_group = |level, key: &OccurrenceKey| {
        let witness = reuse.witness(key.0).ok()?;
        let occurrence = before.notes.get(key)?;
        if witness.kind != actual_source_beam_binding::SharingBeamKind::OwnedDecision
            || witness.source != occurrence.source || witness.attack != occurrence.attack
        { return None; }
        let (_, group, state) = witness.levels.iter().find(|(number, _, _)| *number == level)?;
        (state.as_ref() == occurrence.states.get(&level)).then_some((*group)?)
    };
    check_beam_transition_with_loss_proof(&before, &after,
        |(level, left, right)| {
            sources.contains(&before.notes[left].source)
                && sources.contains(&before.notes[right].source)
                && original_group(*level, left).is_some_and(|group|
                    original_group(*level, right) == Some(group))
        },
        |_, old, new| old == OwnerEdgeReadback::SplitVoice && new == OwnerEdgeReadback::Present,
        |(level, left, right)| {
            // A label-derived pair between two distinct, positively captured
            // original groups cannot represent an original beam obligation.
            (sources.contains(&before.notes[left].source) || sources.contains(&before.notes[right].source))
                && original_group(*level, left).zip(original_group(*level, right))
                    .is_some_and(|(a, b)| a != b)
        })
        .map_err(|error| sharing_error(error.reason()))?;
    for (old, new) in before.owner_edges.iter().zip(&after.owner_edges) {
        improved |= *old == OwnerEdgeReadback::SplitVoice && *new == OwnerEdgeReadback::Present;
    }
    if !changed || !improved { return Err(sharing_error("original_component_has_no_proved_improvement")); }
    Ok(())
}

#[derive(Default)]
struct OwnerBeatClosure {
    added_pairs: BTreeSet<BeamEdge>,
    moved: BTreeSet<OccurrenceKey>,
    improved_edges: BTreeSet<usize>,
}

type KeyedNote<'a> = (&'a OccurrenceKey, &'a SharingNote);

#[derive(Default)]
struct BeatClosureIndex<'a> {
    measure_context: BTreeMap<usize, Vec<KeyedNote<'a>>>,
    chord_members: BTreeMap<OccurrenceKey, Vec<KeyedNote<'a>>>,
}

impl<'a> BeatClosureIndex<'a> {
    fn new(before: &'a SharingProjection, after: &'a SharingProjection) -> Self {
        let mut index = Self::default();
        for (key, note) in before.notes.iter().chain(&after.notes) {
            index.measure_context.entry(note.measure).or_default().push((key, note));
        }
        for (key, note) in &after.notes {
            if let Some(head) = note.chord_head {
                index.chord_members.entry(head).or_default().push((key, note));
            }
        }
        index
    }
}

fn certify_closed_owner_beats(
    before: &SharingProjection, after: &SharingProjection,
    edges: &[OwnerEdge<'_>], original: &ActualSourceBeamBindingObservation,
) -> OwnerBeatClosure {
    let mut groups = BTreeMap::<_, Vec<_>>::new();
    for (index, edge) in edges.iter().enumerate() {
        groups.entry((edge.level, edge.group)).or_default().push((index, edge));
    }
    let mut emissions = BTreeMap::<_, Vec<_>>::new();
    for (key, note) in &after.notes {
        emissions.entry((note.source, note.attack, key.0)).or_default().push(*key);
    }
    let key_for = |row: &actual_source_beam_binding::BoundOccurrence| {
        let [key] = emissions.get(&(row.source, row.attack, row.fragment))?.as_slice() else { return None; };
        Some(*key)
    };
    let mut complete_by_measure = BTreeMap::<_, Vec<_>>::new();
    for ((level, _), group) in groups {
        let Some((_, first)) = group.first() else { continue; };
        if group.len() + 1 != first.group_size || group.iter().any(|(index, _)|
            after.owner_edges.get(*index) != Some(&OwnerEdgeReadback::Present))
        {
            continue;
        }
        let Some(heads) = std::iter::once(first.left)
            .chain(group.iter().map(|(_, edge)| edge.right)).map(key_for).collect::<Option<Vec<_>>>()
        else { continue; };
        if heads.iter().enumerate().any(|(position, key)| {
            let expected = if position == 0 { "begin" }
                else if position + 1 == heads.len() { "end" } else { "continue" };
            after.notes[key].states.get(&level).map(String::as_str) != Some(expected)
        }) { continue; }
        // A compatible subgroup must start at a head of the primary group.
        let measure = after.notes[&heads[0]].measure;
        complete_by_measure.entry(measure).or_default().push(
            (level, heads, group.into_iter().map(|(index, _)| index).collect::<Vec<_>>()));
    }
    let mut proof = OwnerBeatClosure::default();
    let index = std::cell::OnceCell::new();
    for complete in complete_by_measure.values() {
        for (_, heads, _) in complete.iter().filter(|(level, _, indices)| *level == 1
            && indices.iter().any(|&i| before.owner_edges.get(i) == Some(&OwnerEdgeReadback::SplitVoice)))
        {
            let index = index.get_or_init(|| BeatClosureIndex::new(before, after));
            let Some(members) = complete_owner_beat_members(before, after, heads, original, index) else { continue; };
            let mut primary_certified = false;
            for (level, level_heads, indices) in complete {
                if !level_heads.iter().all(|key| heads.contains(key)) { continue; }
                let notes = members.iter().filter(|key| {
                    after.notes.get(*key).is_some_and(|note| level_heads.contains(&note.chord_head.unwrap_or(**key)))
                }).copied().collect::<Vec<_>>();
                let pairs = notes.iter().enumerate().flat_map(|(index, &left)|
                    notes[index + 1..].iter().map(move |&right| (*level, left, right))).collect::<BTreeSet<_>>();
                if !pairs.is_subset(&after.edges) { continue; }
                primary_certified |= *level == 1 && level_heads == heads;
                proof.added_pairs.extend(pairs.difference(&before.edges).copied());
                proof.improved_edges.extend(indices.iter().copied().filter(|&i|
                    before.owner_edges.get(i) == Some(&OwnerEdgeReadback::SplitVoice)));
            }
            if primary_certified {
                proof.moved.extend(members.into_iter().filter(|key| before.notes.get(key)
                    .is_some_and(|note| note.label != after.notes[key].label)));
            }
        }
    }
    proof
}

fn complete_owner_beat_members(
    before: &SharingProjection, after: &SharingProjection,
    heads: &[OccurrenceKey], original: &ActualSourceBeamBindingObservation,
    index: &BeatClosureIndex<'_>,
) -> Option<BTreeSet<OccurrenceKey>> {
    let first = after.notes.get(heads.first()?)?;
    if first.beat_unit <= 0 || first.onset.rem_euclid(first.beat_unit) != 0 { return None; }
    let end = first.onset.checked_add(first.beat_unit)?;
    let mut cursor = first.onset;
    let mut labels = BTreeSet::new();
    let mut next_onset = BTreeMap::new();
    labels.insert(first.label.as_str());
    for key in heads {
        let new = after.notes.get(key)?;
        let old = before.notes.get(key)?;
        if new.measure != first.measure || new.staff != first.staff || new.label != first.label
            || new.beat_unit != first.beat_unit || new.onset != cursor || new.duration <= 0
            || new.duration >= first.beat_unit / 2 || !(2..=4).contains(&new.beam_levels)
            || new.chord_head.is_some() || new.non_pitched_chord_head.is_some()
            || next_onset.get(old.label.as_str()).is_some_and(|&next| next != new.onset)
        {
            return None;
        }
        cursor = cursor.checked_add(new.duration)?;
        next_onset.insert(old.label.as_str(), cursor);
        labels.insert(old.label.as_str());
    }
    if cursor != end { return None; }
    let mut members = heads.iter().copied().collect::<BTreeSet<_>>();
    for (&head_key, &(key, note)) in heads.iter().flat_map(|head|
        index.chord_members.get(head).into_iter().flatten().map(move |member| (head, member)))
    {
        let head = &after.notes[&head_key];
        let old = before.notes.get(key)?;
        if original.original_chord_head((note.source, note.attack, key.0)) != Some((head.source, head.attack, head_key.0))
            || note.measure != head.measure || note.staff != head.staff || note.onset != head.onset
            || note.duration != head.duration || !note.states.is_empty()
            || note.label != head.label || old.label != before.notes.get(&head_key)?.label
        {
            return None;
        }
        members.insert(*key);
    }
    // Both affected label strands must contain only the authenticated beat
    // material, including cross-staff conflicts and events entering the beat.
    for &(key, note) in &index.measure_context[&first.measure] {
        if labels.contains(note.label.as_str())
            && note.onset < end
            && (note.onset >= first.onset || note.onset.checked_add(note.duration)? > first.onset)
            && !members.contains(key)
        {
            return None;
        }
    }
    Some(members)
}

#[derive(Debug, PartialEq, Eq)]
enum BeamTransitionFailure {
    LostWrittenPair(BeamEdge),
    OrderedCoverage { before: usize, after: usize },
    LostOrderedEdge { index: usize, after: OwnerEdgeReadback },
    UnprovedOrderedChange { index: usize, before: OwnerEdgeReadback, after: OwnerEdgeReadback },
    UnprovedWrittenPair(BeamEdge),
}

impl BeamTransitionFailure {
    fn reason(&self) -> &'static str {
        match self {
            Self::LostWrittenPair(_) => "existing_written_beam_edge_lost",
            Self::OrderedCoverage { .. } => "ordered_owner_edge_coverage_changed",
            Self::LostOrderedEdge { .. } => "existing_ordered_owner_beam_edge_lost",
            Self::UnprovedOrderedChange { .. } => "ordered_owner_beam_change_unproved",
            Self::UnprovedWrittenPair(_) => "new_written_beam_edge_unproved",
        }
    }
}

fn check_beam_transition(
    before: &SharingProjection,
    after: &SharingProjection,
    prove_added_pair: impl Fn(&BeamEdge) -> bool,
    prove_ordered_change: impl Fn(usize, OwnerEdgeReadback, OwnerEdgeReadback) -> bool,
) -> Result<(), BeamTransitionFailure> {
    check_beam_transition_with_loss_proof(before, after, prove_added_pair, prove_ordered_change, |_| false)
}

fn check_beam_transition_with_loss_proof(
    before: &SharingProjection,
    after: &SharingProjection,
    prove_added_pair: impl Fn(&BeamEdge) -> bool,
    prove_ordered_change: impl Fn(usize, OwnerEdgeReadback, OwnerEdgeReadback) -> bool,
    disprove_lost_pair: impl Fn(&BeamEdge) -> bool,
) -> Result<(), BeamTransitionFailure> {
    if let Some(&pair) = before.edges.difference(&after.edges).find(|edge| !disprove_lost_pair(edge)) {
        return Err(BeamTransitionFailure::LostWrittenPair(pair));
    }
    if before.owner_edges.len() != after.owner_edges.len() {
        return Err(BeamTransitionFailure::OrderedCoverage {
            before: before.owner_edges.len(), after: after.owner_edges.len(),
        });
    }
    for (index, (&old, &new)) in before.owner_edges.iter().zip(&after.owner_edges).enumerate() {
        if old == new { continue; }
        if old == OwnerEdgeReadback::Present {
            return Err(BeamTransitionFailure::LostOrderedEdge { index, after: new });
        }
        if !prove_ordered_change(index, old, new) {
            return Err(BeamTransitionFailure::UnprovedOrderedChange { index, before: old, after: new });
        }
    }
    if let Some(&pair) = after.edges.difference(&before.edges).find(|edge| !prove_added_pair(edge)) {
        return Err(BeamTransitionFailure::UnprovedWrittenPair(pair));
    }
    Ok(())
}

fn validate_written_only_projections(
    before: &SharingProjection, after: &SharingProjection, closure: &OwnerBeatClosure,
    report: Option<&mut String>,
    original_non_beamable: impl Fn(usize, &SharingNote) -> bool,
) -> io::Result<()> {
    if before.voices.source_to_role != after.voices.source_to_role
        || before.non_pitched != after.non_pitched
        || before.non_pitched_note_positions != after.non_pitched_note_positions
        || before.non_pitched_shadow_roles != after.non_pitched_shadow_roles
        || before.notes.keys().ne(after.notes.keys())
    {
        return Err(sharing_error("written_identity_role_or_rest_resources_changed"));
    }
    let beam_transition = check_beam_transition(before, after, |edge| closure.added_pairs.contains(edge),
        |index, _, _| closure.improved_edges.contains(&index));
    if let Some(out) = report {
        let added = after.edges.difference(&before.edges);
        let (proved_pairs, unproved_pairs) = added.fold((0, 0), |(proved, unproved), pair|
            if closure.added_pairs.contains(pair) { (proved + 1, unproved) } else { (proved, unproved + 1) });
        let result = beam_transition.as_ref().err().map_or("preserved", BeamTransitionFailure::reason);
        let _ = writeln!(out,
            "WrittenOnlyBeamTransition: result={result}; lost_pairs={}; proved_added_pairs={proved_pairs}; unproved_added_pairs={unproved_pairs}; first_failure={:?}; scope=canonical_voice_encoding; authority=none",
            before.edges.difference(&after.edges).count(), beam_transition.as_ref().err());
    }
    beam_transition.map_err(|_| sharing_error("written_beam_connections_changed"))?;

    let mut changed_lanes = BTreeSet::new();
    for (key, old) in &before.notes {
        let new = &after.notes[key];
        let role = before.voices.role_for_label(&old.label);
        if old.source != new.source || old.attack != new.attack
            || old.measure != new.measure || old.staff != new.staff || old.onset != new.onset
            || old.beat_unit != new.beat_unit || old.duration != new.duration || old.chord_head != new.chord_head
            || old.non_pitched_chord_head != new.non_pitched_chord_head
            || old.non_voice_bytes != new.non_voice_bytes
            || role.is_none() || role != after.voices.role_for_label(&new.label)
        {
            return Err(sharing_error("written_occurrence_or_role_changed"));
        }
        if old.label != new.label {
            let note = roxmltree::Document::parse(&old.non_voice_bytes)
                .map_err(|_| sharing_error("unreadable_changed_written_occurrence"))?;
            if (old.chord_head.is_some() && !closure.moved.contains(key)) || old.non_pitched_chord_head.is_some()
                || note.descendants().any(|node| node.has_tag_name("tie") || node.has_tag_name("tied") || node.has_tag_name("grace"))
            {
                return Err(sharing_error("changed_chord_member_tie_or_grace_label"));
            }
            if !closure.moved.contains(key) {
                changed_lanes.insert((old.measure, old.label.as_str()));
                changed_lanes.insert((new.measure, new.label.as_str()));
            }
        }
    }
    // Optional metric bridges have no final lifecycle at this boundary.
    // Keep their entire potential lane input fixed, on both ends of every move.
    if before.notes.iter().chain(&after.notes).any(|(key, note)|
        changed_lanes.contains(&(note.measure, note.label.as_str()))
            && (note.beam_levels > 0 || !note.states.is_empty() || !original_non_beamable(key.0, note)))
    {
        return Err(sharing_error("changed_lane_has_beam_or_optional_bridge_input"));
    }
    for (key, old) in &before.notes {
        if let Some(head) = old.chord_head
            && before.notes[&head].label != after.notes[&head].label
            && !closure.moved.contains(key)
        {
            return Err(sharing_error("changed_chord_head_label"));
        }
    }
    Ok(())
}

fn append_voice_probe_comparison(
    out: &mut String,
    scope: u64,
    before: &SharingProjection,
    after: &SharingProjection,
) {
    let missing = before.notes.keys().filter(|key| !after.notes.contains_key(key)).count();
    let extra = after.notes.keys().filter(|key| !before.notes.contains_key(key)).count();
    let changed_non_voice = before.notes.iter().filter(|(key, note)|
        after.notes.get(key).is_some_and(|other|
            note.source != other.source || note.attack != other.attack
                || note.measure != other.measure || note.staff != other.staff
                || note.onset != other.onset || note.beat_unit != other.beat_unit
                || note.non_voice_bytes != other.non_voice_bytes
                || note.chord_head != other.chord_head
                || note.non_pitched_chord_head != other.non_pitched_chord_head)).count();
    let lost = before.edges.difference(&after.edges).count();
    let added = after.edges.difference(&before.edges).count();
    let _ = writeln!(out,
        "WrittenOnlyChannelProbeProjection: scope={scope:016x}; stage=canonical_voice_encoding; occurrences_before={} occurrences_after={} missing={missing} extra={extra} changed_non_voice={changed_non_voice}; non_pitched_equal={} non_pitched_positions_equal={} shadow_roles_equal={}; beam_edges_before={} beam_edges_after={} lost={lost} added={added}; downstream_render=unmeasured; authority=none",
        before.notes.len(), after.notes.len(), before.non_pitched == after.non_pitched,
        before.non_pitched_note_positions == after.non_pitched_note_positions,
        before.non_pitched_shadow_roles == after.non_pitched_shadow_roles,
        before.edges.len(), after.edges.len());
    for edge in before.edges.difference(&after.edges) {
        let left = &before.notes[&edge.1];
        let right = &before.notes[&edge.2];
        let row = format!(
            "WrittenOnlyChannelProbeLostEdge: scope={scope:016x}; level={} left={:?} right={:?} measure={} staff={} sources={:?}; authority=none",
            edge.0, edge.1, edge.2, left.measure, left.staff, (left.source, right.source));
        let _ = writeln!(out, "{row}");
        eprintln!("{row}");
    }
}

fn generated_alignment_staff(staff: Option<&str>, original: &BTreeSet<String>) -> bool {
    staff.map(str::trim).is_some_and(|staff| matches!(staff,"1"|"2") && !original.contains(staff))
}

fn original_cursor_staffs(source: &str) -> io::Result<BTreeSet<String>> {
    let document=roxmltree::Document::parse(source).map_err(|e|sharing_error(&e.to_string()))?;
    let mut staffs=BTreeSet::new();
    for cursor in document.descendants().filter(|n|n.has_tag_name("forward")) {
        match cursor.children().find(|n|n.has_tag_name("staff")).and_then(|n|n.text()).map(str::trim) {
            Some(staff @ ("1"|"2"))=>{staffs.insert(staff.to_string());},
            _=>{staffs.extend(["1".to_string(),"2".to_string()]);},
        }
    }
    Ok(staffs)
}

fn voice_neutral_bytes(raw: &str, node: roxmltree::Node<'_, '_>) -> String {
    // Read slices only; there is no temporary XML mutation or additional writer.
    let mut ranges=node.children().filter(|n|n.has_tag_name("voice")).map(|n|n.range()).collect::<Vec<_>>();
    ranges.sort_by_key(|r|r.start);
    ranges.iter().flat_map(|r|[r.start,r.end]).chain([raw.len()])
        .scan(0,|previous,boundary|{let range=*previous..boundary;*previous=boundary;Some(range)})
        .enumerate().filter(|(index,_)| index%2==0).map(|(_,range)|&raw[range]).collect()
}

fn sharing_error(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[derive(Debug, PartialEq, Eq)]
enum WrittenLossNecessity {
    SplitPhysicalHeads(BeamEdge),
    InterruptedPhysicalHeads { edge: BeamEdge, blocker: OccurrenceKey },
}

// Necessary condition only: ambiguous or equal-onset ordering, attached members
// and missing bindings fall through to the complete projector and verifier.
fn unavoidable_written_edge_loss(before: &SharingProjection, voices: &FrozenSerializedVoicePlan)
    -> Option<WrittenLossNecessity> {
    let mut heads = BTreeMap::<(usize, &str, &str), Vec<(OccurrenceKey, &SharingNote)>>::new();
    for (&key, note) in &before.notes {
        if note.chord_head.is_none() && note.non_pitched_chord_head.is_none()
            && let Some(label) = voices.label_for_source(note.source)
        { heads.entry((note.measure, note.staff.as_str(), label)).or_default().push((key, note)); }
    }
    for lane in heads.values_mut() { lane.sort_by_key(|(_, note)| note.onset); }
    for &(level, left, right) in &before.edges {
        let a = &before.notes[&left];
        let b = &before.notes[&right];
        if a.chord_head.is_some() || b.chord_head.is_some()
            || a.non_pitched_chord_head.is_some() || b.non_pitched_chord_head.is_some()
        { continue; }
        let (Some(a_label), Some(b_label)) = (voices.label_for_source(a.source), voices.label_for_source(b.source))
            else { continue; };
        if a_label != b_label { return Some(WrittenLossNecessity::SplitPhysicalHeads((level, left, right))); }
        let low = a.onset.min(b.onset);
        let high = a.onset.max(b.onset);
        if let Some(lane) = heads.get(&(a.measure, a.staff.as_str(), a_label)) {
            let start = lane.partition_point(|(_, note)| note.onset <= low);
            for &(key, note) in lane[start..].iter().take_while(|(_, note)| note.onset < high) {
                if note.states.get(&level).map(String::as_str) != Some("continue") {
                    return Some(WrittenLossNecessity::InterruptedPhysicalHeads { edge: (level, left, right), blocker: key });
                }
            }
        }
    }
    None
}

fn preserved_written_connections(
    before: &SharingProjection, voices: &FrozenSerializedVoicePlan,
) -> Option<BTreeSet<BeamEdge>> {
    let mut relabelled = before.notes.clone();
    for note in relabelled.values_mut() {
        note.label = voices.label_for_source(note.source)?.to_owned();
    }
    // Ordinal is not an independently preserved field. With equal head onsets
    // the projector may choose another order; that case needs the complete path.
    let mut head_onsets = BTreeSet::new();
    for note in relabelled.values() {
        if note.chord_head.and_then(|key| relabelled.get(&key))
            .is_some_and(|head| head.staff == note.staff && head.label == note.label)
        { continue; }
        if !head_onsets.insert((note.measure, note.staff.as_str(), note.label.as_str(), note.onset)) {
            return None;
        }
    }
    Some(written_connections(&relabelled))
}

fn first_unmoved_connection_change(
    before: &SharingProjection, expected: &BTreeSet<BeamEdge>, moved: &BTreeSet<usize>,
) -> Option<BeamEdge> {
    before.edges.symmetric_difference(expected).copied().find(|(_,left,right)|
        !moved.contains(&before.notes[left].source) && !moved.contains(&before.notes[right].source))
}

#[cfg(test)]
fn unavoidable_unmoved_connection_change(
    before: &SharingProjection, voices: &FrozenSerializedVoicePlan, moved: &BTreeSet<usize>,
) -> Option<BeamEdge> {
    first_unmoved_connection_change(before, &preserved_written_connections(before, voices)?, moved)
}

fn written_connections(notes: &BTreeMap<OccurrenceKey, SharingNote>) -> BTreeSet<BeamEdge> {
    let mut lanes = BTreeMap::<(usize, String, String), Vec<(OccurrenceKey, &SharingNote)>>::new();
    let mut members = BTreeMap::<OccurrenceKey, Vec<OccurrenceKey>>::new();
    for (key, note) in notes {
        if let Some(head) = note.chord_head
            && notes.get(&head).is_some_and(|container| container.staff == note.staff && container.label == note.label)
        {
            members.entry(head).or_default().push(*key);
        } else {
            lanes.entry((note.measure, note.staff.clone(), note.label.clone()))
                .or_default().push((*key, note));
        }
    }
    let mut out = BTreeSet::new();
    for lane in lanes.values_mut() {
        lane.sort_by_key(|(_, note)| (note.onset, note.ordinal));
        for level in 1..=4 {
            let mut chain: Option<Vec<OccurrenceKey>> = None;
            for (key, note) in lane.iter() {
                match note.states.get(&level).map(String::as_str) {
                    Some("begin") => chain = Some(vec![*key]),
                    Some("continue") => {
                        if let Some(open) = &mut chain { open.push(*key); }
                    }
                    Some("end") => {
                        if let Some(mut open) = chain.take() {
                            open.push(*key);
                            // Members share the physical head's connection, not
                            // its independently owned written state. No state is
                            // copied and no original cohort filters these edges.
                            let group = open.iter().flat_map(|head| std::iter::once(*head)
                                .chain(members.get(head).into_iter().flatten().copied())).collect::<Vec<_>>();
                            for (index, left) in group.iter().enumerate() {
                                for right in &group[index + 1..] {
                                    out.insert((level, (*left).min(*right), (*left).max(*right)));
                                }
                            }
                        }
                    }
                    // An interruption cancels an incomplete group. Neither an
                    // orphan continuation nor a dangling begin proves an edge.
                    _ => chain = None,
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_transition_preserves_ordered_edges_even_without_pair_delta() {
        let mut before = marked_fixture(&["begin", "continue", "end"], 17, 1);
        let mut after = marked_fixture(&["begin", "continue", "end"], 17, 1);
        before.owner_edges = vec![OwnerEdgeReadback::Present, OwnerEdgeReadback::SplitVoice];
        after.owner_edges = vec![OwnerEdgeReadback::Present, OwnerEdgeReadback::Present];
        let prove_ordered = |index, old, new| index == 1
            && old == OwnerEdgeReadback::SplitVoice && new == OwnerEdgeReadback::Present;
        assert!(check_beam_transition(&before, &after, |_| false, prove_ordered).is_ok());
        assert_eq!(check_beam_transition(&before, &after, |_| false, |_, _, _| false),
            Err(BeamTransitionFailure::UnprovedOrderedChange { index: 1, before: OwnerEdgeReadback::SplitVoice, after: OwnerEdgeReadback::Present }));
        after.owner_edges[0] = OwnerEdgeReadback::Interrupted { pitched_ordinal: 1 };
        assert_eq!(check_beam_transition(&before, &after, |_| true, |_, _, _| true),
            Err(BeamTransitionFailure::LostOrderedEdge { index: 0, after: OwnerEdgeReadback::Interrupted { pitched_ordinal: 1 } }));
    }

    #[test]
    fn beam_transition_reports_lost_and_unproved_pairs_without_changing_refusal() {
        let mut before = measure_context_fixture(&[]);
        let mut after = measure_context_fixture(&[]);
        let lost = (1, (17, 0, 0), (23, 0, 0));
        let proved = (1, (29, 0, 0), (31, 0, 0));
        let unproved = (2, (37, 0, 0), (41, 0, 0));
        before.edges.insert(lost);
        after.edges.extend([proved, unproved]);
        let closure = OwnerBeatClosure { added_pairs: BTreeSet::from([proved]), ..Default::default() };
        let mut report = String::new();
        let error = validate_written_only_projections(&before, &after, &closure, Some(&mut report), |_, _| true)
            .unwrap_err();
        assert_eq!(error.to_string(), "written_beam_connections_changed");
        assert!(report.contains("result=existing_written_beam_edge_lost; lost_pairs=1; proved_added_pairs=1; unproved_added_pairs=1"));
        assert!(report.contains(&format!("first_failure=Some(LostWrittenPair({lost:?}))")));

        after.edges.insert(lost);
        let mut report = String::new();
        let error = validate_written_only_projections(&before, &after, &closure, Some(&mut report), |_, _| true)
            .unwrap_err();
        assert_eq!(error.to_string(), "written_beam_connections_changed");
        assert!(report.contains("result=new_written_beam_edge_unproved; lost_pairs=0; proved_added_pairs=1; unproved_added_pairs=1"));
        assert!(report.contains(&format!("first_failure=Some(UnprovedWrittenPair({unproved:?}))")));
    }

    #[test]
    fn voice_probe_readback_detects_lost_edges_without_note_loss() {
        for offset in [17, 1017] {
            for copies in [1, 2] {
                let mut before = marked_fixture(&["begin", "continue", "end"], offset, copies);
                before.edges = written_connections(&before.notes);
                assert_eq!(before.edges.len(), 12 * copies);
                let mut after = marked_fixture(&["begin", "continue", "end"], offset, copies);
                for (key, note) in &mut after.notes {
                    note.label = format!("separate-{}", key.0);
                }
                after.edges = written_connections(&after.notes);
                let mut out = String::new();
                append_voice_probe_comparison(&mut out, 0, &before, &after);
                assert!(out.contains("missing=0 extra=0 changed_non_voice=0"));
                assert!(out.contains(&format!("lost={} added=0", before.edges.len())));
                assert_eq!(out.lines().filter(|line| line.starts_with("WrittenOnlyChannelProbeLostEdge:")).count(),
                    before.edges.len());
                out.clear();
                append_voice_probe_comparison(&mut out, 0, &before, &before);
                assert!(out.contains("lost=0 added=0"));
            }
        }
    }

    #[test]
    fn original_cursor_protection_is_scoped_to_its_known_staff_resource() {
        let source="<score><forward><duration>24</duration><voice>opaque</voice><staff>2</staff></forward></score>";
        let staffs=original_cursor_staffs(source).unwrap();
        assert!(!staffs.contains("1")); assert!(staffs.contains("2"));
        assert!(original_cursor_staffs("<score/>").unwrap().is_empty());
    }
    #[test]
    fn an_unbound_cursor_staff_protects_both_resources() {
        for staff in ["", "<staff>unknown</staff>"] {
            let source=format!("<score><forward><duration>24</duration>{staff}</forward></score>");
            assert_eq!(original_cursor_staffs(&source).unwrap(),["1".to_string(),"2".to_string()].into_iter().collect());
        }
        assert!(original_cursor_staffs("malformed").is_err());
    }
    #[test]
    fn generated_alignment_comparison_ignores_only_the_voice_scalar() {
        let before="<forward extra=\"preserved\"><duration>24</duration><voice>old</voice><staff>2</staff></forward>";
        let after=before.replace("old","new");
        let neutral=|raw:&str| {let d=roxmltree::Document::parse(raw).unwrap();voice_neutral_bytes(raw,d.root_element())};
        assert_eq!(neutral(before),neutral(&after));
        assert_ne!(neutral(before),neutral(&after.replace("24","48")));
        assert_ne!(neutral(before),neutral(&after.replace("<staff>2","<staff>1")));
        assert_ne!(neutral(before),neutral(&after.replace("preserved","changed")));
    }
    #[test]
    fn generated_cursor_admission_requires_a_known_unprotected_staff() {
        let empty=BTreeSet::new();
        assert!(generated_alignment_staff(Some(" 2 "),&empty));
        for staff in [None,Some("3"),Some("unknown")] {assert!(!generated_alignment_staff(staff,&empty));}
        let protected=original_cursor_staffs("<score><forward><duration>24</duration><staff> 2 </staff></forward></score>").unwrap();
        assert!(!generated_alignment_staff(Some(" 2 "),&protected));
        assert!(generated_alignment_staff(Some("1"),&protected));
        let unknown=original_cursor_staffs("<score><forward><duration>24</duration></forward></score>").unwrap();
        assert!(!generated_alignment_staff(Some(" 2 "),&unknown));
        assert!(!generated_alignment_staff(Some("1"),&unknown));
    }

    fn measure_context_fixture(rows: &[(usize,usize,&str,i64,&str)]) -> SharingProjection {
        let notes=rows.iter().enumerate().map(|(ordinal,(id,measure,staff,onset,label))|
            ((*id,0,0),SharingNote{source:*id,attack:*id+17,measure:*measure,staff:staff.to_string(),
                onset:*onset,duration:24,ordinal,beat_unit:96,beam_levels:2,label:label.to_string(),
                non_voice_bytes:"identical-note-bytes".into(),states:BTreeMap::new(),chord_head:None,non_pitched_chord_head:None})).collect();
        SharingProjection{voices:FrozenSerializedVoicePlan::default(),notes,non_pitched:Vec::new(),
            entry_rests:BTreeMap::new(),cursor_channels:Vec::new(),
            protected_channels:BTreeSet::new(),protected_resources:BTreeMap::new(),
            non_pitched_shadow_roles:Vec::new(),non_pitched_note_positions:Vec::new(),edges:BTreeSet::new(),owner_edges:Vec::new()}
    }
    #[test]
    fn restored_entry_rest_preserves_all_other_non_pitched_resources() {
        let mut before = measure_context_fixture(&[(17, 43, "1", 0, "a")]);
        let mut after = measure_context_fixture(&[(17, 43, "1", 0, "b")]);
        let rest = BoundEntryRest {
            interval: (43, 0, 24, "1".into()), label: "a".into(), raw_index: 0, note_index: None,
        };
        before.entry_rests.insert(91, rest.clone());
        after.entry_rests.insert(91, BoundEntryRest { note_index: Some(0), ..rest.clone() });
        before.non_pitched = vec!["original cursor".into(), "foreign cursor".into()];
        after.non_pitched = vec!["original rest".into(), "foreign cursor".into()];
        after.non_pitched_note_positions.push(rest.interval);
        after.non_pitched_shadow_roles.push(Some(true));
        assert!(prove_entry_rest_restoration(&before, &after, &BTreeSet::from([17])).is_ok());
        assert!(prove_entry_rest_restoration(&before, &after, &BTreeSet::from([29])).is_err());
        after.non_pitched[1] = "changed foreign cursor".into();
        assert!(prove_entry_rest_restoration(&before, &after, &BTreeSet::from([17])).is_err());
    }

    #[test]
    fn written_only_refuses_new_neighbour_in_beam_context_even_if_endpoints_stay() {
        let before_rows = [(17, 43, "1", 0, "a"), (29, 43, "1", 24, "b"), (41, 43, "1", 48, "a")];
        let after_rows = [(17, 43, "1", 0, "a"), (29, 43, "1", 24, "a"), (41, 43, "1", 48, "a")];
        let mut before = measure_context_fixture(&before_rows);
        let mut after = measure_context_fixture(&after_rows);
        for projection in [&mut before, &mut after] {
            projection.voices.label_to_role = ["a", "b"].into_iter()
                .map(|label| (label.to_owned(), SerializedVoicePresentationRole::UpperPrimary)).collect();
            for note in projection.notes.values_mut() {
                note.non_voice_bytes = "<note><duration>24</duration></note>".to_owned();
                note.beam_levels = 0;
            }
        }
        assert!(validate_written_only_projections(&before, &after, &OwnerBeatClosure::default(), None, |_, _| true).is_ok());
        for projection in [&mut before, &mut after] {
            projection.notes.get_mut(&(17, 0, 0)).unwrap().beam_levels = 1;
            projection.notes.get_mut(&(41, 0, 0)).unwrap().beam_levels = 1;
        }
        let error = validate_written_only_projections(&before, &after, &OwnerBeatClosure::default(), None, |_, _| true).unwrap_err();
        assert_eq!(error.to_string(), "changed_lane_has_beam_or_optional_bridge_input");
    }

    #[test]
    fn preservation_uses_complete_cross_staff_measure_input_and_ignores_independent_other_measures() {
        let before=measure_context_fixture(&[(17,3,"1",0,"a"),(29,3,"2",24,"a"),(41,8,"1",0,"b")]);
        let after=measure_context_fixture(&[(17,3,"1",0,"x"),(29,3,"2",24,"x"),(41,8,"1",0,"x")]);
        assert!(SharingContext::unchanged_measure_beam_context(&before,&after,&(17,0,0)));
        let renamed_before=measure_context_fixture(&[(1017,3,"1",0,"a"),(1029,3,"2",24,"a"),(1041,8,"1",0,"b")]);
        let renamed_after=measure_context_fixture(&[(1017,3,"1",0,"x"),(1029,3,"2",24,"x"),(1041,8,"1",0,"x")]);
        assert!(SharingContext::unchanged_measure_beam_context(&renamed_before,&renamed_after,&(1017,0,0)));
    }
    #[test]
    fn preservation_refuses_a_new_other_staff_partner_and_an_equal_geometry_foreign_identity() {
        let before=measure_context_fixture(&[(17,3,"1",0,"a"),(29,3,"2",24,"b")]);
        let after=measure_context_fixture(&[(17,3,"1",0,"x"),(29,3,"2",24,"x")]);
        assert!(!SharingContext::unchanged_measure_beam_context(&before,&after,&(17,0,0)));
        let before=measure_context_fixture(&[(17,3,"1",0,"a"),(29,3,"2",24,"a")]);
        let foreign=measure_context_fixture(&[(17,3,"1",0,"x"),(43,3,"2",24,"x")]);
        assert!(!SharingContext::unchanged_measure_beam_context(&before,&foreign,&(17,0,0)));
    }
    #[test]
    fn preservation_refuses_changed_order_staff_and_unknown_written_marks() {
        let before=measure_context_fixture(&[(17,3,"1",0,"a"),(29,3,"1",0,"a")]);
        let reordered=measure_context_fixture(&[(29,3,"1",0,"x"),(17,3,"1",0,"x")]);
        assert!(!SharingContext::unchanged_measure_beam_context(&before,&reordered,&(17,0,0)));
        let changed_staff=measure_context_fixture(&[(17,3,"1",0,"x"),(29,3,"2",0,"x")]);
        assert!(!SharingContext::unchanged_measure_beam_context(&before,&changed_staff,&(17,0,0)));
        let mut marked=measure_context_fixture(&[(17,3,"1",0,"x"),(29,3,"1",0,"x")]);
        marked.notes.get_mut(&(17,0,0)).unwrap().states.insert(1,"begin".to_string());
        assert!(!SharingContext::unchanged_measure_beam_context(&before,&marked,&(17,0,0)));
    }
    #[test]
    fn preservation_refuses_unsupported_note_applicability() {
        let before=measure_context_fixture(&[(17,3,"1",0,"a")]);
        let mut after=measure_context_fixture(&[(17,3,"1",0,"x")]);
        after.notes.get_mut(&(17,0,0)).unwrap().beam_levels=5;
        assert!(!SharingContext::unchanged_measure_beam_context(&before,&after,&(17,0,0)));
    }

    #[test]
    fn indexed_measure_components_equal_fixed_point_for_every_guest_subset() {
        let projection = measure_context_fixture(&[
            (17, 3, "1", 0, "a"), (29, 3, "2", 24, "a"),
            (31, 8, "1", 0, "a"), (41, 8, "2", 24, "a"),
            (53, 11, "1", 0, "a"), (67, 12, "2", 0, "a"),
            (71, 12, "1", 24, "a"),
        ]);
        let classes = BTreeMap::from([(17,101),(29,202),(31,202),(41,303),
            (53,404),(67,505),(71,606)]);
        let index = MeasureIncidence::new(&projection, &classes);
        let universe = [101,202,303,404,505,606,707]; // includes an unrepresented class
        for mask in 0..(1 << universe.len()) {
            let mut members = universe.iter().enumerate().filter_map(|(i,c)|
                ((mask & (1 << i)) != 0).then_some(*c)).collect::<Vec<_>>();
            for _ in 0..2 {
                assert_eq!(index.groups(&members), SharingContext::evacuation_groups(
                    &projection, &members, &classes));
                members.reverse();
            }
            if let Some(first) = members.first().copied() { members.push(first); }
            assert_eq!(index.groups(&members), SharingContext::evacuation_groups(
                &projection, &members, &classes));
        }
    }

    #[test]
    fn evacuation_components_keep_cross_staff_measure_and_cross_measure_class_closure() {
        let projection=measure_context_fixture(&[(17,3,"1",0,"a"),(29,3,"2",24,"a"),
            (31,8,"1",0,"a"),(41,8,"2",24,"a"),(53,11,"1",0,"a")]);
        let classes=BTreeMap::from([(17,101),(29,202),(31,202),(41,303),(53,404)]);
        let members=[101,202,303,404];
        assert_eq!(SharingContext::evacuation_groups(&projection,&members,&classes),vec![vec![101,202,303],vec![404]]);
        let renamed=measure_context_fixture(&[(117,23,"1",0,"a"),(129,23,"2",24,"a"),
            (131,28,"1",0,"a"),(141,28,"2",24,"a"),(153,31,"1",0,"a")]);
        let renamed_classes=classes.iter().map(|(s,c)|(*s+100,*c+700)).collect();
        assert_eq!(SharingContext::evacuation_groups(&renamed,&[801,902,1003,1104],&renamed_classes),vec![vec![801,902,1003],vec![1104]]);
    }


    fn continuation_fixture(offset: usize, copies: usize) -> (FrozenSerializationVoiceChannelPlan, Vec<LegacyVoiceLabelPreferenceRow>) {
        let roles = [(17, SerializedVoicePresentationRole::UpperSecondary),
            (29, SerializedVoicePresentationRole::UpperSecondary),
            (41, SerializedVoicePresentationRole::UpperAuxiliary)];
        let mut sources = BTreeMap::from([(offset+101, offset+17), (offset+103, offset+29)]);
        for index in 0..copies { sources.insert(offset+200+index, offset+41); }
        let channel_roles = roles.into_iter().map(|(id,role)|(id+offset,role)).collect::<BTreeMap<_,_>>();
        let source_roles = sources.iter().map(|(&source,ch)|(source,channel_roles[ch])).collect();
        let legacy = sources.iter().map(|(&source,&channel)|LegacyVoiceLabelPreferenceRow {
            source_event_id:source, label:if channel==offset+29 {"61".into()} else if channel==offset+41 {"79".into()} else {"89".into()},
        }).collect();
        (FrozenSerializationVoiceChannelPlan { authority_bound:true, source_to_channel:sources,
            source_to_role:source_roles, channel_to_role:channel_roles, channel_count:3,
            semantic_fingerprint_fnv64:91, consumer_allocation:ChannelConsumerAllocation::Undetermined("synthetic"),
            label_continuation:None }, legacy)
    }

    fn evacuate_fixture(before: &FrozenSerializationVoiceChannelPlan, legacy: &[LegacyVoiceLabelPreferenceRow],
        guest: usize, host: usize) -> FrozenSerializationVoiceChannelPlan {
        let mut after=before.clone();
        after.label_continuation=Some(bind_musicxml_voice_labels(before,legacy).unwrap());
        for channel in after.source_to_channel.values_mut() { if *channel==guest {*channel=host;} }
        after.channel_to_role.remove(&guest);after.channel_count-=1;
        after
    }

    #[test]
    fn a_large_guest_majority_cannot_rename_the_bound_surviving_host() {
        for offset in [0,10000] {for copies in [3,9] {
            let (before,legacy)=continuation_fixture(offset,copies);
            let baseline=bind_musicxml_voice_labels(&before,&legacy).unwrap();
            let after=evacuate_fixture(&before,&legacy,offset+41,offset+29);
            let mut unbound=after.clone();unbound.label_continuation=None;
            let revoted=bind_musicxml_voice_labels(&unbound,&legacy).unwrap();
            assert_ne!(revoted.label_for_source(offset+103),baseline.label_for_source(offset+103));
            let continued=bind_musicxml_voice_labels(&after,&legacy).unwrap();
            for source in [offset+101,offset+103] {
                assert_eq!(continued.label_for_source(source),baseline.label_for_source(source));
            }
            assert_eq!(continued.source_to_role,baseline.source_to_role);
            assert_eq!(continued.label_for_source(offset+200),baseline.label_for_source(offset+103));
            assert_eq!(continued.channel_to_label.len(),2);
            // The subsequent productive binder must ignore a fresh legacy vote.
            let changed=legacy.iter().cloned().map(|mut row|{row.label="991".into();row}).collect::<Vec<_>>();
            assert_eq!(bind_musicxml_voice_labels(&after,&changed).unwrap().source_to_label,continued.source_to_label);
        }}
    }

    #[test]
    fn vacated_canonical_labels_are_not_reassigned_to_surviving_channels() {
        let (before,legacy)=continuation_fixture(0,3);
        let baseline=bind_musicxml_voice_labels(&before,&legacy).unwrap();
        let after=evacuate_fixture(&before,&legacy,17,29);
        let continued=bind_musicxml_voice_labels(&after,&legacy).unwrap();
        assert_eq!(continued.channel_to_label[&29],baseline.channel_to_label[&29]);
        assert!(!continued.label_to_role.contains_key(&baseline.channel_to_label[&17]));
    }

    #[test]
    fn continuation_refuses_foreign_missing_or_internally_inconsistent_bindings() {
        let (before,legacy)=continuation_fixture(0,3);
        let after=evacuate_fixture(&before,&legacy,41,29);
        for kind in 0..8 {
            let mut broken=after.clone();
            let sealed=broken.label_continuation.as_mut().unwrap();
            match kind {
                0=>sealed.authority_bound=false,
                1=>{sealed.source_to_role.remove(&101);},
                2=>{sealed.source_to_label.remove(&101);},
                3=>{sealed.source_to_channel.insert(9001,17);},
                4=>{sealed.channel_to_label.insert(29,sealed.channel_to_label[&17].clone());},
                5=>{sealed.label_to_role.clear();},
                6=>{broken.channel_to_role.insert(29,SerializedVoicePresentationRole::UpperAuxiliary);},
                _=>{broken.source_to_channel.insert(101,29);},
            }
            assert!(bind_musicxml_voice_labels(&broken,&legacy).is_err(),"case {kind}");
        }
    }

    #[test]
    fn continuation_does_not_create_a_new_channel_or_drop_source_roles() {
        let (before,legacy)=continuation_fixture(0,3);
        let after=evacuate_fixture(&before,&legacy,41,29);
        let mut new_channel=after.clone();
        new_channel.channel_to_role.insert(71,SerializedVoicePresentationRole::UpperAuxiliary);
        new_channel.channel_count+=1;new_channel.source_to_channel.insert(200,71);
        assert!(bind_musicxml_voice_labels(&new_channel,&legacy).is_err());
        let mut missing=after;missing.source_to_role.remove(&200);
        assert!(bind_musicxml_voice_labels(&missing,&legacy).is_err());
    }

    #[test]
    fn the_consumer_uses_actual_supplemental_and_vacated_label_owners() {
        let (before,legacy)=continuation_fixture(0,3);
        let baseline=bind_musicxml_voice_labels(&before,&legacy).unwrap();
        let after=evacuate_fixture(&before,&legacy,17,29);
        let voices=bind_musicxml_voice_labels(&after,&legacy).unwrap();
        let mut notes=vec![ChannelConsumerNote { staff:1,chordrest:true,required:true,class:Some(103),label:"61".into() }];
        let partition=BTreeMap::from([(103,29)]);
        let role=BTreeMap::from([(29,SerializedVoicePresentationRole::UpperSecondary)]);
        for label in [&baseline.channel_to_label[&17],&baseline.channel_to_label[&41],&"83".to_string()] {
            notes.push(ChannelConsumerNote {staff:1,chordrest:true,required:false,class:None,label:label.clone()});
        }
        // The host's actual label aliases its unchanged rest, while its old
        // vacated canonical owner remains a separate voice.
        notes.push(ChannelConsumerNote {staff:1,chordrest:true,required:false,class:None,label:voices.channel_to_label[&29].clone()});
        assert!(channel_consumer_overflow_with_labels(&ChannelConsumerModel{notes:notes.clone()},&partition,&role,Some(&voices)).is_ok());
        notes.push(ChannelConsumerNote {staff:1,chordrest:true,required:false,class:None,label:"97".into()});
        assert!(channel_consumer_overflow_with_labels(&ChannelConsumerModel{notes},&partition,&role,Some(&voices)).is_err());
    }


    #[test]
    fn edge_loss_observation_lists_each_edge_and_complete_ordered_lane_without_granting_witnesses() {
        for offset in [0,10000] {
            let rows=[(offset+17,3,"1",0,"a"),(offset+29,3,"1",48,"a"),(offset+41,3,"1",24,"b")];
            let mut before=measure_context_fixture(&rows);let mut after=measure_context_fixture(&rows);
            for projection in [&mut before,&mut after] {
                projection.notes.get_mut(&(offset+17,0,0)).unwrap().states.insert(1,"begin".into());
                projection.notes.get_mut(&(offset+29,0,0)).unwrap().states.insert(1,"end".into());
            }
            after.notes.get_mut(&(offset+41,0,0)).unwrap().label="a".into();
            before.edges=written_connections(&before.notes);after.edges=written_connections(&after.notes);
            assert_eq!(before.edges.len(),1);assert!(after.edges.is_empty());
            let canonical=CanonicalFinalChordStructuralPlan::default();
            let observation=ActualSourceBeamBindingObservation::default();
            let classes=rows.iter().map(|(id,..)|(*id,*id)).collect::<BTreeMap<_,_>>();
            let initial=BTreeMap::from([(offset+17,17),(offset+29,17),(offset+41,29)]);
            let proposed=classes.keys().map(|id|(*id,17)).collect::<BTreeMap<_,_>>();
            let roles=Roles::from([(17,SerializedVoicePresentationRole::UpperSecondary)]);
            let source_roles=classes.keys().map(|id|(*id,SerializedVoicePresentationRole::UpperSecondary)).collect();
            let context=SharingContext::new(&canonical,&observation,&classes,&source_roles,&[]);
            let lines=context.beam_edge_loss_lines(&before,&after,&initial,&proposed,&roles);
            assert_eq!(lines.iter().filter(|line|line.contains("phase=written_edge_loss_edge;")).count(),1);
            assert_eq!(lines.iter().filter(|line|line.contains("phase=written_edge_loss_lane;")).count(),2);
            let proposed_notes=lines.iter().filter(|line|line.contains("phase=written_edge_loss_note; projection=proposed;")).collect::<Vec<_>>();
            assert_eq!(proposed_notes.len(),3);
            for (line,id) in proposed_notes.iter().zip([offset+17,offset+41,offset+29]) {
                assert!(line.contains(&format!("occurrence=({id}, 0, 0)")));
                assert!(line.contains("original_witness=Err("));
            }
            assert!(lines[0].contains("lost_edges=1; baseline_edges=1; proposed_edges=0;"));
            assert_eq!(context.diagnostic_count("existing_written_beam_edge_lost"),0);
            assert_eq!(before.edges.len(),1);assert!(after.edges.is_empty());
        }
    }

    #[test]
    fn edge_loss_observation_keeps_all_levels_and_all_independent_lanes() {
        let rows=[(17,3,"1",0,"a"),(29,3,"1",24,"a"),(41,8,"2",0,"b"),(53,8,"2",24,"b")];
        let mut before=measure_context_fixture(&rows);let after=measure_context_fixture(&rows);
        before.edges=BTreeSet::from([(1,(17,0,0),(29,0,0)),(2,(17,0,0),(29,0,0)),(1,(41,0,0),(53,0,0))]);
        let canonical=CanonicalFinalChordStructuralPlan::default();let observation=ActualSourceBeamBindingObservation::default();
        let classes=rows.iter().map(|(id,..)|(*id,*id)).collect::<BTreeMap<_,_>>();
        let roles=Roles::new();let source_roles=Roles::new();
        let context=SharingContext::new(&canonical,&observation,&classes,&source_roles,&[]);
        let lines=context.beam_edge_loss_lines(&before,&after,&classes,&classes,&roles);
        assert_eq!(lines.iter().filter(|line|line.contains("phase=written_edge_loss_edge;")).count(),3);
        // A lane is printed once per projection, not once per edge or level.
        assert_eq!(lines.iter().filter(|line|line.contains("phase=written_edge_loss_lane;")).count(),4);
        assert_eq!(lines.iter().filter(|line|line.contains("phase=written_edge_loss_note;")).count(),8);
        assert!(lines[0].contains("lost_edges=3;"));
        assert_eq!(lines,context.beam_edge_loss_lines(&before,&after,&classes,&classes,&roles));
    }

    fn marked_fixture(marks: &[&str], offset: usize, copies: usize) -> SharingProjection {
        let rows = (0..copies).flat_map(|copy| marks.iter().enumerate().map(move |(index, _)|
            (offset + copy * 100 + index, offset + copy, "1", index as i64 * 120, "a"))).collect::<Vec<_>>();
        let mut projection = measure_context_fixture(&rows);
        for (index, note) in projection.notes.values_mut().enumerate() {
            for level in 1..=4 { note.states.insert(level, marks[index % marks.len()].into()); }
        }
        projection
    }

    #[test]
    fn written_groups_require_an_explicit_begin_and_end_at_every_level() {
        for offset in [17, 10017] { for copies in [1, 2] {
            for marks in [vec!["continue", "continue", "end"], vec!["begin", "continue"],
                vec!["begin", "continue", "none", "end"], vec!["begin", "continue", "forward hook", "end"]] {
                let projection = marked_fixture(&marks, offset, copies);
                assert!(written_connections(&projection.notes).is_empty());
            }
            let projection = marked_fixture(&["begin", "continue", "end"], offset, copies);
            assert_eq!(written_connections(&projection.notes).len(), 12 * copies);
            // Onsets span several beats: no beat or original cohort ID cuts a real group.
            let edges = written_connections(&projection.notes);
            for level in 1..=4 { assert!(edges.contains(&(level, (offset,0,0), (offset+2,0,0)))); }
        }}
    }

    #[test]
    fn a_replacement_begin_discards_only_the_incomplete_previous_group() {
        let projection = marked_fixture(&["begin", "continue", "begin", "end"], 17, 1);
        let expected = (1..=4).map(|level| (level, (19,0,0), (20,0,0))).collect::<BTreeSet<_>>();
        assert_eq!(written_connections(&projection.notes), expected);
        let orphan_then_closed = marked_fixture(&["continue", "end", "begin", "end"], 17, 1);
        assert_eq!(written_connections(&orphan_then_closed.notes), expected);
    }

    #[test]
    fn a_bound_unmarked_member_shares_connections_without_interrupting_or_owning_states() {
        for offset in [17, 10017] { for copies in [1, 2] {
            let mut projection = marked_fixture(&["begin", "continue", "member", "end"], offset, copies);
            for copy in 0..copies {
                let key = (offset + copy * 100 + 2,0,0);
                let member = projection.notes.get_mut(&key).unwrap();
                member.chord_head = Some((offset + copy * 100 + 1,0,0));
                member.states.clear(); member.onset = 120;
            }
            let edges = written_connections(&projection.notes);
            assert_eq!(edges.len(), 24 * copies);
            for level in 1..=4 { assert!(edges.contains(&(level,(offset,0,0),(offset+3,0,0)))); }
            assert!(projection.notes[&(offset+2,0,0)].states.is_empty());
            // An independent unmarked pitched head really interrupts the group.
            for note in projection.notes.values_mut() { note.chord_head = None; }
            assert!(written_connections(&projection.notes).is_empty());
        }}
    }

    fn producer_group_fixture(two_groups: bool, offset: usize) -> (CanonicalFinalChordStructuralPlan, ActualSourceBeamBindingObservation) {
        let mut source = crate::pipeline::actual_beam_owner_capture_tests::input();
        if two_groups {
            source = source.replace("<duration>288</duration>", "<duration>192</duration>").replace("<dot/>", "");
            let extra = ["G", "A", "B", "C"].into_iter().map(|step| format!(
                "<note><pitch><step>{step}</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff></note>"
            )).collect::<String>();
            source = source.replace("</measure>", &format!("{extra}</measure>"));
        } else {
            let second = source.find("<step>D</step>").unwrap();
            let end = second + source[second..].find("</note>").unwrap() + "</note>".len();
            source = format!("{}<note><chord/><pitch><step>A</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff></note>{}", &source[..end], &source[end..]);
        }
        source = source.replace("probe", &offset.to_string());
        let (xml, _, capture) = crate::pipeline::actual_beam_owner_capture_tests::execute(&source);
        let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical, _) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
        let observation = actual_source_beam_binding::observe_actual_source_beam_binding(&canonical, &xml, Some(&Ok(capture)));
        (canonical, observation)
    }

    #[test]
    fn written_loss_preflight_is_a_necessary_condition_under_all_label_assignments() {
        let mut checked = 0;
        let mut rejected = 0;
        for offset in [59, 1059] { for copies in [1, 2] {
            for mark in ["begin", "continue", "end", "none", "forward hook"] {
                for onset in [0, 60, 120, 180, 240, 300] {
                    for attached in [false, true] {
                        let mut before = marked_fixture(&["begin", "continue", "end", mark], offset, copies);
                        for copy in 0..copies {
                            let blocker = before.notes.get_mut(&(offset + copy * 100 + 3, 0, 0)).unwrap();
                            blocker.label = "b".into(); blocker.onset = onset;
                            if attached { blocker.chord_head = Some((offset + copy * 100 + 1, 0, 0)); }
                        }
                        before.edges = written_connections(&before.notes);
                        for assignment in 0..16 {
                            let mut after = before.clone();
                            let mut voices = FrozenSerializedVoicePlan::default();
                            for (&key, note) in &mut after.notes {
                                let index = (key.0 - offset) % 100;
                                note.label = if assignment & (1 << index) == 0 { "a" } else { "b" }.into();
                                voices.source_to_label.insert(note.source, note.label.clone());
                            }
                            let actual = written_connections(&after.notes);
                            if unavoidable_written_edge_loss(&before, &voices).is_some() {
                                assert!(!before.edges.is_subset(&actual));
                                rejected += 1;
                            }
                            if before.edges.is_subset(&actual) {
                                assert!(unavoidable_written_edge_loss(&before, &voices).is_none());
                            }
                            checked += 1;
                        }
                    }
                }
            }
        }}
        assert!(rejected > 0 && rejected < checked);
        eprintln!("WrittenLossPreflightSynthetic: assignments={checked}; proved_refusals={rejected}; false_refusals=0; authority=none");
    }

    #[test]
    fn unmoved_connection_preflight_is_necessary_under_every_preserved_ordinal_order() {
        let mut certificates = 0;
        let mut added_certificates = 0;
        for offset in [17, 1017] {
            for marks in [&["begin", "continue", "end", "begin", "end"][..], &["begin", "none", "end"][..]] {
                let mut before = marked_fixture(marks, offset, 1);
                before.edges = written_connections(&before.notes);
                let keys = before.notes.keys().copied().collect::<Vec<_>>();
                for mask in 0..(1 << keys.len()) { for moved_mask in 0..(1 << keys.len()) {
                    let mut voices = before.voices.clone();
                    let mut moved = BTreeSet::new();
                    let mut after = before.clone();
                    for (index, key) in keys.iter().enumerate() {
                        let label = if mask & (1 << index) == 0 { "left" } else { "right" };
                        voices.source_to_label.insert(before.notes[key].source, label.into());
                        after.notes.get_mut(key).unwrap().label = label.into();
                        if moved_mask & (1 << index) != 0 { moved.insert(before.notes[key].source); }
                    }
                    let proof = unavoidable_unmoved_connection_change(&before, &voices, &moved);
                    certificates += usize::from(proof.is_some());
                    for reverse in [false, true] {
                        for (index, key) in keys.iter().enumerate() {
                            after.notes.get_mut(key).unwrap().ordinal = if reverse { keys.len()-index } else { index };
                        }
                        after.edges = written_connections(&after.notes);
                        let unchanged_pair = |(_,left,right): &BeamEdge|
                            !moved.contains(&before.notes[left].source) && !moved.contains(&before.notes[right].source);
                        let changed = before.edges.symmetric_difference(&after.edges).any(unchanged_pair);
                        assert_eq!(proof.is_some(), changed);
                        added_certificates += usize::from(after.edges.difference(&before.edges).any(unchanged_pair));
                    }
                    // Equal physical-head onsets cannot supply a certificate.
                    let mut ambiguous = before.clone();
                    for note in ambiguous.notes.values_mut() { note.onset = 0; }
                    for key in &keys { voices.source_to_label.insert(before.notes[key].source, "one".into()); }
                    assert!(unavoidable_unmoved_connection_change(&ambiguous, &voices, &moved).is_none());
                }}
            }
        }
        assert!(certificates > 0);
        assert!(added_certificates > 0);
    }

    #[test]
    fn exact_projection_reuse_preserves_cold_results_and_rejects_foreign_scope() {
        for offset in [43, 1043] {
            let (canonical, observation) = producer_group_fixture(true, offset);
            let attacks = canonical.source_locators.iter().map(|row| FrozenVoiceAttackEvidenceRow {
                source_event_id: row.source_event_id, attack_id: row.attack_id,
                absolute_start: row.onset, absolute_end: row.onset + row.duration,
                staff: row.staff.parse().unwrap(), technical_lane: 1,
            }).collect::<Vec<_>>();
            let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
            let voices = bind_musicxml_voice_labels(&channels, &[]).unwrap();
            let edges = ordered_received_beam_readback::owner_edges(&canonical, &observation).unwrap();
            let reuse = ProjectionReuse::new(&canonical, &observation);
            let cold = SharingContext::project_voices(&canonical, voices.clone(), Some(&edges)).unwrap();
            let start = std::time::Instant::now();
            for _ in 0..16 {
                assert_eq!(*reuse.project(voices.clone()).unwrap(), cold);
            }
            let warm_elapsed = start.elapsed();
            let parsed_source = reuse.binding_source.get().unwrap() as *const FinalChordBindingSource<'_>;
            assert_eq!(reuse.projection_misses.get(), 1);
            assert_eq!(reuse.projection_hits.get(), 15);
            // Equal diagnostic fingerprints do not permit reuse of a different
            // bound label assignment, or even a changed metadata field.
            let names = voices.channel_to_label.values().map(|name| format!("renamed-{name}")).collect::<Vec<_>>();
            let alpha = final_chord_permute_sealed_voice_labels(&voices, &names).unwrap();
            let alpha_cold = SharingContext::project_voices(&canonical, alpha.clone(), Some(&edges)).unwrap();
            assert_eq!(*reuse.project(alpha).unwrap(), alpha_cold);
            let mut changed_metadata = voices.clone();
            changed_metadata.semantic_fingerprint_fnv64 ^= 1;
            let _ = reuse.project(changed_metadata);
            assert_eq!(reuse.projection_misses.get(), 3);
            assert_eq!(reuse.binding_source.get().unwrap() as *const FinalChordBindingSource<'_>, parsed_source);
            // Every field participates in the structural key, including the
            // separately sealed canonical role allocation, never just labels.
            let mut mutations = Vec::new();
            let mut changed = voices.clone(); changed.authority_bound = !changed.authority_bound; mutations.push(changed);
            let mut changed = voices.clone(); changed.source_to_role.clear(); mutations.push(changed);
            let mut changed = voices.clone(); changed.source_to_channel.clear(); mutations.push(changed);
            let mut changed = voices.clone(); changed.source_to_label.clear(); mutations.push(changed);
            let mut changed = voices.clone(); changed.label_to_role.clear(); mutations.push(changed);
            let mut changed = voices.clone(); changed.channel_to_label.clear(); mutations.push(changed);
            let mut changed = voices.clone(); changed.channel_to_role.clear(); mutations.push(changed);
            let mut changed = voices.clone(); changed.canonical_role_allocation = None; mutations.push(changed);
            for changed in mutations { assert_ne!(voices, changed); }
            let fragment = canonical.source_locators[0].fragment_id;
            let witness = observation.sharing_witness_diagnostic(&canonical, fragment).unwrap();
            for _ in 0..16 {
                let reused = reuse.witness(fragment).unwrap();
                assert_eq!((reused.source, reused.attack, reused.levels), (witness.source, witness.attack, witness.levels));
                assert!(reused.kind == witness.kind);
            }
            for _ in 0..2 { assert!(reuse.witness(usize::MAX).is_err()); }
            assert_eq!(reuse.witness_misses.get(), 2);
            assert_eq!(reuse.witness_hits.get(), 16);
            let other_canonical = canonical.clone();
            assert!(!reuse.matches(&other_canonical, &observation));
            let missing = ActualSourceBeamBindingObservation::default();
            assert!(!reuse.matches(&canonical, &missing));
            let classes = attacks.iter().map(|a| (a.source_event_id, a.source_event_id)).collect();
            let context = SharingContext::new(&canonical, &missing, &classes, &voices.source_to_role, &[])
                .with_reuse(Some(&reuse));
            assert!(context.shared_reuse.is_none());
            assert!(context.row(fragment).is_none());
            let start = std::time::Instant::now();
            for _ in 0..16 {
                assert_eq!(SharingContext::project_voices(&canonical, voices.clone(), Some(&edges)).unwrap(), cold);
            }
            eprintln!("ProjectionReuseSyntheticBenchmark: scope={offset}; repeated=16; cold_us={}; warm_us={}; actual_projections=1; authority=none",
                start.elapsed().as_micros(), warm_elapsed.as_micros());
            for index in 0..=ProjectionReuse::RETAINED_PROJECTIONS {
                let names = voices.channel_to_label.values().map(|name| format!("eviction-{index}-{name}")).collect::<Vec<_>>();
                let renamed = final_chord_permute_sealed_voice_labels(&voices, &names).unwrap();
                reuse.project(renamed).unwrap();
            }
            assert_eq!(reuse.projections.borrow().len(), ProjectionReuse::RETAINED_PROJECTIONS);
            let misses = reuse.projection_misses.get();
            assert_eq!(*reuse.project(voices).unwrap(), cold);
            assert_eq!(reuse.projection_misses.get(), misses + 1);
        }
    }

    #[test]
    fn indexed_beat_context_keeps_cross_staff_and_cross_measure_custody_checks() {
        let (_, original) = producer_group_fixture(false, 83);
        let heads = [(17, 0, 0), (29, 0, 0), (41, 0, 0), (53, 0, 0)];
        let rows = [(17, 83, "1", 0, "a"), (29, 83, "1", 24, "a"),
            (41, 83, "1", 48, "a"), (53, 83, "1", 72, "a"), (67, 84, "2", 0, "a")];
        let before = measure_context_fixture(&rows);
        let mut after = measure_context_fixture(&rows);
        let certify = |after: &SharingProjection| {
            let index = BeatClosureIndex::new(&before, after);
            complete_owner_beat_members(&before, after, &heads, &original, &index)
        };
        assert_eq!(certify(&after), Some(BTreeSet::from(heads)));

        let unrelated = (67, 0, 0);
        after.notes.get_mut(&unrelated).unwrap().measure = 83;
        assert_eq!(certify(&after), None);

        let member = after.notes.get_mut(&unrelated).unwrap();
        member.measure = 84;
        member.chord_head = Some(heads[0]);
        assert_eq!(certify(&after), None);
    }

    #[test]
    fn allocation_closes_the_original_full_beat_and_carries_its_chord() {
        for offset in [83, 1083] {
            let (canonical, observation) = producer_group_fixture(false, offset);
            let start = canonical.source_locators.iter().map(|row| row.onset).min().unwrap();
            let end = canonical.source_locators.iter().map(|row| row.onset + row.duration).max().unwrap();
            let attacks = canonical.source_locators.iter().map(|row| FrozenVoiceAttackEvidenceRow {
                source_event_id: row.source_event_id, attack_id: row.attack_id,
                absolute_start: row.onset,
                absolute_end: if row.onset == start { end } else { row.onset + row.duration },
                staff: row.staff.parse().unwrap(), technical_lane: 1,
            }).collect::<Vec<_>>();
            let baseline = build_frozen_serialization_voice_channels_with_original_beams(
                &canonical, &attacks, Some(&observation), &[],
            ).unwrap();
            let voices = bind_musicxml_voice_labels(&baseline, &[]).unwrap();
            assert_eq!(baseline.channel_count(), 2);
            let mut report = String::new();
            let (replacement, labels) = baseline.checked_written_only_replacement(
                &mut report, &canonical, &attacks, &observation, &[], &voices,
            ).unwrap_or_else(|| panic!("{report}"));
            assert_eq!(replacement.channel_count(), 1);
            assert!(report.contains("WrittenOnlyOwnerBeatClosure: certified_pairs=20;"));
            replacement.require_consumer_admission(&canonical, &labels).unwrap();
        }
    }

    #[test]
    fn checked_written_allocation_adopts_a_producer_bound_sounding_overhang() {
        for first_measure in [17, 1017] {
            let next_measure = first_measure + 1;
            let base = crate::pipeline::actual_beam_owner_capture_tests::input().replace("probe", &first_measure.to_string());
            let next = format!(r#"<measure number="{next_measure}"><note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>3</voice><type>quarter</type><staff>1</staff></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>96</duration><voice>3</voice><type>quarter</type><staff>1</staff></note><note><rest/><duration>192</duration><voice>3</voice><type>half</type><staff>1</staff></note></measure>"#);
            let input = base.replace("</part></score-partwise>", &format!("{next}</part></score-partwise>"));
            let (xml, _, capture) = crate::pipeline::actual_beam_owner_capture_tests::execute(&input);
            let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
            let (canonical, _) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
            let original = actual_source_beam_binding::observe_actual_source_beam_binding(&canonical, &xml, Some(&Ok(capture)));
            let attacks = canonical.source_locators.iter().map(|row| {
                let start = row.onset + if row.measure == next_measure { 384 } else { 0 };
                let overhang = if row.measure == next_measure && row.onset == 0 { 12 } else { 0 };
                FrozenVoiceAttackEvidenceRow {
                    source_event_id: row.source_event_id, attack_id: row.attack_id,
                    absolute_start: start, absolute_end: start + row.duration + overhang,
                    staff: row.staff.parse().unwrap(), technical_lane: 1,
                }
            }).collect::<Vec<_>>();
            let baseline = build_frozen_serialization_voice_channels_with_original_beams(
                &canonical, &attacks, Some(&original), &[],
            ).unwrap();
            let voices = bind_musicxml_voice_labels(&baseline, &[]).unwrap();
            assert_eq!(baseline.channel_count(), 2);
            let mut report = String::new();
            let (replacement, labels) = baseline.checked_written_only_replacement(
                &mut report, &canonical, &attacks, &original, &[], &voices,
            ).unwrap_or_else(|| panic!("{report}"));
            assert_eq!(replacement.channel_count(), 1);
            replacement.require_consumer_admission(&canonical, &labels).unwrap();
        }
    }

    #[test]
    fn voice_probe_uses_the_existing_canonical_projector_and_receipt() {
        for offset in [83, 1083] {
            let (canonical, _) = producer_group_fixture(false, offset);
            let attacks = canonical.source_locators.iter().map(|row| FrozenVoiceAttackEvidenceRow {
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                absolute_start: row.onset,
                absolute_end: row.onset + row.duration,
                staff: row.staff.parse().unwrap(),
                technical_lane: 1,
            }).collect::<Vec<_>>();
            let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
            let voices = bind_musicxml_voice_labels(&channels, &[]).unwrap();
            let mut out = String::new();
            let projection = SharingContext::projection_for_voices(&canonical, voices.clone()).unwrap();
            append_voice_probe_comparison(&mut out, canonical.semantic_fingerprint_fnv64, &projection, &projection);
            assert!(out.contains("missing=0 extra=0 changed_non_voice=0"));
            assert!(out.contains("lost=0 added=0"));
            assert!(out.contains("non_pitched_equal=true"));
        }
    }

    #[test]
    fn producer_bound_sharing_preserves_a_closed_host_group_through_a_retained_chord() {
        for offset in [83, 1083] {
            let (canonical, observation) = producer_group_fixture(false, offset);
            let mut heads = canonical.source_locators.iter().filter(|l| !l.chord_syntax).collect::<Vec<_>>();
            heads.sort_by_key(|l| l.raw_note_index);
            let classes = canonical.source_locators.iter().map(|l| (l.source_event_id,
                canonical.actions.iter().find(|a| a.fragment_id == l.fragment_id)
                    .and_then(|a| a.host_source_event_id).unwrap_or(l.source_event_id))).collect::<Partition>();
            let source_roles = classes.keys().map(|id| (*id, SerializedVoicePresentationRole::UpperPrimary)).collect::<Roles>();
            let channels = classes.values().map(|id| (*id, if *id == heads[0].source_event_id || *id == heads[3].source_event_id { 17 } else { 29 })).collect::<Partition>();
            let roles = Roles::from([(17,SerializedVoicePresentationRole::UpperPrimary),(29,SerializedVoicePresentationRole::UpperPrimary)]);
            let legacy = classes.iter().map(|(source,class)| LegacyVoiceLabelPreferenceRow { source_event_id:*source,
                label:format!("opaque-{}", channels[class]) }).collect::<Vec<_>>();
            let context = SharingContext::new(&canonical,&observation,&classes,&source_roles,&legacy);
            let before = context.projection(&channels,&roles,None).unwrap();
            assert_eq!(before.edges.len(),2);
            let proposed = channels.keys().map(|id| (*id,17)).collect::<Partition>();
            let surviving = Roles::from([(17,SerializedVoicePresentationRole::UpperPrimary)]);
            assert!(context.compatible(&before,&channels,&proposed,&surviving));
            let after = context.projection(&proposed,&surviving,Some(&before.voices)).unwrap();
            assert_eq!(after.edges.len(),20);
            for (key,note) in &before.notes {
                assert_eq!(note.non_voice_bytes,after.notes[key].non_voice_bytes);
                assert_eq!(note.chord_head,after.notes[key].chord_head);
                if note.chord_head.is_some() { assert!(after.notes[key].states.is_empty()); }
            }
            let missing = ActualSourceBeamBindingObservation::default();
            let unknown = SharingContext::new(&canonical,&missing,&classes,&source_roles,&legacy);
            assert!(!unknown.compatible(&before,&channels,&proposed,&surviving));
        }
    }

    #[test]
    fn a_new_closed_written_group_cannot_join_distinct_original_cohorts() {
        let (canonical, observation) = producer_group_fixture(true, 83);
        let mut heads = canonical.source_locators.iter().collect::<Vec<_>>(); heads.sort_by_key(|l| l.raw_note_index);
        let classes = heads.iter().map(|l|(l.source_event_id,l.source_event_id)).collect::<Partition>();
        let source_roles = classes.keys().map(|id|(*id,SerializedVoicePresentationRole::UpperPrimary)).collect::<Roles>();
        let channels = heads.iter().enumerate().map(|(index,l)|(l.source_event_id,
            if index==0 {17} else if index>=5 {29} else {100+index})).collect::<Partition>();
        let roles = channels.values().map(|id|(*id,SerializedVoicePresentationRole::UpperPrimary)).collect::<Roles>();
        let legacy = classes.iter().map(|(source,class)|LegacyVoiceLabelPreferenceRow {source_event_id:*source,
            label:format!("opaque-{}",channels[class])}).collect::<Vec<_>>();
        let context = SharingContext::new(&canonical,&observation,&classes,&source_roles,&legacy);
        let before = context.projection(&channels,&roles,None).unwrap(); assert!(before.edges.is_empty());
        let proposed = channels.iter().map(|(id,ch)|(*id,if *ch==29 {17} else {*ch})).collect::<Partition>();
        let mut surviving=roles; surviving.remove(&29);
        let misses = context.reuse().projection_misses.get();
        assert!(!context.compatible(&before,&channels,&proposed,&surviving));
        assert_eq!(context.diagnostic_count("new_beam_edge_cohort_mismatch"),1);
        assert_eq!(context.diagnostic_count("new_connection_owner_proof_preflight"),1);
        assert_eq!(context.reuse().projection_misses.get(), misses);
        let after = context.projection(&proposed,&surviving,Some(&before.voices)).unwrap();
        assert!(!after.edges.is_empty());
    }

    #[test]
    fn a_physical_member_on_another_staff_or_voice_cannot_inherit_head_connections() {
        for other_staff in [false, true] {
            let mut projection = marked_fixture(&["begin", "continue", "member", "end"], 17, 1);
            let member = projection.notes.get_mut(&(19,0,0)).unwrap();
            member.chord_head = Some((18,0,0)); member.states.clear(); member.onset = 120;
            if other_staff { member.staff = "2".into(); } else { member.label = "other".into(); }
            let edges = written_connections(&projection.notes);
            assert_eq!(edges.len(),12);
            assert!(edges.iter().all(|(_,left,right)| *left != (19,0,0) && *right != (19,0,0)));
        }
    }

    #[test]
    fn non_pitched_physical_heads_are_bound_without_beam_inheritance() {
        for offset in [83, 1083] { for copies in [1, 2] {
            let mut source = crate::pipeline::actual_beam_owner_capture_tests::input()
                .replace("probe", &offset.to_string());
            let extra = (0..copies).map(|index| format!(
                "<measure number=\"{}\"><note><rest/><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note><note print-object=\"no\" print-spacing=\"no\"><chord/><pitch><step>G</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note><note><rest/><duration>360</duration><voice>2</voice><type>half</type><dot/><dot/><dot/><staff>2</staff></note></measure>",
                offset + index + 1,
            )).collect::<String>();
            source = source.replace("</part>", &format!("{extra}</part>"));
            let (xml, _, capture) = crate::pipeline::actual_beam_owner_capture_tests::execute(&source);
            let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
            let (canonical, _) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
            let observation = actual_source_beam_binding::observe_actual_source_beam_binding(&canonical, &xml, Some(&Ok(capture)));
            let classes = canonical.source_locators.iter().map(|l| (l.source_event_id, l.source_event_id)).collect::<Partition>();
            let source_roles = canonical.source_locators.iter().map(|l| (l.source_event_id,
                if l.staff == "1" { SerializedVoicePresentationRole::UpperSecondary } else { SerializedVoicePresentationRole::LowerPrimary })).collect::<Roles>();
            let channels = canonical.source_locators.iter().map(|l| (l.source_event_id,
                if l.staff == "1" { 17 } else { 29 })).collect::<Partition>();
            let roles = Roles::from([(17, SerializedVoicePresentationRole::UpperSecondary), (29, SerializedVoicePresentationRole::LowerPrimary)]);
            let context = SharingContext::new(&canonical, &observation, &classes, &source_roles, &[]);
            let projection = context.projection(&channels, &roles, None).unwrap();
            let carriers = projection.notes.iter().filter(|(_, n)| n.non_pitched_chord_head.is_some()).collect::<Vec<_>>();
            assert_eq!(carriers.len(), copies);
            for (key, note) in carriers {
                assert!(note.chord_head.is_none());
                assert!(projection.edges.iter().all(|(_, a, b)| a != key && b != key));
            }
            assert!(projection.protected_channels.contains(&29));
            assert!(context.compatible(&projection, &channels, &channels, &roles));
        }}
    }

    #[test]
    fn a_refused_complete_distribution_can_use_another_host_without_relaxing_beams() {
        for offset in [83, 1083] {
            let mut source = crate::pipeline::actual_beam_owner_capture_tests::input()
                .replace("probe", &offset.to_string())
                .replace("<duration>288</duration>", "<duration>192</duration>").replace("<dot/>", "");
            let extra_group = ["G", "A", "B", "C"].into_iter().map(|step| format!(
                "<note><pitch><step>{step}</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff></note>"
            )).collect::<String>();
            source = source.replace("</measure>", &format!("{extra_group}</measure>"));
            let extra = (0..2).map(|index| format!(
                "<measure number=\"{}\"><note><pitch><step>G</step><octave>3</octave></pitch><duration>384</duration><voice>5</voice><type>whole</type><staff>1</staff></note></measure>", offset + index + 1,
            )).collect::<String>();
            source = source.replace("</part>", &format!("{extra}</part>"));
            let (xml, _, capture) = crate::pipeline::actual_beam_owner_capture_tests::execute(&source);
            let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
            let (canonical, _) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
            let observation = actual_source_beam_binding::observe_actual_source_beam_binding(&canonical, &xml, Some(&Ok(capture)));
            let mut locators = canonical.source_locators.iter().collect::<Vec<_>>();
            locators.sort_by_key(|l| (l.measure, l.raw_note_index));
            assert_eq!(locators.len(), 10);
            let class_order = locators.iter().map(|l| l.source_event_id).collect::<Vec<_>>();
            let classes = class_order.iter().map(|&id| (id, id)).collect::<Partition>();
            let assignments = [17, 41, 41, 41, 29, 29, 29, 67, 53, 67];
            let initial = class_order.iter().copied().zip(assignments).collect::<Partition>();
            let roles = Roles::from([(17, SerializedVoicePresentationRole::UpperPrimary),
                (29, SerializedVoicePresentationRole::UpperPrimary), (41, SerializedVoicePresentationRole::UpperSecondary),
                (53, SerializedVoicePresentationRole::UpperPrimary), (67, SerializedVoicePresentationRole::UpperPrimary)]);
            let source_roles = classes.keys().map(|&id| (id, roles[&initial[&id]])).collect::<Roles>();
            let legacy = classes.keys().map(|&id| LegacyVoiceLabelPreferenceRow { source_event_id: id,
                label: format!("opaque-{}", initial[&id]) }).collect::<Vec<_>>();
            let mut conflicts = BTreeSet::new();
            let tail = class_order[9];
            // These already bound host conflicts make the refused complete
            // placement a dead end; no later whole-host merge can rescue it.
            for (&left, &left_channel) in &initial {
                for (&right, &right_channel) in &initial {
                    if [17, 29, 53].contains(&left_channel) && [17, 29, 53].contains(&right_channel)
                        && left_channel != right_channel
                    { conflicts.insert((left.min(right), left.max(right))); }
                }
            }
            for (&class, &channel) in &initial {
                if channel != 17 && channel != 67 {
                    conflicts.insert((class.min(tail), class.max(tail)));
                }
            }
            let class_role = source_roles.clone();
            let model = channel_consumer_model(canonical.written_projection.as_ref().unwrap(), &classes).unwrap();
            let empty = BTreeSet::new();
            let input = ChannelSearchInput { model: &model, class_order: &class_order,
                class_role: &class_role, sounding_different: &TimeConflictEdges::fixture(&conflicts), written_different: &TimeConflictEdges::fixture(&empty) };
            let context = SharingContext::new(&canonical, &observation, &classes, &source_roles, &legacy);
            let baseline = context.projection(&initial, &roles, None).unwrap();
            let groups = SharingContext::evacuation_groups(&baseline, &[class_order[7], tail], &classes);
            let hosts = [17, 29, 53];
            let evacuation = ChannelEvacuation { guest: 67, hosts: &hosts, groups: &groups };
            let measure_incidence = MeasureIncidence::new(&baseline, &classes);
            let refused_compatibility = RefCell::new(SeenPartitions::new());
            let traversal = SharingTraversal { distributed_candidates: true, input: &input, initial: &initial, baseline: &baseline, measure_incidence: &measure_incidence, refused_compatibility: &refused_compatibility };
            let first = context.place_evacuation_groups(&traversal, &evacuation, &initial, &roles, None).unwrap();
            let mut survivors = roles.clone(); survivors.remove(&67);
            assert!(!context.compatible(&baseline, &initial, &first, &survivors));
            assert!(context.diagnostic_count("new_beam_edge_cohort_mismatch") > 0);
            let result = context.evacuate_groups(&traversal, &evacuation, &initial, &roles, &mut SeenPartitions::new()).unwrap();
            let hits = context.reuse().projection_hits.get();
            let refusals_reused = context.diagnostic_count("compatibility_refusal_reused");
            let repeated = context.evacuate_groups(&traversal, &evacuation, &initial, &roles, &mut SeenPartitions::new()).unwrap();
            assert_eq!(result, repeated);
            assert!(context.diagnostic_count("compatibility_refusal_reused") > refusals_reused);
            assert!(context.reuse().projection_hits.get() > hits);
            assert_eq!(result.0[&class_order[7]], 53);
            assert!(context.diagnostic_count("unmoved_beam_connections_changed") > 0);
            assert_eq!(result.0[&tail], 17);
            assert!(context.diagnostic_count("whole_channel_distributed_alternative_attempted") > 0);
            assert!(context.compatible(&baseline, &initial, &result.0, &result.1));
            assert!(channel_consumer_overflow_with_labels(&model, &result.0, &result.1,
                Some(&context.projection(&result.0, &result.1, Some(&baseline.voices)).unwrap().voices)).is_ok());
        }
    }

}
