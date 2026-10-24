#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MaterialSourceEventId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SoundingAttackId(usize);

#[derive(Debug, Clone, Copy)]
struct CandidateMaterialDeclaredEffects {
    attack_existence: bool,
    attack_onset: bool,
    sounding_end: bool,
    pitch: bool,
    staff: bool,
    voice: bool,
}

impl CandidateMaterialDeclaredEffects {
    const fn representation_only() -> Self {
        Self {
            attack_existence: false,
            attack_onset: false,
            sounding_end: false,
            pitch: false,
            staff: false,
            voice: false,
        }
    }

    const fn candidate_material() -> Self {
        Self {
            attack_existence: true,
            attack_onset: false,
            sounding_end: false,
            pitch: true,
            staff: true,
            voice: true,
        }
    }

    const fn attack_existence_only() -> Self {
        Self {
            attack_existence: true,
            attack_onset: false,
            sounding_end: false,
            pitch: false,
            staff: false,
            voice: false,
        }
    }

    const fn consensus_gesture_relocation() -> Self {
        Self {
            attack_existence: false,
            attack_onset: true,
            sounding_end: false,
            pitch: false,
            staff: true,
            voice: true,
        }
    }

    const fn final_chord_sounding_materialization() -> Self {
        Self {
            attack_existence: false,
            attack_onset: true,
            sounding_end: false,
            pitch: false,
            staff: false,
            voice: false,
        }
    }

    const fn staff_relocation() -> Self {
        Self {
            attack_existence: false,
            attack_onset: false,
            sounding_end: false,
            pitch: false,
            staff: true,
            voice: false,
        }
    }

    const fn staff_voice_relocation() -> Self {
        Self {
            attack_existence: false,
            attack_onset: false,
            sounding_end: false,
            pitch: false,
            staff: true,
            voice: true,
        }
    }

