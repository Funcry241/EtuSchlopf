use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

// r1280: frozen fixtures (state r1279) instead of live runtime or governed data files.
const IDENTITY: &str = include_str!("ssc_test_fixture_r1279_master_state_identity.txt");
const FIXTURE_ENVELOPES: &str = "schema=ssc_containment_envelopes_v1\n";
const FIXTURE_EXTENSIONS: &str = "schema=ssc_surface_extensions_v1\n";
const FIXTURE_WIRING: &str = "schema=ssc_verdict_wiring_v1\n";
const LEDGER: &str = include_str!("kodex_capability_retirements.txt");
const BEAM_ROW: &str = "row=GOULD_METRIC_BEAM_PRESERVING|Preserving|Projection|beam|beam@number|change,insert,remove|owner_conserved_projection|no|metric beaming regroups beams per beat; pitch and rhythm untouched";
const BEAM_ROW_NARROW: &str = "row=GOULD_METRIC_BEAM_PRESERVING|Preserving|Projection|beam|beam@number|change|owner_conserved_projection|no|metric beaming regroups beams per beat; pitch and rhythm untouched";
const RETIRE_FILE: &str = "src/build_gate/synthetic_retire_kodex.rs";
const RETIRE_SOURCE: &str = "#[test]\nfn keep_check() {}\n#[test]\nfn old_check() {\n    assert!(true);\n}\n";

