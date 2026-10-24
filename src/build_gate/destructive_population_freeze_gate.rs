use super::productive_write_path_scan::ProductiveWriteSurface;
use super::rust_source_scan_primitives::{
    mask_non_code, token_boundary_after, token_boundary_before,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const POPULATION_PATH: &str = "src/build_gate/destructive_transformation_population_v1.tsv";
const EXCLUSIONS_PATH: &str = "src/build_gate/destructive_transformation_exclusions_v1.tsv";
const SURFACE_BASELINE_PATH: &str =
    "src/build_gate/destructive_transformation_surface_baseline_v1.tsv";
const NESTED_OWNER_RECEIPT_TRANSPORT_PATH: &str =
    "src/build_gate/destructive_transformation_nested_owner_v1.tsv";
const ADOPTION_EXCLUSION_ID: &str = "ADOPTION_BOUNDARY_V1";

#[derive(Debug, Clone)]
struct PopulationRow {
    id: usize,
    owner: String,
    owner_signature: String,
    effect_class: String,
    identity_kind: String,
    domain: String,
    review_status: String,
    prerequisite_state: String,
}

#[derive(Debug, Clone)]
struct ExclusionRow {
    exclusion_id: String,
    owner: String,
    owner_signature: String,
    classification: String,
}

#[derive(Debug, Clone)]
struct NestedOwnerReceiptTransportRow {
    outer_id: usize,
    inner_id: usize,
    carrier_parameter: String,
    binding_path: String,
    binding_expression: String,
    order: String,
}

#[derive(Debug, Clone)]
struct SurfaceBaselineRow {
    path: String,
    function: String,
    owner_signature: String,
    max_instances: usize,
    max_xml_buffer_mutations: usize,
    max_xml_file_writes: usize,
    coverage: String,
}

#[derive(Debug, Default)]
struct CurrentSurfaceGroup {
    instances: usize,
    max_xml_buffer_mutations: usize,
    max_xml_file_writes: usize,
    direct_from_real_main: bool,
}

pub(super) fn validate_destructive_population_freeze(
    manifest_dir: &Path,
    surfaces: &[ProductiveWriteSurface],
) -> Result<(), String> {
    crate::take_change_control::receipt_surface_normalization_kodex::validate_current_tree(
        manifest_dir,
    )?;
    let population_text = fs::read_to_string(manifest_dir.join(POPULATION_PATH))
        .map_err(|error| format!("POPULATION FREEZE: cannot read {POPULATION_PATH}: {error}"))?;
    let exclusions_text = fs::read_to_string(manifest_dir.join(EXCLUSIONS_PATH))
        .map_err(|error| format!("POPULATION FREEZE: cannot read {EXCLUSIONS_PATH}: {error}"))?;
    let surface_baseline_text = fs::read_to_string(manifest_dir.join(SURFACE_BASELINE_PATH))
        .map_err(|error| {
            format!("POPULATION FREEZE: cannot read {SURFACE_BASELINE_PATH}: {error}")
        })?;
    let nested_owner_receipt_transport_text = fs::read_to_string(
        manifest_dir.join(NESTED_OWNER_RECEIPT_TRANSPORT_PATH),
    )
    .map_err(|error| {
        format!("POPULATION FREEZE: cannot read {NESTED_OWNER_RECEIPT_TRANSPORT_PATH}: {error}")
    })?;

    let population = parse_population(&population_text)?;
    let render_row67_state =
        super::take_change_control::render_sovereignty_kodex::validate_current_tree(manifest_dir)?;
    validate_population_shape_with_positive_k2(manifest_dir, &population, render_row67_state)?;
    let nested_owner_receipt_transports =
        parse_nested_owner_receipt_transports(&nested_owner_receipt_transport_text)?;
    validate_nested_owner_receipt_transports(
        manifest_dir,
        &population,
        &nested_owner_receipt_transports,
    )?;
    validate_current_owner_signatures(
        manifest_dir,
        &population,
        &nested_owner_receipt_transports,
        render_row67_state,
    )?;

    let exclusions = parse_exclusions(&exclusions_text)?;
    validate_exclusion_signatures(manifest_dir, &exclusions)?;
    if !exclusions
        .iter()
        .any(|row| row.exclusion_id == ADOPTION_EXCLUSION_ID)
    {
        return Err(format!(
            "POPULATION FREEZE: {EXCLUSIONS_PATH} is missing {ADOPTION_EXCLUSION_ID}"
        ));
    }

    let surface_baseline = parse_surface_baseline(&surface_baseline_text, super::take_change_control::ssc_evidence_proof_shadow_permit_kodex::k2::checkpoint_prepare::writer_retirement_state(manifest_dir)?)?;
    validate_baseline_references(&surface_baseline, &population, &exclusions)?;
    validate_current_surfaces(
        manifest_dir,
        surfaces,
        &surface_baseline,
        &population,
        &exclusions,
        &nested_owner_receipt_transports,
    )?;

    println!(
        "cargo:rustc-env=ETUSCHLOPF_DESTRUCTIVE_POPULATION_ROWS={}",
        population.len()
    );
    println!(
        "cargo:rustc-env=ETUSCHLOPF_DESTRUCTIVE_POPULATION_EXCLUSIONS={}",
        exclusions.len()
    );
    println!(
        "cargo:rustc-env=ETUSCHLOPF_DESTRUCTIVE_POPULATION_SURFACE_BASELINE={}",
        surface_baseline.len()
    );
    println!(
        "cargo:rustc-env=ETUSCHLOPF_DESTRUCTIVE_NESTED_OWNER_RECEIPT_TRANSPORTS={}",
        nested_owner_receipt_transports.len()
    );
    println!("cargo:rustc-env=ETUSCHLOPF_DESTRUCTIVE_POPULATION_FREEZE_GREEN=1");
    Ok(())
}

fn parse_population(text: &str) -> Result<Vec<PopulationRow>, String> {
    const HEADER: &str = "id\towner\towner_signature\teffect_class\tidentity_kind\tdomain\tedit_source\tadoption_archetype\treview_status\tprerequisite_state\tnote";
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err("POPULATION FREEZE: population TSV header changed".to_string());
    }
    let mut rows = Vec::new();
    for (index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let columns = line.split('\t').collect::<Vec<_>>();
        if columns.len() != 11 {
            return Err(format!(
                "POPULATION FREEZE: population line {} has {} columns, expected 11",
                index + 2,
                columns.len()
            ));
        }
        let id = columns[0].parse::<usize>().map_err(|error| {
            format!(
                "POPULATION FREEZE: population line {} has invalid id: {error}",
                index + 2
            )
        })?;
        for (column_index, value) in columns.iter().enumerate().take(10) {
            if value.trim().is_empty() {
                return Err(format!(
                    "POPULATION FREEZE: population line {} column {} is empty",
                    index + 2,
                    column_index + 1
                ));
            }
        }
        rows.push(PopulationRow {
            id,
            owner: columns[1].to_string(),
            owner_signature: columns[2].to_string(),
            effect_class: columns[3].to_string(),
            identity_kind: columns[4].to_string(),
            domain: columns[5].to_string(),
            review_status: columns[8].to_string(),
            prerequisite_state: columns[9].to_string(),
        });
    }
    Ok(rows)
}

fn validate_population_shape_with_positive_k2(
    root: &Path,
    rows: &[PopulationRow],
    row67: crate::take_change_control::render_sovereignty_kodex::Row67State,
) -> Result<(), String> {
    let mut projected = rows.to_vec();
    if let Some(row) = projected.iter_mut().find(|row| row.id == 57)
        && row.prerequisite_state == "READY"
    {
        // A READY status alone never changes the historical shape contract.
        if !crate::take_change_control::ssc_evidence_proof_shadow_permit_kodex::k2::installed(root) {
            return Err("POPULATION FREEZE: row 57 READY requires installed positive K2".into());
        }
        let installed = parse_population(&fs::read_to_string(root.join(POPULATION_PATH))
            .map_err(|error| format!("POPULATION FREEZE: {error}"))?)?;
        if !installed.iter().any(|row| row.id == 57 && row.prerequisite_state == "READY") {
            return Err("POPULATION FREEZE: row 57 READY lacks matching installed population".into());
        }
        crate::take_change_control::ssc_evidence_proof_shadow_permit_kodex::k2::validate_tree(root)?;
        row.prerequisite_state = "DEFERRED_RENDER_PREREQUISITE".into();
    }
    validate_population_shape_in_context(&projected,
        crate::take_change_control::ssc_evidence_proof_shadow_permit_kodex::k2::installed(root), row67)
}

fn validate_population_shape_in_context(
    rows: &[PopulationRow], k2_installed: bool,
    row67: crate::take_change_control::render_sovereignty_kodex::Row67State,
) -> Result<(), String> {
    let mut projected = rows.to_vec();
    if let Some(row) = projected.iter_mut().find(|row| row.id == 67)
        && row.prerequisite_state == "READY"
    {
        if !k2_installed || row67 != crate::take_change_control::render_sovereignty_kodex::Row67State::Relocated {
            return Err("POPULATION FREEZE: row 67 READY requires installed K2 and full K-R Relocated proof".into());
        }
        // Only the historical shape view is projected; no population data changes.
        row.prerequisite_state = "DEFERRED_RENDER_PREREQUISITE".into();
    }
    validate_population_shape(&projected)
}

fn validate_population_shape(rows: &[PopulationRow]) -> Result<(), String> {
    if rows.len() != 69 {
        return Err(format!(
            "POPULATION FREEZE: expected 69 population rows, got {}",
            rows.len()
        ));
    }
    let ids = rows.iter().map(|row| row.id).collect::<BTreeSet<_>>();
    let expected = (1_usize..=69).collect::<BTreeSet<_>>();
    if ids != expected {
        return Err("POPULATION FREEZE: population ids are not exactly 1..=69".to_string());
    }

    let destructive = rows
        .iter()
        .filter(|row| row.effect_class == "DESTRUCTIVE")
        .count();
    let unknown = rows
        .iter()
        .filter(|row| row.effect_class == "UNKNOWN")
        .count();
    if destructive != 62 || unknown != 7 {
        return Err(format!(
            "POPULATION FREEZE: effect distribution changed: DESTRUCTIVE {destructive}, UNKNOWN {unknown}"
        ));
    }

    let exact = rows
        .iter()
        .filter(|row| row.identity_kind == "EXACT")
        .count();
    let block_note = rows
        .iter()
        .filter(|row| row.identity_kind == "BLOCK_NOTE")
        .count();
    let block_measure = rows
        .iter()
        .filter(|row| row.identity_kind == "BLOCK_MEASURE")
        .count();
    let block_document = rows
        .iter()
        .filter(|row| row.identity_kind == "BLOCK_DOCUMENT")
        .count();
    let block_generated = rows
        .iter()
        .filter(|row| row.identity_kind == "BLOCK_GENERATED")
        .count();
    let genesis = rows
        .iter()
        .filter(|row| row.identity_kind == "GENESIS")
        .count();
    if (
        exact,
        block_note,
        block_measure,
        block_document,
        block_generated,
        genesis,
    ) != (43, 12, 5, 6, 1, 2)
    {
        return Err(format!(
            "POPULATION FREEZE: identity distribution changed: EXACT {exact}, BLOCK_NOTE {block_note}, BLOCK_MEASURE {block_measure}, BLOCK_DOCUMENT {block_document}, BLOCK_GENERATED {block_generated}, GENESIS {genesis}"
        ));
    }

    let master = rows.iter().filter(|row| row.domain == "M").count();
    let render = rows.iter().filter(|row| row.domain == "R").count();
    let both = rows.iter().filter(|row| row.domain == "B").count();
    if (master, render, both) != (25, 33, 11) {
        return Err(format!(
            "POPULATION FREEZE: domain distribution changed: M {master}, R {render}, B {both}"
        ));
    }

    let needs_review = rows
        .iter()
        .filter(|row| row.review_status == "NEEDS_REVIEW")
        .count();
    let opus_verified = rows
        .iter()
        .filter(|row| row.review_status == "OPUS_VERIFIED")
        .count();
    let agent_traced = rows
        .iter()
        .filter(|row| row.review_status == "AGENT_TRACED")
        .count();
    if (opus_verified, agent_traced, needs_review) != (9, 60, 0) {
        return Err(format!(
            "POPULATION FREEZE: review distribution changed: OPUS_VERIFIED {opus_verified}, AGENT_TRACED {agent_traced}, NEEDS_REVIEW {needs_review}"
        ));
    }

    for id in [57_usize, 67] {
        let row = rows.iter().find(|row| row.id == id).expect("id set is complete");
        if row.prerequisite_state != "DEFERRED_RENDER_PREREQUISITE" {
            return Err(format!(
                "POPULATION FREEZE: row {id} must remain DEFERRED_RENDER_PREREQUISITE before r1205"
            ));
        }
    }
    for id in [4_usize, 63] {
        let row = rows.iter().find(|row| row.id == id).expect("id set is complete");
        if row.prerequisite_state != "RESOLVED_R1203" {
            return Err(format!(
                "POPULATION FREEZE: row {id} must remain RESOLVED_R1203"
            ));
        }
    }
    Ok(())
}