    const fn pitch_staff_voice() -> Self {
        Self {
            attack_existence: false,
            attack_onset: false,
            sounding_end: false,
            pitch: true,
            staff: true,
            voice: true,
        }
    }

    
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CandidateMaterialAttackState {
    source_event_id: MaterialSourceEventId,
    attack_id: SoundingAttackId,
    exists: bool,
    measure: usize,
    onset: i64,
    sounding_end: i64,
    pitch: i32,
    staff: String,
    voice: String,
}

/// Exact pre-lane notation relation emitted from the authenticated material state.
/// Private fields prevent a report or final XML scan from minting this witness.
#[derive(Debug, Clone)]
pub(crate) struct OriginalNotationVoiceBindings {
    original: std::sync::Arc<str>,
    voices: std::collections::BTreeMap<(usize, usize), String>,
}
impl OriginalNotationVoiceBindings {
    pub(crate) fn original_voice(&self, original: &str, source: usize, attack: usize) -> Option<&str> {
        (self.original.as_ref() == original).then(|| self.voices.get(&(source, attack)))
            .flatten().map(String::as_str).filter(|voice| !voice.trim().is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CandidateMaterialFragmentState {
    measure: usize,
    raw_note_index: usize,
    onset: i64,
    duration: i64,
    pitch: Option<i32>,
    staff: String,
    voice: String,
    visible: bool,
    is_chord: bool,
    tie_start: bool,
    tie_stop: bool,
    source_event_id: Option<MaterialSourceEventId>,
    attack_id: Option<SoundingAttackId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CandidateMaterialRepresentationTransferCarrier {
    before_note_start: usize,
    before_note_end: usize,
    after_measure: String,
    after_onset: i64,
    after_pitch: i32,
    after_staff: String,
    after_voice: String,
    after_is_chord: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CandidateMaterialRepresentationTransferPreflight {
    Eligible(Vec<(usize, usize)>),
    PreexistingTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CandidateMaterialExactNoteMoveCarrier {
    before_note_start: usize,
    before_note_end: usize,
    after_note_start: usize,
}

impl CandidateMaterialFragmentState {
    fn exact_material_signature(&self) -> String {
        format!(
            "{}|{}|{}|{:?}|{}|{}|{}|{}|{}|{}",
            self.measure,
            self.onset,
            self.duration,
            self.pitch,
            self.staff,
            self.voice,
            self.visible,
            self.is_chord,
            self.tie_start,
            self.tie_stop
        )
    }

    fn temporal_key(&self) -> (usize, i64) {
        (self.measure, self.onset)
    }

    fn is_sounding_attack_fragment(&self) -> bool {
        self.pitch.is_some() && !self.tie_stop
    }
}

#[derive(Debug, Clone)]
struct CandidateMaterialMutationReceipt {
    owner: String,
    declared: CandidateMaterialDeclaredEffects,
    active_before: usize,
    active_after: usize,
    existence_changes: usize,
    onset_changes: usize,
    sounding_end_changes: usize,
    pitch_changes: usize,
    staff_changes: usize,
    voice_changes: usize,
    changed_source_ordinals: Vec<usize>,
    state_fingerprint_fnv64: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct FrozenSoundingTemporalLaneMember {
    source_event_id: MaterialSourceEventId,
    attack_id: SoundingAttackId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FrozenSoundingTemporalLanePlanRow {
    staff: u8,
    absolute_start: i64,
    absolute_end: i64,
    lane: usize,
    members: std::sync::Arc<[FrozenSoundingTemporalLaneMember]>,
}

#[derive(Debug, Clone)]
struct FrozenSoundingTemporalLanePlan {
    authority_bound: bool,
    active_attack_count: usize,
    exact_interval_cohort_count: usize,
    assigned_attack_count: usize,
    upper_staff_lane_count: usize,
    lower_staff_lane_count: usize,
    overlap_collision_count: usize,
    secondary_lane_cohort_count: usize,
    secondary_lane_attack_count: usize,
    cohort_segment_count: usize,
    event_segment_count: usize,
    cross_measure_cohort_count: usize,
    cross_measure_cohort_boundary_count: usize,
    cross_measure_event_boundary_count: usize,
    frozen_state_fingerprint_fnv64: u64,
    frozen_projection_fingerprint_fnv64: u64,
    fingerprint_fnv64: u64,
    rows: std::sync::Arc<[FrozenSoundingTemporalLanePlanRow]>,
    notation_voice_pre_serialization_constraint_audit:
        crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::
            NotationVoicePreSerializationConstraintClosureAuditProjection,
    actual_source_beam_binding: crate::structural_write_authority::ActualSourceBeamBindingObservation,
    final_chord_structural_plan: crate::structural_write_authority::CanonicalFinalChordStructuralPlan,
    frozen_serialized_voice_plan: crate::structural_write_authority::FrozenSerializedVoicePlan,
}

impl Default for FrozenSoundingTemporalLanePlan {
    fn default() -> Self {
        Self {
            authority_bound: false,
            active_attack_count: 0,
            exact_interval_cohort_count: 0,
            assigned_attack_count: 0,
            upper_staff_lane_count: 0,
            lower_staff_lane_count: 0,
            overlap_collision_count: 0,
            secondary_lane_cohort_count: 0,
            secondary_lane_attack_count: 0,
            cohort_segment_count: 0,
            event_segment_count: 0,
            cross_measure_cohort_count: 0,
            cross_measure_cohort_boundary_count: 0,
            cross_measure_event_boundary_count: 0,
            frozen_state_fingerprint_fnv64: 0,
            frozen_projection_fingerprint_fnv64: 0,
            fingerprint_fnv64: 0,
            rows: std::sync::Arc::from(Vec::<FrozenSoundingTemporalLanePlanRow>::new()),
            notation_voice_pre_serialization_constraint_audit:
                crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::
                    NotationVoicePreSerializationConstraintClosureAuditProjection::default(),
            actual_source_beam_binding: Default::default(),
            final_chord_structural_plan:
                crate::structural_write_authority::CanonicalFinalChordStructuralPlan::default(),
            frozen_serialized_voice_plan:
                crate::structural_write_authority::FrozenSerializedVoicePlan::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct CandidateMaterialLineageSpine {
    authority_bound: bool,
    source_event_count: usize,
    attack_count: usize,
    active_attack_count: usize,
    source_events: std::sync::Arc<[MaterialSourceEventId]>,
    attacks: std::sync::Arc<[CandidateMaterialAttackState]>,
    fragments: std::sync::Arc<[CandidateMaterialFragmentState]>,
    receipts: std::sync::Arc<[CandidateMaterialMutationReceipt]>,
    state_fingerprint_fnv64: u64,
    ambiguous_source_groups: usize,
    freeze_boundary_candidate_reached: bool,
    final_sounding_material_frozen: bool,
    frozen_state_fingerprint_fnv64: u64,
    frozen_active_attack_count: usize,
    frozen_projection_fingerprint_fnv64: u64,
    frozen_sounding_temporal_lane_plan: FrozenSoundingTemporalLanePlan,
}

fn material_lineage_fnv1a64_update(mut state: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(0x100000001b3);
    }
    state
}

fn sounding_attack_id_for_source(source_event_id: MaterialSourceEventId) -> SoundingAttackId {
    // r666 currently permits at most one sounding attack identity per retained SourceEvent.
    // The type remains distinct so a future explicitly authorized one-to-many transition cannot
    // silently collapse SourceEventId and SoundingAttackId into the same semantic domain.
    SoundingAttackId(source_event_id.0)
}

fn material_fragment_from_atomic(
    xml: &str,
    note: &AtomicNote,
    raw_note_index: usize,
) -> anyhow::Result<CandidateMaterialFragmentState> {
    let measure = note.measure.parse::<usize>().map_err(|_| {
        anyhow!(
            "CandidateMaterialLineageSpine: nichtnumerischer Measure-Locator {:?}",
            note.measure
        )
    })?;
    let block = xml.get(note.note_start..note.note_end).ok_or_else(|| {
        anyhow!("CandidateMaterialLineageSpine: AtomicNote-Span liegt ausserhalb des XML")
    })?;
    let tie_start = block.contains("<tie type=\"start\"") || block.contains("<tie type='start'");
    let tie_stop = block.contains("<tie type=\"stop\"") || block.contains("<tie type='stop'");
    Ok(CandidateMaterialFragmentState {
        measure,
        raw_note_index,
        onset: note.start,
        duration: note.duration,
        pitch: note.pitch.as_ref().map(|pitch| pitch.midi),
        staff: note.staff.clone(),
        voice: note.voice.clone(),
        visible: note.visible,
        is_chord: note.is_chord,
        tie_start,
        tie_stop,
        source_event_id: None,
        attack_id: None,
    })
}

fn material_fragments_from_xml(xml: &str) -> anyhow::Result<Vec<CandidateMaterialFragmentState>> {
    let notes = parse_atomic_notes(xml);
    let mut next_raw_index_by_measure = BTreeMap::<usize, usize>::new();
    let mut fragments = Vec::with_capacity(notes.len());
    for note in &notes {
        let measure = note.measure.parse::<usize>().map_err(|_| {
            anyhow!(
                "CandidateMaterialLineageSpine: nichtnumerischer Measure-Locator {:?}",
                note.measure
            )
        })?;
        let raw_note_index = next_raw_index_by_measure.entry(measure).or_default();
        fragments.push(material_fragment_from_atomic(xml, note, *raw_note_index)?);
        *raw_note_index = raw_note_index.saturating_add(1);
    }
    Ok(fragments)
}

#[derive(Debug, Clone)]
struct CandidateMaterialLineageGapAudit {
    authority_bound: bool,
    material_fragment_count: usize,
    xml_atomic_note_count: usize,
    failure_measure: Option<usize>,
    failure_onset: Option<i64>,
    unmatched_new_fragment_count: usize,
    unmatched_old_fragment_count: usize,
    unmatched_pitched_fragments: Vec<CandidateMaterialFragmentState>,
    mapping_status: String,
    receipt_count: usize,
    last_receipt: Option<String>,
    state_fingerprint_fnv64: u64,
}

fn audit_candidate_material_lineage_gap(
    candidate: &SolverCandidate,
    declared: CandidateMaterialDeclaredEffects,
) -> anyhow::Result<CandidateMaterialLineageGapAudit> {
    let before_fragments = candidate.material_lineage.fragments.as_ref();
    let mut after_fragments = material_fragments_from_xml(candidate.layout_xml())?;
    let mapping_status = if candidate.material_lineage.authority_bound {
        match map_candidate_material_fragments_forward_with_explicit_pairs(
            before_fragments,
            &mut after_fragments,
            declared,
            &[],
            true,
        ) {
            Ok(()) => "mapped".to_string(),
            Err(error) => format!("blocked: {error:#}"),
        }
    } else {
        "authority_unbound".to_string()
    };

    let failure_key = after_fragments
        .iter()
        .filter(|fragment| fragment.source_event_id.is_none() && fragment.pitch.is_some())
        .map(CandidateMaterialFragmentState::temporal_key)
        .min();
    let unmatched_new_fragment_count = failure_key.map_or(0, |key| {
        after_fragments
            .iter()
            .filter(|fragment| {
                fragment.temporal_key() == key && fragment.source_event_id.is_none()
            })
            .count()
    });
    let unmatched_old_fragment_count = failure_key.map_or(0, |key| {
        let mut old_counts = BTreeMap::<MaterialSourceEventId, usize>::new();
        let mut mapped_counts = BTreeMap::<MaterialSourceEventId, usize>::new();
        for source_event_id in before_fragments
            .iter()
            .filter(|fragment| fragment.temporal_key() == key)
            .filter_map(|fragment| fragment.source_event_id)
        {
            *old_counts.entry(source_event_id).or_default() += 1;
        }
        for source_event_id in after_fragments
            .iter()
            .filter(|fragment| fragment.temporal_key() == key)
            .filter_map(|fragment| fragment.source_event_id)
        {
            *mapped_counts.entry(source_event_id).or_default() += 1;
        }
        old_counts
            .into_iter()
            .map(|(source_event_id, old_count)| {
                old_count.saturating_sub(mapped_counts.get(&source_event_id).copied().unwrap_or(0))
            })
            .sum()
    });
    let unmatched_pitched_fragments = failure_key.map_or_else(Vec::new, |key| {
        after_fragments
            .iter()
            .filter(|fragment| {
                fragment.temporal_key() == key
                    && fragment.source_event_id.is_none()
                    && fragment.pitch.is_some()
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    let last_receipt = candidate.material_lineage.receipts.last().map(|receipt| {
        format!(
            "owner={} active={}->{} existence={} onset={} sounding_end={} pitch={} staff={} voice={} changed_sources={} fp={:016x}",
            receipt.owner,
            receipt.active_before,
            receipt.active_after,
            receipt.existence_changes,
            receipt.onset_changes,
            receipt.sounding_end_changes,
            receipt.pitch_changes,
            receipt.staff_changes,
            receipt.voice_changes,
            receipt.changed_source_ordinals.len(),
            receipt.state_fingerprint_fnv64
        )
    });

    Ok(CandidateMaterialLineageGapAudit {
        authority_bound: candidate.material_lineage.authority_bound,
        material_fragment_count: before_fragments.len(),
        xml_atomic_note_count: after_fragments.len(),
        failure_measure: failure_key.map(|key| key.0),
        failure_onset: failure_key.map(|key| key.1),
        unmatched_new_fragment_count,
        unmatched_old_fragment_count,
        unmatched_pitched_fragments,
        mapping_status,
        receipt_count: candidate.material_lineage.receipts.len(),
        last_receipt,
        state_fingerprint_fnv64: candidate.material_lineage.state_fingerprint_fnv64,
    })
}

fn material_lineage_state_fingerprint(
    source_events: &[MaterialSourceEventId],
    attacks: &[CandidateMaterialAttackState],
) -> u64 {
    let mut state = 0xcbf29ce484222325u64;
    state = material_lineage_fnv1a64_update(state, b"ETUSCHLOPF_CANDIDATE_MATERIAL_STATE_V2\0");
    let mut ordered_sources = source_events.to_vec();
    ordered_sources.sort_unstable();
    for source_event_id in ordered_sources {
        state = material_lineage_fnv1a64_update(
            state,
            &u64::try_from(source_event_id.0)
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
    }
    state = material_lineage_fnv1a64_update(state, &[0xfe]);
    let mut ordered_attacks = attacks.iter().collect::<Vec<_>>();
    ordered_attacks.sort_by_key(|attack| attack.source_event_id);
    for attack in ordered_attacks {
        state = material_lineage_fnv1a64_update(
            state,
            &u64::try_from(attack.source_event_id.0)
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
        state = material_lineage_fnv1a64_update(
            state,
            &u64::try_from(attack.attack_id.0)
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
        state = material_lineage_fnv1a64_update(state, &[u8::from(attack.exists)]);
        state = material_lineage_fnv1a64_update(
            state,
            &u64::try_from(attack.measure).unwrap_or(u64::MAX).to_le_bytes(),
        );
        state = material_lineage_fnv1a64_update(state, &attack.onset.to_le_bytes());
        state = material_lineage_fnv1a64_update(state, &attack.sounding_end.to_le_bytes());
        state = material_lineage_fnv1a64_update(state, &attack.pitch.to_le_bytes());
        state = material_lineage_fnv1a64_update(state, attack.staff.as_bytes());
        state = material_lineage_fnv1a64_update(state, &[0]);
        state = material_lineage_fnv1a64_update(state, attack.voice.as_bytes());
        state = material_lineage_fnv1a64_update(state, &[0xff]);
    }
    state
}

fn bind_candidate_material_lineage_from_post_legacy(
    candidate: &mut SolverCandidate,
) -> anyhow::Result<()> {
    let causal = &candidate.midi_seed_stats.source_selection_full_causal;
    let disposition = &causal.productive_post_legacy_disposition_plan;
    let serialization = &causal.midi_seed_serialized_source_identity_plan;
    if !disposition.authority_bound || !serialization.authority_bound {
        return Ok(());
    }
    if candidate.material_lineage.authority_bound {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: Authority wurde mehrfach gebunden"
        ));
    }
    if serialization.mapped_event_count != disposition.retained_event_count
        || serialization.rows.len() != disposition.retained_event_count
        || disposition.rows.len() != disposition.retained_event_count
    {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: autoritative SourceIdentity-/Disposition-Coverage ist nicht exakt"
        ));
    }

    let mut serialization_by_source = BTreeMap::<usize, (usize, usize)>::new();
    let mut source_by_pre_raw = BTreeMap::<(usize, usize), usize>::new();
    for row in serialization.rows.iter() {
        if serialization_by_source
            .insert(row.source_ordinal, (row.measure_ordinal, row.raw_note_index))
            .is_some()
            || source_by_pre_raw
                .insert((row.measure_ordinal, row.raw_note_index), row.source_ordinal)
                .is_some()
        {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: MidiSeed-SourceIdentity ist nicht eindeutig"
            ));
        }
    }

    let expected_measure_count = serialization.measure_raw_note_counts.len();
    if candidate
        .arrangement_stats
        .representation_post_raw_note_counts
        .len()
        != expected_measure_count
    {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: VisibleRhythm-PostRawNote-Count-Coverage {} widerspricht Serialisierungs-Taktzahl {}",
            candidate
                .arrangement_stats
                .representation_post_raw_note_counts
                .len(),
            expected_measure_count
        ));
    }

    let mut lineage_by_pre = BTreeMap::<(usize, usize), Vec<(usize, usize)>>::new();
    let mut seen_post = BTreeSet::<(usize, usize)>::new();
    for row in &candidate.arrangement_stats.representation_lineage_rows {
        if row.measure_ordinal == 0 || row.measure_ordinal > expected_measure_count {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: ungueltiger VisibleRhythm-Lineage-Takt {}",
                row.measure_ordinal
            ));
        }
        let post_count = candidate.arrangement_stats.representation_post_raw_note_counts
            [row.measure_ordinal - 1];
        if row.post_ledger_raw_note_index >= post_count {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: VisibleRhythm-PostIndex {} ausserhalb Takt {} mit {} RawNotes",
                row.post_ledger_raw_note_index,
                row.measure_ordinal,
                post_count
            ));
        }
        if !seen_post.insert((row.measure_ordinal, row.post_ledger_raw_note_index)) {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: VisibleRhythm-PostIndex Takt {} Index {} mehrfach belegt",
                row.measure_ordinal,
                row.post_ledger_raw_note_index
            ));
        }
        lineage_by_pre
            .entry((row.measure_ordinal, row.pre_ledger_raw_note_index))
            .or_default()
            .push((row.fragment_ordinal, row.post_ledger_raw_note_index));
    }
    for measure_ordinal in 1..=expected_measure_count {
        let expected_pre = serialization.measure_raw_note_counts[measure_ordinal - 1];
        for raw_note_index in 0..expected_pre {
            if !lineage_by_pre.contains_key(&(measure_ordinal, raw_note_index)) {
                return Err(anyhow!(
                    "CandidateMaterialLineageSpine: VisibleRhythm-Lineage fehlt fuer Takt {} PreIndex {}",
                    measure_ordinal,
                    raw_note_index
                ));
            }
        }
    }
    for rows in lineage_by_pre.values_mut() {
        rows.sort_unstable();
        for (expected_fragment, (fragment_ordinal, _)) in rows.iter().copied().enumerate() {
            if fragment_ordinal != expected_fragment {
                return Err(anyhow!(
                    "CandidateMaterialLineageSpine: VisibleRhythm-Fragmentfolge ist nicht lueckenlos"
                ));
            }
        }
    }

    let mut fragments = material_fragments_from_xml(candidate.layout_xml())?;
    let mut current_post_counts = vec![0usize; expected_measure_count];
    let mut fragment_index_by_raw = BTreeMap::<(usize, usize), usize>::new();
    for (fragment_index, fragment) in fragments.iter().enumerate() {
        if fragment.measure == 0 || fragment.measure > expected_measure_count {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: Candidate-Fragment liegt ausserhalb der Source-Taktmenge: {}",
                fragment.measure
            ));
        }
        current_post_counts[fragment.measure - 1] =
            current_post_counts[fragment.measure - 1].saturating_add(1);
        if fragment_index_by_raw
            .insert((fragment.measure, fragment.raw_note_index), fragment_index)
            .is_some()
        {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: Candidate-RawNote-Locator ist nicht eindeutig"
            ));
        }
    }
    if current_post_counts
        != candidate
            .arrangement_stats
            .representation_post_raw_note_counts
    {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: Polish/Layout hat die VisibleRhythm-RawNote-Topologie vor der Identity-Bindung veraendert"
        ));
    }

    let disposition_by_source = disposition
        .rows
        .iter()
        .map(|row| (row.source_ordinal, row))
        .collect::<BTreeMap<_, _>>();
    if disposition_by_source.len() != disposition.retained_event_count {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: PostLegacy-Disposition besitzt doppelte SourceOrdinal"
        ));
    }

    let mut source_events = Vec::<MaterialSourceEventId>::with_capacity(disposition.retained_event_count);
    let mut attacks = Vec::<CandidateMaterialAttackState>::with_capacity(
        disposition
            .visible_pitched_event_count
            .saturating_add(disposition.hidden_pitched_event_count),
    );
    let mut used_fragment_indices = BTreeSet::<usize>::new();

    for source_ordinal in serialization_by_source.keys().copied() {
        let source_event_id = MaterialSourceEventId(source_ordinal);
        source_events.push(source_event_id);
        let (measure, pre_raw_note_index) = serialization_by_source
            .get(&source_ordinal)
            .copied()
            .ok_or_else(|| anyhow!("CandidateMaterialLineageSpine: interne SourceIdentity-Luecke"))?;
        let row = disposition_by_source.get(&source_ordinal).copied().ok_or_else(|| {
            anyhow!(
                "CandidateMaterialLineageSpine: SourceOrdinal {} ohne PostLegacy-Disposition",
                source_ordinal
            )
        })?;
        let lineage_rows = lineage_by_pre
            .get(&(measure, pre_raw_note_index))
            .cloned()
            .ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: SourceOrdinal {} ohne VisibleRhythm-Forward-Carrier",
                    source_ordinal
                )
            })?;
        if lineage_rows.is_empty() {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: SourceOrdinal {} besitzt keine RepresentationFragments",
                source_ordinal
            ));
        }

        let mut mapped_fragment_indices = Vec::<usize>::with_capacity(lineage_rows.len());
        for (_fragment_ordinal, post_raw_note_index) in lineage_rows {
            let fragment_index = fragment_index_by_raw
                .get(&(measure, post_raw_note_index))
                .copied()
                .ok_or_else(|| {
                    anyhow!(
                        "CandidateMaterialLineageSpine: VisibleRhythm-PostLocator Takt {} Index {} fehlt nach Polish/Layout",
                        measure,
                        post_raw_note_index
                    )
                })?;
            if !used_fragment_indices.insert(fragment_index) {
                return Err(anyhow!(
                    "CandidateMaterialLineageSpine: RepresentationFragment wurde mehreren SourceEvents zugeordnet"
                ));
            }
            if fragments[fragment_index].source_event_id.is_some() {
                return Err(anyhow!(
                    "CandidateMaterialLineageSpine: RepresentationFragment besitzt bereits SourceLineage"
                ));
            }
            fragments[fragment_index].source_event_id = Some(source_event_id);
            mapped_fragment_indices.push(fragment_index);
        }

        let exists = matches!(
            row.disposition,
            crate::source_selection::SourceDurationMaterialDisposition::Visible
                | crate::source_selection::SourceDurationMaterialDisposition::Hidden
        );
        if exists {
            let attack_id = sounding_attack_id_for_source(source_event_id);
            let attack_starts = mapped_fragment_indices
                .iter()
                .copied()
                .filter(|index| fragments[*index].is_sounding_attack_fragment())
                .collect::<Vec<_>>();
            if attack_starts.len() != 1 {
                return Err(anyhow!(
                    "CandidateMaterialLineageSpine: SourceOrdinal {} besitzt {} klingende Attack-Fragmente direkt hinter VisibleRhythm",
                    source_ordinal,
                    attack_starts.len()
                ));
            }
            let attack_start = &fragments[attack_starts[0]];
            let attack_start_measure = attack_start.measure;
            let attack_start_pitch = attack_start.pitch;
            let attack_start_staff = attack_start.staff.clone();
            let attack_start_voice = attack_start.voice.clone();
            let source_onset = attack_start.onset;
            let sounding_end = source_onset.saturating_add(row.source_len.max(1));
            let expected_staff = row.projected_staff.unwrap_or(row.selection_staff).to_string();
            // r666: CandidateMaterialAttackState uses one measure-local temporal domain.
            // SourceDuration remains semantic source truth; the current MusicXML fragment duration
            // is representation and is not promoted to SoundingEnd by ConsensusGesture relocation.
            // The structural SourceIdentity carrier proves which post-VisibleRhythm fragment belongs
            // to this SourceEvent. Pitch and projected staff remain valid post-legacy material checks.
            if attack_start_measure != measure
                || attack_start_pitch != Some(row.source_midi)
                || attack_start_staff != expected_staff
            {
                return Err(anyhow!(
                    "CandidateMaterialLineageSpine: SourceOrdinal {} widerspricht der autoritativen PostLegacy-Materialsignatur (measure expected={} actual={}; pitch expected={} actual={:?}; staff expected={} actual={})",
                    source_ordinal,
                    measure,
                    attack_start_measure,
                    row.source_midi,
                    attack_start_pitch,
                    expected_staff,
                    attack_start_staff
                ));
            }
            for fragment_index in mapped_fragment_indices.iter().copied() {
                fragments[fragment_index].attack_id = Some(attack_id);
            }
            attacks.push(CandidateMaterialAttackState {
                source_event_id,
                attack_id,
                exists: true,
                measure,
                onset: source_onset,
                sounding_end,
                pitch: row.source_midi,
                staff: expected_staff,
                voice: attack_start_voice,
            });
        } else {
            if mapped_fragment_indices
                .iter()
                .copied()
                .any(|index| fragments[index].pitch.is_some())
            {
                return Err(anyhow!(
                    "CandidateMaterialLineageSpine: restifizierte SourceOrdinal {} ist hinter VisibleRhythm noch pitched",
                    source_ordinal
                ));
            }
        }
    }

    source_events.sort_unstable();
    attacks.sort_by_key(|attack| attack.source_event_id);
    if source_events.len() != disposition.retained_event_count {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: SourceEvent-Coverage {} widerspricht retained {}",
            source_events.len(),
            disposition.retained_event_count
        ));
    }
    let expected_active = disposition
        .visible_pitched_event_count
        .saturating_add(disposition.hidden_pitched_event_count);
    if attacks.len() != expected_active {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: initiale SoundingAttack-Coverage {} widerspricht PostLegacy pitched {}",
            attacks.len(),
            expected_active
        ));
    }
    let attack_ids = attacks
        .iter()
        .map(|attack| attack.attack_id)
        .collect::<BTreeSet<_>>();
    if attack_ids.len() != attacks.len() {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: initiale SoundingAttackId ist nicht eindeutig"
        ));
    }

    let state_fingerprint_fnv64 = material_lineage_state_fingerprint(&source_events, &attacks);
    candidate.material_lineage = CandidateMaterialLineageSpine {
        authority_bound: true,
        source_event_count: source_events.len(),
        attack_count: attacks.len(),
        active_attack_count: attacks.len(),
        source_events: std::sync::Arc::from(source_events),
        attacks: std::sync::Arc::from(attacks),
        fragments: std::sync::Arc::from(fragments),
        receipts: std::sync::Arc::from(Vec::<CandidateMaterialMutationReceipt>::new()),
        state_fingerprint_fnv64,
        ambiguous_source_groups: 0,
        freeze_boundary_candidate_reached: false,
        final_sounding_material_frozen: false,
        frozen_state_fingerprint_fnv64: 0,
        frozen_active_attack_count: 0,
        frozen_projection_fingerprint_fnv64: 0,
        frozen_sounding_temporal_lane_plan: FrozenSoundingTemporalLanePlan::default(),
    };
    Ok(())
}

