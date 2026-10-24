//! Typed productive authority for the rest/voice family cut over in v0.10.1231 and identity-name-closed in v0.10.1232.
//!
//! The detector and caller remain responsible for musical selection. This
//! module only consumes exact source/range/path-bound capabilities for already
//! selected rest-lane, rest-glyph and strict-staff candidates.

use crate::master_state_identity::ExactTransformationReceipt;
use super::destructive_edit_attribution::{DestructiveEditBatchAttribution, IndividualEditIdentity};
use super::authority_primitives::{
    fingerprint as source_fingerprint,
    syntactic_exact_rest_note as exact_rest_note,
    optional_xml_fragment_parses as xml_fragment_parses,
    child_text,
    contains_document_boundary,
};
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReplacementEffect {
    RestChainRewrite,
    RestVisibilityRewrite,
    RestDisplayPlacement,
    CarrierPreservingMetricProjection,
    StrictStaffGapMaterialization,
}

#[derive(Debug)]
struct ExactReplacement {
    identity: IndividualEditIdentity,
    start: usize,
    end: usize,
    original: String,
    original_fingerprint: u64,
    replacement: String,
}

#[derive(Debug)]
struct ExactReplacementBatchBinding {
    source: String,
    source_fingerprint: u64,
    effect: ReplacementEffect,
    replacements_descending: Vec<ExactReplacement>,
    attribution: DestructiveEditBatchAttribution,
}

impl ExactReplacementBatchBinding {
    fn issue(
        source: &str,
        replacements: Vec<(usize, usize, String)>,
        effect: ReplacementEffect,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        let mut exact = Vec::with_capacity(replacements.len());
        for (ordinal, (start, end, replacement)) in replacements.into_iter().enumerate() {
            let identity = attribution.identity(ordinal)?;
            if start > end || end > source.len() {
                return None;
            }
            let original = source.get(start..end)?.to_string();
            if !replacement_allowed(effect, &original, &replacement, start == end) {
                return None;
            }
            exact.push(ExactReplacement {
                identity,
                start,
                end,
                original_fingerprint: source_fingerprint(&original),
                original,
                replacement,
            });
        }
        exact.sort_by(|left, right| right.start.cmp(&left.start).then_with(|| right.end.cmp(&left.end)));
        for pair in exact.windows(2) {
            let later = &pair[0];
            let earlier = &pair[1];
            if earlier.end > later.start {
                return None;
            }
        }
        for replacement in &exact {
            attribution.mark_bound(replacement.identity);
        }
        Some(Self {
            source: source.to_string(),
            source_fingerprint: source_fingerprint(source),
            effect,
            replacements_descending: exact,
            attribution,
        })
    }

    fn apply(self, xml: &str) -> Option<String> {
        let Self {
            source,
            source_fingerprint: bound_fingerprint,
            effect,
            replacements_descending,
            attribution,
        } = self;
        if source != xml || bound_fingerprint != source_fingerprint(xml) {
            return None;
        }
        let identities = replacements_descending
            .iter()
            .map(|replacement| replacement.identity)
            .collect::<Vec<_>>();
        let mut attributed_xml = source;
        for replacement in replacements_descending {
            let current = attributed_xml.get(replacement.start..replacement.end)?;
            if current != replacement.original
                || source_fingerprint(current) != replacement.original_fingerprint
                || !replacement_allowed(
                    effect,
                    current,
                    &replacement.replacement,
                    replacement.start == replacement.end,
                )
            {
                return None;
            }
            attributed_xml.replace_range(
                replacement.start..replacement.end,
                &replacement.replacement,
            );
        }
        attribution.mark_applied_batch(&identities, xml, &attributed_xml);
        Some(attributed_xml)
    }
}

#[derive(Debug)]
struct RestChainRewriteCapability(ExactReplacementBatchBinding);

impl RestChainRewriteCapability {
    fn issue(
        source: &str,
        replacements: Vec<(usize, usize, String)>,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        ExactReplacementBatchBinding::issue(
            source,
            replacements,
            ReplacementEffect::RestChainRewrite,
            attribution,
        )
        .map(Self)
    }

    fn apply(self, source: &str) -> Option<String> {
        self.0.apply(source)
    }
}

#[derive(Debug)]
struct CarrierPreservingMetricProjectionCapability(ExactReplacementBatchBinding);

