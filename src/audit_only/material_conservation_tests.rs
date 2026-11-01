use super::*;
fn xml(notes: &str) -> String {
    format!(
        "<score-partwise><part id='P'><measure number='x'><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{notes}</measure></part></score-partwise>"
    )
}
fn note(visible: bool, stop: bool) -> String {
    format!(
        "<note print-object='{}'><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration>{}<voice>1</voice><staff>1</staff></note>",
        if visible { "yes" } else { "no" },
        if stop { "<tie type='stop'/>" } else { "" }
    )
}
fn source(ordinal: usize, retained: bool) -> SourceRow {
    SourceRow {
        ordinal,
        retained,
        raw_onset: ordinal as i64,
        raw_duration: 4,
        pitch: 60,
        removal_reason: None,
        restified_at_entry: Some(false),
    }
}
fn attack(source: usize, exists: bool) -> AttackRow<'static> {
    AttackRow {
        source,
        exists,
        grace: false,
        measure: 1,
        onset: 0,
        end: 96,
        pitch: 60,
        staff: "1",
        voice: "1",
        notated_duration: Some(96),
    }
}
fn population<'a>(
    sources: &'a [SourceRow],
    retained: &'a [usize],
    attacks: &'a [AttackRow<'a>],
    origins: &'a [OriginRecord<'a>],
) -> MaterialPopulation<'a> {
    MaterialPopulation {
        authority_bound: true,
        canonical: sources.len(),
        lineage: retained.len(),
        active: attacks.iter().filter(|r| r.exists).count(),
        source_divisions: 480,
        sources,
        retained_ids: retained,
        attacks,
        origins,
    }
}
#[test]
fn canonical_and_lineage_populations_are_distinct() {
    let sources = [source(0, false), source(1, true), source(2, true)];
    let attacks = [attack(1, true)];
    let p = populations(&population(&sources, &[1, 2], &attacks, &[]));
    assert!(p.valid);
    assert_eq!(
        (p.removed_before, p.inactive_within, p.missing_origin),
        (1, 1, 1)
    );
}
#[test]
fn missing_origin_is_reported_without_inventing_owner() {
    let sources = [source(0, true)];
    let p = populations(&population(&sources, &[0], &[], &[]));
    assert!(p.valid);
    assert_eq!(p.missing_origin, 1);
}
#[test]
fn exact_existence_membership_requires_all_listed_sources() {
    let a = OriginRecord {
        owner: "SyntheticOwner",
        sources: &[3, 4],
        existence_changes: 1,
        other_changes: 1,
    };
    assert!(!exact_existence_membership(&a, 3));
    let b = OriginRecord {
        owner: "SyntheticOwner",
        sources: &[3, 4],
        existence_changes: 2,
        other_changes: 1,
    };
    assert!(exact_existence_membership(&b, 3));
    assert!(!exact_existence_membership(&b, 5));
    let c = OriginRecord {
        owner: "SyntheticOwner",
        sources: &[3, 3],
        existence_changes: 2,
        other_changes: 0,
    };
    assert!(!exact_existence_membership(&c, 3));
}
#[test]
fn hidden_fragments_split_into_attacks_and_tied_continuations() {
    let notes = material_fragments(&xml(&(note(false, false)
        + &note(false, true)
        + &note(true, false))))
    .unwrap();
    assert_eq!(
        head_counts(&notes),
        HeadCounts {
            visible_attacks: 1,
            hidden_attacks: 1,
            hidden_continuations: 1,
            hidden_fragments: 2
        }
    );
}
#[test]
fn grace_is_not_an_attack_or_hidden_fragment() {
    let text = xml(
        "<note print-object='no'><grace/><pitch><step>C</step><octave>4</octave></pitch></note>",
    );
    assert_eq!(
        head_counts(&material_fragments(&text).unwrap()),
        HeadCounts::default()
    );
}
#[test]
fn duplicate_source_identity_invalidates_population() {
    let sources = [source(0, true), source(0, true)];
    assert!(!populations(&population(&sources, &[0], &[], &[])).valid);
}
#[test]
fn active_identity_outside_retained_population_is_invalid() {
    let sources = [source(0, true)];
    let attacks = [attack(4, true)];
    assert!(!populations(&population(&sources, &[0], &attacks, &[])).valid);
}
#[test]
fn notated_duration_reference_is_separate_from_raw_source_ticks() {
    let sources = [source(0, true)];
    let mut attacks = [attack(0, true)];
    assert_eq!(
        populations(&population(&sources, &[0], &attacks, &[])).duration_changed,
        0
    );
    attacks[0].end = 48;
    assert_eq!(
        populations(&population(&sources, &[0], &attacks, &[])).duration_changed,
        1
    );
    attacks[0].notated_duration = None;
    assert_eq!(
        populations(&population(&sources, &[0], &attacks, &[])).duration_unknown,
        1
    );
}
#[test]
fn hidden_origin_remains_unknown_even_with_existence_record() {
    let sources = [source(0, true)];
    let attacks = [attack(0, true)];
    let origins = [OriginRecord {
        owner: "SyntheticOwner",
        sources: &[0],
        existence_changes: 1,
        other_changes: 0,
    }];
    let mut out = String::new();
    append_material_conservation(
        &mut out,
        &population(&sources, &[0], &attacks, &origins),
        &xml(&note(false, false)),
        None,
    );
    assert!(out.contains("hidden_attacks 1; proven_owner 0; unreceipted 1"));
}
#[test]
fn duplicate_class_requires_equal_first_fragment_duration() {
    let before = material_fragments(&xml(&(note(true, false)
        + "<backup><duration>4</duration></backup>"
        + &note(true, false))))
    .unwrap();
    let after = material_fragments(&xml(&note(true, false))).unwrap();
    let mut out = String::new();
    append_render_difference(&mut out, &before, &after);
    assert!(out.contains("compatible_reduction 1; absent_classes 0; added_heads 0"));
    let changed = material_fragments(&xml(
        &note(true, false).replace("<duration>4</duration>", "<duration>8</duration>")
    ))
    .unwrap();
    out.clear();
    append_render_difference(&mut out, &before, &changed);
    assert!(out.contains("compatible_reduction 0; absent_classes 2; added_heads 1"));
}
#[test]
fn report_cardinality_is_measured_without_source_binding_claim() {
    let sources = [source(0, true)];
    let attacks = [attack(0, true)];
    let mut out = String::new();
    append_material_conservation(
        &mut out,
        &population(&sources, &[0], &attacks, &[]),
        &xml(&note(true, false)),
        None,
    );
    assert!(out.contains("lineage_identity true; canonical_identity true"));
    assert!(out.contains("identity_kind cardinality_only_not_a_late_source_binding"));
    assert!(out.contains("DOCUMENT_UNAVAILABLE"));
}
#[test]
fn malformed_document_has_unknown_counts() {
    let mut out = String::new();
    append_material_conservation(&mut out, &population(&[], &[], &[], &[]), "broken", None);
    assert!(out.contains("MODEL_INPUT_UNAVAILABLE"));
    assert!(out.contains("counts unknown"));
}

#[test]
fn visible_continuation_is_separate_from_visible_attack_partner() {
    let master = material_fragments(&xml(&note(true, false))).unwrap();
    let render = material_fragments(&xml(&note(true, true))).unwrap();
    let mut out = String::new();
    append_render_difference(&mut out, &master, &render);
    assert!(out.contains("compatible_reduction 0; absent_classes 1; added_heads 0; with_visible_continuation_partner 1"));
    assert_eq!(head_counts(&render).visible_attacks, 0);
}
