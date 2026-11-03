use super::*;

// Complete accepted r1582 implementations, test-only differential reference.
impl OwnedRhythmicContinuation {
    fn validate_binding_repeated_reference(&self, xml: &str) -> Result<()> {
        if self.validated.0.load(Ordering::Relaxed) {
            ensure!(self.snapshot.as_ref() == xml, "Owned rhythm belongs to another document state");
            return Ok(());
        }
        self.validate_consolidated_origins()?;
        ensure!(self.snapshot.as_ref() == xml, "Owned rhythm belongs to another document state");
        // receive validates the original custody before storing this immutable Arc.
        // Current ancestry, coverage and representation checks remain independent.
        let physical = physical_inventory_repeated_reference(xml)?;
        ensure!(physical.len() == self.occurrences.len() + self.encoded_note_count(), "Owned rhythm has incomplete physical coverage");
        let by_range = physical.iter().map(|p| ((p.bytes.start, p.bytes.end), p)).collect::<BTreeMap<_, _>>();
        let mut identities = BTreeSet::new(); let mut ranges = BTreeSet::new();
        for occurrence in &self.occurrences {
            let origin = &occurrence.origin;
            ensure!(identities.insert(occurrence.creation) && ranges.insert((occurrence.bytes.start, occurrence.bytes.end)),
                "Duplicate owned rhythmic identity or occurrence");
            ensure!(self.history.get(&occurrence.creation) == Some(origin), "Missing owner-bound ancestry");
            let physical = by_range.get(&(occurrence.bytes.start, occurrence.bytes.end))
                .ok_or_else(|| anyhow!("Owner continuation is not an entire physical note"))?;
            ensure!(physical.interval == origin.interval && physical.staff == occurrence.staff,
                "Owner continuation changed licensed support or presentation without a transition");
            self.validate_received_origin_path(occurrence.creation)?;
        }
        self.validate_encoded_repeated_reference(xml, &mut ranges, &mut identities)?;
        // The union, rather than an interval hull, must cover each received
        // obligation. A transfer explicitly represents both its rest and pitch.
        let support = self.support_by_root()?;
        for (id, root) in self.history.iter().filter(|(_, o)| o.parent.is_none()) {
            let mut intervals = support.get(id).cloned().unwrap_or_default();
            intervals.sort_by_key(|r| (r.start, r.end));
            let mut cursor = root.interval.start;
            for span in intervals {
                ensure!(span.start <= cursor && span.end <= root.interval.end, "Lost or extended origin support");
                cursor = cursor.max(span.end);
            }
            ensure!(cursor == root.interval.end, "Owner continuation lost a received obligation");
        }
        self.validated.0.store(true, Ordering::Relaxed);
        Ok(())
    }
    fn validate_encoded_repeated_reference(&self, xml: &str, used: &mut BTreeSet<(usize, usize)>,
        identities: &mut BTreeSet<RhythmicCreationIdentity>) -> Result<()> {
        let physical = physical_inventory_repeated_reference(xml)?.into_iter().map(|note|
            ((note.bytes.start, note.bytes.end), note)).collect::<BTreeMap<_, _>>();
        let cursors = forwards(xml)?;
        let mut emission_identities = BTreeSet::new();
        for encoded in &self.encoded {
            let old = &encoded.received;
            ensure!(encoded.emission_count == encoded.fragments.len()
                && encoded.fragments.iter().map(|fragment| fragment.ordinal).collect::<BTreeSet<_>>() ==
                    (0..encoded.emission_count).collect(), "Original emission ordinal drift");
            ensure!(encoded.original_emissions == encoded.fragments.iter()
                .map(|fragment| fragment.original_emissions.clone()).collect::<Vec<_>>()
                && encoded.fragments.iter().all(|fragment| !fragment.original_emissions.is_empty()
                    && emission_identities.insert((old.creation, fragment.original_emissions.clone()))),
                "Original emission execution reused or changed");
            ensure!(identities.insert(old.creation) && self.history.get(&old.creation) == Some(&old.origin),
                "Encoded occurrence lost or reused its original ancestry");
            self.validate_received_origin_path(old.creation)?;
            for emission in encoded.original_emissions.iter().flatten() {
                if let EncodingExecution::FinalChord(execution) = emission.execution {
                    ensure!(self.final_chord_execution == Some(execution), "FinalChord execution epoch was reset or substituted");
                }
            }
            let spans = encoded.fragments.iter().map(|fragment| {
                fragment.correspondence.validate()?;
                ensure!(fragment.correspondence.image == fragment.interval, "Partial carrier lacks certified partition");
                Ok(fragment.correspondence.origin.clone())
            }).collect::<Result<Vec<_>>>()?;
            exact_partition(spans, &old.origin.interval)?;
            for fragment in &encoded.fragments {
                ensure!(used.insert((fragment.bytes.start, fragment.bytes.end)), "Duplicate encoded output");
                if fragment.forward {
                    ensure!(cursors.get(&(fragment.bytes.start, fragment.bytes.end)) ==
                        Some(&(fragment.interval.clone(), Some(old.staff))), "Rest obligation cursor drift");
                } else {
                    let note = physical.get(&(fragment.bytes.start, fragment.bytes.end))
                        .ok_or_else(|| anyhow!("Encoded note is no longer an entire note"))?;
                    ensure!(note.interval == fragment.interval && note.staff == old.staff
                        && note.text.contains("<rest") == fragment.rest
                        && note.text.contains("<chord") == fragment.chord
                        && tie_marks(&note.text) == fragment.ties, "Encoded support or disposition drift");
                }
            }
        }
        Ok(())
    }
}