impl CarrierPreservingMetricProjectionCapability {
    fn issue(
        source: &str,
        replacements: Vec<(usize, usize, String)>,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        if !carrier_preserving_metric_projection_batch_allowed(source, &replacements) {
            return None;
        }
        ExactReplacementBatchBinding::issue(
            source,
            replacements,
            ReplacementEffect::CarrierPreservingMetricProjection,
            attribution,
        )
        .map(Self)
    }

    fn apply(self, source: &str) -> Option<String> {
        self.0.apply(source)
    }
}

#[derive(Debug)]
struct RestVisibilityRewriteCapability(ExactReplacementBatchBinding);

impl RestVisibilityRewriteCapability {
    fn issue(
        source: &str,
        replacements: Vec<(usize, usize, String)>,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        ExactReplacementBatchBinding::issue(
            source,
            replacements,
            ReplacementEffect::RestVisibilityRewrite,
            attribution,
        )
        .map(Self)
    }

    fn apply(self, source: &str) -> Option<String> {
        self.0.apply(source)
    }
}

#[derive(Debug)]
struct RestDisplayPlacementCapability(ExactReplacementBatchBinding);

impl RestDisplayPlacementCapability {
    fn issue(
        source: &str,
        replacements: Vec<(usize, usize, String)>,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        ExactReplacementBatchBinding::issue(
            source,
            replacements,
            ReplacementEffect::RestDisplayPlacement,
            attribution,
        )
        .map(Self)
    }

    fn apply(self, source: &str) -> Option<String> {
        self.0.apply(source)
    }
}

#[derive(Debug)]
struct StrictStaffGapMaterializationCapability(ExactReplacementBatchBinding);

impl StrictStaffGapMaterializationCapability {
    fn issue(
        source: &str,
        replacements: Vec<(usize, usize, String)>,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        ExactReplacementBatchBinding::issue(
            source,
            replacements,
            ReplacementEffect::StrictStaffGapMaterialization,
            attribution,
        )
        .map(Self)
    }

    fn apply(self, source: &str) -> Option<String> {
        self.0.apply(source)
    }
}

fn replacement_allowed(
    effect: ReplacementEffect,
    original: &str,
    replacement: &str,
    insertion: bool,
) -> bool {
    if contains_document_boundary(replacement) || !xml_fragment_parses(replacement) {
        return false;
    }
    match effect {
        ReplacementEffect::RestChainRewrite => {
            !insertion
                && rest_timeline_fragment(original)
                && (replacement.is_empty() || rest_timeline_fragment(replacement))
        }
        ReplacementEffect::RestVisibilityRewrite => {
            !insertion
                && exact_rest_note(original)
                && (exact_rest_note(replacement) || exact_forward_fragment(replacement))
        }
        ReplacementEffect::RestDisplayPlacement => {
            !insertion
                && exact_rest_note(original)
                && exact_rest_note(replacement)
                && preserved_note_ledger_fields(original, replacement)
        }
        ReplacementEffect::CarrierPreservingMetricProjection => {
            if insertion {
                replacement.contains("<backup")
                    && replacement.contains("<rest")
                    && !replacement.contains("<pitch>")
            } else {
                exact_rest_note(original)
                    && exact_rest_note(replacement)
                    && preserved_note_ledger_fields(original, replacement)
                    && hidden_nonspacing_rest(replacement)
            }
        }
        ReplacementEffect::StrictStaffGapMaterialization => {
            (insertion || rest_timeline_fragment(original))
                && !replacement.trim().is_empty()
                && rest_timeline_fragment(replacement)
        }
    }
}



fn hidden_nonspacing_rest(fragment: &str) -> bool {
    exact_rest_note(fragment)
        && (fragment.contains("print-object=\"no\"") || fragment.contains("print-object='no'"))
        && (fragment.contains("print-spacing=\"no\"") || fragment.contains("print-spacing='no'"))
}

