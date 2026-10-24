//! GovernedChange (contract v4, Teil B). A child may use only change kinds that this
//! parent-compiled module already knows. It grants no runtime, writer or music authority:
//! it decides solely whether a declared governance change and its use are admissible.
use super::{fail, item, mask_non_code, read};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub(crate) const CLASS: &str = "GovernedChange";
pub(crate) const MANIFEST_PATH: &str = "src/build_gate/take_change_class.txt";
pub(crate) const ENVELOPES_PATH: &str = "src/build_gate/ssc_containment_envelopes_v1.tsv";
pub(crate) const EXTENSIONS_PATH: &str = "src/build_gate/ssc_surface_extensions_v1.txt";
pub(crate) const WIRING_PATH: &str = "src/build_gate/ssc_verdict_wiring_v1.txt";
pub(crate) const LEDGER_PATH: &str = "src/build_gate/kodex_capability_retirements.txt";
pub(crate) const TRUST_ANCHOR_PATH: &str = "src/identity_gate_trust_anchor.rs";
const ENVELOPES_HEADER: &str = "schema=ssc_containment_envelopes_v1";
const EXTENSIONS_HEADER: &str = "schema=ssc_surface_extensions_v1";
const WIRING_HEADER: &str = "schema=ssc_verdict_wiring_v1";
/// The six change kinds installed by r1277. No other kind exists for a child.
pub(crate) const KINDS: [&str; 6] = [
    "envelope_extend",
    "envelope_narrow",
    "surface_extend",
    "proof_rule",
    "verdict_wiring",
    "retire",
];
/// Proof sources. They stay byte-frozen against the parent unless a GovernedChange
/// declares proof_rule for exactly that file; the content firewall applies always.
pub(crate) const PROOF_FILES: [&str; 4] = [
    "src/ssc_evidence_proof.rs",
    "src/ssc_shadow_permit.rs",
    "src/ssc_shadow_measurement.rs",
    "src/ssc_shadow_report.rs",
];
/// The only adoption boundaries of contract v4 A7 that may ever be wired: an exact
/// function in an exact file (r1278 F4), never a whole file.
pub(crate) const WIRING_ALLOWED: [(&str, &str); 3] = [
    ("src/master_state_identity.rs", "commit"),
    ("src/master_state_identity.rs", "commit_owner_receipts"),
    ("src/master_state_identity/render_lineage.rs", "commit"),
];
/// surface_extend (r1278 F5): only a bare unit variant, only on these enums.
pub(crate) const EXTENSIBLE_ENUMS: [&str; 3] =
    ["SscProofFamily", "SscEvidenceIntegrityFailure", "SscInvariantFailureKind"];
/// retire never touches the protections of contract v4 B3 or this module.
const RETIRE_PROTECTED: [&str; 8] = [
    "src/take_change_control.rs",
    "src/build_gate/semantic_capability_diff.rs",
    "src/build_gate/ssc_governed_change.rs",
    "src/identity_gate_trust_anchor.rs",
    "src/kodex_build_orchestrator.rs",
    "src/bin/etuschlopf_take_bootstrap.rs",
    "src/build_gate/multi_revision_evasion.rs",
    "src/build_gate/kodex_capability_retirements.txt",
];
const WILDCARDS: [&str; 11] = [
    "*", "any", "all", "every", "everything", "alle", "beliebig", "%", "?", "..", ".*",
];
const EXEMPTION_WORDS: [&str; 8] = [
    "exempt", "optional", "harmless", "administrative", "nonmandatory", "waive", "bypass", "skip",
];
const CONTENT_NAMES: [&str; 13] = [
    "pitch", "step", "alter", "octave", "duration", "unpitched", "divisions", "time", "beats",
    "beat-type", "chord", "tie", "rest",
];
/// The single bound checker each envelope kind may name (r1278 F3).
const INVARIANTS: [(&str, &str); 5] = [
    ("Projection", "owner_conserved_projection"),
    ("Insertion", "single_contiguous_insertion"),
    ("Edit", "edit_script_locality"),
    ("Lane", "lane_fields_only"),
    ("Block", "bound_span_locality"),
];
/// Lane envelopes exist only for the three frozen lane owners (F5, F7, F9).
const LANE_DESCRIPTORS: [&str; 3] =
    ["STAFF_VOICE_MOVE_EXACT", "RENDER_ROW_44_EXACT", "CURSOR_NEUTRAL_RENDER_EXACT"];

