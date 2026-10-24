//! Conservative token erasure; ambiguous syntax and deleted error exits are rejected.
use crate::rust_source_scan_primitives::mask_non_code;
use std::collections::BTreeSet;

pub(crate) fn tokens(source: &str) -> Result<Vec<String>, String> {
    let masked = mask_non_code(source);
    let bytes = source.as_bytes();
    let m = masked.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if bytes[i..].starts_with(b"//") {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if bytes[i..].starts_with(b"/*") {
            let mut d = 1;
            i += 2;
            while i < bytes.len() && d > 0 {
                if bytes[i..].starts_with(b"/*") {
                    d += 1;
                    i += 2;
                } else if bytes[i..].starts_with(b"*/") {
                    d -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            if d != 0 {
                return Err("SSC ERASURE: unterminated comment".into());
            }
            continue;
        }
        // A lexical literal has blank masked bytes; keep all its original bytes.
        if m[i].is_ascii_whitespace() {
            let start = i;
            // Include raw-string prefixes which remain blank in the scanner.
            let mut quote = i;
            while quote < bytes.len() && matches!(bytes[quote], b'b' | b'c' | b'r' | b'#') {
                quote += 1;
            }
            if quote < bytes.len() && bytes[quote] == b'"' {
                let hashes = bytes[start..quote].iter().filter(|b| **b == b'#').count();
                let raw = bytes[start..quote].contains(&b'r');
                i = quote + 1;
                loop {
                    if i >= bytes.len() {
                        return Err("SSC ERASURE: unterminated literal".into());
                    }
                    if !raw && bytes[i] == b'\\' {
                        i += 2;
                        continue;
                    }
                    if bytes[i] == b'"'
                        && (!raw
                            || bytes
                                .get(i + 1..i + 1 + hashes)
                                .is_some_and(|s| s.iter().all(|b| *b == b'#')))
                    {
                        i += 1 + hashes;
                        break;
                    }
                    i += 1;
                }
            } else if bytes[i] == b'\'' || (bytes[i] == b'b' && bytes.get(i + 1) == Some(&b'\'')) {
                i += usize::from(bytes[i] == b'b') + 1;
                loop {
                    if i >= bytes.len() {
                        return Err("SSC ERASURE: unterminated character".into());
                    }
                    if bytes[i] == b'\\' {
                        i += 2;
                    } else if bytes[i] == b'\'' {
                        i += 1;
                        break;
                    } else {
                        i += 1;
                    }
                }
            } else {
                return Err("SSC ERASURE: unrecognized masked syntax".into());
            }
            out.push(source[start..i].to_owned());
            continue;
        }
        let start = i;
        if bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] >= 128 {
            i += 1;
            while i < bytes.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] >= 128)
            {
                i += 1;
            }
        } else {
            i += 1;
        }
        out.push(source[start..i].to_owned());
    }
    // Ensure delimiter integrity before selecting nodes.
    let mut stack = Vec::new();
    for t in &out {
        match t.as_str() {
            "(" | "[" | "{" => stack.push(t.as_str()),
            ")" | "]" | "}" => {
                let expected = match t.as_str() {
                    ")" => "(",
                    "]" => "[",
                    _ => "{",
                };
                if stack.pop() != Some(expected) {
                    return Err("SSC ERASURE: unmatched delimiter".into());
                }
            }
            _ => {}
        }
    }
    if !stack.is_empty() {
        return Err("SSC ERASURE: unclosed delimiter".into());
    }
    Ok(out)
}
fn close(t: &[String], open: usize) -> Result<usize, String> {
    let pair = match t.get(open).map(String::as_str) {
        Some("{") => "}",
        Some("(") => ")",
        Some("[") => "]",
        _ => return Err("SSC ERASURE: expected node delimiter".into()),
    };
    let mut depth = 1;
    for i in open + 1..t.len() {
        if t[i] == t[open] {
            depth += 1;
        }
        if t[i] == pair {
            depth -= 1;
            if depth == 0 {
                return Ok(i);
            }
        }
    }
    Err("SSC ERASURE: node not closed".into())
}
fn function(t: &[String], name: &str) -> Result<(usize, usize, usize), String> {
    let found = t
        .windows(2)
        .enumerate()
        .filter(|(_, w)| w[0] == "fn" && w[1] == name)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    if found.len() != 1 {
        return Err("SSC ERASURE: function missing or ambiguous".into());
    }
    let start = found[0];
    let open = (start + 2..t.len())
        .find(|i| t[*i] == "{" || t[*i] == ";")
        .ok_or("SSC ERASURE: function body missing")?;
    if t[open] != "{" {
        return Err("SSC ERASURE: declaration has no body".into());
    }
    Ok((start, open, close(t, open)?))
}
fn statements(t: &[String], open: usize, end: usize) -> Result<Vec<(usize, usize)>, String> {
    let mut result = Vec::new();
    let mut start = open + 1;
    let mut i = start;
    while i < end {
        if matches!(t[i].as_str(), "(" | "[") {
            i = close(t, i)? + 1;
            continue;
        }
        if t[i] == "{" {
            i = close(t, i)? + 1;
            if t.get(i).is_some_and(|s| s == "else") {
                i += 1;
                continue;
            }
            // A semicolon closes bindings and macro expressions. Control nodes
            // without semicolons close here; unknown expression tails reject later.
            if t.get(i).is_some_and(|s| s == ";") {
                i += 1;
            }
            result.push((start, i));
            start = i;
            continue;
        }
        if t[i] == ";" {
            i += 1;
            result.push((start, i));
            start = i;
        } else {
            i += 1;
        }
    }
    if start < end {
        result.push((start, end));
    }
    Ok(result)
}
fn exits(t: &[String]) -> bool {
    t.iter().any(|s| {
        matches!(
            s.as_str(),
            "?" | "return"
                | "break"
                | "continue"
                | "ensure"
                | "bail"
                | "assert"
                | "assert_eq"
                | "assert_ne"
                | "debug_assert"
                | "debug_assert_eq"
                | "debug_assert_ne"
                | "panic"
                | "unwrap"
                | "expect"
                | "abort"
                | "exit"
                | "unreachable"
                | "todo"
                | "unimplemented"
                | "unsafe"
                | "await"
                | "yield"
        )
    })
}

