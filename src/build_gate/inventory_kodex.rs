use super::{read_baseline, BaselineRow};
use super::identity_debt_scan::{scan_source_roots, IdentityDebtFinding};
use super::productive_write_path_scan::{
    parse_productive_write_bypass_baseline,
    productive_write_bypass_baseline_fingerprint,
    scan_productive_write_paths_roots,
    validate_productive_write_bypass_register,
    ProductiveWriteBypassRegisterValidation,
    ProductiveWriteSurface,
};
use super::thirty_surface_batch_scan::{
    enforce_thirty_surface_productive_cutover,
    parse_thirty_surface_batch_manifest,
    validate_thirty_surface_productive_cutover,
};
use std::{
    collections::BTreeMap,
    env,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

fn render_generated_inventory(
    findings: &[IdentityDebtFinding],
    baseline: &BTreeMap<String, BaselineRow>,
    source_files: usize,
    allowed_localizations: usize,
    scanner_fingerprint: u64,
    baseline_fingerprint: u64,
) -> Result<String, String> {
    let mut out = String::new();
    out.push_str("pub const GLOBAL_IDENTITY_DEBT_SOURCE_FILES: usize = ");
    out.push_str(&source_files.to_string());
    out.push_str(";\n");
    out.push_str("pub const GLOBAL_IDENTITY_DEBT_ALLOWED_LOCALIZATIONS: usize = ");
    out.push_str(&allowed_localizations.to_string());
    out.push_str(";\n");
    out.push_str("pub const GLOBAL_IDENTITY_DEBT_UNKNOWN: usize = 0;\n");
    out.push_str("pub const GLOBAL_IDENTITY_SCANNER_FNV1A64: u64 = 0x");
    write!(&mut out, "{:04x}_{:04x}_{:04x}_{:04x}", scanner_fingerprint >> 48, (scanner_fingerprint >> 32) & 0xffff, (scanner_fingerprint >> 16) & 0xffff, scanner_fingerprint & 0xffff).expect("writing to String cannot fail");
    out.push_str(";\n");
    out.push_str("pub const GLOBAL_IDENTITY_BASELINE_FNV1A64: u64 = 0x");
    write!(&mut out, "{:04x}_{:04x}_{:04x}_{:04x}", baseline_fingerprint >> 48, (baseline_fingerprint >> 32) & 0xffff, (baseline_fingerprint >> 16) & 0xffff, baseline_fingerprint & 0xffff).expect("writing to String cannot fail");
    out.push_str(";\n");
    out.push_str("pub const GLOBAL_LEXICAL_IDENTITY_DEBT_ZERO: bool = true;\n");
    out.push_str("pub const PRODUCTIVE_MUSIC_IDENTITY_PROVENANCE_FIREWALL_GREEN: bool = true;\n");
    out.push_str("pub const GLOBAL_IDENTITY_ZERO_DEBT_ENFORCED: bool = true;\n");
    out.push_str(
        "pub const GLOBAL_IDENTITY_DEBT_FINDINGS: &[StaticIdentityDebtFinding] = &[\n",
    );
    for finding in findings {
        let key = finding.key();
        let row = baseline
            .get(&key)
            .ok_or_else(|| format!("missing baseline row while rendering {key}"))?;
        out.push_str("    StaticIdentityDebtFinding {\n");
        writeln!(&mut out, "        kind: {:?},", finding.kind).expect("writing to String cannot fail");
        writeln!(&mut out, "        path: {:?},", finding.path).expect("writing to String cannot fail");
        writeln!(&mut out, "        function: {:?},", finding.function).expect("writing to String cannot fail");
        writeln!(&mut out, "        line: {},", finding.line).expect("writing to String cannot fail");
        writeln!(&mut out, "        payload: {:?},", finding.payload).expect("writing to String cannot fail");
        writeln!(&mut out, "        evidence: {:?},", finding.evidence).expect("writing to String cannot fail");
        writeln!(&mut out, "        owner: {:?},", row.owner).expect("writing to String cannot fail");
        writeln!(&mut out, "        decision: {:?},", row.decision).expect("writing to String cannot fail");
        out.push_str("    },\n");
    }
    out.push_str("];\n");
    Ok(out)
}


fn enforce_write_bypass_shrink_register(
    validation: &ProductiveWriteBypassRegisterValidation,
) -> Result<(), String> {
    if validation.is_green() {
        return Ok(());
    }
    let mut message = String::from("\nHARD PRODUCTIVE WRITE BYPASS SHRINK REGISTER VIOLATION:\n");
    writeln!(
        &mut message,
        "  baseline={} current={} retired={}",
        validation.baseline_external_surfaces,
        validation.current_external_surfaces,
        validation.retired_external_surfaces
    )
    .expect("writing to String cannot fail");
    for key in &validation.unknown_external_surfaces {
        writeln!(&mut message, "  + NEW EXTERNAL WRITE SURFACE: {key}").expect("writing to String cannot fail");
    }
    for detail in &validation.operation_escalations {
        writeln!(&mut message, "  + WRITE OPERATION ESCALATION: {detail}").expect("writing to String cannot fail");
    }
    for key in &validation.direct_main_escalations {
        writeln!(&mut message, "  + NEW REAL_MAIN REACHABILITY: {key}").expect("writing to String cannot fail");
    }
    for key in &validation.duplicate_current_external_surfaces {
        writeln!(&mut message, "  + DUPLICATE CURRENT EXTERNAL SURFACE: {key}").expect("writing to String cannot fail");
    }
    message.push_str(
        "Existing external surfaces may disappear or move into central authority, but no new external surface, primitive-count increase or new real_main reachability is allowed.\n",
    );
    Err(message)
}

fn render_generated_write_path_inventory(
    surfaces: &[ProductiveWriteSurface],
    source_files: usize,
    functions_scanned: usize,
    validation: &ProductiveWriteBypassRegisterValidation,
    baseline_fingerprint: u64,
) -> String {
    let mut out = String::new();
    out.push_str("pub const PRODUCTIVE_WRITE_PATH_SOURCE_FILES: usize = ");
    out.push_str(&source_files.to_string());
    out.push_str(";\n");
    out.push_str("pub const PRODUCTIVE_WRITE_PATH_FUNCTIONS_SCANNED: usize = ");
    out.push_str(&functions_scanned.to_string());
    out.push_str(";\n");
    out.push_str("pub const PRODUCTIVE_WRITE_BYPASS_BASELINE_EXTERNAL_SURFACES: usize = ");
    out.push_str(&validation.baseline_external_surfaces.to_string());
    out.push_str(";\n");
    out.push_str("pub const PRODUCTIVE_WRITE_BYPASS_CURRENT_EXTERNAL_SURFACES: usize = ");
    out.push_str(&validation.current_external_surfaces.to_string());
    out.push_str(";\n");
    out.push_str("pub const PRODUCTIVE_WRITE_BYPASS_RETIRED_EXTERNAL_SURFACES: usize = ");
    out.push_str(&validation.retired_external_surfaces.to_string());
    out.push_str(";\n");
    out.push_str(
        "pub const PRODUCTIVE_WRITE_BYPASS_UNKNOWN_EXTERNAL_SURFACES: usize = 0;\n",
    );
    out.push_str("pub const PRODUCTIVE_WRITE_BYPASS_OPERATION_ESCALATIONS: usize = 0;\n");
    out.push_str("pub const PRODUCTIVE_WRITE_BYPASS_DIRECT_MAIN_ESCALATIONS: usize = 0;\n");
    out.push_str(
        "pub const PRODUCTIVE_WRITE_BYPASS_DUPLICATE_CURRENT_SURFACES: usize = 0;\n",
    );
    out.push_str("pub const PRODUCTIVE_WRITE_BYPASS_BASELINE_FNV1A64: u64 = 0x");
    write!(&mut out, "{:04x}_{:04x}_{:04x}_{:04x}", baseline_fingerprint >> 48, (baseline_fingerprint >> 32) & 0xffff, (baseline_fingerprint >> 16) & 0xffff, baseline_fingerprint & 0xffff).expect("writing to String cannot fail");
    out.push_str(";\n");
    out.push_str("pub const PRODUCTIVE_WRITE_BYPASS_REGISTER_GREEN: bool = true;\n");
    out.push_str(
        "pub const PRODUCTIVE_WRITE_PATH_SURFACES: &[StaticProductiveWriteSurface] = &[\n",
    );
    for surface in surfaces {
        out.push_str("    StaticProductiveWriteSurface {\n");
        writeln!(&mut out, "        path: {:?},", surface.path).expect("writing to String cannot fail");
        writeln!(&mut out, "        function: {:?},", surface.function).expect("writing to String cannot fail");
        writeln!(&mut out, "        line: {},", surface.line).expect("writing to String cannot fail");
        writeln!(
            &mut out,
            "        xml_buffer_mutations: {},",
            surface.xml_buffer_mutations
        )
        .expect("writing to String cannot fail");
        writeln!(
            &mut out,
            "        xml_file_writes: {},",
            surface.xml_file_writes
        )
        .expect("writing to String cannot fail");
        writeln!(
            &mut out,
            "        direct_from_real_main: {},",
            surface.direct_from_real_main
        )
        .expect("writing to String cannot fail");
        writeln!(
            &mut out,
            "        authority_boundary: {},",
            surface.authority_boundary
        )
        .expect("writing to String cannot fail");
        writeln!(&mut out, "        class: {:?},", surface.class).expect("writing to String cannot fail");
        writeln!(&mut out, "        evidence: {:?},", surface.evidence).expect("writing to String cannot fail");
        out.push_str("    },\n");
    }
    out.push_str("];\n");
    out
}

pub(super) struct InventoryKodexInputs<'a> {
    pub(super) manifest_dir: &'a Path,
    pub(super) source_root: &'a Path,
    pub(super) kodex_gate_core_source_root: &'a Path,
    pub(super) baseline_path: &'a Path,
    pub(super) write_bypass_baseline_path: &'a Path,
    pub(super) thirty_surface_batch_manifest_path: &'a Path,
    pub(super) scanner_fingerprint: u64,
    pub(super) baseline_fingerprint: u64,
    pub(super) feedback_gate_fingerprint: u64,
    pub(super) feedback_admission_fingerprint: u64,
}

