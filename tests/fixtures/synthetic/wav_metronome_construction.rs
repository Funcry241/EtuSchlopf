//! Corpus-independent functional tests; compiled only by the cfg(test) module.
fn insert_tempo(xml: &mut String, bpm: u32, staff: &str) -> usize {
    let locator = crate::structural_write_authority::first_structural_measure_locator(xml).unwrap();
    super::insert_tempo_words_direction_into_measure(xml, &locator, bpm, staff).0
}

fn synthetic_score() -> crate::model::Score {
    use crate::model::{ExpectedDuration, Measure, Part, Score};
    Score {
        title: Some("Synthetic Tempo".to_string()),
        parts: vec![Part {
            id: "SyntheticPart".to_string(),
            name: "Synthetic Instrument".to_string(),
            measures: [(12, 4, 4), (96, 6, 8)]
                .into_iter()
                .enumerate()
                .map(|(index, (divisions, beats, beat_type))| Measure {
                    number: (index + 17).to_string(),
                    divisions,
                    beats,
                    beat_type,
                    expected: ExpectedDuration::new(divisions, beats, beat_type),
                    cursor_min: 0,
                    cursor_max: 0,
                    events: Vec::new(),
                })
                .collect(),
        }],
    }
}

fn synthetic_reference(seconds: Option<f64>) -> crate::reference::ReferenceReport {
    crate::reference::ReferenceReport {
        wav: Some(crate::reference::WavReference {
            path: std::path::PathBuf::from("synthetic.wav"),
            seconds,
        }),
    }
}

fn synthetic_empty_xml() -> String {
    let score = synthetic_score();
    let first = &score.parts[0].measures[0].number;
    format!(
        "<score-partwise><part id=\"Synthetic\"><measure number=\"{first}\"><attributes/></measure></part></score-partwise>"
    )
}

// Independent parsed oracle: one container must own both semantics.
fn assert_coherent_initial_tempo(xml: &str, bpm: u32) {
    let doc = roxmltree::Document::parse(xml).unwrap();
    let first = doc
        .descendants()
        .find(|node| node.has_tag_name("measure"))
        .unwrap();
    let sound = first
        .descendants()
        .filter(|node| node.has_tag_name("sound") && node.attribute("tempo").is_some())
        .collect::<Vec<_>>();
    assert_eq!(sound.len(), 1);
    let value = bpm.to_string();
    assert_eq!(sound[0].attribute("tempo"), Some(value.as_str()));
    let direction = sound[0].parent().unwrap();
    assert!(direction.has_tag_name("direction"));
    let words = direction
        .descendants()
        .find(|node| node.has_tag_name("words"))
        .unwrap()
        .text()
        .unwrap();
    assert_eq!(words, format!("♩ = {bpm}"));
    assert_eq!(
        first
            .children()
            .filter(|node| node.has_tag_name("direction"))
            .count(),
        1
    );
    assert_eq!(
        doc.descendants()
            .filter(|node| node.has_tag_name("sound") && node.attribute("tempo").is_some())
            .count(),
        1
    );
}

#[test]
fn coherent_tempo_preserves_notes_and_unrelated_measure_under_renaming_and_relocation() {
    for (part, number) in [("SyntheticAlpha", "17"), ("SyntheticBeta", "43")] {
        for unrelated in [
            "",
            "<direction><direction-type><words>synthetic annotation</words></direction-type></direction>",
        ] {
            let note = "<note><pitch><step>D</step><octave>4</octave></pitch><duration>12</duration><voice>1</voice><staff>1</staff></note>";
            let next = number.parse::<usize>().unwrap() + 1;
            let other = format!("<measure number=\"{next}\">{unrelated}{note}{note}</measure>");
            let mut xml = format!(
                "<score-partwise><part id=\"{part}\"><measure number=\"{number}\"><attributes><divisions>12</divisions></attributes>{note}</measure>{other}</part></score-partwise>"
            );
            assert_eq!(insert_tempo(&mut xml, 105, "1"), 1);
            assert_coherent_initial_tempo(&xml, 105);
            assert!(xml.contains(&other));
            assert_eq!(xml.matches(note).count(), 3);
            let once = xml.clone();
            assert_eq!(insert_tempo(&mut xml, 105, "1"), 0);
            assert_eq!(xml, once);
        }
    }
}

