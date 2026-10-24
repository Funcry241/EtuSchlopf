//! K-R: closed Render origin/event contracts and one atomic future #67 relocation.
//! This module owns governance only; it neither creates nor adopts a Render document.
use crate::rust_source_scan_primitives::{
    collect_rust_files, mask_non_code, token_boundary_after, token_boundary_before,
};
use std::fs;
use std::path::Path;

pub(crate) const RELOCATION_PATH: &str = "src/build_gate/render_owner_relocation_v1.tsv";
pub(crate) const BOUNDARIES_PATH: &str = "src/build_gate/render_event_boundaries_v1.tsv";
pub(crate) const ORIGINS_PATH: &str = "src/build_gate/render_origins_v1.tsv";
pub(crate) const SELECTION_PATH: &str = "src/build_gate/render_legacy_selection_v1.tsv";
const POPULATION_PATH: &str = "src/build_gate/destructive_transformation_population_v1.tsv";
const BUDGET_PATH: &str = "src/build_gate/destructive_transformation_surface_baseline_v1.tsv";
pub(crate) const LEGACY_PATH: &str =
    "src/render/silent_voice_visible_measure_coverage_render_writer.rs";
pub(crate) const LEGACY_FUNCTION: &str =
    "materialize_silent_voice_visible_measure_coverage_for_pdf_source";
pub(crate) const TARGET_FUNCTION: &str = "same_lane_strict_interior_raw_splice";
pub(crate) const TARGET_SIGNATURE: &str = "fn same_lane_strict_interior_raw_splice( final_candidate: &str, edit_start: usize, edit_end: usize, replacement: &str, ) -> String";
const LEGACY_RAW_SPLICE: &str = "let mut candidate = String::with_capacity(final_candidate.len() + replacement.len()); candidate.push_str(&final_candidate[..edit_start]); candidate.push_str(&replacement); candidate.push_str(&final_candidate[edit_end..]);";
const RAW_SPLICE: &str = "let mut candidate = String::with_capacity(final_candidate.len() + replacement.len()); candidate.push_str(&final_candidate[..edit_start]); candidate.push_str(replacement); candidate.push_str(&final_candidate[edit_end..]);";
const SPLICE_CALL: &str = "let candidate = same_lane_strict_interior_raw_splice(&final_candidate, edit_start, edit_end, &replacement);";
const RECEIPT_SPLICE_CALL: &str = "let (candidate, same_lane_splice_receipt) = same_lane_strict_interior_raw_splice(&final_candidate, edit_start, edit_end, &replacement);";
const CONDITION: &str =
    "if let Some((edit_start, edit_end, replacement)) = same_lane_strict_interior_edit";
const K_R_NAME: &str = "RenderSovereigntyPrerequisiteKodex";
// K-R4: preserve the legacy String borrow, but admit only the canonical &str helper insertion.
// Immutable wrapper bindings remain frozen; this changes no splice geometry.
// The accepted bootstrap still validates the immutable r1220 base TSV before copy.
// Amend only its effective three permit bindings; every splice/owner/budget field
// remains validated against the original closed table. No selectable chain exists.
const PERMIT_PARENT_REVISION: &str = "r1223";
const PERMIT_PARENT_NAME: &str = "RenderOwnerBoundaryClippyCanonicalizationPermit";
const REFACTOR_NAME: &str = "RenderOwnerBoundaryNormalization";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Row67State {
    Legacy,
    Relocated,
}

#[derive(Debug)]
struct FunctionSource<'a> {
    signature: &'a str,
    body: &'a str,
}

fn compact(source: &str) -> String {
    source.split_whitespace().collect()
}

fn code(source: &str) -> String {
    compact(&mask_non_code(source))
}