fn parse_exclusions(text: &str) -> Result<Vec<ExclusionRow>, String> {
    const HEADER: &str = "exclusion_id\towner\towner_signature\tclassification\treason";
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err("POPULATION FREEZE: exclusion TSV header changed".to_string());
    }
    let mut rows = Vec::new();
    let mut ids = BTreeSet::new();
    for (index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let columns = line.split('\t').collect::<Vec<_>>();
        if columns.len() != 5 || columns.iter().any(|value| value.trim().is_empty()) {
            return Err(format!(
                "POPULATION FREEZE: exclusion line {} is not a complete 5-column row",
                index + 2
            ));
        }
        if !ids.insert(columns[0].to_string()) {
            return Err(format!(
                "POPULATION FREEZE: duplicate exclusion id {}",
                columns[0]
            ));
        }
        rows.push(ExclusionRow {
            exclusion_id: columns[0].to_string(),
            owner: columns[1].to_string(),
            owner_signature: columns[2].to_string(),
            classification: columns[3].to_string(),
        });
    }
    if rows.is_empty() {
        return Err("POPULATION FREEZE: exclusion table is empty".to_string());
    }
    Ok(rows)
}

fn parse_nested_owner_receipt_transports(
    text: &str,
) -> Result<Vec<NestedOwnerReceiptTransportRow>, String> {
    const HEADER: &str =
        "outer_id\tinner_id\tcarrier_parameter\tbinding_path\tbinding_expression\torder";
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err("POPULATION FREEZE: nested-owner TSV header changed".to_string());
    }
    let mut rows = Vec::new();
    for (index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let columns = line.split('\t').collect::<Vec<_>>();
        if columns.len() != 6 || columns.iter().any(|value| value.trim().is_empty()) {
            return Err(format!(
                "POPULATION FREEZE: nested-owner line {} is not a complete 6-column row",
                index + 2
            ));
        }
        let outer_id = columns[0].parse::<usize>().map_err(|error| {
            format!(
                "POPULATION FREEZE: nested-owner line {} has invalid outer id: {error}",
                index + 2
            )
        })?;
        let inner_id = columns[1].parse::<usize>().map_err(|error| {
            format!(
                "POPULATION FREEZE: nested-owner line {} has invalid inner id: {error}",
                index + 2
            )
        })?;
        rows.push(NestedOwnerReceiptTransportRow {
            outer_id,
            inner_id,
            carrier_parameter: columns[2].to_string(),
            binding_path: columns[3].to_string(),
            binding_expression: columns[4].to_string(),
            order: columns[5].to_string(),
        });
    }
    Ok(rows)
}

fn validate_nested_owner_receipt_transport_shape(
    rows: &[NestedOwnerReceiptTransportRow],
) -> Result<(), String> {
    if rows.len() != 1 {
        return Err(format!(
            "POPULATION FREEZE: nested-owner transport v1 must contain exactly one frozen pair, got {}",
            rows.len()
        ));
    }
    let row = &rows[0];
    if row.outer_id != 59
        || row.inner_id != 60
        || row.carrier_parameter
            != "functional_respelling: FunctionalMasterRespellingOperation"
        || row.binding_path
            != "src/pipeline_shards/pipeline_orchestrator_metric_final_notation_stage.rs"
        || row.binding_expression
            != "structural_write_authority::FunctionalMasterRespellingOperation::new(canonical_master_functional_respelling_from_render,)"
        || row.order != "outer_then_inner"
    {
        return Err(
            "POPULATION FREEZE: nested-owner transport v1 is frozen exclusively to population 59 -> 60 via FunctionalMasterRespellingOperation"
                .to_string(),
        );
    }
    Ok(())
}

fn validate_nested_owner_receipt_transports(
    manifest_dir: &Path,
    population: &[PopulationRow],
    rows: &[NestedOwnerReceiptTransportRow],
) -> Result<(), String> {
    validate_nested_owner_receipt_transport_shape(rows)?;
    let row = &rows[0];
    let outer = population
        .iter()
        .find(|candidate| candidate.id == row.outer_id)
        .ok_or_else(|| {
            format!(
                "POPULATION FREEZE: nested-owner outer population {} is missing",
                row.outer_id
            )
        })?;
    let inner = population
        .iter()
        .find(|candidate| candidate.id == row.inner_id)
        .ok_or_else(|| {
            format!(
                "POPULATION FREEZE: nested-owner inner population {} is missing",
                row.inner_id
            )
        })?;
    if outer.identity_kind != "EXACT"
        || inner.identity_kind != "EXACT"
        || outer.domain != "M"
        || inner.domain != "M"
    {
        return Err(
            "POPULATION FREEZE: nested-owner 59 -> 60 must remain EXACT/M on both frozen population rows"
                .to_string(),
        );
    }
    if !outer.owner_signature.contains(&row.carrier_parameter) {
        return Err(format!(
            "POPULATION FREEZE: nested-owner outer signature lost frozen carrier parameter {:?}",
            row.carrier_parameter
        ));
    }
    let binding_source_path = manifest_dir.join(&row.binding_path);
    let binding_source = fs::read_to_string(&binding_source_path).map_err(|error| {
        format!(
            "POPULATION FREEZE: cannot read nested-owner binding source {}: {error}",
            binding_source_path.display()
        )
    })?;
    let binding_compact = binding_source.split_whitespace().collect::<String>();
    if binding_compact.matches(&row.binding_expression).count() != 1 {
        return Err(format!(
            "POPULATION FREEZE: nested-owner 59 -> 60 requires exactly one frozen carrier construction {:?} in {}",
            row.binding_expression, row.binding_path
        ));
    }
    let inner_function = inner
        .owner
        .rsplit_once("::")
        .map(|(_, function)| function)
        .ok_or_else(|| {
            format!(
                "POPULATION FREEZE: malformed nested-owner inner owner {:?}",
                inner.owner
            )
        })?;
    if !row.binding_expression.contains(inner_function) {
        return Err(
            "POPULATION FREEZE: nested-owner carrier construction is not bound to the frozen inner owner"
                .to_string(),
        );
    }
    let (outer_path, _) = outer.owner.rsplit_once("::").ok_or_else(|| {
        format!(
            "POPULATION FREEZE: malformed nested-owner outer owner {:?}",
            outer.owner
        )
    })?;
    let outer_source_path = manifest_dir.join(outer_path);
    let outer_source = fs::read_to_string(&outer_source_path).map_err(|error| {
        format!(
            "POPULATION FREEZE: cannot read nested-owner outer source {}: {error}",
            outer_source_path.display()
        )
    })?;
    let outer_compact = outer_source.split_whitespace().collect::<String>();
    let carrier_name = row
        .carrier_parameter
        .split_once(':')
        .map(|(name, _)| name.trim())
        .ok_or_else(|| {
            "POPULATION FREEZE: nested-owner carrier parameter must be name: type".to_string()
        })?;
    let apply_needle = format!("{carrier_name}.apply(");
    if outer_compact.matches(&apply_needle).count() != 1 {
        return Err(format!(
            "POPULATION FREEZE: nested-owner outer owner must consume its frozen carrier exactly once via {apply_needle:?}"
        ));
    }
    Ok(())
}

fn parse_surface_baseline(text: &str, writer_retired: bool) -> Result<Vec<SurfaceBaselineRow>, String> {
    const HEADER: &str = "path\tfunction\towner_signature\tmax_instances\tmax_xml_buffer_mutations\tmax_xml_file_writes\tcoverage";
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err("POPULATION FREEZE: surface baseline TSV header changed".to_string());
    }
    let mut rows = Vec::new();
    let mut keys = BTreeSet::new();
    for (index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let columns = line.split('\t').collect::<Vec<_>>();
        if columns.len() != 7 || columns.iter().any(|value| value.trim().is_empty()) {
            return Err(format!(
                "POPULATION FREEZE: surface baseline line {} is not a complete 7-column row",
                index + 2
            ));
        }
        let max_instances = columns[3].parse::<usize>().map_err(|error| {
            format!(
                "POPULATION FREEZE: surface baseline line {} invalid instance budget: {error}",
                index + 2
            )
        })?;
        let max_xml_buffer_mutations = columns[4].parse::<usize>().map_err(|error| {
            format!(
                "POPULATION FREEZE: surface baseline line {} invalid buffer budget: {error}",
                index + 2
            )
        })?;
        let max_xml_file_writes = columns[5].parse::<usize>().map_err(|error| {
            format!(
                "POPULATION FREEZE: surface baseline line {} invalid file budget: {error}",
                index + 2
            )
        })?;
        if max_instances == 0 || (max_xml_buffer_mutations == 0 && max_xml_file_writes == 0) {
            return Err(format!(
                "POPULATION FREEZE: surface baseline line {} has an empty budget",
                index + 2
            ));
        }
        let key = format!("{}::{}|{}", columns[0], columns[1], columns[2]);
        if !keys.insert(key.clone()) {
            return Err(format!("POPULATION FREEZE: duplicate surface baseline key {key}"));
        }
        rows.push(SurfaceBaselineRow {
            path: columns[0].to_string(),
            function: columns[1].to_string(),
            owner_signature: columns[2].to_string(),
            max_instances,
            max_xml_buffer_mutations,
            max_xml_file_writes,
            coverage: columns[6].to_string(),
        });
    }
    let expected_count = if writer_retired { 52 } else { 55 };
    if rows.len() != expected_count {
        return Err(format!(
            "POPULATION FREEZE: expected {expected_count} frozen physical surface groups, got {}",
            rows.len()
        ));
    }
    Ok(rows)
}