fn outcome(value: Result<()>) -> std::result::Result<(), String> {
    value.map_err(|error| format!("{error:#}"))
}

fn compare(state: &OwnedRhythmicContinuation, xml: &str) {
    let shared = state.clone();
    let repeated = state.clone();
    assert_eq!(outcome(shared.validate_binding(xml)),
        outcome(repeated.validate_binding_repeated_reference(xml)));
}

#[test]
fn readback_reuse_preserves_complete_validation_and_exact_failures() {
    let initial = OwnedRhythmicContinuation::synthetic_pitched_received_for_test(
        &[(31, 0, 1, 60), (32, 0, 1, 64), (33, 96, 2, 48), (34, 720, 2, 52)], 2);
    let states = [initial.clone(), initial.original_final_chord_for_test(),
        OwnedRhythmicContinuation::original_forward_obligations_for_test(false, 2),
        OwnedRhythmicContinuation::original_forward_obligations_for_test(true, 2)];
    for state in states {
        compare(&state, state.document());
        compare(&state, &(state.document().to_owned() + " "));
        let mut missing = state.clone(); missing.history.clear(); compare(&missing, state.document());
        let mut lost = state.clone(); lost.occurrences.pop(); compare(&lost, state.document());
        for index in 0..state.encoded.len() {
            let mut drift = state.clone(); drift.encoded[index].emission_count += 1;
            compare(&drift, state.document());
            let mut drift = state.clone(); drift.encoded[index].fragments[0].bytes.start += 1;
            compare(&drift, state.document());
            let mut duplicate = state.clone(); duplicate.encoded.push(duplicate.encoded[index].clone());
            compare(&duplicate, state.document());
        }
        let accepted = state.clone();
        accepted.validate_binding(state.document()).unwrap();
        assert!(accepted.validate_binding(&(state.document().to_owned() + " ")).is_err());
    }
}

#[test]
fn readback_reuse_compares_full_validator_cost_on_synthetic_scores() {
    for measures in [8usize, 32, 96] {
        let rows = (0..measures).flat_map(|m| [
            (m * 4, m as i64 * 384, 1, 60),
            (m * 4 + 1, m as i64 * 384, 1, 64),
            (m * 4 + 2, m as i64 * 384 + 96, 2, 48),
            (m * 4 + 3, m as i64 * 384 + 288, 2, 52),
        ]).collect::<Vec<_>>();
        let state = OwnedRhythmicContinuation::synthetic_pitched_received_for_test(&rows, measures);
        let mut repeated = Vec::new(); let mut shared = Vec::new();
        for turn in 0..6 {
            let before = state.clone(); let after = state.clone();
            let mut old = || {
                let clock = std::time::Instant::now();
                std::hint::black_box(before.validate_binding_repeated_reference(state.document())).unwrap();
                repeated.push(clock.elapsed().as_micros());
            };
            let mut new = || {
                let clock = std::time::Instant::now();
                std::hint::black_box(after.validate_binding(state.document())).unwrap();
                shared.push(clock.elapsed().as_micros());
            };
            if turn % 2 == 0 { old(); new(); } else { new(); old(); }
        }
        repeated.sort(); shared.sort();
        eprintln!("ReadbackReuseProbe measures={measures} notes={} repeated_median_us={} shared_median_us={} optimized_build_not_claimed=true", state.occurrences.len(), repeated[3], shared[3]);
    }
}