/// A frozen purpose class (r1278 F3): the upper bound an envelope row may reach.
/// `elements` may change as a whole; `hosts` may change only in the listed `attrs`
/// or in child elements that are themselves in `elements`. `*` as host means any
/// element hosts these attrs; it is never accepted inside a child's table row.
pub(crate) struct PurposeClass {
    pub(crate) name: &'static str,
    elements: &'static [&'static str],
    hosts: &'static [&'static str],
    attrs: &'static [&'static str],
    operations: &'static [&'static str],
    content: bool,
}
const ALL_OPS: &[&str] = &["change", "insert", "remove"];
const POSITION_ATTRS: &[&str] = &[
    "default-x", "default-y", "relative-x", "relative-y", "placement", "width", "new-system",
    "new-page", "blank-page", "page-number",
];
pub(crate) const PURPOSE_CLASSES: &[PurposeClass] = &[
    // r1283: only the documented generated Words carrier at an E007 layout restart.
    PurposeClass { name: "ottava_recue", elements: &["direction-type", "words"], hosts: &[], attrs: &[], operations: &["remove"], content: false },
    // r1283: the key-only attributes wrapper inserted by the two tonal-frame owners.
    // Pitch/key contents remain governed by their existing pitch_spelling class.
    PurposeClass { name: "tonal_key_container", elements: &["attributes"], hosts: &[], attrs: &[], operations: &["insert"], content: false },
    PurposeClass { name: "beam", elements: &["beam", "stem"], hosts: &["note"], attrs: &[], operations: ALL_OPS, content: false },
    PurposeClass { name: "clef", elements: &["attributes", "clef", "sign", "line", "clef-octave-change", "staff-details", "staff-lines"], hosts: &["attributes"], attrs: &["number", "print-object", "after-barline"], operations: ALL_OPS, content: false },
    PurposeClass { name: "layout", elements: &["print", "system-layout", "page-layout", "staff-layout", "system-margins", "system-distance", "top-system-distance", "staff-distance", "left-margin", "right-margin", "top-margin", "bottom-margin", "page-margins", "page-height", "page-width", "measure-layout", "measure-distance", "defaults", "scaling", "millimeters", "tenths", "appearance", "line-width", "note-size", "distance", "glyph", "other-appearance", "staff-size", "word-font", "music-font", "lyric-font", "system-dividers", "left-divider", "right-divider"], hosts: &["*"], attrs: POSITION_ATTRS, operations: ALL_OPS, content: false },
    PurposeClass { name: "direction", elements: &["direction", "direction-type", "words", "dynamics", "metronome", "beat-unit", "beat-unit-dot", "per-minute", "sound", "offset", "wedge", "octave-shift", "dashes", "bracket", "pedal", "other-direction"], hosts: &["direction"], attrs: &["placement", "directive"], operations: ALL_OPS, content: false },
    PurposeClass { name: "visibility", elements: &[], hosts: &["*"], attrs: &["print-object", "print-spacing", "print-leger", "print-dot", "print-lyric"], operations: ALL_OPS, content: false },
    PurposeClass { name: "lane", elements: &["voice", "staff"], hosts: &["note"], attrs: &["print-object", "print-spacing"], operations: ALL_OPS, content: false },
    PurposeClass { name: "barline", elements: &["barline", "bar-style", "repeat", "ending"], hosts: &["barline"], attrs: &["location"], operations: &["change", "insert"], content: false },
    PurposeClass { name: "notation_mark", elements: &["notations", "slur", "tied", "articulations", "accent", "tenuto", "staccato", "staccatissimo", "strong-accent", "fermata", "ornaments", "technical", "fingering", "arpeggiate"], hosts: &["note"], attrs: &[], operations: ALL_OPS, content: false },
    PurposeClass { name: "rest_rhythm", elements: &["note", "rest", "forward", "backup", "duration", "type", "dot", "voice", "staff", "chord", "time-modification", "display-step", "display-octave", "beam", "notations", "tie", "tied", "stem"], hosts: &["note", "rest"], attrs: &["print-object", "print-spacing", "measure"], operations: ALL_OPS, content: true },
    PurposeClass { name: "pitch_spelling", elements: &["pitch", "step", "alter", "octave", "accidental", "key", "fifths", "mode", "cancel"], hosts: &["note", "attributes"], attrs: &[], operations: ALL_OPS, content: true },
    PurposeClass { name: "sounding_content", elements: &["note", "rest", "forward", "backup", "duration", "type", "dot", "voice", "staff", "chord", "time-modification", "pitch", "step", "alter", "octave", "accidental", "beam", "notations", "tie", "tied", "stem", "display-step", "display-octave"], hosts: &["note"], attrs: &["print-object", "print-spacing"], operations: ALL_OPS, content: true },
];
const DOCUMENT: &[&str] = &["beam", "clef", "layout", "direction", "visibility", "lane", "barline", "notation_mark", "rest_rhythm", "pitch_spelling", "sounding_content"];
/// Frozen owner purpose per descriptor (r1278 F3): receipt kind and purpose classes,
/// derived from the frozen owner contract (population/exclusion notes, owner signature
/// and code), never from the corpus distribution. Genesis owners have no envelope.
pub(crate) const DESCRIPTOR_PURPOSES: &[(&str, &str, &[&str])] = &[
    ("CURSOR_NEUTRAL_RENDER_EXACT", "Exact", &["lane"]),
    ("SAME_LANE_RAW_SPLICE_EXACT", "Exact", &["rest_rhythm"]),
    ("MISSING_REST_RENDER_EXACT", "Exact", &["rest_rhythm", "visibility"]),
    ("DIRECTION_RELOCATION_EXACT", "Exact", &["direction", "layout"]),
    ("VISIBLE_RHYTHM_BLOCK", "Block", DOCUMENT),
    ("BEAT_SCOPED_BEAMING_BLOCK", "Block", &["beam", "rest_rhythm", "visibility"]),
    ("CLEF_MEASURE_BLOCK", "Block", &["clef"]),
    ("OCTAVE_MEASURE_BLOCK", "Block", &["direction", "pitch_spelling"]),
    ("CURSOR_LANE_END_CLOSURE_BLOCK", "Block", &["rest_rhythm", "lane", "visibility"]),
    ("SAFE_TRANSFER_EXACT", "Exact", &["lane", "rest_rhythm"]),
    ("STAFF_VOICE_MOVE_EXACT", "Exact", &["lane"]),
    ("RESTIFY_PATCH_EXACT", "Exact", &["rest_rhythm", "sounding_content"]),
    ("VOICE_OVERFULL_CROP_EXACT", "Exact", &["sounding_content"]),
    ("RESTIFICATION_BATCH_EXACT", "Exact", &["rest_rhythm", "sounding_content"]),
    ("STAFF_VOICE_SCALAR_BATCH_EXACT", "Exact", &["lane"]),
    ("EDITORIAL_VOICE_COMPLETE_REWRITE_EXACT", "Exact", &["sounding_content"]),
    ("NOTE_VISIBILITY_EXACT", "Exact", &["visibility"]),
    ("ALLOWED_RANGE_REMOVAL_EXACT", "Exact", &["notation_mark", "rest_rhythm", "clef"]),
    ("HARMONIC_PITCH_SWAP_EXACT", "Exact", &["pitch_spelling", "sounding_content"]),
    ("HIDDEN_ATTRIBUTE_REMOVAL_EXACT", "Exact", &["visibility"]),
    ("CHORD_GESTURE_DISTRIBUTION_EXACT", "Exact", &["lane", "rest_rhythm"]),
    ("VISIBILITY_STAFF_DISTRIBUTION_EXACT", "Exact", &["lane", "visibility"]),
    ("MASTER_EXPORT_FIFTHS_EXACT", "Exact", &["pitch_spelling"]),
    ("FUNCTIONAL_RESPELLING_EXACT", "Exact", &["pitch_spelling", "tonal_key_container"]),
    ("FINAL_CHORD_ENCODING_EXACT", "Exact", &["sounding_content", "lane"]),
    ("NOTATION_POLISH_PRESERVING", "Preserving", &["beam", "clef", "layout", "visibility", "notation_mark", "direction", "barline"]),
    ("NOTATION_CLEF_ISLAND_PRESERVING", "Preserving", &["clef", "layout"]),
    ("PIANIST_LAYOUT_PRESERVING", "Preserving", &["layout"]),
    ("PARTIAL_BEAT_BEAM_PRESERVING", "Preserving", &["beam"]),
    ("PARTIAL_BEAT_LANE_TRANSFER_PRESERVING", "Preserving", &["rest_rhythm", "lane"]),
    ("GOULD_METRIC_BEAM_PRESERVING", "Preserving", &["beam"]),
    ("SCOREWIDE_METRIC_BEAM_PRESERVING", "Preserving", &["beam"]),
    ("STRUCTURAL_BEAM_CLOSURE_PRESERVING", "Preserving", &["beam"]),
    ("COMMITTED_CROSS_STAFF_BEAM_PRESERVING", "Preserving", &["beam"]),
    ("STRUCTURAL_REST_BRIDGE_VARIANT_A_PRESERVING", "Preserving", &["beam"]),
    ("OCTAVE_WORD_LABEL_PRESERVING", "Preserving", &["direction"]),
    ("VISUAL_LAYOUT_DEFAULTS_PRESERVING", "Preserving", &["layout"]),
    ("LOCAL_STAFF_DISTANCE_PRESERVING", "Preserving", &["layout"]),
    ("RUSTIG_CLEF_COURSE_PRESERVING", "Preserving", &["clef", "layout"]),
    ("FINAL_COMPLETE_BEAT_PRESERVING", "Preserving", &["beam"]),
    ("LEDGER_BOUNDED_LOWER_STAFF_CLEF_PRESERVING", "Preserving", &["clef"]),
    ("FINAL_LAYOUT_PHRASE_PRESERVING", "Preserving", &["notation_mark", "layout", "ottava_recue"]),
    ("FINAL_BARLINE_ADDITIVE", "Additive", &["barline"]),
    ("DIRECTION_INSERTION_ADDITIVE", "Additive", &["direction"]),
    ("E032_FRAGMENT_ADDITIVE", "Additive", &["rest_rhythm"]),
    ("RENDER_ROW_7_BLOCK", "Block", &["sounding_content"]),
    ("RENDER_ROW_17_BLOCK", "Block", &["rest_rhythm"]),
    ("RENDER_ROW_19_BLOCK", "Block", &["visibility"]),
    ("RENDER_ROW_20_BLOCK", "Block", &["visibility"]),
    ("RENDER_ROW_21_BLOCK", "Block", &["rest_rhythm"]),
    ("RENDER_ROW_22_EXACT", "Exact", &["sounding_content", "visibility"]),
    ("RENDER_ROW_24_BLOCK", "Block", &["rest_rhythm", "lane", "visibility"]),
    ("RENDER_ROW_30_EXACT", "Exact", &["rest_rhythm", "visibility", "layout"]),
    ("RENDER_ROW_31_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_32_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_33_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_34_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_35_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_36_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_37_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_38_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_41_EXACT", "Exact", &["rest_rhythm", "lane"]),
    ("RENDER_ROW_42_EXACT", "Exact", &["sounding_content", "visibility"]),
    ("RENDER_ROW_43_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_44_EXACT", "Exact", &["visibility", "lane"]),
    ("RENDER_ROW_45_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_46_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_47_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_48_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_51_EXACT", "Exact", &["rest_rhythm", "visibility", "lane"]),
    ("RENDER_ROW_54_BLOCK", "Block", &["rest_rhythm", "visibility"]),
    ("RENDER_ROW_55_BLOCK", "Block", &["rest_rhythm", "visibility"]),
    ("RENDER_ROW_57_BLOCK", "Block", DOCUMENT),
    ("RENDER_ROW_58_EXACT", "Exact", DOCUMENT),
    ("RENDER_ROW_61_BLOCK", "Block", &["clef", "visibility"]),
    ("RENDER_ROW_64_EXACT", "Exact", &["sounding_content", "lane"]),
    ("RENDER_ROW_66_BLOCK", "Block", &["pitch_spelling", "tonal_key_container"]),
];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct GovernLine {
    pub(crate) kind: String,
    pub(crate) target: String,
    pub(crate) reason: String,
}

