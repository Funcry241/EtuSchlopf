// DeadCodeDetectionIntegrity owns dead-code suppression policy only.
// Rust source discovery, lexical masking and path/line/token facts remain owned by RUST_KODEX_GATE_CORE.

use crate::rust_source_scan_primitives::{
    collect_rust_files, line_number_at, line_starts, mask_non_code, normalize_path,
    token_boundary_after, token_boundary_before,
};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const DEBT_CEILING_RELATIVE_PATH: &str =
    "src/build_gate/dead_code_suppression_debt_ceiling.tsv";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SuppressionIdentity {
    path: String,
    scope_kind: String,
    anchor: String,
    level: String,
    selector: String,
    cfg: String,
}

#[derive(Debug, Clone)]
struct AttributeSpan {
    start: usize,
    end: usize,
    inner: String,
    is_inner: bool,
}

pub(super) fn validate_transition(
    current_root: &Path,
    incoming_root: &Path,
) -> Result<usize, String> {
    validate_root_dead_code_deny(current_root)?;
    validate_root_dead_code_deny(incoming_root)?;
    validate_no_new_warning_weakening_attributes(current_root, incoming_root)?;
    validate_cargo_lint_weakening_absent(current_root)?;
    validate_cargo_lint_weakening_absent(incoming_root)?;

    let current_ceiling = read_debt_ceiling(current_root)?;
    let incoming_ceiling = read_debt_ceiling(incoming_root)?;
    if !incoming_ceiling.is_subset(&current_ceiling) {
        let widened = incoming_ceiling
            .difference(&current_ceiling)
            .map(identity_text)
            .collect::<Vec<_>>();
        return Err(format!(
            "DEAD-CODE DETECTION INTEGRITY: incoming legacy-debt ceiling widens the installed ceiling: {widened:?}"
        ));
    }

    let current = collect_project_suppressions(current_root)?;
    let incoming = collect_project_suppressions(incoming_root)?;
    validate_against_ceiling(&current, &current_ceiling, "installed parent")?;
    validate_against_ceiling(&incoming, &incoming_ceiling, "incoming tree")?;

    if !incoming.is_subset(&current) {
        let new_or_widened = incoming
            .difference(&current)
            .map(identity_text)
            .collect::<Vec<_>>();
        return Err(format!(
            "DEAD-CODE DETECTION INTEGRITY: new, moved or widened dead-code suppression blocked before copy: {new_or_widened:?}"
        ));
    }
    Ok(incoming.len())
}

pub(super) fn validate_installed(project_root: &Path) -> Result<usize, String> {
    validate_root_dead_code_deny(project_root)?;
    validate_cargo_lint_weakening_absent(project_root)?;
    let ceiling = read_debt_ceiling(project_root)?;
    let installed = collect_project_suppressions(project_root)?;
    validate_against_ceiling(&installed, &ceiling, "installed tree")?;
    let encoded_rustflags = std::env::var("CARGO_ENCODED_RUSTFLAGS").ok();
    validate_compiler_session(encoded_rustflags.as_deref())?;
    Ok(installed.len())
}

