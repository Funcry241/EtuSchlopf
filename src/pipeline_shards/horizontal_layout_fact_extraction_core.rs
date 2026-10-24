#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MeasureHorizontalLayoutFacts {
    is_initial_measure_in_sequence: bool,
    note_element_count: usize,
    beam_element_count: usize,
    alter_element_count: usize,
    octave_shift_element_count: usize,
    clef_element_count: usize,
    direction_element_count: usize,
    eighth_type_count: usize,
    sixteenth_type_count: usize,
    thirty_second_type_count: usize,
    extreme_register_note_count: usize,
    distinct_voice_count: usize,
    distinct_staff_count: usize,
    lower_clef_element_count: usize,
    lower_visible_pitched_note_count: usize,
    lower_visible_non_rest_note_count: usize,
    lower_visible_rest_count: usize,
    octave_shift_start_count: usize,
    following_accidental_clearance_edges: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LegacyHorizontalLayoutStateObservation {
    has_new_system_print: bool,
    double_quoted_lower_clef_count: usize,
}

fn extract_measure_horizontal_layout_facts(
    measure: &str,
    is_initial_measure_in_sequence: bool,
) -> MeasureHorizontalLayoutFacts {
    MeasureHorizontalLayoutFacts {
        is_initial_measure_in_sequence,
        note_element_count: measure.matches("<note").count(),
        beam_element_count: measure.matches("<beam").count(),
        alter_element_count: measure.matches("<alter>").count(),
        octave_shift_element_count: measure.matches("<octave-shift").count(),
        clef_element_count: measure.matches("<clef").count(),
        direction_element_count: measure.matches("<direction").count(),
        eighth_type_count: measure.matches("<type>eighth</type>").count(),
        sixteenth_type_count: measure.matches("<type>16th</type>").count(),
        thirty_second_type_count: measure.matches("<type>32nd</type>").count(),
        extreme_register_note_count: count_extreme_register_notes(measure),
        distinct_voice_count: collect_simple_tag_values(measure, "voice").len(),
        distinct_staff_count: collect_simple_tag_values(measure, "staff").len(),
        lower_clef_element_count: measure.matches("<clef number=\"2\"").count()
            + measure.matches("<clef number='2'").count(),
        lower_visible_pitched_note_count: count_visible_staff_notes(measure, "2"),
        lower_visible_non_rest_note_count: count_visible_staff_notes_in_measure(measure, "2"),
        lower_visible_rest_count: count_visible_staff_rests(measure, "2"),
        octave_shift_start_count: count_octave_shift_starts_in_fragment(measure),
        following_accidental_clearance_edges: 0,
    }
}

