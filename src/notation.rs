// v0.10.1521 pipeline-owned notation support module.
// No musical behavior changes; the module now shares the pipeline owner explicitly.

    use crate::clef_readability_observation::{
        observe_lower_staff_octave_display_shift, observe_pitch, ClefPitchObservation, PianoClef,
        LowerChordObservation,
    };
    use anyhow::Context;
    use crate::arrangement::representation_transport::{RepresentationCapture, RepresentationStep, RhythmicRepresentationTransport};
    use crate::master_state_identity::PreservingTransformationReceipt;
    use std::collections::{BTreeMap, BTreeSet};
    
    #[derive(Debug, Clone, Default)]
    pub struct PolishStats {
        pub beam_groups: usize,
        pub beamed_notes: usize,
        pub removed_beams: usize,
        pub blocked_beam_groups: usize,
        pub blocked_beam_notes: usize,
        pub partial_beat_post_rest_beam_groups: usize,
        pub partial_beat_post_rest_beam_notes: usize,
        pub lower_staff_clef_beam_boundary_splits: usize,
        pub lower_staff_clef_beam_boundary_notes: usize,
    }
    
    #[derive(Debug)]
    pub struct PolishResult {
        pub(crate) rhythmic_transport: RhythmicRepresentationTransport,
        pub xml: String,
        pub stats: PolishStats,
        /// PRESERVING receipt (E051), minted by this owner only if the
        /// conserved musical content of input and output is identical.
        pub(crate) preserving_receipt: Option<crate::master_state_identity::PreservingTransformationReceipt>,
    }

    /// Sealed mint capability of `polish_musicxml` (PRESERVING E051).
    pub(crate) struct NotationPolishPreservingMint {
        _sealed: (),
    }

    /// Sealed mint capability of `decompose_lower_staff_clef_islands` (PRESERVING E052).
    pub(crate) struct NotationClefIslandPreservingMint {
        _sealed: (),
    }
    
    #[derive(Debug, Clone)]
    struct MeasureSpan {
        start: usize,
        end: usize,
    }
    
    #[derive(Debug, Clone)]
    struct NoteInfo {
        index: usize,
        start: i64,
        duration: i64,
        staff: String,
        voice: String,
        note_type: String,
        is_rest: bool,
        is_chord: bool,
        is_hidden: bool,
    }
    
    #[derive(Debug, Clone)]
    struct RawNote {
        start: usize,
        end: usize,
        text: String,
    }
    
    #[derive(Debug, Clone)]
    struct MeasurePolish {
        edits: Vec<(std::ops::Range<usize>, String)>,
        text: String,
        divisions: i64,
        stats: PolishStats,
    }
    
    #[derive(Debug, Clone)]
    pub enum LowerStaffClefReadbackScope {
        BeamSafety,
        BeamSafetyCaptured(RepresentationCapture),
        FinalPdfSource,
    }

    /// Notensatz-Politur: baut ein kleines NotationModel über dem MusicXML-Text
    /// und ergänzt metrisch lesbare Balkenangaben für kurze Noten.
    ///
    /// v0.10.216 MetricBeamPolicy nach Notensatz-Feedback:
    /// - Balkengruppen beginnen an Zählzeiten.
    /// - Wenn eine Zählzeit mit sichtbarer Pause beginnt, darf die anschließende
    ///   beambare Rest-Zählzeitgruppe als PartialBeatPostRestBeamingRule
    ///   verbunden werden.
    /// - Sechzehntel werden bevorzugt innerhalb einer Zählzeit gruppiert.
    /// - Balken überdecken keine Schlaggrenzen.
    /// - Für 16tel/32tel werden auch Doppel-/Mehrfachbalken gesetzt.
    /// - LowerStaffClefIslandDecompositionRule: Im unteren System werden sichtbare
    ///   Balkengruppen an einer echten Schlüsselgrenze getrennt. Die historische
    ///   Same-Measure-Staffprojektion kurzer Schlüsselinseln ist pensioniert;
    ///   die finale F4/G2-Entscheidung gehört dem zentralen ClefReadability-Kurs.
    ///
    /// Wichtig: Dies ist keine neue musikalische Reparatur. Dauern, Tonhöhen,
    /// Stimmen, Hände und Staff-Zuordnung bleiben unverändert. Ab v0.10.1778
    /// trennt die Politur drei Beam-Zustaende: positive eigene `BeamPlan`-
    /// Wahrheit wird kanonisch materialisiert; strukturell vollstaendig
    /// gebundene fremde Beam-Wahrheit bleibt byteidentisch; nur ungebundene
    /// `begin`/`continue`/`end`-Fragmente innerhalb derselben Staff/Voice/Beat-
    /// Domain werden als explizit negative, strukturell ungueltige Wahrheit
    /// entfernt. Ein fehlender positiver Plan allein erteilt kein Write-Recht.
    pub fn polish_musicxml(xml: &str) -> anyhow::Result<PolishResult> {
        let _origin = crate::audit_only::panic_origin_observation::enter("NotationPolish", xml, None);
        let lineage_pre_xml = xml;
        // v0.10.1481 PostCleanupLedgerBoundedLowerStaffClefReadback:
        // Vor der finalen Balkenpolitur wird der bereits vorhandene untere
        // Schlüsselzustand ausschließlich dort ergänzt, wo eine sichtbare
        // Staff-2-Note im aktiven Schlüssel mehr als drei Hilfslinien benötigen
        // würde und der alternative Klavierschlüssel die gesamte Note bzw. den
        // gesamten Akkord mit höchstens drei Hilfslinien darstellt. Dadurch
        // sieht die nachfolgende Balkenpolitur die echte Schlüsselgrenze und
        // kann eine Gruppe dort korrekt trennen. Pitch, Onset, Dauer, Voice und
        // Staff bleiben bytegleich.
        let first_capture = RepresentationCapture::default();
        let (clef_readback_xml, clef_receipt) = ledger_bounded_lower_staff_clef_readback(
            xml,
            LowerStaffClefReadbackScope::BeamSafetyCaptured(first_capture.clone()),
        );
        let first_transport = first_capture.take()?;
        let mut internal = crate::master_state_identity::RenderTransitionReceipts::new();
        internal.push_preserving(clef_receipt, &clef_readback_xml);
        internal.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);

        let xml = clef_readback_xml.as_str();
        let spans = find_measure_spans(xml);
        if spans.is_empty() {
            let preserving_receipt =
                crate::master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(
                    NotationPolishPreservingMint { _sealed: () },
                    lineage_pre_xml,
                    xml,
                );
            let rhythmic_transport = RhythmicRepresentationTransport::polish(
                &NotationPolishPreservingMint { _sealed: () }, vec![first_transport]);
            let preserving_receipt = preserving_receipt.map(|receipt|
                receipt.with_recorded_representation(rhythmic_transport.clone()));
            return Ok(PolishResult {
                rhythmic_transport,
                xml: xml.to_string(),
                stats: PolishStats::default(),
                preserving_receipt,
            });
        }
    
        let mut out = String::with_capacity(xml.len() + 1024);
        let mut last = 0_usize;
        let mut divisions = 1_i64;
        let mut total = PolishStats::default();
        let mut representation_edits = Vec::new();
    
        for span in spans {
            out.push_str(&xml[last..span.start]);
            let measure_text = &xml[span.start..span.end];
            let polished = polish_measure(measure_text, divisions)
                .with_context(|| "Takt konnte für Notensatz-Politur nicht verarbeitet werden")?;
            divisions = polished.divisions;
            total.beam_groups += polished.stats.beam_groups;
            total.beamed_notes += polished.stats.beamed_notes;
            total.removed_beams += polished.stats.removed_beams;
            total.blocked_beam_groups += polished.stats.blocked_beam_groups;
            total.blocked_beam_notes += polished.stats.blocked_beam_notes;
            total.partial_beat_post_rest_beam_groups += polished.stats.partial_beat_post_rest_beam_groups;
            total.partial_beat_post_rest_beam_notes += polished.stats.partial_beat_post_rest_beam_notes;
            total.lower_staff_clef_beam_boundary_splits += polished.stats.lower_staff_clef_beam_boundary_splits;
            total.lower_staff_clef_beam_boundary_notes += polished.stats.lower_staff_clef_beam_boundary_notes;
            representation_edits.extend(polished.edits.into_iter().map(|(range, text)|
                (range.start + span.start..range.end + span.start, text)));
            out.push_str(&polished.text);
            last = span.end;
        }
        out.push_str(&xml[last..]);
    
        // Derselbe idempotente Readback läuft nach der lokalen Clef-/Beam-
        // Politur erneut, damit ein dort entfernter Quell-Rückkehrschlüssel
        // nicht als sichtbare Hilfslinienregression im Endartefakt verbleibt.
        let last_capture = RepresentationCapture::default();
        let (post_cleanup_clef_readback_xml, post_cleanup_receipt) = ledger_bounded_lower_staff_clef_readback(
            &out,
            LowerStaffClefReadbackScope::BeamSafetyCaptured(last_capture.clone()),
        );
        let last_transport = last_capture.take()?;
        let mut internal = crate::master_state_identity::RenderTransitionReceipts::new();
        internal.push_preserving(post_cleanup_receipt, &post_cleanup_clef_readback_xml);
        internal.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);

        let preserving_receipt =
            crate::master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(
                NotationPolishPreservingMint { _sealed: () },
                lineage_pre_xml,
                &post_cleanup_clef_readback_xml,
            );
        let rhythmic_transport = RhythmicRepresentationTransport::polish(
            &NotationPolishPreservingMint { _sealed: () }, vec![
                first_transport, RepresentationStep::recorded(xml, &out, representation_edits), last_transport]);
        let preserving_receipt = preserving_receipt.map(|receipt|
            receipt.with_recorded_representation(rhythmic_transport.clone()));
        Ok(PolishResult {
            rhythmic_transport,
            xml: post_cleanup_clef_readback_xml,
            stats: total,
            preserving_receipt,
        })
    }
    
    pub(crate) struct LedgerBoundedLowerStaffClefReadbackPreservingMint { _sealed: () }

    pub fn ledger_bounded_lower_staff_clef_readback(
        xml: &str,
        scope: LowerStaffClefReadbackScope,
    ) -> (String, Option<PreservingTransformationReceipt>) {
        let spans = find_measure_spans(xml);
        if spans.is_empty() {
            if let LowerStaffClefReadbackScope::BeamSafetyCaptured(capture) = &scope {
                capture.record(RepresentationStep::recorded(xml, xml, Vec::new()));
            }
            return (xml.to_string(), None);
        }
        let mut out = String::with_capacity(xml.len() + 512);
        let mut last = 0_usize;
        let mut representation_edits = Vec::new();
        let mut active_clef = "F".to_string();
        let mut octave_display_shift = 0_i64;
        let mut divisions = 1_i64;
        let policy = if matches!(scope, LowerStaffClefReadbackScope::FinalPdfSource) {
            LowerStaffClefReadbackScope::FinalPdfSource
        } else {
            LowerStaffClefReadbackScope::BeamSafety
        };
        for span in spans {
            out.push_str(&xml[last..span.start]);
            let measure = &xml[span.start..span.end];
            let mut local_edits = Vec::new();
            let rewritten = ledger_bounded_lower_staff_clef_measure(
                measure,
                &mut active_clef,
                &mut octave_display_shift,
                &mut divisions,
                &policy,
                &mut local_edits,
            );
            representation_edits.extend(local_edits.into_iter().map(|(range, text)|
                (range.start + span.start..range.end + span.start, text)));
            out.push_str(&rewritten);
            last = span.end;
        }
        out.push_str(&xml[last..]);
        let receipt = crate::master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(
            LedgerBoundedLowerStaffClefReadbackPreservingMint { _sealed: () }, xml, &out);
        assert!(xml == out || receipt.is_some(), "changed E054 requires preserving proof");
        let journal = RepresentationStep::recorded(xml, &out, representation_edits);
        if let LowerStaffClefReadbackScope::BeamSafetyCaptured(capture) = &scope {
            capture.record(journal.clone());
        }
        let transport = RhythmicRepresentationTransport::ledger_clef(
            &LedgerBoundedLowerStaffClefReadbackPreservingMint { _sealed: () }, journal);
        let receipt = receipt.map(|receipt| receipt.with_recorded_representation(transport));
        (out, receipt)
    }

    const SUSTAINED_CLEF_MIN_ATTACKS: usize = 3;
    const SUSTAINED_CLEF_MIN_QUARTER_NOTES: i64 = 2;
    const SUSTAINED_CLEF_MIN_PRESSURE_GAIN: i64 = 3;

    fn ledger_bounded_lower_staff_clef_measure(
        measure: &str,
        active_clef: &mut String,
        octave_display_shift: &mut i64,
        divisions: &mut i64,
        scope: &LowerStaffClefReadbackScope,
        representation_edits: &mut Vec<(std::ops::Range<usize>, String)>,
    ) -> String {
        *divisions = extract_first_i64(measure, "divisions")
            .unwrap_or(*divisions)
            .max(1);
        let mut sustained_preference = if matches!(scope, LowerStaffClefReadbackScope::FinalPdfSource) {
            sustained_lower_staff_measure_clef_preference(
                measure,
                active_clef,
                *octave_display_shift,
                *divisions,
            )
        } else {
            None
        };
        let mut out = String::with_capacity(measure.len() + 128);
        let measure_opening_note_boundary = measure.find("<note").filter(|&start| {
            measure[start..].find("</note>").is_some_and(|end| {
                !measure[start..start + end].contains("<chord")
            })
        });
        let mut lower_pitched_group_seen = false;
        let mut lower_staff_notation_state_seen = false;
        let mut last = 0_usize;
        let mut scan = 0_usize;
        while scan < measure.len() {
            let next_attributes = measure[scan..].find("<attributes").map(|p| scan + p);
            let next_direction = measure[scan..].find("<direction").map(|p| scan + p);
            let next_note = measure[scan..].find("<note").map(|p| scan + p);
            let mut candidates = Vec::new();
            if let Some(pos) = next_attributes {
                candidates.push((pos, 0_u8));
            }
            if let Some(pos) = next_direction {
                candidates.push((pos, 1_u8));
            }
            if let Some(pos) = next_note {
                candidates.push((pos, 2_u8));
            }
            candidates.sort_by_key(|(pos, _)| *pos);
            let Some((pos, kind)) = candidates.first().copied() else {
                break;
            };
            match kind {
                0 => {
                    let Some(end) = measure[pos..]
                        .find("</attributes>")
                        .map(|rel| pos + rel + "</attributes>".len())
                    else {
                        break;
                    };
                    if let Some(sign) = exact_lower_staff_clef_sign(&measure[pos..end]) {
                        *active_clef = sign.to_string();
                        lower_staff_notation_state_seen = true;
                    }
                    scan = end;
                }
                1 => {
                    let Some(end) = measure[pos..]
                        .find("</direction>")
                        .map(|rel| pos + rel + "</direction>".len())
                    else {
                        break;
                    };
                    if let Some(shift) =
                        observe_lower_staff_octave_display_shift(&measure[pos..end])
                    {
                        *octave_display_shift = shift;
                        lower_staff_notation_state_seen = true;
                    }
                    scan = end;
                }
                2 => {
                    let Some(note_end) = measure[pos..]
                        .find("</note>")
                        .map(|rel| pos + rel + "</note>".len())
                    else {
                        break;
                    };
                    let group_end = lower_staff_chord_group_end(measure, note_end);
                    let positions = lower_staff_visible_group_positions(
                        &measure[pos..group_end],
                        *octave_display_shift,
                    );
                    if !positions.is_empty() {
                        let bass_ledgers = positions
                            .iter()
                            .map(|pitch| PianoClef::BassF4.ledger_lines(*pitch))
                            .max()
                            .unwrap_or(0);
                        let treble_ledgers = positions
                            .iter()
                            .map(|pitch| PianoClef::TrebleG2.ledger_lines(*pitch))
                            .max()
                            .unwrap_or(0);
                        let wanted = if measure[pos..note_end].contains("<chord") {
                            None
                        } else { sustained_preference.take().or_else(|| {
                            if active_clef == "F"
                                && bass_ledgers > 3
                                && treble_ledgers <= 3
                            {
                                Some("G")
                            } else if active_clef == "G"
                                && treble_ledgers > 3
                                && bass_ledgers <= 3
                            {
                                Some("F")
                            } else {
                                None
                            }
                        }) };
                        if let Some(wanted) = wanted {
                            let insertion = if lower_pitched_group_seen
                                || lower_staff_notation_state_seen
                            {
                                pos
                            } else {
                                measure_opening_note_boundary.unwrap_or(pos)
                            };
                            out.push_str(&measure[last..insertion]);
                            let block = ledger_bounded_lower_staff_clef_block(wanted);
                            representation_edits.push((insertion..insertion, block.to_owned()));
                            out.push_str(block);
                            last = insertion;
                            *active_clef = wanted.to_string();
                        }
                        lower_pitched_group_seen = true;
                    }
                    scan = group_end;
                }
                _ => break,
            }
        }
        out.push_str(&measure[last..]);
        out
    }

    fn sustained_lower_staff_measure_clef_preference(
        measure: &str,
        active_clef: &str,
        octave_display_shift: i64,
        divisions: i64,
    ) -> Option<&'static str> {
        let (current_clef, wanted, wanted_clef) = match active_clef {
            "F" => (PianoClef::BassF4, "G", PianoClef::TrebleG2),
            "G" => (PianoClef::TrebleG2, "F", PianoClef::BassF4),
            _ => return None,
        };
        if measure_contains_lower_staff_clef(measure)
            || measure_contains_lower_staff_octave_shift(measure)
        {
            return None;
        }

        let mut attacks = 0_usize;
        let mut attack_duration = 0_i64;
        let mut current_pressure = 0_i64;
        let mut wanted_pressure = 0_i64;
        let mut peak_current_ledgers = 0_i64;
        let mut peak_wanted_ledgers = 0_i64;
        let mut chord = LowerChordObservation::default();
        let mut previous_end = 0;
        for raw in collect_raw_notes(measure) {
            let gap = &measure[previous_end..raw.start];
            previous_end = raw.end;
            let note = raw.text;
            let hidden = note.contains("print-object=\"no\"")
                || note.contains("print-object='no'");
            let lower = extract_first_text(&note, "staff").as_deref() == Some("2");
            let lower_pitch = !hidden && lower && !note.contains("<rest") && note.contains("<pitch>");
            let relation = chord.observe(gap, raw.start, &note, lower_pitch);
            if !lower_pitch {
                continue;
            }
            if relation.counted() {
                attacks = attacks.saturating_add(1);
                attack_duration = attack_duration
                    .saturating_add(extract_first_i64(&note, "duration")?.max(0));
            }
            let pitch = observe_pitch(&note)?.shifted(octave_display_shift);
            let current_ledgers = current_clef.ledger_lines(pitch);
            let wanted_ledgers = wanted_clef.ledger_lines(pitch);
            if wanted_ledgers > current_ledgers {
                return None;
            }
            peak_current_ledgers = peak_current_ledgers.max(current_ledgers);
            peak_wanted_ledgers = peak_wanted_ledgers.max(wanted_ledgers);
            current_pressure = current_pressure
                .saturating_add(clef_course_pressure(current_ledgers));
            wanted_pressure = wanted_pressure
                .saturating_add(clef_course_pressure(wanted_ledgers));
        }

        let pressure_gain = current_pressure.saturating_sub(wanted_pressure);
        (attacks >= SUSTAINED_CLEF_MIN_ATTACKS
            && attack_duration >= divisions.saturating_mul(SUSTAINED_CLEF_MIN_QUARTER_NOTES)
            && peak_current_ledgers <= 3
            && peak_wanted_ledgers <= 3
            && pressure_gain >= SUSTAINED_CLEF_MIN_PRESSURE_GAIN)
            .then_some(wanted)
    }

    fn clef_course_pressure(ledger_lines: i64) -> i64 {
        ledger_lines * ledger_lines + (ledger_lines - 3).max(0) * 8
    }

    fn measure_contains_lower_staff_clef(measure: &str) -> bool {
        let mut scan = 0_usize;
        while let Some(relative_start) = measure[scan..].find("<clef") {
            let start = scan + relative_start;
            let Some(open_end) = measure[start..].find('>').map(|relative| start + relative + 1)
            else {
                return true;
            };
            let opening = &measure[start..open_end];
            if opening.contains("number=\"2\"") || opening.contains("number='2'") {
                return true;
            }
            scan = open_end;
        }
        false
    }

    fn measure_contains_lower_staff_octave_shift(measure: &str) -> bool {
        let mut scan = 0_usize;
        while let Some(relative_start) = measure[scan..].find("<direction") {
            let start = scan + relative_start;
            let Some(end) = measure[start..]
                .find("</direction>")
                .map(|relative_end| start + relative_end + "</direction>".len())
            else {
                return true;
            };
            let direction = &measure[start..end];
            if extract_first_text(direction, "staff").as_deref() == Some("2")
                && direction.contains("<octave-shift")
            {
                return true;
            }
            scan = end;
        }
        false
    }

    fn exact_lower_staff_clef_sign(attributes: &str) -> Option<&'static str> {
        // v0.10.1482 RendererOrderAwareLowerStaffClefCanonicalReadback:
        // MusicXML-Renderer verwenden bei mehreren Staff-2-Clefs im selben
        // attributes-Block den letzten sichtbaren Zustand. Der Readback muss
        // dieselbe Reihenfolge abbilden, statt beim ersten Clef abzubrechen.
        let mut resolved = None;
        let mut scan = 0_usize;
        while let Some(rel_start) = attributes[scan..].find("<clef") {
            let start = scan + rel_start;
            let Some(open_end) = attributes[start..].find('>').map(|rel| start + rel + 1) else {
                break;
            };
            let Some(end) = attributes[open_end..]
                .find("</clef>")
                .map(|rel| open_end + rel + "</clef>".len())
            else {
                break;
            };
            let opening = &attributes[start..open_end];
            let staff_two = opening.contains("number=\"2\"") || opening.contains("number='2'");
            if staff_two {
                let clef = &attributes[start..end];
                if clef.contains("<sign>G</sign>") {
                    resolved = Some("G");
                } else if clef.contains("<sign>F</sign>") {
                    resolved = Some("F");
                }
            }
            scan = end;
        }
        resolved
    }

    fn lower_staff_chord_group_end(measure: &str, mut end: usize) -> usize {
        loop {
            let tail = &measure[end..];
            let whitespace = tail.len().saturating_sub(tail.trim_start().len());
            let note_start = end + whitespace;
            if !measure[note_start..].starts_with("<note") {
                break;
            }
            let Some(note_end) = measure[note_start..]
                .find("</note>")
                .map(|rel| note_start + rel + "</note>".len())
            else {
                break;
            };
            if !measure[note_start..note_end].contains("<chord") {
                break;
            }
            end = note_end;
        }
        end
    }

    fn lower_staff_visible_group_positions(
        group: &str,
        octave_shift: i64,
    ) -> Vec<ClefPitchObservation> {
        let mut positions = Vec::new();
        let mut scan = 0_usize;
        while let Some(rel_start) = group[scan..].find("<note") {
            let start = scan + rel_start;
            let Some(end) = group[start..]
                .find("</note>")
                .map(|rel| start + rel + "</note>".len())
            else {
                break;
            };
            let note = &group[start..end];
            let hidden = note.contains("print-object=\"no\"")
                || note.contains("print-object='no'");
            let staff_two = extract_first_text(note, "staff").as_deref() == Some("2");
            if !hidden && staff_two && !note.contains("<rest") && note.contains("<pitch>")
                && let Some(pitch) = observe_pitch(note) {
                    positions.push(pitch.shifted(octave_shift));
                }
            scan = end;
        }
        positions
    }

    fn ledger_bounded_lower_staff_clef_block(sign: &str) -> &'static str {
        match sign {
            "G" => "\n      <attributes>\n        <clef number=\"2\"><!-- etuschlopf-ledger-bound-clef-readback etuschlopf-register-pressure-gould-write --><sign>G</sign><line>2</line></clef>\n      </attributes>",
            _ => "\n      <attributes>\n        <clef number=\"2\"><!-- etuschlopf-ledger-bound-clef-readback etuschlopf-register-pressure-gould-write --><sign>F</sign><line>4</line></clef>\n      </attributes>",
        }
    }

    #[cfg(test)]
    mod sustained_lower_staff_measure_clef_preference_tests {
        use super::{
            ledger_bounded_lower_staff_clef_readback,
            LowerStaffClefReadbackScope,
        };

        #[test]
        fn standalone_pdf_clef_readback_transports_its_own_preserving_receipt() {
            let low = format!("{}{}{}", note("B", 3, 24), note("B", 3, 96), note("B", 3, 96));
            let high = format!("{}{}{}", note("B", 4, 24), note("B", 4, 48), note("B", 4, 96));
            for (low_ordinal, high_ordinal) in [(13, 29), (71, 137)] {
                let xml = score(&format!("<measure number=\"{low_ordinal}\">{low}</measure><measure number=\"{high_ordinal}\">{high}</measure>"));
                let (post, receipt) = ledger_bounded_lower_staff_clef_readback(&xml, LowerStaffClefReadbackScope::FinalPdfSource);
                assert_ne!(post, xml);
                receipt.as_ref().expect("actual E054 clef change").validate_preserving_transition::<super::LedgerBoundedLowerStaffClefReadbackPreservingMint>(&xml, &post).unwrap();
                let mut trial = crate::master_state_identity::RenderTransitionReceipts::new();
                trial.push_preserving(receipt, &post);
                assert_eq!(trial.len(), 1);
                trial.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            }
        }


        #[test]
        fn chord_attack_projection_e054_counts_first_lower_duration_and_protects_orphans() {
            for base in [1, 40] {
                for head_kind in ["upper", "hidden", "rest"] {
                    let head = match head_kind {
                        "upper" => note("G", 4, 96).replace("<staff>2", "<staff>1"),
                        "hidden" => note("G", 4, 96).replacen("<note>", "<note print-object='no'>", 1),
                        _ => "<note><rest/><duration>96</duration><voice>2</voice><staff>2</staff></note>".to_owned(),
                    };
                    for duration in [48, 96] {
                        let member = note("D", 4, duration).replacen("<note>", "<note><chord/>", 1);
                        let body = format!("{head}{member}").repeat(3);
                        let preference = super::sustained_lower_staff_measure_clef_preference(&body, "F", 0, 96);
                        assert_eq!(preference, (duration == 96).then_some("G"));
                        let next = base + 1;
                        let xml = format!("<score-partwise><part id='P'><measure number='{base}'><attributes><divisions>96</divisions><clef number='2'><sign>F</sign><line>4</line></clef></attributes><note><rest/><duration>384</duration><voice>2</voice><staff>2</staff></note></measure><measure number='{next}'>{body}<note><rest/><duration>96</duration><voice>2</voice><staff>2</staff></note></measure></part></score-partwise>");
                        let (post, receipt) = ledger_bounded_lower_staff_clef_readback(&xml, LowerStaffClefReadbackScope::FinalPdfSource);
                        if duration == 96 { assert_ne!(post, xml); }
                        let doc = roxmltree::Document::parse(&post).unwrap();
                        for n in doc.descendants().filter(|n| n.has_tag_name("note") && n.children().any(|c| c.has_tag_name("chord"))) {
                            let mut previous = n.prev_sibling();
                            while previous.is_some_and(|node| !node.is_element()) {
                                previous = previous.and_then(|node| node.prev_sibling());
                            }
                            assert!(!previous.is_some_and(|node| node.has_tag_name("attributes")));
                        }
                        let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
                        receipts.push_preserving(receipt, &post);
                        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
                    }
                }
                // A later valid group must not borrow an orphan upper member as
                // the opening-measure fallback insertion position.
                let orphan = note("G", 5, 96).replacen("<note>", "<note><chord/>", 1).replace("<staff>2", "<staff>1");
                let valid = note("B", 5, 96);
                let xml = format!("<score-partwise><part id='P'><measure number='{base}'><attributes><divisions>96</divisions></attributes>{orphan}{valid}</measure></part></score-partwise>");
                let (post, receipt) = ledger_bounded_lower_staff_clef_readback(&xml, LowerStaffClefReadbackScope::FinalPdfSource);
                assert!(post.contains(&format!("</attributes>{orphan}")));
                let clef = post.find("etuschlopf-ledger-bound-clef-readback").expect("independent valid group still changes clef");
                assert!(clef > post.find(&orphan).unwrap() + orphan.len());
                let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
                receipts.push_preserving(receipt, &post);
                receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
                for boundary in ["<backup><duration>96</duration></backup>", "<forward><duration>96</duration></forward>", "<attributes><divisions>96</divisions></attributes>", ""] {
                    let orphan = note("B", 5, 96).replacen("<note>", "<note><chord/>", 1);
                    let xml = format!("<score-partwise><part id='P'><measure number='{base}'><attributes><divisions>96</divisions><clef number='2'><sign>F</sign><line>4</line></clef></attributes>{boundary}{orphan}{}</measure></part></score-partwise>", note("C", 3, 96));
                    let (post, receipt) = ledger_bounded_lower_staff_clef_readback(&xml, LowerStaffClefReadbackScope::FinalPdfSource);
                    assert_eq!(post, xml, "an orphan cannot supply a clef insertion position");
                    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
                    receipts.push_preserving(receipt, &post);
                    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
                }
            }
        }

        fn note(step: &str, octave: i64, duration: i64) -> String {
            format!(
                "<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration><voice>2</voice><staff>2</staff></note>"
            )
        }

        fn score(measures: &str) -> String {
            format!(
                r#"<score-partwise><part id="P1"><measure number="1"><attributes><divisions>96</divisions><clef number="2"><sign>G</sign><line>2</line></clef></attributes></measure>{measures}</part></score-partwise>"#
            )
        }

        #[test]
        fn sustained_subthreshold_low_passage_recovers_bass_context() {
            let low = format!(
                "{}{}{}",
                note("B", 3, 24),
                note("B", 3, 96),
                note("B", 3, 96),
            );
            let high = format!(
                "{}{}{}",
                note("B", 4, 24),
                note("B", 4, 48),
                note("B", 4, 96),
            );
            let xml = score(&format!(
                "<measure number=\"\
2\">{low}</measure><measure number=\"\
3\">{high}</measure>"
            ));
            let (out, receipt) = ledger_bounded_lower_staff_clef_readback(
                &xml,
                LowerStaffClefReadbackScope::FinalPdfSource,
            );
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &out);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(out.matches("etuschlopf-ledger-bound-clef-readback").count(), 2);
            assert!(out.contains(
                "<measure number=\"\
2\">\n      <attributes>\n        <clef number=\"2\"><!-- etuschlopf-ledger-bound-clef-readback",
            ));
            assert!(out.contains("<sign>F</sign><line>4</line>"));
            assert!(out.contains("<sign>G</sign><line>2</line>"));
            let (readback, receipt) = ledger_bounded_lower_staff_clef_readback(
                &out,
                LowerStaffClefReadbackScope::FinalPdfSource,
            );
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &readback);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(readback, out);
        }

        #[test]
        fn short_low_fragment_does_not_interrupt_treble_context() {
            let low = format!(
                "{}{}{}",
                note("B", 3, 24),
                note("B", 3, 24),
                note("B", 3, 24),
            );
            let xml = score(&format!("<measure number=\"\
2\">{low}</measure>"));
            let (readback, receipt) = ledger_bounded_lower_staff_clef_readback(
                &xml,
                LowerStaffClefReadbackScope::FinalPdfSource,
            );
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &readback);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(readback, xml);
        }

        #[test]
        fn two_low_attacks_do_not_interrupt_treble_context() {
            let low = format!("{}{}", note("B", 3, 96), note("B", 3, 96));
            let xml = score(&format!("<measure number=\"\
2\">{low}</measure>"));
            let (readback, receipt) = ledger_bounded_lower_staff_clef_readback(
                &xml,
                LowerStaffClefReadbackScope::FinalPdfSource,
            );
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &readback);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(readback, xml);
        }

        #[test]
        fn mixed_register_measure_remains_fail_closed() {
            let mixed = format!(
                "{}{}{}",
                note("B", 3, 96),
                note("B", 3, 96),
                note("B", 4, 96),
            );
            let xml = score(&format!("<measure number=\"\
2\">{mixed}</measure>"));
            let (readback, receipt) = ledger_bounded_lower_staff_clef_readback(
                &xml,
                LowerStaffClefReadbackScope::FinalPdfSource,
            );
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &readback);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(readback, xml);
        }

        #[test]
        fn beam_safety_scope_does_not_publish_sustained_subthreshold_clefs() {
            let low = format!(
                "{}{}{}",
                note("B", 3, 24),
                note("B", 3, 96),
                note("B", 3, 96),
            );
            let xml = score(&format!("<measure number=\"\
2\">{low}</measure>"));
            let (readback, receipt) = ledger_bounded_lower_staff_clef_readback(
                &xml,
                LowerStaffClefReadbackScope::BeamSafety,
            );
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &readback);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(readback, xml);
        }

        #[test]
        fn first_lower_passage_clef_precedes_the_first_note_without_moving_directions() {
            let low = format!(
                "{}{}{}",
                note("B", 3, 24),
                note("B", 3, 96),
                note("B", 3, 96),
            );
            let measure = format!(
                "<measure number=\"\
2\"><print new-system=\"yes\"/><direction placement=\"above\"><direction-type><octave-shift type=\"down\" size=\"8\"/></direction-type><staff>1</staff></direction><note><pitch><step>C</step><octave>5</octave></pitch><duration>192</duration><voice>1</voice><staff>1</staff></note><backup><duration>192</duration></backup>{low}</measure>"
            );
            let xml = score(&measure);
            let (out, receipt) = ledger_bounded_lower_staff_clef_readback(
                &xml,
                LowerStaffClefReadbackScope::FinalPdfSource,
            );
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &out);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            let direction = out.find(r#"<direction placement="above">"#).unwrap();
            let clef = out.find("etuschlopf-ledger-bound-clef-readback").unwrap();
            let upper_note = out.find("<note><pitch><step>C</step>").unwrap();
            assert!(direction < clef && clef < upper_note);
        }

        #[test]
        fn later_lower_passage_clef_remains_at_its_trigger() {
            let low = note("C", 3, 96);
            let high = note("B", 4, 96);
            let measure = format!(
                "<measure number=\"\
2\"><note><pitch><step>C</step><octave>5</octave></pitch><duration>192</duration><voice>1</voice><staff>1</staff></note><backup><duration>192</duration></backup>{low}{high}</measure>"
            );
            let xml = score(&measure).replacen("<sign>G</sign>", "<sign>F</sign>", 1);
            let (out, receipt) = ledger_bounded_lower_staff_clef_readback(
                &xml,
                LowerStaffClefReadbackScope::FinalPdfSource,
            );
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &out);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            let low_note = out.find("<note><pitch><step>C</step><octave>3</octave>").unwrap();
            let clef = out.find("etuschlopf-ledger-bound-clef-readback").unwrap();
            let high_note = out.find("<note><pitch><step>B</step><octave>4</octave>").unwrap();
            assert!(low_note < clef && clef < high_note);
        }
    }

    fn find_measure_spans(xml: &str) -> Vec<MeasureSpan> {
        let mut spans = Vec::new();
        let mut search_from = 0_usize;
    
        while let Some(rel_start) = xml[search_from..].find("<measure") {
            let start = search_from + rel_start;
            let Some(open_end_rel) = xml[start..].find('>') else {
                break;
            };
            let content_start = start + open_end_rel + 1;
            let Some(close_rel) = xml[content_start..].find("</measure>") else {
                break;
            };
            let end = content_start + close_rel + "</measure>".len();
            spans.push(MeasureSpan { start, end });
            search_from = end;
        }
    
        spans
    }
    
    fn polish_measure(text: &str, inherited_divisions: i64) -> anyhow::Result<MeasurePolish> {
        let divisions = extract_first_i64(text, "divisions").unwrap_or(inherited_divisions).max(1);
        let raw_notes = collect_raw_notes(text);
        if raw_notes.is_empty() {
            return Ok(MeasurePolish {
                edits: Vec::new(),
                text: text.to_string(),
                divisions,
                stats: PolishStats::default(),
            });
        }
    
        let note_infos = collect_note_infos(text, &raw_notes);
        let lower_staff_clef_boundaries = collect_lower_staff_clef_boundaries(text);
        let mut beam_state_by_note: BTreeMap<usize, Vec<(usize, &'static str)>> = BTreeMap::new();
        let mut stats = PolishStats::default();
    
        let mut natural_group_counts: BTreeMap<(String, String, i64), usize> = BTreeMap::new();
        let mut groups: BTreeMap<(String, String, i64, usize), Vec<usize>> = BTreeMap::new();
        for info in &note_infos {
            if !is_beam_candidate(info) {
                continue;
            }
            let beat_bucket = info.start.div_euclid(divisions);
            let natural_key = (info.staff.clone(), info.voice.clone(), beat_bucket);
            *natural_group_counts.entry(natural_key).or_default() += 1;
            let clef_segment = lower_staff_clef_beam_segment(info, &lower_staff_clef_boundaries);
            groups
                .entry((info.staff.clone(), info.voice.clone(), beat_bucket, clef_segment))
                .or_default()
                .push(info.index);
        }
    
        for ((staff, voice, beat_bucket), natural_count) in natural_group_counts {
            if staff.trim() != "2" || natural_count < 2 {
                continue;
            }
            let split_segments = groups
                .keys()
                .filter(|(g_staff, g_voice, g_beat, _)| g_staff == &staff && g_voice == &voice && *g_beat == beat_bucket)
                .count();
            if split_segments > 1 {
                stats.lower_staff_clef_beam_boundary_splits += split_segments - 1;
                stats.lower_staff_clef_beam_boundary_notes += natural_count;
            }
        }
    
        for (_key, mut indices) in groups {
            indices.sort_by_key(|idx| {
                note_infos
                    .iter()
                    .find(|info| info.index == *idx)
                    .map_or(0, |info| info.start)
            });
            if indices.len() < 2 {
                continue;
            }
    
            let partial_post_rest = is_partial_beat_post_rest_beam_group(&note_infos, &indices, divisions);
            if !is_safe_beam_group(&note_infos, &indices, divisions) {
                stats.blocked_beam_groups += 1;
                stats.blocked_beam_notes += indices.len();
                continue;
            }
    
            stats.beam_groups += 1;
            stats.beamed_notes += indices.len();
            if partial_post_rest {
                stats.partial_beat_post_rest_beam_groups += 1;
                stats.partial_beat_post_rest_beam_notes += indices.len();
            }
            if let Some((ordered_note_indices, plan)) =
                metric_beam_plan(&note_infos, &indices)
            {
                let planned_states = plan
                    .edits()
                    .iter()
                    .map(|edit| {
                        ordered_note_indices.get(edit.event_ordinal()).map(|note_index| {
                            (
                                *note_index,
                                usize::from(edit.level()),
                                edit.state().as_musicxml_text(),
                            )
                        })
                    })
                    .collect::<Option<Vec<_>>>();
                let Some(planned_states) = planned_states else {
                    continue;
                };
                for (note_index, level, state) in planned_states {
                    beam_state_by_note.entry(note_index).or_default().push((level, state));
                }
            }
        }
    
        // v0.10.1778 NotationPolishBeamDecisionTrichotomyClosure:
        // Positive plan notes are owned and canonicalized. Outside that positive
        // plan, complete same-Staff/Voice/Beat Beam chains are foreign truth and
        // stay byteidentical. Only recognized Beam levels that cannot be bound
        // into a complete structural chain are explicit negative decisions and
        // may be removed. This keeps the v0.10.1777 noninterference goal without
        // preserving orphan/cross-beat Beam fragments that made MuseScore render
        // a changed page in the Etuede-4 stress run.
        let structurally_bound_external_levels = structurally_complete_external_beam_levels(
            &note_infos,
            &raw_notes,
            divisions,
            &lower_staff_clef_boundaries,
        );
        let mut rebuilt = String::with_capacity(text.len() + stats.beamed_notes * 40);
        let mut edits = Vec::new();
        let mut last = 0_usize;
        for note in &raw_notes {
            rebuilt.push_str(&text[last..note.start]);
            let output_start = rebuilt.len();
            if let Some(states) = beam_state_by_note.get(&note.start) {
                stats.removed_beams += count_beam_tags(&note.text);
                let cleaned = strip_beam_tags(&note.text);
                rebuilt.push_str(&insert_beam_tags(&cleaned, states));
            } else {
                let (retained, removed) = retain_only_structurally_bound_external_beams(
                    &note.text,
                    note.start,
                    &structurally_bound_external_levels,
                );
                stats.removed_beams += removed;
                rebuilt.push_str(&retained);
            }
            edits.push((note.start..note.end, rebuilt[output_start..].to_owned()));
            last = note.end;
        }
        rebuilt.push_str(&text[last..]);
    
        Ok(MeasurePolish {
            edits,
            text: rebuilt,
            divisions,
            stats,
        })
    }
    
    fn collect_raw_notes(text: &str) -> Vec<RawNote> {
        let mut notes = Vec::new();
        let mut search_from = 0_usize;
    
        while let Some(rel_start) = text[search_from..].find("<note") {
            let start = search_from + rel_start;
            let Some(open_end_rel) = text[start..].find('>') else {
                break;
            };
            let start_tag = &text[start..start + open_end_rel + 1];
            if start_tag.ends_with("/>") {
                let end = start + open_end_rel + 1;
                notes.push(RawNote {
                    start,
                    end,
                    text: text[start..end].to_string(),
                });
                search_from = end;
                continue;
            }
    
            let content_start = start + open_end_rel + 1;
            let Some(close_rel) = text[content_start..].find("</note>") else {
                break;
            };
            let end = content_start + close_rel + "</note>".len();
            notes.push(RawNote {
                start,
                end,
                text: text[start..end].to_string(),
            });
            search_from = end;
        }
    
        notes
    }
    
    fn collect_note_infos(measure_text: &str, raw_notes: &[RawNote]) -> Vec<NoteInfo> {
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
                    note_pos += 1;
                    let duration = extract_first_i64(&raw.text, "duration").unwrap_or(0).max(0);
                    let is_chord = raw.text.contains("<chord") || raw.text.contains("<chord/");
                    let is_rest = raw.text.contains("<rest") || raw.text.contains("<rest/");
                    let is_hidden = raw.text.contains("print-object=\"no\"");
                    let start = if is_chord { last_non_chord_start } else { cursor };
                    let staff = extract_first_text(&raw.text, "staff").unwrap_or_else(|| "1".to_string());
                    let voice = extract_first_text(&raw.text, "voice").unwrap_or_else(|| "1".to_string());
                    let note_type = extract_first_text(&raw.text, "type").unwrap_or_default();
    
                    infos.push(NoteInfo {
                        index: raw.start,
                        start,
                        duration,
                        staff,
                        voice,
                        note_type,
                        is_rest,
                        is_chord,
                        is_hidden,
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
    
    fn is_beam_candidate(info: &NoteInfo) -> bool {
        !info.is_rest
            && !info.is_hidden
            && !info.is_chord
            && info.duration > 0
            && matches!(info.note_type.as_str(), "eighth" | "16th" | "32nd" | "64th")
    }
    
    fn collect_lower_staff_clef_boundaries(measure_text: &str) -> Vec<i64> {
        let mut boundaries = Vec::new();
        let mut cursor = 0_i64;
        let mut scan = 0_usize;
    
        while scan < measure_text.len() {
            let next_attr = measure_text[scan..].find("<attributes").map(|p| scan + p);
            let next_note = measure_text[scan..].find("<note").map(|p| scan + p);
            let next_backup = measure_text[scan..].find("<backup").map(|p| scan + p);
            let next_forward = measure_text[scan..].find("<forward").map(|p| scan + p);
    
            let mut candidates = Vec::new();
            if let Some(p) = next_attr { candidates.push((p, "attributes")); }
            if let Some(p) = next_note { candidates.push((p, "note")); }
            if let Some(p) = next_backup { candidates.push((p, "backup")); }
            if let Some(p) = next_forward { candidates.push((p, "forward")); }
            candidates.sort_by_key(|(p, _)| *p);
            let Some((pos, kind)) = candidates.first().copied() else { break; };
    
            match kind {
                "attributes" => {
                    let Some(end) = measure_text[pos..].find("</attributes>").map(|e| pos + e + "</attributes>".len()) else { break; };
                    let segment = &measure_text[pos..end];
                    if cursor > 0 && has_lower_staff_clef_boundary(segment) {
                        boundaries.push(cursor);
                    }
                    scan = end;
                }
                "backup" => {
                    let Some(end) = measure_text[pos..].find("</backup>").map(|e| pos + e + "</backup>".len()) else { break; };
                    cursor -= extract_first_i64(&measure_text[pos..end], "duration").unwrap_or(0).max(0);
                    scan = end;
                }
                "forward" => {
                    let Some(end) = measure_text[pos..].find("</forward>").map(|e| pos + e + "</forward>".len()) else { break; };
                    cursor += extract_first_i64(&measure_text[pos..end], "duration").unwrap_or(0).max(0);
                    scan = end;
                }
                "note" => {
                    let Some(end) = measure_text[pos..].find("</note>").map(|e| pos + e + "</note>".len()) else { break; };
                    let segment = &measure_text[pos..end];
                    let is_chord = segment.contains("<chord") || segment.contains("<chord/");
                    if !is_chord {
                        cursor += extract_first_i64(segment, "duration").unwrap_or(0).max(0);
                    }
                    scan = end;
                }
                _ => break,
            }
        }
    
        boundaries.sort_unstable();
        boundaries.dedup();
        boundaries
    }
    
    fn has_lower_staff_clef_boundary(attributes: &str) -> bool {
        attributes.contains("<clef")
            && (attributes.contains("number=\"2\"") || attributes.contains("number='2'"))
    }
    
    fn lower_staff_clef_beam_segment(info: &NoteInfo, boundaries: &[i64]) -> usize {
        if info.staff.trim() != "2" {
            return 0;
        }
        boundaries.iter().filter(|boundary| **boundary <= info.start).count()
    }
    
    
    /// Render-only Normalisierung unterer-System-Schlüsseloberflächen.
    ///
    /// Lokale Same-Measure- und Cross-Measure-Clef-Island-Umschreibungen sind
    /// pensioniert. Der zentrale `ClefReadability`-Kurs besitzt die finale
    /// F4/G2-Entscheidung. Dieser historische Einstieg darf nur noch die bereits
    /// bestehende, scoreweit eindeutige Registerdruck-Eintrittsfläche ergänzen.
    /// Pitch, Onset, Dauer, Voice, Staff, Tie/Notations und Beam-Zugehoerigkeit
    /// der vorhandenen Noten bleiben unverändert.
    pub fn decompose_lower_staff_clef_islands(xml: &str) -> (anyhow::Result<String>, Option<PreservingTransformationReceipt>) {
        let (out, _) = complete_unique_register_pressure_clef_island(xml);
        let receipt =
            crate::master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(
                NotationClefIslandPreservingMint { _sealed: () },
                xml,
                &out,
            );
        (Ok(out), receipt)
    }
    
    const REGISTER_PRESSURE_TREBLE_CLEF_BLOCK: &str = r#"
      <attributes>
        <!-- etuschlopf-register-pressure-gould-write -->
        <clef number="2"><sign>G</sign><line>2</line></clef>
      </attributes>"#;

    #[derive(Debug, Clone)]
    struct RegisterPressureMeasureSummary {
        start: usize,
        end: usize,
        opening_tag_end: usize,
        lower_staff_midis: Vec<i64>,
        has_lower_staff_clef: bool,
    }

    #[derive(Debug, Clone)]
    struct RegisterPressureClefEvent {
        start: usize,
        end: usize,
        sign: String,
        line: String,
    }

    #[derive(Debug, Clone)]
    struct RegisterPressureCompletionCandidate {
        insertion_byte: usize,
    }

    fn complete_unique_register_pressure_clef_island(xml: &str) -> (String, usize) {
        let candidates = detect_register_pressure_clef_island_completion_candidates(xml);
        if candidates.len() != 1 {
            return (xml.to_string(), 0);
        }
        let candidate = &candidates[0];
        let before_notes = collect_raw_notes(xml)
            .into_iter()
            .map(|note| note.text)
            .collect::<Vec<_>>();
        let before_clefs = collect_register_pressure_lower_staff_clef_events(xml).len();
        let mut out =
            String::with_capacity(xml.len() + REGISTER_PRESSURE_TREBLE_CLEF_BLOCK.len());
        out.push_str(&xml[..candidate.insertion_byte]);
        out.push_str(REGISTER_PRESSURE_TREBLE_CLEF_BLOCK);
        out.push_str(&xml[candidate.insertion_byte..]);
        let after_notes = collect_raw_notes(&out)
            .into_iter()
            .map(|note| note.text)
            .collect::<Vec<_>>();
        let after_clefs = collect_register_pressure_lower_staff_clef_events(&out).len();
        let exact_single_clef_delta = after_clefs == before_clefs.saturating_add(1)
            && out
                .matches("etuschlopf-register-pressure-gould-write")
                .count()
                == xml
                    .matches("etuschlopf-register-pressure-gould-write")
                    .count()
                    .saturating_add(1);
        if before_notes != after_notes
            || !exact_single_clef_delta
            || !detect_register_pressure_clef_island_completion_candidates(&out).is_empty()
        {
            return (xml.to_string(), 0);
        }
        (out, 1)
    }

    fn detect_register_pressure_clef_island_completion_candidates(
        xml: &str,
    ) -> Vec<RegisterPressureCompletionCandidate> {
        let measures = collect_register_pressure_measure_summaries(xml);
        let clefs = collect_register_pressure_lower_staff_clef_events(xml);
        let mut candidates = Vec::new();
        for (index, return_clef) in clefs.iter().enumerate() {
            if return_clef.sign != "F" || return_clef.line != "4" {
                continue;
            }
            let Some(previous_clef) = clefs[..index].last() else {
                continue;
            };
            if previous_clef.sign != "F" || previous_clef.line != "4" {
                continue;
            }
            let Some(return_measure_index) = measures.iter().position(|measure| {
                return_clef.start >= measure.start && return_clef.start < measure.end
            }) else {
                continue;
            };
            let Some(high_measure_index) = measures[..return_measure_index]
                .iter()
                .rposition(|measure| !measure.lower_staff_midis.is_empty())
            else {
                continue;
            };
            if measures[high_measure_index + 1..return_measure_index]
                .iter()
                .any(|measure| !measure.lower_staff_midis.is_empty())
            {
                continue;
            }
            let high_measure = &measures[high_measure_index];
            let high_f_pressure = register_pressure_sum(&high_measure.lower_staff_midis, "F");
            let high_g_pressure = register_pressure_sum(&high_measure.lower_staff_midis, "G");
            if high_f_pressure == 0 || high_g_pressure >= high_f_pressure {
                continue;
            }
            let mut entry_measure_index = high_measure_index;
            while entry_measure_index > 0 {
                let prior = &measures[entry_measure_index - 1];
                if prior.lower_staff_midis.is_empty() || prior.has_lower_staff_clef {
                    break;
                }
                let prior_f_pressure = register_pressure_sum(&prior.lower_staff_midis, "F");
                let prior_g_pressure = register_pressure_sum(&prior.lower_staff_midis, "G");
                if prior_f_pressure == 0 || prior_g_pressure >= prior_f_pressure {
                    break;
                }
                entry_measure_index -= 1;
            }
            let entry_measure = &measures[entry_measure_index];
            if entry_measure.has_lower_staff_clef
                || previous_clef.end >= entry_measure.start
                || previous_clef.end >= return_clef.start
            {
                continue;
            }
            let run_midis = measures[entry_measure_index..=high_measure_index]
                .iter()
                .flat_map(|measure| measure.lower_staff_midis.iter().copied())
                .collect::<Vec<_>>();
            let inherited_pressure = register_pressure_sum(&run_midis, "F");
            let inserted_pressure = register_pressure_sum(&run_midis, "G");
            if inherited_pressure == 0 || inserted_pressure >= inherited_pressure {
                continue;
            }
            candidates.push(RegisterPressureCompletionCandidate {
                insertion_byte: entry_measure.opening_tag_end,
            });
        }
        candidates.sort_by_key(|candidate| candidate.insertion_byte);
        candidates.dedup_by_key(|candidate| candidate.insertion_byte);
        candidates
    }

    fn collect_register_pressure_measure_summaries(
        xml: &str,
    ) -> Vec<RegisterPressureMeasureSummary> {
        let mut measures = Vec::new();
        for span in find_measure_spans(xml) {
            let measure = &xml[span.start..span.end];
            let opening_tag_end = span.start + measure.find('>').unwrap_or(0) + 1;
            let lower_staff_midis = collect_raw_notes(measure)
                .into_iter()
                .filter(|note| {
                    !note.text.contains("print-object=\"no\"")
                        && !note.text.contains("print-object='no'")
                        && !note.text.contains("<rest")
                        && extract_first_text(&note.text, "staff").as_deref() == Some("2")
                })
                .filter_map(|note| note_text_midi(&note.text))
                .collect::<Vec<_>>();
            measures.push(RegisterPressureMeasureSummary {
                start: span.start,
                end: span.end,
                opening_tag_end,
                lower_staff_midis,
                has_lower_staff_clef: !collect_register_pressure_lower_staff_clef_events(measure)
                    .is_empty(),
            });
        }
        measures
    }

    fn collect_register_pressure_lower_staff_clef_events(
        xml: &str,
    ) -> Vec<RegisterPressureClefEvent> {
        let mut events = Vec::new();
        let mut scan = 0_usize;
        while let Some(relative_start) = xml[scan..].find("<clef") {
            let start = scan + relative_start;
            let Some(relative_end) = xml[start..].find("</clef>") else {
                break;
            };
            let end = start + relative_end + "</clef>".len();
            let clef = &xml[start..end];
            let is_lower = clef.contains("number=\"2\"") || clef.contains("number='2'");
            if is_lower
                && let (Some(sign), Some(line)) = (
                    extract_first_text(clef, "sign"),
                    extract_first_text(clef, "line"),
                ) {
                    events.push(RegisterPressureClefEvent {
                        start,
                        end,
                        sign,
                        line,
                    });
                }
            scan = end;
        }
        events
    }

    fn register_pressure_sum(midis: &[i64], clef: &str) -> i64 {
        midis
            .iter()
            .copied()
            .map(|midi| super::lower_staff_ledger_pressure(midi, clef))
            .sum()
    }

    #[cfg(test)]
    mod register_pressure_clef_island_completion_tests {
        use super::{
            complete_unique_register_pressure_clef_island,
            decompose_lower_staff_clef_islands, NotationClefIslandPreservingMint,
        };

        fn note(step: &str, octave: i64) -> String {
            format!(
                "<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff></note>"
            )
        }

        #[test]
        fn completes_unique_missing_treble_entry_before_bass_return() {
            let xml = format!(
                "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes></measure><measure number=\"\
4\">{}</measure><measure number=\"\
5\">{}{}{}{}</measure><measure number=\"\
6\"><note><rest/><duration>96</duration><voice>2</voice><staff>2</staff></note></measure><measure number=\"\
7\"><attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes></measure></part></score-partwise>",
                note("C", 4),
                note("F", 4),
                note("D", 5),
                note("E", 5),
                note("D", 5),
            );
            let (result, receipt) = decompose_lower_staff_clef_islands(&xml);
            let result = result.unwrap();
            let receipt = receipt.expect("E052 changed post requires its own preserving receipt");
            assert!(receipt
                .validate_preserving_transition::<NotationClefIslandPreservingMint>(&xml, &result)
                .is_ok());
            assert_eq!(
                result.matches("etuschlopf-register-pressure-gould-write").count(),
                1
            );
            let (second, written) = complete_unique_register_pressure_clef_island(&result);
            assert_eq!(written, 0);
            assert_eq!(second, result);
        }

        #[test]
        fn bass_readable_passage_does_not_create_island() {
            let xml = format!(
                "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes></measure><measure number=\"\
2\">{}</measure><measure number=\"\
3\"><attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes></measure></part></score-partwise>",
                note("C", 4),
            );
            let (out, written) = complete_unique_register_pressure_clef_island(&xml);
            assert_eq!(written, 0);
            assert_eq!(out, xml);
        }
    }

    #[cfg(test)]
    mod clef_island_staff_projection_retirement_tests {
        use super::decompose_lower_staff_clef_islands;

        #[test]
        fn same_measure_lower_staff_island_preserves_staff_and_beam_membership() {
            let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\">\n<attributes><divisions>96</divisions><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>\n<attributes><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>\n<note><pitch><step>A</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><stem>down</stem><staff>2</staff><beam number=\"1\">continue</beam><beam number=\"2\">continue</beam></note>\n<attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>\n</measure></part></score-partwise>";

            let (result, receipt) = decompose_lower_staff_clef_islands(xml);
            let result = result.unwrap();
            assert!(receipt.is_none());

            assert_eq!(result.matches("<staff>2</staff>").count(), 1);
            assert_eq!(result.matches("<staff>1</staff>").count(), 0);
            assert_eq!(
                result
                    .matches("<beam number=\"1\">continue</beam>")
                    .count(),
                1
            );
            assert_eq!(
                result
                    .matches("<beam number=\"2\">continue</beam>")
                    .count(),
                1
            );
            assert_eq!(result.matches("<sign>G</sign>").count(), 1);
            assert_eq!(result.matches("<sign>F</sign>").count(), 2);
        }

        #[test]
        fn cross_measure_lower_staff_clef_surfaces_are_not_locally_removed() {
            let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\">\n<attributes><divisions>96</divisions><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>\n<attributes><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>\n<note><pitch><step>A</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>\n</measure><measure number=\"\
2\">\n<attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>\n</measure></part></score-partwise>";

            let (result, receipt) = decompose_lower_staff_clef_islands(xml);
            let result = result.unwrap();
            assert!(receipt.is_none());

            assert_eq!(result.matches("<sign>G</sign>").count(), 1);
            assert_eq!(result.matches("<sign>F</sign>").count(), 2);
            assert_eq!(result.matches("<staff>2</staff>").count(), 1);
            assert_eq!(result.matches("<staff>1</staff>").count(), 0);
        }
    }

    fn note_text_midi(note_text: &str) -> Option<i64> {
        let step = extract_first_text(note_text, "step")?;
        let octave = extract_first_i64(note_text, "octave")?;
        let alter = extract_first_i64(note_text, "alter").unwrap_or(0);
        let base = match step.trim() {
            "C" => 0,
            "D" => 2,
            "E" => 4,
            "F" => 5,
            "G" => 7,
            "A" => 9,
            "B" => 11,
            _ => return None,
        };
        Some((octave + 1) * 12 + base + alter)
    }
    
    fn is_safe_beam_group(note_infos: &[NoteInfo], indices: &[usize], divisions: i64) -> bool {
        let mut group: Vec<&NoteInfo> = indices
            .iter()
            .filter_map(|idx| note_infos.iter().find(|info| info.index == *idx))
            .collect();
        if group.len() < 2 {
            return false;
        }
    
        group.sort_by_key(|info| (info.start, info.index));
    
        // MetricBeamPolicy v0.10.216:
        // Balkengruppen müssen entweder direkt an einer Zählzeit beginnen oder als
        // PartialBeatPostRestBeamingRule nach einer sichtbaren Anfangspause die
        // verbleibenden beambaren Noten derselben Zählzeit zusammenfassen. Dadurch
        // bleibt die Beat-Struktur lesbar, ohne Takt- oder Hand-Spezialfälle in den
        // Code zu schreiben. Tonhöhe/Stimmrichtung blockiert die Balkung nicht;
        // entscheidend ist die metrische Lesbarkeit.
        let beat_unit = divisions.max(1);
        let beat = group[0].start.div_euclid(beat_unit);
        let beat_start = beat * beat_unit;
        let beat_end = (beat + 1) * beat_unit;
        let starts_at_beat = group[0].start == beat_start;
        let starts_after_visible_leading_rest = leading_visible_rest_covers_beat_gap(note_infos, &group, beat_start);
        if !starts_at_beat && !starts_after_visible_leading_rest {
            return false;
        }
        let mut previous_end: Option<i64> = None;
        let mut last_start: Option<i64> = None;
        for info in &group {
            if info.start.div_euclid(beat_unit) != beat {
                return false;
            }
            if info.start + info.duration > beat_end {
                return false;
            }
            if Some(info.start) == last_start {
                return false;
            }
            if let Some(end) = previous_end
                && info.start != end {
                    return false;
                }
            previous_end = Some(info.start + info.duration);
            last_start = Some(info.start);
        }
    
        true
    }
    
    fn is_partial_beat_post_rest_beam_group(note_infos: &[NoteInfo], indices: &[usize], divisions: i64) -> bool {
        let mut group: Vec<&NoteInfo> = indices
            .iter()
            .filter_map(|idx| note_infos.iter().find(|info| info.index == *idx))
            .collect();
        if group.len() < 2 {
            return false;
        }
        group.sort_by_key(|info| (info.start, info.index));
        let beat_unit = divisions.max(1);
        let beat_start = group[0].start.div_euclid(beat_unit) * beat_unit;
        group[0].start > beat_start && leading_visible_rest_covers_beat_gap(note_infos, &group, beat_start)
    }
    
    fn leading_visible_rest_covers_beat_gap(note_infos: &[NoteInfo], group: &[&NoteInfo], beat_start: i64) -> bool {
        let Some(first) = group.first().copied() else { return false; };
        if first.start <= beat_start {
            return false;
        }
        let mut rests: Vec<&NoteInfo> = note_infos
            .iter()
            .filter(|info| {
                info.staff == first.staff
                    && info.voice == first.voice
                    && info.is_rest
                    && !info.is_hidden
                    && !info.is_chord
                    && info.duration > 0
                    && info.start >= beat_start
                    && info.start < first.start
            })
            .collect();
        rests.sort_by_key(|info| (info.start, info.index));
    
        let mut cursor = beat_start;
        for rest in rests {
            if rest.start != cursor {
                return false;
            }
            let end = rest.start + rest.duration;
            if end > first.start {
                return false;
            }
            cursor = end;
        }
        cursor == first.start
    }
    
    fn metric_beam_plan(
        note_infos: &[NoteInfo],
        indices: &[usize],
    ) -> Option<(Vec<usize>, crate::structural_write_authority::BeamPlan)> {
        let mut group: Vec<&NoteInfo> = indices
            .iter()
            .filter_map(|idx| note_infos.iter().find(|info| info.index == *idx))
            .collect();
        group.sort_by_key(|info| (info.start, info.index));
        if group.len() < 2 {
            return None;
        }

        let ordered_note_indices = group.iter().map(|info| info.index).collect::<Vec<_>>();
        let mut owned_level_groups = vec![((0..group.len()).collect::<Vec<_>>(), 1_u8)];
        for level in 2..=4 {
            let mut run = Vec::new();
            for (event_ordinal, info) in group.iter().enumerate() {
                if beam_level(info) >= usize::from(level) {
                    run.push(event_ordinal);
                } else if run.len() >= 2 {
                    owned_level_groups.push((std::mem::take(&mut run), level));
                } else {
                    run.clear();
                }
            }
            if run.len() >= 2 {
                owned_level_groups.push((run, level));
            }
        }
        let level_groups = owned_level_groups
            .iter()
            .map(|(ordinals, level)| (ordinals.as_slice(), *level))
            .collect::<Vec<_>>();
        let plan = crate::structural_write_authority::BeamPlan::from_level_groups(
            group.len(),
            &level_groups,
        )?;
        Some((ordered_note_indices, plan))
    }

    fn beam_level(info: &NoteInfo) -> usize {
        match info.note_type.as_str() {
            "eighth" => 1,
            "16th" => 2,
            "32nd" => 3,
            "64th" => 4,
            _ => 0,
        }
    }
    
    fn extract_first_i64(text: &str, tag: &str) -> Option<i64> {
        extract_first_text(text, tag).and_then(|value| value.trim().parse::<i64>().ok())
    }
    
    fn extract_first_text(text: &str, tag: &str) -> Option<String> {
        let open = format!("<{tag}>");
        let close = format!("</{tag}>");
        let start = text.find(&open)? + open.len();
        let end = text[start..].find(&close)? + start;
        Some(text[start..end].trim().to_string())
    }
    

    fn structurally_complete_external_beam_levels(
        note_infos: &[NoteInfo],
        raw_notes: &[RawNote],
        divisions: i64,
        lower_staff_clef_boundaries: &[i64],
    ) -> BTreeSet<(usize, usize)> {
        let beat_unit = divisions.max(1);
        let mut groups: BTreeMap<(String, String, i64, usize), Vec<&NoteInfo>> = BTreeMap::new();
        for info in note_infos {
            if is_beam_candidate(info) {
                groups
                    .entry((
                        info.staff.clone(),
                        info.voice.clone(),
                        info.start.div_euclid(beat_unit),
                        lower_staff_clef_beam_segment(info, lower_staff_clef_boundaries),
                    ))
                    .or_default()
                    .push(info);
            }
        }

        let mut valid = BTreeSet::new();
        for (_key, mut group) in groups {
            group.sort_by_key(|info| (info.start, info.index));
            for level in 1_usize..=4 {
                let mut open: Vec<&NoteInfo> = Vec::new();
                for info in &group {
                    let state = raw_notes
                        .iter()
                        .find(|raw| raw.start == info.index)
                        .and_then(|raw| observed_beam_state(&raw.text, level));
                    if state.is_some() && beam_level(info) < level {
                        open.clear();
                        continue;
                    }
                    match state.as_deref() {
                        Some("begin") => {
                            open.clear();
                            open.push(*info);
                        }
                        Some("continue") => {
                            let Some(previous) = open.last().copied() else {
                                continue;
                            };
                            if external_beam_gap_is_structurally_bound(
                                note_infos,
                                previous,
                                info,
                            ) {
                                open.push(*info);
                            } else {
                                open.clear();
                            }
                        }
                        Some("end") => {
                            let Some(previous) = open.last().copied() else {
                                continue;
                            };
                            if external_beam_gap_is_structurally_bound(
                                note_infos,
                                previous,
                                info,
                            ) {
                                open.push(*info);
                                if open.len() >= 2 {
                                    for member in &open {
                                        valid.insert((member.index, level));
                                    }
                                }
                            }
                            open.clear();
                        }
                        Some(_) | None => {
                            open.clear();
                        }
                    }
                }
            }
        }
        valid
    }

    fn external_beam_gap_is_structurally_bound(
        note_infos: &[NoteInfo],
        left: &NoteInfo,
        right: &NoteInfo,
    ) -> bool {
        let mut cursor = left.start + left.duration;
        if cursor == right.start {
            return true;
        }
        if cursor > right.start {
            return false;
        }

        let mut rests = note_infos
            .iter()
            .filter(|info| {
                info.staff == left.staff
                    && info.voice == left.voice
                    && info.is_rest
                    && !info.is_hidden
                    && !info.is_chord
                    && info.duration > 0
                    && info.start >= cursor
                    && info.start < right.start
            })
            .collect::<Vec<_>>();
        rests.sort_by_key(|info| (info.start, info.index));
        for rest in rests {
            if rest.start != cursor {
                return false;
            }
            cursor = rest.start + rest.duration;
            if cursor > right.start {
                return false;
            }
        }
        cursor == right.start
    }

    fn observed_beam_state(note_text: &str, wanted_level: usize) -> Option<String> {
        let mut found: Option<String> = None;
        let mut scan = 0_usize;
        while let Some(relative_start) = note_text[scan..].find("<beam") {
            let start = scan + relative_start;
            let open_end = note_text[start..].find('>')? + start;
            let open_tag = &note_text[start..=open_end];
            let content_start = open_end + 1;
            let close = note_text[content_start..].find("</beam>")? + content_start;
            if beam_tag_level(open_tag) == Some(wanted_level) {
                if found.is_some() {
                    return None;
                }
                found = Some(note_text[content_start..close].trim().to_ascii_lowercase());
            }
            scan = close + "</beam>".len();
        }
        found
    }

    fn beam_tag_level(open_tag: &str) -> Option<usize> {
        for quote in ['"', '\''] {
            let needle = format!("number={quote}");
            if let Some(start) = open_tag.find(&needle) {
                let value_start = start + needle.len();
                let value_end = open_tag[value_start..].find(quote)? + value_start;
                return open_tag[value_start..value_end].trim().parse::<usize>().ok();
            }
        }
        Some(1)
    }

    fn retain_only_structurally_bound_external_beams(
        note_text: &str,
        note_index: usize,
        valid_levels: &BTreeSet<(usize, usize)>,
    ) -> (String, usize) {
        let mut out = String::with_capacity(note_text.len());
        let mut removed = 0_usize;
        let mut scan = 0_usize;
        while let Some(relative_start) = note_text[scan..].find("<beam") {
            let start = scan + relative_start;
            out.push_str(&note_text[scan..start]);
            let Some(open_end_relative) = note_text[start..].find('>') else {
                out.push_str(&note_text[start..]);
                return (out, removed);
            };
            let open_end = start + open_end_relative;
            let Some(close_relative) = note_text[open_end + 1..].find("</beam>") else {
                out.push_str(&note_text[start..]);
                return (out, removed);
            };
            let close = open_end + 1 + close_relative;
            let end = close + "</beam>".len();
            let open_tag = &note_text[start..=open_end];
            let state = note_text[open_end + 1..close].trim().to_ascii_lowercase();
            let level = beam_tag_level(open_tag);
            let recognized_state = matches!(state.as_str(), "begin" | "continue" | "end");
            let keep = !recognized_state
                || level.is_none()
                || level.is_some_and(|value| valid_levels.contains(&(note_index, value)));
            if keep {
                out.push_str(&note_text[start..end]);
            } else {
                removed += 1;
            }
            scan = end;
        }
        out.push_str(&note_text[scan..]);
        (out, removed)
    }

    fn count_beam_tags(text: &str) -> usize {
        text.match_indices("<beam").count()
    }
    
    fn strip_beam_tags(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut scan = 0_usize;
        while let Some(relative_start) = text[scan..].find("<beam") {
            let start = scan + relative_start;
            let Some(close_relative) = text[start..].find("</beam>") else {
                out.push_str(&text[scan..]);
                return out;
            };
            let tag_end = start + close_relative + "</beam>".len();
            let line_start = text[..start].rfind('\n').map_or(0, |position| position + 1);
            let mut remove_start = start;
            let mut remove_end = tag_end;
            if text[line_start..start].trim().is_empty() {
                let line_end = text[tag_end..]
                    .find('\n')
                    .map_or(text.len(), |relative| tag_end + relative);
                if text[tag_end..line_end].trim().is_empty() {
                    remove_start = line_start;
                    remove_end = if line_end < text.len() { line_end + 1 } else { line_end };
                }
            }
            out.push_str(&text[scan..remove_start]);
            scan = remove_end;
        }
        out.push_str(&text[scan..]);
        out
    }
    
    fn insert_beam_tags(note_text: &str, states: &[(usize, &'static str)]) -> String {
        if let Some(close_pos) = note_text.rfind("</note>") {
            let insert_pos = ["<notations", "<lyric", "<play", "<listen"]
                .into_iter()
                .filter_map(|marker| note_text.find(marker))
                .min()
                .unwrap_or(close_pos);
            let insert_pos = note_text[..insert_pos]
                .rfind('\n')
                .map_or(insert_pos, |line_break| line_break + 1);
            let mut beams = String::new();
            let mut ordered = states.to_vec();
            ordered.sort_by_key(|(level, _)| *level);
            for (level, state) in ordered {
                beams.push_str(&format!("    <beam number=\"{level}\">{state}</beam>\n"));
            }
            let mut out = String::with_capacity(note_text.len() + beams.len());
            out.push_str(&note_text[..insert_pos]);
            if !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(&beams);
            out.push_str(&note_text[insert_pos..]);
            out
        } else {
            note_text.to_string()
        }
    }


    #[cfg(test)]
    mod metric_beam_decision_trichotomy_tests {
        use super::polish_measure;

        fn note_on_staff(
            step: &str,
            duration: i64,
            note_type: &str,
            staff: &str,
            beams: &str,
        ) -> String {
            format!(
                "<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>{duration}</duration><voice>1</voice><type>{note_type}</type><staff>{staff}</staff>{beams}</note>"
            )
        }

        fn note(step: &str, duration: i64, note_type: &str, beams: &str) -> String {
            note_on_staff(step, duration, note_type, "1", beams)
        }

        #[test]
        fn inserted_beams_precede_existing_notations() {
            let note = "<note>\n<pitch><step>C</step><octave>4</octave></pitch>\n<duration>24</duration>\n<voice>1</voice>\n<type>16th</type>\n<staff>1</staff>\n<notations><articulations><accent/></articulations></notations>\n</note>";
            let out = super::insert_beam_tags(note, &[(1, "begin"), (2, "begin")]);
            assert!(
                out.find("<beam number=\"1\">").expect("beam")
                    < out.find("<notations>").expect("notations"),
                "{out}"
            );
        }

        #[test]
        fn polish_preserves_complete_foreign_beam_truth_outside_positive_plan() {
            let foreign_a = note(
                "E",
                24,
                "16th",
                "<beam number=\"1\">begin</beam><beam number=\"2\">begin</beam>",
            );
            let foreign_b = note(
                "F",
                24,
                "16th",
                "<beam number=\"1\">end</beam><beam number=\"2\">end</beam>",
            );
            let measure = format!(
                "<measure number=\"\
1\"><attributes><divisions>96</divisions></attributes><forward><duration>24</duration></forward>{foreign_a}{foreign_b}</measure>"
            );
            let polished = polish_measure(&measure, 96).expect("polish");

            assert!(polished.text.contains(&foreign_a));
            assert!(polished.text.contains(&foreign_b));
            assert_eq!(polished.stats.removed_beams, 0);
        }

        #[test]
        fn polish_preserves_complete_lower_staff_foreign_beam_truth_without_clef_boundary() {
            let foreign_a = note_on_staff(
                "E",
                24,
                "16th",
                "2",
                "<beam number=\"1\">begin</beam><beam number=\"2\">begin</beam>",
            );
            let foreign_b = note_on_staff(
                "F",
                24,
                "16th",
                "2",
                "<beam number=\"1\">end</beam><beam number=\"2\">end</beam>",
            );
            let measure = format!(
                "<measure number=\"\
1\"><attributes><divisions>96</divisions></attributes><forward><duration>24</duration></forward>{foreign_a}{foreign_b}</measure>"
            );
            let polished = polish_measure(&measure, 96).expect("polish");

            assert!(polished.text.contains(&foreign_a));
            assert!(polished.text.contains(&foreign_b));
            assert_eq!(polished.stats.removed_beams, 0);
        }

        #[test]
        fn polish_drops_foreign_beam_truth_crossing_lower_staff_clef_boundary() {
            let begin = note_on_staff(
                "G",
                24,
                "16th",
                "2",
                "<beam number=\"1\">begin</beam>",
            );
            let end = note_on_staff(
                "A",
                24,
                "16th",
                "2",
                "<beam number=\"1\">end</beam>",
            );
            let clef = "<attributes><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>";
            let measure = format!(
                "<measure number=\"\
1\"><attributes><divisions>96</divisions></attributes><forward><duration>24</duration></forward>{begin}{clef}{end}</measure>"
            );
            let polished = polish_measure(&measure, 96).expect("polish");

            assert!(!polished.text.contains("<beam"));
            assert!(polished.text.contains(clef));
            assert!(polished.text.contains("<pitch><step>G</step><octave>4</octave></pitch>"));
            assert!(polished.text.contains("<pitch><step>A</step><octave>4</octave></pitch>"));
            assert_eq!(polished.stats.removed_beams, 2);
            assert_eq!(polished.stats.lower_staff_clef_beam_boundary_splits, 1);
        }

        #[test]
        fn polish_removes_unbound_foreign_beam_fragment_without_touching_note_payload() {
            let orphan = note(
                "E",
                24,
                "16th",
                "<beam number=\"1\">begin</beam><beam number=\"2\">begin</beam>",
            );
            let measure = format!(
                "<measure number=\"\
1\"><attributes><divisions>96</divisions></attributes><forward><duration>192</duration></forward>{orphan}<forward><duration>72</duration></forward></measure>"
            );
            let polished = polish_measure(&measure, 96).expect("polish");

            assert!(!polished.text.contains("<beam"));
            assert!(polished.text.contains(
                "<pitch><step>E</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff>"
            ));
            assert_eq!(polished.stats.removed_beams, 2);
        }

        #[test]
        fn polish_rejects_cross_beat_beam_pair_as_two_unbound_fragments() {
            let begin = note("C", 24, "16th", "<beam number=\"1\">begin</beam>");
            let end = note("D", 24, "16th", "<beam number=\"1\">end</beam>");
            let measure = format!(
                "<measure number=\"\
1\"><attributes><divisions>96</divisions></attributes><forward><duration>72</duration></forward>{begin}{end}</measure>"
            );
            let polished = polish_measure(&measure, 96).expect("polish");

            assert!(!polished.text.contains("<beam"));
            assert_eq!(polished.stats.removed_beams, 2);
        }

        #[test]
        fn trichotomy_polish_is_immediately_idempotent() {
            let measure = format!(
                "<measure number=\"\
1\"><attributes><divisions>96</divisions></attributes>{}{}</measure>",
                note("C", 48, "eighth", "<beam number=\"1\">end</beam>"),
                note("D", 48, "eighth", "<beam number=\"1\">begin</beam>"),
            );
            let first = polish_measure(&measure, 96).expect("first polish");
            let second = polish_measure(&first.text, 96).expect("second polish");
            assert_eq!(first.text, second.text);
        }
    }