pub(crate) const COMPLETION_NODES: &str = include_str!("ssc_retirement_completion_nodes_v2.txt");
pub(crate) const WAV_COMPLETION_NODES: &str = include_str!("ssc_wav_retirement_completion_nodes_v2.txt");
pub(crate) const WAV_NODES: &str = include_str!("ssc_wav_retirement_nodes_v1.txt");
const NEUTRAL_NODES: &str = include_str!("ssc_neutral_retirement_nodes_v1.txt");
#[derive(Clone)]
struct ClosedNode<'a> {
    kind: &'a str,
    key: &'a str,
    obligation: &'a str,
    path: &'a str,
    symbol: &'a str,
    container: &'a str,
    hex: &'a str,
    after_hex: Option<&'a str>,
}
fn closed_nodes() -> Result<Vec<ClosedNode<'static>>, String> {
    NEUTRAL_NODES
        .lines()
        .chain(COMPLETION_NODES.lines())
        .chain(WAV_NODES.lines())
        .chain(WAV_COMPLETION_NODES.lines())
        .filter(|s| !s.starts_with('#') && !s.is_empty())
        .map(|line| {
            let r = line.split('|').collect::<Vec<_>>();
            if r.len() != 7 && !(r.len() == 8 && r[0] == "replacement") {
                return Err("SSC ERASURE: invalid compiled neutral node".into());
            }
            Ok(ClosedNode {
                kind: r[0],
                key: r[1],
                obligation: r[2],
                path: r[3],
                symbol: r[4],
                container: r[5],
                hex: r[6],
                after_hex: r.get(7).copied(),
            })
        })
        .collect()
}
fn node_text(node: &ClosedNode<'_>) -> Result<String, String> {
    if !node.hex.len().is_multiple_of(2) {
        return Err("SSC ERASURE: invalid compiled hex".into());
    }
    let bytes = (0..node.hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&node.hex[i..i + 2], 16)
                .map_err(|_| "SSC ERASURE: invalid compiled hex".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    String::from_utf8(bytes).map_err(|_| "SSC ERASURE: invalid compiled UTF-8".into())
}
fn closed_node(key: &str) -> Result<ClosedNode<'static>, String> {
    let found = closed_nodes()?
        .into_iter()
        .filter(|n| matches!(n.kind, "node" | "replacement") && n.key == key)
        .collect::<Vec<_>>();
    if found.len() != 1 {
        return Err("SSC ERASURE: unknown/ambiguous closed node".into());
    }
    Ok(found[0].clone())
}
fn unique_range(
    t: &[String],
    needle: &[String],
    start: usize,
    end: usize,
) -> Result<(usize, usize), String> {
    if needle.is_empty() || end < start {
        return Err("SSC ERASURE: empty closed node".into());
    }
    let found = t[start..end]
        .windows(needle.len())
        .enumerate()
        .filter(|(_, w)| *w == needle)
        .map(|(i, _)| start + i)
        .collect::<Vec<_>>();
    if found.len() != 1 {
        return Err("SSC ERASURE: closed node missing/ambiguous or changed".into());
    }
    Ok((found[0], found[0] + needle.len()))
}
fn closed_range(t: &[String], node: &ClosedNode<'_>) -> Result<(usize, usize), String> {
    let (start, end) = if let Some(name) = node.container.strip_prefix("fn:") {
        let (a, _, b) = function(t, name)?;
        (a, b + 1)
    } else if let Some(name) = node.container.strip_prefix("enum:") {
        let found = t
            .windows(2)
            .enumerate()
            .filter(|(_, w)| w[0] == "enum" && w[1] == name)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        if found.len() != 1 {
            return Err("SSC ERASURE: enum missing/ambiguous".into());
        }
        let a = found[0];
        let open = (a + 2..t.len())
            .find(|i| t[*i] == "{")
            .ok_or("SSC ERASURE: enum body missing")?;
        (open + 1, close(t, open)?)
    } else if node.container == "file" {
        (0, t.len())
    } else {
        return Err("SSC ERASURE: bad compiled container".into());
    };
    unique_range(t, &tokens(&node_text(node)?)?, start, end)
}
pub(crate) fn validate_closed_contexts(
    root: &std::path::Path,
    regions: &[String],
    ids: &BTreeSet<String>,
) -> Result<(), String> {
    let mut groups = BTreeSet::new();
    for region in regions {
        if let Some(key) = region
            .strip_prefix("ClosedItem::")
            .or_else(|| region.strip_prefix("BoundNeutralClosure::"))
            .or_else(|| region.strip_prefix("ClosedReplacement::"))
        {
            let node = closed_node(key)?;
            if !ids.contains(node.obligation) {
                return Err("SSC ERASURE: neutral node obligation mismatch".into());
            }
            if region.starts_with("BoundNeutralClosure::")
                || (region.starts_with("ClosedReplacement::") && node.key.ends_with("_caller"))
            {
                groups.insert(node.obligation);
            }
        }
    }
    for node in closed_nodes()?
        .into_iter()
        .filter(|n| n.kind == "proof" && groups.contains(n.obligation))
    {
        let text = std::fs::read_to_string(root.join(node.path)).map_err(|e| e.to_string())?;
        let t = tokens(&text)?;
        let needle = tokens(&node_text(&node)?)?;
        unique_range(&t, &needle, 0, t.len())?;
        // No second declaration or manual Default may override the sealed proof context.
        if matches!(node.key, "ra5_audit" | "wav_accent_audit_default")
            && mask_non_code(&text)
                .contains("impl Default for AssertiveVisibleArticulationRenderWriteAudit")
        {
            return Err("SSC ERASURE: neutral audit Default changed".into());
        }
    }
    Ok(())
}
pub(crate) fn declared_closed_symbol(
    root: &std::path::Path,
    path: &str,
    name: &str,
) -> Result<bool, String> {
    let text = std::fs::read_to_string(root.join(path)).map_err(|e| e.to_string())?;
    let t = tokens(&text)?;
    Ok(closed_nodes()?
        .into_iter()
        .filter(|n| matches!(n.kind, "node" | "replacement") && n.path == path && n.symbol == name)
        .any(|n| closed_range(&t, &n).is_ok()))
}