/// Parses one `govern=` value. Shape only; admission is decided by validate_transition.
pub(crate) fn parse_govern_value(value: &str) -> Result<GovernLine, String> {
    let fields = value.split('|').map(str::trim).collect::<Vec<_>>();
    if fields.len() != 3 || fields.iter().any(|field| field.is_empty()) {
        return Err(fail("govern expects <kind>|<target>|<stable reason>"));
    }
    if !KINDS.contains(&fields[0]) {
        return Err(fail(format!(
            "unknown govern kind {:?}; the parent admits only {KINDS:?}",
            fields[0]
        )));
    }
    if fields[2].chars().count() < 12 {
        return Err(fail("govern reason must state a stable reason (>= 12 characters)"));
    }
    Ok(GovernLine {
        kind: fields[0].to_owned(),
        target: fields[1].to_owned(),
        reason: fields[2].to_owned(),
    })
}

/// Parses one `music_effect=` value: `none` or `declared|<reason>`.
pub(crate) fn parse_music_effect(value: &str) -> Result<String, String> {
    let value = value.trim();
    if value == "none" {
        return Ok(value.to_owned());
    }
    match value.split_once('|') {
        Some(("declared", reason)) if reason.trim().chars().count() >= 12 => {
            Ok(format!("declared|{}", reason.trim()))
        }
        _ => Err(fail(
            "music_effect expects none or declared|<reason>; declared needs explicit user release",
        )),
    }
}