fn functions<'a>(source: &'a str, name: &str) -> Result<Vec<FunctionSource<'a>>, String> {
    let masked = mask_non_code(source);
    let bytes = masked.as_bytes();
    let needle = format!("fn {name}");
    let mut cursor = 0;
    let mut found = Vec::new();
    while let Some(relative) = masked[cursor..].find(&needle) {
        let start = cursor + relative;
        let after = start + needle.len();
        cursor = after;
        if !token_boundary_before(bytes, start) || !token_boundary_after(bytes, after) {
            continue;
        }
        let open = masked[after..]
            .find('{')
            .map(|offset| after + offset)
            .ok_or_else(|| format!("K-R: function {name} has no body"))?;
        let mut depth = 1_usize;
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
            return Err(format!("K-R: unterminated function {name}"));
        }
        found.push(FunctionSource {
            signature: &source[start..open],
            body: &source[open + 1..end - 1],
        });
    }
    Ok(found)
}

fn exactly_one<'a>(source: &'a str, name: &str) -> Result<FunctionSource<'a>, String> {
    let mut found = functions(source, name)?;
    if found.len() != 1 {
        return Err(format!(
            "K-R: owner {name} must exist exactly once, got {}",
            found.len()
        ));
    }
    Ok(found.remove(0))
}

fn read(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative))
        .map_err(|error| format!("K-R: cannot read {relative}: {error}"))
}

fn root_metadata(root: &Path) -> Result<String, String> {
    let direct = root.join("ZIP_KIND.txt");
    let path = if direct.is_file() {
        direct
    } else {
        root.join("root_files/ZIP_KIND.txt")
    };
    fs::read_to_string(path).map_err(|error| format!("K-R: missing Take identity: {error}"))
}

fn value<'a>(text: &'a str, key: &str) -> Result<&'a str, String> {
    let needle = format!("{key}=");
    let values = text
        .lines()
        .filter_map(|line| line.strip_prefix(&needle))
        .collect::<Vec<_>>();
    if values.len() != 1 || values[0].trim().is_empty() {
        return Err(format!("K-R: {key} must have exactly one nonempty value"));
    }
    Ok(values[0])
}

fn table<'a>(text: &'a str, header: &str, width: usize) -> Result<Vec<Vec<&'a str>>, String> {
    let mut lines = text.lines();
    if lines.next() != Some(header) {
        return Err("K-R: contract TSV header changed".to_string());
    }
    let mut rows = Vec::new();
    for line in lines {
        let row = line.split('\t').collect::<Vec<_>>();
        let optional_note = width == 11 && header.ends_with("\tnote");
        if row.len() != width
            || row
                .iter()
                .enumerate()
                .any(|(index, field)| field.is_empty() && !(optional_note && index == 10))
        {
            return Err("K-R: incomplete contract row".to_string());
        }
        rows.push(row);
    }
    Ok(rows)
}

fn relocation(text: &str) -> Result<Vec<&str>, String> {
    let rows = table(
        text,
        "population_id\tlegacy_owner\ttarget_owner\ttarget_signature\tidentity_kind\tdomain\tprerequisite_state\tgeometry\tlegacy_occurrences\tbuffer_splices\tfile_writes\tparent_revision\tparent_name\trefactor_name",
        14,
    )?;
    if rows.len() != 1 {
        return Err("K-R: exactly one future owner relocation is authorized".to_string());
    }
    let row = &rows[0];
    if row[0] != "67"
        || row[1] != format!("{LEGACY_PATH}::{LEGACY_FUNCTION}")
        || row[2] != format!("{LEGACY_PATH}::{TARGET_FUNCTION}")
        || row[3] != TARGET_SIGNATURE
        || row[4] != "EXACT"
        || row[5] != "R"
        || row[6] != "DEFERRED_RENDER_PREREQUISITE"
        || row[7] != "(edit_start, edit_end, replacement)"
        || row[8] != "1"
        || row[9] != "1"
        || row[10] != "0"
        || row[11] != "r1220"
        || row[12] != K_R_NAME
        || row[13] != "RenderSovereigntyClosureMaxi"
    {
        return Err(
            "K-R: future relocation is frozen exclusively to the same #67 Raw-Splice".to_string(),
        );
    }
    let mut effective = rows.into_iter().next().expect("one row");
    effective[11] = PERMIT_PARENT_REVISION;
    effective[12] = PERMIT_PARENT_NAME;
    effective[13] = REFACTOR_NAME;
    Ok(effective)
}