fn compatible_fragment_transition(
    before: &CandidateMaterialFragmentState,
    after: &CandidateMaterialFragmentState,
    declared: CandidateMaterialDeclaredEffects,
) -> bool {
    if before.measure != after.measure {
        return false;
    }
    if !declared.attack_onset && before.onset != after.onset {
        return false;
    }
    let before_pitched = before.pitch.is_some();
    let after_pitched = after.pitch.is_some();
    if !declared.attack_existence && before_pitched != after_pitched {
        return false;
    }
    // Representation fragment duration is not SoundingEnd. SourceDuration remains
    // semantic material truth until a dedicated sounding-end owner exists.
    if !declared.pitch && before.pitch != after.pitch {
        return false;
    }
    if !declared.staff && before.staff != after.staff {
        return false;
    }
    if !declared.voice && before.voice != after.voice {
        return false;
    }
    true
}

fn carry_candidate_fragment_lineage(
    before: &CandidateMaterialFragmentState,
    after: &mut CandidateMaterialFragmentState,
) -> anyhow::Result<()> {
    if after.source_event_id.is_some() || after.attack_id.is_some() {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: Ziel-RepresentationFragment besitzt bereits Lineage"
        ));
    }
    after.source_event_id = before.source_event_id;
    after.attack_id = before.attack_id;
    Ok(())
}

fn map_candidate_material_fragments_forward_with_explicit_pairs(
    before: &[CandidateMaterialFragmentState],
    after: &mut [CandidateMaterialFragmentState],
    declared: CandidateMaterialDeclaredEffects,
    explicit_pairs: &[(usize, usize)],
    allow_raw_position_transition: bool,
) -> anyhow::Result<()> {
    let mut explicit_old = BTreeSet::<usize>::new();
    let mut explicit_new = BTreeSet::<usize>::new();
    for (old_index, new_index) in explicit_pairs.iter().copied() {
        if old_index >= before.len() || new_index >= after.len() {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: explicit move carrier index outside fragment range"
            ));
        }
        if !explicit_old.insert(old_index) || !explicit_new.insert(new_index) {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED duplicate explicit move carrier"
            ));
        }
        let old_fragment = &before[old_index];
        if old_fragment.source_event_id.is_none() {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: explicit move carrier source has no forward SourceLineage"
            ));
        }
        if !compatible_fragment_transition(old_fragment, &after[new_index], declared) {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: explicit move carrier contradicts declared owner effects at Takt {} Onset {} -> {}",
                old_fragment.measure,
                old_fragment.onset,
                after[new_index].onset
            ));
        }
        let new_fragment = &after[new_index];
        if old_fragment.tie_start != new_fragment.tie_start
            || old_fragment.tie_stop != new_fragment.tie_stop
        {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED exact move carrier changed tie role at Takt {} Onset {} -> {}",
                old_fragment.measure,
                old_fragment.onset,
                new_fragment.onset
            ));
        }
        if old_fragment.onset != new_fragment.onset
            && (old_fragment.tie_start
                || old_fragment.tie_stop
                || new_fragment.tie_start
                || new_fragment.tie_stop)
        {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED exact move carrier relocates tied fragment at Takt {} Onset {} -> {}",
                old_fragment.measure,
                old_fragment.onset,
                new_fragment.onset
            ));
        }
        carry_candidate_fragment_lineage(old_fragment, &mut after[new_index])?;
    }

    let mut old_by_temporal = BTreeMap::<(usize, i64), Vec<usize>>::new();
    let mut new_by_temporal = BTreeMap::<(usize, i64), Vec<usize>>::new();
    for (index, fragment) in before.iter().enumerate() {
        if explicit_old.contains(&index) {
            continue;
        }
        old_by_temporal
            .entry(fragment.temporal_key())
            .or_default()
            .push(index);
    }
    for (index, fragment) in after.iter().enumerate() {
        if explicit_new.contains(&index) {
            continue;
        }
        new_by_temporal
            .entry(fragment.temporal_key())
            .or_default()
            .push(index);
    }

    let mut keys = old_by_temporal.keys().copied().collect::<BTreeSet<_>>();
    keys.extend(new_by_temporal.keys().copied());
    for key in keys {
        let old_indices = old_by_temporal.get(&key).cloned().unwrap_or_default();
        let new_indices = new_by_temporal.get(&key).cloned().unwrap_or_default();
        let mut old_unmatched = old_indices
            .iter()
            .copied()
            .filter(|index| before[*index].source_event_id.is_some())
            .collect::<BTreeSet<_>>();
        let mut new_unmatched = new_indices.iter().copied().collect::<BTreeSet<_>>();

        // Preserve byte-neutral fragments first through the same local representation position.
        // RawNoteIndex is only a transaction-local locator and never becomes SourceEventId.
        let old_snapshot = old_unmatched.iter().copied().collect::<Vec<_>>();
        for old_index in old_snapshot {
            let old_fragment = &before[old_index];
            let matches = new_unmatched
                .iter()
                .copied()
                .filter(|new_index| {
                    after[*new_index].raw_note_index == old_fragment.raw_note_index
                        && after[*new_index].exact_material_signature()
                            == old_fragment.exact_material_signature()
                })
                .collect::<Vec<_>>();
            if matches.len() == 1 {
                let new_index = matches[0];
                carry_candidate_fragment_lineage(old_fragment, &mut after[new_index])?;
                old_unmatched.remove(&old_index);
                new_unmatched.remove(&new_index);
            }
        }

        // Exact semantic fragments are next. This intentionally precedes the looser same-position
        // transition so an insertion/removal inside a chord cohort cannot shift RawNoteIndex and
        // steal a neighbouring SourceEventId.
        let mut exact_signatures = BTreeMap::<String, (Vec<usize>, Vec<usize>)>::new();
        for index in old_unmatched.iter().copied() {
            let signature = before[index].exact_material_signature();
            exact_signatures.entry(signature).or_default().0.push(index);
        }
        for index in new_unmatched.iter().copied() {
            let signature = after[index].exact_material_signature();
            exact_signatures.entry(signature).or_default().1.push(index);
        }
        for (_signature, (old_exact, new_exact)) in exact_signatures {
            if old_exact.len() == 1 && new_exact.len() == 1 {
                let old_index = old_exact[0];
                let new_index = new_exact[0];
                carry_candidate_fragment_lineage(&before[old_index], &mut after[new_index])?;
                old_unmatched.remove(&old_index);
                new_unmatched.remove(&new_index);
            } else if !old_exact.is_empty() && old_exact.len() == new_exact.len() {
                return Err(anyhow!(
                    "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED identische Source-gebundene Fragmente an Takt {} Onset {}",
                    key.0,
                    key.1
                ));
            }
        }

        // Attribute-only changes of the same already-existing node may use RawNoteIndex inside one
        // owner transaction. Explicit move-carrier reconciliation disables this looser shortcut.
        if allow_raw_position_transition {
            let old_snapshot = old_unmatched.iter().copied().collect::<Vec<_>>();
            for old_index in old_snapshot {
                let old_fragment = &before[old_index];
                let matches = new_unmatched
                    .iter()
                    .copied()
                    .filter(|new_index| {
                        after[*new_index].raw_note_index == old_fragment.raw_note_index
                            && compatible_fragment_transition(
                                old_fragment,
                                &after[*new_index],
                                declared,
                            )
                    })
                    .collect::<Vec<_>>();
                if matches.len() == 1 {
                    let new_index = matches[0];
                    let reverse_matches = old_unmatched
                        .iter()
                        .copied()
                        .filter(|candidate_old| {
                            before[*candidate_old].raw_note_index
                                == after[new_index].raw_note_index
                                && compatible_fragment_transition(
                                    &before[*candidate_old],
                                    &after[new_index],
                                    declared,
                                )
                        })
                        .collect::<Vec<_>>();
                    if reverse_matches.len() == 1 {
                        carry_candidate_fragment_lineage(
                            old_fragment,
                            &mut after[new_index],
                        )?;
                        old_unmatched.remove(&old_index);
                        new_unmatched.remove(&new_index);
                    }
                }
            }
        }

        // Remaining source-bound fragments may move only along dimensions explicitly declared by
        // the existing owner. Reciprocal uniqueness is mandatory; otherwise no identity is guessed.
        loop {
            let mut progress = false;
            let old_snapshot = old_unmatched.iter().copied().collect::<Vec<_>>();
            for old_index in old_snapshot {
                let candidates = new_unmatched
                    .iter()
                    .copied()
                    .filter(|new_index| {
                        compatible_fragment_transition(
                            &before[old_index],
                            &after[*new_index],
                            declared,
                        )
                    })
                    .collect::<Vec<_>>();
                if candidates.len() == 1 {
                    let new_index = candidates[0];
                    let reverse_candidates = old_unmatched
                        .iter()
                        .copied()
                        .filter(|candidate_old| {
                            compatible_fragment_transition(
                                &before[*candidate_old],
                                &after[new_index],
                                declared,
                            )
                        })
                        .collect::<Vec<_>>();
                    if reverse_candidates.len() == 1 {
                        carry_candidate_fragment_lineage(
                            &before[old_index],
                            &mut after[new_index],
                        )?;
                        old_unmatched.remove(&old_index);
                        new_unmatched.remove(&new_index);
                        progress = true;
                    }
                }
            }
            if !progress {
                break;
            }
        }

        if !old_unmatched.is_empty() {
            let unresolved = old_unmatched.iter().copied().collect::<Vec<_>>();
            for old_index in unresolved {
                let fragment = &before[old_index];
                if fragment.is_sounding_attack_fragment() && !declared.attack_existence {
                    return Err(anyhow!(
                        "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED owner transition at Takt {} Onset {}; unresolved sounding SourceFragment without AttackExistence declaration",
                        key.0,
                        key.1
                    ));
                }
                old_unmatched.remove(&old_index);
            }
        }

        // SourceEventId may remain in MaterialState with no RepresentationFragment. New pitched
        // material is forbidden unless an explicit forward carrier or unique prior fragment proved
        // its lineage at this exact owner boundary.
        for new_index in new_unmatched {
            let fragment = &after[new_index];
            if fragment.pitch.is_some() {
                return Err(anyhow!(
                    "CandidateMaterialLineageSpine: new pitched RepresentationFragment without forward SourceLineage at Takt {} Onset {} Dauer {} Pitch {:?}",
                    fragment.measure,
                    fragment.onset,
                    fragment.duration,
                    fragment.pitch
                ));
            }
        }
    }
    Ok(())
}