fn validate_baseline_references(
    baseline: &[SurfaceBaselineRow],
    population: &[PopulationRow],
    exclusions: &[ExclusionRow],
) -> Result<(), String> {
    let population_ids = population.iter().map(|row| row.id).collect::<BTreeSet<_>>();
    let exclusion_ids = exclusions
        .iter()
        .map(|row| row.exclusion_id.as_str())
        .collect::<BTreeSet<_>>();
    for row in baseline {
        if let Some(value) = row.coverage.strip_prefix("population:") {
            let id = value.parse::<usize>().map_err(|error| {
                format!(
                    "POPULATION FREEZE: invalid population reference {:?}: {error}",
                    row.coverage
                )
            })?;
            if !population_ids.contains(&id) {
                return Err(format!(
                    "POPULATION FREEZE: physical surface {}::{} references missing population row {id}",
                    row.path, row.function
                ));
            }
        } else if let Some(value) = row.coverage.strip_prefix("exclusion:") {
            if !exclusion_ids.contains(value) {
                return Err(format!(
                    "POPULATION FREEZE: physical surface {}::{} references missing exclusion {value}",
                    row.path, row.function
                ));
            }
        } else {
            return Err(format!(
                "POPULATION FREEZE: physical surface {}::{} has invalid coverage {:?}",
                row.path, row.function, row.coverage
            ));
        }
    }
    Ok(())
}

fn validate_receipt_only_signature_migration(
    frozen_signature: &str,
    migrated_signature: &str,
    identity_kind: &str,
) -> Result<(), String> {
    if frozen_signature.starts_with("macro:fn ") {
        return Err(
            "macro-generated owners are not eligible for receipt-signature migration".to_string(),
        );
    }
    let frozen = split_function_signature(frozen_signature)?;
    let migrated = split_function_signature(migrated_signature)?;
    if frozen.head != migrated.head {
        return Err("function name, generics or parameters changed".to_string());
    }
    let receipt_types: &[&str] = if identity_kind == "EXACT" {
        &["ExactTransformationReceipt"]
    } else if identity_kind.starts_with("BLOCK_") {
        &["BlockTransformationReceipt"]
    } else {
        return Err(format!(
            "identity kind {identity_kind:?} is not eligible for transformation-receipt return migration"
        ));
    };
    let expected_returns = receipt_types
        .iter()
        .map(|receipt_type| {
            if frozen.return_type == "()" {
                format!("Option<{receipt_type}>")
            } else {
                format!("({}, Option<{receipt_type}>)", frozen.return_type)
            }
        })
        .collect::<Vec<_>>();
    if !expected_returns.iter().any(|expected| migrated.return_type == expected) {
        return Err(format!(
            "return type must change only from {:?} to one of {:?}, got {:?}",
            frozen.return_type, expected_returns, migrated.return_type
        ));
    }
    Ok(())
}

// One closed, immutable original-chain input for the existing EXACT owner #60.
// Runtime evidence construction and validation remain a later implementation.
const RESPELLING_CHAIN_OWNER: &str = "src/pipeline_shards/key_signature_enharmonic_report_core.rs::canonical_master_functional_respelling_from_render";
const RESPELLING_CHAIN_FROZEN_SIGNATURE: &str = "fn canonical_master_functional_respelling_from_render( master_xml: &str, render_xml: &str, ) -> Result<String, String>";
const RESPELLING_CHAIN_INPUT_SIGNATURE: &str = "fn canonical_master_functional_respelling_from_render( master_xml: &str, render_xml: &str, original_chains: &crate::master_state_identity::OriginalFunctionalRespellingChains, ) -> (Result<String, String>, Option<ExactTransformationReceipt>)";

fn validate_respelling_chain_input_signature(
    owner: &str, frozen: &str, migrated: &str, identity_kind: &str,
) -> Result<(), String> {
    if owner != RESPELLING_CHAIN_OWNER || identity_kind != "EXACT"
        || frozen != RESPELLING_CHAIN_FROZEN_SIGNATURE
        || migrated != RESPELLING_CHAIN_INPUT_SIGNATURE {
        return Err("original-chain input is restricted to the exact #60 owner, immutable typed input and original single EXACT receipt return".into());
    }
    Ok(())
}

// E053 may carry its original receipt plus its own read-only observation.
// No other owner or signature can consume this closed migration.
const BEAM_COMPANION_OWNER: &str = "src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::enforce_beat_scoped_beaming_and_rest_completion_in_xml";
const BEAM_COMPANION_FIXTURE_PATH: &str = "src/build_gate/actual_beam_receipt_companion_v1.txt";
const BEAM_COMPANION_FIXTURE: &str = include_str!("actual_beam_receipt_companion_v1.txt");

fn validate_beam_companion_signature(owner: &str, frozen: &str, migrated: &str,
    identity_kind: &str) -> Result<(), String> {
    if owner != BEAM_COMPANION_OWNER || identity_kind != "BLOCK_DOCUMENT" {
        return Err("beam observation companion is restricted to E053 BLOCK_DOCUMENT".into());
    }
    let frozen = split_function_signature(frozen)?;
    let migrated = split_function_signature(migrated)?;
    if frozen.head != migrated.head || frozen.return_type != "()"
        || migrated.return_type != "crate::master_state_identity::CapturedBlockReceiptInput" {
        return Err("beam companion changes only the E053 return type".into());
    }
    Ok(())
}

fn validate_beam_companion_carrier(source: &str) -> Result<(), String> {
    let code = mask_non_code(source).split_whitespace().collect::<String>();
    let expected = mask_non_code(BEAM_COMPANION_FIXTURE).split_whitespace().collect::<String>();
    let start = code.find(&expected).ok_or_else(|| "missing actual beam companion definition".to_string())?;
    let prefix = &code[..start];
    let depth = prefix.chars().fold(0isize, |depth, token| match token {
        '{' => depth + 1, '}' => depth - 1, _ => depth,
    });
    // The definition must be the actual root item, with no enclosing macro,
    // nested decoy module or outer conditional-compilation attribute.
    if depth != 0 || prefix.chars().last().is_some_and(|token| token != '}' && token != ';')
        || code.matches("structCapturedBlockReceiptInput").count() != 1
        || code.matches("implCapturedBlockReceiptInput").count() != 1
        || code.matches("implFrom<Option<BlockTransformationReceipt>>forCapturedBlockReceiptInput").count() != 1 {
        return Err("beam companion must retain the closed original-receipt carrier and its original-event binding".into());
    }
    Ok(())
}

fn admits_beam_companion(manifest_dir: &Path, owner: &str, frozen: &str,
    migrated: &str, identity_kind: &str) -> Result<(), String> {
    validate_beam_companion_signature(owner, frozen, migrated, identity_kind)?;
    let fixture = fs::read_to_string(manifest_dir.join(BEAM_COMPANION_FIXTURE_PATH))
        .map_err(|error| format!("beam companion fixture unreadable: {error}"))?;
    if fixture != BEAM_COMPANION_FIXTURE {
        return Err("beam companion contract differs from the compiled parent".into());
    }
    let source = fs::read_to_string(manifest_dir.join("src/master_state_identity.rs"))
        .map_err(|error| format!("beam companion carrier unreadable: {error}"))?;
    validate_beam_companion_carrier(&source)
}

fn neutral_receipt_type_for_identity(identity_kind: &str) -> Result<&'static str, String> {
    if identity_kind == "EXACT" {
        Ok("ExactTransformationReceipt")
    } else if identity_kind.starts_with("BLOCK_") {
        Ok("BlockTransformationReceipt")
    } else {
        Err(format!(
            "identity kind {identity_kind:?} is not eligible for nested frozen-owner receipt transport"
        ))
    }
}

fn validate_nested_owner_receipt_transport_signature_migration(
    frozen_signature: &str,
    migrated_signature: &str,
    outer_identity_kind: &str,
    inner_identity_kind: &str,
) -> Result<(), String> {
    if frozen_signature.starts_with("macro:fn ") {
        return Err(
            "macro-generated owners are not eligible for nested frozen-owner receipt transport"
                .to_string(),
        );
    }
    let frozen = split_function_signature(frozen_signature)?;
    let migrated = split_function_signature(migrated_signature)?;
    if frozen.head != migrated.head {
        return Err("function name, generics or parameters changed".to_string());
    }
    let outer_receipt = neutral_receipt_type_for_identity(outer_identity_kind)?;
    let inner_receipt = neutral_receipt_type_for_identity(inner_identity_kind)?;
    let expected_return = if frozen.return_type == "()" {
        format!("(Option<{outer_receipt}>, Option<{inner_receipt}>)")
    } else {
        format!(
            "({}, Option<{outer_receipt}>, Option<{inner_receipt}>)",
            frozen.return_type
        )
    };
    if migrated.return_type != expected_return {
        return Err(format!(
            "nested frozen-owner return type must change only from {:?} to {:?}, got {:?}",
            frozen.return_type, expected_return, migrated.return_type
        ));
    }
    Ok(())
}

fn validate_exclusion_receipt_only_signature_migration(
    frozen_signature: &str,
    migrated_signature: &str,
    classification: &str,
) -> Result<(), String> {
    if frozen_signature.starts_with("macro:fn ") {
        return Err(
            "macro-generated exclusions are not eligible for receipt-signature migration"
                .to_string(),
        );
    }
    let frozen = split_function_signature(frozen_signature)?;
    let migrated = split_function_signature(migrated_signature)?;
    if frozen.head != migrated.head {
        return Err("function name, generics or parameters changed".to_string());
    }
    let receipt_types: &[&str] = match classification {
        "PRESERVING" => &["PreservingTransformationReceipt"],
        "ADDITIVE" => &["AdditiveTransformationReceipt"],
        other => {
            return Err(format!(
                "exclusion classification {other:?} is not eligible for transformation-receipt return migration"
            ));
        }
    };
    let expected_returns = receipt_types
        .iter()
        .map(|receipt_type| {
            if frozen.return_type == "()" {
                format!("Option<{receipt_type}>")
            } else {
                format!("({}, Option<{receipt_type}>)", frozen.return_type)
            }
        })
        .collect::<Vec<_>>();
    if !expected_returns.iter().any(|expected| migrated.return_type == expected) {
        return Err(format!(
            "return type must change only from {:?} to one of {:?}, got {:?}",
            frozen.return_type, expected_returns, migrated.return_type
        ));
    }
    Ok(())
}

struct ParsedFunctionSignature<'a> {
    head: &'a str,
    return_type: &'a str,
}

fn split_function_signature(signature: &str) -> Result<ParsedFunctionSignature<'_>, String> {
    let signature = signature.trim();
    let open = signature
        .find('(')
        .ok_or_else(|| format!("missing parameter list in signature {signature:?}"))?;
    let bytes = signature.as_bytes();
    let mut depth = 0usize;
    let mut close = None;
    for (offset, byte) in bytes[open..].iter().enumerate() {
        match *byte {
            b'(' => depth += 1,
            b')' => {
                depth = depth.checked_sub(1).ok_or_else(|| {
                    format!("unbalanced parameter list in signature {signature:?}")
                })?;
                if depth == 0 {
                    close = Some(open + offset);
                    break;
                }
            }
            _ => {}
        }
    }
    let close = close
        .ok_or_else(|| format!("unterminated parameter list in signature {signature:?}"))?;
    let head = signature[..=close].trim();
    let suffix = signature[close + 1..].trim();
    let return_type = if suffix.is_empty() {
        "()"
    } else {
        suffix
            .strip_prefix("-> ")
            .or_else(|| suffix.strip_prefix("->"))
            .map(str::trim)
            .ok_or_else(|| format!("unexpected signature suffix {suffix:?}"))?
    };
    Ok(ParsedFunctionSignature { head, return_type })
}