pub(crate) fn validate_row67_source(source: &str) -> Result<Row67State, String> {
    let wrapper = exactly_one(source, LEGACY_FUNCTION)?;
    let wrapper_code = code(wrapper.body);
    let legacy_core = code(LEGACY_RAW_SPLICE);
    let targets = functions(source, TARGET_FUNCTION)?;
    if targets.is_empty() {
        let core = &legacy_core;
        if wrapper_code.matches(core).count() != 1
            || code(source).matches(core).count() != 1
            || !wrapper_code.contains(&format!("{}{{{core}", code(CONDITION)))
            || wrapper_code.contains(&format!("{TARGET_FUNCTION}("))
        {
            return Err(
                "K-R: Legacy #67 must retain exactly its one bound inline Raw-Splice".to_string(),
            );
        }
        return Ok(Row67State::Legacy);
    }
    if targets.len() != 1 {
        return Err("K-R: relocated #67 target must exist exactly once".to_string());
    }
    let core = code(RAW_SPLICE);
    let target = &targets[0];
    let signature = compact(target.signature);
    let original = compact(TARGET_SIGNATURE);
    let receipt_signature = compact(&TARGET_SIGNATURE.replace(
        "-> String",
        "-> (String, Option<ExactTransformationReceipt>)",
    ));
    let body = code(target.body);
    let valid_body = if signature == original {
        body == format!("{core}candidate")
    } else if signature == receipt_signature {
        body.strip_prefix(&format!("{core}letreceipt="))
            .and_then(|tail| tail.strip_suffix(";(candidate,receipt)"))
            .is_some_and(valid_receipt_mint_expression)
    } else {
        false
    };
    let expected_call = if signature == original {
        code(SPLICE_CALL)
    } else {
        code(RECEIPT_SPLICE_CALL)
    };
    if !valid_body
        || code(source).contains(&legacy_core)
        || wrapper_code.contains(&core)
        || wrapper_code.contains("[..edit_start]")
        || wrapper_code.contains("[edit_end..]")
        || code(source).matches(&core).count() != 1
        || wrapper_code.matches(&format!("{TARGET_FUNCTION}(")).count() != 1
        || !wrapper_code.contains(&format!("{}{{{expected_call}", code(CONDITION)))
    {
        return Err("K-R: #67 relocation must atomically preserve the exact Raw-Splice, its guarded call and one-splice/zero-file-write budget".to_string());
    }
    Ok(Row67State::Relocated)
}

fn valid_receipt_mint_expression(expression: &str) -> bool {
    let Some(path) =
        expression.strip_suffix("(final_candidate,edit_start,edit_end,replacement,&candidate)")
    else {
        return false;
    };
    // Only one typed mint call, bound to the unchanged pre coordinates and read-only post.
    // No additional document operation, mutable argument, output substitution or statement.
    !path.is_empty()
        && path.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || character == ':'
        })
}

pub(crate) fn effective_row67_owner(state: Row67State) -> (&'static str, &'static str) {
    match state {
        Row67State::Legacy => (LEGACY_PATH, LEGACY_FUNCTION),
        Row67State::Relocated => (LEGACY_PATH, TARGET_FUNCTION),
    }
}