fn reconcile_candidate_material_lineage_with_pairs(
    candidate: &mut SolverCandidate,
    owner: &str,
    declared: CandidateMaterialDeclaredEffects,
    explicit_pairs: &[(usize, usize)],
    allow_raw_position_transition: bool,
) -> anyhow::Result<()> {
    if !candidate.material_lineage.authority_bound {
        return Ok(());
    }
    if candidate.material_lineage.final_sounding_material_frozen {
        return Err(anyhow!(
            "FinalSoundingMaterialFreeze: owner={owner} attempted CandidateMaterialState reconciliation after freeze"
        ));
    }
    let source_events = candidate.material_lineage.source_events.as_ref().to_vec();
    let before_attacks = candidate.material_lineage.attacks.as_ref().to_vec();
    let before_fragments = candidate.material_lineage.fragments.as_ref();
    let mut after_fragments = material_fragments_from_xml(candidate.layout_xml())?;
    map_candidate_material_fragments_forward_with_explicit_pairs(
        before_fragments,
        &mut after_fragments,
        declared,
        explicit_pairs,
        allow_raw_position_transition,
    )
    .with_context(|| format!("CandidateMaterialLineageSpine owner={owner}"))?;

    let source_event_set = source_events.iter().copied().collect::<BTreeSet<_>>();
    if source_event_set.len() != source_events.len() {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: SourceEventId ist nicht eindeutig"
        ));
    }

    let disposition = &candidate
        .midi_seed_stats
        .source_selection_full_causal
        .productive_post_legacy_disposition_plan;
    let source_truth_by_source = disposition
        .rows
        .iter()
        .map(|row| (MaterialSourceEventId(row.source_ordinal), *row))
        .collect::<BTreeMap<_, _>>();
    let serialization = &candidate
        .midi_seed_stats
        .source_selection_full_causal
        .midi_seed_serialized_source_identity_plan;
    let source_measure_by_source = serialization
        .rows
        .iter()
        .map(|row| (MaterialSourceEventId(row.source_ordinal), row.measure_ordinal))
        .collect::<BTreeMap<_, _>>();
    if source_truth_by_source.len() != source_events.len()
        || source_measure_by_source.len() != source_events.len()
    {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: SourceTruth-/Measure-Coverage ging hinter Initialbindung verloren"
        ));
    }

    let mut after_attacks = before_attacks.clone();
    let mut attack_index_by_source = BTreeMap::<MaterialSourceEventId, usize>::new();
    let mut used_attack_ids = BTreeSet::<SoundingAttackId>::new();
    for (index, attack) in after_attacks.iter().enumerate() {
        if attack_index_by_source
            .insert(attack.source_event_id, index)
            .is_some()
            || !used_attack_ids.insert(attack.attack_id)
        {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: bestehende AttackIdentity ist nicht eindeutig"
            ));
        }
    }

    let mut fragments_by_source = BTreeMap::<MaterialSourceEventId, Vec<usize>>::new();
    for (index, fragment) in after_fragments.iter().enumerate() {
        if let Some(source_event_id) = fragment.source_event_id {
            if !source_event_set.contains(&source_event_id) {
                return Err(anyhow!(
                    "CandidateMaterialLineageSpine: RepresentationFragment traegt unbekannte SourceEventId"
                ));
            }
            fragments_by_source
                .entry(source_event_id)
                .or_default()
                .push(index);
        }
    }

    for source_event_id in source_events.iter().copied() {
        let source_fragment_indices = fragments_by_source
            .get(&source_event_id)
            .cloned()
            .unwrap_or_default();
        if source_fragment_indices.is_empty() {
            if let Some(attack_index) = attack_index_by_source.get(&source_event_id).copied()
                && after_attacks[attack_index].exists
            {
                if !declared.attack_existence {
                    return Err(anyhow!(
                        "CandidateMaterialLineageSpine: owner {owner} removed the last sounding RepresentationFragment for SourceEventId {} without AttackExistence declaration",
                        source_event_id.0
                    ));
                }
                after_attacks[attack_index].exists = false;
            }
            continue;
        }
        let sounding_starts = source_fragment_indices
            .iter()
            .copied()
            .filter(|index| after_fragments[*index].is_sounding_attack_fragment())
            .collect::<Vec<_>>();
        if sounding_starts.len() > 1 {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: SourceEventId {} erzeugt hinter Owner {} {} SoundingAttack-Anfaenge; one-to-many ist in r666 nicht autorisiert",
                source_event_id.0,
                owner,
                sounding_starts.len()
            ));
        }

        let existing_attack_index = attack_index_by_source.get(&source_event_id).copied();
        if let Some(start_index) = sounding_starts.first().copied() {
            let attack_index = if let Some(existing_index) = existing_attack_index {
                existing_index
            } else {
                if !declared.attack_existence {
                    return Err(anyhow!(
                        "CandidateMaterialLineageSpine: owner {owner} materialisiert einen SoundingAttack ohne AttackExistence-Deklaration"
                    ));
                }
                let attack_id = sounding_attack_id_for_source(source_event_id);
                if !used_attack_ids.insert(attack_id) {
                    return Err(anyhow!(
                        "CandidateMaterialLineageSpine: neue SoundingAttackId kollidiert"
                    ));
                }
                let fragment = &after_fragments[start_index];
                let source_truth = source_truth_by_source
                    .get(&source_event_id)
                    .copied()
                    .ok_or_else(|| {
                        anyhow!(
                            "CandidateMaterialLineageSpine: neue AttackIdentity ohne SourceTruth"
                        )
                    })?;
                let source_measure = source_measure_by_source
                    .get(&source_event_id)
                    .copied()
                    .ok_or_else(|| {
                        anyhow!(
                            "CandidateMaterialLineageSpine: neue AttackIdentity ohne SourceMeasure"
                        )
                    })?;
                if fragment.measure != source_measure {
                    return Err(anyhow!(
                        "CandidateMaterialLineageSpine: owner {owner} verschob SourceEventId {} ueber eine Taktgrenze ohne AttackOnset-Authority",
                        source_event_id.0
                    ));
                }
                let semantic_onset = fragment.onset;
                let semantic_sounding_end = if declared.sounding_end {
                    semantic_onset.saturating_add(fragment.duration.max(1))
                } else {
                    semantic_onset.saturating_add(source_truth.source_len.max(1))
                };
                let attack = CandidateMaterialAttackState {
                    source_event_id,
                    attack_id,
                    exists: true,
                    measure: source_measure,
                    onset: semantic_onset,
                    sounding_end: semantic_sounding_end,
                    pitch: fragment.pitch.ok_or_else(|| {
                        anyhow!("CandidateMaterialLineageSpine: Attack-Fragment verlor Pitch")
                    })?,
                    staff: fragment.staff.clone(),
                    voice: fragment.voice.clone(),
                };
                after_attacks.push(attack);
                let new_index = after_attacks.len() - 1;
                attack_index_by_source.insert(source_event_id, new_index);
                new_index
            };
            let attack_id = after_attacks[attack_index].attack_id;
            for fragment_index in source_fragment_indices.iter().copied() {
                if let Some(existing_attack_id) = after_fragments[fragment_index].attack_id
                    && existing_attack_id != attack_id
                {
                    return Err(anyhow!(
                        "CandidateMaterialLineageSpine: opaque AttackIdentity wechselte innerhalb Owner {owner}"
                    ));
                }
                after_fragments[fragment_index].attack_id = Some(attack_id);
            }
            let fragment = &after_fragments[start_index];
            let attack = &mut after_attacks[attack_index];
            attack.exists = true;
            if declared.attack_onset {
                let preserved_sounding_duration = if declared.sounding_end {
                    None
                } else {
                    Some(
                        attack
                            .sounding_end
                            .checked_sub(attack.onset)
                            .filter(|duration| *duration > 0)
                            .ok_or_else(|| {
                                anyhow!(
                                    "CandidateMaterialLineageSpine: owner {owner} verschiebt AttackOnset bei nichtpositiver bestehender SoundingDuration"
                                )
                            })?,
                    )
                };
                attack.measure = fragment.measure;
                attack.onset = fragment.onset;
                if let Some(duration) = preserved_sounding_duration {
                    attack.sounding_end =
                        attack.onset.checked_add(duration).ok_or_else(|| {
                            anyhow!(
                                "CandidateMaterialLineageSpine: owner {owner} verschiebt AttackOnset ausserhalb des darstellbaren SoundingEnd-Bereichs"
                            )
                        })?;
                }
            }
            if declared.sounding_end {
                attack.sounding_end = fragment.onset.saturating_add(fragment.duration.max(1));
            }
            if declared.pitch {
                attack.pitch = fragment.pitch.ok_or_else(|| {
                    anyhow!("CandidateMaterialLineageSpine: Attack-Fragment verlor Pitch")
                })?;
            }
            if declared.staff {
                attack.staff = fragment.staff.clone();
            }
            if declared.voice {
                attack.voice = fragment.voice.clone();
            }
        } else if let Some(attack_index) = existing_attack_index {
            let attack_id = after_attacks[attack_index].attack_id;
            for fragment_index in source_fragment_indices.iter().copied() {
                if let Some(existing_attack_id) = after_fragments[fragment_index].attack_id
                    && existing_attack_id != attack_id
                {
                    return Err(anyhow!(
                        "CandidateMaterialLineageSpine: opaque AttackIdentity wechselte innerhalb Owner {owner}"
                    ));
                }
                after_fragments[fragment_index].attack_id = Some(attack_id);
            }
            after_attacks[attack_index].exists = false;
        }
    }

    after_attacks.sort_by_key(|attack| attack.source_event_id);
    let after_attack_ids = after_attacks
        .iter()
        .map(|attack| attack.attack_id)
        .collect::<BTreeSet<_>>();
    if after_attack_ids.len() != after_attacks.len() {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: AttackIdentity ist nach Owner nicht eindeutig"
        ));
    }

    let before_by_source = before_attacks
        .iter()
        .map(|attack| (attack.source_event_id, attack))
        .collect::<BTreeMap<_, _>>();
    let after_by_source = after_attacks
        .iter()
        .map(|attack| (attack.source_event_id, attack))
        .collect::<BTreeMap<_, _>>();

    let mut existence_changes = 0usize;
    let mut onset_changes = 0usize;
    let mut sounding_end_changes = 0usize;
    let mut pitch_changes = 0usize;
    let mut staff_changes = 0usize;
    let mut voice_changes = 0usize;
    let mut changed_source_ordinals = Vec::<usize>::new();

    for source_event_id in source_events.iter().copied() {
        let before = before_by_source.get(&source_event_id).copied();
        let after = after_by_source.get(&source_event_id).copied();
        let mut changed = false;
        match (before, after) {
            (None, None) => {}
            (None, Some(after)) => {
                if !after.exists {
                    return Err(anyhow!(
                        "CandidateMaterialLineageSpine: neu erzeugte AttackIdentity ist nicht klingend"
                    ));
                }
                existence_changes = existence_changes.saturating_add(1);
                changed = true;
            }
            (Some(_), None) => {
                return Err(anyhow!(
                    "CandidateMaterialLineageSpine: bestehende AttackIdentity wurde geloescht"
                ));
            }
            (Some(before), Some(after)) => {
                if before.attack_id != after.attack_id {
                    return Err(anyhow!(
                        "CandidateMaterialLineageSpine: opaque AttackIdentity changed inside owner {owner}"
                    ));
                }
                if before.exists != after.exists {
                    existence_changes = existence_changes.saturating_add(1);
                    changed = true;
                }
                if before.exists && after.exists {
                    if before.measure != after.measure || before.onset != after.onset {
                        onset_changes = onset_changes.saturating_add(1);
                        changed = true;
                    }
                    if before.sounding_end != after.sounding_end {
                        if declared.attack_onset && !declared.sounding_end {
                            let before_duration = before
                                .sounding_end
                                .checked_sub(before.onset)
                                .filter(|duration| *duration > 0)
                                .ok_or_else(|| {
                                    anyhow!(
                                        "CandidateMaterialLineageSpine: owner {owner} has non-positive pre-translation SoundingDuration"
                                    )
                                })?;
                            let after_duration = after
                                .sounding_end
                                .checked_sub(after.onset)
                                .filter(|duration| *duration > 0)
                                .ok_or_else(|| {
                                    anyhow!(
                                        "CandidateMaterialLineageSpine: owner {owner} has non-positive post-translation SoundingDuration"
                                    )
                                })?;
                            if before_duration != after_duration {
                                return Err(anyhow!(
                                    "CandidateMaterialLineageSpine: owner {owner} changed SoundingDuration without SoundingEnd declaration"
                                ));
                            }
                        } else {
                            sounding_end_changes = sounding_end_changes.saturating_add(1);
                            changed = true;
                        }
                    }
                    if before.pitch != after.pitch {
                        pitch_changes = pitch_changes.saturating_add(1);
                        changed = true;
                    }
                    if before.staff != after.staff {
                        staff_changes = staff_changes.saturating_add(1);
                        changed = true;
                    }
                    if before.voice != after.voice {
                        voice_changes = voice_changes.saturating_add(1);
                        changed = true;
                    }
                }
            }
        }
        if changed {
            changed_source_ordinals.push(source_event_id.0);
        }
    }

    if existence_changes > 0 && !declared.attack_existence {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: owner {owner} changed AttackExistence without declaration"
        ));
    }
    if onset_changes > 0 && !declared.attack_onset {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: owner {owner} changed AttackOnset without declaration"
        ));
    }
    if sounding_end_changes > 0 && !declared.sounding_end {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: owner {owner} changed SoundingEnd without declaration"
        ));
    }
    if pitch_changes > 0 && !declared.pitch {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: owner {owner} changed Pitch without declaration"
        ));
    }
    if staff_changes > 0 && !declared.staff {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: owner {owner} changed Staff without declaration"
        ));
    }
    if voice_changes > 0 && !declared.voice {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: owner {owner} changed Voice without declaration"
        ));
    }

    changed_source_ordinals.sort_unstable();
    changed_source_ordinals.dedup();
    let active_before = before_attacks.iter().filter(|attack| attack.exists).count();
    let active_after = after_attacks.iter().filter(|attack| attack.exists).count();
    let state_fingerprint_fnv64 =
        material_lineage_state_fingerprint(&source_events, &after_attacks);
    let receipt = CandidateMaterialMutationReceipt {
        owner: owner.to_string(),
        declared,
        active_before,
        active_after,
        existence_changes,
        onset_changes,
        sounding_end_changes,
        pitch_changes,
        staff_changes,
        voice_changes,
        changed_source_ordinals,
        state_fingerprint_fnv64,
    };
    let mut receipts = candidate.material_lineage.receipts.as_ref().to_vec();
    receipts.push(receipt);
    candidate.material_lineage.attack_count = after_attacks.len();
    candidate.material_lineage.active_attack_count = active_after;
    candidate.material_lineage.attacks = std::sync::Arc::from(after_attacks);
    candidate.material_lineage.fragments = std::sync::Arc::from(after_fragments);
    candidate.material_lineage.receipts = std::sync::Arc::from(receipts);
    candidate.material_lineage.state_fingerprint_fnv64 = state_fingerprint_fnv64;
    Ok(())
}


