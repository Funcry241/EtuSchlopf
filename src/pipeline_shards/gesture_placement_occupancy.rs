// Read-only seed binding and alternative-local simulation for the existing
// same-measure gesture placers. No source/attack identity is minted here.
#[derive(Clone)]
struct GesturePlacementOccupancy {
    notes: Vec<AtomicNote>,
    by_span: BTreeMap<(usize, usize), usize>,
    independent: Vec<bool>,
    core: coincidence_occupancy::CoincidenceOccupancy<MaterialSourceEventId, SoundingAttackId>,
}

impl GesturePlacementOccupancy {
    fn from_seed(seed: &SolverCandidate) -> Option<Self> {
        if !seed.material_lineage.authority_bound
            || !seed
                .midi_seed_stats
                .source_selection_full_causal
                .productive_plan_authority
            || seed.midi_seed_stats.notation_time.source_domain.divisions <= 0
        {
            return None;
        }
        let raw = seed
            .midi_seed_stats
            .source_selection_full_causal
            .shadow_rows
            .iter()
            .map(|row| {
                (
                    MaterialSourceEventId(row.source_ordinal),
                    coincidence_occupancy::RawPair {
                        onset: row.pos,
                        pitch: row.midi,
                    },
                )
            })
            .collect::<Vec<_>>();
        Self::from_bound_parts(
            seed.layout_xml(),
            &seed.material_lineage.fragments,
            &seed.material_lineage.attacks,
            raw,
        )
    }

    fn from_bound_parts(
        xml: &str,
        bound: &[CandidateMaterialFragmentState],
        attacks: &[CandidateMaterialAttackState],
        raw: Vec<(MaterialSourceEventId, coincidence_occupancy::RawPair)>,
    ) -> Option<Self> {
        let document = roxmltree::Document::parse(xml).ok()?;
        if document
            .root_element()
            .children()
            .filter(|n| n.has_tag_name("part"))
            .count()
            != 1
        {
            return None;
        }
        if document
            .descendants()
            .filter(|n| n.has_tag_name("divisions"))
            .any(|n| n.text().and_then(|v| v.trim().parse::<i64>().ok()) != Some(96))
        {
            return None;
        }
        let notes = parse_atomic_notes(xml);
        let mut fresh = material_fragments_from_xml(xml).ok()?;
        if fresh.len() != bound.len() || fresh.len() != notes.len() {
            return None;
        }
        for (observed, carried) in fresh.iter_mut().zip(bound.iter()) {
            observed.source_event_id = carried.source_event_id;
            observed.attack_id = carried.attack_id;
            if observed != carried {
                return None;
            }
        }
        let mut attack_sources = BTreeMap::new();
        for attack in attacks.iter().filter(|row| row.exists) {
            if attack_sources
                .insert(attack.attack_id, attack.source_event_id)
                .is_some()
            {
                return None;
            }
        }
        let mut by_span = BTreeMap::new();
        let mut independent = Vec::new();
        let mut rows = Vec::new();
        for (index, (note, fragment)) in notes.iter().zip(bound.iter()).enumerate() {
            if by_span
                .insert((note.note_start, note.note_end), index)
                .is_some()
            {
                return None;
            }
            let block = xml.get(note.note_start..note.note_end)?;
            independent.push(single_note_gesture_support_is_independent(block));
            let role = if fragment.pitch.is_none() {
                coincidence_occupancy::Role::NonPitched
            } else if fragment.is_grace {
                coincidence_occupancy::Role::Grace
            } else {
                // Missing IDs never establish that a pitched fragment is a
                // continuation. A carried source/attack relation is mandatory.
                match (fragment.source_event_id, fragment.attack_id) {
                    (Some(source), Some(attack))
                        if attack_sources.get(&attack) == Some(&source) =>
                    {
                        if fragment.tie_stop {
                            coincidence_occupancy::Role::ProvenContinuation
                        } else {
                            coincidence_occupancy::Role::RegularAttack
                        }
                    }
                    _ => coincidence_occupancy::Role::Unknown,
                }
            };
            if role == coincidence_occupancy::Role::Unknown {
                return None;
            }
            let placed = fragment
                .pitch
                .map(|pitch| coincidence_occupancy::PlacedPair {
                    measure: fragment.measure,
                    onset: fragment.onset,
                    pitch,
                });
            rows.push(coincidence_occupancy::Occurrence {
                source: fragment.source_event_id,
                attack: fragment.attack_id,
                role,
                placed,
            });
        }
        let core =
            coincidence_occupancy::CoincidenceOccupancy::from_observations(true, raw, rows).ok()?;
        Some(Self {
            notes,
            by_span,
            independent,
            core,
        })
    }

