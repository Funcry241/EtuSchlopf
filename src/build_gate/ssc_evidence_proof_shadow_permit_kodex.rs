//! Closed SSC observation contract. This gate grants no musical or writer authority.
use crate::rust_source_scan_primitives::{
    collect_rust_files, mask_non_code, normalize_path, token_boundary_after, token_boundary_before,
};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

#[path = "ssc_k2_retirement_governance.rs"]
pub(crate) mod k2;
#[path = "ssc_structural_erasure.rs"]
pub(crate) mod structural_erasure;

const CONTRACT_PATH: &str = "src/build_gate/ssc_evidence_proof_shadow_contract_v1.txt";
const CONTRACT: &str = include_str!("ssc_evidence_proof_shadow_contract_v1.txt");
const BASELINE_PATH: &str = "src/build_gate/ssc_exit_dependencies_r1238_v1.tsv";
const BASELINE: &str = include_str!("ssc_exit_dependencies_r1238_v1.tsv");
const REMAINING_PATH: &str = "src/build_gate/ssc_exit_remaining_v1.txt";
// Immutable r1238 initial set; the current OPEN file may shrink under K2.
const INITIAL_REMAINING: &str = "schema=ssc_exit_remaining_v1\nexit_claim=not_closed\nopen=deferred_render_57\nopen=deferred_render_67\nopen=wav_pending_28\nopen=wav_accent_productive\nopen=wav_salience_master_adoptions\nopen=wav_metronome\nopen=e031_tenuto\nopen=ra5_accent_reserve\nopen=macro_hand_role\nopen=macro_cross_staff\nopen=macro_terminal_shadow\nopen=macro_same_pitch_tie\nopen=macro_lower_rhythm_lane\nopen=master_receipt_legacy_alias_contract\nopen=audit_carrier_grandfathering\n";
const PROOF_PATH: &str = "src/ssc_evidence_proof.rs";
const SHADOW_PATH: &str = "src/ssc_shadow_permit.rs";
const SINKS: [&str; 2] = ["src/ssc_shadow_report.rs", "src/ssc_shadow_measurement.rs"];
const SINK_NAMES: [&str; 4] = [
    "ssc_shadow_report",
    "ssc_shadow_measurement",
    "append_ssc_shadow_observation",
    "append_ssc_shadow_measurement",
];