fn reconcile_candidate_material_lineage_with_visible_rhythm_forward_rows(
    before_xml: &str,
    candidate: &mut SolverCandidate,
    owner: &str,
    rows: &[arrangement::ArrangementRepresentationLineageRow],
    post_raw_note_counts: &[usize],
) -> anyhow::Result<()> {
    if !candidate.material_lineage.authority_bound {
        return Ok(());
    }
    if candidate.material_lineage.final_sounding_material_frozen {
        return Err(anyhow!(
            "FinalSoundingMaterialFreeze: owner={owner} attempted VisibleRhythm forward-lineage reconciliation after freeze"
        ));
    }

    let before_notes = parse_atomic_notes(before_xml);
    let before_fragments = candidate.material_lineage.fragments.as_ref();
    if before_notes.len() != before_fragments.len() {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: VisibleRhythm before-note topology disagrees with bound fragment state"
        ));
    }

    let after_fragments = material_fragments_from_xml(candidate.layout_xml())?;
    let mut after_index_by_raw = BTreeMap::<(usize, usize), usize>::new();
    let mut actual_post_counts = vec![0usize; post_raw_note_counts.len()];
    for (index, fragment) in after_fragments.iter().enumerate() {
        if fragment.measure == 0 || fragment.measure > post_raw_note_counts.len() {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: VisibleRhythm target fragment lies outside declared measure coverage: {}",
                fragment.measure
            ));
        }
        actual_post_counts[fragment.measure - 1] =
            actual_post_counts[fragment.measure - 1].saturating_add(1);
        if after_index_by_raw
            .insert((fragment.measure, fragment.raw_note_index), index)
            .is_some()
        {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: VisibleRhythm target RawNote locator is not unique"
            ));
        }
    }
    if actual_post_counts != post_raw_note_counts {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: VisibleRhythm target topology disagrees with ledger post-RawNote counts"
        ));
    }

    let mut rows_by_pre = BTreeMap::<
        (usize, usize),
        Vec<&arrangement::ArrangementRepresentationLineageRow>,
    >::new();
    let mut seen_post = BTreeSet::<(usize, usize)>::new();
    for row in rows {
        if row.measure_ordinal == 0 || row.measure_ordinal > post_raw_note_counts.len() {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: VisibleRhythm forward row has invalid measure {}",
                row.measure_ordinal
            ));
        }
        if row.post_ledger_raw_note_index >= post_raw_note_counts[row.measure_ordinal - 1] {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: VisibleRhythm forward row target index {} outside measure {} topology",
                row.post_ledger_raw_note_index,
                row.measure_ordinal
            ));
        }
        if !seen_post.insert((row.measure_ordinal, row.post_ledger_raw_note_index)) {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED VisibleRhythm forward target is duplicated at measure {} raw index {}",
                row.measure_ordinal,
                row.post_ledger_raw_note_index
            ));
        }
        rows_by_pre
            .entry((row.measure_ordinal, row.pre_ledger_raw_note_index))
            .or_default()
            .push(row);
    }

    let mut pairs = Vec::<(usize, usize)>::new();
    for (old_index, before) in before_fragments.iter().enumerate() {
        if before.source_event_id.is_none() {
            continue;
        }
        let lineage_rows = rows_by_pre
            .get(&(before.measure, before.raw_note_index))
            .ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: VisibleRhythm forward lineage missing for source-bound fragment at measure {} pre raw index {}",
                    before.measure,
                    before.raw_note_index
                )
            })?;
        if lineage_rows.len() != 1 || lineage_rows[0].fragment_ordinal != 0 {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED VisibleRhythm one-to-many source-bound split at measure {} pre raw index {} requires dedicated tie-preserving continuation lineage; target_count={}",
                before.measure,
                before.raw_note_index,
                lineage_rows.len()
            ));
        }
        let row = lineage_rows[0];
        let new_index = after_index_by_raw
            .get(&(row.measure_ordinal, row.post_ledger_raw_note_index))
            .copied()
            .ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: VisibleRhythm forward target missing at measure {} raw index {}",
                    row.measure_ordinal,
                    row.post_ledger_raw_note_index
                )
            })?;
        pairs.push((old_index, new_index));
    }

    // r689: VisibleRhythm Representation-Onset Diagnostic Audit.
    // A structural ledger row proves which representation fragment survived, but it must not
    // silently grant semantic AttackOnset authority.  If the ledger changes the parsed onset of
    // an already source-bound fragment, stop before generic reconciliation and project enough
    // evidence to decide whether this is a representation-only timeline effect or an actual
    // sounding-time mutation.
    if let Some((old_index, new_index)) = pairs
        .iter()
        .copied()
        .find(|(old_index, new_index)| {
            before_fragments[*old_index].onset != after_fragments[*new_index].onset
        })
    {
        let before = &before_fragments[old_index];
        let after = &after_fragments[new_index];
        let attack_detail = before
            .source_event_id
            .and_then(|source_event_id| {
                candidate
                    .material_lineage
                    .attacks
                    .iter()
                    .find(|attack| {
                        attack.source_event_id == source_event_id
                            && before
                                .attack_id
                                .is_none_or(|attack_id| attack.attack_id == attack_id)
                    })
            })
            .map(|attack| {
                format!(
                    "source_event_id={} attack_id={} exists={} measure={} onset={} sounding_end={} pitch={} staff={} voice={}",
                    attack.source_event_id.0,
                    attack.attack_id.0,
                    attack.exists,
                    attack.measure,
                    attack.onset,
                    attack.sounding_end,
                    attack.pitch,
                    attack.staff,
                    attack.voice
                )
            })
            .unwrap_or_else(|| "none".to_string());
        let forward_row_detail = rows_by_pre
            .get(&(before.measure, before.raw_note_index))
            .and_then(|lineage_rows| lineage_rows.first().copied())
            .map(|row| {
                format!(
                    "measure={} pre_raw={} post_raw={} fragment_ordinal={}",
                    row.measure_ordinal,
                    row.pre_ledger_raw_note_index,
                    row.post_ledger_raw_note_index,
                    row.fragment_ordinal
                )
            })
            .unwrap_or_else(|| "none".to_string());
        let unmapped_same_measure = after_fragments
            .iter()
            .filter(|fragment| {
                fragment.measure == before.measure
                    && !seen_post.contains(&(fragment.measure, fragment.raw_note_index))
            })
            .take(12)
            .map(|fragment| {
                format!(
                    "raw={} onset={} duration={} pitch={:?} staff={} voice={} visible={} chord={} tie_start={} tie_stop={}",
                    fragment.raw_note_index,
                    fragment.onset,
                    fragment.duration,
                    fragment.pitch,
                    fragment.staff,
                    fragment.voice,
                    fragment.visible,
                    fragment.is_chord,
                    fragment.tie_start,
                    fragment.tie_stop
                )
            })
            .collect::<Vec<_>>()
            .join(";");

        return Err(anyhow!(
            "CandidateMaterialLineageSpine: VisibleRhythmRepresentationOnsetAudit owner={owner} source_bound_forward_pair_changed_representation_onset; before=[measure={} raw={} onset={} duration={} pitch={:?} staff={} voice={} visible={} chord={} tie_start={} tie_stop={} source_event_id={:?} attack_id={:?}] after=[measure={} raw={} onset={} duration={} pitch={:?} staff={} voice={} visible={} chord={} tie_start={} tie_stop={}] forward_row=[{}] semantic_attack=[{}] ledger_unmapped_same_measure=[{}]",
            before.measure,
            before.raw_note_index,
            before.onset,
            before.duration,
            before.pitch,
            before.staff,
            before.voice,
            before.visible,
            before.is_chord,
            before.tie_start,
            before.tie_stop,
            before.source_event_id,
            before.attack_id,
            after.measure,
            after.raw_note_index,
            after.onset,
            after.duration,
            after.pitch,
            after.staff,
            after.voice,
            after.visible,
            after.is_chord,
            after.tie_start,
            after.tie_stop,
            forward_row_detail,
            attack_detail,
            unmapped_same_measure
        ));
    }

    reconcile_candidate_material_lineage_with_pairs(
        candidate,
        owner,
        CandidateMaterialDeclaredEffects::representation_only(),
        &pairs,
        false,
    )
}


fn reconcile_candidate_material_lineage(
    candidate: &mut SolverCandidate,
    owner: &str,
    declared: CandidateMaterialDeclaredEffects,
) -> anyhow::Result<()> {
    reconcile_candidate_material_lineage_with_pairs(candidate, owner, declared, &[], true)
}



fn preflight_candidate_material_representation_transfers(
    before_xml: &str,
    candidate: &SolverCandidate,
    carriers: &[CandidateMaterialRepresentationTransferCarrier],
) -> anyhow::Result<CandidateMaterialRepresentationTransferPreflight> {
    let before_notes = parse_atomic_notes(before_xml);
    if before_notes.len() != candidate.material_lineage.fragments.len() {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: representation transfer before-note topology disagrees with bound fragment state"
        ));
    }

    let mut before_index_by_span = BTreeMap::<(usize, usize), usize>::new();
    for (index, note) in before_notes.iter().enumerate() {
        if before_index_by_span
            .insert((note.note_start, note.note_end), index)
            .is_some()
        {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: duplicate before-note span in representation transfer carrier"
            ));
        }
    }

    let mut moved_old = BTreeSet::<usize>::new();
    let mut rows = Vec::<(usize, usize)>::with_capacity(carriers.len());
    for carrier in carriers {
        let old_index = before_index_by_span
            .get(&(carrier.before_note_start, carrier.before_note_end))
            .copied()
            .ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: representation transfer source span not found"
                )
            })?;
        if !moved_old.insert(old_index) {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED representation transfer source is duplicated"
            ));
        }

        let old_fragment = &candidate.material_lineage.fragments[old_index];
        let transfer_source_has_bound_attack = match (
            old_fragment.source_event_id,
            old_fragment.attack_id,
        ) {
            (Some(source_event_id), Some(attack_id)) => candidate
                .material_lineage
                .attacks
                .iter()
                .any(|attack| {
                    attack.source_event_id == source_event_id
                        && attack.attack_id == attack_id
                        && attack.exists
                }),
            _ => false,
        };
        let transfer_source_is_bound = transfer_source_has_bound_attack
            && (old_fragment.is_sounding_attack_fragment() || old_fragment.tie_stop);
        if !transfer_source_is_bound {
            let same_temporal_pitch = candidate
                .material_lineage
                .fragments
                .iter()
                .enumerate()
                .filter(|(_, fragment)| {
                    fragment.measure == old_fragment.measure
                        && fragment.onset == old_fragment.onset
                        && fragment.pitch == old_fragment.pitch
                })
                .map(|(index, fragment)| {
                    format!(
                        "idx={index},raw={},staff={},voice={},visible={},chord={},tie_start={},tie_stop={},source_event_id={:?},attack_id={:?}",
                        fragment.raw_note_index,
                        fragment.staff,
                        fragment.voice,
                        fragment.visible,
                        fragment.is_chord,
                        fragment.tie_start,
                        fragment.tie_stop,
                        fragment.source_event_id,
                        fragment.attack_id
                    )
                })
                .collect::<Vec<_>>()
                .join(";");
            let last_receipt_owner = candidate
                .material_lineage
                .receipts
                .last()
                .map(|receipt| receipt.owner.as_str())
                .unwrap_or("none");
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: representation transfer source has no bound attack-or-tie-continuation lineage; source_index={old_index} measure={} raw_note_index={} onset={} duration={} pitch={:?} staff={} voice={} visible={} chord={} tie_start={} tie_stop={} is_sounding_attack_fragment={} source_event_id={:?} attack_id={:?} receipt_count={} last_receipt_owner={} state_fp={:016x} carrier_target=Takt{}:Onset{}:Pitch{}:Staff{}:Voice{}:Chord{} same_temporal_pitch=[{}]",
                old_fragment.measure,
                old_fragment.raw_note_index,
                old_fragment.onset,
                old_fragment.duration,
                old_fragment.pitch,
                old_fragment.staff,
                old_fragment.voice,
                old_fragment.visible,
                old_fragment.is_chord,
                old_fragment.tie_start,
                old_fragment.tie_stop,
                old_fragment.is_sounding_attack_fragment(),
                old_fragment.source_event_id,
                old_fragment.attack_id,
                candidate.material_lineage.receipts.len(),
                last_receipt_owner,
                candidate.material_lineage.state_fingerprint_fnv64,
                carrier.after_measure,
                carrier.after_onset,
                carrier.after_pitch,
                carrier.after_staff,
                carrier.after_voice,
                carrier.after_is_chord,
                same_temporal_pitch
            ));
        }

        let after_measure = carrier.after_measure.parse::<usize>().map_err(|_| {
            anyhow!(
                "CandidateMaterialLineageSpine: representation transfer target measure is not numeric: {:?}",
                carrier.after_measure
            )
        })?;

        let preexisting_target = candidate
            .material_lineage
            .fragments
            .iter()
            .enumerate()
            .any(|(index, fragment)| {
                index != old_index
                    && fragment.measure == after_measure
                    && fragment.onset == carrier.after_onset
                    && fragment.pitch == Some(carrier.after_pitch)
                    && fragment.staff == carrier.after_staff
                    && fragment.voice == carrier.after_voice
                    && fragment.is_chord == carrier.after_is_chord
            });
        if preexisting_target {
            return Ok(CandidateMaterialRepresentationTransferPreflight::PreexistingTarget);
        }

        rows.push((old_index, after_measure));
    }

    Ok(CandidateMaterialRepresentationTransferPreflight::Eligible(rows))
}


fn reconcile_candidate_material_lineage_with_representation_transfers(
    before_xml: &str,
    candidate: &mut SolverCandidate,
    owner: &str,
    declared: CandidateMaterialDeclaredEffects,
    carriers: &[CandidateMaterialRepresentationTransferCarrier],
) -> anyhow::Result<()> {
    if !candidate.material_lineage.authority_bound {
        return Ok(());
    }
    if candidate.material_lineage.final_sounding_material_frozen {
        return Err(anyhow!(
            "FinalSoundingMaterialFreeze: owner={owner} attempted representation-transfer reconciliation after freeze"
        ));
    }
    if carriers.is_empty() {
        return reconcile_candidate_material_lineage(candidate, owner, declared);
    }

    let preflight = preflight_candidate_material_representation_transfers(
        before_xml,
        candidate,
        carriers,
    )?;
    let preflight_rows = match preflight {
        CandidateMaterialRepresentationTransferPreflight::Eligible(rows) => rows,
        CandidateMaterialRepresentationTransferPreflight::PreexistingTarget => {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED representation transfer target already existed before owner {owner}"
            ));
        }
    };
    let after_fragments = material_fragments_from_xml(candidate.layout_xml())?;

    let mut pairs = Vec::<(usize, usize)>::with_capacity(carriers.len());
    let mut moved_new = BTreeSet::<usize>::new();
    for (carrier, (old_index, after_measure)) in carriers.iter().zip(preflight_rows) {
        let matches = after_fragments
            .iter()
            .enumerate()
            .filter(|(index, fragment)| {
                !moved_new.contains(index)
                    && fragment.measure == after_measure
                    && fragment.onset == carrier.after_onset
                    && fragment.pitch == Some(carrier.after_pitch)
                    && fragment.staff == carrier.after_staff
                    && fragment.voice == carrier.after_voice
                    && fragment.is_chord == carrier.after_is_chord
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED representation transfer target count={} at Takt {} Onset {} Pitch {} Staff {} Voice {} Chord {}",
                matches.len(),
                carrier.after_measure,
                carrier.after_onset,
                carrier.after_pitch,
                carrier.after_staff,
                carrier.after_voice,
                carrier.after_is_chord
            ));
        }
        let new_index = matches[0];
        if !moved_new.insert(new_index) {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED representation transfer target is duplicated"
            ));
        }
        pairs.push((old_index, new_index));
    }

    reconcile_candidate_material_lineage_with_pairs(
        candidate,
        owner,
        declared,
        &pairs,
        true,
    )
}