fn carrier_preserving_metric_projection_batch_allowed(
    source: &str,
    replacements: &[(usize, usize, String)],
) -> bool {
    let mut insertion: Option<(usize, &str)> = None;
    let mut rewritten_rest_count = 0_usize;
    let mut source_duration_sum = 0_i64;
    let mut lane_voice: Option<String> = None;
    let mut lane_staff: Option<String> = None;
    let mut latest_rewrite_end = 0_usize;

    for (start, end, replacement) in replacements {
        if *start > *end || *end > source.len() {
            return false;
        }
        if start == end {
            if insertion.replace((*start, replacement.as_str())).is_some() {
                return false;
            }
            continue;
        }

        let Some(original) = source.get(*start..*end) else {
            return false;
        };
        if !exact_rest_note(original)
            || !exact_rest_note(replacement)
            || !preserved_note_ledger_fields(original, replacement)
            || !hidden_nonspacing_rest(replacement)
        {
            return false;
        }
        let Some(duration) = child_text(original, "duration")
            .and_then(|value| value.parse::<i64>().ok())
            .filter(|duration| *duration > 0)
        else {
            return false;
        };
        let Some(voice) = child_text(original, "voice") else {
            return false;
        };
        let Some(staff) = child_text(original, "staff") else {
            return false;
        };
        if lane_voice.as_ref().is_some_and(|expected| expected != &voice)
            || lane_staff.as_ref().is_some_and(|expected| expected != &staff)
        {
            return false;
        }
        if lane_voice.is_none() {
            lane_voice = Some(voice);
        }
        if lane_staff.is_none() {
            lane_staff = Some(staff);
        }
        source_duration_sum = match source_duration_sum.checked_add(duration) {
            Some(sum) => sum,
            None => return false,
        };
        rewritten_rest_count += 1;
        latest_rewrite_end = latest_rewrite_end.max(*end);
    }

    let Some((insertion_pos, overlay)) = insertion else {
        return false;
    };
    if rewritten_rest_count < 2
        || source_duration_sum <= 0
        || insertion_pos < latest_rewrite_end
        || contains_document_boundary(overlay)
        || !xml_fragment_parses(overlay)
        || overlay.contains("<pitch>")
        || overlay.contains("<forward")
    {
        return false;
    }

    let wrapped = format!("<authority-root>{overlay}</authority-root>");
    let Ok(document) = roxmltree::Document::parse(&wrapped) else {
        return false;
    };
    let root = document.root_element();
    let mut backup_duration: Option<i64> = None;
    let mut visible_duration_sum = 0_i64;
    let mut visible_rest_count = 0_usize;
    for child in root.children().filter(|node| node.is_element()) {
        match child.tag_name().name() {
            "backup" => {
                if backup_duration.is_some() {
                    return false;
                }
                backup_duration = child
                    .children()
                    .find(|node| node.is_element() && node.tag_name().name() == "duration")
                    .and_then(|node| node.text())
                    .and_then(|text| text.trim().parse::<i64>().ok());
            }
            "note" => {
                if child.children().any(|node| node.is_element() && node.tag_name().name() == "pitch")
                    || !child.children().any(|node| node.is_element() && node.tag_name().name() == "rest")
                {
                    return false;
                }
                let Some(duration) = child
                    .children()
                    .find(|node| node.is_element() && node.tag_name().name() == "duration")
                    .and_then(|node| node.text())
                    .and_then(|text| text.trim().parse::<i64>().ok())
                    .filter(|duration| *duration > 0)
                else {
                    return false;
                };
                let voice = child
                    .children()
                    .find(|node| node.is_element() && node.tag_name().name() == "voice")
                    .and_then(|node| node.text())
                    .map(str::trim);
                let staff = child
                    .children()
                    .find(|node| node.is_element() && node.tag_name().name() == "staff")
                    .and_then(|node| node.text())
                    .map(str::trim);
                if voice != lane_voice.as_deref()
                    || staff != lane_staff.as_deref()
                    || child.attribute("print-object") == Some("no")
                {
                    return false;
                }
                visible_duration_sum = match visible_duration_sum.checked_add(duration) {
                    Some(sum) => sum,
                    None => return false,
                };
                visible_rest_count += 1;
            }
            _ => return false,
        }
    }

    backup_duration == Some(source_duration_sum)
        && visible_duration_sum == source_duration_sum
        && visible_rest_count > 0
        && visible_rest_count < rewritten_rest_count
}

fn rest_timeline_fragment(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.is_empty()
        || exact_rest_note(trimmed)
        || exact_forward_fragment(trimmed)
        || exact_backup_fragment(trimmed)
        || (trimmed.contains("<note")
            && trimmed.contains("<rest")
            && !trimmed.contains("<pitch>"))
        || (trimmed.contains("<backup") && trimmed.contains("<forward"))
}