fn validate_root_dead_code_deny(root: &Path) -> Result<(), String> {
    let path = root.join("Cargo.toml");
    let text = fs::read_to_string(&path).map_err(|error| {
        format!(
            "DEAD-CODE DETECTION INTEGRITY: Cargo.toml nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    validate_root_dead_code_deny_text(&text)
}

fn validate_root_dead_code_deny_text(text: &str) -> Result<(), String> {
    let mut in_lints_rust = false;
    let mut lints_rust_sections = 0usize;
    let mut warnings_values = Vec::new();
    let mut dead_code_values = Vec::new();
    let mut other_lint_rows = Vec::new();

    for raw in text.lines() {
        let line = raw.split('#').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_lints_rust = line == "[lints.rust]";
            if in_lints_rust {
                lints_rust_sections += 1;
            }
            continue;
        }
        if !in_lints_rust {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            other_lint_rows.push(line.to_string());
            continue;
        };
        match key.trim().trim_matches('"') {
            "warnings" => warnings_values.push(value.trim().trim_matches('"').to_string()),
            "dead_code" => dead_code_values.push(value.trim().trim_matches('"').to_string()),
            _ => other_lint_rows.push(line.to_string()),
        }
    }

    let hard_fail_authority = warnings_values == ["deny"] && dead_code_values == ["deny"];
    let warning_debt_authority = warnings_values == ["warn"] && dead_code_values.is_empty();
    if lints_rust_sections != 1
        || !other_lint_rows.is_empty()
        || !(hard_fail_authority || warning_debt_authority)
    {
        return Err(format!(
            "WARNING POLICY INTEGRITY: root package authority must be exactly either [lints.rust] warnings=deny + dead_code=deny or quarantined [lints.rust] warnings=warn; sections={lints_rust_sections}, warnings={warnings_values:?}, dead_code={dead_code_values:?}, other={other_lint_rows:?}"
        ));
    }
    Ok(())
}

fn validate_against_ceiling(
    actual: &BTreeSet<SuppressionIdentity>,
    ceiling: &BTreeSet<SuppressionIdentity>,
    label: &str,
) -> Result<(), String> {
    if actual.is_subset(ceiling) {
        return Ok(());
    }
    let unknown = actual
        .difference(ceiling)
        .map(identity_text)
        .collect::<Vec<_>>();
    Err(format!(
        "DEAD-CODE DETECTION INTEGRITY: {label} contains suppression identities outside the legacy debt ceiling: {unknown:?}"
    ))
}

fn read_debt_ceiling(root: &Path) -> Result<BTreeSet<SuppressionIdentity>, String> {
    let path = root.join(DEBT_CEILING_RELATIVE_PATH);
    let text = fs::read_to_string(&path).map_err(|error| {
        format!(
            "DEAD-CODE DETECTION INTEGRITY: legacy debt ceiling nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    parse_debt_ceiling(&path, &text)
}

fn parse_debt_ceiling(
    path: &Path,
    text: &str,
) -> Result<BTreeSet<SuppressionIdentity>, String> {
    let mut lines = text.lines();
    if lines.next() != Some("# EtuSchlopf dead-code suppression debt ceiling v1")
        || lines.next() != Some("path\tscope_kind\tanchor\tlevel\tselector\tcfg")
    {
        return Err(format!(
            "DEAD-CODE DETECTION INTEGRITY: debt ceiling {} has no exact v1 schema",
            path.display()
        ));
    }

    let mut out = BTreeSet::new();
    for (index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 6 || fields.iter().any(|field| field.trim().is_empty()) {
            return Err(format!(
                "DEAD-CODE DETECTION INTEGRITY: debt ceiling {} line {} is not an exact six-column identity row",
                path.display(),
                index + 3
            ));
        }
        let identity = SuppressionIdentity {
            path: fields[0].to_string(),
            scope_kind: fields[1].to_string(),
            anchor: fields[2].to_string(),
            level: fields[3].to_string(),
            selector: fields[4].to_string(),
            cfg: fields[5].to_string(),
        };
        if !out.insert(identity.clone()) {
            return Err(format!(
                "DEAD-CODE DETECTION INTEGRITY: duplicate debt identity {}",
                identity_text(&identity)
            ));
        }
    }
    Ok(out)
}

fn validate_no_new_warning_weakening_attributes(
    current_root: &Path,
    incoming_root: &Path,
) -> Result<(), String> {
    let current = collect_project_warning_weakening_attributes(current_root)?;
    let incoming = collect_project_warning_weakening_attributes(incoming_root)?;
    if incoming.is_subset(&current) {
        return Ok(());
    }
    let added_or_widened = incoming
        .difference(&current)
        .map(identity_text)
        .collect::<Vec<_>>();
    Err(format!(
        "WARNING-FREE BUILD INTEGRITY: new, moved or widened Rust lint weakening attribute blocked before copy: {added_or_widened:?}"
    ))
}

fn collect_project_warning_weakening_attributes(
    root: &Path,
) -> Result<BTreeSet<SuppressionIdentity>, String> {
    let mut files = Vec::<PathBuf>::new();
    for relative in ["src", "tests", "tools"] {
        let scan_root = root.join(relative);
        if scan_root.is_dir() {
            collect_rust_files(&scan_root, &mut files).map_err(|error| {
                format!(
                    "WARNING-FREE BUILD INTEGRITY: Rust source collection failed at {}: {error}",
                    scan_root.display()
                )
            })?;
        }
    }
    files.sort();
    files.dedup();

    let mut out = BTreeSet::new();
    for path in files {
        let text = fs::read_to_string(&path).map_err(|error| {
            format!(
                "WARNING-FREE BUILD INTEGRITY: Rust source not readable {}: {error}",
                path.display()
            )
        })?;
        let relative = path.strip_prefix(root).map_err(|_| {
            format!(
                "WARNING-FREE BUILD INTEGRITY: source escaped project root {}",
                path.display()
            )
        })?;
        let relative = normalize_path(relative);
        for identity in collect_file_warning_weakening_attributes(&relative, &text)? {
            out.insert(identity);
        }
    }
    Ok(out)
}

fn collect_file_warning_weakening_attributes(
    relative_path: &str,
    text: &str,
) -> Result<Vec<SuppressionIdentity>, String> {
    let masked = mask_non_code(text);
    let attrs = parse_attributes(&masked)?;
    let starts = line_starts(&masked);
    let mut out = Vec::new();

    for (index, attr) in attrs.iter().enumerate() {
        let specs = warning_weakening_specs(&attr.inner, &[])?;
        if specs.is_empty() {
            continue;
        }
        let (group_start, group_end) = attribute_group(&masked, &attrs, index);
        let group_cfg = attrs[group_start..=group_end]
            .iter()
            .filter_map(|item| direct_cfg_condition(&item.inner))
            .collect::<Vec<_>>();
        let anchor = if attr.is_inner {
            "crate_module".to_string()
        } else {
            anchor_after_group(&masked, attrs[group_end].end)
        };
        let scope_kind = if attr.is_inner { "module" } else { "item" };
        let line = line_number_at(attr.start, &starts);

        for (level, selector, nested_cfg) in specs {
            let mut cfg = group_cfg.clone();
            cfg.extend(nested_cfg);
            cfg.sort();
            cfg.dedup();
            let cfg = if cfg.is_empty() {
                "-".to_string()
            } else {
                cfg.join("&&")
            };
            if anchor.is_empty() {
                return Err(format!(
                    "WARNING-FREE BUILD INTEGRITY: lint weakening attribute without stable anchor in {relative_path}:{line}"
                ));
            }
            out.push(SuppressionIdentity {
                path: relative_path.to_string(),
                scope_kind: scope_kind.to_string(),
                anchor: anchor.clone(),
                level,
                selector,
                cfg,
            });
        }
    }
    Ok(out)
}

fn warning_weakening_specs(
    inner: &str,
    inherited_cfg: &[String],
) -> Result<Vec<(String, String, Vec<String>)>, String> {
    let trimmed = inner.trim();
    if let Some(body) = function_body(trimmed, "cfg_attr") {
        let fields = split_top_level(body);
        if fields.len() < 2 {
            return Err(
                "WARNING-FREE BUILD INTEGRITY: cfg_attr lint weakening is not statically resolvable"
                    .to_string(),
            );
        }
        let mut cfg = inherited_cfg.to_vec();
        cfg.push(normalize_space(fields[0]));
        let mut out = Vec::new();
        for nested in &fields[1..] {
            out.extend(warning_weakening_specs(nested, &cfg)?);
        }
        return Ok(out);
    }

    for level in ["allow", "warn", "expect"] {
        let Some(body) = function_body(trimmed, level) else {
            continue;
        };
        let selectors = split_top_level(body)
            .into_iter()
            .map(normalize_lint_selector)
            .filter(|selector| !selector.is_empty() && !selector.starts_with("reason ="))
            .collect::<BTreeSet<_>>();
        if selectors.is_empty() {
            return Ok(Vec::new());
        }
        return Ok(vec![(
            level.to_string(),
            selectors.into_iter().collect::<Vec<_>>().join(","),
            inherited_cfg.to_vec(),
        )]);
    }
    Ok(Vec::new())
}

fn validate_cargo_lint_weakening_absent(root: &Path) -> Result<(), String> {
    let mut manifests = Vec::<PathBuf>::new();
    collect_cargo_manifests(root, root, &mut manifests)?;
    manifests.sort();
    for manifest in manifests {
        let text = fs::read_to_string(&manifest).map_err(|error| {
            format!(
                "WARNING-FREE BUILD INTEGRITY: Cargo manifest not readable {}: {error}",
                manifest.display()
            )
        })?;
        let canonical_root_warning_debt = manifest == root.join("Cargo.toml");
        validate_cargo_lint_weakening_text(&manifest, &text, canonical_root_warning_debt)?;
    }
    Ok(())
}

fn collect_cargo_manifests(
    root: &Path,
    directory: &Path,
    out: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(directory).map_err(|error| {
        format!(
            "WARNING-FREE BUILD INTEGRITY: Cargo manifest scan failed at {}: {error}",
            directory.display()
        )
    })? {
        let entry = entry.map_err(|error| {
            format!("WARNING-FREE BUILD INTEGRITY: directory entry unreadable: {error}")
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| {
            format!(
                "WARNING-FREE BUILD INTEGRITY: file type unreadable {}: {error}",
                path.display()
            )
        })?;
        if file_type.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if matches!(name.as_ref(), ".git" | "target" | "transfer") {
                continue;
            }
            collect_cargo_manifests(root, &path, out)?;
        } else if file_type.is_file() && entry.file_name().to_string_lossy() == "Cargo.toml" {
            if path.strip_prefix(root).is_err() {
                return Err(format!(
                    "WARNING-FREE BUILD INTEGRITY: Cargo manifest escaped project root {}",
                    path.display()
                ));
            }
            out.push(path);
        }
    }
    Ok(())
}

fn validate_cargo_lint_weakening_text(
    path: &Path,
    text: &str,
    canonical_root_warning_debt: bool,
) -> Result<(), String> {
    let mut protected_section = false;
    for (line_index, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            let section = line.trim_start_matches('[').trim_end_matches(']').trim();
            protected_section = matches!(
                section,
                "lints.rust" | "lints.clippy" | "workspace.lints.rust" | "workspace.lints.clippy"
            );
            continue;
        }
        if !protected_section {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let normalized = normalize_space(value).replace(' ', "");
        let canonical_root_warning_row = canonical_root_warning_debt
            && key.trim().trim_matches('"') == "warnings"
            && normalized == "\"warn\"";
        let weakens = normalized == "\"allow\""
            || (normalized == "\"warn\"" && !canonical_root_warning_row)
            || normalized.contains("level=\"allow\"")
            || normalized.contains("level=\"warn\"");
        if weakens {
            return Err(format!(
                "WARNING-FREE BUILD INTEGRITY: Cargo lint weakening blocked at {}:{}: {}",
                path.display(),
                line_index + 1,
                line
            ));
        }
    }
    Ok(())
}

fn collect_project_suppressions(root: &Path) -> Result<BTreeSet<SuppressionIdentity>, String> {
    let mut files = Vec::<PathBuf>::new();
    for relative in ["src", "tests", "tools/RUST_KODEX_GATE_CORE"] {
        let scan_root = root.join(relative);
        if scan_root.is_dir() {
            collect_rust_files(&scan_root, &mut files).map_err(|error| {
                format!(
                    "DEAD-CODE DETECTION INTEGRITY: Rust source collection failed at {}: {error}",
                    scan_root.display()
                )
            })?;
        }
    }
    files.sort();

    let mut out = BTreeSet::new();
    for path in files {
        let text = fs::read_to_string(&path).map_err(|error| {
            format!(
                "DEAD-CODE DETECTION INTEGRITY: Rust source not readable {}: {error}",
                path.display()
            )
        })?;
        let relative = path.strip_prefix(root).map_err(|_| {
            format!(
                "DEAD-CODE DETECTION INTEGRITY: source escaped project root {}",
                path.display()
            )
        })?;
        let relative = normalize_path(relative);
        for identity in collect_file_suppressions(&relative, &text)? {
            out.insert(identity);
        }
    }
    Ok(out)
}

fn collect_file_suppressions(
    relative_path: &str,
    text: &str,
) -> Result<Vec<SuppressionIdentity>, String> {
    let masked = mask_non_code(text);
    let attrs = parse_attributes(&masked)?;
    let starts = line_starts(&masked);
    let mut out = Vec::new();

    for (index, attr) in attrs.iter().enumerate() {
        let specs = suppression_specs(&attr.inner, &[])?;
        if specs.is_empty() {
            continue;
        }
        let (group_start, group_end) = attribute_group(&masked, &attrs, index);
        let group_cfg = attrs[group_start..=group_end]
            .iter()
            .filter_map(|item| direct_cfg_condition(&item.inner))
            .collect::<Vec<_>>();
        let anchor = if attr.is_inner {
            "crate_module".to_string()
        } else {
            anchor_after_group(&masked, attrs[group_end].end)
        };
        let scope_kind = if attr.is_inner { "module" } else { "item" };
        let line = line_number_at(attr.start, &starts);

        for (level, selector, nested_cfg) in specs {
            let mut cfg = group_cfg.clone();
            cfg.extend(nested_cfg);
            cfg.sort();
            cfg.dedup();
            let cfg = if cfg.is_empty() {
                "-".to_string()
            } else {
                cfg.join("&&")
            };
            if anchor.is_empty() {
                return Err(format!(
                    "DEAD-CODE DETECTION INTEGRITY: protected lint attribute without stable anchor in {relative_path}:{line}"
                ));
            }
            out.push(SuppressionIdentity {
                path: relative_path.to_string(),
                scope_kind: scope_kind.to_string(),
                anchor: anchor.clone(),
                level,
                selector,
                cfg,
            });
        }
    }
    Ok(out)
}

fn parse_attributes(masked: &str) -> Result<Vec<AttributeSpan>, String> {
    let bytes = masked.as_bytes();
    let mut out = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'#' {
            index += 1;
            continue;
        }
        let mut cursor = index + 1;
        let is_inner = bytes.get(cursor) == Some(&b'!');
        if is_inner {
            cursor += 1;
        }
        if bytes.get(cursor) != Some(&b'[') {
            index += 1;
            continue;
        }
        let open = cursor;
        let mut depth = 1usize;
        cursor += 1;
        while cursor < bytes.len() && depth > 0 {
            match bytes[cursor] {
                b'[' => depth += 1,
                b']' => depth -= 1,
                _ => {}
            }
            cursor += 1;
        }
        if depth != 0 {
            return Err("DEAD-CODE DETECTION INTEGRITY: unterminated Rust attribute".to_string());
        }
        let close = cursor - 1;
        out.push(AttributeSpan {
            start: index,
            end: cursor,
            inner: masked[open + 1..close].trim().to_string(),
            is_inner,
        });
        index = cursor;
    }
    Ok(out)
}

fn attribute_group(masked: &str, attrs: &[AttributeSpan], index: usize) -> (usize, usize) {
    let mut start = index;
    while start > 0
        && masked[attrs[start - 1].end..attrs[start].start]
            .trim()
            .is_empty()
    {
        start -= 1;
    }
    let mut end = index;
    while end + 1 < attrs.len()
        && masked[attrs[end].end..attrs[end + 1].start]
            .trim()
            .is_empty()
    {
        end += 1;
    }
    (start, end)
}

fn anchor_after_group(masked: &str, end: usize) -> String {
    let tail = masked[end..].trim_start();
    let first_line = tail.lines().next().unwrap_or_default();
    normalize_space(first_line)
}

fn direct_cfg_condition(inner: &str) -> Option<String> {
    let trimmed = inner.trim();
    let body = function_body(trimmed, "cfg")?;
    Some(normalize_space(body))
}

fn suppression_specs(
    inner: &str,
    inherited_cfg: &[String],
) -> Result<Vec<(String, String, Vec<String>)>, String> {
    let trimmed = inner.trim();

    if let Some(body) = function_body(trimmed, "cfg_attr") {
        let fields = split_top_level(body);
        if fields.len() < 2 {
            return Err(
                "DEAD-CODE DETECTION INTEGRITY: cfg_attr with protected lint is not statically resolvable"
                    .to_string(),
            );
        }
        let mut cfg = inherited_cfg.to_vec();
        cfg.push(normalize_space(fields[0]));
        let mut out = Vec::new();
        for nested in &fields[1..] {
            out.extend(suppression_specs(nested, &cfg)?);
        }
        return Ok(out);
    }

    for level in ["allow", "warn", "expect"] {
        let Some(body) = function_body(trimmed, level) else {
            continue;
        };
        let selectors = split_top_level(body)
            .into_iter()
            .map(normalize_lint_selector)
            .filter(|selector| is_protected_selector(selector))
            .collect::<BTreeSet<_>>();
        if selectors.is_empty() {
            return Ok(Vec::new());
        }
        return Ok(vec![(
            level.to_string(),
            selectors.into_iter().collect::<Vec<_>>().join(","),
            inherited_cfg.to_vec(),
        )]);
    }
    Ok(Vec::new())
}

fn function_body<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let rest = text.strip_prefix(name)?.trim_start();
    let rest = rest.strip_prefix('(')?;
    let body = rest.strip_suffix(')')?;
    Some(body.trim())
}

fn split_top_level(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut depth = 0isize;
    let mut start = 0usize;
    for (index, byte) in bytes.iter().enumerate() {
        match byte {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            b',' if depth == 0 => {
                out.push(text[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }
    out.push(text[start..].trim());
    out
}

fn normalize_lint_selector(selector: &str) -> String {
    normalize_space(selector).replace('-', "_")
}

fn is_protected_selector(selector: &str) -> bool {
    let bytes = selector.as_bytes();
    for protected in ["dead_code", "unused"] {
        if selector == protected
            && token_boundary_before(bytes, 0)
            && token_boundary_after(bytes, protected.len())
        {
            return true;
        }
    }
    false
}

fn normalize_space(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn identity_text(identity: &SuppressionIdentity) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}",
        identity.path,
        identity.scope_kind,
        identity.anchor,
        identity.level,
        identity.selector,
        identity.cfg
    )
}

fn validate_compiler_session(encoded_rustflags: Option<&str>) -> Result<(), String> {
    let Some(encoded) = encoded_rustflags.filter(|value| !value.is_empty()) else {
        return Ok(());
    };
    validate_compiler_flags_tokens(&encoded.split('\u{1f}').collect::<Vec<_>>())
}

fn validate_compiler_flags_tokens(tokens: &[&str]) -> Result<(), String> {
    let mut index = 0usize;
    while index < tokens.len() {
        let token = tokens[index].trim();
        if token == "--cap-lints" {
            let value = tokens.get(index + 1).copied().unwrap_or_default();
            if matches!(value, "allow" | "warn") {
                return Err(format!(
                    "DEAD-CODE COMPILER SESSION INTEGRITY: --cap-lints {value} weakens package dead_code=deny"
                ));
            }
            index += 2;
            continue;
        }
        if let Some(value) = token.strip_prefix("--cap-lints=")
            && matches!(value, "allow" | "warn")
        {
            return Err(format!(
                "DEAD-CODE COMPILER SESSION INTEGRITY: --cap-lints={value} weakens package dead_code=deny"
            ));
        }

        if let Some((level, selector, consumed_next)) = weakening_lint_flag(tokens, index) {
            if matches!(level, "-A" | "--allow") {
                return Err(format!(
                    "WARNING-FREE COMPILER SESSION INTEGRITY: compiler flag {level} {selector} permits a warning suppression"
                ));
            }
            if is_protected_selector(&normalize_lint_selector(selector)) {
                return Err(format!(
                    "DEAD-CODE COMPILER SESSION INTEGRITY: compiler flag {level} {selector} weakens protected dead-code authority"
                ));
            }
            if consumed_next {
                index += 2;
                continue;
            }
        }
        index += 1;
    }
    Ok(())
}

fn weakening_lint_flag<'a>(
    tokens: &'a [&'a str],
    index: usize,
) -> Option<(&'a str, &'a str, bool)> {
    let token = tokens[index].trim();
    for flag in ["--allow", "--warn", "--force-warn"] {
        if token == flag {
            return tokens
                .get(index + 1)
                .map(|selector| (flag, selector.trim(), true));
        }
        if let Some(selector) = token.strip_prefix(&format!("{flag}=")) {
            return Some((flag, selector.trim(), false));
        }
    }
    for flag in ["-A", "-W"] {
        if token == flag {
            return tokens
                .get(index + 1)
                .map(|selector| (flag, selector.trim(), true));
        }
        if let Some(selector) = token.strip_prefix(flag)
            && !selector.is_empty()
        {
            return Some((flag, selector.trim(), false));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dead_code_suppression_is_rejected_by_monotonic_identity() {
        let old = collect_file_suppressions("src/a.rs", "fn live() {}\n").expect("old");
        let new = collect_file_suppressions(
            "src/a.rs",
            concat!("#[allow(", "dead_code)]\nfn hidden() {}\n"),
        )
        .expect("new");
        let old = old.into_iter().collect::<BTreeSet<_>>();
        let new = new.into_iter().collect::<BTreeSet<_>>();
        assert!(!new.is_subset(&old));
    }

    #[test]
    fn dead_code_suppression_scope_widening_changes_identity() {
        let narrow = collect_file_suppressions(
            "src/a.rs",
            concat!("#[cfg(test)]\n#[allow(", "dead_code)]\nmod shared;\n"),
        )
        .expect("narrow");
        let wide = collect_file_suppressions(
            "src/a.rs",
            concat!("#[allow(", "dead_code)]\nmod shared;\n"),
        )
        .expect("wide");
        assert_ne!(narrow, wide);
        assert_eq!(narrow[0].cfg, "test");
        assert_eq!(wide[0].cfg, "-");
    }

    #[test]
    fn cfg_attr_and_unused_group_are_protected_suppressions() {
        let rows = collect_file_suppressions(
            "src/a.rs",
            concat!("#[cfg_attr(test, allow(", "unused))]\nmod shared;\n"),
        )
        .expect("rows");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].selector, "unused");
        assert_eq!(rows[0].cfg, "test");
    }

    #[test]
    fn compiler_dead_code_lint_weakening_is_rejected() {
        for flags in [
            vec!["--cap-lints", "warn"],
            vec!["--allow=dead-code"],
            vec!["-Wunused"],
            vec!["--force-warn", "dead_code"],
        ] {
            assert!(validate_compiler_flags_tokens(&flags).is_err(), "{flags:?}");
        }
        assert!(validate_compiler_flags_tokens(&["--cap-lints=deny", "-Ddead_code"]).is_ok());
    }

    #[test]
    fn root_package_lint_authority_cannot_be_lowered() {
        assert!(validate_root_dead_code_deny_text(
            "[package]\nname=\"x\"\n[lints.rust]\nwarnings = \"deny\"\ndead_code = \"deny\"\n"
        )
        .is_ok());
        for incomplete_or_weakened in [
            "[package]\nname=\"x\"\n[lints.rust]\ndead_code = \"deny\"\n",
            "[package]\nname=\"x\"\n[lints.rust]\ndead_code = \"warn\"\n",
        ] {
            assert!(validate_root_dead_code_deny_text(incomplete_or_weakened).is_err());
        }
    }

    #[test]
    fn root_warning_policy_accepts_hard_fail_and_quarantined_warning_debt_only() {
        assert!(validate_root_dead_code_deny_text(
            "[package]\nname=\"x\"\n[lints.rust]\nwarnings = \"deny\"\ndead_code = \"deny\"\n"
        )
        .is_ok());
        assert!(validate_root_dead_code_deny_text(
            "[package]\nname=\"x\"\n[lints.rust]\nwarnings = \"warn\"\n"
        )
        .is_ok());
        for invalid in [
            "[lints.rust]\nwarnings = \"allow\"\n",
            "[lints.rust]\nwarnings = \"warn\"\ndead_code = \"deny\"\n",
            "[lints.rust]\nwarnings = \"warn\"\nunused_imports = \"allow\"\n",
        ] {
            assert!(validate_root_dead_code_deny_text(invalid).is_err());
        }
    }

    #[test]
    fn canonical_root_warning_debt_row_is_not_treated_as_generic_lint_weakening() {
        assert!(validate_cargo_lint_weakening_text(
            Path::new("Cargo.toml"),
            "[lints.rust]\nwarnings = \"warn\"\n",
            true,
        )
        .is_ok());
        assert!(validate_cargo_lint_weakening_text(
            Path::new("tools/x/Cargo.toml"),
            "[lints.rust]\nwarnings = \"warn\"\n",
            false,
        )
        .is_err());
        assert!(validate_cargo_lint_weakening_text(
            Path::new("Cargo.toml"),
            "[lints.rust]\nwarnings = \"allow\"\n",
            true,
        )
        .is_err());
    }

    #[test]
    fn warning_free_foundation_blocks_new_allow_expect_warn_and_cfg_attr() {
        let parent = collect_file_warning_weakening_attributes(
            "src/a.rs",
            "#[allow(dead_code)]\nfn legacy() {}\n",
        )
        .expect("parent");
        for incoming in [
            "#[allow(dead_code)]\nfn legacy() {}\n#[allow(unused_imports)]\nfn added() {}\n",
            "#[allow(dead_code)]\nfn legacy() {}\n#[expect(clippy::collapsible_if)]\nfn added() {}\n",
            "#[allow(dead_code)]\nfn legacy() {}\n#[warn(unused_variables)]\nfn added() {}\n",
            "#[allow(dead_code)]\nfn legacy() {}\n#[cfg_attr(test, allow(unused_imports))]\nfn added() {}\n",
        ] {
            let incoming = collect_file_warning_weakening_attributes("src/a.rs", incoming)
                .expect("incoming");
            let parent = parent.iter().cloned().collect::<BTreeSet<_>>();
            let incoming = incoming.into_iter().collect::<BTreeSet<_>>();
            assert!(!incoming.is_subset(&parent));
        }
    }

    #[test]
    fn warning_free_foundation_allows_monotonic_suppression_removal() {
        let parent = collect_file_warning_weakening_attributes(
            "src/a.rs",
            "#[allow(dead_code)]\nfn legacy() {}\n",
        )
        .expect("parent")
        .into_iter()
        .collect::<BTreeSet<_>>();
        let incoming = collect_file_warning_weakening_attributes("src/a.rs", "fn legacy() {}\n")
            .expect("incoming")
            .into_iter()
            .collect::<BTreeSet<_>>();
        assert!(incoming.is_subset(&parent));
    }

    #[test]
    fn exact_namespace_relocation_changes_identity_but_preserves_suppression_cardinality() {
        let parent = collect_file_warning_weakening_attributes(
            "src/old_owner.rs",
            "#[allow(dead_code)]\nmod take_change_control;\n",
        )
        .expect("parent")
        .into_iter()
        .collect::<BTreeSet<_>>();
        let incoming = collect_file_warning_weakening_attributes(
            "src/kodex_build_orchestrator.rs",
            "#[allow(dead_code)]\nmod take_change_control;\n",
        )
        .expect("incoming")
        .into_iter()
        .collect::<BTreeSet<_>>();
        assert!(!incoming.is_subset(&parent));
        assert_eq!(parent.len(), incoming.len());
    }

    #[test]
    fn cargo_lint_allow_and_warn_are_rejected_semantically() {
        for text in [
            "[lints.rust]\nwarnings = \"allow\"\n",
            "[lints.rust]\nunused_imports = { level = \"warn\", priority = 1 }\n",
            "[lints.clippy]\ncollapsible_if = \"allow\"\n",
            "[workspace.lints.clippy]\npedantic = { level = \"warn\", priority = -1 }\n",
        ] {
            assert!(validate_cargo_lint_weakening_text(Path::new("Cargo.toml"), text, false).is_err());
        }
        assert!(validate_cargo_lint_weakening_text(
            Path::new("Cargo.toml"),
            "[lints.rust]\nwarnings = \"deny\"\ndead_code = \"deny\"\n",
            false,
        )
        .is_ok());
    }

    #[test]
    fn compiler_allow_flags_are_rejected_without_blocking_deny() {
        for flags in [
            vec!["-Awarnings"],
            vec!["-A", "unused-imports"],
            vec!["--allow=warnings"],
            vec!["--allow", "unused_imports"],
        ] {
            assert!(validate_compiler_flags_tokens(&flags).is_err(), "{flags:?}");
        }
        assert!(validate_compiler_flags_tokens(&["-Dwarnings", "-D", "unused-imports"]).is_ok());
    }

}