fn validate_target_uses(root: &Path, state: Row67State) -> Result<(), String> {
    let mut paths = Vec::new();
    collect_rust_files(&root.join("src"), &mut paths)
        .map_err(|error| format!("K-R: cannot inspect target uses: {error}"))?;
    let mut uses = 0_usize;
    for path in paths {
        let source = fs::read_to_string(&path)
            .map_err(|error| format!("K-R: cannot read {}: {error}", path.display()))?;
        let masked = mask_non_code(&source);
        for (offset, _) in masked.match_indices(TARGET_FUNCTION) {
            if token_boundary_before(masked.as_bytes(), offset)
                && token_boundary_after(masked.as_bytes(), offset + TARGET_FUNCTION.len())
            {
                uses += 1;
                if path != root.join(LEGACY_PATH) {
                    return Err("K-R: #67 target cannot be defined, imported or called outside its frozen owner source".to_string());
                }
            }
        }
    }
    let expected = if state == Row67State::Legacy { 0 } else { 2 };
    if uses != expected {
        return Err(format!(
            "K-R: #67 target permits only its one definition and one guarded caller; got {uses}, expected {expected}"
        ));
    }
    Ok(())
}

fn validate_population_contract(text: &str, k2: bool) -> Result<(), String> {
    let rows = table(
        text,
        "id\towner\towner_signature\teffect_class\tidentity_kind\tdomain\tedit_source\tadoption_archetype\treview_status\tprerequisite_state\tnote",
        11,
    )?;
    let row = rows
        .iter()
        .find(|row| row[0] == "67")
        .ok_or("K-R: population #67 is missing")?;
    if rows.iter().filter(|row| row[0] == "67").count() != 1
        || row[1] != format!("{LEGACY_PATH}::{LEGACY_FUNCTION}")
        || row[3] != "DESTRUCTIVE"
        || row[4] != "EXACT"
        || row[5] != "R"
        || (row[9] != "DEFERRED_RENDER_PREREQUISITE" && !(k2 && row[9] == "READY"))
        || !row[6].starts_with("(edit_start, edit_end, replacement) raw splice")
    {
        return Err(
            "K-R: stable population #67 identity/class/domain/prerequisite/geometry changed"
                .to_string(),
        );
    }
    for (id, owner, signature, geometry) in [
        ("2", CURSOR_OWNER, CURSOR_SIGNATURE, "Vec<TextEdit>"),
        (
            "69",
            "src/structural_write_authority/render_measure_articulation_authority.rs::apply_bound_missing_rest_batch_authorized",
            "fn apply_bound_missing_rest_batch_authorized( source: &str, edits: Vec<(usize, usize, String)>, ) -> Option<String>",
            "(forward_start, forward_end, rest_note)",
        ),
    ] {
        let row = rows
            .iter()
            .find(|row| row[0] == id)
            .ok_or_else(|| format!("K-R: population {id} missing"))?;
        if rows.iter().filter(|row| row[0] == id).count() != 1
            || row[1] != owner
            || row[2] != signature
            || row[3] != "DESTRUCTIVE"
            || row[4] != "EXACT"
            || row[5] != "R"
            || !row[6].starts_with(geometry)
        {
            return Err(format!(
                "K-R: population {id} owner/signature/class/domain/geometry changed"
            ));
        }
    }
    // The original population record stays trust-pinned. Only the effective physical
    // anchor changes, by the already-frozen joint relocation contract, never by TSV edits.
    Ok(())
}

const CURSOR_OWNER: &str =
    "src/structural_write_authority/writer_entry_boundary.rs::apply_authorized_serialization_plan";
const DIRECTION_OWNER: &str = "src/structural_write_authority/renderer_direction_relocation_writer_boundary.rs::compose_authorized_direction_delta_onto_sink";
const CURSOR_SIGNATURE: &str = "fn apply_authorized_serialization_plan( source_xml: &str, locator: &super::StructuralLocator, plan: &AuthorizedSerializationPlan, source_stage: &str, audit: &mut AuthorizedWriterEntryBoundaryAudit, direction_relocation_plan: &mut Option<ProvenRendererDirectionRelocationPlan>, ) -> String";
const DIRECTION_SIGNATURE: &str = "fn compose_authorized_direction_delta_onto_sink( sink_xml: &str, instruction: &RendererDirectionRelocationWriterInstruction, ) -> Option<String>";

pub(crate) fn direction_owner() -> (&'static str, &'static str) {
    (DIRECTION_OWNER, DIRECTION_SIGNATURE)
}