/// Local constants disappear with a complete, exact registered private function,
/// never with a surviving function or a replacement node.
pub(crate) fn registered_whole_function_erasure(
    parent: &std::path::Path, child: &std::path::Path, path: &str, name: &str,
) -> Result<Option<(String, String)>, String> {
    let before = std::fs::read_to_string(parent.join(path)).map_err(|e| e.to_string())?;
    let after = std::fs::read_to_string(child.join(path)).unwrap_or_default();
    if super::item(&after, "fn", name).is_ok() {
        return Ok(None);
    }
    let body = tokens(super::item(&before, "fn", name)?)?;
    let parent_tokens = tokens(&before)?;
    let mut found = Vec::new();
    for node in closed_nodes()?.into_iter().filter(|n| {
        n.kind == "node" && n.key.starts_with("wav_complete_")
            && n.path == path && n.container == "file"
    }) {
        if tokens(&node_text(&node)?)? == body && closed_range(&parent_tokens, &node).is_ok() {
            found.push((node.obligation.to_owned(), node.symbol.to_owned()));
        }
    }
    Ok(if found.len() == 1 { found.pop() } else { None })
}

pub(crate) fn registered_audit_initializer_file_loss(
    parent: &std::path::Path, child: &std::path::Path, path: &str,
) -> Result<bool, String> {
    if path != "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs"
        || !registered_caller_loss(parent, child, path, "real_main")?
    { return Ok(false); }
    let node = closed_node("wav_complete_wav_salience_reference_initializer")?;
    let before = std::fs::read_to_string(parent.join(path)).map_err(|e| e.to_string())?;
    let after = std::fs::read_to_string(child.join(path)).map_err(|e| e.to_string())?;
    Ok(verify_in_path(&before, &after,
        &[format!("ClosedReplacement::{}", node.key)],
        &[node.symbol.to_owned()].into_iter().collect(), Some(path)).is_ok())
}