fn manifest_lines(root: &Path) -> Result<(Vec<GovernLine>, Option<String>), String> {
    let Some(text) = read_optional(root, MANIFEST_PATH)? else {
        return Ok((Vec::new(), None));
    };
    let mut govern = Vec::new();
    let mut music = None;
    for line in text.lines().map(str::trim) {
        if let Some(value) = line.strip_prefix("govern=") {
            govern.push(parse_govern_value(value)?);
        } else if let Some(value) = line.strip_prefix("music_effect=")
            && music.replace(parse_music_effect(value)?).is_some()
        {
            return Err(fail("music_effect must occur exactly once"));
        }
    }
    Ok((govern, music))
}

/// Path policy of GovernedChange for Kodex paths: governed data files only, plus the
/// exact files of a declared retire. Validator code is never a GovernedChange path.
pub(crate) fn kodex_path_admissible(path: &str, govern: &[GovernLine]) -> bool {
    if [ENVELOPES_PATH, EXTENSIONS_PATH, WIRING_PATH].contains(&path) {
        return true;
    }
    let retire = govern.iter().filter(|line| line.kind == "retire").collect::<Vec<_>>();
    if retire.is_empty() {
        return false;
    }
    path == LEDGER_PATH
        || path == TRUST_ANCHOR_PATH
        || retire
            .iter()
            .any(|line| line.target.rsplit_once("::").is_some_and(|(p, _)| p == path))
}

fn read_optional(root: &Path, path: &str) -> Result<Option<String>, String> {
    if root.join(path).is_file() {
        read(root, path).map(Some)
    } else {
        Ok(None)
    }
}

fn is_ident(value: &str, upper: bool) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    let first_ok = if upper { first.is_ascii_uppercase() } else { first.is_ascii_lowercase() };
    first_ok && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || (!upper && c == '-'))
}

fn wildcard(value: &str) -> bool {
    let lower = value.trim().to_ascii_lowercase();
    lower.is_empty() || WILDCARDS.iter().any(|w| lower == *w || lower.contains('*'))
}

// ---------------------------------------------------------------- envelope table

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EnvelopeRow {
    pub(crate) descriptor: String,
    pub(crate) receipt: String,
    pub(crate) envelope: String,
    pub(crate) elements: BTreeSet<String>,
    pub(crate) fields: BTreeSet<String>,
    pub(crate) operations: BTreeSet<String>,
    pub(crate) invariant: String,
    pub(crate) content_changing: bool,
    pub(crate) purpose: String,
}

fn list(value: &str, label: &str) -> Result<BTreeSet<String>, String> {
    let items = value.split(',').map(str::trim).collect::<Vec<_>>();
    let mut out = BTreeSet::new();
    for entry in &items {
        if wildcard(entry) {
            return Err(fail(format!("envelope {label} wildcard or empty entry forbidden")));
        }
        if !out.insert((*entry).to_owned()) {
            return Err(fail(format!("envelope {label} duplicate entry {entry}")));
        }
    }
    Ok(out)
}

fn descriptor_receipt(identity_source: &str, descriptor: &str) -> Result<&'static str, String> {
    for (kind, receipt) in [
        ("PreservingDescriptor", "Preserving"),
        ("AdditiveDescriptor", "Additive"),
        ("TransformationDescriptor", ""),
    ] {
        let Ok(body) = item(identity_source, "const", descriptor) else {
            continue;
        };
        let head = body.split('=').next().unwrap_or("");
        if !head.contains(kind) {
            continue;
        }
        if !receipt.is_empty() {
            return Ok(receipt);
        }
        let compact = mask_non_code(body).split_whitespace().collect::<String>();
        return if compact.contains("identity:IdentityContract::Exact") {
            Ok("Exact")
        } else if compact.contains("identity:IdentityContract::Block") {
            Ok("Block")
        } else {
            Err(fail(format!("descriptor {descriptor} has no adoptable receipt identity")))
        };
    }
    Err(fail(format!("envelope descriptor {descriptor} is not a frozen owner descriptor")))
}