fn validate_boundaries(text: &str) -> Result<(), String> {
    let rows = table(
        text,
        "event\tpopulation_id\towner\towner_signature\tclassification\tdomain\tpre_basis\tgeometry",
        8,
    )?;
    let expected = [
        [
            "CursorNeutral",
            "2",
            CURSOR_OWNER,
            CURSOR_SIGNATURE,
            "EXACT",
            "R",
            "source_xml before cursor-neutral Vec<TextEdit>",
            "Vec<TextEdit>",
        ],
        [
            "DirectionRelocation",
            "none",
            DIRECTION_OWNER,
            DIRECTION_SIGNATURE,
            "EXACT",
            "R",
            "last #58 committed render state",
            "matched direction removal + host insertion on sink pre coordinates",
        ],
    ];
    if rows.len() != expected.len()
        || rows
            .iter()
            .zip(expected)
            .any(|(row, expected)| row.as_slice() != expected)
    {
        return Err("K-R: only the separate cursor-neutral #2 and historical #58-based DirectionRelocation boundaries are permitted".to_string());
    }
    Ok(())
}

fn require_signature(root: &Path, owner: &str, frozen: &str) -> Result<(), String> {
    let (path, function) = owner
        .rsplit_once("::")
        .ok_or("K-R: malformed boundary owner")?;
    let source = read(root, path)?;
    let actual = exactly_one(&source, function)?;
    let migrated = frozen
        .rsplit_once("-> ")
        .map(|(head, payload)| format!("{head}-> ({payload}, Option<ExactTransformationReceipt>)"));
    if compact(actual.signature) != compact(frozen)
        && !migrated.is_some_and(|signature| compact(actual.signature) == compact(&signature))
    {
        return Err(format!(
            "K-R: boundary {owner} permits only its frozen return or one EXACT receipt"
        ));
    }
    Ok(())
}

const ORIGINS_HEADER: &str =
    "id\tkind\tfork_point\towner\treason\tsource\tsource_epoch\tlegacy_selection\thistory";
const IR_OWNER: &str = "src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs::run_initial_render_transaction_stage";
const FN_OWNER: &str = "src/pipeline_shards/pipeline_orchestrator_final_notation_support.rs::run_final_render_notation_authority_stage";
const ORIGINS: [[&str; 9]; 7] = [
    [
        "G1",
        "Genesis",
        "PrunedPdfTrial",
        IR_OWNER,
        "RenderGenesisPrunedPdf",
        "best.layout_xml()",
        "selected Master before initial Render trials",
        "pruned trial writes first Render state",
        "absent",
    ],
    [
        "G2",
        "Genesis",
        "FirstInitialTrial2To8",
        IR_OWNER,
        "RenderGenesisFirstTrial",
        "best.layout_xml()",
        "selected Master before initial Render trials",
        "first writing initial trial 2..8 after no pruned write",
        "absent",
    ],
    [
        "G3",
        "Genesis",
        "FinalNotationB1",
        FN_OWNER,
        "RenderGenesisLateFinalNotation",
        "final_export_xml",
        "Master after #56 and before #65",
        "!avail && !path.exists() && B1 writes",
        "absent",
    ],
    [
        "G4",
        "Genesis",
        "FinalNotationB2",
        FN_OWNER,
        "RenderGenesisLateFinalNotation",
        "final_export_xml",
        "Master after #56 and before #65",
        "!avail && B2 writes",
        "absent",
    ],
    [
        "G5",
        "Genesis",
        "FinalNotationB4",
        FN_OWNER,
        "RenderGenesisLateFinalNotation",
        "final_export_xml",
        "Master after #56 and before #65",
        "!avail && B4 writes",
        "absent",
    ],
    [
        "S3",
        "FromMaster",
        "FinalNotationB2",
        FN_OWNER,
        "FinalCompleteBeatWithoutRenderSource",
        "final_export_xml",
        "Master after #56 and before #65",
        "!avail && B2 writes",
        "present_preserved",
    ],
    [
        "S4",
        "FromMaster",
        "FinalNotationB4",
        FN_OWNER,
        "CursorNeutralWriterWithoutRenderSource",
        "final_export_xml",
        "Master after #56 and before #65",
        "!avail && B4 writes",
        "present_preserved",
    ],
];

