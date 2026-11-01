//! Registry-only controls. No music writer, receipt mint or global session is
//! invoked. These tests do not claim document/adoption atomicity.
use super::*;

fn proposal(label: &str) -> RegisternotationsSichtmaterialisierungsauftrag {
    RegisternotationsSichtmaterialisierungsauftrag {
        plaene: Vec::new(),
        semantischer_plan_fingerabdruck: format!("synthetic-semantic-{label}"),
        sicht_fingerabdruck: format!("synthetic-visible-{label}"),
        semantische_plaene: 1,
        kontaktfusionen: 0,
        stille_bruecken: 0,
    }
}

fn births(display: RegisternotationsDarstellung) -> Vec<Birth> {
    let start = match display {
        RegisternotationsDarstellung::Achtva | RegisternotationsDarstellung::Fuenfzehnma => "down",
        _ => "up",
    };
    [(start, 0), ("stop", 1)].into_iter().map(|(kind, onset)| Birth {
        part: "P".into(), staff: "2".into(), slot: 1, onset,
        kind: kind.into(), display, line: 1,
    }).collect()
}

fn registry_xml(births: &[Birth], ids: &[String]) -> String {
    assert_eq!(births.len(), ids.len());
    let body = births.iter().zip(ids).map(|(birth, id)| {
        let place = match birth.display {
            RegisternotationsDarstellung::Achtva | RegisternotationsDarstellung::Fuenfzehnma => "above",
            _ => "below",
        };
        format!("<direction placement=\"{place}\"><!--{id}--><direction-type><octave-shift type=\"{}\" size=\"{}\" number=\"{}\"/></direction-type><staff>{}</staff></direction>",
            birth.kind, birth.display.groesse(), birth.line, birth.staff)
    }).collect::<String>();
    format!("<score-partwise><part id=\"P\"><measure number=\"synthetic\">{body}</measure></part></score-partwise>")
}

fn original() -> (Session, String) {
    let mut state = Session::default();
    let births = births(RegisternotationsDarstellung::Achtva);
    let prepared = prepare_origin_update(&state, &proposal("old"), &births, None).unwrap();
    let xml = registry_xml(&births, &prepared.ids);
    let validated = validate_origin_update(prepared, &xml).unwrap();
    commit_origin_update(&mut state, validated).unwrap();
    (state, xml)
}

#[test]
fn preparation_and_validation_leave_the_original_session_unchanged() {
    let (state, source) = original();
    let saved = state.clone();
    let births = births(RegisternotationsDarstellung::Achtvb);
    let prepared = prepare_origin_update(&state, &proposal("new"), &births, Some(&source)).unwrap();
    assert_eq!(state, saved);
    let xml = registry_xml(&births, &prepared.ids);
    let _validated = validate_origin_update(prepared, &xml).unwrap();
    assert_eq!(state, saved);
}

#[test]
fn successful_registry_commit_replaces_the_complete_active_set() {
    let (mut state, source) = original();
    let old_ids = state.origins.keys().cloned().collect::<BTreeSet<_>>();
    let births = births(RegisternotationsDarstellung::Achtvb);
    let prepared = prepare_origin_update(&state, &proposal("new"), &births, Some(&source)).unwrap();
    let ids = prepared.ids.clone();
    let xml = registry_xml(&births, &ids);
    let validated = validate_origin_update(prepared, &xml).unwrap();
    assert_eq!(commit_origin_update(&mut state, validated).unwrap(), ids);
    assert!(state.origins.keys().all(|id| !old_ids.contains(id)));
    assert!(state.origins.values().all(|origin| origin.validated));
    validate_materialized_origins(&roxmltree::Document::parse(&xml).unwrap(), &state.origins).unwrap();
}

#[test]
fn incomplete_proposed_document_cannot_change_the_original_registry() {
    let (state, source) = original();
    let saved = state.clone();
    let births = births(RegisternotationsDarstellung::Achtvb);
    let prepared = prepare_origin_update(&state, &proposal("new"), &births, Some(&source)).unwrap();
    let xml = registry_xml(&births[..1], &prepared.ids[..1]);
    assert!(validate_origin_update(prepared, &xml).is_err());
    assert_eq!(state, saved);
}

#[test]
fn stale_commit_preserves_the_intervening_reader_observation() {
    let (mut state, source) = original();
    let births = births(RegisternotationsDarstellung::Achtvb);
    let prepared = prepare_origin_update(&state, &proposal("new"), &births, Some(&source)).unwrap();
    let xml = registry_xml(&births, &prepared.ids);
    let validated = validate_origin_update(prepared, &xml).unwrap();
    state.probes.insert(("another-document".into(), "another-layout".into()), BTreeMap::new());
    let intervening = state.clone();
    assert!(commit_origin_update(&mut state, validated).is_err());
    assert_eq!(state, intervening);
}

#[test]
fn existing_downstream_observation_refuses_replacement_preparation() {
    let (mut state, source) = original();
    state.probes.insert(("observed-document".into(), "observed-layout".into()), BTreeMap::new());
    let saved = state.clone();
    assert!(prepare_origin_update(&state, &proposal("new"), &[], Some(&source)).is_err());
    assert_eq!(state, saved);
}

#[test]
fn foreign_or_incomplete_original_document_cannot_retire_origins() {
    let (state, source) = original();
    let saved = state.clone();
    let foreign = source.replace("<staff>2</staff>", "<staff>1</staff>");
    assert!(prepare_origin_update(&state, &proposal("new"), &[], Some(&foreign)).is_err());
    assert_eq!(state, saved);
}