pub(super) fn validate_and_emit_inventory_kodex(
    inputs: InventoryKodexInputs<'_>,
) -> Result<(), String> {
    let InventoryKodexInputs {
        manifest_dir,
        source_root,
        kodex_gate_core_source_root,
        baseline_path,
        write_bypass_baseline_path,
        thirty_surface_batch_manifest_path,
        scanner_fingerprint,
        baseline_fingerprint,
        feedback_gate_fingerprint,
        feedback_admission_fingerprint,
    } = inputs;
    let scan = scan_source_roots(
        manifest_dir,
        &[source_root, kodex_gate_core_source_root],
    )
    .map_err(|error| error.to_string())?;
    let baseline = read_baseline(baseline_path)?;
    if !baseline.is_empty() {
        return Err(format!(
            "IDENTITY ZERO-DEBT POLICY VIOLATION: {} baseline rows found. Registered productive identity debt is no longer buildable.",
            baseline.len()
        ));
    }
    if !scan.findings.is_empty() {
        let mut message =
            String::from("\nPRODUCTIVE IDENTITY FIREWALL VIOLATION (zero debt required):\n");
        for finding in &scan.findings {
            writeln!(
                &mut message,
                "  + {}:{} {} :: {}",
                finding.path,
                finding.line,
                finding.key(),
                finding.evidence
            )
            .expect("writing to String cannot fail");
        }
        message.push_str("Exact Etuede/Takt feedback may remain in read-only audit localization only. Normative test admission is governed separately by the Corpus-Independence Take-Change-Control; productive selection and writing must use structural evidence without origin identity.\n");
        return Err(message);
    }

    let generated = render_generated_inventory(
        &scan.findings,
        &baseline,
        scan.source_files,
        scan.allowed_localizations,
        scanner_fingerprint,
        baseline_fingerprint,
    )?;
    let out_dir = PathBuf::from(env::var("OUT_DIR").map_err(|error| error.to_string())?);
    let generated_path = out_dir.join("identity_debt_inventory_generated.rs");
    fs::write(&generated_path, generated).map_err(|error| error.to_string())?;
    println!(
        "cargo:rustc-env=ETUSCHLOPF_IDENTITY_DEBT_INVENTORY_RS={}",
        generated_path.display()
    );
    println!(
        "cargo:rustc-env=ETUSCHLOPF_FEEDBACK_GATE_FNV1A64={:016x}",
        feedback_gate_fingerprint
    );
    println!(
        "cargo:rustc-env=ETUSCHLOPF_FEEDBACK_ADMISSION_FNV1A64={:016x}",
        feedback_admission_fingerprint
    );

    let write_scan = scan_productive_write_paths_roots(
        manifest_dir,
        &[source_root, kodex_gate_core_source_root],
    )
    .map_err(|error| error.to_string())?;
    super::destructive_population_freeze_gate::validate_destructive_population_freeze(
        manifest_dir,
        &write_scan.surfaces,
    )?;
    let write_bypass_baseline_text =
        fs::read_to_string(write_bypass_baseline_path).map_err(|error| error.to_string())?;
    const PINNED_WRITE_BYPASS_BASELINE_FNV1A64: u64 = 0xe81d_f25a_184e_d737;
    let write_bypass_baseline_fingerprint =
        productive_write_bypass_baseline_fingerprint(&write_bypass_baseline_text);
    if write_bypass_baseline_fingerprint != PINNED_WRITE_BYPASS_BASELINE_FNV1A64 {
        return Err(format!(
            "HARD BYPASS SHRINK REGISTER BASELINE CHANGED: expected FNV1A64 {:016x}, got {:016x}. A rename, move or policy change requires an explicit source-level fingerprint update.",
            PINNED_WRITE_BYPASS_BASELINE_FNV1A64,
            write_bypass_baseline_fingerprint
        ));
    }
    let write_bypass_baseline =
        parse_productive_write_bypass_baseline(&write_bypass_baseline_text)?;
    let write_bypass_validation =
        validate_productive_write_bypass_register(&write_scan.surfaces, &write_bypass_baseline);
    enforce_write_bypass_shrink_register(&write_bypass_validation)?;
    // Permanent single-owner contract marker; no longer a rollout/disabled-state switch.
    const REQUIRE_ZERO_EXTERNAL_MUSICXML_SURFACES: bool = true;
    let _ = REQUIRE_ZERO_EXTERNAL_MUSICXML_SURFACES;
    if write_bypass_validation.current_external_surfaces != 0 {
        return Err(format!(
            "SINGLE MANDATORY WRITE BOUNDARY REGRESSION: expected 0 external MusicXML surfaces after orchestrator cutover, got {}. Every productive MusicXML write must remain inside src/structural_write_authority/.",
            write_bypass_validation.current_external_surfaces
        ));
    }

    let thirty_surface_batch_manifest_text =
        fs::read_to_string(thirty_surface_batch_manifest_path)
            .map_err(|error| error.to_string())?;
    const PINNED_THIRTY_SURFACE_BATCH_FNV1A64: u64 = 0x669c_72b4_a8b5_eac7;
    let thirty_surface_batch_fingerprint =
        productive_write_bypass_baseline_fingerprint(&thirty_surface_batch_manifest_text);
    if thirty_surface_batch_fingerprint != PINNED_THIRTY_SURFACE_BATCH_FNV1A64 {
        return Err(format!(
            "THIRTY-SURFACE BATCH MANIFEST CHANGED: expected FNV1A64 {:016x}, got {:016x}. Candidate replacement requires an explicit source-level fingerprint update.",
            PINNED_THIRTY_SURFACE_BATCH_FNV1A64,
            thirty_surface_batch_fingerprint
        ));
    }
    let thirty_surface_batch_rows =
        parse_thirty_surface_batch_manifest(&thirty_surface_batch_manifest_text)?;
    let thirty_surface_batch_validation = validate_thirty_surface_productive_cutover(
        source_root,
        &write_scan.surfaces,
        &thirty_surface_batch_rows,
    )?;
    enforce_thirty_surface_productive_cutover(&thirty_surface_batch_validation)?;
    println!("cargo:rustc-env=ETUSCHLOPF_THIRTY_SURFACE_BATCH_PRODUCTIVE=30");

    let write_generated = render_generated_write_path_inventory(
        &write_scan.surfaces,
        write_scan.source_files,
        write_scan.functions_scanned,
        &write_bypass_validation,
        write_bypass_baseline_fingerprint,
    );
    let write_generated_path = out_dir.join("productive_write_path_inventory_generated.rs");
    fs::write(&write_generated_path, write_generated).map_err(|error| error.to_string())?;
    println!(
        "cargo:rustc-env=ETUSCHLOPF_PRODUCTIVE_WRITE_PATH_INVENTORY_RS={}",
        write_generated_path.display()
    );

    Ok(())
}