fn validate_removed_expression(t: &[String]) -> Result<(), String> {
    for (i, token) in t.iter().enumerate() {
        if token == "!"
            && t.get(i + 1)
                .is_some_and(|n| matches!(n.as_str(), "(" | "[" | "{"))
        {
            return Err("SSC ERASURE: macro inside removed expression".into());
        }
        if token == "(" && i > 0 && matches!(t[i - 1].as_str(), ">" | ")" | "]") {
            return Err("SSC ERASURE: generic/indirect call inside removed expression".into());
        }
        if token == "="
            && !t
                .get(i.wrapping_sub(1))
                .is_some_and(|s| matches!(s.as_str(), "=" | "!" | "<" | ">"))
            && !t
                .get(i + 1)
                .is_some_and(|s| matches!(s.as_str(), "=" | ">"))
            && !(t.first().is_some_and(|s| s == "let") && i == 2)
        {
            return Err("SSC ERASURE: assignment inside removed expression".into());
        }
        if matches!(
            token.as_str(),
            "+" | "-" | "*" | "/" | "%" | "&" | "|" | "^"
        ) && t.get(i + 1).is_some_and(|s| s == "=")
        {
            return Err("SSC ERASURE: compound assignment inside removed expression".into());
        }
    }
    Ok(())
}