pub(crate) fn parse_envelopes(text: &str, identity_source: &str) -> Result<Vec<EnvelopeRow>, String> {
    let mut lines = text.lines();
    if lines.next() != Some(ENVELOPES_HEADER) {
        return Err(fail("envelope table header changed"));
    }
    let mut seen = BTreeSet::new();
    let mut rows = Vec::new();
    for line in lines {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let value = line
            .strip_prefix("row=")
            .ok_or_else(|| fail("envelope table accepts only row= lines"))?;
        let f = value.split('|').map(str::trim).collect::<Vec<_>>();
        if f.len() != 9 || f.iter().any(|field| field.is_empty()) {
            return Err(fail("envelope row expects 9 non-empty fields"));
        }
        if !f[0].chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') {
            return Err(fail("envelope descriptor must be a frozen descriptor constant"));
        }
        if !seen.insert(f[0].to_owned()) {
            return Err(fail(format!("envelope descriptor {} has more than one entry", f[0])));
        }
        // The parent-compiled purpose table decides; the child's identity file must
        // only agree with it, so both cannot be redefined together (r1278 F3).
        let (_, frozen_receipt, classes) = DESCRIPTOR_PURPOSES
            .iter()
            .find(|(name, _, _)| *name == f[0])
            .ok_or_else(|| fail(format!("envelope descriptor {} has no frozen owner purpose", f[0])))?;
        let receipt = descriptor_receipt(identity_source, f[0])?;
        if f[1] != receipt || f[1] != *frozen_receipt {
            return Err(fail(format!("envelope {} receipt {} != frozen {frozen_receipt}", f[0], f[1])));
        }
        let compatible = matches!(
            (f[1], f[2]),
            ("Preserving", "Projection")
                | ("Additive", "Insertion")
                | ("Exact", "Edit")
                | ("Exact", "Lane")
                | ("Block", "Block")
        );
        if !compatible || (f[2] == "Lane" && !LANE_DESCRIPTORS.contains(&f[0])) {
            return Err(fail(format!("envelope {} kind {} does not fit receipt {}", f[0], f[2], f[1])));
        }
        let elements = list(f[3], "elements")?;
        if elements.iter().any(|e| !is_ident(e, false)) {
            return Err(fail("envelope element names must be concrete MusicXML element names"));
        }
        let fields = if f[4] == "-" { BTreeSet::new() } else { list(f[4], "fields")? };
        for field in &fields {
            let (element, name) = field
                .split_once('.')
                .or_else(|| field.split_once('@'))
                .ok_or_else(|| fail(format!("envelope field {field} must be element.child or element@attr")))?;
            if !elements.contains(element) || !is_ident(name, false) {
                return Err(fail(format!("envelope field {field} outside its declared elements")));
            }
        }
        let operations = list(f[5], "operations")?;
        if operations.iter().any(|op| !matches!(op.as_str(), "change" | "insert" | "remove")) {
            return Err(fail("envelope operations are change, insert or remove"));
        }
        if !INVARIANTS.contains(&(f[2], f[6])) {
            return Err(fail(format!(
                "envelope {} invariant {} is not the bound checker of kind {}",
                f[0], f[6], f[2]
            )));
        }
        let content_changing = match f[7] {
            "yes" => true,
            "no" => false,
            _ => return Err(fail("envelope content_changing is yes or no")),
        };
        let touches_content = elements.iter().any(|e| CONTENT_NAMES.contains(&e.as_str()))
            || fields.iter().any(|field| {
                field
                    .rsplit(['.', '@'])
                    .next()
                    .is_some_and(|name| CONTENT_NAMES.contains(&name))
            })
            || (whole_element(&elements, &fields, "note")
                && (operations.contains("insert") || operations.contains("remove")));
        if touches_content && !content_changing {
            return Err(fail(format!(
                "envelope {} changes pitch, duration or note count without content_changing=yes",
                f[0]
            )));
        }
        if f[8].chars().count() < 20 || (content_changing && !f[8].contains("content:")) {
            return Err(fail(format!(
                "envelope {} purpose must cite the owner purpose (content_changing needs content:<why>)",
                f[0]
            )));
        }
        within_purpose(f[0], classes, &elements, &fields, &operations, content_changing)?;
        rows.push(EnvelopeRow {
            descriptor: f[0].to_owned(),
            receipt: f[1].to_owned(),
            envelope: f[2].to_owned(),
            elements,
            fields,
            operations,
            invariant: f[6].to_owned(),
            content_changing,
            purpose: f[8].to_owned(),
        });
    }
    Ok(rows)
}

/// An element listed without any field of its own changes as a whole.
fn whole_element(elements: &BTreeSet<String>, fields: &BTreeSet<String>, element: &str) -> bool {
    elements.contains(element)
        && !fields.iter().any(|field| {
            field
                .split_once(['.', '@'])
                .is_some_and(|(host, _)| host == element)
        })
}

