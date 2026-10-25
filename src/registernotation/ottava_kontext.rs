//! Read-only register/visible-plan evidence and final XML continuation census.
//! No cue preference, PDF geometry, release decision or musical write is inferred.

use super::{
    RegisternotationsBeweis, RegisternotationsDarstellung,
    RegisternotationsSichtmaterialisierungsauftrag,
};
use roxmltree::Node;
use std::collections::BTreeMap;
use std::fmt::Write as _;

#[derive(Debug, Clone)]
pub(crate) struct OttavaCarrier {
    pub(crate) part: String,
    pub(crate) staff: String,
    pub(crate) measure: usize,
    pub(crate) onset: i64,
    pub(crate) display: RegisternotationsDarstellung,
    pub(crate) line: usize,
}

#[derive(Debug, Clone)]
pub(super) struct MaterializedContext {
    pub(super) visible: RegisternotationsSichtmaterialisierungsauftrag,
    pub(super) carriers: Vec<OttavaCarrier>,
}

fn hex(text: &str) -> String {
    let mut out = String::with_capacity(text.len().saturating_mul(2));
    for byte in text.as_bytes() {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

pub(super) fn append_context(
    out: &mut String,
    early: Option<&RegisternotationsBeweis>,
    materialized: Option<&MaterializedContext>,
    master: &str,
    render: Option<&str>,
) {
    out.push_str("OttavaContext: schema 1; MusicalAuthority none; ReaderPreference UNKNOWN; PdfLineContinuity UNKNOWN.\n");
    out.push_str("OttavaContextBasis: captured early planner and validated materialization; final XML observations do not reconstruct earlier identity; XML breaks are layout declarations, not PDF geometry.\n");
    if let Some(early) = early {
        let _ = writeln!(
            out,
            "OttavaEarly: PlanSha256 {}; Events {}; Plans {}.",
            early.plan_fingerabdruck,
            early.ereignisse.len(),
            early.plaene.len()
        );
        for (index, plan) in early.plaene.iter().enumerate() {
            let _ = writeln!(
                out,
                "OttavaSemantic: Plan {}; PartHex {}; StaffHex {}; Measures {}..{}; Canonical {}..{}; Display {}.",
                index,
                hex(&plan.teil),
                hex(&plan.system),
                plan.beginn_takt_ordinal,
                plan.ende_takt_ordinal,
                plan.beginn,
                plan.ende,
                plan.darstellung.bezeichnung()
            );
            for (role, events) in [
                ("Affected", &plan.betroffene_ereignisse),
                ("Context", &plan.entscheidungskontext),
            ] {
                for event in events {
                    let _ = writeln!(
                        out,
                        "OttavaSemanticEvent: Plan {index}; Role {role}; Event {event:?}."
                    );
                }
            }
        }
    } else {
        out.push_str("OttavaEarly: UNKNOWN; missing successful captured planner.\n");
    }
    if let Some(materialized) = materialized {
        let visible = &materialized.visible;
        let _ = writeln!(
            out,
            "OttavaMaterialized: SemanticSha256 {}; VisibleSha256 {}; VisiblePlans {}; Bridges {}; Carriers {}.",
            visible.semantischer_plan_fingerabdruck,
            visible.sicht_fingerabdruck,
            visible.plaene.len(),
            visible.stille_bruecken,
            materialized.carriers.len()
        );
        for (index, plan) in visible.plaene.iter().enumerate() {
            let _ = writeln!(
                out,
                "OttavaVisible: Plan {}; PartHex {}; StaffHex {}; Start {}:{}; End {}:{}; Display {}.",
                index,
                hex(&plan.teil),
                hex(&plan.system),
                plan.beginn_takt_ordinal,
                plan.beginn_einsatz,
                plan.ende_takt_ordinal,
                plan.ende_einsatz,
                plan.darstellung.bezeichnung()
            );
            for reminder in &plan.sichtwiederholungen {
                let _ = writeln!(
                    out,
                    "OttavaVisibleReminder: Plan {index}; MeasureOrdinal {}; Onset {}.",
                    reminder.takt_ordinal, reminder.einsatz
                );
            }
        }
        for carrier in &materialized.carriers {
            let _ = writeln!(
                out,
                "OttavaCarrier: PartHex {}; StaffHex {}; MeasureOrdinal {}; Onset {}; Display {}; Line {}; Origin validated_materialization.",
                hex(&carrier.part),
                hex(&carrier.staff),
                carrier.measure,
                carrier.onset,
                carrier.display.bezeichnung(),
                carrier.line
            );
            append_gap(out, early, carrier);
        }
    } else {
        out.push_str("OttavaMaterialized: UNKNOWN; no validated materialization capture.\n");
    }
    append_final(out, "MasterSnapshot", master);
    match render {
        Some(xml) => append_final(out, "RenderArtifact", xml),
        None => out.push_str(
            "OttavaFinal: RenderArtifact UNKNOWN; unavailable; no Master substitution.\n",
        ),
    }
}

fn append_gap(out: &mut String, early: Option<&RegisternotationsBeweis>, carrier: &OttavaCarrier) {
    let Some(early) = early else {
        out.push_str("OttavaGap: UNKNOWN; early planner unavailable.\n");
        return;
    };
    let right = early
        .ereignisse
        .iter()
        .filter(|event| {
            event.teil == carrier.part
                && event.system == carrier.staff
                && event.takt_ordinal == carrier.measure
                && event.einsatz == carrier.onset
        })
        .collect::<Vec<_>>();
    let starts = right
        .iter()
        .map(|event| event.kanonischer_einsatz)
        .collect::<std::collections::BTreeSet<_>>();
    let [right_start] = starts.iter().copied().collect::<Vec<_>>()[..] else {
        out.push_str("OttavaGap: UNKNOWN; early entry missing or ambiguous.\n");
        return;
    };
    let domain = early
        .ereignisse
        .iter()
        .filter(|event| event.teil == carrier.part && event.system == carrier.staff)
        .collect::<Vec<_>>();
    if domain.iter().any(|event| {
        event.kanonischer_einsatz < right_start && event.kanonisches_ende > right_start
    }) {
        out.push_str("OttavaGap: UNKNOWN; pitched domain overlap at entry.\n");
        return;
    }
    let left_end = domain
        .iter()
        .filter(|event| event.kanonisches_ende <= right_start)
        .map(|event| event.kanonisches_ende)
        .max();
    let Some(left_end) = left_end.filter(|end| *end < right_start) else {
        out.push_str("OttavaGap: UNKNOWN; no positive captured staff-local gap.\n");
        return;
    };
    let left = domain
        .iter()
        .filter(|event| event.kanonisches_ende == left_end)
        .map(|event| event.id)
        .collect::<Vec<_>>();
    let right = right.iter().map(|event| event.id).collect::<Vec<_>>();
    let foreign = early
        .ereignisse
        .iter()
        .filter(|event| {
            event.teil == carrier.part
                && event.system != carrier.staff
                && event.kanonischer_einsatz < right_start
                && event.kanonisches_ende > left_end
        })
        .map(|event| event.id)
        .collect::<Vec<_>>();
    let _ = writeln!(
        out,
        "OttavaGap: Canonical {}..{}; Units captured_planner_scale; Left {:?}; Right {:?}; ForeignStaffEvents {:?}; Scope staff_local_only.",
        left_end, right_start, left, right, foreign
    );
}

fn child_text<'a>(node: Node<'a, '_>, name: &str) -> Option<&'a str> {
    node.children()
        .find(|child| child.has_tag_name(name))
        .and_then(|child| child.text())
}

fn append_final(out: &mut String, artifact: &str, xml: &str) {
    let _ = writeln!(
        out,
        "OttavaFinalArtifact: {artifact}; Bytes {}; Sha256 {}.",
        xml.len(),
        crate::sha256::hex(xml.as_bytes())
    );
    let Ok(document) = roxmltree::Document::parse(xml) else {
        let _ = writeln!(out, "OttavaFinal: {artifact} UNKNOWN; invalid XML.");
        return;
    };
    if !document.root_element().has_tag_name("score-partwise") {
        let _ = writeln!(out, "OttavaFinal: {artifact} UNKNOWN; unsupported root.");
        return;
    }
    let mut count = 0usize;
    for (part_index, part) in document
        .root_element()
        .children()
        .filter(|node| node.has_tag_name("part"))
        .enumerate()
    {
        // MusicXML line numbers are part-local and may be reused after a stop.
        let mut active = BTreeMap::<String, Option<(String, String, String, usize)>>::new();
        for (measure_index, measure) in part
            .children()
            .filter(|node| node.has_tag_name("measure"))
            .enumerate()
        {
            let restart = measure
                .children()
                .filter(|node| node.has_tag_name("print"))
                .any(|node| {
                    node.attribute("new-system") == Some("yes")
                        || node.attribute("new-page") == Some("yes")
                });
            for direction in measure
                .children()
                .filter(|node| node.has_tag_name("direction"))
            {
                for shift in direction
                    .descendants()
                    .filter(|node| node.has_tag_name("octave-shift"))
                {
                    let line = shift.attribute("number").unwrap_or("1").to_string();
                    let staff = child_text(direction, "staff");
                    let placement = direction.attribute("placement");
                    let size = shift.attribute("size");
                    let kind = shift.attribute("type");
                    match kind {
                        Some("up" | "down") => {
                            let value =
                                staff
                                    .zip(size)
                                    .zip(placement)
                                    .map(|((staff, size), place)| {
                                        (
                                            staff.to_string(),
                                            size.to_string(),
                                            place.to_string(),
                                            measure_index + 1,
                                        )
                                    });
                            // Overlapping starts make that serialized line ambiguous until its stop.
                            let value = if active.contains_key(&line) {
                                None
                            } else {
                                value
                            };
                            active.insert(line, value);
                        }
                        Some("stop") => {
                            active.remove(&line);
                        }
                        Some("continue") => {
                            count += 1;
                            let prior = active.get(&line).and_then(Option::as_ref);
                            let consistent = prior.is_some_and(
                                |(previous_staff, previous_size, previous_place, _)| {
                                    staff == Some(previous_staff.as_str())
                                        && size == Some(previous_size.as_str())
                                        && placement == Some(previous_place.as_str())
                                },
                            );
                            let first_pitch = staff.and_then(|staff| {
                                measure.children().find(|node| {
                                    node.has_tag_name("note")
                                        && child_text(*node, "staff") == Some(staff)
                                        && node.children().any(|child| child.has_tag_name("pitch"))
                                })
                            });
                            let before_first = first_pitch
                                .map(|note| direction.range().start < note.range().start);
                            let words = direction
                                .descendants()
                                .filter(|node| node.has_tag_name("words"))
                                .map(|node| {
                                    (
                                        hex(node.text().unwrap_or("")),
                                        node.attribute("print-object").unwrap_or("yes"),
                                    )
                                })
                                .collect::<Vec<_>>();
                            let _ = writeln!(
                                out,
                                "OttavaFinalCue: {artifact}; PartOrdinal {}; PartHex {}; MeasureOrdinal {}; LabelHex {}; StaffHex {}; LineHex {}; SizeHex {}; PlacementHex {}; OffsetHex {}; DirectionPrint {}; Words {:?}; ExplicitRestart {}; BeforeFirstStaffPitch {:?}; PriorLine {:?}; CompatiblePrior {}; DirectionSha256 {}.",
                                part_index + 1,
                                hex(part.attribute("id").unwrap_or("")),
                                measure_index + 1,
                                hex(measure.attribute("number").unwrap_or("")),
                                hex(staff.unwrap_or("")),
                                hex(&line),
                                hex(size.unwrap_or("")),
                                hex(placement.unwrap_or("")),
                                hex(child_text(direction, "offset").unwrap_or("")),
                                direction.attribute("print-object").unwrap_or("yes"),
                                words,
                                restart,
                                before_first,
                                prior,
                                if consistent {
                                    "observed_serialized"
                                } else {
                                    "UNKNOWN"
                                },
                                crate::sha256::hex(xml[direction.range()].as_bytes())
                            );
                            if !consistent {
                                active.insert(line, None);
                            }
                        }
                        _ => {
                            active.insert(line, None);
                        }
                    }
                }
            }
        }
    }
    let _ = writeln!(
        out,
        "OttavaFinalCensus: {artifact}; Continues {count}; LinkToCapturedCarrier not_proven; PdfLineContinuity UNKNOWN.\n"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn direction(kind: &str, size: &str, placement: &str, word: &str) -> String {
        format!(
            "<direction placement=\"{placement}\"><direction-type><octave-shift type=\"{kind}\" size=\"{size}\" number=\"7\"/></direction-type><direction-type><words>{word}</words></direction-type><staff>upper</staff></direction>"
        )
    }

    fn measure(body: &str) -> String {
        format!("<measure number=\"repeat\">{body}</measure>")
    }
    fn score(body: &str) -> String {
        format!("<score-partwise><part id=\"renamed\">{body}</part></score-partwise>")
    }

    #[test]
    fn all_displays_keep_later_cues_and_distinguish_layout_declarations() {
        for (kind, size, place, word) in [
            ("down", "8", "above", "8va"),
            ("down", "15", "above", "15ma"),
            ("up", "8", "below", "8vb"),
            ("up", "15", "below", "15mb"),
        ] {
            let pitch =
                "<note><pitch><step>C</step><octave>4</octave></pitch><staff>upper</staff></note>";
            let body = measure(&direction(kind, size, place, ""))
                + &measure(&format!(
                    "<print new-system=\"yes\"/>{}{}{}",
                    direction("continue", size, place, word),
                    pitch,
                    direction("continue", size, place, word)
                ));
            let xml = score(&body);
            let before = xml.clone();
            let mut out = String::new();
            append_final(&mut out, "RenderArtifact", &xml);
            assert_eq!(xml, before);
            assert_eq!(out.matches("OttavaFinalCue:").count(), 2);
            assert_eq!(
                out.matches("CompatiblePrior observed_serialized").count(),
                2
            );
            assert!(out.contains("BeforeFirstStaffPitch Some(true)"));
            assert!(out.contains("BeforeFirstStaffPitch Some(false)"));
            assert!(out.contains("ExplicitRestart true"));
            assert!(out.contains("PdfLineContinuity UNKNOWN"));
            assert!(out.contains(&hex(word)));
        }
    }

    #[test]
    fn missing_invalid_and_independent_line_state_stay_unknown() {
        let continued = measure(&direction("continue", "8", "above", "8va"));
        let mut out = String::new();
        append_context(&mut out, None, None, &score(&continued), None);
        assert!(out.contains("CompatiblePrior UNKNOWN"));
        assert!(out.contains("RenderArtifact UNKNOWN; unavailable"));
        assert!(out.contains("OttavaEarly: UNKNOWN"));
        let mut bad = String::new();
        append_final(&mut bad, "RenderArtifact", "<broken");
        assert!(bad.contains("UNKNOWN; invalid XML"));
        let started = measure(&direction("down", "8", "above", ""));
        let xml = format!(
            "<score-partwise><part id=\"a\">{started}{continued}</part><part id=\"b\">{continued}</part></score-partwise>"
        );
        let mut out = String::new();
        append_final(&mut out, "RenderArtifact", &xml);
        assert_eq!(
            out.matches("CompatiblePrior observed_serialized").count(),
            1
        );
        assert_eq!(out.matches("CompatiblePrior UNKNOWN").count(), 1);
    }

    #[test]
    fn independent_duplication_preserves_original_facts() {
        let body = measure(&direction("down", "8", "above", ""))
            + &measure(&direction("continue", "8", "above", "8va"));
        let one = score(&body);
        let two = format!(
            "<score-partwise><part id=\"renamed\">{body}</part><part id=\"copy\">{body}</part></score-partwise>"
        );
        let mut first = String::new();
        let mut duplicate = String::new();
        append_final(&mut first, "RenderArtifact", &one);
        append_final(&mut duplicate, "RenderArtifact", &two);
        let row = first
            .lines()
            .find(|line| line.starts_with("OttavaFinalCue:"))
            .unwrap();
        assert!(duplicate.lines().any(|line| line == row));
        assert_eq!(
            duplicate
                .matches("CompatiblePrior observed_serialized")
                .count(),
            2
        );
    }

    #[test]
    fn stopped_changed_or_ambiguous_lines_cannot_prove_continuity() {
        let start = direction("down", "8", "above", "");
        for middle in [direction("stop", "8", "above", ""), start.clone()] {
            let xml = score(&measure(
                &(start.clone() + &middle + &direction("continue", "8", "above", "8va")),
            ));
            let mut out = String::new();
            append_final(&mut out, "RenderArtifact", &xml);
            assert!(out.contains("CompatiblePrior UNKNOWN"));
        }
        let xml = score(&measure(
            &(start + &direction("continue", "15", "below", "15mb")),
        ));
        let mut out = String::new();
        append_final(&mut out, "RenderArtifact", &xml);
        assert!(out.contains("CompatiblePrior UNKNOWN"));
    }

    fn event(
        index: usize,
        staff: &str,
        start: i128,
        end: i128,
    ) -> super::super::RegisternotationsEreignis {
        super::super::RegisternotationsEreignis {
            id: super::super::RegisternotationsEreignisId {
                teil_ordinal: 1,
                takt_ordinal: 1,
                ereignis_ordinal: index,
            },
            teil: "arbitrary".to_string(),
            system: staff.to_string(),
            stimme: "renamed".to_string(),
            takt_ordinal: 1,
            taktbezeichnung: "not-a-number".to_string(),
            einsatz: i64::try_from(start).unwrap(),
            dauer: i64::try_from(end - start).unwrap(),
            kanonischer_einsatz: start,
            kanonisches_ende: end,
            tonhoehe_midi: 60,
            sichtbarkeit: super::super::RegisternotationsSichtbarkeit::Gedruckt,
            akkordton: false,
            ueberbindungsfortsetzung: false,
        }
    }

    #[test]
    fn captured_gaps_keep_other_staff_activity_and_have_no_duration_cutoff() {
        for gap in [1, 11, 100_000] {
            let early = RegisternotationsBeweis {
                ereignisse: vec![
                    event(1, "domain", 0, 9),
                    event(2, "domain", 9 + gap, 20 + gap),
                    event(3, "other", 9, 9 + gap),
                ],
                ..RegisternotationsBeweis::default()
            };
            let carrier = OttavaCarrier {
                part: "arbitrary".to_string(),
                staff: "domain".to_string(),
                measure: 1,
                onset: i64::try_from(9 + gap).unwrap(),
                display: RegisternotationsDarstellung::Achtva,
                line: 3,
            };
            let mut out = String::new();
            append_gap(&mut out, Some(&early), &carrier);
            assert!(out.contains(&format!("Canonical 9..{}", 9 + gap)));
            assert!(out.contains("ForeignStaffEvents [RegisternotationsEreignisId"));
            assert!(out.contains("Scope staff_local_only"));
            assert!(!out.contains("UNKNOWN"));
        }
    }

    #[test]
    fn missing_entry_and_overlapping_domain_do_not_fabricate_a_gap() {
        let carrier = OttavaCarrier {
            part: "arbitrary".to_string(),
            staff: "domain".to_string(),
            measure: 1,
            onset: 10,
            display: RegisternotationsDarstellung::Fuenfzehnmb,
            line: 2,
        };
        for events in [
            vec![],
            vec![event(1, "domain", 0, 12), event(2, "domain", 10, 20)],
        ] {
            let early = RegisternotationsBeweis {
                ereignisse: events,
                ..RegisternotationsBeweis::default()
            };
            let mut out = String::new();
            append_gap(&mut out, Some(&early), &carrier);
            assert!(out.contains("OttavaGap: UNKNOWN"));
        }
    }

    #[test]
    fn session_reset_clears_new_capture_and_report_does_not_replan() {
        let _guard = super::super::sperre_fuer_tests();
        super::super::loesche();
        {
            let mut state = super::super::sitzung().lock().unwrap();
            state.ottava_kontext = Some(MaterializedContext {
                visible: RegisternotationsSichtmaterialisierungsauftrag {
                    plaene: Vec::new(),
                    semantischer_plan_fingerabdruck: "semantic".to_string(),
                    sicht_fingerabdruck: "visible".to_string(),
                    semantische_plaene: 0,
                    kontaktfusionen: 0,
                    stille_bruecken: 0,
                },
                carriers: Vec::new(),
            });
        }
        let mut out = String::new();
        super::super::fuege_ottava_kontext_hinzu(&mut out, &score(""), None);
        assert!(out.contains("VisibleSha256 visible"));
        assert!(super::super::sitzung().lock().unwrap().beweis.is_none());
        super::super::loesche();
        assert!(
            super::super::sitzung()
                .lock()
                .unwrap()
                .ottava_kontext
                .is_none()
        );
    }
}