fn validate_origins(text: &str) -> Result<(), String> {
    // The same origin chronology, with an explicit read-only temporal boundary.
    // The full new table is also SHA-bound by the parent retirement contract.
    let old_view = text.replace("Unchanged Master after temporal check and before #65", "Master after #56 and before #65");
    let text = old_view.as_str();

    let rows = table(text, ORIGINS_HEADER, 9)?;
    if rows.len() != ORIGINS.len()
        || rows
            .iter()
            .zip(ORIGINS)
            .any(|(row, expected)| row.as_slice() != expected)
    {
        return Err("K-R: Render origin set is closed to G1..G5/S3/S4 with their exact owner/reason/source/epoch/selection/history".to_string());
    }
    Ok(())
}

fn validate_retired_origins(text: &str) -> Result<(), String> {
    let normalized = text.replace("Unchanged Master after temporal check and before #65", "Master after #56 and before #65");
    let rows = table(&normalized, ORIGINS_HEADER, 9)?;
    let expected = ORIGINS.iter().filter(|row| !matches!(row[0], "G4" | "S3")).collect::<Vec<_>>();
    if rows.len()!=expected.len() || rows.iter().zip(expected).any(|(a,b)|a.as_slice()!=b.as_slice()) {
        return Err("K-R: retired Render origin set must be exactly G1/G2/G3/G5/S4".into());
    }
    Ok(())
}
fn validate_legacy_selection(text: &str) -> Result<(), String> {
    let rows = table(text, "rule\tcondition\tobligation\tstate_reset", 4)?;
    let expected = [
        [
            "LegacyAvailability",
            "legacy avail/legacy flags",
            "historical source selection only; never erase existing Render history",
            "none",
        ],
        [
            "WrittenTrialHistory",
            "actual written trial states including V1/V2 and IR trials 2..5 with false legacy availability",
            "retain actual Render history; B1 path.exists may read it",
            "S3/S4 only with preserved history",
        ],
        [
            "ClassF",
            "existing Render state or avail but missing state/readback",
            "fatal sovereignty error in sovereign Render model",
            "forbidden",
        ],
        [
            "AuditReaders",
            "lineage checkpoint; causal lane audit; E059 preview",
            "read-only existing Render state or explicit no Render state",
            "forbidden",
        ],
    ];
    if rows.len() != expected.len()
        || rows
            .iter()
            .zip(expected)
            .any(|(row, expected)| row.as_slice() != expected)
    {
        return Err("K-R: trial-history/class-F/audit-read contracts must not legalize another Render reset".to_string());
    }
    Ok(())
}

// Normative request contract for the following sovereign Render adoption owner.
// A file read, an audit or a legacy availability flag cannot stand in for has_lineage.
#[derive(Debug, Clone)]
pub(crate) struct OriginRequest<'a> {
    pub(crate) id: &'a str,
    pub(crate) kind: &'a str,
    pub(crate) fork_point: &'a str,
    pub(crate) owner: &'a str,
    pub(crate) reason: &'a str,
    pub(crate) source: &'a str,
    pub(crate) source_epoch: &'a str,
    pub(crate) legacy_selection: &'a str,
    pub(crate) has_lineage: bool,
    pub(crate) lineage_preserved: bool,
}

pub(crate) fn validate_origin_request(request: &OriginRequest<'_>) -> Result<(), String> {
    let Some(row) = ORIGINS.iter().find(|row| row[0] == request.id) else {
        return Err("K-R: unlisted Master-to-Render origin is forbidden".to_string());
    };
    if [
        request.id,
        request.kind,
        request.fork_point,
        request.owner,
        request.reason,
        request.source,
        request.source_epoch,
        request.legacy_selection,
    ] != row[..8]
        || request.has_lineage != (row[8] == "present_preserved")
        || (request.has_lineage && !request.lineage_preserved)
    {
        return Err("K-R: origin must retain its exact binding; no new Genesis on existing history, unlisted reset or history loss".to_string());
    }
    Ok(())
}