fn reconcile_candidate_material_lineage_with_exact_note_moves(
    before_xml: &str,
    candidate: &mut SolverCandidate,
    owner: &str,
    declared: CandidateMaterialDeclaredEffects,
    carriers: &[CandidateMaterialExactNoteMoveCarrier],
) -> anyhow::Result<()> {
    if !candidate.material_lineage.authority_bound {
        return Ok(());
    }
    if candidate.material_lineage.final_sounding_material_frozen {
        return Err(anyhow!(
            "FinalSoundingMaterialFreeze: owner={owner} attempted exact move reconciliation after freeze"
        ));
    }
    if carriers.is_empty() {
        return reconcile_candidate_material_lineage(candidate, owner, declared);
    }

    let before_notes = parse_atomic_notes(before_xml);
    let after_notes = parse_atomic_notes(candidate.layout_xml());
    if before_notes.len() != candidate.material_lineage.fragments.len() {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: exact move carrier before-note topology disagrees with bound fragment state"
        ));
    }

    let mut before_index_by_span = BTreeMap::<(usize, usize), usize>::new();
    for (index, note) in before_notes.iter().enumerate() {
        if before_index_by_span
            .insert((note.note_start, note.note_end), index)
            .is_some()
        {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: duplicate before-note span in exact move carrier"
            ));
        }
    }
    let mut after_index_by_start = BTreeMap::<usize, usize>::new();
    for (index, note) in after_notes.iter().enumerate() {
        if after_index_by_start.insert(note.note_start, index).is_some() {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: duplicate after-note start in exact move carrier"
            ));
        }
    }

    let mut pairs = Vec::<(usize, usize)>::with_capacity(before_notes.len());
    let mut moved_old = BTreeSet::<usize>::new();
    let mut moved_new = BTreeSet::<usize>::new();
    for carrier in carriers {
        let old_index = before_index_by_span
            .get(&(carrier.before_note_start, carrier.before_note_end))
            .copied()
            .ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: exact move carrier source span not found"
                )
            })?;
        let new_index = after_index_by_start
            .get(&carrier.after_note_start)
            .copied()
            .ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: exact move carrier target start not found"
                )
            })?;
        if !moved_old.insert(old_index) || !moved_new.insert(new_index) {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED exact move carrier is not one-to-one"
            ));
        }
        pairs.push((old_index, new_index));
    }

    if before_notes.len() != after_notes.len() {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: exact move transaction changed note cardinality {} -> {}",
            before_notes.len(),
            after_notes.len()
        ));
    }
    let stable_old = (0..before_notes.len())
        .filter(|index| !moved_old.contains(index))
        .collect::<Vec<_>>();
    let stable_new = (0..after_notes.len())
        .filter(|index| !moved_new.contains(index))
        .collect::<Vec<_>>();
    if stable_old.len() != stable_new.len() {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: exact move transaction stable topology is not bijective"
        ));
    }
    for (old_index, new_index) in stable_old.into_iter().zip(stable_new) {
        let old_note = &before_notes[old_index];
        let new_note = &after_notes[new_index];
        let old_block = before_xml
            .get(old_note.note_start..old_note.note_end)
            .ok_or_else(|| anyhow!("CandidateMaterialLineageSpine: invalid stable before-note span"))?;
        let new_block = candidate
            .layout_xml()
            .get(new_note.note_start..new_note.note_end)
            .ok_or_else(|| anyhow!("CandidateMaterialLineageSpine: invalid stable after-note span"))?;
        if old_block != new_block {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: AMBIGUOUS_FAIL_CLOSED exact move transaction changed a non-moved note"
            ));
        }
        if candidate.material_lineage.fragments[old_index]
            .source_event_id
            .is_some()
        {
            pairs.push((old_index, new_index));
        }
    }

    reconcile_candidate_material_lineage_with_pairs(candidate, owner, declared, &pairs, false)
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CandidateFinalSoundingProjectionRow {
    measure: usize,
    onset: i64,
    pitch: i32,
}

fn final_sounding_projection_fingerprint(
    rows: &[CandidateFinalSoundingProjectionRow],
) -> u64 {
    let mut state = 0xcbf29ce484222325u64;
    state = material_lineage_fnv1a64_update(
        state,
        b"ETUSCHLOPF_FINAL_SOUNDING_ATTACK_PROJECTION_V1\0",
    );
    for row in rows {
        state = material_lineage_fnv1a64_update(
            state,
            &u64::try_from(row.measure).unwrap_or(u64::MAX).to_le_bytes(),
        );
        state = material_lineage_fnv1a64_update(state, &row.onset.to_le_bytes());
        state = material_lineage_fnv1a64_update(state, &row.pitch.to_le_bytes());
    }
    state
}

fn final_sounding_projection_from_material_state(
    spine: &CandidateMaterialLineageSpine,
) -> anyhow::Result<Vec<CandidateFinalSoundingProjectionRow>> {
    let mut rows = Vec::with_capacity(spine.active_attack_count);
    for attack in spine.attacks.iter().filter(|attack| attack.exists) {
        if attack.sounding_end <= attack.onset {
            return Err(anyhow!(
                "FinalSoundingMaterialFreeze: gebundener SoundingAttack besitzt keine positive SoundingDuration"
            ));
        }
        rows.push(CandidateFinalSoundingProjectionRow {
            measure: attack.measure,
            onset: attack.onset,
            pitch: attack.pitch,
        });
    }
    rows.sort_unstable();
    Ok(rows)
}

fn final_sounding_projection_from_xml(
    xml: &str,
) -> anyhow::Result<Vec<CandidateFinalSoundingProjectionRow>> {
    let fragments = material_fragments_from_xml(xml)?;
    let mut rows = fragments
        .iter()
        .filter(|fragment| fragment.is_sounding_attack_fragment())
        .map(|fragment| CandidateFinalSoundingProjectionRow {
            measure: fragment.measure,
            onset: fragment.onset,
            pitch: fragment
                .pitch
                .expect("sounding attack fragment is necessarily pitched"),
        })
        .collect::<Vec<_>>();
    rows.sort_unstable();
    Ok(rows)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CandidateMaterialForwardOnsetTranslation {
    source_event_id: MaterialSourceEventId,
    attack_id: SoundingAttackId,
    target_onset: i64,
}

fn reconcile_candidate_material_forward_onset_translations(
    candidate: &mut SolverCandidate,
    owner: &str,
    declared: CandidateMaterialDeclaredEffects,
    translations: &[CandidateMaterialForwardOnsetTranslation],
) -> anyhow::Result<()> {
    if !candidate.material_lineage.authority_bound {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: owner {owner} forward-onset reconciliation without bound authority"
        ));
    }
    if candidate.material_lineage.final_sounding_material_frozen {
        return Err(anyhow!(
            "FinalSoundingMaterialFreeze: owner={owner} attempted CandidateMaterialState reconciliation after freeze"
        ));
    }
    if !declared.attack_onset
        || declared.attack_existence
        || declared.sounding_end
        || declared.pitch
        || declared.staff
        || declared.voice
    {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: owner {owner} forward-onset reconciliation requires AttackOnset-only declaration"
        ));
    }

    let source_events = candidate.material_lineage.source_events.as_ref().to_vec();
    let before_attacks = candidate.material_lineage.attacks.as_ref().to_vec();
    let mut after_attacks = before_attacks.clone();

    let mut attack_index_by_source = BTreeMap::<MaterialSourceEventId, usize>::new();
    let mut known_attack_ids = BTreeSet::<SoundingAttackId>::new();
    for (index, attack) in after_attacks.iter().enumerate() {
        if attack_index_by_source
            .insert(attack.source_event_id, index)
            .is_some()
            || !known_attack_ids.insert(attack.attack_id)
        {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: owner {owner} existing attack identity is not unique"
            ));
        }
    }

    let mut translated_source_ids = BTreeSet::<MaterialSourceEventId>::new();
    let mut translated_attack_ids = BTreeSet::<SoundingAttackId>::new();
    for translation in translations {
        if !translated_source_ids.insert(translation.source_event_id)
            || !translated_attack_ids.insert(translation.attack_id)
        {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: owner {owner} forward-onset carrier is not one-to-one"
            ));
        }
        let attack_index = attack_index_by_source
            .get(&translation.source_event_id)
            .copied()
            .ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: owner {owner} forward-onset SourceEventId has no SoundingAttack"
                )
            })?;
        let attack = &mut after_attacks[attack_index];
        if attack.attack_id != translation.attack_id || !attack.exists {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: owner {owner} forward-onset carrier disagrees with opaque SoundingAttackId"
            ));
        }
        let duration = attack
            .sounding_end
            .checked_sub(attack.onset)
            .filter(|duration| *duration > 0)
            .ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: owner {owner} forward-onset translation has non-positive source SoundingDuration"
                )
            })?;
        if attack.onset != translation.target_onset {
            attack.onset = translation.target_onset;
            attack.sounding_end = attack.onset.checked_add(duration).ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: owner {owner} forward-onset translation overflows SoundingEnd"
                )
            })?;
        }
    }

    after_attacks.sort_by_key(|attack| attack.source_event_id);
    if before_attacks.len() != after_attacks.len() {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: owner {owner} forward-onset reconciliation changed SoundingAttack cardinality"
        ));
    }
    let after_by_source = after_attacks
        .iter()
        .map(|attack| (attack.source_event_id, attack))
        .collect::<BTreeMap<_, _>>();
    if after_by_source.len() != after_attacks.len() {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: owner {owner} forward-onset reconciliation lost unique SoundingAttack source identity"
        ));
    }

    let mut onset_changes = 0usize;
    let mut changed_source_ordinals = Vec::<usize>::new();
    for before in &before_attacks {
        let source_event_id = before.source_event_id;
        let after = after_by_source
            .get(&source_event_id)
            .copied()
            .ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: owner {owner} forward-onset reconciliation changed SoundingAttack identity"
                )
            })?;
        if before.attack_id != after.attack_id
            || before.exists != after.exists
            || before.measure != after.measure
            || before.pitch != after.pitch
            || before.staff != after.staff
            || before.voice != after.voice
        {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: owner {owner} forward-onset reconciliation changed a non-onset material invariant for SourceEventId {}",
                source_event_id.0
            ));
        }
        let before_duration = before
            .sounding_end
            .checked_sub(before.onset)
            .filter(|duration| *duration > 0)
            .ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: owner {owner} has non-positive pre-translation SoundingDuration"
                )
            })?;
        let after_duration = after
            .sounding_end
            .checked_sub(after.onset)
            .filter(|duration| *duration > 0)
            .ok_or_else(|| {
                anyhow!(
                    "CandidateMaterialLineageSpine: owner {owner} has non-positive post-translation SoundingDuration"
                )
            })?;
        if before_duration != after_duration {
            return Err(anyhow!(
                "CandidateMaterialLineageSpine: owner {owner} changed SoundingDuration during forward-onset translation"
            ));
        }
        if before.onset != after.onset {
            onset_changes = onset_changes.saturating_add(1);
            changed_source_ordinals.push(source_event_id.0);
        }
    }

    let active_before = before_attacks.iter().filter(|attack| attack.exists).count();
    let active_after = after_attacks.iter().filter(|attack| attack.exists).count();
    if active_before != active_after {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: owner {owner} changed AttackExistence during forward-onset translation"
        ));
    }

    changed_source_ordinals.sort_unstable();
    let state_fingerprint_fnv64 =
        material_lineage_state_fingerprint(&source_events, &after_attacks);
    let receipt = CandidateMaterialMutationReceipt {
        owner: owner.to_string(),
        declared,
        active_before,
        active_after,
        existence_changes: 0,
        onset_changes,
        sounding_end_changes: 0,
        pitch_changes: 0,
        staff_changes: 0,
        voice_changes: 0,
        changed_source_ordinals,
        state_fingerprint_fnv64,
    };
    let mut receipts = candidate.material_lineage.receipts.as_ref().to_vec();
    receipts.push(receipt);
    candidate.material_lineage.attacks = std::sync::Arc::from(after_attacks);
    candidate.material_lineage.receipts = std::sync::Arc::from(receipts);
    candidate.material_lineage.state_fingerprint_fnv64 = state_fingerprint_fnv64;
    candidate.material_lineage.active_attack_count = active_after;
    Ok(())
}


const FROZEN_SOUNDING_TEMPORAL_MEASURE_DIVISIONS: i64 = 384;

