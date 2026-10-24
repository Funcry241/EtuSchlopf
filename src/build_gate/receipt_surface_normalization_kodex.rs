//! Closed, one-shot governance for receipt surface normalization. No productive implementation.
use crate::rust_source_scan_primitives::{
    fnv1a64, mask_non_code, token_boundary_after, token_boundary_before,
};
use std::fs;
use std::path::Path;
const PERMIT: &str = "src/build_gate/render_receipt_surface_permit_v1.txt";
const CONTRACT: &str = include_str!("render_receipt_surface_permit_v1.txt");
const MACRO_PATH: &str = "src/structural_write_authority/strict_staff_visible_lane_authority.rs";
const ADDITIVE_PATH: &str = "src/structural_write_authority/thirty_surface_productive_authority.rs";
const MACRO_LEGACY: &str = include_str!("receipt_surface_macro_legacy_v1.txt");
const MACRO_MIGRATED: &str = include_str!("receipt_surface_macro_migrated_fixture_v1.txt");
const VERIFIER: &str = include_str!("receipt_surface_existing_additive_verifier_v1.txt");
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Legacy,
    Consumed,
}
struct Function<'a> {
    signature: &'a str,
    body: &'a str,
    start: usize,
}
// Canonicalize code whitespace only. Literal bytes remain significant.
fn contains_token(source: &str, token: &str) -> bool {
    source.match_indices(token).any(|(start,_)| token_boundary_before(source.as_bytes(),start) && token_boundary_after(source.as_bytes(),start+token.len()))
}
fn canonical(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut output = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'"' {
            output.push(bytes[index]);
            index += 1;
            while index < bytes.len() {
                let byte = bytes[index];
                output.push(byte);
                index += 1;
                if byte == b'\\' && index < bytes.len() {
                    output.push(bytes[index]);
                    index += 1;
                } else if byte == b'"' {
                    break;
                }
            }
        } else {
            if !bytes[index].is_ascii_whitespace() {
                output.push(bytes[index]);
            }
            index += 1;
        }
    }
    String::from_utf8(output).expect("canonical source preserves UTF-8")
}
fn functions<'a>(source: &'a str, name: &str) -> Result<Vec<Function<'a>>, String> {
    let masked = mask_non_code(source);
    let bytes = masked.as_bytes();
    let needle = format!("fn {name}");
    let mut cursor = 0;
    let mut out = Vec::new();
    while let Some(relative) = masked[cursor..].find(&needle) {
        let start = cursor + relative;
        let after = start + needle.len();
        cursor = after;
        if !token_boundary_before(bytes, start) || !token_boundary_after(bytes, after) {
            continue;
        }
        let open = masked[after..]
            .find('{')
            .map(|n| n + after)
            .ok_or("receipt owner body missing")?;
        let mut depth = 1;
        let mut end = open + 1;
        while depth > 0 && end < bytes.len() {
            match bytes[end] {
                b'{' => depth += 1,
                b'}' => depth -= 1,
                _ => {}
            }
            end += 1;
        }
        if depth != 0 {
            return Err("receipt owner body unbalanced".into());
        }
        out.push(Function {
            signature: &source[start..open],
            body: &source[open + 1..end - 1],
            start,
        });
    }
    Ok(out)
}
fn one<'a>(source: &'a str, name: &str) -> Result<Function<'a>, String> {
    let mut found = functions(source, name)?;
    if found.len() != 1 {
        return Err(format!("RSP: {name} must exist exactly once"));
    }
    Ok(found.remove(0))
}
fn matches(source: &str, fixture: &str, name: &str) -> Result<bool, String> {
    let actual = one(source, name)?;
    let expected = one(fixture, name)?;
    Ok(canonical(actual.signature) == canonical(expected.signature)
        && canonical(actual.body) == canonical(expected.body))
}
fn macro_calls(source: &str) -> Result<Vec<String>, String> {
    let masked = mask_non_code(source);
    let mut cursor = 0;
    let mut calls = Vec::new();
    let needle = "define_replacement_authority!";
    while let Some(offset) = masked[cursor..].find(needle) {
        let start = cursor + offset;
        let after = start + needle.len();
        cursor = after;
        if !token_boundary_before(masked.as_bytes(), start) {
            continue;
        }
        let open = masked[after..]
            .find('(')
            .map(|n| n + after)
            .ok_or("RSP: malformed macro")?;
        if !masked[after..open].trim().is_empty() {
            return Err("RSP: malformed macro".into());
        }
        let mut depth = 1;
        let mut end = open + 1;
        while depth > 0 && end < masked.len() {
            match masked.as_bytes()[end] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                _ => {}
            }
            end += 1;
        }
        if depth != 0 {
            return Err("RSP: unbalanced macro".into());
        }
        calls.push(canonical(&source[open + 1..end - 1]));
        cursor = end;
    }
    Ok(calls)
}
fn read(root: &Path, path: &str) -> Result<String, String> {
    fs::read_to_string(root.join(path)).map_err(|e| format!("RSP: cannot read {path}: {e}"))
}
const MACROS: &[&str] = &[
    "apply_backup_anchor_rewrite_authorized",
    "apply_duplicate_same_pitch_suppression_authorized",
    "apply_empty_voice_rest_suppression_authorized",
    "apply_hidden_rest_visibility_rewrite_authorized",
    "apply_lower_visible_rest_suppression_authorized",
    "apply_rest_beat_materialization_authorized",
    "apply_upper_duplicate_rest_suppression_authorized",
    "apply_upper_initial_rest_coalescing_authorized",
];
const ADDITIVE: &[(&str, &str, &str, &str, &str)] = &[
    (
        "add_articulation_mark_to_note_fragment_authorized",
        include_str!("receipt_surface_e029_legacy_v1.txt"),
        include_str!("receipt_surface_e029_migrated_fixture_v1.txt"),
        "ArticulationFragmentAdditiveMint",
        "mint_articulation_fragment_receipt",
    ),
    (
        "add_tenuto_to_note_fragment_authorized",
        include_str!("receipt_surface_e031_legacy_v1.txt"),
        include_str!("receipt_surface_e031_migrated_fixture_v1.txt"),
        "TenutoFragmentAdditiveMint",
        "mint_tenuto_fragment_receipt",
    ),
    (
        "add_tie_markup_authorized",
        include_str!("receipt_surface_e032_legacy_v1.txt"),
        include_str!("receipt_surface_e032_migrated_fixture_v1.txt"),
        "TieFragmentAdditiveMint",
        "mint_tie_fragment_receipt",
    ),
];
const TABLES: &[(&str, u64)] = &[
    (
        "src/build_gate/destructive_transformation_population_v1.tsv",
        0xd677cc900f0ea72b,
    ),
    (
        "src/build_gate/destructive_transformation_exclusions_v1.tsv",
        0xc8fefaa66e688b64,
    ),
    (
        "src/build_gate/destructive_transformation_surface_baseline_v1.tsv",
        0x0bb8e11dacf3320d,
    ),
    (
        "src/build_gate/destructive_transformation_nested_owner_v1.tsv",
        0xfcdb5f60b51d98bd,
    ),
    (
        "src/build_gate/render_owner_relocation_v1.tsv",
        0xa81555efafe33a17,
    ),
    (
        "src/build_gate/render_event_boundaries_v1.tsv",
        0x81d3de20ccabf6fe,
    ),
    ("src/build_gate/render_origins_v1.tsv", 0x1a478383a778765a),
    (
        "src/build_gate/render_legacy_selection_v1.tsv",
        0x3809ad499de8a062,
    ),
];
fn legacy_macro_state(source: &str) -> Result<State, String> {
    macro_state(source, false)
}
fn macro_state(source: &str, retirement: bool) -> Result<State, String> {
    let actual = macro_calls(source)?;
    let original = macro_calls(MACRO_LEGACY)?;
    let mut legacy = 0;
    let mut consumed = 0;
    for name in MACROS {
        let needle = format!("{name},");
        let expected = original
            .iter()
            .find(|v| v.starts_with(&needle))
            .ok_or("RSP: missing frozen macro")?;
        let candidates: Vec<_> = actual.iter().filter(|v| v.starts_with(&needle)).collect();
        if candidates.len() == 1
            && *candidates[0] == *expected
            && functions(source, name)?.is_empty()
        {
            legacy += 1;
        } else if candidates.is_empty() && matches(source, MACRO_MIGRATED, name)? {
            let function = one(source, name)?;
            let prelude = canonical(&source[..function.start]);
            if !prelude.ends_with("#[track_caller]pub") {
                return Err("RSP: track_caller/public owner changed".into());
            }
            consumed += 1;
        } else {
            return Err(format!(
                "RSP: macro owner {name} changed identity/body or duplicated"
            ));
        }
    }
    // Locate the real definition in code, never in a comment/string decoy.
    fn actual_definition(source: &str) -> Result<&str, String> {
        let masked = mask_non_code(source);
        let needle = "macro_rules! define_replacement_authority";
        let mut positions = masked.match_indices(needle);
        let (position, _) = positions.next().ok_or("RSP: shared macro missing")?;
        if positions.next().is_some() {
            return Err("RSP: shared macro duplicated".into());
        }
        Ok(&source[position + needle.len()..])
    }
    let frozen_definition = actual_definition(MACRO_LEGACY)?;
    let removed_witnesses = original.iter().filter(|call| !MACROS.iter().any(|name| call.starts_with(&format!("{name},"))) && !actual.contains(call)).count();
    let definition = if retirement && actual.is_empty() { None } else { Some(actual_definition(source)?) };
    // In a migrated file explicit wrappers precede remaining calls. Compare the balanced definition only.
    fn definition_body(s: &str) -> Result<&str, String> {
        let masked = mask_non_code(s);
        let open = masked.find('{').ok_or("RSP: macro body missing")?;
        let mut d = 1;
        let mut e = open + 1;
        while d > 0 && e < masked.len() {
            match masked.as_bytes()[e] {
                b'{' => d += 1,
                b'}' => d -= 1,
                _ => {}
            }
            e += 1;
        }
        if d != 0 {
            return Err("RSP: macro body malformed".into());
        }
        Ok(&s[..e])
    }
    if let Some(definition) = definition
        && canonical(definition_body(definition)?) != canonical(definition_body(frozen_definition)?) {
        return Err("RSP: shared macro widened".into());
    }
    for call in &original {
        if !MACROS
            .iter()
            .any(|name| call.starts_with(&format!("{name},")))
            && actual.iter().filter(|v| *v == call).count() != 1
            && !(retirement && !actual.contains(call) && !mask_non_code(source).contains(call.split(',').next().unwrap_or("")))
        {
            return Err("RSP: unrelated macro contract changed".into());
        }
    }
    if actual.len() != original.len() - consumed - if retirement { removed_witnesses } else { 0 } {
        return Err("RSP: added macro authority".into());
    }
    match (legacy, consumed) {
        (8, 0) => Ok(State::Legacy),
        (0, 8) => Ok(State::Consumed),
        _ => Err("RSP: partial macro migration".into()),
    }
}
const FRAGMENT_ENUM: &str = "enum AdditiveInsertionClass{FinalBarline,Direction,ArticulationFragment,TenutoFragment,TieFragment,}";
const LEGACY_ENUM: &str = "enum AdditiveInsertionClass{FinalBarline,Direction,}";
fn legacy_validate_additive_type_surface(source: &str, status: State) -> Result<(), String> {
    validate_additive_type_surface(source, status, false)
}
fn validate_additive_type_surface(source: &str, status: State, retirement: bool) -> Result<(), String> {
    let masked = mask_non_code(source);
    let start = masked
        .find("enum AdditiveInsertionClass")
        .ok_or("RSP: additive class missing")?;
    let end = masked[start..]
        .find('}')
        .map(|n| n + start + 1)
        .ok_or("RSP: additive class missing")?;
    let expected = if status == State::Legacy {
        LEGACY_ENUM
    } else {
        FRAGMENT_ENUM
    };
    let articulation = contains_token(&masked, "ArticulationFragment");
    let tenuto = contains_token(&masked, "TenutoFragment");
    let mut enum_expected=expected.to_string();
    if retirement && status == State::Consumed {
        if !articulation {enum_expected=enum_expected.replace("ArticulationFragment,", "");}
        if !tenuto {enum_expected=enum_expected.replace("TenutoFragment,", "");}
    }
    if canonical(&masked[start..end]) != canonical(&enum_expected) {
        return Err("RSP: arbitrary additive insertion class".into());
    }
    let method = one(source, "element_name")?;
    let legacy =
        "match self { Self::FinalBarline => \"barline\", Self::Direction => \"direction\", }";
    let consumed = "match self { Self::FinalBarline => \"barline\", Self::Direction => \"direction\", Self::ArticulationFragment | Self::TenutoFragment | Self::TieFragment => \"__owner_local_fragment_only\", }";
    let mut matcher=if status==State::Legacy {legacy.to_string()} else {consumed.to_string()};
    if retirement && status==State::Consumed {
        if !articulation {matcher=matcher.replace("Self::ArticulationFragment | ", "");}
        if !tenuto {matcher=matcher.replace("Self::TenutoFragment | ", "");}
    }
    if canonical(method.body) != canonical(&matcher)
    {
        return Err("RSP: generic additive class matcher changed".into());
    }
    let compact = canonical(&masked);
    if compact.contains("implAdditiveMintOwnerfor") {
        return Err("RSP: generic additive owner impl added".into());
    }
    let prefix = "bind_mint_owner!(AdditiveMintOwner,";
    let calls: Vec<_> = compact
        .match_indices(prefix)
        .map(|(start, _)| compact[start..].split(");").next().unwrap_or(""))
        .collect();
    let expected_calls = [
        "bind_mint_owner!(AdditiveMintOwner,crate::structural_write_authority::FinalBarlineAdditiveMint,AdditiveDescriptor,FINAL_BARLINE_ADDITIVE",
        "bind_mint_owner!(AdditiveMintOwner,crate::structural_write_authority::DirectionInsertionAdditiveMint,AdditiveDescriptor,DIRECTION_INSERTION_ADDITIVE",
    ];
    if calls != expected_calls {
        return Err("RSP: generic additive mint owner widened".into());
    }
    if status == State::Consumed {
        validate_runtime_delta(source, source)?;
    }
    Ok(())
}
fn state(root: &Path) -> Result<State, String> {
    if read(root, PERMIT)? != CONTRACT {
        return Err("RSP: closed permit changed".into());
    }
    let retirement=crate::take_change_control::ssc_evidence_proof_shadow_permit_kodex::k2::installed(root);
    let writer_retired=crate::take_change_control::ssc_evidence_proof_shadow_permit_kodex::k2::checkpoint_prepare::writer_retirement_state(root)?;
    for (path, hash) in TABLES {
        if writer_retired && matches!(*path,
            "src/build_gate/destructive_transformation_exclusions_v1.tsv" |
            "src/build_gate/destructive_transformation_surface_baseline_v1.tsv" |
            "src/build_gate/render_legacy_selection_v1.tsv") { continue; }
        if matches!(*path, "src/build_gate/render_origins_v1.tsv" | "src/build_gate/ssc_containment_envelopes_v1.tsv")
            && crate::take_change_control::ssc_evidence_proof_shadow_permit_kodex::k2::cursor_temporal::installed(root)
        {
            crate::take_change_control::ssc_evidence_proof_shadow_permit_kodex::k2::cursor_temporal::validate_tree(root)?;
            continue;
        }
        if retirement && *path=="src/build_gate/destructive_transformation_population_v1.tsv" {
            crate::take_change_control::ssc_evidence_proof_shadow_permit_kodex::k2::validate_population(root)?;
            continue;
        }
        if fnv1a64(read(root, path)?.as_bytes()) != *hash {
            return Err(format!("RSP: frozen table changed {path}"));
        }
    }
    let macro_status = if retirement { macro_state(&read(root, MACRO_PATH)?, true)? } else { legacy_macro_state(&read(root, MACRO_PATH)?)? };
    let additive = read(root, ADDITIVE_PATH)?;
    for (name, legacy, migrated, _, _) in ADDITIVE {
        if retirement && *name != "add_tie_markup_authorized" && functions(&additive, name)?.is_empty() { continue; }
        if !matches(
            &additive,
            if macro_status == State::Legacy {
                legacy
            } else {
                migrated
            },
            name,
        )? {
            return Err(format!("RSP: non-atomic/additive body drift {name}"));
        }
    }
    let runtime = read(root, "src/master_state_identity.rs")?;
    let runtime_code = mask_non_code(&runtime);
    if retirement { validate_additive_type_surface(&runtime, macro_status, true)?; } else { legacy_validate_additive_type_surface(&runtime, macro_status)?; }
    let start = runtime_code
        .find("impl AdditiveTransformationReceipt {")
        .ok_or("RSP: existing verifier absent")?;
    let end = runtime_code[start..]
        .find("fn conserved_projection_is_subsequence")
        .map(|i| i + start)
        .ok_or("RSP: existing verifier absent")?;
    if canonical(&runtime[start..end]) != canonical(VERIFIER) {
        return Err("RSP: E004/E036 verifier changed".into());
    }
    for (_, _, _, token, name) in ADDITIVE {
        let found = functions(&runtime, name)?;
        if macro_status == State::Legacy {
            if !found.is_empty() {
                return Err("RSP: early additive runtime capability".into());
            }
        } else {
            if retirement && *name != "mint_tie_fragment_receipt" && found.is_empty() {
                let fragment=if *name=="mint_articulation_fragment_receipt" {"ArticulationFragment"} else {"TenutoFragment"};
                if contains_token(&runtime_code, fragment) || contains_token(&runtime_code, token) { return Err("RSP: partial fragment retirement".into()); }
                continue;
            }
            let expected = format!(
                "fn {name}(_owner:crate::structural_write_authority::{token},pre_xml:&str,post_xml:&str,insertions:Vec<ExactEdit>)->Option<Self>"
            );
            if found.len() != 1
                || canonical(found[0].signature).replace(",)", ")") != canonical(&expected)
            {
                return Err("RSP: missing/unsealed fragment runtime".into());
            }
        }
    }
    Ok(macro_status)
}
pub(crate) fn validate_current_tree(root: &Path) -> Result<(), String> {
    if !root.join(PERMIT).exists() {
        return Ok(());
    }
    state(root).map(|_| ())
}
fn metadata(root: &Path) -> Result<std::collections::BTreeMap<String, String>, String> {
    let path = if root.join("ZIP_KIND.txt").is_file() {
        root.join("ZIP_KIND.txt")
    } else {
        root.join("root_files/ZIP_KIND.txt")
    };
    let raw = fs::read_to_string(path).map_err(|e| format!("RSP: take identity absent: {e}"))?;
    let mut out = std::collections::BTreeMap::new();
    for line in raw.lines() {
        if let Some((k, v)) = line.split_once('=')
            && out.insert(k.to_string(), v.to_string()).is_some()
        {
            return Err("RSP: duplicate take metadata".into());
        }
    }
    Ok(out)
}
fn identity(parent: &Path, child: &Path, class: &str, install: bool) -> Result<(), String> {
    let p = metadata(parent)?;
    let c = metadata(child)?;
    let get = |m: &std::collections::BTreeMap<String, String>, k: &str| {
        m.get(k).cloned().unwrap_or_default()
    };
    let (pn, cn, revision, required) = if install {
        (
            "ToolchainSignalHygiene",
            "RenderReceiptSurfaceNormalizationPermit",
            1225,
            "KodexOnly",
        )
    } else {
        (
            "RenderReceiptFixtureClosurePermit",
            "RenderReceiptSurfaceNormalization",
            1228,
            "RefactorOnly",
        )
    };
    let old = format!("r{revision}");
    let next = format!("r{}", revision + 1);
    if class != required
        || get(&p, "name") != pn
        || get(&p, "revision") != old
        || get(&c, "name") != cn
        || get(&c, "revision") != next
        || get(&c, "parent_revision") != old
        || get(&p, "version") != "0.10.1839"
        || get(&c, "version") != "0.10.1839"
        || get(&c, "kind") != "EtuSchlopfTake"
    {
        return Err("RSP: only the exact immediate typed child may consume this permit".into());
    }
    Ok(())
}
// Only the three owner-specific ADDITIVE impl methods receive new authority.
// Ordinary typed EXACT/BLOCK/PRESERVING descriptor additions remain governed
// by the existing migrations; no additional generic ADDITIVE impl is admitted.
fn validate_runtime_delta(_parent: &str, child: &str) -> Result<(), String> {
    let masked = mask_non_code(child);
    let needle = "impl AdditiveTransformationReceipt";
    let mut cursor = 0;
    let original_impl = masked
        .find(needle)
        .ok_or("RSP: existing additive impl missing")?;
    while let Some(offset) = masked[cursor..].find(needle) {
        let start = cursor + offset;
        let open = masked[start..]
            .find('{')
            .map(|n| n + start)
            .ok_or("RSP: impl missing")?;
        let mut depth = 1;
        let mut end = open + 1;
        while depth > 0 && end < masked.len() {
            match masked.as_bytes()[end] {
                b'{' => depth += 1,
                b'}' => depth -= 1,
                _ => {}
            }
            end += 1;
        }
        if depth != 0 {
            return Err("RSP: impl unbalanced".into());
        }
        let body = &child[open + 1..end - 1];
        let mut residue = canonical(body);
        let mut targeted = false;
        for (_, _, _, _, name) in ADDITIVE {
            let found = functions(body, name)?;
            for function in found {
                targeted = true;
                let method = format!(
                    "{}{{{}}}",
                    canonical(function.signature),
                    canonical(function.body)
                );
                let visible = format!("pub(crate){method}");
                if residue.contains(&visible) {
                    residue = residue.replacen(&visible, "", 1);
                } else {
                    residue = residue.replacen(&method, "", 1);
                }
            }
        }
        if targeted {
            if !residue.is_empty() {
                return Err("RSP: extra generic runtime authority".into());
            }
        } else if start != original_impl {
            return Err("RSP: extra generic additive runtime impl".into());
        }
        cursor = end;
    }
    // Other existing EXACT/BLOCK/PRESERVING receipt migrations may legitimately
    // add descriptors and sealed bindings in this file. Only the ADDITIVE
    // construction surface receives new authority here.
    Ok(())
}
pub(crate) fn validate_transition(parent: &Path, child: &Path, class: &str) -> Result<(), String> {
    let before = parent.join(PERMIT).exists();
    let after = child.join(PERMIT).exists();
    match (before, after) {
        (false, false) => Ok(()),
        (true, false) => Err("RSP: permit cannot disappear".into()),
        (false, true) => {
            identity(parent, child, class, true)?;
            if state(child)? != State::Legacy {
                return Err("RSP: KodexOnly cannot perform normalization".into());
            }
            Ok(())
        }
        (true, true) => {
            let p = state(parent)?;
            let c = state(child)?;
            match (p, c) {
                (State::Legacy, State::Consumed) => {
                    identity(parent, child, class, false)?;
                    validate_runtime_delta(
                        &read(parent, "src/master_state_identity.rs")?,
                        &read(child, "src/master_state_identity.rs")?,
                    )?;
                    let old = read(parent, MACRO_PATH)?;
                    let new = read(child, MACRO_PATH)?;
                    if canonical(
                        old.split("macro_rules! define_replacement_authority")
                            .next()
                            .unwrap_or(""),
                    ) != canonical(
                        new.split("macro_rules! define_replacement_authority")
                            .next()
                            .unwrap_or(""),
                    ) {
                        return Err(
                            "RSP: shared selection/batch guards changed during migration".into(),
                        );
                    }
                    Ok(())
                }
                (State::Consumed, State::Legacy) => {
                    Err("RSP: receipt migration cannot reverse".into())
                }
                _ => Ok(()),
            }
        }
    }
}
pub(crate) fn admits_macro_owner(root: &Path, owner: &str, frozen: &str) -> Result<bool, String> {
    let Some((path, name)) = owner.rsplit_once("::") else {
        return Ok(false);
    };
    if !root.join(PERMIT).exists()
        || path != MACRO_PATH
        || !MACROS.contains(&name)
        || !frozen.starts_with("macro:fn ")
    {
        return Ok(false);
    }
    Ok(state(root)? == State::Consumed)
}
#[cfg(test)]
#[path = "receipt_surface_normalization_kodex_tests.rs"]
mod tests;