fn validate_legacy_bindings(root: &Path) -> Result<(), String> {
    let ir_source = read(root, IR_OWNER.rsplit_once("::").expect("owner").0)?;
    let ir = exactly_one(&ir_source, "run_initial_render_transaction_stage")?;
    for needle in [
        "build_render_xml_pruned_pdf_trial(best.layout_xml(),",
        "RenderXmlCheckpointState::new(render_xml_snapshot_reader,best.layout_xml())",
        "build_render_xml_hidden_rest_forward_collapse_trial(initial_render_checkpoint.current_xml(),",
        "build_render_xml_pianistic_rest_repair_trial(initial_render_checkpoint.current_xml(),",
        "build_render_xml_print_cleanup_one_safe_visual_trial(initial_render_checkpoint.current_xml(),",
        "build_render_xml_pianistic_missing_rest_insert_trial(initial_render_checkpoint.current_xml(),",
        "build_behind_bars_rule_check_rest_and_tie_audit(initial_render_checkpoint.current_xml(),",
        "build_cross_staff_hand_role_materialization_trial(initial_render_checkpoint.current_xml(),",
        "build_lower_staff_visible_rhythm_rest_lane_trial(initial_render_checkpoint.current_xml(),",
    ] {
        if compact(ir.body).matches(needle).count() != 1 {
            return Err(format!(
                "K-R: G1/G2 legacy origin binding changed: {needle}"
            ));
        }
    }
    let fn_source = read(root, FN_OWNER.rsplit_once("::").expect("owner").0)?;
    let final_notation = exactly_one(&fn_source, "run_final_render_notation_authority_stage")?;
    let body = compact(final_notation.body);
    for needle in [
        "letlower_clef_pair_prewrite_xml=render_xml_snapshot_reader.select_after_write(final_render_source_available||render_xml_trial_path.exists(),final_export_xml,)",
        "letbeam_source_xml=render_xml_snapshot_reader.select_after_write(final_render_source_available,final_export_xml,)",
        "letstructural_authority_source_xml=render_xml_snapshot_reader.select_after_write(final_render_source_available,final_export_xml,)",
    ] {
        if body.matches(needle).count() != 1 {
            return Err(format!(
                "K-R: late Genesis/substitution legacy source binding changed: {needle}"
            ));
        }
    }
    let direction_source = read(root, "src/structural_write_authority/mod.rs")?;
    let direction = exactly_one(
        &direction_source,
        "execute_authorized_cursor_neutral_writer_candidate",
    )?;
    if !code(direction.body).contains("letproductive_note_beam_sink_xml=orchestrator_artifact_sink_authority::current_thread_render_checkpoint_xml().unwrap_or_default();") {
        return Err("K-R: DirectionRelocation must retain the last #58 committed checkpoint basis".to_string());
    }
    Ok(())
}