/// Generic regions are WholeFunction::name or Statement::caller::zero_based_ordinal.
/// Path-bound ClosedItem and BoundNeutralClosure keys require compiled parent contexts.
/// Calls and guards must be exclusively linked to declared retired names.
pub(crate) fn verify(
    parent: &str,
    child: &str,
    regions: &[String],
    retired: &BTreeSet<String>,
) -> Result<(), String> {
    if regions
        .iter()
        .any(|r| !r.starts_with("WholeFunction::") && !r.starts_with("Statement::"))
    {
        return Err("SSC ERASURE: unsupported unbound region class".into());
    }
    verify_in_path(parent, child, regions, retired, None)
}
pub(crate) fn verify_in_path(
    parent: &str,
    child: &str,
    regions: &[String],
    retired: &BTreeSet<String>,
    path: Option<&str>,
) -> Result<(), String> {
    let p = tokens(parent)?;
    let c = tokens(child)?;
    let mut erase = BTreeSet::new();
    let mut replacements = std::collections::BTreeMap::new();
    for region in regions {
        let fields = region.split("::").collect::<Vec<_>>();
        let (start, end, whole) = match fields.as_slice() {
            ["WholeFunction", name] => {
                if !retired.contains(*name) {
                    return Err("SSC ERASURE: undeclared retired function".into());
                }
                let (mut start, _, end) = function(&p, name)?;
                // Include visibility and attributes attached to this item.
                if start >= 4 && p[start - 4..start] == ["pub", "(", "crate", ")"] {
                    start -= 4;
                } else if start > 0 && p[start - 1] == "pub" {
                    start -= 1;
                }
                while start > 0 && p[start - 1] == "]" {
                    let mut a = start - 1;
                    let mut depth = 1;
                    while a > 0 && depth > 0 {
                        a -= 1;
                        if p[a] == "]" {
                            depth += 1;
                        } else if p[a] == "[" {
                            depth -= 1;
                        }
                    }
                    if a == 0 || p[a - 1] != "#" {
                        return Err("SSC ERASURE: ambiguous attached attribute".into());
                    }
                    start = a - 1;
                }
                (start, end + 1, true)
            }
            ["ClosedItem", key] | ["BoundNeutralClosure", key] | ["ClosedReplacement", key] => {
                let node = closed_node(key)?;
                if path != Some(node.path) || !retired.contains(node.symbol) {
                    return Err("SSC ERASURE: closed node path/symbol not bound".into());
                }
                if (node.kind == "replacement") != (fields[0] == "ClosedReplacement") {
                    return Err("SSC ERASURE: replacement class mismatch".into());
                }
                let closure = node.kind != "replacement" && node.key.ends_with("_caller");
                if closure != (fields[0] == "BoundNeutralClosure") {
                    return Err("SSC ERASURE: closed node class mismatch".into());
                }
                let (start, end) = closed_range(&p, &node)?;
                if let Some(after) = node.after_hex {
                    let mut replacement = node.clone();
                    replacement.hex = after;
                    replacements.insert(start, tokens(&node_text(&replacement)?)?);
                }
                (start, end, true)
            }
            ["Statement", name, ordinal] => {
                let (_, open, end) = function(&p, name)?;
                let n = ordinal
                    .parse::<usize>()
                    .map_err(|_| "SSC ERASURE: bad ordinal")?;
                let nodes = statements(&p, open, end)?;
                let (start, end) = *nodes.get(n).ok_or("SSC ERASURE: statement missing")?;
                (start, end, false)
            }
            _ => return Err("SSC ERASURE: unsupported region class".into()),
        };
        let removed = &p[start..end];
        if !whole {
            if exits(removed) {
                return Err("SSC ERASURE: removed control-flow/error exit".into());
            }
            if !removed.iter().any(|s| retired.contains(s)) {
                return Err("SSC ERASURE: region has no declared retired edge".into());
            }
            validate_removed_expression(removed)?;
            // No unbound call may disappear: unknown calls require a later exact
            // declared scope, never an optimistic macro/indirect-call interpretation.
            for w in removed.windows(2) {
                if w[1] == "("
                    && w[0].bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
                    && !retired.contains(&w[0])
                    && !matches!(w[0].as_str(), "if" | "match" | "Some" | "None")
                {
                    return Err("SSC ERASURE: non-retired call inside removed node".into());
                }
            }
            if removed.first().is_some_and(|s| s == "let") {
                let binding = removed.get(1).ok_or("SSC ERASURE: binding missing")?;
                if matches!(binding.as_str(), "mut" | "(" | "{") {
                    return Err("SSC ERASURE: complex binding unsupported".into());
                }
                if p.iter()
                    .enumerate()
                    .any(|(i, s)| s == binding && (i < start || i >= end))
                {
                    return Err("SSC ERASURE: binding used outside region".into());
                }
            }
        }
        for i in start..end {
            if !erase.insert(i) {
                return Err("SSC ERASURE: overlapping regions".into());
            }
        }
    }
    let mut survived = Vec::new();
    for (i, token) in p.into_iter().enumerate() {
        if let Some(replacement) = replacements.remove(&i) {
            survived.extend(replacement);
        }
        if !erase.contains(&i) {
            survived.push(token);
        }
    }
    if survived != c {
        return Err("SSC ERASURE: child differs from exact declared erasure".into());
    }
    Ok(())
}
#[cfg(test)]
#[path = "ssc_structural_erasure_tests.rs"]
mod tests;