/// r1278 F3: a row must lie inside the frozen purpose classes of its descriptor.
fn within_purpose(
    descriptor: &str,
    classes: &[&str],
    elements: &BTreeSet<String>,
    fields: &BTreeSet<String>,
    operations: &BTreeSet<String>,
    content_changing: bool,
) -> Result<(), String> {
    let bound = PURPOSE_CLASSES
        .iter()
        .filter(|class| classes.contains(&class.name))
        .collect::<Vec<_>>();
    if bound.len() != classes.len() {
        return Err(fail(format!("descriptor {descriptor} names an unknown purpose class")));
    }
    let outside = |what: String| {
        fail(format!(
            "envelope {descriptor}: {what} lies outside its frozen owner purpose {classes:?}"
        ))
    };
    let whole = |name: &str| bound.iter().any(|class| class.elements.contains(&name));
    let host = |name: &str| {
        bound
            .iter()
            .any(|class| class.hosts.contains(&name) || class.hosts.contains(&"*"))
    };
    for element in elements {
        if whole_element(elements, fields, element) {
            if !whole(element) {
                return Err(outside(format!("element {element}")));
            }
        } else if !whole(element) && !host(element) {
            return Err(outside(format!("host {element}")));
        }
    }
    for field in fields {
        let (owner, separator, name) = match field.split_once('.') {
            Some((owner, name)) => (owner, '.', name),
            None => {
                let (owner, name) = field.split_once('@').unwrap_or((field, ""));
                (owner, '@', name)
            }
        };
        let covered = whole(owner)
            || bound.iter().any(|class| {
                (class.hosts.contains(&owner) || class.hosts.contains(&"*"))
                    && if separator == '@' {
                        class.attrs.contains(&name)
                    } else {
                        class.elements.contains(&name)
                    }
            });
        if !covered {
            return Err(outside(format!("field {field}")));
        }
    }
    for operation in operations {
        if !bound.iter().any(|class| class.operations.contains(&operation.as_str())) {
            return Err(outside(format!("operation {operation}")));
        }
    }
    if content_changing && !bound.iter().any(|class| class.content) {
        return Err(outside("content change".into()));
    }
    Ok(())
}

fn narrows(old: &EnvelopeRow, new: &EnvelopeRow) -> bool {
    old.receipt == new.receipt
        && old.envelope == new.envelope
        && old.invariant == new.invariant
        && new.elements.is_subset(&old.elements)
        && new.fields.is_subset(&old.fields)
        && new.operations.is_subset(&old.operations)
        && (old.content_changing || !new.content_changing)
}

// ------------------------------------------------------------- surface extensions

pub(crate) fn parse_extensions(text: &str) -> Result<Vec<(String, String)>, String> {
    let mut lines = text.lines();
    if lines.next() != Some(EXTENSIONS_HEADER) {
        return Err(fail("surface extension header changed"));
    }
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for line in lines {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (owner, source) = line
            .strip_prefix("variant=")
            .and_then(|v| v.split_once('|'))
            .ok_or_else(|| fail("surface extension accepts only variant=<Item>|<source>"))?;
        let name = extension_name(source)?;
        // r1278 F5: a bare unit variant on a parent-listed enum; never a field, type,
        // visibility or payload.
        if !EXTENSIBLE_ENUMS.contains(&owner.trim()) || source.trim() != name || !is_ident(&name, true) {
            return Err(fail(format!(
                "surface extension {owner}|{source} is not a bare unit variant of {EXTENSIBLE_ENUMS:?}"
            )));
        }
        if !seen.insert((owner.trim().to_owned(), name.clone())) {
            return Err(fail("surface extension variant duplicated"));
        }
        let lower = name.to_ascii_lowercase();
        if EXEMPTION_WORDS.iter().any(|w| lower.contains(w)) {
            return Err(fail(format!("surface extension {name} would introduce an exemption class")));
        }
        out.push((owner.trim().to_owned(), source.trim().to_owned()));
    }
    Ok(out)
}

pub(crate) fn extension_name(source: &str) -> Result<String, String> {
    let name = source
        .trim()
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .next()
        .unwrap_or("");
    if name.is_empty() || !name.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
        return Err(fail("surface extension needs a named variant or field"));
    }
    Ok(name.to_owned())
}

// ---------------------------------------------------------------------- wiring

pub(crate) fn parse_wiring(text: &str) -> Result<BTreeSet<String>, String> {
    let mut lines = text.lines();
    if lines.next() != Some(WIRING_HEADER) {
        return Err(fail("verdict wiring header changed"));
    }
    let mut out = BTreeSet::new();
    for line in lines {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let target = line
            .strip_prefix("wiring=")
            .ok_or_else(|| fail("verdict wiring accepts only wiring=<path>::<function>"))?;
        let bound = target
            .rsplit_once("::")
            .is_some_and(|pair| WIRING_ALLOWED.contains(&pair));
        if !bound || !out.insert(target.to_owned()) {
            return Err(fail(format!(
                "verdict wiring {target} is not an A7 adoption function or duplicated"
            )));
        }
    }
    Ok(out)
}

/// Installed wiring of a tree (empty before installation).
pub(crate) fn wiring(root: &Path) -> Result<BTreeSet<String>, String> {
    read_optional(root, WIRING_PATH)?.map_or_else(|| Ok(BTreeSet::new()), |t| parse_wiring(&t))
}

/// Installed surface extensions of a tree (empty before installation).
pub(crate) fn extensions(root: &Path) -> Result<Vec<(String, String)>, String> {
    read_optional(root, EXTENSIONS_PATH)?.map_or_else(|| Ok(Vec::new()), |t| parse_extensions(&t))
}

