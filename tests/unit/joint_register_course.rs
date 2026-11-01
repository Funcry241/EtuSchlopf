use super::*;

fn notes(step: &str, octave: u8) -> String {
    (0..16).map(|_| format!("<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>1</duration><voice>Lower</voice><type>16th</type><staff>2</staff></note>")).collect()
}
fn part(id: &str, octave: u8, shifted: bool) -> String {
    let start = if shifted { "<direction><direction-type><octave-shift type=\"up\" size=\"8\" number=\"1\"/></direction-type><staff>2</staff></direction>" } else { "" };
    let stop = if shifted { "<direction><direction-type><octave-shift type=\"stop\" size=\"8\" number=\"1\"/></direction-type><staff>2</staff></direction>" } else { "" };
    format!("<part id=\"{id}\"><measure number=\"Synthetic\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>{start}{}{stop}</measure></part>", notes("D", octave))
}
fn score(parts: &str) -> String { format!("<score-partwise version=\"4.0\">{parts}</score-partwise>") }
fn seed(source: &str) {
    loesche();
    beobachte_vor_k00(source);
    let mut state = sitzung().lock().unwrap();
    let proof = state.beweis.as_mut().unwrap().as_mut().unwrap();
    proof.plaene.clear();
    let all_events = proof.ereignisse.clone();
    for (key, events) in gruppiere_systemfolge(&all_events) {
        let first = events.first().unwrap();
        let last = events.last().unwrap();
        proof.plaene.push(RegisterNotationPlan {
            entscheidungskontext: events.iter().map(|e| e.id).collect(),
            betroffene_ereignisse: events.iter().map(|e| e.id).collect(),
            teil: key.teil, system: key.system,
            beginn_takt_ordinal: first.takt_ordinal, ende_takt_ordinal: last.takt_ordinal,
            beginn: first.kanonischer_einsatz, ende: last.kanonisches_ende,
            darstellung: RegisternotationsDarstellung::Achtvb,
            gruende: vec![RegisterPlanGrund::BestehendeBewertungsformel],
        });
    }
    sortiere_plaene(&mut proof.plaene);
    proof.plan_fingerabdruck = plan_fingerabdruck(&proof.plaene);
}
fn current() -> Vec<RegisternotationsMaterialisierungsPlan> { sichtmaterialisierungsauftrag().unwrap().plaene }
fn objective(proposal: &RegisternotationsSichtmaterialisierungsauftrag, ids: &[&str]) -> Option<BTreeMap<String, JointLowerCourseCost>> {
    let parts = ids.iter().map(|id| part(id, 4, proposal.plaene.iter().any(|p| p.teil == *id))).collect::<String>();
    Some(crate::structural_write_authority::RustigClefSurfaceDecision::joint_lower_course_measurements(&score(&parts)))
}
#[test]
fn complete_actual_clef_objective_can_prefer_loco_without_new_weights() {
    let _guard = sperre_fuer_tests();
    let source = score(&part("Original", 4, false));
    seed(&source);
    let baseline = objective(&sichtmaterialisierungsauftrag().unwrap(), &["Original"]).unwrap();
    vergleiche_gebundene_lesekurse(&source, |proposal| objective(proposal, &["Original"])).unwrap();
    assert!(current().is_empty());
    let revised = objective(&sichtmaterialisierungsauftrag().unwrap(), &["Original"]).unwrap();
    assert!(revised["Original"].total < baseline["Original"].total);
    assert_eq!(revised["Original"].objects, 16);
}
#[test]
fn equal_or_unavailable_course_retains_incumbent() {
    let _guard = sperre_fuer_tests();
    let source = score(&part("Original", 4, false));
    for available in [true, false] {
        seed(&source); let before = current();
        let fixed = objective(&sichtmaterialisierungsauftrag().unwrap(), &["Original"]);
        vergleiche_gebundene_lesekurse(&source, |_| if available { fixed.clone() } else { None }).unwrap();
        assert_eq!(before, current());
    }
}
#[test]
fn a_changed_original_epoch_cannot_revise_the_proposal() {
    let _guard = sperre_fuer_tests();
    let source = score(&part("Original", 4, false));
    seed(&source); let before = current();
    let changed = source.replace("Synthetic", "Another");
    assert!(vergleiche_gebundene_lesekurse(&changed, |_| None).is_err());
    assert_eq!(before, current());
}
#[test]
fn independent_part_duplication_preserves_both_local_improvements() {
    let _guard = sperre_fuer_tests();
    for ids in [vec!["First"], vec!["First", "Independent"]] {
        let source = score(&ids.iter().map(|id| part(id, 4, false)).collect::<String>());
        seed(&source);
        vergleiche_gebundene_lesekurse(&source, |proposal| objective(proposal, &ids)).unwrap();
        assert!(current().is_empty());
    }
}
#[test]
fn a_lower_cost_with_a_changed_object_denominator_is_not_adopted() {
    let _guard = sperre_fuer_tests();
    let source = score(&part("Original", 4, false));
    seed(&source); let before = current();
    vergleiche_gebundene_lesekurse(&source, |proposal| {
        let mut metrics = objective(proposal, &["Original"])?;
        if proposal.plaene.is_empty() { metrics.get_mut("Original")?.objects -= 1; }
        Some(metrics)
    }).unwrap();
    assert_eq!(before, current());
}
#[test]
fn necessary_double_octave_is_strictly_cheaper_than_loco_at_the_actual_owner() {
    let source = part("High", 8, false);
    let high = source.replace(&notes("D", 8), &format!("<direction><direction-type><octave-shift type=\"down\" size=\"15\"/></direction-type><staff>2</staff></direction>{}<direction><direction-type><octave-shift type=\"stop\" size=\"15\"/></direction-type><staff>2</staff></direction>", notes("D", 8)));
    let loco = crate::structural_write_authority::RustigClefSurfaceDecision::joint_lower_course_measurements(&score(&source));
    let shifted = crate::structural_write_authority::RustigClefSurfaceDecision::joint_lower_course_measurements(&score(&high));
    assert!(shifted["High"].total < loco["High"].total);
    assert!(shifted["High"].maximum_ledgers < loco["High"].maximum_ledgers);
    assert_eq!(shifted["High"].objects, loco["High"].objects);
}
#[test]
fn renaming_and_time_scaling_do_not_change_the_complete_objective() {
    let source = score(&part("Original", 4, true));
    let shifted = source.replace("Original", "Renamed").replace("Synthetic", "RelocatedLabel")
        .replace("<divisions>4</divisions>", "<divisions>12</divisions>")
        .replace("<duration>1</duration>", "<duration>3</duration>")
        .replace("Lower", "OtherVoice");
    let first = crate::structural_write_authority::RustigClefSurfaceDecision::joint_lower_course_measurements(&source);
    let second = crate::structural_write_authority::RustigClefSurfaceDecision::joint_lower_course_measurements(&shifted);
    assert_eq!(first["Original"], second["Renamed"]);
}
#[test]
fn leading_silence_relocates_time_without_changing_the_objective() {
    let original = part("Original", 4, true);
    let start = original.find("<attributes>").unwrap();
    let end = original.find("</attributes>").unwrap() + "</attributes>".len();
    let attributes = &original[start..end];
    let body = original[start..].replacen(attributes, "", 1);
    let relocated = format!("<part id=\"Original\"><measure number=\"Prelude\">{attributes}<note><rest/><duration>16</duration><voice>Lower</voice><staff>2</staff></note></measure><measure number=\"Relocated\">{body}");
    let first = crate::structural_write_authority::RustigClefSurfaceDecision::joint_lower_course_measurements(&score(&original));
    let second = crate::structural_write_authority::RustigClefSurfaceDecision::joint_lower_course_measurements(&score(&relocated));
    assert_eq!(first["Original"], second["Original"]);
}
#[test]
fn hidden_held_carriers_and_ties_are_not_lost_from_the_current_input() {
    let original = part("Original", 4, false);
    let tied = original.replacen("</note>", "<tie type=\"start\"/></note>", 1);
    let stop = tied.find("</note>").unwrap() + "</note>".len();
    let tied = format!("{}{}", &tied[..stop], tied[stop..].replacen("</note>", "<tie type=\"stop\"/></note>", 1));
    let hidden = "<backup><duration>16</duration></backup><note print-object=\"no\"><pitch><step>C</step><octave>2</octave></pitch><duration>16</duration><voice>Structural</voice><staff>2</staff></note>";
    let current = score(&tied.replace("</measure>", &format!("{hidden}</measure>")));
    let before = current.clone();
    let metrics = crate::structural_write_authority::RustigClefSurfaceDecision::joint_lower_course_measurements(&current);
    assert_eq!(metrics["Original"].objects, 16);
    assert_eq!(current, before);
}
#[test]
fn unsupported_independent_part_does_not_suppress_a_supported_part() {
    let unsupported = part("Unsupported", 4, false).replace("<sign>F</sign><line>4</line>", "<sign>C</sign><line>3</line>");
    let source = score(&(part("Supported", 4, false) + &unsupported));
    let metrics = crate::structural_write_authority::RustigClefSurfaceDecision::joint_lower_course_measurements(&source);
    assert!(metrics.contains_key("Supported"));
    assert!(!metrics.contains_key("Unsupported"));
}
#[test]
fn a_numerical_switch_without_original_materialization_permission_is_unavailable() {
    let original = part("Original", 4, false);
    let all = notes("D", 4);
    let one = "<note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration><voice>Lower</voice><type>16th</type><staff>2</staff></note>";
    let octave = "<direction><direction-type><octave-shift type=\"up\" size=\"8\"/></direction-type><staff>2</staff></direction>";
    // A low margin makes an initial treble switch genuinely unfavorable;
    // the numerical optimum must use the later octave boundary instead.
    let low = one.replace("<step>D</step>", "<step>C</step>").replace("<octave>4</octave>", "<octave>2</octave>");
    let moving = format!("{}{octave}{}", low.repeat(8), one.repeat(8));
    let control = original.replace(&all, &moving);
    let hidden = "<note print-object=\"no\"><pitch><step>C</step><octave>3</octave></pitch><duration>16</duration><tie type=\"start\"/><voice>Held</voice><staff>2</staff></note><backup><duration>16</duration></backup>";
    let continuation = "<measure number=\"Continuation\"><note print-object=\"no\"><pitch><step>C</step><octave>3</octave></pitch><duration>16</duration><tie type=\"stop\"/><voice>Held</voice><staff>2</staff></note></measure>";
    let protected = control.replace(&moving, &format!("{hidden}{moving}")).replace("</part>", &format!("{continuation}</part>"));
    let available = crate::structural_write_authority::RustigClefSurfaceDecision::joint_lower_course_measurements(&score(&control));
    let unavailable = crate::structural_write_authority::RustigClefSurfaceDecision::joint_lower_course_measurements(&score(&protected));
    assert!(available.contains_key("Original"));
    assert!(!unavailable.contains_key("Original"));
}