#[test]
fn real_duration_signature_feeds_the_expression_wrapper_with_unchanged_clamps() {
    let score = synthetic_score(); // 4/4 plus 6/8 = seven quarter notes.
    for (seconds, expected) in [(4.0, 105), (60.0, 40), (1.0, 220)] {
        let signature = crate::reference::build_reference_signature(
            &synthetic_reference(Some(seconds)),
            &score,
        );
        assert_eq!(signature.estimated_bpm, Some(420.0 / seconds));
        let mut xml = synthetic_empty_xml();
        super::apply_expression_polish(&mut xml, signature.estimated_bpm);
        let doc = roxmltree::Document::parse(&xml).unwrap();
        let sound = doc
            .descendants()
            .filter(|node| node.has_tag_name("sound") && node.attribute("tempo").is_some())
            .collect::<Vec<_>>();
        assert_eq!(sound.len(), 1);
        let value = expected.to_string();
        assert_eq!(sound[0].attribute("tempo"), Some(value.as_str()));
        assert!(
            sound[0]
                .parent()
                .unwrap()
                .descendants()
                .any(|node| node.has_tag_name("words")
                    && node.text() == Some(format!("♩ = {expected}").as_str()))
        );
    }
}

#[test]
fn expression_rounds_before_clamping_and_rejects_absent_tempo() {
    for (estimate, expected) in [(100.49, 100), (100.5, 101), (39.49, 40), (220.5, 220)] {
        let mut xml = synthetic_empty_xml();
        super::apply_expression_polish(&mut xml, Some(estimate));
        let doc = roxmltree::Document::parse(&xml).unwrap();
        let sound = doc
            .descendants()
            .find(|node| node.has_tag_name("sound") && node.attribute("tempo").is_some())
            .unwrap();
        let value = expected.to_string();
        assert_eq!(sound.attribute("tempo"), Some(value.as_str()));
    }
    for seconds in [None, Some(0.0), Some(-1.0)] {
        let signature = crate::reference::build_reference_signature(
            &synthetic_reference(seconds),
            &synthetic_score(),
        );
        assert_eq!(signature.estimated_bpm, None);
        let mut xml = synthetic_empty_xml();
        super::apply_expression_polish(&mut xml, signature.estimated_bpm);
        let doc = roxmltree::Document::parse(&xml).unwrap();
        assert!(
            !doc.descendants()
                .any(|node| node.has_tag_name("sound") && node.attribute("tempo").is_some())
        );
    }
    let mut absent = "<score-partwise/>".to_string();
    super::apply_expression_polish(&mut absent, Some(105.0));
    assert_eq!(absent, "<score-partwise/>");
}

#[test]
fn signature_uses_first_part_and_scales_with_actual_duration_not_part_count() {
    let mut score = synthetic_score();
    let reference = synthetic_reference(Some(4.0));
    let before = crate::reference::build_reference_signature(&reference, &score).estimated_bpm;
    let mut independent = score.parts[0].clone();
    independent.id = "IndependentSyntheticPart".to_string();
    score.parts.push(independent);
    score.title = Some("Renamed Synthetic".to_string());
    assert_eq!(
        crate::reference::build_reference_signature(&reference, &score).estimated_bpm,
        before
    );
    let repeated = score.parts[0].measures.clone();
    score.parts[0].measures.extend(repeated);
    assert_eq!(
        crate::reference::build_reference_signature(&reference, &score).estimated_bpm,
        Some(210.0)
    );
    score.parts.clear();
    assert_eq!(
        crate::reference::build_reference_signature(&reference, &score).estimated_bpm,
        None
    );
}

#[test]
fn combined_direction_has_one_receipt_and_survives_real_master_adoption() {
    use crate::master_state_identity::MasterLineage;
    use crate::master_state_identity::tests::synthetic_genesis_for_tests;
    use crate::structural_write_authority::MasterAdoptedState;
    let xml = synthetic_empty_xml();
    let lineage = MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(&xml));
    let mut adopted = MasterAdoptedState::adopt_genesis_state(xml, lineage);
    let count = adopted.adopt_in_place_transformation("SyntheticTempoConstruction", |xml| {
        let locator =
            crate::structural_write_authority::first_structural_measure_locator(xml).unwrap();
        let (count, receipts) =
            super::insert_tempo_words_direction_into_measure(xml, &locator, 105, "1");
        assert_eq!(receipts.len(), 1);
        (count, receipts)
    });
    assert_eq!(count, 1);
    assert_coherent_initial_tempo(adopted.document(), 105);
    assert!(
        adopted
            .lineage()
            .lineage_continuity_holds(adopted.document())
    );
    let before = adopted.document().to_string();
    adopted.adopt_in_place_transformation("SyntheticTempoRepeated", |xml| {
        let locator =
            crate::structural_write_authority::first_structural_measure_locator(xml).unwrap();
        let (count, receipts) =
            super::insert_tempo_words_direction_into_measure(xml, &locator, 105, "1");
        assert_eq!(count, 0);
        assert_eq!(receipts.len(), 0);
        ((), receipts)
    });
    assert_eq!(adopted.document(), before);
    assert!(
        adopted
            .lineage()
            .lineage_continuity_holds(adopted.document())
    );
}