/// Tree rules: the three governed data files exist and are well-formed.
pub(crate) fn validate_tree(root: &Path) -> Result<(), String> {
    let identity = read(root, "src/master_state_identity.rs")?;
    parse_envelopes(&read(root, ENVELOPES_PATH)?, &identity)?;
    parse_extensions(&read(root, EXTENSIONS_PATH)?)?;
    parse_wiring(&read(root, WIRING_PATH)?)?;
    Ok(())
}

// ------------------------------------------------------------------ transition

struct Admission {
    govern: Vec<GovernLine>,
    consumed: BTreeSet<usize>,
}

impl Admission {
    fn take(&mut self, kind: &str, target: &str) -> Result<(), String> {
        let index = self
            .govern
            .iter()
            .position(|line| line.kind == kind && line.target == target)
            .ok_or_else(|| {
                fail(format!(
                    "GovernedChange: change requires govern={kind}|{target}|<reason> admitted by the parent"
                ))
            })?;
        self.consumed.insert(index);
        Ok(())
    }
}

fn governed_file_transition(
    parent: &Path,
    child: &Path,
    path: &str,
    governed: bool,
    named_revision: bool,
) -> Result<Option<(String, String)>, String> {
    if path == ENVELOPES_PATH && super::cursor_temporal::verified_first_transition(parent,child)? {
        return Ok(None);
    }
    let before = read_optional(parent, path)?;
    let after = read_optional(child, path)?;
    match (before, after) {
        (_, None) => Err(fail(format!("governed data file cannot disappear: {path}"))),
        (None, Some(_)) if named_revision => Ok(None),
        (None, Some(_)) => Err(fail(format!(
            "governed data file {path} is installed only by a named K2 governance revision"
        ))),
        (Some(before), Some(after)) if before == after => Ok(None),
        (Some(before), Some(after)) => {
            if governed {
                Ok(Some((before, after)))
            } else if named_revision {
                Ok(None)
            } else {
                Err(fail(format!("governed data file {path} changes only under GovernedChange")))
            }
        }
    }
}

fn pin_normalized(source: &str) -> Vec<String> {
    source
        .lines()
        .map(|line| match line.split_once("== 0x") {
            Some((head, tail)) => {
                let rest = tail.trim_start_matches(|c: char| c.is_ascii_hexdigit() || c == '_');
                format!("{head}== 0x<pin>{rest}")
            }
            None => line.to_owned(),
        })
        .collect()
}

/// Pure deletion: the child file is an ordered line subsequence of the parent file.
fn pure_deletion(before: &str, after: &str) -> bool {
    let mut old = before.lines();
    after.lines().all(|line| old.by_ref().any(|candidate| candidate == line))
}

fn fn_count(source: &str, name: &str) -> usize {
    let masked = mask_non_code(source);
    let needle = format!("fn {name}");
    masked
        .match_indices(&needle)
        .filter(|(start, _)| {
            let end = start + needle.len();
            masked[end..]
                .chars()
                .next()
                .is_none_or(|c| !(c.is_ascii_alphanumeric() || c == '_'))
        })
        .count()
}