fn fail(message: impl std::fmt::Display) -> String {
    format!("SSC SHADOW CONTRACT: {message}")
}
fn read(root: &Path, path: &str) -> Result<String, String> {
    fs::read_to_string(root.join(path))
        .map_err(|error| fail(format!("cannot read {path}: {error}")))
}
fn code(source: &str) -> String {
    mask_non_code(source).split_whitespace().collect()
}
fn contains_token(source: &str, token: &str) -> bool {
    source.match_indices(token).any(|(start, _)| {
        token_boundary_before(source.as_bytes(), start)
            && token_boundary_after(source.as_bytes(), start + token.len())
    })
}
fn item<'a>(source: &'a str, kind: &str, name: &str) -> Result<&'a str, String> {
    let masked = mask_non_code(source);
    let needle = format!("{kind} {name}");
    let matches = masked
        .match_indices(&needle)
        .filter(|(start, _)| {
            token_boundary_before(masked.as_bytes(), *start)
                && token_boundary_after(masked.as_bytes(), *start + needle.len())
        })
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return Err(fail(format!("{name} must exist exactly once")));
    }
    let start = matches[0].0;
    let open = start
        + masked[start..]
            .find('{')
            .ok_or_else(|| fail("missing item body"))?;
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
        return Err(fail("unbalanced item"));
    }
    Ok(&source[start..end])
}
fn remaining(text: &str) -> Result<BTreeSet<String>, String> {
    let mut lines = text.lines();
    if lines.next() != Some("schema=ssc_exit_remaining_v1")
        || lines.next() != Some("exit_claim=not_closed")
    {
        return Err(fail("remaining-set schema or premature SSC exit claim"));
    }
    let mut result = BTreeSet::new();
    for line in lines {
        let id = line
            .strip_prefix("open=")
            .ok_or_else(|| fail("unknown remaining-set field"))?;
        if id.is_empty()
            || !id
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
            || !result.insert(id.to_owned())
        {
            return Err(fail("empty, malformed or duplicate obligation"));
        }
    }
    Ok(result)
}
fn monotone(parent: &BTreeSet<String>, child: &BTreeSet<String>) -> Result<(), String> {
    if !child.is_subset(parent) {
        return Err(fail("N_A may not grow or reopen a resolved obligation"));
    }
    Ok(())
}
fn baseline_ids(text: &str) -> Result<BTreeSet<String>, String> {
    let mut lines = text.lines();
    if lines.next() != Some("# SSC exit dependencies r1238 v1")
        || lines.next() != Some("id\tparent_status\tanchor_path\tanchor_symbol")
    {
        return Err(fail("baseline schema"));
    }
    let mut ids = BTreeSet::new();
    for line in lines {
        let row = line.split('\t').collect::<Vec<_>>();
        if row.len() != 4
            || row[1] != "OPEN"
            || !row[2].starts_with("src/")
            || row[3].is_empty()
            || !ids.insert(row[0].to_string())
        {
            return Err(fail(
                "baseline must contain unique, anchored OPEN obligations",
            ));
        }
    }
    Ok(ids)
}
fn validate_remaining(root: &Path) -> Result<(), String> {
    if read(root, BASELINE_PATH)? != BASELINE {
        return Err(fail("r1238 baseline changed"));
    }
    let initial = remaining(INITIAL_REMAINING)?;
    if initial != baseline_ids(BASELINE)? {
        return Err(fail("baseline and initial remaining set differ"));
    }
    let actual = remaining(&read(root, REMAINING_PATH)?)?;
    monotone(&initial, &actual)?;
    // This prerequisite installs no resolution capability. K2 must supply one later.
    if actual != initial && !k2::installed(root) {
        return Err(fail(
            "obligation resolution requires the later SSC-K2 contract; no N_A-empty claim is granted here",
        ));
    }
    Ok(())
}
fn forbid_tokens(source: &str, tokens: &[&str], reason: &str) -> Result<(), String> {
    let masked = mask_non_code(source);
    if let Some(token) = tokens.iter().find(|token| contains_token(&masked, token)) {
        return Err(fail(format!("{reason}: {token}")));
    }
    Ok(())
}
fn validate_exports(source: &str, allowed: &str) -> Result<(), String> {
    let masked = mask_non_code(source);
    let compact = code(source);
    if compact.contains("pubuse")
        || compact.contains("pub(crate)use")
        || compact.contains("pubtype")
        || compact.contains("pub(crate)type")
        || compact.contains("pubstatic")
        || compact.contains("pubconst")
    {
        return Err(fail(
            "Shadow and its sinks may not reexport or alias verdicts",
        ));
    }
    for (start, _) in masked.match_indices("pub") {
        if !token_boundary_before(masked.as_bytes(), start)
            || !token_boundary_after(masked.as_bytes(), start + 3)
        {
            continue;
        }
        let tail = &masked[start..];
        let Some(open) = tail.find('{') else {
            return Err(fail("unresolved exported item"));
        };
        let head = &tail[..open];
        if !head.contains("fn ") {
            return Err(fail("unapproved public Shadow surface"));
        }
        if head.contains("->")
            || head.contains("Fn")
            || code(head).replace("&mutString", "").contains("&mut")
        {
            return Err(fail(
                "report/measurement exports return unit and accept no decision callbacks or mutable authority",
            ));
        }
        if !head.contains(&format!("fn {allowed}")) {
            return Err(fail("unapproved Shadow export"));
        }
    }
    Ok(())
}
fn validate_sink(path: &str, source: &str) -> Result<(), String> {
    forbid_tokens(
        source,
        &[
            "bool",
            "structural_write_authority",
            "MasterAdoptedState",
            "RenderSovereignState",
            "OwnerReceipts",
            "AtomicBool",
            "Cell",
            "RefCell",
            "Mutex",
            "OnceLock",
            "env",
            "fs",
            "File",
            "process",
            "From",
            "TryFrom",
            "Deref",
            "PartialEq",
            "Ord",
        ],
        "report/measurement sink must be non-authorizing and free of writer side effects",
    )?;
    let allowed = if path == SINKS[0] {
        SINK_NAMES[2]
    } else {
        SINK_NAMES[3]
    };
    validate_exports(source, allowed)
}
pub(crate) fn validate_current_tree(root: &Path) -> Result<(), String> {
    if !root.join(CONTRACT_PATH).exists() {
        return Ok(());
    }
    if read(root, CONTRACT_PATH)? != CONTRACT {
        return Err(fail("closed evidence/proof/shadow contract changed"));
    }
    validate_remaining(root)?;
    k2::validate_tree(root)?;
    let proof_present = root.join(PROOF_PATH).is_file();
    let shadow_present = root.join(SHADOW_PATH).is_file();
    if proof_present != shadow_present {
        return Err(fail("partial evidence/proof/shadow implementation"));
    }
    let mut files = Vec::new();
    collect_rust_files(&root.join("src"), &mut files).map_err(|error| fail(error.to_string()))?;
    for file in files {
        let path = normalize_path(
            file.strip_prefix(root)
                .map_err(|error| fail(error.to_string()))?,
        );
        let source = fs::read_to_string(&file).map_err(|error| fail(error.to_string()))?;
        if SINKS.contains(&path.as_str()) {
            if !shadow_present {
                return Err(fail("observation sink without shadow core"));
            }
            validate_sink(&path, &source)?;
        }
    }
    Ok(())
}
pub(crate) fn validate_transition(parent: &Path, child: &Path, class: &str) -> Result<(), String> {
    k2::validate_transition(parent, child, class)?;
    let prior = parent.join(CONTRACT_PATH).exists();
    let next = child.join(CONTRACT_PATH).exists();
    if prior && !next {
        return Err(fail("installed SSC gate cannot disappear"));
    }
    if !next {
        return Ok(());
    }
    validate_current_tree(child)?;
    if prior {
        // Contract v4 B4.2: the accepted parent is not re-judged by the child's rules;
        // only the parent-relative transition (monotone OPEN set) is checked here.
        monotone(
            &remaining(&read(parent, REMAINING_PATH)?)?,
            &remaining(&read(child, REMAINING_PATH)?)?,
        )?;
    } else {
        return Err(fail(
            "SSC contract installation epoch is closed; a parent without the contract is older than the epoch base",
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "ssc_evidence_proof_shadow_permit_kodex_tests.rs"]
mod tests;