fn exact_forward_fragment(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.starts_with("<forward")
        && trimmed.ends_with("</forward>")
        && trimmed.contains("<duration>")
        && trimmed.matches("</forward>").count() == 1
}

fn exact_backup_fragment(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.starts_with("<backup")
        && trimmed.ends_with("</backup>")
        && trimmed.contains("<duration>")
        && trimmed.matches("</backup>").count() == 1
}

fn preserved_note_ledger_fields(before: &str, after: &str) -> bool {
    ["duration", "voice", "staff", "type"]
        .iter()
        .all(|tag| child_text(before, tag) == child_text(after, tag))
}



#[track_caller]
pub fn apply_lower_staff_rest_chain_compaction_authorized(
    source: &str,
    replacements: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    
    let geometry = replacements.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        replacements.len(),
    );
    RestChainRewriteCapability::issue(source, replacements, attribution)?.apply(source)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow33ExactMint { _private: () }, source, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

#[track_caller]
pub fn apply_layer_balanced_rest_consolidation_authorized(
    source: &str,
    replacements: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    
    let geometry = replacements.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        replacements.len(),
    );
    if let Some(capability) = RestChainRewriteCapability::issue(
        source,
        replacements.clone(),
        attribution.clone(),
    ) {
        return capability.apply(source);
    }
    CarrierPreservingMetricProjectionCapability::issue(source, replacements, attribution)?.apply(source)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow32ExactMint { _private: () }, source, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

#[track_caller]
pub fn apply_pianistic_rest_consolidation_authorized(
    source: &str,
    replacements: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    
    let geometry = replacements.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        replacements.len(),
    );
    RestChainRewriteCapability::issue(source, replacements, attribution)?.apply(source)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow34ExactMint { _private: () }, source, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

#[track_caller]
pub fn apply_rest_only_staff_readability_authorized(
    source: &str,
    replacements: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    
    let geometry = replacements.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        replacements.len(),
    );
    RestChainRewriteCapability::issue(source, replacements, attribution)?.apply(source)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow35ExactMint { _private: () }, source, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

#[track_caller]
pub fn apply_single_rest_consolidation_authorized(
    source: &str,
    replacements: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    
    let geometry = replacements.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        replacements.len(),
    );
    RestChainRewriteCapability::issue(source, replacements, attribution)?.apply(source)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow37ExactMint { _private: () }, source, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

#[track_caller]
pub fn apply_secondary_reanchored_rest_visibility_authorized(
    source: &str,
    replacements: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    
    let geometry = replacements.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        replacements.len(),
    );
    RestVisibilityRewriteCapability::issue(source, replacements, attribution)?.apply(source)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow36ExactMint { _private: () }, source, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

#[track_caller]
pub fn apply_final_visible_rest_role_rewrite_authorized(
    source: &str,
    replacements: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    
    let geometry = replacements.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        replacements.len(),
    );
    RestVisibilityRewriteCapability::issue(source, replacements, attribution)?.apply(source)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow31ExactMint { _private: () }, source, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

#[track_caller]
pub fn apply_final_rest_glyph_height_authorized(
    source: &str,
    replacements: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    
    let geometry = replacements.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        replacements.len(),
    );
    RestDisplayPlacementCapability::issue(source, replacements, attribution)?.apply(source)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow30ExactMint { _private: () }, source, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

#[track_caller]
pub fn apply_strict_staff_gap_materialization_authorized(
    source: &str,
    replacements: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    
    let geometry = replacements.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        replacements.len(),
    );
    StrictStaffGapMaterializationCapability::issue(source, replacements, attribution)?.apply(source)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow38ExactMint { _private: () }, source, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

#[derive(Debug)]
struct ExactTrialFileBinding {
    source_fingerprint: u64,
    candidate: String,
    candidate_fingerprint: u64,
    output_path: PathBuf,
    semantic_kind: &'static str,
}

impl ExactTrialFileBinding {
    fn issue(source: &str, candidate: &str, output_path: &Path, semantic_kind: &'static str) -> Result<Self> {
        if candidate.trim().is_empty() || roxmltree::Document::parse(candidate).is_err() {
            bail!("{semantic_kind}: candidate is not a complete parseable XML document");
        }
        Ok(Self {
            source_fingerprint: source_fingerprint(source),
            candidate: candidate.to_string(),
            candidate_fingerprint: source_fingerprint(candidate),
            output_path: output_path.to_path_buf(),
            semantic_kind,
        })
    }