fn frozen_sounding_temporal_lane_plan(
    spine: &CandidateMaterialLineageSpine,
) -> anyhow::Result<FrozenSoundingTemporalLanePlan> {
    if !spine.authority_bound || !spine.final_sounding_material_frozen {
        return Err(anyhow!(
            "FrozenSoundingTemporalLanePlan: final sounding material is not frozen"
        ));
    }
    if spine.frozen_state_fingerprint_fnv64 != spine.state_fingerprint_fnv64
        || spine.frozen_active_attack_count != spine.active_attack_count
        || spine.frozen_active_attack_count == 0
    {
        return Err(anyhow!(
            "FrozenSoundingTemporalLanePlan: material freeze seal is inconsistent"
        ));
    }
    if spine.frozen_sounding_temporal_lane_plan.authority_bound
        || !spine.frozen_sounding_temporal_lane_plan.rows.is_empty()
    {
        return Err(anyhow!(
            "FrozenSoundingTemporalLanePlan: authority was already bound"
        ));
    }

    let mut cohorts = BTreeMap::<
        (u8, i64, i64),
        Vec<FrozenSoundingTemporalLaneMember>,
    >::new();
    let mut active_source_ids = BTreeSet::<MaterialSourceEventId>::new();
    let mut active_attack_ids = BTreeSet::<SoundingAttackId>::new();

    for attack in spine.attacks.iter().filter(|attack| attack.exists) {
        if !active_source_ids.insert(attack.source_event_id)
            || !active_attack_ids.insert(attack.attack_id)
        {
            return Err(anyhow!(
                "FrozenSoundingTemporalLanePlan: active sounding identity is not unique"
            ));
        }
        let staff = attack.staff.parse::<u8>().map_err(|_| {
            anyhow!(
                "FrozenSoundingTemporalLanePlan: invalid final staff {:?} for SourceEventId {}",
                attack.staff,
                attack.source_event_id.0
            )
        })?;
        if !matches!(staff, 1 | 2) {
            return Err(anyhow!(
                "FrozenSoundingTemporalLanePlan: unsupported final staff {} for SourceEventId {}",
                staff,
                attack.source_event_id.0
            ));
        }
        if attack.measure == 0
            || !(0..FROZEN_SOUNDING_TEMPORAL_MEASURE_DIVISIONS).contains(&attack.onset)
        {
            return Err(anyhow!(
                "FrozenSoundingTemporalLanePlan: invalid measure-local onset for SourceEventId {}",
                attack.source_event_id.0
            ));
        }
        let duration = attack
            .sounding_end
            .checked_sub(attack.onset)
            .filter(|duration| *duration > 0)
            .ok_or_else(|| {
                anyhow!(
                    "FrozenSoundingTemporalLanePlan: non-positive SoundingDuration for SourceEventId {}",
                    attack.source_event_id.0
                )
            })?;
        let measure_index = i64::try_from(attack.measure - 1).map_err(|_| {
            anyhow!(
                "FrozenSoundingTemporalLanePlan: measure index overflow for SourceEventId {}",
                attack.source_event_id.0
            )
        })?;
        let measure_base = measure_index
            .checked_mul(FROZEN_SOUNDING_TEMPORAL_MEASURE_DIVISIONS)
            .ok_or_else(|| {
                anyhow!(
                    "FrozenSoundingTemporalLanePlan: measure base overflow for SourceEventId {}",
                    attack.source_event_id.0
                )
            })?;
        let absolute_start = measure_base.checked_add(attack.onset).ok_or_else(|| {
            anyhow!(
                "FrozenSoundingTemporalLanePlan: absolute start overflow for SourceEventId {}",
                attack.source_event_id.0
            )
        })?;
        let absolute_end = absolute_start.checked_add(duration).ok_or_else(|| {
            anyhow!(
                "FrozenSoundingTemporalLanePlan: absolute end overflow for SourceEventId {}",
                attack.source_event_id.0
            )
        })?;
        cohorts
            .entry((staff, absolute_start, absolute_end))
            .or_default()
            .push(FrozenSoundingTemporalLaneMember {
                source_event_id: attack.source_event_id,
                attack_id: attack.attack_id,
            });
    }

    if active_source_ids.len() != spine.frozen_active_attack_count
        || active_attack_ids.len() != spine.frozen_active_attack_count
    {
        return Err(anyhow!(
            "FrozenSoundingTemporalLanePlan: active sounding identity coverage disagrees with freeze seal"
        ));
    }
    for members in cohorts.values_mut() {
        members.sort_unstable();
    }

    let mut rows = Vec::<FrozenSoundingTemporalLanePlanRow>::with_capacity(cohorts.len());
    let mut fingerprint = 0xcbf29ce484222325u64;
    fingerprint = material_lineage_fnv1a64_update(
        fingerprint,
        b"ETUSCHLOPF_FROZEN_SOUNDING_TEMPORAL_LANE_PLAN_V1\0",
    );
    fingerprint = material_lineage_fnv1a64_update(
        fingerprint,
        &spine.frozen_state_fingerprint_fnv64.to_le_bytes(),
    );
    fingerprint = material_lineage_fnv1a64_update(
        fingerprint,
        &spine.frozen_projection_fingerprint_fnv64.to_le_bytes(),
    );

    let mut assigned_attack_count = 0usize;
    let mut overlap_collision_count = 0usize;
    let mut secondary_lane_cohort_count = 0usize;
    let mut secondary_lane_attack_count = 0usize;
    let mut cohort_segment_count = 0usize;
    let mut event_segment_count = 0usize;
    let mut cross_measure_cohort_count = 0usize;
    let mut cross_measure_cohort_boundary_count = 0usize;
    let mut cross_measure_event_boundary_count = 0usize;
    let mut upper_staff_lane_count = 0usize;
    let mut lower_staff_lane_count = 0usize;

    for staff in [1u8, 2u8] {
        let mut lane_ends = Vec::<i64>::new();
        for (&(cohort_staff, absolute_start, absolute_end), members) in &cohorts {
            if cohort_staff != staff {
                continue;
            }
            let reusable_lane = lane_ends
                .iter()
                .position(|lane_end| *lane_end <= absolute_start);
            let lane_index = if let Some(index) = reusable_lane {
                if lane_ends[index] > absolute_start {
                    overlap_collision_count = overlap_collision_count.saturating_add(1);
                }
                lane_ends[index] = absolute_end;
                index
            } else {
                lane_ends.push(absolute_end);
                lane_ends.len() - 1
            };
            let lane = lane_index.saturating_add(1);

            let first_measure = absolute_start
                .div_euclid(FROZEN_SOUNDING_TEMPORAL_MEASURE_DIVISIONS);
            let last_measure = absolute_end
                .saturating_sub(1)
                .div_euclid(FROZEN_SOUNDING_TEMPORAL_MEASURE_DIVISIONS);
            let cohort_boundaries =
                usize::try_from((last_measure - first_measure).max(0)).unwrap_or(usize::MAX);
            let segments = cohort_boundaries.saturating_add(1);
            cohort_segment_count = cohort_segment_count.saturating_add(segments);
            event_segment_count = event_segment_count
                .saturating_add(segments.saturating_mul(members.len()));
            if cohort_boundaries > 0 {
                cross_measure_cohort_count = cross_measure_cohort_count.saturating_add(1);
                cross_measure_cohort_boundary_count = cross_measure_cohort_boundary_count
                    .saturating_add(cohort_boundaries);
                cross_measure_event_boundary_count = cross_measure_event_boundary_count
                    .saturating_add(cohort_boundaries.saturating_mul(members.len()));
            }
            if lane > 1 {
                secondary_lane_cohort_count = secondary_lane_cohort_count.saturating_add(1);
                secondary_lane_attack_count =
                    secondary_lane_attack_count.saturating_add(members.len());
            }

            assigned_attack_count = assigned_attack_count.saturating_add(members.len());
            fingerprint = material_lineage_fnv1a64_update(fingerprint, &[staff]);
            fingerprint = material_lineage_fnv1a64_update(
                fingerprint,
                &absolute_start.to_le_bytes(),
            );
            fingerprint =
                material_lineage_fnv1a64_update(fingerprint, &absolute_end.to_le_bytes());
            fingerprint = material_lineage_fnv1a64_update(
                fingerprint,
                &u64::try_from(lane).unwrap_or(u64::MAX).to_le_bytes(),
            );
            fingerprint = material_lineage_fnv1a64_update(
                fingerprint,
                &u64::try_from(members.len()).unwrap_or(u64::MAX).to_le_bytes(),
            );
            for member in members {
                fingerprint = material_lineage_fnv1a64_update(
                    fingerprint,
                    &u64::try_from(member.source_event_id.0)
                        .unwrap_or(u64::MAX)
                        .to_le_bytes(),
                );
                fingerprint = material_lineage_fnv1a64_update(
                    fingerprint,
                    &u64::try_from(member.attack_id.0)
                        .unwrap_or(u64::MAX)
                        .to_le_bytes(),
                );
            }

            rows.push(FrozenSoundingTemporalLanePlanRow {
                staff,
                absolute_start,
                absolute_end,
                lane,
                members: std::sync::Arc::from(members.clone()),
            });
        }
        if staff == 1 {
            upper_staff_lane_count = lane_ends.len();
        } else {
            lower_staff_lane_count = lane_ends.len();
        }
    }

    let rows_self_consistent = rows.iter().all(|row| {
        matches!(row.staff, 1 | 2)
            && row.absolute_end > row.absolute_start
            && row.lane > 0
            && !row.members.is_empty()
    });
    if overlap_collision_count != 0
        || rows.len() != cohorts.len()
        || assigned_attack_count != spine.frozen_active_attack_count
        || !rows_self_consistent
    {
        return Err(anyhow!(
            "FrozenSoundingTemporalLanePlan: exact collision-free assignment failed: rows={} cohorts={} assigned={} active={} collisions={} self_consistent={}",
            rows.len(),
            cohorts.len(),
            assigned_attack_count,
            spine.frozen_active_attack_count,
            overlap_collision_count,
            rows_self_consistent
        ));
    }

    Ok(FrozenSoundingTemporalLanePlan {
        authority_bound: true,
        active_attack_count: spine.frozen_active_attack_count,
        exact_interval_cohort_count: rows.len(),
        assigned_attack_count,
        upper_staff_lane_count,
        lower_staff_lane_count,
        overlap_collision_count,
        secondary_lane_cohort_count,
        secondary_lane_attack_count,
        cohort_segment_count,
        event_segment_count,
        cross_measure_cohort_count,
        cross_measure_cohort_boundary_count,
        cross_measure_event_boundary_count,
        frozen_state_fingerprint_fnv64: spine.frozen_state_fingerprint_fnv64,
        frozen_projection_fingerprint_fnv64: spine.frozen_projection_fingerprint_fnv64,
        fingerprint_fnv64: fingerprint,
        rows: std::sync::Arc::from(rows),
        notation_voice_pre_serialization_constraint_audit:
            crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::
                NotationVoicePreSerializationConstraintClosureAuditProjection::default(),
        actual_source_beam_binding: Default::default(),
        final_chord_structural_plan:
            crate::structural_write_authority::CanonicalFinalChordStructuralPlan::default(),
        frozen_serialized_voice_plan:
            crate::structural_write_authority::FrozenSerializedVoicePlan::default(),
    })
}

fn frozen_final_chord_original_evidence(
    spine: &CandidateMaterialLineageSpine,
) -> anyhow::Result<Vec<crate::structural_write_authority::VoiceFreeFinalChordEvidenceRow>> {
    let mut rows = Vec::new();
    for (index, fragment) in spine.fragments.iter().enumerate() {
        let Some(pitch) = fragment.pitch else {
            continue;
        };
        let source_event_id = fragment.source_event_id.ok_or_else(|| {
            anyhow!("FrozenSerializedVoiceChordConstraintCutover: pitched original fragment has no SourceEventId")
        })?;
        let attack_id = fragment.attack_id.ok_or_else(|| {
            anyhow!("FrozenSerializedVoiceChordConstraintCutover: pitched original fragment has no SoundingAttackId")
        })?;
        let staff = fragment.staff.parse::<u8>().map_err(|_| {
            anyhow!("FrozenSerializedVoiceChordConstraintCutover: original fragment has non-numeric staff")
        })?;
        rows.push(crate::structural_write_authority::VoiceFreeFinalChordEvidenceRow {
            fragment_id: index.saturating_add(1),
            source_event_id: source_event_id.0,
            attack_id: attack_id.0,
            measure: fragment.measure,
            raw_note_index: fragment.raw_note_index,
            onset: fragment.onset,
            duration: fragment.duration,
            pitch,
            staff,
            chord_syntax: fragment.is_chord,
            tie_start: fragment.tie_start,
            tie_stop: fragment.tie_stop,
        });
    }
    if rows.is_empty() {
        return Err(anyhow!(
            "FrozenSerializedVoiceChordConstraintCutover: no pitched original lineage evidence"
        ));
    }
    Ok(rows)
}