struct Tree(std::path::PathBuf);
impl Tree {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let t = Self(std::env::temp_dir().join(format!(
            "ssc-governed-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        )));
        t.put(ENVELOPES_PATH, FIXTURE_ENVELOPES);
        t.put(EXTENSIONS_PATH, FIXTURE_EXTENSIONS);
        t.put(WIRING_PATH, FIXTURE_WIRING);
        t.put("src/master_state_identity.rs", IDENTITY);
        t.put(LEDGER_PATH, LEDGER);
        t.put(TRUST_ANCHOR_PATH, "const _: () = assert!(fnv1a64(X) == 0x00000000000000aa);\n");
        t.put(RETIRE_FILE, RETIRE_SOURCE);
        for path in PROOF_FILES {
            t.put(path, "// synthetic proof source\n");
        }
        t
    }
    fn put(&self, p: &str, s: &str) {
        let path = self.0.join(p);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, s).unwrap();
    }
    fn manifest(&self, class: &str, extra: &[&str], changed: &[&str]) {
        let mut text = format!("schema_version=1\nchange_class={class}\nreason=test\n");
        for line in extra {
            text.push_str(line);
            text.push('\n');
        }
        for path in changed {
            text.push_str(&format!("change=modified|{path}|fnv64:0000000000000001\n"));
        }
        self.put(MANIFEST_PATH, &text);
    }
    fn append(&self, p: &str, line: &str) {
        let text = read(&self.0, p).unwrap();
        self.put(p, &format!("{text}{line}\n"));
    }
}
impl Drop for Tree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
const NONE: &str = "music_effect=none";
fn governed(p: &Tree, c: &Tree) -> Result<(), String> {
    validate_transition(&p.0, &c.0, CLASS, false)
}

#[test]
fn parent_knows_exactly_six_kinds_and_rejects_any_other() {
    assert_eq!(KINDS.len(), 6);
    for kind in KINDS {
        parse_govern_value(&format!("{kind}|target|stable reason text")).unwrap();
    }
    for unknown in ["envelope_widen", "proof_rule_any", "self_admit", "relax"] {
        assert!(parse_govern_value(&format!("{unknown}|target|stable reason text")).unwrap_err().contains("unknown govern kind"));
    }
    assert!(parse_govern_value("proof_rule|src/ssc_evidence_proof.rs|short").is_err());
    assert!(parse_govern_value("proof_rule||stable reason text").is_err());
}

#[test]
fn music_effect_is_mandatory_for_governed_change_and_foreign_to_other_classes() {
    assert_eq!(parse_music_effect("none").unwrap(), "none");
    assert!(parse_music_effect("declared|user released clef change on 2026-10-07").is_ok());
    assert!(parse_music_effect("declared").is_err());
    assert!(parse_music_effect("declared|ok").is_err());
    assert!(parse_music_effect("maybe").is_err());
    let p = Tree::new();
    let c = Tree::new();
    c.manifest(CLASS, &[], &[]);
    assert!(governed(&p, &c).unwrap_err().contains("music_effect"));
    c.manifest("RefactorOnly", &[NONE], &[]);
    assert!(validate_transition(&p.0, &c.0, "RefactorOnly", false).unwrap_err().contains("only for GovernedChange"));
    c.manifest(CLASS, &[NONE], &[]);
    governed(&p, &c).unwrap();
}

#[test]
fn validator_code_is_never_a_governed_change_path() {
    let all = KINDS
        .iter()
        .map(|kind| GovernLine { kind: (*kind).into(), target: format!("{RETIRE_FILE}::old_check"), reason: "stable reason text".into() })
        .collect::<Vec<_>>();
    for path in [
        "src/build_gate/ssc_governed_change.rs",
        "src/build_gate/ssc_k2_retirement_governance.rs",
        "src/take_change_control.rs",
        "src/build_gate/semantic_capability_diff.rs",
        "src/bin/etuschlopf_take_bootstrap.rs",
    ] {
        assert!(!kodex_path_admissible(path, &all), "{path}");
    }
    for path in [ENVELOPES_PATH, EXTENSIONS_PATH, WIRING_PATH] {
        assert!(kodex_path_admissible(path, &[]));
    }
    assert!(!kodex_path_admissible(LEDGER_PATH, &[]));
    assert!(!kodex_path_admissible(TRUST_ANCHOR_PATH, &[]));
    assert!(kodex_path_admissible(LEDGER_PATH, &all));
    assert!(kodex_path_admissible(RETIRE_FILE, &all));
}

#[test]
fn envelope_extend_is_admitted_only_when_declared_and_well_formed() {
    let p = Tree::new();
    let c = Tree::new();
    c.append(ENVELOPES_PATH, BEAM_ROW);
    c.manifest(CLASS, &[NONE, "govern=envelope_extend|GOULD_METRIC_BEAM_PRESERVING|owner purpose: metric beaming"], &[ENVELOPES_PATH]);
    governed(&p, &c).unwrap();
    validate_tree(&c.0).unwrap();
    // Undeclared extension: the parent admits nothing implicitly.
    c.manifest(CLASS, &[NONE], &[ENVELOPES_PATH]);
    assert!(governed(&p, &c).unwrap_err().contains("govern=envelope_extend|GOULD_METRIC_BEAM_PRESERVING"));
    // Declared as the wrong kind: narrowing cannot carry a widening.
    c.manifest(CLASS, &[NONE, "govern=envelope_narrow|GOULD_METRIC_BEAM_PRESERVING|owner purpose: metric beaming"], &[ENVELOPES_PATH]);
    assert!(governed(&p, &c).unwrap_err().contains("envelope_extend"));
    // Another class cannot touch the table at all.
    c.manifest("RefactorOnly", &[], &[ENVELOPES_PATH]);
    assert!(validate_transition(&p.0, &c.0, "RefactorOnly", false).unwrap_err().contains("only under GovernedChange"));
}

#[test]
fn envelope_rows_reject_wildcards_unknown_descriptors_and_undeclared_content_change() {
    let header = FIXTURE_ENVELOPES;
    parse_envelopes(&format!("{header}{BEAM_ROW}\n"), IDENTITY).unwrap();
    let bad = [
        BEAM_ROW.replace("|beam|beam@number|", "|*|-|"),
        BEAM_ROW.replace("|beam|beam@number|", "|all|-|"),
        BEAM_ROW.replace("|beam@number|", "|beam@*|"),
        BEAM_ROW.replace("change,insert,remove", "any"),
        BEAM_ROW.replace("change,insert,remove", "change,rewrite"),
        BEAM_ROW.replace("GOULD_METRIC_BEAM_PRESERVING", "INVENTED_OWNER_PRESERVING"),
        BEAM_ROW.replace("|Preserving|Projection|", "|Exact|Edit|"),
        BEAM_ROW.replace("|Preserving|Projection|", "|Preserving|Block|"),
        BEAM_ROW.replace("|beam|beam@number|", "|beam,pitch|pitch.step|"),
        BEAM_ROW.replace("|beam|beam@number|", "|note|-|"),
        BEAM_ROW.replace("|beam|beam@number|", "|beam|clef@number|"),
        BEAM_ROW.replace("|no|", "|maybe|"),
        BEAM_ROW.replace("|no|", "|yes|"),
        BEAM_ROW.replace("metric beaming regroups beams per beat; pitch and rhythm untouched", "short"),
        format!("{BEAM_ROW}|extra"),
    ];
    for row in bad {
        assert!(parse_envelopes(&format!("{header}{row}\n"), IDENTITY).is_err(), "{row}");
    }
    assert!(parse_envelopes(&format!("{header}{BEAM_ROW}\n{BEAM_ROW}\n"), IDENTITY).unwrap_err().contains("more than one"));
    // r1278 F3 (Codex counterexample): a beam owner cannot become a respelling owner by
    // declaring content and writing a purpose text; its frozen purpose class decides.
    let content = BEAM_ROW
        .replace("|beam|beam@number|", "|beam,pitch|pitch.step|")
        .replace("|no|", "|yes|")
        .replace("metric beaming", "content: respelling owner; metric beaming");
    assert!(parse_envelopes(&format!("{header}{content}\n"), IDENTITY).unwrap_err().contains("frozen owner purpose"));
    let respelling = "row=FUNCTIONAL_RESPELLING_EXACT|Exact|Edit|pitch,step,alter,accidental|-|change|edit_script_locality|yes|content: functional respelling from the render key list";
    parse_envelopes(&format!("{header}{respelling}\n"), IDENTITY).unwrap();
    assert!(parse_envelopes("schema=other\n", IDENTITY).is_err());
}

#[test]
fn envelope_narrow_is_admitted_for_subsets_and_removal_but_not_for_widening() {
    let p = Tree::new();
    p.append(ENVELOPES_PATH, BEAM_ROW);
    let c = Tree::new();
    c.append(ENVELOPES_PATH, BEAM_ROW_NARROW);
    c.manifest(CLASS, &[NONE, "govern=envelope_narrow|GOULD_METRIC_BEAM_PRESERVING|owner never inserts beams"], &[ENVELOPES_PATH]);
    governed(&p, &c).unwrap();
    let removed = Tree::new();
    removed.manifest(CLASS, &[NONE, "govern=envelope_narrow|GOULD_METRIC_BEAM_PRESERVING|owner never inserts beams"], &[ENVELOPES_PATH]);
    governed(&p, &removed).unwrap();
    // The reverse direction widens and needs envelope_extend.
    let wide = Tree::new();
    wide.append(ENVELOPES_PATH, BEAM_ROW);
    let narrow_parent = Tree::new();
    narrow_parent.append(ENVELOPES_PATH, BEAM_ROW_NARROW);
    wide.manifest(CLASS, &[NONE, "govern=envelope_narrow|GOULD_METRIC_BEAM_PRESERVING|owner never inserts beams"], &[ENVELOPES_PATH]);
    assert!(governed(&narrow_parent, &wide).unwrap_err().contains("envelope_extend"));
}

#[test]
fn dangling_or_duplicate_govern_lines_are_rejected() {
    let p = Tree::new();
    let c = Tree::new();
    c.manifest(CLASS, &[NONE, "govern=envelope_extend|GOULD_METRIC_BEAM_PRESERVING|owner purpose: metric beaming"], &[]);
    assert!(governed(&p, &c).unwrap_err().contains("dangling"));
    c.manifest(CLASS, &[NONE, "govern=proof_rule|src/ssc_evidence_proof.rs|proof rule change reason", "govern=proof_rule|src/ssc_evidence_proof.rs|proof rule change reason two"], &[]);
    assert!(governed(&p, &c).unwrap_err().contains("duplicate"));
    c.manifest(CLASS, &[NONE, "govern=proof_rule|src/structural_write_authority/mod.rs|not a proof file at all"], &[]);
    assert!(governed(&p, &c).unwrap_err().contains("dangling"));
}

#[test]
fn proof_files_change_only_with_a_per_file_proof_rule() {
    for path in PROOF_FILES {
        let p = Tree::new();
        let c = Tree::new();
        c.put(path, "// changed proof source\n");
        c.manifest(CLASS, &[NONE, &format!("govern=proof_rule|{path}|containment verdict wiring")], &[path]);
        governed(&p, &c).unwrap();
        c.manifest(CLASS, &[NONE], &[path]);
        assert!(governed(&p, &c).unwrap_err().contains("proof_rule"));
        let other = PROOF_FILES.iter().find(|other| **other != path).unwrap();
        c.manifest(CLASS, &[NONE, &format!("govern=proof_rule|{other}|containment verdict wiring")], &[path]);
        assert!(governed(&p, &c).is_err());
        for class in ["RefactorOnly", "KodexOnly", "AuditOnly"] {
            c.manifest(class, &[], &[path]);
            assert!(validate_transition(&p.0, &c.0, class, false).unwrap_err().contains("only under GovernedChange"));
        }
    }
}

#[test]
fn surface_extend_is_append_only_declared_and_never_an_exemption() {
    let p = Tree::new();
    let c = Tree::new();
    c.append(EXTENSIONS_PATH, "variant=SscProofFamily|Containment");
    c.manifest(CLASS, &[NONE, "govern=surface_extend|SscProofFamily::Containment|contract v4 containment family"], &[EXTENSIONS_PATH]);
    governed(&p, &c).unwrap();
    validate_tree(&c.0).unwrap();
    c.manifest(CLASS, &[NONE], &[EXTENSIONS_PATH]);
    assert!(governed(&p, &c).unwrap_err().contains("surface_extend"));
    // Rewriting history is not an extension.
    let p2 = Tree::new();
    p2.append(EXTENSIONS_PATH, "variant=SscProofFamily|Containment");
    let c2 = Tree::new();
    c2.append(EXTENSIONS_PATH, "variant=SscProofFamily|Envelope");
    c2.manifest(CLASS, &[NONE, "govern=surface_extend|SscProofFamily::Envelope|contract v4 containment family"], &[EXTENSIONS_PATH]);
    assert!(governed(&p2, &c2).unwrap_err().contains("append-only"));
    for name in ["Exempt", "OptionalStyle", "HarmlessLayout", "AdministrativeCleanup", "SkipNecessity"] {
        assert!(parse_extensions(&format!("schema=ssc_surface_extensions_v1\nvariant=SscProofFamily|{name}\n")).unwrap_err().contains("exemption"));
    }
}

#[test]
fn verdict_wiring_only_at_declared_a7_adoption_boundaries() {
    let p = Tree::new();
    let c = Tree::new();
    c.append(WIRING_PATH, "wiring=src/master_state_identity.rs::commit");
    c.manifest(CLASS, &[NONE, "govern=verdict_wiring|src/master_state_identity.rs::commit|contract v4 A7 master adoption"], &[WIRING_PATH]);
    governed(&p, &c).unwrap();
    c.manifest(CLASS, &[NONE], &[WIRING_PATH]);
    assert!(governed(&p, &c).unwrap_err().contains("verdict_wiring"));
    assert!(parse_wiring("schema=ssc_verdict_wiring_v1\nwiring=src/structural_write_authority/mod.rs::commit\n").unwrap_err().contains("adoption function"));
    assert!(parse_wiring("schema=ssc_verdict_wiring_v1\nwiring=src/master_state_identity.rs::commit\nwiring=src/master_state_identity.rs::commit\n").is_err());
}

#[test]
fn governed_files_are_installed_only_by_a_named_revision_and_never_disappear() {
    let p = Tree::new();
    for path in [ENVELOPES_PATH, EXTENSIONS_PATH, WIRING_PATH] {
        std::fs::remove_file(p.0.join(path)).unwrap();
    }
    let c = Tree::new();
    c.manifest("KodexOnly", &[], &[]);
    validate_transition(&p.0, &c.0, "KodexOnly", true).unwrap();
    assert!(validate_transition(&p.0, &c.0, "KodexOnly", false).unwrap_err().contains("named K2 governance revision"));
    let p = Tree::new();
    for path in [ENVELOPES_PATH, EXTENSIONS_PATH, WIRING_PATH] {
        let c = Tree::new();
        std::fs::remove_file(c.0.join(path)).unwrap();
        c.manifest(CLASS, &[NONE], &[]);
        assert!(governed(&p, &c).unwrap_err().contains("cannot disappear"), "{path}");
    }
}

fn retire_child(extra_changed: &[&str]) -> Tree {
    let c = Tree::new();
    c.put(RETIRE_FILE, "#[test]\nfn keep_check() {}\n");
    c.append(LEDGER_PATH, &format!("retire=KodexNegativeCheck|{RETIRE_FILE}|old_check#1|fnv64:0000000000000001|reason:synthetic_retirement"));
    let mut changed = vec![RETIRE_FILE, LEDGER_PATH];
    changed.extend_from_slice(extra_changed);
    c.manifest(CLASS, &[NONE, &format!("govern=retire|{RETIRE_FILE}::old_check|synthetic check retired")], &changed);
    c
}

#[test]
fn retire_is_a_pure_ledgered_deletion_without_runtime_change() {
    let p = Tree::new();
    governed(&p, &retire_child(&[])).unwrap();
    // A pin-literal rebind of the trust anchor is part of the deletion.
    let c = retire_child(&[TRUST_ANCHOR_PATH]);
    c.put(TRUST_ANCHOR_PATH, "const _: () = assert!(fnv1a64(X) == 0x00000000000000bb);\n");
    governed(&p, &c).unwrap();
    // Weakening and use stay separate: no runtime change in the same take.
    assert!(governed(&p, &retire_child(&["src/ssc_evidence_proof.rs"])).unwrap_err().contains("runtime change"));
    // Anything but pin literals in the anchor is rejected.
    let c = retire_child(&[TRUST_ANCHOR_PATH]);
    c.put(TRUST_ANCHOR_PATH, "const _: () = assert!(true);\n");
    assert!(governed(&p, &c).unwrap_err().contains("pin literals"));
    // Missing ledger entry.
    let c = retire_child(&[]);
    c.put(LEDGER_PATH, LEDGER);
    assert!(governed(&p, &c).unwrap_err().contains("ledger"));
    // Not a pure deletion.
    let c = retire_child(&[]);
    c.put(RETIRE_FILE, "#[test]\nfn keep_check() { let weakened = 1; }\n");
    assert!(governed(&p, &c).unwrap_err().contains("only delete"));
}

#[test]
fn retire_never_reaches_protected_or_k2_checker_files() {
    for target in [
        "src/take_change_control.rs::validate_incoming_take",
        "src/build_gate/ssc_governed_change.rs::validate_transition",
        "src/build_gate/ssc_k2_retirement_governance.rs::validate_tree",
        "src/build_gate/semantic_capability_diff.rs::validate_policy",
        "src/ssc_evidence_proof.rs::observe_ssc_event_v2",
    ] {
        let p = Tree::new();
        let c = Tree::new();
        c.manifest(CLASS, &[NONE, &format!("govern=retire|{target}|synthetic check retired")], &[]);
        assert!(governed(&p, &c).unwrap_err().contains("protected"), "{target}");
    }
}

#[test]
fn envelope_rows_stay_inside_the_frozen_owner_purpose() {
    let header = FIXTURE_ENVELOPES;
    let ok = [
        "row=STAFF_VOICE_MOVE_EXACT|Exact|Lane|voice,staff|-|change,insert|lane_fields_only|no|sealed F5 owner moves notes between voice and staff lanes",
        "row=RENDER_ROW_44_EXACT|Exact|Lane|note|note@print-object|change|lane_fields_only|no|sealed F7 owner restores visibility of uncovered rests",
        "row=DIRECTION_INSERTION_ADDITIVE|Additive|Insertion|direction|-|insert|single_contiguous_insertion|no|E036 inserts exactly one direction into a measure",
        "row=PIANIST_LAYOUT_PRESERVING|Preserving|Projection|print,system-layout,measure|measure@width|change,insert,remove|owner_conserved_projection|no|E056 sets print and layout breaks only",
        "row=PARTIAL_BEAT_LANE_TRANSFER_PRESERVING|Preserving|Projection|note,forward|-|insert,remove|owner_conserved_projection|yes|content: E057 exchanges forward and note with zero event multiset delta",
    ];
    for row in ok {
        parse_envelopes(&format!("{header}{row}\n"), IDENTITY).unwrap_or_else(|e| panic!("{row}: {e}"));
    }
    let bad = [
        // Lane only for the three sealed lane owners.
        "row=SAFE_TRANSFER_EXACT|Exact|Lane|voice,staff|-|change|lane_fields_only|no|safe transfer between lanes of the same measure",
        // Invariant must be the bound checker of the kind.
        "row=STAFF_VOICE_MOVE_EXACT|Exact|Lane|voice,staff|-|change|owner_conserved_projection|no|sealed F5 owner moves notes between lanes",
        // Layout owner reaching clefs or beams.
        "row=PIANIST_LAYOUT_PRESERVING|Preserving|Projection|print,clef|-|change|owner_conserved_projection|no|E056 sets print and layout breaks only",
        "row=VISUAL_LAYOUT_DEFAULTS_PRESERVING|Preserving|Projection|beam|-|change|owner_conserved_projection|no|E044 layout defaults only, nothing else",
        // Field on a host the purpose does not own.
        "row=GOULD_METRIC_BEAM_PRESERVING|Preserving|Projection|note|note@pitch|change|owner_conserved_projection|no|metric beaming regroups beams per beat only",
        // Barline owner may not remove.
        "row=FINAL_BARLINE_ADDITIVE|Additive|Insertion|barline|-|remove|single_contiguous_insertion|no|E004 inserts the final light-heavy barline",
        // Content change by a no-content purpose.
        "row=DIRECTION_INSERTION_ADDITIVE|Additive|Insertion|direction,chord|-|insert|single_contiguous_insertion|yes|content: E036 inserts exactly one direction",
        // Genesis owners have no envelope.
        "row=MIDI_SEED_GENESIS|Exact|Edit|note|-|change|edit_script_locality|yes|content: midi seed genesis of the first master",
    ];
    for row in bad {
        assert!(parse_envelopes(&format!("{header}{row}\n"), IDENTITY).is_err(), "{row}");
    }
}

#[test]
fn every_frozen_descriptor_has_exactly_one_owner_purpose() {
    let mut frozen = std::collections::BTreeSet::new();
    for line in IDENTITY.lines() {
        let line = line.trim();
        for kind in ["TransformationDescriptor", "PreservingDescriptor", "AdditiveDescriptor"] {
            if let Some(rest) = line.strip_prefix("const ")
                && let Some((name, tail)) = rest.split_once(':')
                && tail.trim_start().starts_with(kind)
                && tail.contains(&format!("= {kind} {{"))
            {
                frozen.insert(name.trim().to_owned());
            }
        }
    }
    let genesis = ["MIDI_SEED_GENESIS", "ARRANGEMENT_GENESIS"];
    let purposes = DESCRIPTOR_PURPOSES.iter().map(|(name, _, _)| (*name).to_owned()).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(purposes.len(), DESCRIPTOR_PURPOSES.len(), "duplicate purpose entry");
    let expected = frozen.iter().filter(|name| !genesis.contains(&name.as_str())).cloned().collect::<std::collections::BTreeSet<_>>();
    assert_eq!(purposes, expected);
    for (name, receipt, classes) in DESCRIPTOR_PURPOSES {
        assert_eq!(descriptor_receipt(IDENTITY, name).unwrap(), *receipt, "{name}");
        assert!(classes.iter().all(|class| PURPOSE_CLASSES.iter().any(|c| c.name == *class)), "{name}");
    }
    for lane in LANE_DESCRIPTORS {
        assert!(DESCRIPTOR_PURPOSES.iter().any(|(name, _, classes)| *name == lane && classes.contains(&"lane")));
    }
}

#[test]
fn surface_extension_is_only_a_bare_unit_variant_of_a_listed_enum() {
    let header = "schema=ssc_surface_extensions_v1\n";
    parse_extensions(&format!("{header}variant=SscProofFamily|Edit\n")).unwrap();
    parse_extensions(&format!("{header}variant=SscEvidenceIntegrityFailure|EnvelopeMissing\n")).unwrap();
    for line in [
        // Codex F5 counterexample: a field with visibility and a borrowed input.
        "variant=SscEventBinding|pub ambient: &'event str",
        "variant=SscProofFamily|Containment { accept: bool }",
        "variant=SscProofFamily|Containment(bool)",
        "variant=SscPermitVerdictV2|AlwaysPermit",
        "variant=SscContainmentVerdict|PermitAnyway",
        "variant=SscProofFamily|containment",
        "variant=SscProofFamily|Containment,",
    ] {
        assert!(parse_extensions(&format!("{header}{line}\n")).is_err(), "{line}");
    }
}

#[test]
fn wiring_binds_exact_adoption_functions_and_their_later_changes() {
    assert!(parse_wiring("schema=ssc_verdict_wiring_v1\nwiring=src/master_state_identity.rs\n").is_err());
    assert!(parse_wiring("schema=ssc_verdict_wiring_v1\nwiring=src/master_state_identity.rs::mint_block_receipt\n").is_err());
    parse_wiring("schema=ssc_verdict_wiring_v1\nwiring=src/master_state_identity.rs::commit\nwiring=src/master_state_identity/render_lineage.rs::commit\n").unwrap();
    // A change inside an already wired function needs the same parent admission.
    let p = Tree::new();
    p.append(WIRING_PATH, "wiring=src/master_state_identity.rs::commit");
    let c = Tree::new();
    c.append(WIRING_PATH, "wiring=src/master_state_identity.rs::commit");
    let anchor = "fn commit(self, domain: crate::ssc_evidence_proof::SscAdoptionDomain) -> CommittedTransformationEvent {";
    assert!(IDENTITY.contains(anchor));
    c.put("src/master_state_identity.rs", &IDENTITY.replacen(anchor, &format!("{anchor}\n        let _gate = 1;"), 1));
    c.manifest(CLASS, &[NONE], &["src/master_state_identity.rs"]);
    assert!(governed(&p, &c).unwrap_err().contains("verdict_wiring"));
    c.manifest(CLASS, &[NONE, "govern=verdict_wiring|src/master_state_identity.rs::commit|contract v4 A7 master adoption gate"], &["src/master_state_identity.rs"]);
    governed(&p, &c).unwrap();
    c.manifest("RefactorOnly", &[], &["src/master_state_identity.rs"]);
    assert!(validate_transition(&p.0, &c.0, "RefactorOnly", false).unwrap_err().contains("only under GovernedChange"));
}

// r1283: use the r1280-frozen identity fixture, never installed runtime/data files.
#[test]
fn allowed_range_removal_owns_safe_clef_deletion_but_visibility_neighbor_does_not() {
    let row = "row=ALLOWED_RANGE_REMOVAL_EXACT|Exact|Edit|attributes|-|remove|edit_script_locality|no|sealed removal owner deletes safe lower staff clef attributes only";
    parse_envelopes(&format!("{FIXTURE_ENVELOPES}{row}\n"), IDENTITY).unwrap();
    let neighbor = row.replace("ALLOWED_RANGE_REMOVAL_EXACT", "NOTE_VISIBILITY_EXACT");
    assert!(parse_envelopes(&format!("{FIXTURE_ENVELOPES}{neighbor}\n"), IDENTITY).is_err());
}

#[test]
fn final_phrase_layout_owns_documented_recue_carrier_but_plain_layout_neighbor_does_not() {
    let row = "row=FINAL_LAYOUT_PHRASE_PRESERVING|Preserving|Projection|direction-type,words|-|remove|owner_conserved_projection|no|documented final layout restart removes generated ottava words carrier only";
    parse_envelopes(&format!("{FIXTURE_ENVELOPES}{row}\n"), IDENTITY).unwrap();
    let neighbor = row.replace("FINAL_LAYOUT_PHRASE_PRESERVING", "PIANIST_LAYOUT_PRESERVING");
    assert!(parse_envelopes(&format!("{FIXTURE_ENVELOPES}{neighbor}\n"), IDENTITY).is_err());
    let purpose = PURPOSE_CLASSES.iter().find(|class| class.name == "ottava_recue").unwrap();
    assert_eq!(purpose.elements, &["direction-type", "words"]);
    assert_eq!(purpose.operations, &["remove"]);
    assert!(purpose.hosts.is_empty() && purpose.attrs.is_empty() && !purpose.content);
}

#[test]
fn functional_master_owns_inserted_tonal_key_container_but_pitch_swap_neighbor_does_not() {
    let row = "row=FUNCTIONAL_RESPELLING_EXACT|Exact|Edit|attributes,key,fifths|-|insert|edit_script_locality|yes|content: frozen final section master mirror inserts the bound tonal key container";
    parse_envelopes(&format!("{FIXTURE_ENVELOPES}{row}\n"), IDENTITY).unwrap();
    let neighbor = row.replace("FUNCTIONAL_RESPELLING_EXACT", "HARMONIC_PITCH_SWAP_EXACT");
    assert!(parse_envelopes(&format!("{FIXTURE_ENVELOPES}{neighbor}\n"), IDENTITY).is_err());
}

#[test]
fn render_spelling_owns_inserted_tonal_key_container_but_clef_measure_neighbor_does_not() {
    let row = "row=RENDER_ROW_66_BLOCK|Block|Block|attributes,key,fifths|-|insert|bound_span_locality|yes|content: frozen tonal render owner inserts the bound local key container";
    parse_envelopes(&format!("{FIXTURE_ENVELOPES}{row}\n"), IDENTITY).unwrap();
    let neighbor = row.replace("RENDER_ROW_66_BLOCK", "CLEF_MEASURE_BLOCK");
    assert!(parse_envelopes(&format!("{FIXTURE_ENVELOPES}{neighbor}\n"), IDENTITY).is_err());
    let purpose = PURPOSE_CLASSES.iter().find(|class| class.name == "tonal_key_container").unwrap();
    assert_eq!(purpose.elements, &["attributes"]);
    assert_eq!(purpose.operations, &["insert"]);
    assert!(purpose.hosts.is_empty() && purpose.attrs.is_empty() && !purpose.content);
}