    fn proposal(
        &self,
        tone: &HiddenChordToneRecoveryCandidate,
        slot: &VisibleChordToneSwapSlot,
    ) -> Option<(usize, coincidence_occupancy::PlacedPair)> {
        let &index = self.by_span.get(&(tone.abs_start, tone.abs_end))?;
        let &anchor_index = self.by_span.get(&(slot.note_start, slot.note_end))?;
        if index == anchor_index || !self.independent[index] {
            return None;
        }
        let source = &self.notes[index];
        let anchor = &self.notes[anchor_index];
        let pitch = source.pitch.as_ref()?.midi;
        if source.measure.parse::<usize>().ok()? != tone.measure
            || source.start != tone.start_tick
            || source.duration != tone.duration
            || source.staff != tone.staff.to_string()
            || source.voice != tone.voice
            || Some(pitch) != tone.midi
            || anchor.measure.parse::<usize>().ok()? != slot.measure
            || slot.measure != tone.measure
            || anchor.start != slot.start_tick
            || anchor.staff != slot.staff.to_string()
            || anchor.voice != slot.voice
            || anchor.pitch.as_ref().map(|p| p.midi) != slot.midi
            || slot.midi.is_none()
        {
            return None;
        }
        // The existing serializer writes slot.duration.max(1), not necessarily
        // the duration of the atomic chord member behind which it inserts.
        slot.start_tick.checked_add(slot.duration.max(1))?;
        Some((
            index,
            coincidence_occupancy::PlacedPair {
                measure: slot.measure,
                onset: slot.start_tick,
                pitch,
            },
        ))
    }

    fn permits(
        &self,
        tone: &HiddenChordToneRecoveryCandidate,
        slot: &VisibleChordToneSwapSlot,
    ) -> bool {
        self.proposal(tone, slot)
            .is_some_and(|(index, placed)| self.core.allows(index, placed).is_ok())
    }

    fn advance(
        &mut self,
        tone: &HiddenChordToneRecoveryCandidate,
        slot: &VisibleChordToneSwapSlot,
        gauss: bool,
    ) -> bool {
        let Some((index, placed)) = self.proposal(tone, slot) else {
            return false;
        };
        if gauss && !gauss_destination_is_available(tone, slot, &self.notes) {
            return false;
        }
        if self.core.move_to(index, placed).is_err() {
            return false;
        }
        let note = &mut self.notes[index];
        note.measure = slot.measure.to_string();
        note.staff = slot.staff.to_string();
        note.voice = slot.voice.clone();
        note.start = placed.onset;
        note.duration = slot.duration.max(1);
        // The point kernel and the A7 physical interval view advance together.
        self.core.occurrences()[index].placed == Some(placed)
    }
}

fn consensus_guarded_gauss_anchor<'a>(
    tone: &HiddenChordToneRecoveryCandidate,
    slots: &'a [VisibleChordToneSwapSlot],
    used: &BTreeSet<(usize, usize, String, i64)>,
    ordinal: usize,
    occupancy: &GesturePlacementOccupancy,
) -> Option<&'a VisibleChordToneSwapSlot> {
    let indices = slots
        .iter()
        .enumerate()
        .filter(|(_, slot)| occupancy.permits(tone, slot))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let admitted = indices
        .iter()
        .map(|&index| slots[index].clone())
        .collect::<Vec<_>>();
    // Original distance/half-cell/rank/tie policy and unchanged A7 are reused.
    let chosen =
        consensus_gauss_anchor_for_tone(tone, &admitted, used, ordinal, Some(&occupancy.notes))?;
    let local = admitted
        .iter()
        .position(|slot| std::ptr::eq(slot, chosen))?;
    Some(&slots[indices[local]])
}

fn consensus_guarded_broken_anchor<'a>(
    tone: &HiddenChordToneRecoveryCandidate,
    slots: &'a [VisibleChordToneSwapSlot],
    used: &BTreeSet<(usize, usize, String, i64)>,
    mode: usize,
    ordinal: usize,
    occupancy: &GesturePlacementOccupancy,
) -> Option<&'a VisibleChordToneSwapSlot> {
    let indices = slots
        .iter()
        .enumerate()
        .filter(|(_, slot)| occupancy.permits(tone, slot))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let admitted = indices
        .iter()
        .map(|&index| slots[index].clone())
        .collect::<Vec<_>>();
    let chosen = consensus_broken_anchor_for_tone(tone, &admitted, used, mode, ordinal)?;
    let local = admitted
        .iter()
        .position(|slot| std::ptr::eq(slot, chosen))?;
    Some(&slots[indices[local]])
}

#[cfg(test)]
#[path = "../../tests/unit/gesture_placement.rs"]
mod gesture_placement_tests;