#[test]
fn an_empty_replacement_set_still_requires_an_empty_written_marker_set() {
    let (mut state, source) = original();
    let prepared = prepare_origin_update(&state, &proposal("none"), &[], Some(&source)).unwrap();
    assert!(validate_origin_update(prepared, &source).is_err());
    let prepared = prepare_origin_update(&state, &proposal("none"), &[], Some(&source)).unwrap();
    let validated = validate_origin_update(prepared, &registry_xml(&[], &[])).unwrap();
    commit_origin_update(&mut state, validated).unwrap();
    assert!(state.origins.is_empty());
}

#[test]
fn unregistered_or_ambiguous_markers_are_not_silently_ignored() {
    let state = Session::default();
    let births = births(RegisternotationsDarstellung::Achtva);
    for ids in [vec!["ets-ottava-foreign".into()], vec!["ets-ottava-one--><!--ets-ottava-two".into()]] {
        let prepared = prepare_origin_update(&state, &proposal("none"), &[], None).unwrap();
        let xml = registry_xml(&births[..1], &ids);
        assert!(validate_origin_update(prepared, &xml).is_err());
    }
    assert_eq!(state, Session::default());
}

#[test]
fn initial_issuance_retains_birth_order_and_existing_entries() {
    let (state, _) = original();
    let births = births(RegisternotationsDarstellung::Achtvb);
    let prepared = prepare_origin_update(&state, &proposal("additional"), &births, None).unwrap();
    assert_eq!(prepared.ids.len(), births.len());
    for (birth, id) in births.iter().zip(&prepared.ids) {
        assert_eq!(&prepared.proposed.origins[id].birth, birth);
    }
    for (id, origin) in &state.origins {
        assert_eq!(prepared.proposed.origins.get(id), Some(origin));
    }
}

fn first_direction_range(source: &str) -> Range<usize> {
    roxmltree::Document::parse(source).unwrap().descendants()
        .find(|node| node.has_tag_name("direction")).unwrap().range()
}

#[test]
fn original_removal_evidence_accepts_only_a_complete_confirmed_owned_direction() {
    let (state, source) = original();
    let range = first_direction_range(&source);
    validate_original_direction_removal(&state, &source, range.clone()).unwrap();
    assert!(validate_original_direction_removal(&state, &source, range.start + 1..range.end).is_err());
    assert!(validate_original_direction_removal(&Session::default(), &source, range.clone()).is_err());
    let mut unconfirmed = state.clone();
    for origin in unconfirmed.origins.values_mut() { origin.validated = false; }
    assert!(validate_original_direction_removal(&unconfirmed, &source, range).is_err());
}

#[test]
fn original_removal_evidence_refuses_other_music_even_with_a_valid_marker() {
    let (state, source) = original();
    for extra in [
        "<direction-type><dynamics><p/></dynamics></direction-type>",
        "<direction-type><words>cantabile</words></direction-type>",
        "<sound tempo=\"60\"/>",
        "<note><rest/><duration>1</duration></note>",
        "<direction-type><!--foreign annotation--></direction-type>",
        "<direction-type>foreign text</direction-type>",
        "<direction-type><?foreign keep-this?></direction-type>",
        "<staff>1</staff>",
    ] {
        let changed = source.replacen("</direction>", &format!("{extra}</direction>"), 1);
        assert!(validate_original_direction_removal(&state, &changed, first_direction_range(&changed)).is_err(), "{extra}");
    }
}

#[test]
fn original_removal_evidence_does_not_borrow_duplicate_or_changed_origins() {
    let (state, source) = original();
    let range = first_direction_range(&source);
    let doubled = source.replacen("</measure>", &format!("{}</measure>", &source[range]), 1);
    assert!(validate_original_direction_removal(&state, &doubled, first_direction_range(&doubled)).is_err());
    for changed in [source.replace("<staff>2</staff>", "<staff>1</staff>"),
        source.replacen("size=\"8\"", "size=\"15\"", 1)] {
        assert!(validate_original_direction_removal(&state, &changed, first_direction_range(&changed)).is_err());
    }
}

#[test]
fn original_removal_evidence_preserves_the_session_on_success_and_failure() {
    let (mut state, source) = original();
    let saved = state.clone();
    validate_original_direction_removal(&state, &source, first_direction_range(&source)).unwrap();
    assert_eq!(state, saved);
    state.probes.insert(("observed".into(), "layout".into()), BTreeMap::new());
    let observed = state.clone();
    assert!(validate_original_direction_removal(&state, &source, first_direction_range(&source)).is_err());
    assert_eq!(state, observed);
}

#[test]
fn original_removal_witness_binds_exactly_the_prepared_document_and_operation() {
    use crate::arrangement::representation_transport::OriginalOctaveRemovalEvidence;
    let (state, source) = original();
    let saved = state.clone();
    let range = first_direction_range(&source);
    validate_original_direction_removal(&state, &source, range.clone()).unwrap();
    let witness = ConfirmedOriginalOctaveRemoval {
        source: std::sync::Arc::from(source.as_str()), range: range.clone(),
    };
    witness.verifies(&source, &range).unwrap();
    assert!(witness.verifies(&format!("{source} "), &range).is_err());
    assert!(witness.verifies(&source, &(range.start + 1..range.end)).is_err());
    let mut proposed = source.clone();
    proposed.replace_range(range.clone(), "");
    let step = crate::arrangement::representation_transport::RepresentationStep::recorded(
        &source, &proposed, vec![(range.clone(), String::new())]);
    let bound = bind_original_removals_against(&state, step.clone()).unwrap();
    assert_eq!(bound.original_removal_ranges(), vec![range]);
    assert!(bind_original_removals_against(&Session::default(), step).is_err());
    assert_eq!(state, saved);
}