// Exact accepted r1582 readers, including per-note metric extraction.
fn physical_inventory_repeated_reference(xml: &str) -> Result<Vec<PhysicalOccurrence>> {
    let mut offset = 0i64;
    let mut context = RhythmContext::default();
    let mut inventory = Vec::new();
    for span in find_measure_spans(xml) {
        let measure = &xml[span.start..span.end];
        context = context.update_from_measure(measure);
        let notes = collect_raw_notes(measure);
        let metas = collect_note_metas_repeated_reference(measure, &notes);
        ensure!(notes.len() == metas.len(), "Incomplete rhythmic physical readback");
        for (note, meta) in notes.into_iter().zip(metas) {
            let staff = extract_first_i64(&note.text, "staff");
            ensure!(matches!(staff, Some(1 | 2)) && (meta.duration > 0
                || (note.text.contains("<grace") && !note.text.contains("<duration>"))),
                "Unsupported explicit staff or rhythmic duration");
            inventory.push(PhysicalOccurrence {
                bytes: span.start + note.start..span.start + note.end,
                interval: offset + meta.start..offset + meta.start + meta.duration,
                staff: staff.unwrap() as u8, text: note.text,
            });
        }
        offset = offset.checked_add(context.expected_duration())
            .ok_or_else(|| anyhow!("Rhythmic domain overflow"))?;
    }
    ensure!(inventory.len() == collect_raw_notes(xml).len(), "Physical readback omitted an occurrence");
    Ok(inventory)
}
    fn collect_note_metas_repeated_reference(measure_text: &str, raw_notes: &[RawNote]) -> Vec<NoteMeta> {
        let mut infos = Vec::new();
        let mut cursor = 0_i64;
        let mut last_non_chord_start = 0_i64;
        let mut note_pos = 0_usize;
        let mut scan = 0_usize;
        loop {
            let next_note = measure_text[scan..].find("<note").map(|p| scan + p);
            let next_backup = measure_text[scan..].find("<backup").map(|p| scan + p);
            let next_forward = measure_text[scan..].find("<forward").map(|p| scan + p);
            let mut candidates = Vec::new();
            if let Some(p) = next_note { candidates.push((p, "note")); }
            if let Some(p) = next_backup { candidates.push((p, "backup")); }
            if let Some(p) = next_forward { candidates.push((p, "forward")); }
            candidates.sort_by_key(|(p, _)| *p);
            let Some((pos, kind)) = candidates.first().copied() else { break; };
            match kind {
                "backup" => {
                    if let Some(end) = measure_text[pos..].find("</backup>").map(|e| pos + e + "</backup>".len()) {
                        let segment = &measure_text[pos..end];
                        cursor -= extract_first_i64(segment, "duration").unwrap_or(0).max(0);
                        scan = end;
                    } else {
                        break;
                    }
                }
                "forward" => {
                    if let Some(end) = measure_text[pos..].find("</forward>").map(|e| pos + e + "</forward>".len()) {
                        let segment = &measure_text[pos..end];
                        cursor += extract_first_i64(segment, "duration").unwrap_or(0).max(0);
                        scan = end;
                    } else {
                        break;
                    }
                }
                "note" => {
                    let Some(raw) = raw_notes.get(note_pos) else { break; };
                    let raw_idx = note_pos;
                    note_pos += 1;
                    let duration = extract_first_i64(&raw.text, "duration").unwrap_or(0).max(0);
                    let is_chord = raw.text.contains("<chord") || raw.text.contains("<chord/");
                    let is_rest = raw.text.contains("<rest") || raw.text.contains("<rest/");
                    let start = if is_chord { last_non_chord_start } else { cursor };
                    let staff = extract_first_text(&raw.text, "staff")
                        .and_then(|s| match s.trim() {
                            "1" => Some(PianoStaff::Right),
                            "2" => Some(PianoStaff::Left),
                            _ => None,
                        })
                        .or_else(|| staff_from_pitch(&raw.text))
                        .unwrap_or(PianoStaff::Right);
                    let voice = extract_first_text(&raw.text, "voice").unwrap_or_else(|| staff.default_voice().to_string());
                    let midi = midi_from_note_text(&raw.text);
                    infos.push(NoteMeta {
                        metric: RhythmContext::default().update_from_measure(measure_text),
                        raw_idx,
                        start,
                        duration,
                        staff,
                        voice,
                        is_rest,
                        is_chord,
                        midi,
                    });
                    if !is_chord {
                        last_non_chord_start = start;
                        cursor += duration;
                    }
                    scan = raw.end;
                }
                _ => break,
            }
        }
        infos
    }

#[test]
fn measure_metric_reuse_matches_every_original_note_meta_field() {
    for attributes in ["", "<attributes><divisions>24</divisions><time><beats>6</beats><beat-type>8</beat-type></time></attributes>",
        "<attributes><divisions>0</divisions><time><beats>-2</beats><beat-type>0</beat-type></time></attributes>",
        "<attributes><divisions>invalid</divisions><time><beats>3</beats><beat-type>4</beat-type></time></attributes>"] {
        for count in [0, 1, 4, 24] {
            let mut text = format!("<measure>{attributes}");
            for i in 0..count {
                text.push_str(&format!("<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>{}</voice><staff>{}</staff></note>", i % 3 + 1, i % 2 + 1));
                text.push_str("<backup><duration>24</duration></backup><forward><duration>12</duration></forward>");
            }
            text.push_str("</measure>");
            let notes = collect_raw_notes(&text);
            assert_eq!(format!("{:?}", collect_note_metas(&text, &notes)),
                format!("{:?}", collect_note_metas_repeated_reference(&text, &notes)));
        }
    }
}