fn validate_current_owner_signatures(
    manifest_dir: &Path,
    rows: &[PopulationRow],
    nested_owner_receipt_transports: &[NestedOwnerReceiptTransportRow],
    render_row67_state: super::take_change_control::render_sovereignty_kodex::Row67State,
) -> Result<(), String> {
    for row in rows {
        if row.id == 67
            && render_row67_state
                == super::take_change_control::render_sovereignty_kodex::Row67State::Relocated
        {
            // The immutable K-R overlay now owns #67's physical anchor. Its source,
            // exact body, signature, unique guarded caller and budgets were checked
            // together by validate_current_tree; the old wrapper is an orchestrator.
            let (path, function) =
                super::take_change_control::render_sovereignty_kodex::effective_row67_owner(
                    render_row67_state,
                );
            validate_owner_signature_if_present(
                manifest_dir,
                &format!("{path}::{function}"),
                super::take_change_control::render_sovereignty_kodex::TARGET_SIGNATURE,
                Some("EXACT"),
                None,
                "relocated population #67",
            )?;
            continue;
        }
        let nested_inner_identity_kind = nested_owner_receipt_transports
            .iter()
            .find(|contract| contract.outer_id == row.id)
            .and_then(|contract| {
                rows.iter()
                    .find(|candidate| candidate.id == contract.inner_id)
                    .map(|inner| inner.identity_kind.as_str())
            });
        validate_owner_signature_if_present(
            manifest_dir,
            &row.owner,
            &row.owner_signature,
            (row.id != 67).then_some(row.identity_kind.as_str()),
            nested_inner_identity_kind,
            "population",
        )?;
    }
    let (owner, signature) =
        super::take_change_control::render_sovereignty_kodex::direction_owner();
    validate_owner_signature_if_present(
        manifest_dir,
        owner,
        signature,
        Some("EXACT"),
        None,
        "DirectionRelocation event",
    )?;
    Ok(())
}

fn validate_exclusion_signatures(manifest_dir: &Path, rows: &[ExclusionRow]) -> Result<(), String> {
    for row in rows {
        if row.exclusion_id == ADOPTION_EXCLUSION_ID {
            if row.classification != "ADOPTION_CONSOLIDATION_BOUNDARY" {
                return Err(format!(
                    "POPULATION FREEZE: {ADOPTION_EXCLUSION_ID} classification changed"
                ));
            }
            continue;
        }
        validate_exclusion_owner_signature_if_present(manifest_dir, row)?;
    }
    Ok(())
}

// One closed read-only input migration for the existing beam-only E050 owner.
// The exclusion, owner, original inputs and single preserving receipt stay fixed.
// A second exact shape permits one typed writer-authored extension companion.
// Interface admission neither constructs evidence nor grants a music decision.
const BRIDGE_VOICE_INPUT_OWNER: &str = "src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::apply_structural_rest_bridge_beam_variant_a_render_write";
const BRIDGE_VOICE_INPUT_FROZEN: &str = "fn apply_structural_rest_bridge_beam_variant_a_render_write( xml: &str, source_label: &str, output_path: &std::path::Path, ) -> (String, SisterFeedbackBeamVariantATinyRenderWriteAudit)";
const BRIDGE_VOICE_INPUT_MIGRATED: &str = "fn apply_structural_rest_bridge_beam_variant_a_render_write( xml: &str, source_label: &str, output_path: &std::path::Path, voice_guard: &structural_write_authority::CanonicalFinalChordStructuralPlan, ) -> ((String, SisterFeedbackBeamVariantATinyRenderWriteAudit), Option<PreservingTransformationReceipt>)";
const BRIDGE_EXTENSION_OUTPUT_MIGRATED: &str = "fn apply_structural_rest_bridge_beam_variant_a_render_write( xml: &str, source_label: &str, output_path: &std::path::Path, voice_guard: &structural_write_authority::CanonicalFinalChordStructuralPlan, ) -> ((String, SisterFeedbackBeamVariantATinyRenderWriteAudit), Option<PreservingTransformationReceipt>, Option<structural_write_authority::RestBridgeExtensionOccurrenceEvidence>)";

fn has_one_bridge_voice_input_definition(source: &str, function: &str) -> bool {
    let masked = mask_non_code(source);
    let needle = format!("fn {function}");
    let bytes = masked.as_bytes();
    masked.match_indices(&needle).filter(|(start, _)| {
        let after = start + needle.len();
        token_boundary_before(bytes, *start) && token_boundary_after(bytes, after)
            && find_function_open_brace(&masked, after).is_some()
    }).count() == 1
}

fn validate_bridge_voice_input_signature(row: &ExclusionRow, migrated: &str) -> Result<(), String> {
    if row.exclusion_id != "E050" || row.owner != BRIDGE_VOICE_INPUT_OWNER
        || row.classification != "PRESERVING" || row.owner_signature != BRIDGE_VOICE_INPUT_FROZEN
        || (migrated != BRIDGE_VOICE_INPUT_MIGRATED
            && migrated != BRIDGE_EXTENSION_OUTPUT_MIGRATED)
    {
        return Err("bridge interface is restricted to exact E050, unchanged original parameters, one shared canonical-plan borrow, one preserving receipt and at most the exact typed extension companion".into());
    }
    Ok(())
}

fn validate_exclusion_owner_signature_if_present(
    manifest_dir: &Path,
    row: &ExclusionRow,
) -> Result<(), String> {
    let Some((path, function)) = row.owner.rsplit_once("::") else {
        return Err(format!(
            "POPULATION FREEZE: malformed exclusion owner {:?}",
            row.owner
        ));
    };
    let source_path = manifest_dir.join(path);
    if !source_path.exists() {
        return Ok(());
    }
    let source = fs::read_to_string(&source_path).map_err(|error| {
        format!(
            "POPULATION FREEZE: cannot read exclusion owner source {}: {error}",
            source_path.display()
        )
    })?;

    if row.owner_signature.starts_with("macro:fn ") {
        let compact = source.split_whitespace().collect::<String>();
        let needle = format!("define_replacement_authority!({function},");
        if compact.contains(&needle) {
            return Ok(());
        }
        if !compact.contains(function) {
            return Ok(());
        }
        return Err(format!(
            "POPULATION FREEZE: exclusion macro owner {} still exists but no longer matches its frozen macro signature",
            row.owner
        ));
    }

    let signatures = function_signatures(&source, function);
    if signatures.is_empty() {
        return Ok(());
    }
    if signatures.iter().any(|signature| signature == &row.owner_signature) {
        return Ok(());
    }
    if signatures.len() == 1
        && validate_exclusion_receipt_only_signature_migration(
            &row.owner_signature,
            &signatures[0],
            &row.classification,
        )
        .is_ok()
    {
        return Ok(());
    }
    if signatures.len() == 1 && has_one_bridge_voice_input_definition(&source, function)
        && validate_bridge_voice_input_signature(row, &signatures[0]).is_ok() {
        return Ok(());
    }
    Err(format!(
        "POPULATION FREEZE: exclusion owner {} signature drifted; expected {:?}, current {:?}. Receipt-only migrations retain name/generics/parameters and one classified receipt. Only exact E050 additionally admits its immutable canonical voice-plan input and optional typed extension companion, retaining one preserving receipt.",
        row.owner, row.owner_signature, signatures
    ))
}

fn validate_owner_signature_if_present(
    manifest_dir: &Path,
    owner: &str,
    expected_signature: &str,
    receipt_migration_identity_kind: Option<&str>,
    nested_inner_identity_kind: Option<&str>,
    label: &str,
) -> Result<(), String> {
    let Some((path, function)) = owner.rsplit_once("::") else {
        return Err(format!("POPULATION FREEZE: malformed {label} owner {owner:?}"));
    };
    let source_path = manifest_dir.join(path);
    if !source_path.exists() {
        return Ok(());
    }
    let source = fs::read_to_string(&source_path).map_err(|error| {
        format!(
            "POPULATION FREEZE: cannot read {label} owner source {}: {error}",
            source_path.display()
        )
    })?;

    if expected_signature.starts_with("macro:fn ") {
        if crate::take_change_control::receipt_surface_normalization_kodex::admits_macro_owner(
            manifest_dir,
            owner,
            expected_signature,
        )? {
            return Ok(());
        }
        let compact = source.split_whitespace().collect::<String>();
        let needle = format!("define_replacement_authority!({function},");
        if compact.contains(&needle) {
            return Ok(());
        }
        // A removed macro-generated owner is allowed: retirement/consolidation may shrink the set.
        if !compact.contains(function) {
            return Ok(());
        }
        return Err(format!(
            "POPULATION FREEZE: {label} macro owner {owner} still exists but no longer matches its frozen macro signature"
        ));
    }

    let signatures = function_signatures(&source, function);
    if signatures.is_empty() {
        // Retirement is monotonic: a registered transformation/sink may disappear.
        return Ok(());
    }
    if signatures.iter().any(|signature| signature == expected_signature) {
        return Ok(());
    }
    if let Some(identity_kind) = receipt_migration_identity_kind
        && signatures.len() == 1
        && validate_receipt_only_signature_migration(
            expected_signature,
            &signatures[0],
            identity_kind,
        )
        .is_ok()
    {
        return Ok(());
    }
    if let Some(identity_kind) = receipt_migration_identity_kind
        && signatures.len() == 1
        && validate_respelling_chain_input_signature(
            owner, expected_signature, &signatures[0], identity_kind,
        ).is_ok()
    {
        return Ok(());
    }
    if let Some(identity_kind) = receipt_migration_identity_kind
        && signatures.len() == 1
        && admits_beam_companion(manifest_dir, owner, expected_signature, &signatures[0], identity_kind).is_ok()
    {
        return Ok(());
    }
    if let (Some(outer_identity_kind), Some(inner_identity_kind)) =
        (receipt_migration_identity_kind, nested_inner_identity_kind)
        && signatures.len() == 1
        && validate_nested_owner_receipt_transport_signature_migration(
            expected_signature,
            &signatures[0],
            outer_identity_kind,
            inner_identity_kind,
        )
        .is_ok()
    {
        return Ok(());
    }
    Err(format!(
        "POPULATION FREEZE: {label} owner {owner} signature drifted; expected {expected_signature:?}, current {signatures:?}. The admitted population-owner migrations keep function name/generics/parameters identical and add either one typed receipt return or the single pinned nested-owner 59 -> 60 receipt pair, or the closed E053 original-receipt observation companion. Only the exact #60 original-chain input has its separate closed parameter migration."
    ))
}