fn observe_legacy_horizontal_layout_state_from_planned_system_state(
    measure: &str,
    planned_starts_new_system: bool,
) -> LegacyHorizontalLayoutStateObservation {
    LegacyHorizontalLayoutStateObservation {
        has_new_system_print: planned_starts_new_system,
        double_quoted_lower_clef_count: measure.matches("<clef number=\"2\">").count(),
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct HorizontalLayoutFactSnapshot {
    slots: Vec<MeasureHorizontalLayoutFacts>,
}

impl HorizontalLayoutFactSnapshot {
    fn capture(xml: &str) -> anyhow::Result<Self> {
        let mut slots = Vec::new();
        let mut scan = 0usize;

        while let Some(rel) = xml[scan..].find("<measure") {
            let start = scan + rel;
            let Some(open_end_rel) = xml[start..].find('>') else {
                anyhow::bail!("HorizontalLayoutFactSnapshot: unvollstaendiges measure-Starttag");
            };
            let open_end = start + open_end_rel + 1;
            let Some(close_rel) = xml[open_end..].find("</measure>") else {
                anyhow::bail!("HorizontalLayoutFactSnapshot: unvollstaendiger measure-Block");
            };
            let close_end = open_end + close_rel + "</measure>".len();
            let measure = &xml[start..close_end];
            let is_initial_measure_in_sequence = slots.is_empty();
            slots.push(extract_measure_horizontal_layout_facts(
                measure,
                is_initial_measure_in_sequence,
            ));
            scan = close_end;
        }

        if slots.is_empty() {
            anyhow::bail!("HorizontalLayoutFactSnapshot: keine strukturellen measure-Slots gefunden");
        }
        Ok(Self { slots })
    }

    fn capture_final_notation(xml: &str) -> anyhow::Result<Self> {
        let mut snapshot = Self::capture(xml)?;
        let timeline_facts =
            crate::final_notation_timeline_facts::observe_final_notation_timeline_measure_facts(
                xml,
            )?;
        if snapshot.slots.len() != timeline_facts.len() {
            anyhow::bail!(
                "HorizontalLayoutFinalNotationFacts: strukturelle Slots {} und Timeline-Facts {} besitzen keine exakte Bindung",
                snapshot.slots.len(),
                timeline_facts.len()
            );
        }
        for (slot, timeline) in snapshot.slots.iter_mut().zip(timeline_facts.iter()) {
            slot.following_accidental_clearance_edges =
                timeline.following_accidental_clearance_edges;
        }
        Ok(snapshot)
    }


}

#[cfg(test)]
fn observe_legacy_horizontal_layout_state(measure: &str) -> LegacyHorizontalLayoutStateObservation {
    observe_legacy_horizontal_layout_state_from_planned_system_state(
        measure,
        measure.contains("<print new-system=\"yes\""),
    )
}

#[cfg(test)]
mod horizontal_layout_fact_extraction_tests {
    use super::{
        dense_register_notation_score_and_width, extract_measure_horizontal_layout_facts,
        legacy_pianistic_readability_projection, observe_legacy_horizontal_layout_state,
        visual_layout_pressure_score,
    };

    fn legacy_dense_register_notation_score_and_width(
        measure: &str,
        first_measure: bool,
    ) -> (i32, usize) {
        let lower_clefs = measure.matches("<clef number=\"2\"").count() as i32
            + measure.matches("<clef number='2'").count() as i32;
        let effective_lower_clefs = if first_measure {
            lower_clefs.saturating_sub(1)
        } else {
            lower_clefs
        };
        let lower_visible_notes = super::count_visible_staff_notes(measure, "2") as i32;
        let lower_visible_rests = super::count_visible_staff_rests(measure, "2") as i32;
        let octave_shift_count = measure.matches("<octave-shift").count() as i32;
        let alter_count = measure.matches("<alter>").count() as i32;
        let beam_count = measure.matches("<beam").count() as i32;
        let short_count = measure.matches("<type>16th</type>").count() as i32
            + measure.matches("<type>32nd</type>").count() as i32 * 2;
        let extremes = super::count_extreme_register_notes(measure) as i32;

        let mut score = 0_i32;
        score += effective_lower_clefs * 20;
        score += octave_shift_count * 10;
        score += lower_visible_notes * 3;
        score += lower_visible_rests * 4;
        score += alter_count * 2;
        score += beam_count / 2;
        score += short_count / 2;
        score += extremes * 3;

        let target_width = (1120 + (score.max(0) as usize).saturating_mul(5)).min(1680);
        (score, target_width)
    }

    fn legacy_visual_layout_pressure_score(
        measure: &str,
        first_measure: bool,
    ) -> (i32, Vec<&'static str>) {
        let note_count = measure.matches("<note").count() as i32;
        let beam_count = measure.matches("<beam").count() as i32;
        let alter_count = measure.matches("<alter>").count() as i32;
        let octave_shift_count = measure.matches("<octave-shift").count() as i32;
        let clef_count = measure.matches("<clef").count() as i32;
        let extra_clef_count = if first_measure {
            clef_count.saturating_sub(2)
        } else {
            clef_count
        };
        let direction_count = measure.matches("<direction").count() as i32;
        let short_note_count = measure.matches("<type>16th</type>").count() as i32
            + measure.matches("<type>32nd</type>").count() as i32
            + measure.matches("<type>eighth</type>").count() as i32 / 2;
        let high_or_low_notes = super::count_extreme_register_notes(measure) as i32;
        let voice_count = super::collect_simple_tag_values(measure, "voice").len() as i32;
        let staff_count = super::collect_simple_tag_values(measure, "staff").len() as i32;

        let mut score = 0_i32;
        let mut reasons = Vec::new();
        if note_count >= 24 {
            score += note_count / 2;
            reasons.push("Notendichte");
        }
        if beam_count >= 12 {
            score += beam_count / 3;
            reasons.push("Balkendichte");
        }
        if alter_count >= 8 {
            score += alter_count;
            reasons.push("Vorzeichendichte");
        }
        if octave_shift_count > 0 {
            score += 11 + octave_shift_count * 2;
            reasons.push("Oktavierungszeichen");
        }
        if extra_clef_count > 0 {
            score += 12 + extra_clef_count * 3;
            reasons.push("temporäre Schlüsselwechsel");
        }
        if direction_count >= 2 {
            score += direction_count * 2;
            reasons.push("Richtungs-/Dynamikzeichen");
        }
        if short_note_count >= 16 {
            score += short_note_count / 3;
            reasons.push("kurze Notenwerte");
        }
        if high_or_low_notes >= 4 {
            score += high_or_low_notes * 2;
            reasons.push("extreme Registerlage");
        }
        if voice_count >= 3 || staff_count >= 2 {
            score += 5;
            reasons.push("mehrere Stimmen/Systeme");
        }

        (score, reasons)
    }

    fn legacy_pianistic_readability_projection_from_measure(measure: &str) -> (bool, i32) {
        let clefs = measure.matches("<clef number=\"2\">").count();
        let octave_starts = super::count_octave_shift_starts_in_fragment(measure);
        let lower_notes = super::count_visible_staff_notes_in_measure(measure, "2");
        let extreme_notes = super::count_extreme_register_notes(measure);
        let has_dense_lower_clef_octave = clefs >= 2 && octave_starts >= 1 && lower_notes >= 4;
        let score = (clefs as i32 * 18)
            + (octave_starts as i32 * 12)
            + (lower_notes as i32 * 2)
            + (extreme_notes as i32 * 2)
            + if measure.contains("<print new-system=\"yes\"") {
                0
            } else {
                6
            };
        (has_dense_lower_clef_octave && score >= 62, score)
    }

    fn parity_fragments() -> Vec<(bool, &'static str)> {
        vec![
            (
                true,
                r#"<measure number="1"><attributes><clef number="1"><sign>G</sign></clef><clef number="2"><sign>F</sign></clef></attributes><note><pitch><step>C</step><octave>2</octave></pitch><type>16th</type><voice>1</voice><staff>2</staff><beam number="1">begin</beam></note><note><rest/><type>16th</type><voice>2</voice><staff>2</staff></note></measure>"#,
            ),
            (
                false,
                r#"<measure number="7"><print new-system="yes"/><attributes><clef number="2"><sign>F</sign></clef><clef number="2"><sign>G</sign></clef></attributes><direction><direction-type><octave-shift type="down" size="8"/></direction-type></direction><note><pitch><step>C</step><alter>1</alter><octave>6</octave></pitch><type>32nd</type><voice>1</voice><staff>2</staff><beam number="1">begin</beam></note><note><pitch><step>D</step><octave>6</octave></pitch><type>16th</type><voice>2</voice><staff>2</staff><beam number="1">end</beam></note><note><pitch><step>E</step><octave>6</octave></pitch><type>eighth</type><voice>3</voice><staff>2</staff></note><note><pitch><step>F</step><octave>6</octave></pitch><type>eighth</type><voice>3</voice><staff>2</staff></note></measure>"#,
            ),
            (
                false,
                r#"<measure number="12"><attributes><clef number='2'><sign>F</sign></clef></attributes><direction><direction-type><words>dolce</words></direction-type></direction><note print-object="no"><pitch><step>C</step><octave>1</octave></pitch><type>16th</type><voice>1</voice><staff>2</staff></note><note><rest/><type>eighth</type><voice>2</voice><staff>2</staff></note></measure>"#,
            ),
        ]
    }

    #[test]
    fn fact_projection_preserves_dense_register_legacy_semantics() {
        for (is_initial_measure_in_sequence, measure) in parity_fragments() {
            let facts = extract_measure_horizontal_layout_facts(
                measure,
                is_initial_measure_in_sequence,
            );
            assert_eq!(
                dense_register_notation_score_and_width(&facts),
                legacy_dense_register_notation_score_and_width(
                    measure,
                    is_initial_measure_in_sequence,
                )
            );
        }
    }

    #[test]
    fn fact_projection_preserves_visual_pressure_legacy_semantics() {
        for (is_initial_measure_in_sequence, measure) in parity_fragments() {
            let facts = extract_measure_horizontal_layout_facts(
                measure,
                is_initial_measure_in_sequence,
            );
            assert_eq!(
                visual_layout_pressure_score(&facts),
                legacy_visual_layout_pressure_score(
                    measure,
                    is_initial_measure_in_sequence,
                )
            );
        }
    }

    #[test]
    fn fact_plus_legacy_layout_state_preserves_pianistic_readability_semantics() {
        for (is_initial_measure_in_sequence, measure) in parity_fragments() {
            let facts = extract_measure_horizontal_layout_facts(
                measure,
                is_initial_measure_in_sequence,
            );
            let state = observe_legacy_horizontal_layout_state(measure);
            assert_eq!(
                legacy_pianistic_readability_projection(&facts, &state),
                legacy_pianistic_readability_projection_from_measure(measure)
            );
        }
    }

    #[test]
    fn horizontal_layout_facts_expose_observation_not_decision() {
        let measure = "<measure number=\"\
9\"><print new-system=\"yes\"/><note><pitch><step>C</step><octave>4</octave></pitch><voice>1</voice><staff>1</staff></note></measure>";
        let facts = extract_measure_horizontal_layout_facts(measure, false);
        let state = observe_legacy_horizontal_layout_state(measure);

        assert_eq!(facts.note_element_count, 1);
        assert!(!facts.is_initial_measure_in_sequence);
        assert!(state.has_new_system_print);
    }
    #[test]
    fn planned_system_state_observation_matches_legacy_materialized_state() {
        let measure = r#"<measure><print new-system="yes"/><clef number="2"><sign>F</sign></clef></measure>"#;
        let legacy = observe_legacy_horizontal_layout_state(measure);
        let planned = super::observe_legacy_horizontal_layout_state_from_planned_system_state(
            measure,
            true,
        );
        assert_eq!(planned, legacy);
    }



}