fn freeze_final_sounding_material(
    candidate: &mut SolverCandidate,
    projected_final_chord_xml: &str,
    pre_final_chord_xml: &str,
    final_chord_structural_plan: &crate::structural_write_authority::CanonicalFinalChordStructuralPlan,
    pre_serialization_occurrences: &[crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationOccurrenceRow],
    pre_serialization_occupancy: &[crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationVisibleOccupancyRow],
) -> anyhow::Result<()> {
    if !candidate.material_lineage.authority_bound {
        return Err(anyhow!(
            "FinalSoundingMaterialFreeze: freeze without bound CandidateMaterialLineage"
        ));
    }
    if !candidate.material_lineage.freeze_boundary_candidate_reached {
        return Err(anyhow!(
            "FinalSoundingMaterialFreeze: pre-FinalChord freeze-boundary candidate was not reached"
        ));
    }
    if candidate.material_lineage.final_sounding_material_frozen {
        return Err(anyhow!(
            "FinalSoundingMaterialFreeze: freeze authority may only be bound once"
        ));
    }
    let last_receipt_owner = candidate
        .material_lineage
        .receipts
        .last()
        .map(|receipt| receipt.owner.as_str());
    if last_receipt_owner != Some("FinalChordEncodingSoundingMaterialization") {
        return Err(anyhow!(
            "FinalSoundingMaterialFreeze: last material owner before freeze is not FinalChordEncodingSoundingMaterialization"
        ));
    }

    final_chord_structural_plan.validate_written_projection(projected_final_chord_xml)?;

    let expected = final_sounding_projection_from_material_state(&candidate.material_lineage)?;
    let actual = final_sounding_projection_from_xml(projected_final_chord_xml)?;
    if expected != actual {
        return Err(anyhow!(
            "FinalSoundingMaterialFreeze: planned FinalChord sounding projection differs from CandidateMaterialState; expected_attacks={} actual_attacks={} expected_fp={:016x} actual_fp={:016x}",
            expected.len(),
            actual.len(),
            final_sounding_projection_fingerprint(&expected),
            final_sounding_projection_fingerprint(&actual),
        ));
    }

    let projection_fingerprint = final_sounding_projection_fingerprint(&expected);
    candidate.material_lineage.final_sounding_material_frozen = true;
    candidate.material_lineage.frozen_state_fingerprint_fnv64 =
        candidate.material_lineage.state_fingerprint_fnv64;
    candidate.material_lineage.frozen_active_attack_count =
        candidate.material_lineage.active_attack_count;
    candidate.material_lineage.frozen_projection_fingerprint_fnv64 =
        projection_fingerprint;
    if candidate.material_lineage.frozen_state_fingerprint_fnv64
        != candidate.material_lineage.state_fingerprint_fnv64
        || candidate.material_lineage.frozen_active_attack_count
            != candidate.material_lineage.active_attack_count
        || candidate.material_lineage.frozen_projection_fingerprint_fnv64
            != projection_fingerprint
    {
        return Err(anyhow!(
            "FinalSoundingMaterialFreeze: freeze seal does not match bound material state"
        ));
    }
    let mut lane_plan = frozen_sounding_temporal_lane_plan(&candidate.material_lineage)?;
    let mut frozen_voice_attacks = Vec::<crate::structural_write_authority::FrozenVoiceAttackEvidenceRow>::new();
    for row in lane_plan.rows.iter() {
        for member in row.members.iter() {
            frozen_voice_attacks.push(
                crate::structural_write_authority::FrozenVoiceAttackEvidenceRow {
                    source_event_id: member.source_event_id.0,
                    attack_id: member.attack_id.0,
                    absolute_start: row.absolute_start,
                    absolute_end: row.absolute_end,
                    staff: row.staff,
                    technical_lane: row.lane,
                },
            );
        }
    }
    let active_attack_by_source = candidate
        .material_lineage
        .attacks
        .iter()
        .filter(|attack| attack.exists)
        .map(|attack| (attack.source_event_id, attack))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut pre_serialization_source_rows = Vec::<
        crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::
            PreSerializationSourceAttackRow,
    >::new();
    for row in lane_plan.rows.iter() {
        for member in row.members.iter() {
            let attack = active_attack_by_source.get(&member.source_event_id).copied().ok_or_else(|| {
                anyhow!(
                    "NotationVoicePreSerializationConstraintClosureAudit: frozen lane member has no active CandidateMaterial attack"
                )
            })?;
            if attack.attack_id != member.attack_id {
                return Err(anyhow!(
                    "NotationVoicePreSerializationConstraintClosureAudit: frozen lane member attack identity disagrees with CandidateMaterial"
                ));
            }
            let staff = attack.staff.parse::<u8>().map_err(|_| {
                anyhow!(
                    "NotationVoicePreSerializationConstraintClosureAudit: CandidateMaterial attack has non-numeric staff"
                )
            })?;
            let semantic_duration = attack
                .sounding_end
                .checked_sub(attack.onset)
                .filter(|duration| *duration > 0)
                .ok_or_else(|| {
                    anyhow!(
                        "NotationVoicePreSerializationConstraintClosureAudit: CandidateMaterial attack has non-positive semantic duration"
                    )
                })?;
            pre_serialization_source_rows.push(
                crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::
                    PreSerializationSourceAttackRow {
                        source_event_id: member.source_event_id.0,
                        attack_id: member.attack_id.0,
                        measure: attack.measure,
                        semantic_onset: attack.onset,
                        semantic_duration,
                        pitch_midi: attack.pitch,
                        staff,
                        absolute_start: row.absolute_start,
                        absolute_end: row.absolute_end,
                        technical_lane: row.lane,
                        pre_lane_voice: attack.voice.clone(),
                    },
            );
        }
    }
    let mut pre_serialization_constraint_audit =
        crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::
            build_notation_voice_pre_serialization_constraint_closure_audit_projection_with_beam_origin(
                projected_final_chord_xml,
                Some(pre_final_chord_xml),
                &pre_serialization_source_rows,
                pre_serialization_occurrences,
                pre_serialization_occupancy,
            );

    lane_plan.actual_source_beam_binding = crate::structural_write_authority::observe_actual_source_beam_binding(
        final_chord_structural_plan, pre_final_chord_xml,
        candidate.master_state.actual_beam_capture(),
    );
    let legacy_label_preferences = candidate
        .material_lineage
        .attacks
        .iter()
        .filter(|attack| attack.exists)
        .map(|attack| crate::structural_write_authority::LegacyVoiceLabelPreferenceRow {
            source_event_id: attack.source_event_id.0,
            label: attack.voice.clone(),
        })
        .collect::<Vec<_>>();
    let frozen_channels =
        crate::structural_write_authority::build_frozen_serialization_voice_channels_with_original_beams(
            final_chord_structural_plan,
            &frozen_voice_attacks,
            Some(&lane_plan.actual_source_beam_binding),
            &legacy_label_preferences,
        )?;
    // Report the channel plan selected by the existing canonical serialization authority.
    eprintln!(
        "FrozenChannelConsumerAllocation: {} channels={} evidence={:?}",
        frozen_channels.consumer_allocation().label(),
        frozen_channels.channel_count(),
        frozen_channels.consumer_allocation()
    );
    crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::
        append_notation_voice_materialization_counterfactual_audit(
            &mut pre_serialization_constraint_audit,
            final_chord_structural_plan,
            &pre_serialization_source_rows,
            pre_serialization_occurrences,
            &frozen_voice_attacks,
            &frozen_channels,
        );
    crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::
        append_notation_voice_metric_line_baseline_overlay_audit(
            &mut pre_serialization_constraint_audit,
            &pre_serialization_source_rows,
            pre_serialization_occurrences,
            pre_serialization_occupancy,
            &frozen_channels,
        );
    crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::
        append_notation_voice_upstream_lineage_preservation_audit(
            &mut pre_serialization_constraint_audit,
            &pre_serialization_source_rows,
            &frozen_channels,
        );
    let frozen_serialized_voice_plan =
        crate::structural_write_authority::bind_musicxml_voice_labels(
            &frozen_channels,
            &legacy_label_preferences,
        )?;
    frozen_channels.require_consumer_admission(
        final_chord_structural_plan, &frozen_serialized_voice_plan,
    )?;
    if !final_chord_structural_plan.is_bound() || !frozen_serialized_voice_plan.is_bound() {
        return Err(anyhow!(
            "FrozenSerializedVoiceChordConstraintCutover: frozen structure/voice seal did not bind"
        ));
    }
    lane_plan.notation_voice_pre_serialization_constraint_audit =
        pre_serialization_constraint_audit;
    lane_plan.final_chord_structural_plan = final_chord_structural_plan.clone();
    let original_notation_voices = OriginalNotationVoiceBindings {
        original: std::sync::Arc::from(pre_final_chord_xml),
        voices: active_attack_by_source.values().map(|attack|
            ((attack.source_event_id.0, attack.attack_id.0), attack.voice.clone())).collect(),
    };
    lane_plan.final_chord_structural_plan.bind_original_beam_voice_groups(
        pre_final_chord_xml, candidate.master_state.actual_beam_capture(), &original_notation_voices,
    );
    lane_plan.final_chord_structural_plan.bind_original_respelling_input(
        candidate.master_state.original_respelling_input(pre_final_chord_xml),
    );
    lane_plan.frozen_serialized_voice_plan = frozen_serialized_voice_plan;
    candidate.material_lineage.frozen_sounding_temporal_lane_plan = lane_plan;
    Ok(())
}

fn mark_candidate_material_freeze_boundary_candidate(
    candidate: &mut SolverCandidate,
) -> anyhow::Result<()> {
    if !candidate.material_lineage.authority_bound {
        return Err(anyhow!(
            "CandidateMaterialLineageSpine: Freeze-Boundary-Candidate ohne gebundene Lineage"
        ));
    }
    candidate.material_lineage.freeze_boundary_candidate_reached = true;
    Ok(())
}

fn project_actual_beam_source_binding_report(
    text: &mut crate::final_report::CompactedReport,
    spine: &CandidateMaterialLineageSpine,
) -> anyhow::Result<()> {
    text.project_owned_section(
        "actual_canonical_beam_source_binding",
        "ACTUAL-CANONICAL-BEAM-SOURCE-BINDING",
        "------------------------------------",
        |out| spine.frozen_sounding_temporal_lane_plan.actual_source_beam_binding.append_report_body(out),
    )
}

fn append_candidate_material_lineage_spine_body(
    out: &mut String,
    spine: &CandidateMaterialLineageSpine,
) {
    use std::fmt::Write as _;
    let _ = writeln!(
        out,
        "CandidateMaterialLineageSpineAuthority: {}",
        if spine.authority_bound { "bound" } else { "none" }
    );
    let _ = writeln!(out, "CandidateMaterialLineageDecisionAuthority: none");
    let _ = writeln!(out, "CandidateMaterialLineageWriterAuthority: none");
    let _ = writeln!(out, "CandidateMaterialLineageRepairAuthority: none");
    let _ = writeln!(
        out,
        "CandidateMaterialLineageSourceEvents: {}",
        spine.source_event_count
    );
    let _ = writeln!(
        out,
        "CandidateMaterialLineageAttackIds: {}",
        spine.attack_count
    );
    let _ = writeln!(
        out,
        "CandidateMaterialLineageActiveAttacks: {}",
        spine.active_attack_count
    );
    let _ = writeln!(
        out,
        "CandidateMaterialLineageReceipts: {}",
        spine.receipts.len()
    );
    let _ = writeln!(
        out,
        "CandidateMaterialLineageAmbiguousSourceGroups: {}",
        spine.ambiguous_source_groups
    );
    let _ = writeln!(
        out,
        "CandidateMaterialLineageStateFingerprintFNV64: {:016x}",
        spine.state_fingerprint_fnv64
    );
    let _ = writeln!(
        out,
        "CandidateMaterialFreezeBoundaryCandidateReached: {}",
        if spine.freeze_boundary_candidate_reached {
            "ja"
        } else {
            "nein"
        }
    );
    let _ = writeln!(
        out,
        "IdentityContract: SourceEventId opaque; SoundingAttackId exists only for materialized sounding attacks; VisibleRhythm pre->post structural carrier transports SourceEventId before later owners; SourceOnset/SourceDuration initialize semantic MaterialState truth; later AttackOnset/SoundingEnd changes require exact owner-local forward carriers and are never reconstructed by late semantic matching; XML order/pitch/staff/voice are not identity; receipts are forward-only."
    );
    for (index, receipt) in spine.receipts.iter().enumerate() {
        let _ = writeln!(
            out,
            "MaterialMutationReceipt#{} owner={} active={}->{} existence={} onset={} end={} pitch={} staff={} voice={} changed_sources=[{}] declared=E{}O{}D{}P{}S{}V{} state={:016x}",
            index + 1,
            receipt.owner,
            receipt.active_before,
            receipt.active_after,
            receipt.existence_changes,
            receipt.onset_changes,
            receipt.sounding_end_changes,
            receipt.pitch_changes,
            receipt.staff_changes,
            receipt.voice_changes,
            receipt
                .changed_source_ordinals
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            u8::from(receipt.declared.attack_existence),
            u8::from(receipt.declared.attack_onset),
            u8::from(receipt.declared.sounding_end),
            u8::from(receipt.declared.pitch),
            u8::from(receipt.declared.staff),
            u8::from(receipt.declared.voice),
            receipt.state_fingerprint_fnv64,
        );
    }
    let temporal = &spine.frozen_sounding_temporal_lane_plan;
    let _ = writeln!(
        out,
        "FrozenSoundingTemporalLanePlanAuthority: {}",
        if temporal.authority_bound { "bound" } else { "none" }
    );
    let _ = writeln!(
        out,
        "FrozenSoundingTemporalLanePlanCoverage: active_attacks={} cohorts={} assigned={} upper_lanes={} lower_lanes={} collisions={} secondary_lane_cohorts={} secondary_lane_attacks={}",
        temporal.active_attack_count,
        temporal.exact_interval_cohort_count,
        temporal.assigned_attack_count,
        temporal.upper_staff_lane_count,
        temporal.lower_staff_lane_count,
        temporal.overlap_collision_count,
        temporal.secondary_lane_cohort_count,
        temporal.secondary_lane_attack_count,
    );
    let _ = writeln!(
        out,
        "FrozenSoundingTemporalLanePlanSegmentation: cohort_segments={} event_segments={} cross_measure_cohorts={} cohort_boundaries={} event_boundaries={}",
        temporal.cohort_segment_count,
        temporal.event_segment_count,
        temporal.cross_measure_cohort_count,
        temporal.cross_measure_cohort_boundary_count,
        temporal.cross_measure_event_boundary_count,
    );
    let _ = writeln!(
        out,
        "FrozenSoundingTemporalLanePlanFingerprintFNV64: fnv64:{:016x}",
        temporal.fingerprint_fnv64
    );
    let _ = writeln!(
        out,
        "FrozenSoundingTemporalLanePlanFreezeSeal: state={:016x} projection={:016x} seal_consistent={}",
        temporal.frozen_state_fingerprint_fnv64,
        temporal.frozen_projection_fingerprint_fnv64,
        if temporal.authority_bound
            && temporal.frozen_state_fingerprint_fnv64 == spine.frozen_state_fingerprint_fnv64
            && temporal.frozen_projection_fingerprint_fnv64
                == spine.frozen_projection_fingerprint_fnv64
        {
            "ja"
        } else {
            "nein"
        }
    );
    let _ = writeln!(out, "FrozenSoundingTemporalLanePlanMusicMutation: nein");
    let _ = writeln!(out, "FrozenSoundingTemporalLanePlanWriterAuthority: none");
    let _ = writeln!(out, "FrozenSoundingTemporalLanePlanMusicXmlVoiceAuthority: none");
    let _ = writeln!(out, "FrozenSoundingTemporalLanePlanTieWriterAuthority: none");
    let _ = writeln!(
        out,
        "FrozenSoundingTemporalLanePlanDecision: productive_frozen_sounding_temporal_plan_bound_no_musicxml_writer_cutover"
    );
}