/// Admission of a transition by the parent-compiled rules.
pub(crate) fn validate_transition(
    parent: &Path,
    child: &Path,
    class: &str,
    named_revision: bool,
) -> Result<(), String> {
    let (govern, music) = manifest_lines(child)?;
    let governed = class == CLASS;
    if !governed && (!govern.is_empty() || music.is_some()) {
        return Err(fail("govern= and music_effect= are valid only for GovernedChange"));
    }
    if governed && music.is_none() {
        return Err(fail("GovernedChange requires music_effect=none or declared|<reason>"));
    }
    let unique = govern.iter().map(|l| (&l.kind, &l.target)).collect::<BTreeSet<_>>();
    if unique.len() != govern.len() {
        return Err(fail("duplicate govern line"));
    }
    let mut admission = Admission { govern, consumed: BTreeSet::new() };

    // proof_rule: proof files are frozen against the parent unless declared.
    for path in PROOF_FILES {
        if read_optional(parent, path)? != read_optional(child, path)? {
            if !governed {
                return Err(fail(format!(
                    "proof source {path} changes only under GovernedChange with govern=proof_rule"
                )));
            }
            admission.take("proof_rule", path)?;
        }
    }

    // envelope_extend / envelope_narrow.
    if let Some((before, after)) =
        governed_file_transition(parent, child, ENVELOPES_PATH, governed, named_revision)?
    {
        let identity = read(child, "src/master_state_identity.rs")?;
        let parent_identity = read(parent, "src/master_state_identity.rs")?;
        let old = parse_envelopes(&before, &parent_identity)?
            .into_iter()
            .map(|r| (r.descriptor.clone(), r))
            .collect::<BTreeMap<_, _>>();
        let new = parse_envelopes(&after, &identity)?
            .into_iter()
            .map(|r| (r.descriptor.clone(), r))
            .collect::<BTreeMap<_, _>>();
        for descriptor in old.keys().chain(new.keys()).collect::<BTreeSet<_>>() {
            match (old.get(descriptor), new.get(descriptor)) {
                (Some(a), Some(b)) if a == b => {}
                (Some(a), Some(b)) if narrows(a, b) => admission.take("envelope_narrow", descriptor)?,
                (Some(_), None) => admission.take("envelope_narrow", descriptor)?,
                _ => admission.take("envelope_extend", descriptor)?,
            }
        }
    }

    // surface_extend: append-only.
    if let Some((before, after)) =
        governed_file_transition(parent, child, EXTENSIONS_PATH, governed, named_revision)?
    {
        let old = parse_extensions(&before)?;
        let new = parse_extensions(&after)?;
        if new.len() < old.len() || new[..old.len()] != old[..] {
            return Err(fail("surface extensions are append-only"));
        }
        for (owner, source) in &new[old.len()..] {
            admission.take("surface_extend", &format!("{owner}::{}", extension_name(source)?))?;
        }
    }

    // verdict_wiring: additions need a declaration; removal narrows. A change inside an
    // already wired adoption function needs the same parent admission (r1278 F4).
    let wiring_change = governed_file_transition(parent, child, WIRING_PATH, governed, named_revision)?;
    let old_wiring = read_optional(parent, WIRING_PATH)?
        .map_or_else(|| Ok(BTreeSet::new()), |text| parse_wiring(&text))?;
    if let Some((_, after)) = &wiring_change {
        for target in parse_wiring(after)?.difference(&old_wiring) {
            admission.take("verdict_wiring", target)?;
        }
    }
    for target in &old_wiring {
        let Some((path, function)) = target.rsplit_once("::") else { continue };
        let before = read_optional(parent, path)?.map(|text| item(&text, "fn", function).map(str::to_owned)).transpose()?;
        let after = read_optional(child, path)?.map(|text| item(&text, "fn", function).map(str::to_owned)).transpose()?;
        if before != after {
            if !governed {
                return Err(fail(format!("wired adoption function {target} changes only under GovernedChange")));
            }
            admission.take("verdict_wiring", target)?;
        }
    }

    // retire: pure deletion of a Kodex check, never together with a runtime change.
    let retires = admission
        .govern
        .iter()
        .enumerate()
        .filter(|(_, line)| line.kind == "retire")
        .map(|(index, line)| (index, line.target.clone()))
        .collect::<Vec<_>>();
    if !retires.is_empty() {
        validate_retire(parent, child, &retires)?;
        admission.consumed.extend(retires.iter().map(|(index, _)| *index));
    }

    if let Some((_, line)) = admission
        .govern
        .iter()
        .enumerate()
        .find(|(index, _)| !admission.consumed.contains(index))
    {
        return Err(fail(format!(
            "govern={}|{} has no matching change; dangling admissions are rejected",
            line.kind, line.target
        )));
    }
    Ok(())
}

fn validate_retire(parent: &Path, child: &Path, retires: &[(usize, String)]) -> Result<(), String> {
    let manifest = read(child, MANIFEST_PATH)?;
    for row in manifest.lines().filter_map(|line| line.strip_prefix("change=")) {
        let fields = row.split('|').map(str::trim).collect::<Vec<_>>();
        let Some(path) = fields.get(1) else { continue };
        let kodex = path.starts_with("src/build_gate/")
            || path.starts_with("docs/")
            || matches!(*path, "README.md" | "ZIP_KIND.txt")
            || *path == TRUST_ANCHOR_PATH
            || retires.iter().any(|(_, t)| t.rsplit_once("::").is_some_and(|(p, _)| p == *path));
        if !kodex {
            return Err(fail(format!(
                "retire cannot be combined with a runtime change ({path}); weakening and use stay separate"
            )));
        }
    }
    let ledger_before = read(parent, LEDGER_PATH)?;
    let ledger_after = read(child, LEDGER_PATH)?;
    if !ledger_after.starts_with(&ledger_before) {
        return Err(fail("retirement ledger is append-only"));
    }
    let code = super::CODE.iter().map(|(p, _)| *p).collect::<BTreeSet<_>>();
    let mut files = BTreeSet::new();
    for (_, target) in retires {
        let (path, symbol) = target
            .rsplit_once("::")
            .ok_or_else(|| fail("retire target is <path>::<function>"))?;
        if !path.ends_with(".rs")
            || !(path.starts_with("src/build_gate/") || path.starts_with("tools/RUST_"))
            || code.contains(path)
            || RETIRE_PROTECTED.contains(&path)
            || path.contains("..")
        {
            return Err(fail(format!("retire target {path} is protected or not a Kodex check")));
        }
        if fn_count(&read(parent, path)?, symbol) != 1 {
            return Err(fail(format!("retire target {path}::{symbol} is not one parent function")));
        }
        if child.join(path).is_file() && fn_count(&read(child, path)?, symbol) != 0 {
            return Err(fail(format!("retire target {path}::{symbol} still exists")));
        }
        let entry = format!("|{path}|{symbol}#");
        if !ledger_after[ledger_before.len()..].lines().any(|l| l.starts_with("retire=") && l.contains(&entry)) {
            return Err(fail(format!("retire {path}::{symbol} lacks its ledger entry")));
        }
        files.insert(path.to_owned());
    }
    for path in &files {
        if child.join(path).is_file() && !pure_deletion(&read(parent, path)?, &read(child, path)?) {
            return Err(fail(format!("retire may only delete lines in {path}")));
        }
    }
    let anchor_before = read(parent, TRUST_ANCHOR_PATH)?;
    let anchor_after = read(child, TRUST_ANCHOR_PATH)?;
    if anchor_before != anchor_after && pin_normalized(&anchor_before) != pin_normalized(&anchor_after) {
        return Err(fail("retire may rebind trust-anchor pin literals only"));
    }
    Ok(())
}

#[cfg(test)]
#[path = "ssc_governed_change_tests.rs"]
mod tests;