/// Exact registered caller/test loss, never a general modified-writer exemption.
pub(crate) fn registered_caller_loss(
    parent: &std::path::Path,
    child: &std::path::Path,
    path: &str,
    symbol: &str,
) -> Result<bool, String> {
    let name = symbol.split('#').next().unwrap_or(symbol);
    let before = std::fs::read_to_string(parent.join(path)).map_err(|e| e.to_string())?;
    let p = tokens(&before)?;
    let nodes = closed_nodes()?
        .into_iter()
        .filter(|n| {
            matches!(n.kind, "node" | "replacement")
                && n.path == path
                && !matches!(n.key, "wav_salience_reference_field" | "wav_salience_reference_destructure" | "wav_salience_reference_initializer")
                && n.container == format!("fn:{name}")
                && closed_range(&p, n).is_ok()
                && (n.key.ends_with("_caller")
                    || n.key.starts_with("wav_")
                    || name == "raw_splice_helper_and_real_receipt_match_legacy_bytes")
        })
        .collect::<Vec<_>>();
    if nodes.is_empty() {
        return Ok(false);
    }
    let after = std::fs::read_to_string(child.join(path)).map_err(|e| e.to_string())?;
    let mut regions = Vec::new();
    let mut ids = BTreeSet::new();
    let mut retired = BTreeSet::new();
    for node in &nodes {
        let class = if node.kind == "replacement" {
            "ClosedReplacement"
        } else if node.key.ends_with("_caller") {
            "BoundNeutralClosure"
        } else {
            "ClosedItem"
        };
        regions.push(format!("{class}::{}", node.key));
        ids.insert(node.obligation.to_owned());
        retired.insert(node.symbol.to_owned());
    }
    let proof_regions = if name == "raw_splice_helper_and_real_receipt_match_legacy_bytes"
        && nodes.iter().any(|n| n.obligation == "e031_tenuto")
    {
        vec!["BoundNeutralClosure::e031_caller".to_owned()]
    } else {
        regions.clone()
    };
    validate_closed_contexts(parent, &proof_regions, &ids)?;
    Ok(verify_in_path(
        super::item(&before, "fn", name)?,
        super::item(&after, "fn", name)?,
        &regions,
        &retired,
        Some(path),
    )
    .is_ok())
}