pub(crate) fn validate_current_tree(root: &Path) -> Result<Row67State, String> {
    relocation(&read(root, RELOCATION_PATH)?)?;
    validate_boundaries(&read(root, BOUNDARIES_PATH)?)?;
    let writer_retired=crate::take_change_control::ssc_evidence_proof_shadow_permit_kodex::k2::checkpoint_prepare::writer_retirement_state(root)?;
    if writer_retired {
        validate_retired_origins(&read(root, ORIGINS_PATH)?)?;
        // Exact successor bytes are tuple-bound above. Reuse the unchanged
        // legacy-history validator for the surviving S4 rule only.
        let selection=read(root, SELECTION_PATH)?.replace("S4 only with preserved history","S3/S4 only with preserved history");
        validate_legacy_selection(&selection)?;
    } else {
        validate_origins(&read(root, ORIGINS_PATH)?)?;
        validate_legacy_selection(&read(root, SELECTION_PATH)?)?;
    }
    validate_population_contract(&read(root, POPULATION_PATH)?, crate::take_change_control::ssc_evidence_proof_shadow_permit_kodex::k2::installed(root))?;
    require_signature(root, CURSOR_OWNER, CURSOR_SIGNATURE)?;
    require_signature(root, DIRECTION_OWNER, DIRECTION_SIGNATURE)?;
    let state = validate_row67_source(&read(root, LEGACY_PATH)?)?;
    validate_target_uses(root, state)?;
    if read(root, POPULATION_PATH)?.lines().any(|line| { let row=line.split('\t').collect::<Vec<_>>(); row.len()==11 && row[0]=="67" && row[9]=="READY" })
        && state != Row67State::Relocated { return Err("K-R: READY requires Relocated".into()); }
    let identity = root_metadata(root)?;
    if state == Row67State::Legacy {
        if value(&identity, "name")? == REFACTOR_NAME {
            return Err(
                "K-R: Boundary Normalization refactor is missing its required relocated #67 target"
                    .to_string(),
            );
        }
        let population = read(root, POPULATION_PATH)?;
        let frozen = population
            .lines()
            .find_map(|line| {
                let columns = line.split('\t').collect::<Vec<_>>();
                (columns.first() == Some(&"67")).then(|| columns[2])
            })
            .ok_or("K-R: missing Legacy #67 signature")?;
        let source = read(root, LEGACY_PATH)?;
        if compact(exactly_one(&source, LEGACY_FUNCTION)?.signature) != compact(frozen) {
            return Err("K-R: Legacy #67 wrapper cannot mint an aggregate receipt or change its frozen signature".to_string());
        }
        validate_legacy_bindings(root)?;
    }
    // Keep the request validator live as governance: independently validate every
    // admitted binding instead of granting a generic FromMaster capability.
    for row in ORIGINS.into_iter().filter(|row|!writer_retired || !matches!(row[0],"G4"|"S3")) {
        validate_origin_request(&OriginRequest {
            id: row[0],
            kind: row[1],
            fork_point: row[2],
            owner: row[3],
            reason: row[4],
            source: row[5],
            source_epoch: row[6],
            legacy_selection: row[7],
            has_lineage: row[8] == "present_preserved",
            lineage_preserved: true,
        })?;
    }
    Ok(state)
}

pub(crate) fn validate_transition(
    current_root: &Path,
    incoming_root: &Path,
    change_class: &str,
) -> Result<(), String> {
    if !current_root.join(RELOCATION_PATH).is_file() {
        // K-R introduces governance on unchanged r1219 product bytes. Its installed
        // build runs validate_current_tree through the population freeze owner.
        return Ok(());
    }
    for path in [
        RELOCATION_PATH,
        BOUNDARIES_PATH,
        ORIGINS_PATH,
        SELECTION_PATH,
        BUDGET_PATH,
        POPULATION_PATH,
    ] {
        if read(current_root, path)? != read(incoming_root, path)? && change_class == "RefactorOnly"
        {
            return Err(format!(
                "K-R: future RefactorOnly must not change frozen contract/budget {path}"
            ));
        }
    }
    // Contract v4 B4.2: only the parent's #67 state is derived; the accepted parent is
    // not re-judged by the child's rules.
    let before = validate_row67_source(&read(current_root, LEGACY_PATH)?)?;
    let after = validate_current_tree(incoming_root)?;
    if before == Row67State::Relocated && after != before {
        return Err("K-R: relocated #67 cannot revert to the Legacy owner".to_string());
    }
    if before == Row67State::Legacy {
        return Err("K-R: #67 relocation epoch is closed; a Legacy parent is older than the epoch base".to_string());
    }
    Ok(())
}

#[cfg(test)]
#[path = "render_sovereignty_kodex_tests.rs"]
mod tests;