fn validate_current_surfaces(
    manifest_dir: &Path,
    surfaces: &[ProductiveWriteSurface],
    baseline: &[SurfaceBaselineRow],
    population: &[PopulationRow],
    exclusions: &[ExclusionRow],
    nested_owner_receipt_transports: &[NestedOwnerReceiptTransportRow],
) -> Result<(), String> {
    let baseline_map = baseline
        .iter()
        .map(|row| {
            (
                surface_key(&row.path, &row.function, &row.owner_signature),
                row,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let population_by_owner = population
        .iter()
        .map(|row| (row.owner.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    let nested_inner_identity_by_outer = nested_owner_receipt_transports
        .iter()
        .filter_map(|contract| {
            population
                .iter()
                .find(|row| row.id == contract.inner_id)
                .map(|inner| (contract.outer_id, inner.identity_kind.as_str()))
        })
        .collect::<BTreeMap<_, _>>();
    let exclusions_by_owner = exclusions
        .iter()
        .filter(|row| row.exclusion_id != ADOPTION_EXCLUSION_ID)
        .map(|row| (row.owner.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    let mut current = BTreeMap::<String, CurrentSurfaceGroup>::new();

    for surface in surfaces {
        let signature = surface_signature(manifest_dir, surface)?;
        let owner = format!("{}::{}", surface.path, surface.function);
        let macro_receipt_migration = if let Some(row) = population_by_owner.get(owner.as_str()) {
            crate::take_change_control::receipt_surface_normalization_kodex::admits_macro_owner(
                manifest_dir,
                &owner,
                &row.owner_signature,
            )?
        } else {
            false
        };
        let baseline_signature = population_by_owner
            .get(owner.as_str())
            .filter(|row| {
                macro_receipt_migration
                    || validate_receipt_only_signature_migration(
                        &row.owner_signature,
                        &signature,
                        &row.identity_kind,
                    )
                    .is_ok()
                    || validate_respelling_chain_input_signature(
                        &owner, &row.owner_signature, &signature, &row.identity_kind,
                    ).is_ok()
                    || admits_beam_companion(manifest_dir, &owner, &row.owner_signature,
                        &signature, &row.identity_kind).is_ok()
                    || nested_inner_identity_by_outer.get(&row.id).is_some_and(
                        |inner_identity_kind| {
                            validate_nested_owner_receipt_transport_signature_migration(
                                &row.owner_signature,
                                &signature,
                                &row.identity_kind,
                                inner_identity_kind,
                            )
                            .is_ok()
                        },
                    )
            })
            .map(|row| row.owner_signature.as_str())
            .or_else(|| {
                exclusions_by_owner.get(owner.as_str()).and_then(|row| {
                    (validate_exclusion_receipt_only_signature_migration(
                        &row.owner_signature,
                        &signature,
                        &row.classification,
                    ).is_ok() || validate_bridge_voice_input_signature(row, &signature).is_ok())
                    .then_some(row.owner_signature.as_str())
                })
            })
            .unwrap_or(signature.as_str());
        let key = surface_key(&surface.path, &surface.function, baseline_signature);
        let group = current.entry(key).or_default();
        group.instances += 1;
        group.max_xml_buffer_mutations = group
            .max_xml_buffer_mutations
            .max(surface.xml_buffer_mutations);
        group.max_xml_file_writes = group.max_xml_file_writes.max(surface.xml_file_writes);
        group.direct_from_real_main |= surface.direct_from_real_main;
    }

    for (key, group) in current {
        if let Some(row) = baseline_map.get(&key) {
            if group.instances > row.max_instances
                || group.max_xml_buffer_mutations > row.max_xml_buffer_mutations
                || group.max_xml_file_writes > row.max_xml_file_writes
            {
                return Err(format!(
                    "POPULATION FREEZE COVERAGE VIOLATION: {key} escalated: instances {}/{}, buffer {}/{}, file {}/{}",
                    group.instances,
                    row.max_instances,
                    group.max_xml_buffer_mutations,
                    row.max_xml_buffer_mutations,
                    group.max_xml_file_writes,
                    row.max_xml_file_writes
                ));
            }
            continue;
        }

        let (path, function, _signature) = split_surface_key(&key)?;
        if is_master_adoption_consolidation(path, function) && !group.direct_from_real_main {
            continue;
        }
        return Err(format!(
            "POPULATION FREEZE COVERAGE VIOLATION: new productive XML mutation surface {key}. It must be an already-frozen population/exclusion surface or a non-real_main consolidation below src/structural_write_authority/master_adoption/ with an adopt_/mutate_/apply_exact_/apply_block_ owner."
        ));
    }
    // Deliberately no reverse check: a frozen sink may disappear or be consolidated.
    Ok(())
}

fn surface_signature(
    manifest_dir: &Path,
    surface: &ProductiveWriteSurface,
) -> Result<String, String> {
    let source_path = manifest_dir.join(&surface.path);
    let source = fs::read_to_string(&source_path).map_err(|error| {
        format!(
            "POPULATION FREEZE: cannot read current mutation surface {}: {error}",
            source_path.display()
        )
    })?;
    let line_start = byte_offset_of_line(&source, surface.line).ok_or_else(|| {
        format!(
            "POPULATION FREEZE: current mutation surface {}::{} has invalid scanner line {}",
            surface.path, surface.function, surface.line
        )
    })?;
    let masked = mask_non_code(&source);
    let needle = format!("fn {}", surface.function);
    let relative = masked[line_start..].find(&needle).ok_or_else(|| {
        format!(
            "POPULATION FREEZE: cannot bind current mutation surface {}::{} to its function signature",
            surface.path, surface.function
        )
    })?;
    let start = line_start + relative;
    let open = find_function_open_brace(&masked, start + needle.len()).ok_or_else(|| {
        format!(
            "POPULATION FREEZE: cannot find function body for current mutation surface {}::{}",
            surface.path, surface.function
        )
    })?;
    Ok(normalize_signature(&source[start..open]))
}

fn function_signatures(source: &str, function: &str) -> Vec<String> {
    let masked = mask_non_code(source);
    let needle = format!("fn {function}");
    let bytes = masked.as_bytes();
    let mut cursor = 0usize;
    let mut signatures = Vec::new();
    while let Some(relative) = masked[cursor..].find(&needle) {
        let start = cursor + relative;
        let after = start + needle.len();
        if token_boundary_before(bytes, start)
            && token_boundary_after(bytes, after)
            && let Some(open) = find_function_open_brace(&masked, after)
        {
            signatures.push(normalize_signature(&source[start..open]));
        }
        cursor = after;
    }
    signatures.sort();
    signatures.dedup();
    signatures
}

fn find_function_open_brace(masked: &str, start: usize) -> Option<usize> {
    for (relative, byte) in masked.as_bytes()[start..].iter().enumerate() {
        match *byte {
            b'{' => return Some(start + relative),
            b';' => return None,
            _ => {}
        }
    }
    None
}

fn normalize_signature(signature: &str) -> String {
    signature.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn byte_offset_of_line(source: &str, line: usize) -> Option<usize> {
    if line == 0 {
        return None;
    }
    if line == 1 {
        return Some(0);
    }
    let mut current_line = 1usize;
    for (index, byte) in source.bytes().enumerate() {
        if byte == b'\n' {
            current_line += 1;
            if current_line == line {
                return Some(index + 1);
            }
        }
    }
    None
}

fn surface_key(path: &str, function: &str, signature: &str) -> String {
    format!("{path}::{function}|{signature}")
}

fn split_surface_key(key: &str) -> Result<(&str, &str, &str), String> {
    let Some((owner, signature)) = key.split_once('|') else {
        return Err(format!("POPULATION FREEZE: malformed surface key {key:?}"));
    };
    let Some((path, function)) = owner.rsplit_once("::") else {
        return Err(format!("POPULATION FREEZE: malformed surface owner {owner:?}"));
    };
    Ok((path, function, signature))
}

fn is_master_adoption_consolidation(path: &str, function: &str) -> bool {
    path.starts_with("src/structural_write_authority/master_adoption/")
        && ["adopt_", "mutate_", "apply_exact_", "apply_block_"]
            .iter()
            .any(|prefix| function.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn respelling_chain_input_is_closed_to_one_owner_and_immutable_signature() {
        assert!(validate_respelling_chain_input_signature(
            RESPELLING_CHAIN_OWNER, RESPELLING_CHAIN_FROZEN_SIGNATURE,
            RESPELLING_CHAIN_INPUT_SIGNATURE, "EXACT",
        ).is_ok());
        assert!(validate_receipt_only_signature_migration(
            RESPELLING_CHAIN_FROZEN_SIGNATURE, RESPELLING_CHAIN_INPUT_SIGNATURE, "EXACT",
        ).is_err());
        for owner in ["src/other.rs::canonical_master_functional_respelling_from_render", "src/pipeline_shards/key_signature_enharmonic_report_core.rs::other"] {
            assert!(validate_respelling_chain_input_signature(
                owner, RESPELLING_CHAIN_FROZEN_SIGNATURE, RESPELLING_CHAIN_INPUT_SIGNATURE, "EXACT",
            ).is_err());
        }
        for identity in ["BLOCK_DOCUMENT", "PRESERVING", "ADDITIVE"] {
            assert!(validate_respelling_chain_input_signature(
                RESPELLING_CHAIN_OWNER, RESPELLING_CHAIN_FROZEN_SIGNATURE,
                RESPELLING_CHAIN_INPUT_SIGNATURE, identity,
            ).is_err());
        }
        for signature in [
            RESPELLING_CHAIN_INPUT_SIGNATURE.replace("original_chains: &", "original_chains: &mut "),
            RESPELLING_CHAIN_INPUT_SIGNATURE.replace("OriginalFunctionalRespellingChains", "String"),
            RESPELLING_CHAIN_INPUT_SIGNATURE.replace("original_chains:", "other:"),
            RESPELLING_CHAIN_INPUT_SIGNATURE.replace(", ) ->", ", extra: bool, ) ->"),
            RESPELLING_CHAIN_INPUT_SIGNATURE.replace("from_render(", "from_render<T>("),
            RESPELLING_CHAIN_INPUT_SIGNATURE.replace("Option<ExactTransformationReceipt>", "Option<BlockTransformationReceipt>"),
            RESPELLING_CHAIN_INPUT_SIGNATURE.replace("Option<ExactTransformationReceipt>)", "Option<ExactTransformationReceipt>, Option<ExactTransformationReceipt>)"),
            RESPELLING_CHAIN_INPUT_SIGNATURE.replace("(Result<String, String>, Option<ExactTransformationReceipt>)", "Result<String, String>"),
        ] {
            assert!(validate_respelling_chain_input_signature(
                RESPELLING_CHAIN_OWNER, RESPELLING_CHAIN_FROZEN_SIGNATURE, &signature, "EXACT",
            ).is_err(), "unlicensed signature: {signature}");
        }
        assert!(validate_respelling_chain_input_signature(
            RESPELLING_CHAIN_OWNER,
            &RESPELLING_CHAIN_FROZEN_SIGNATURE.replace("render_xml: &str", "render_xml: &mut str"),
            RESPELLING_CHAIN_INPUT_SIGNATURE, "EXACT",
        ).is_err());
    }

    #[test]
    fn respelling_chain_input_reaches_actual_guards_without_more_write_budget() {
        let root = std::env::temp_dir().join(format!("etuschlopf-chain-input-{}", std::process::id()));
        let (path, function) = RESPELLING_CHAIN_OWNER.rsplit_once("::").expect("owner path");
        let file = root.join(path);
        fs::create_dir_all(file.parent().expect("parent")).expect("fixture directory");
        fs::write(&file, format!("{RESPELLING_CHAIN_INPUT_SIGNATURE} {{ unreachable!() }}\n"))
            .expect("synthetic signature fixture");
        let population = vec![PopulationRow {
            id: 60, owner: RESPELLING_CHAIN_OWNER.into(),
            owner_signature: RESPELLING_CHAIN_FROZEN_SIGNATURE.into(),
            effect_class: "DESTRUCTIVE".into(), identity_kind: "EXACT".into(),
            domain: "M".into(), review_status: "AGENT_TRACED".into(),
            prerequisite_state: "READY".into(),
        }];
        let baseline = vec![SurfaceBaselineRow {
            path: path.into(), function: function.into(),
            owner_signature: RESPELLING_CHAIN_FROZEN_SIGNATURE.into(),
            max_instances: 1, max_xml_buffer_mutations: 1, max_xml_file_writes: 0,
            coverage: "population:60".into(),
        }];
        let surface = ProductiveWriteSurface {
            path: path.into(), function: function.into(), line: 1,
            xml_buffer_mutations: 1, xml_file_writes: 0,
            direct_from_real_main: false, authority_boundary: false,
            class: "synthetic".into(), evidence: "synthetic".into(),
        };
        validate_owner_signature_if_present(
            &root, RESPELLING_CHAIN_OWNER, RESPELLING_CHAIN_FROZEN_SIGNATURE,
            Some("EXACT"), None, "population",
        ).expect("closed original-chain owner input");
        validate_current_surfaces(&root, std::slice::from_ref(&surface), &baseline, &population, &[], &[])
            .expect("same original physical surface budget");
        let mut more_buffers = surface.clone(); more_buffers.xml_buffer_mutations += 1;
        assert!(validate_current_surfaces(&root, &[more_buffers], &baseline, &population, &[], &[]).is_err());
        let mut file_write = surface.clone(); file_write.xml_file_writes = 1;
        assert!(validate_current_surfaces(&root, &[file_write], &baseline, &population, &[], &[]).is_err());
        assert!(validate_current_surfaces(&root, &[surface.clone(), surface], &baseline, &population, &[], &[]).is_err());
        fs::write(&file, format!("{} {{ unreachable!() }}\n",
            RESPELLING_CHAIN_INPUT_SIGNATURE.replace("original_chains: &", "original_chains: &mut ")))
            .expect("synthetic invalid signature");
        assert!(validate_owner_signature_if_present(
            &root, RESPELLING_CHAIN_OWNER, RESPELLING_CHAIN_FROZEN_SIGNATURE,
            Some("EXACT"), None, "population",
        ).is_err());
        fs::remove_dir_all(&root).expect("remove synthetic fixture");
    }

    #[test]
    fn future_adoption_boundary_is_narrow() {
        assert!(is_master_adoption_consolidation(
            "src/structural_write_authority/master_adoption/state.rs",
            "adopt_swap"
        ));
        assert!(is_master_adoption_consolidation(
            "src/structural_write_authority/master_adoption/state.rs",
            "mutate_exact"
        ));
        assert!(!is_master_adoption_consolidation(
            "src/structural_write_authority/other.rs",
            "adopt_swap"
        ));
        assert!(!is_master_adoption_consolidation(
            "src/structural_write_authority/master_adoption/state.rs",
            "raw_splice"
        ));
    }

    #[test]
    fn signature_normalization_is_line_independent() {
        assert_eq!(
            normalize_signature("fn demo(\n  source: &str,\n) -> String\n"),
            "fn demo( source: &str, ) -> String"
        );
    }

    #[test]
    fn receipt_signature_migration_accepts_only_typed_return_extension() {
        {
            let receipt_type = "ExactTransformationReceipt";
            assert!(validate_receipt_only_signature_migration(
                "fn apply(source: &str) -> Option<String>",
                &format!("fn apply(source: &str) -> (Option<String>, Option<{receipt_type}>)"),
                "EXACT",
            )
            .is_ok());
        }
        {
            let receipt_type = "BlockTransformationReceipt";
            assert!(validate_receipt_only_signature_migration(
                "fn rebuild(xml: &mut String)",
                &format!("fn rebuild(xml: &mut String) -> Option<{receipt_type}>"),
                "BLOCK_DOCUMENT",
            )
            .is_ok());
        }
    }

    #[test]
    fn nested_owner_receipt_transport_accepts_only_the_pinned_dual_exact_shape() {
        let frozen = "fn write_master_musicxml_output_authorized( path: &Path, candidate_xml: &str, functional_respelling: FunctionalMasterRespellingOperation, ) -> io::Result<()>";
        assert!(validate_nested_owner_receipt_transport_signature_migration(
            frozen,
            "fn write_master_musicxml_output_authorized( path: &Path, candidate_xml: &str, functional_respelling: FunctionalMasterRespellingOperation, ) -> (io::Result<()>, Option<ExactTransformationReceipt>, Option<ExactTransformationReceipt>)",
            "EXACT",
            "EXACT",
        )
        .is_ok());
        assert!(validate_nested_owner_receipt_transport_signature_migration(
            frozen,
            "fn write_master_musicxml_output_authorized( path: &Path, candidate_xml: &str, functional_respelling: FunctionalMasterRespellingOperation, ) -> (io::Result<()>, OwnerReceipts)",
            "EXACT",
            "EXACT",
        )
        .is_err());
        assert!(validate_nested_owner_receipt_transport_signature_migration(
            frozen,
            "fn write_master_musicxml_output_authorized( path: &Path, candidate_xml: &str, functional_respelling: FunctionalMasterRespellingOperation, ) -> (io::Result<()>, Option<ExactTransformationReceipt>)",
            "EXACT",
            "EXACT",
        )
        .is_err());
        assert!(validate_nested_owner_receipt_transport_signature_migration(
            frozen,
            "fn write_master_musicxml_output_authorized( path: &Path, candidate_xml: &str, functional_respelling: FunctionalMasterRespellingOperation, extra: bool, ) -> (io::Result<()>, Option<ExactTransformationReceipt>, Option<ExactTransformationReceipt>)",
            "EXACT",
            "EXACT",
        )
        .is_err());
        assert!(validate_nested_owner_receipt_transport_signature_migration(
            frozen,
            "fn write_master_musicxml_output_authorized( path: &Path, candidate_xml: &str, functional_respelling: FunctionalMasterRespellingOperation, ) -> (io::Result<()>, Option<ExactTransformationReceipt>, Option<ExactTransformationReceipt>, Option<ExactTransformationReceipt>)",
            "EXACT",
            "EXACT",
        )
        .is_err());
    }

    #[test]
    fn nested_owner_receipt_transport_table_is_single_pair_and_not_generic() {
        let fixture = concat!(
            "outer_id\tinner_id\tcarrier_parameter\tbinding_path\tbinding_expression\torder\n",
            "59\t60\tfunctional_respelling: FunctionalMasterRespellingOperation\tsrc/pipeline_shards/pipeline_orchestrator_metric_final_notation_stage.rs\tstructural_write_authority::FunctionalMasterRespellingOperation::new(canonical_master_functional_respelling_from_render,)\touter_then_inner\n",
        );
        let rows = parse_nested_owner_receipt_transports(fixture).expect("frozen nested-owner fixture");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].outer_id, 59);
        assert_eq!(rows[0].inner_id, 60);
        assert_eq!(rows[0].order, "outer_then_inner");

        let extra = format!(
            "{fixture}61\t62\tother: Carrier\tsrc/other.rs\tCarrier::new(other,)\touter_then_inner\n"
        );
        let extra_rows = parse_nested_owner_receipt_transports(&extra).expect("syntactic table");
        assert!(validate_nested_owner_receipt_transport_shape(&extra_rows).is_err());

        let wrong_pair = fixture.replace("59\t60\t", "58\t60\t");
        let wrong_rows =
            parse_nested_owner_receipt_transports(&wrong_pair).expect("syntactic wrong pair");
        assert!(validate_nested_owner_receipt_transport_shape(&wrong_rows).is_err());
    }

    #[test]
    fn receipt_signature_migration_rejects_parameter_or_contract_drift() {
        assert!(validate_receipt_only_signature_migration(
            "fn apply(source: &str) -> Option<String>",
            "fn apply(source: &str, target: usize) -> (Option<String>, Option<ExactTransformationReceipt>)",
            "EXACT",
        )
        .is_err());
        assert!(validate_receipt_only_signature_migration(
            "fn apply(source: &str) -> Option<String>",
            "fn apply(source: &str) -> (Option<String>, Option<BlockTransformationReceipt>)",
            "EXACT",
        )
        .is_err());
        assert!(validate_receipt_only_signature_migration(
            "fn rebuild(xml: &mut String)",
            "fn rebuild(xml: &mut String) -> (bool, Option<BlockTransformationReceipt>)",
            "BLOCK_DOCUMENT",
        )
        .is_err());
        assert!(validate_receipt_only_signature_migration(
            "fn seed(source: &str) -> String",
            "fn seed(source: &str) -> (String, Option<BlockTransformationReceipt>)",
            "GENESIS",
        )
        .is_err());
    }

    #[test]
    fn exclusion_receipt_signature_migration_accepts_only_preserving_or_additive_receipts() {
        {
            let receipt_type = "PreservingTransformationReceipt";
            assert!(validate_exclusion_receipt_only_signature_migration(
                "fn preserve(xml: &str) -> Result<(), &'static str>",
                &format!(
                    "fn preserve(xml: &str) -> (Result<(), &'static str>, Option<{receipt_type}>)"
                ),
                "PRESERVING",
            )
            .is_ok());
        }
        {
            let receipt_type = "AdditiveTransformationReceipt";
            assert!(validate_exclusion_receipt_only_signature_migration(
                "fn add(xml: &mut String, direction: &str) -> usize",
                &format!(
                    "fn add(xml: &mut String, direction: &str) -> (usize, Option<{receipt_type}>)"
                ),
                "ADDITIVE",
            )
            .is_ok());
            assert!(validate_exclusion_receipt_only_signature_migration(
                "fn add_unit(xml: &mut String)",
                &format!("fn add_unit(xml: &mut String) -> Option<{receipt_type}>"),
                "ADDITIVE",
            )
            .is_ok());
        }
    }

    #[test]
    fn exclusion_receipt_signature_migration_rejects_scope_parameter_and_receipt_drift() {
        assert!(validate_exclusion_receipt_only_signature_migration(
            "fn helper(xml: &str) -> String",
            "fn helper(xml: &str) -> (String, Option<PreservingTransformationReceipt>)",
            "IMPLEMENTATION_SURFACE",
        )
        .is_err());
        assert!(validate_exclusion_receipt_only_signature_migration(
            "fn sink(xml: &str) -> String",
            "fn sink(xml: &str) -> (String, Option<PreservingTransformationReceipt>)",
            "IMPLEMENTATION_TRANSPORT_SINK",
        )
        .is_err());
        assert!(validate_exclusion_receipt_only_signature_migration(
            "fn preserve(xml: &str) -> String",
            "fn preserve(xml: &str, staff: usize) -> (String, Option<PreservingTransformationReceipt>)",
            "PRESERVING",
        )
        .is_err());
        assert!(validate_exclusion_receipt_only_signature_migration(
            "fn preserve(xml: &str) -> String",
            "fn preserve(xml: &str) -> (String, Option<AdditiveTransformationReceipt>)",
            "PRESERVING",
        )
        .is_err());
        assert!(validate_exclusion_receipt_only_signature_migration(
            "fn add(xml: &str) -> String",
            "fn add(xml: &str) -> (String, Option<PreservingTransformationReceipt>)",
            "ADDITIVE",
        )
        .is_err());
        assert!(validate_exclusion_receipt_only_signature_migration(
            "fn add(xml: &str) -> String",
            "fn add(xml: &str) -> (String, bool, Option<AdditiveTransformationReceipt>)",
            "ADDITIVE",
        )
        .is_err());
    }

    #[test]
    fn population_receipt_signature_migration_contract_remains_narrow() {
        assert!(validate_receipt_only_signature_migration(
            "fn apply(source: &str) -> Option<String>",
            "fn apply(source: &str) -> (Option<String>, Option<ExactTransformationReceipt>)",
            "EXACT",
        )
        .is_ok());
        assert!(validate_receipt_only_signature_migration(
            "fn apply(source: &str) -> Option<String>",
            "fn apply(source: &str) -> (Option<String>, Option<PreservingTransformationReceipt>)",
            "EXACT",
        )
        .is_err());
        assert!(validate_receipt_only_signature_migration(
            "fn apply(source: &str) -> Option<String>",
            "fn apply(source: &str) -> (Option<String>, Option<PreservingTransformationReceipt>)",
            "EXACT",
        )
        .is_err());
        assert!(validate_receipt_only_signature_migration(
            "fn apply(source: &str) -> Option<String>",
            "fn apply(source: &str) -> (Option<String>, Option<ExactTransformationReceipt>, bool)",
            "EXACT",
        )
        .is_err());
    }

    #[test]
    fn closed_additive_fragment_fixtures_match_existing_freeze_signature_rules() {
        for (legacy, future, name) in [
            (
                include_str!("receipt_surface_e029_legacy_v1.txt"),
                include_str!("receipt_surface_e029_migrated_fixture_v1.txt"),
                "add_articulation_mark_to_note_fragment_authorized",
            ),
            (
                include_str!("receipt_surface_e031_legacy_v1.txt"),
                include_str!("receipt_surface_e031_migrated_fixture_v1.txt"),
                "add_tenuto_to_note_fragment_authorized",
            ),
            (
                include_str!("receipt_surface_e032_legacy_v1.txt"),
                include_str!("receipt_surface_e032_migrated_fixture_v1.txt"),
                "add_tie_markup_authorized",
            ),
        ] {
            let old = function_signatures(legacy, name);
            let new = function_signatures(future, name);
            assert_eq!(old.len(), 1);
            assert_eq!(new.len(), 1);
            validate_exclusion_receipt_only_signature_migration(&old[0], &new[0], "ADDITIVE")
                .unwrap();
            let qualified = new[0].replace(
                "Option<AdditiveTransformationReceipt>",
                "Option<crate::master_state_identity::AdditiveTransformationReceipt>",
            );
            assert!(
                validate_exclusion_receipt_only_signature_migration(
                    &old[0], &qualified, "ADDITIVE"
                )
                .is_err()
            );
        }
    }

    #[test]
    fn ready_row67_requires_full_relocation_and_never_unlocks_row57() {
        use crate::take_change_control::render_sovereignty_kodex::Row67State;
        let mut rows = parse_population(include_str!("ssc_population_prerequisites_r1250_v1.txt")).unwrap();
        rows.iter_mut().find(|row| row.id==67).unwrap().prerequisite_state="READY".into();
        assert!(validate_population_shape_in_context(&rows,true,Row67State::Relocated).is_ok());
        assert!(validate_population_shape_in_context(&rows,false,Row67State::Relocated).is_err());
        assert!(validate_population_shape_in_context(&rows,true,Row67State::Legacy).is_err());
        rows.iter_mut().find(|row|row.id==57).unwrap().prerequisite_state="READY".into();
        assert!(validate_population_shape_in_context(&rows,true,Row67State::Relocated).is_err());
    }
    #[test]
    fn ready_row57_shape_requires_the_full_installed_positive_k2_tree() {
        use crate::take_change_control::render_sovereignty_kodex::Row67State;
        let rows = parse_population(include_str!("destructive_transformation_population_v1.tsv")).unwrap();
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        validate_population_shape_with_positive_k2(root, &rows, Row67State::Relocated).unwrap();
        let missing = root.join("synthetic_missing_positive_k2_tree");
        assert!(validate_population_shape_with_positive_k2(&missing, &rows, Row67State::Relocated).is_err());
        assert!(validate_population_shape_with_positive_k2(root, &rows, Row67State::Legacy).is_err());
    }

}

#[cfg(test)]
mod beam_companion_tests {
    use super::*;
    const OLD: &str = "fn enforce_beat_scoped_beaming_and_rest_completion_in_xml(xml: &mut String)";
    fn migrated() -> String { format!("{OLD} -> crate::master_state_identity::CapturedBlockReceiptInput") }
    #[test]
    fn beam_companion_preserves_original_event_and_unknown_is_observational() {
        assert!(validate_beam_companion_signature(BEAM_COMPANION_OWNER, OLD, &migrated(), "BLOCK_DOCUMENT").is_ok());
        assert!(validate_beam_companion_carrier(BEAM_COMPANION_FIXTURE).is_ok());
        assert!(validate_beam_companion_carrier(&format!("struct Prior {{ value: usize }}\n{BEAM_COMPANION_FIXTURE}")).is_ok());
        assert!(BEAM_COMPANION_FIXTURE.contains("capture.binds_original_receipt(&receipt)?"));
        assert!(BEAM_COMPANION_FIXTURE.contains("Self{receipt:Some(receipt),beam:Some(beam)}"));
    }
    #[test]
    fn beam_companion_rejects_other_owners_identity_kinds_and_signature_changes() {
        for owner in ["other", "src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::other"] {
            assert!(validate_beam_companion_signature(owner, OLD, &migrated(), "BLOCK_DOCUMENT").is_err());
        }
        for kind in ["EXACT", "BLOCK_MEASURE", "PRESERVING", ""] {
            assert!(validate_beam_companion_signature(BEAM_COMPANION_OWNER, OLD, &migrated(), kind).is_err());
        }
        for bad in [migrated().replace("&mut String", "&str"), migrated().replace("xml:", "other:"), migrated().replace("CapturedBlockReceiptInput", "Other"), format!("{OLD} -> Option<BlockTransformationReceipt>")] {
            assert!(validate_beam_companion_signature(BEAM_COMPANION_OWNER, OLD, &bad, "BLOCK_DOCUMENT").is_err());
        }
    }
    #[test]
    fn beam_companion_rejects_forged_dropped_or_unbound_receipts() {
        for bad in [BEAM_COMPANION_FIXTURE.replace("receipt:Some(receipt)", "receipt:None"),
            BEAM_COMPANION_FIXTURE.replace("capture.binds_original_receipt(&receipt)?;", ""),
            BEAM_COMPANION_FIXTURE.replace("receipt: Option<BlockTransformationReceipt>", "receipt: Option<ExactTransformationReceipt>"),
            format!("{BEAM_COMPANION_FIXTURE}\nstruct CapturedBlockReceiptInput;"),
            format!("/* {BEAM_COMPANION_FIXTURE} */"),
            format!("#[cfg(any())]\n{BEAM_COMPANION_FIXTURE}"),
            format!("mod decoy {{ {BEAM_COMPANION_FIXTURE} }}"),
            format!("fake!({BEAM_COMPANION_FIXTURE});"),
            format!("const FAKE: &str = r#\"{BEAM_COMPANION_FIXTURE}\"#;"),
            BEAM_COMPANION_FIXTURE.replace("map_err(|e|e.to_string())", "unwrap()") ] {
            assert!(validate_beam_companion_carrier(&bad).is_err());
        }
    }
    #[test]
    fn beam_companion_owner_gate_requires_the_actual_closed_carrier_and_fixture() {
        let root = std::env::temp_dir().join(format!("beam-companion-admission-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("src/pipeline_shards")).unwrap();
        fs::create_dir_all(root.join("src/build_gate")).unwrap();
        let owner_path = root.join(BEAM_COMPANION_OWNER.rsplit_once("::").unwrap().0);
        fs::write(&owner_path, format!("{} {{ unreachable!() }}", migrated())).unwrap();
        fs::write(root.join(BEAM_COMPANION_FIXTURE_PATH), BEAM_COMPANION_FIXTURE).unwrap();
        fs::write(root.join("src/master_state_identity.rs"), BEAM_COMPANION_FIXTURE).unwrap();
        let check = || validate_owner_signature_if_present(&root, BEAM_COMPANION_OWNER, OLD,
            Some("BLOCK_DOCUMENT"), None, "population");
        assert!(check().is_ok());
        fs::write(root.join("src/master_state_identity.rs"), BEAM_COMPANION_FIXTURE.replace("receipt:Some(receipt)", "receipt:None")).unwrap();
        assert!(check().is_err());
        fs::write(root.join("src/master_state_identity.rs"), BEAM_COMPANION_FIXTURE).unwrap();
        fs::write(root.join(BEAM_COMPANION_FIXTURE_PATH), "changed contract").unwrap();
        assert!(check().is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn beam_companion_does_not_broaden_existing_receipt_migrations() {
        assert!(validate_receipt_only_signature_migration(OLD, &migrated(), "BLOCK_DOCUMENT").is_err());
        assert!(validate_receipt_only_signature_migration(OLD, &format!("{OLD} -> Option<BlockTransformationReceipt>"), "BLOCK_DOCUMENT").is_ok());
    }
}


#[cfg(test)]
mod bridge_voice_input_tests {
    use super::*;
    fn row() -> ExclusionRow {
        ExclusionRow { exclusion_id: "E050".into(), owner: BRIDGE_VOICE_INPUT_OWNER.into(),
            owner_signature: BRIDGE_VOICE_INPUT_FROZEN.into(), classification: "PRESERVING".into() }
    }
    #[test]
    fn exact_bridge_voice_input_migration_is_admitted_only_by_its_closed_rule() {
        assert!(validate_bridge_voice_input_signature(&row(), BRIDGE_VOICE_INPUT_MIGRATED).is_ok());
        assert!(validate_exclusion_receipt_only_signature_migration(BRIDGE_VOICE_INPUT_FROZEN,
            BRIDGE_VOICE_INPUT_MIGRATED, "PRESERVING").is_err());
    }
    #[test]
    fn bridge_voice_input_rejects_another_exclusion_owner_or_class() {
        for (id, owner, class) in [("E047", BRIDGE_VOICE_INPUT_OWNER, "PRESERVING"),
            ("E050", "src/other.rs::apply_structural_rest_bridge_beam_variant_a_render_write", "PRESERVING"),
            ("E050", BRIDGE_VOICE_INPUT_OWNER, "ADDITIVE"),
            ("E050", BRIDGE_VOICE_INPUT_OWNER, "EXACT")] {
            let mut row=row();row.exclusion_id=id.into();row.owner=owner.into();row.classification=class.into();
            assert!(validate_bridge_voice_input_signature(&row, BRIDGE_VOICE_INPUT_MIGRATED).is_err());
        }
    }
    #[test]
    fn bridge_voice_input_rejects_a_changed_frozen_baseline() {
        let mut row=row();row.owner_signature=row.owner_signature.replace("source_label", "label");
        assert!(validate_bridge_voice_input_signature(&row, BRIDGE_VOICE_INPUT_MIGRATED).is_err());
    }
    #[test]
    fn bridge_voice_input_rejects_mutable_owned_erased_or_extra_context() {
        for bad in [
            BRIDGE_VOICE_INPUT_MIGRATED.replace("voice_guard: &structural", "voice_guard: &mut structural"),
            BRIDGE_VOICE_INPUT_MIGRATED.replace("voice_guard: &structural", "voice_guard: structural"),
            BRIDGE_VOICE_INPUT_MIGRATED.replace("&structural_write_authority::CanonicalFinalChordStructuralPlan", "&str"),
            BRIDGE_VOICE_INPUT_MIGRATED.replace("voice_guard:", "other_guard:"),
            BRIDGE_VOICE_INPUT_MIGRATED.replace("voice_guard:", "extra: bool, voice_guard:"),
        ] { assert!(validate_bridge_voice_input_signature(&row(), &bad).is_err(), "{bad}"); }
    }
    #[test]
    fn bridge_voice_input_preserves_every_original_input_and_the_exact_receipt() {
        for bad in [
            BRIDGE_VOICE_INPUT_MIGRATED.replace("xml: &str", "xml: &mut str"),
            BRIDGE_VOICE_INPUT_MIGRATED.replace("source_label: &str, ", ""),
            BRIDGE_VOICE_INPUT_MIGRATED.replace("output_path:", "path:"),
            BRIDGE_VOICE_INPUT_MIGRATED.replace("Option<PreservingTransformationReceipt>", "Option<ExactTransformationReceipt>"),
            BRIDGE_VOICE_INPUT_MIGRATED.replace("Option<PreservingTransformationReceipt>", "(Option<PreservingTransformationReceipt>, Option<PreservingTransformationReceipt>)"),
            BRIDGE_VOICE_INPUT_MIGRATED.replace("Option<PreservingTransformationReceipt>", "()"),
            BRIDGE_VOICE_INPUT_MIGRATED.replace("fn apply_", "fn other_"),
            BRIDGE_VOICE_INPUT_MIGRATED.replace("( xml:", "<T>( xml:"),
        ] { assert!(validate_bridge_voice_input_signature(&row(), &bad).is_err(), "{bad}"); }
    }
    #[test]
    fn bridge_voice_input_is_checked_at_the_actual_owner_admission() {
        let root=std::env::temp_dir().join(format!("bridge-voice-input-admission-{}",std::process::id()));
        let _=fs::remove_dir_all(&root);let file=root.join(BRIDGE_VOICE_INPUT_OWNER.rsplit_once("::").unwrap().0);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file,format!("{} {{ unreachable!() }}",BRIDGE_VOICE_INPUT_MIGRATED)).unwrap();
        assert!(validate_exclusion_owner_signature_if_present(&root,&row()).is_ok());
        fs::write(&file,format!("{} {{ unreachable!() }}",BRIDGE_VOICE_INPUT_MIGRATED.replace("voice_guard: &structural","voice_guard: &mut structural"))).unwrap();
        assert!(validate_exclusion_owner_signature_if_present(&root,&row()).is_err());
        fs::write(&file,format!("{0} {{ unreachable!() }}\n{0} {{ unreachable!() }}",BRIDGE_VOICE_INPUT_MIGRATED)).unwrap();
        assert!(validate_exclusion_owner_signature_if_present(&root,&row()).is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn bridge_voice_input_retains_the_original_physical_surface_budget() {
        let root=std::env::temp_dir().join(format!("bridge-voice-input-budget-{}",std::process::id()));
        let (path,function)=BRIDGE_VOICE_INPUT_OWNER.rsplit_once("::").unwrap();
        let file=root.join(path);fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file,format!("{BRIDGE_VOICE_INPUT_MIGRATED} {{ unreachable!() }}\n")).unwrap();
        let baseline=[SurfaceBaselineRow { path:path.into(),function:function.into(),
            owner_signature:BRIDGE_VOICE_INPUT_FROZEN.into(),max_instances:1,
            max_xml_buffer_mutations:1,max_xml_file_writes:0,coverage:"exclusion:E050".into() }];
        let surface=ProductiveWriteSurface { path:path.into(),function:function.into(),line:1,
            xml_buffer_mutations:1,xml_file_writes:0,direct_from_real_main:false,
            authority_boundary:false,class:"synthetic".into(),evidence:"synthetic".into() };
        let check=|surfaces: &[ProductiveWriteSurface]| validate_current_surfaces(
            &root,surfaces,&baseline,&[],&[row()],&[]);
        assert!(check(std::slice::from_ref(&surface)).is_ok());
        let mut more=surface.clone();more.xml_buffer_mutations+=1;assert!(check(&[more]).is_err());
        let mut file_write=surface.clone();file_write.xml_file_writes=1;assert!(check(&[file_write]).is_err());
        assert!(check(&[surface.clone(),surface.clone()]).is_err());
        fs::write(&file,format!("{} {{ unreachable!() }}\n",BRIDGE_VOICE_INPUT_MIGRATED.replace("voice_guard: &structural","voice_guard: &mut structural"))).unwrap();
        assert!(check(&[surface]).is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn bridge_voice_input_does_not_change_old_owner_or_receipt_only_admission() {
        let frozen=BRIDGE_VOICE_INPUT_FROZEN;
        let existing=frozen.replace("-> (String, SisterFeedbackBeamVariantATinyRenderWriteAudit)",
            "-> ((String, SisterFeedbackBeamVariantATinyRenderWriteAudit), Option<PreservingTransformationReceipt>)");
        assert!(validate_exclusion_receipt_only_signature_migration(frozen,&existing,"PRESERVING").is_ok());
        assert!(validate_bridge_voice_input_signature(&row(),&existing).is_err());
    }

    #[test]
    fn bridge_extension_companion_admits_only_the_exact_additional_return() {
        assert!(validate_bridge_voice_input_signature(&row(), BRIDGE_EXTENSION_OUTPUT_MIGRATED).is_ok());
        assert!(validate_bridge_voice_input_signature(&row(), BRIDGE_VOICE_INPUT_MIGRATED).is_ok());
        assert!(validate_exclusion_receipt_only_signature_migration(
            BRIDGE_VOICE_INPUT_FROZEN, BRIDGE_EXTENSION_OUTPUT_MIGRATED, "PRESERVING").is_err());
    }

    #[test]
    fn bridge_extension_companion_cannot_change_owner_baseline_or_effect_class() {
        let original = row();
        let mut variants = Vec::new();
        for field in ["exclusion", "owner", "classification", "baseline"] {
            let mut altered = original.clone();
            match field {
                "exclusion" => altered.exclusion_id.push_str("_foreign"),
                "owner" => altered.owner.push_str("_foreign"),
                "classification" => altered.classification = "EXACT".into(),
                _ => altered.owner_signature.push(' '),
            }
            variants.push(altered);
        }
        for altered in variants {
            assert!(validate_bridge_voice_input_signature(&altered, BRIDGE_EXTENSION_OUTPUT_MIGRATED).is_err());
        }
    }

    #[test]
    fn bridge_extension_companion_rejects_erasure_extra_context_and_receipt_drift() {
        let companion = "Option<structural_write_authority::RestBridgeExtensionOccurrenceEvidence>";
        for altered in [
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace(companion, "()"),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace(companion, "Option<String>"),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace(companion, "Option<&structural_write_authority::RestBridgeExtensionOccurrenceEvidence>"),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace(companion, "structural_write_authority::RestBridgeExtensionOccurrenceEvidence"),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace(companion, &format!("{companion}, {companion}")),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace("Option<PreservingTransformationReceipt>, ", ""),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace("Option<PreservingTransformationReceipt>", "Option<ExactTransformationReceipt>"),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace("Option<PreservingTransformationReceipt>", "(Option<PreservingTransformationReceipt>, Option<PreservingTransformationReceipt>)"),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace("voice_guard: &structural", "voice_guard: &mut structural"),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace("voice_guard: &structural", "voice_guard: structural"),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace("voice_guard:", "extra: bool, voice_guard:"),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace("source_label: &str, ", ""),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace("output_path:", "other_path:"),
            BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace("( xml:", "<T>( xml:"),
        ] {
            assert!(validate_bridge_voice_input_signature(&row(), &altered).is_err(), "{altered}");
        }
    }

    #[test]
    fn bridge_extension_companion_is_checked_at_real_owner_admission() {
        let root = std::env::temp_dir().join(format!("bridge-extension-admission-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let file = root.join(BRIDGE_VOICE_INPUT_OWNER.rsplit_once("::").unwrap().0);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, format!("{BRIDGE_EXTENSION_OUTPUT_MIGRATED} {{ unreachable!() }}")).unwrap();
        assert!(validate_exclusion_owner_signature_if_present(&root, &row()).is_ok());
        fs::write(&file, format!("{0} {{ unreachable!() }}\n{0} {{ unreachable!() }}", BRIDGE_EXTENSION_OUTPUT_MIGRATED)).unwrap();
        assert!(validate_exclusion_owner_signature_if_present(&root, &row()).is_err());
        fs::write(&file, format!("{} {{ unreachable!() }}", BRIDGE_EXTENSION_OUTPUT_MIGRATED.replace(
            "RestBridgeExtensionOccurrenceEvidence", "ForeignEvidence"))).unwrap();
        assert!(validate_exclusion_owner_signature_if_present(&root, &row()).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn bridge_extension_companion_keeps_the_original_write_surface_budget() {
        let root = std::env::temp_dir().join(format!("bridge-extension-budget-{}", std::process::id()));
        let (path, function) = BRIDGE_VOICE_INPUT_OWNER.rsplit_once("::").unwrap();
        let file = root.join(path);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, format!("{BRIDGE_EXTENSION_OUTPUT_MIGRATED} {{ unreachable!() }}")).unwrap();
        let baseline = [SurfaceBaselineRow { path: path.into(), function: function.into(),
            owner_signature: BRIDGE_VOICE_INPUT_FROZEN.into(), max_instances: 1,
            max_xml_buffer_mutations: 1, max_xml_file_writes: 0, coverage: "exclusion:E050".into() }];
        let surface = ProductiveWriteSurface { path: path.into(), function: function.into(), line: 1,
            xml_buffer_mutations: 1, xml_file_writes: 0, direct_from_real_main: false,
            authority_boundary: false, class: "synthetic".into(), evidence: "synthetic".into() };
        let check = |surfaces: &[ProductiveWriteSurface]| validate_current_surfaces(
            &root, surfaces, &baseline, &[], &[row()], &[]);
        assert!(check(std::slice::from_ref(&surface)).is_ok());
        let mut extra_buffer = surface.clone(); extra_buffer.xml_buffer_mutations += 1;
        assert!(check(&[extra_buffer]).is_err());
        let mut file_write = surface.clone(); file_write.xml_file_writes += 1;
        assert!(check(&[file_write]).is_err());
        assert!(check(&[surface.clone(), surface]).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