    fn commit_bound_candidate(self, source: &str, candidate: &str, output_path: &Path) -> Result<()> {
        if self.source_fingerprint != source_fingerprint(source)
            || self.candidate != candidate
            || self.candidate_fingerprint != source_fingerprint(candidate)
            || self.output_path.as_path() != output_path
        {
            bail!("{}: source, candidate or output path binding changed", self.semantic_kind);
        }
        let candidate_xml = self.candidate;
        let writer_trace_before = crate::audit_only::writer_trace::capture_before(&self.output_path);
        std::fs::write(&self.output_path, candidate_xml.as_bytes()).with_context(|| {
            format!("{}: could not write {}", self.semantic_kind, self.output_path.display())
        })?;
        let readback = std::fs::read(&self.output_path).with_context(|| {
            format!("{}: could not read back {}", self.semantic_kind, self.output_path.display())
        })?;
        if readback != candidate_xml.as_bytes()
            || source_fingerprint(std::str::from_utf8(&readback).unwrap_or_default())
                != self.candidate_fingerprint
        {
            bail!("{}: exact file readback mismatch", self.semantic_kind);
        }
        crate::audit_only::writer_trace::record_after(
            self.semantic_kind,
            &self.output_path,
            writer_trace_before.as_deref(),
            &candidate_xml,
        );
        Ok(())
    }
}

macro_rules! define_trial_sink {
    ($type_name:ident, $function_name:ident, $semantic_kind:literal) => {
        #[derive(Debug)]
        struct $type_name(ExactTrialFileBinding);

        impl $type_name {
            fn issue(source: &str, candidate: &str, output_path: &Path) -> Result<Self> {
                ExactTrialFileBinding::issue(source, candidate, output_path, $semantic_kind).map(Self)
            }

            fn commit(self, source: &str, candidate: &str, output_path: &Path) -> Result<()> {
                self.0.commit_bound_candidate(source, candidate, output_path)
            }
        }

        pub fn $function_name(
            source: &str,
            candidate: &str,
            output_path: &Path,
        ) -> Result<()> {
            $type_name::issue(source, candidate, output_path)?.commit(source, candidate, output_path)
        }
    };
}

define_trial_sink!(
    FragmentedRestChainTrialSink,
    write_fragmented_rest_chain_trial_authorized,
    "fragmented_rest_chain_trial"
);
define_trial_sink!(
    HiddenRestForwardCollapseTrialSink,
    write_hidden_rest_forward_collapse_trial_authorized,
    "hidden_rest_forward_collapse_trial"
);
define_trial_sink!(
    PianisticRestRepairTrialSink,
    write_pianistic_rest_repair_trial_authorized,
    "pianistic_rest_repair_trial"
);
define_trial_sink!(
    PrunedPdfRenderTrialSink,
    write_pruned_pdf_render_trial_authorized,
    "pruned_pdf_render_trial"
);
define_trial_sink!(
    StrictVisibleRestArithmeticTrialSink,
    write_strict_visible_rest_arithmetic_trial_authorized,
    "strict_visible_rest_arithmetic_trial"
);
define_trial_sink!(
    SecondaryReanchoredRestTrialSink,
    write_secondary_reanchored_rest_trial_authorized,
    "secondary_reanchored_rest_trial"
);
define_trial_sink!(
    StrictStaffReleaseTrialSink,
    write_strict_staff_release_trial_authorized,
    "strict_staff_release_trial"
);
define_trial_sink!(
    LowerStaffVisibleRhythmRestLaneTrialSink,
    write_lower_staff_visible_rhythm_rest_lane_trial_authorized,
    "lower_staff_visible_rhythm_rest_lane_trial"
);

pub(crate) struct RenderRow30ExactMint { _private: () }

pub(crate) struct RenderRow31ExactMint { _private: () }

pub(crate) struct RenderRow32ExactMint { _private: () }

pub(crate) struct RenderRow33ExactMint { _private: () }

pub(crate) struct RenderRow34ExactMint { _private: () }

pub(crate) struct RenderRow35ExactMint { _private: () }

pub(crate) struct RenderRow36ExactMint { _private: () }

pub(crate) struct RenderRow37ExactMint { _private: () }

pub(crate) struct RenderRow38ExactMint { _private: () }
