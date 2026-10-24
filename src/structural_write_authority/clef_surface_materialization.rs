//! Typed, musically neutral projection of an already classified clef-surface plan.
//!
//! This module owns source binding, mixed-`attributes` handling, the exact XML
//! replacement and the final typed writer boundary. It owns no register,
//! readability or score-selection heuristic.

use crate::master_state_identity::BlockTransformationReceipt;
use super::authority_primitives::fingerprint;
use super::verified_render_candidate_sink_authority::sink_surface_clef_candidate;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct ClefSurfaceInspection {
    source_measure_fingerprint: u64,
    source_start: usize,
    source_end: usize,
    pub visible_lower_g_clefs: usize,
    pub hidden_lower_g_clefs: usize,
    pub lower_f_clefs: usize,
}

impl ClefSurfaceInspection {
    pub fn hide_visible_lower_staff_g_preserve_f_context(
        self,
        source_xml: &str,
    ) -> Option<ClefSurfaceMaterializationPlan> {
        let source_measure = source_xml.get(self.source_start..self.source_end)?;
        if fingerprint(source_measure) != self.source_measure_fingerprint
            || self.lower_f_clefs == 0
            || self.visible_lower_g_clefs == 0
        {
            return None;
        }
        Some(ClefSurfaceMaterializationPlan {
            source_fingerprint: fingerprint(source_xml),
            source_start: self.source_start,
            source_end: self.source_end,
            expected_visible_lower_g_clefs: self.visible_lower_g_clefs,
            lower_f_clefs: self.lower_f_clefs,
        })
    }
}
pub fn inspect_clef_surface_span(
    source_xml: &str,
    source_start: usize,
    source_end: usize,
) -> Option<ClefSurfaceInspection> {
    let source_measure = source_xml.get(source_start..source_end)?;
    let (visible_lower_g_clefs, hidden_lower_g_clefs, lower_f_clefs) =
        inspect_measure_clefs(source_measure);
    Some(ClefSurfaceInspection {
        source_measure_fingerprint: fingerprint(source_measure),
        source_start,
        source_end,
        visible_lower_g_clefs,
        hidden_lower_g_clefs,
        lower_f_clefs,
    })
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ClefSurfaceMaterializationPlan {
    source_fingerprint: u64,
    source_start: usize,
    source_end: usize,
    expected_visible_lower_g_clefs: usize,
    lower_f_clefs: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct ClefSurfaceMaterializationStats {
    pub hidden_g_clef_blocks: usize,
    pub lower_f_clef_blocks_in_target: usize,
}

#[derive(Debug)]
pub struct PendingClefSurfaceMaterialization {
    source_document: String,
    candidate_xml: String,
    stats: ClefSurfaceMaterializationStats,
}

impl PendingClefSurfaceMaterialization {
    pub fn candidate_xml(&self) -> &str {
        &self.candidate_xml
    }

    pub fn stats(&self) -> ClefSurfaceMaterializationStats {
        self.stats
    }
    pub fn verify_sounding_invariants(
        self,
        visible_sounding_signature_stable: bool,
        note_events_before: usize,
        note_events_after: usize,
    ) -> Option<VerifiedClefSurfaceMaterialization> {
        if !visible_sounding_signature_stable
            || note_events_before != note_events_after
            || self.stats.hidden_g_clef_blocks == 0
            || self.stats.lower_f_clef_blocks_in_target == 0
        {
            return None;
        }
        Some(VerifiedClefSurfaceMaterialization {
            render_receipt_prepared: false,
            source_document: self.source_document,
            candidate_xml: self.candidate_xml,
        })
    }
}
#[derive(Debug)]
pub struct VerifiedClefSurfaceMaterialization {
    render_receipt_prepared: bool,
    source_document: String,
    candidate_xml: String,
}

pub fn project_clef_surface_plan(
    source_xml: &str,
    plan: ClefSurfaceMaterializationPlan,
) -> Option<PendingClefSurfaceMaterialization> {
    if fingerprint(source_xml) != plan.source_fingerprint {
        return None;
    }
    let source_measure = source_xml.get(plan.source_start..plan.source_end)?;
    let (replacement, hidden_g_clef_blocks) = hide_visible_lower_staff_g_clefs(source_measure);
    if plan.lower_f_clefs == 0
        || hidden_g_clef_blocks != plan.expected_visible_lower_g_clefs
        || replacement == source_measure
    {
        return None;
    }
    let output_end = plan.source_start.checked_add(replacement.len())?;
    let mut candidate_xml = String::with_capacity(
        source_xml.len().saturating_sub(plan.source_end - plan.source_start)
            + replacement.len(),
    );
    candidate_xml.push_str(&source_xml[..plan.source_start]);
    candidate_xml.push_str(&replacement);
    candidate_xml.push_str(&source_xml[plan.source_end..]);
    if !is_exact_replacement(
        source_xml,
        &candidate_xml,
        plan.source_start,
        plan.source_end,
        output_end,
    ) {
        return None;
    }
    Some(PendingClefSurfaceMaterialization {
        source_document: source_xml.to_string(),
        candidate_xml,
        stats: ClefSurfaceMaterializationStats {
            hidden_g_clef_blocks,
            lower_f_clef_blocks_in_target: plan.lower_f_clefs,
        },
    })
}

impl VerifiedClefSurfaceMaterialization {
    pub(crate) fn prepare_render_receipt(&mut self) -> Option<BlockTransformationReceipt> {
        self.render_receipt_prepared = true;
        BlockTransformationReceipt::mint_file_bound_block_receipt(RenderRow61BlockMint { _private: () }, &self.source_document, self.candidate_xml.clone(), Some((0, self.source_document.len())))
    }
}

pub fn write_clef_surface_materialization_authorized(
    output_path: &Path,
    candidate: VerifiedClefSurfaceMaterialization,
) -> (io::Result<()>, Option<BlockTransformationReceipt>) {
    let receipt_pre = candidate.source_document.clone();
    
    let output = {
    sink_surface_clef_candidate(output_path, &candidate.candidate_xml)
};
    
    let receipt = Some(candidate.candidate_xml.as_str()).filter(|post| !candidate.render_receipt_prepared && output.is_ok() && *post != receipt_pre.as_str()).map(|post| crate::master_state_identity::BlockTransformationReceipt::mint_block_receipt(RenderRow61BlockMint { _private: () }, receipt_pre.as_str(), post, Some((0, receipt_pre.as_str().len()))).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

fn inspect_measure_clefs(measure_text: &str) -> (usize, usize, usize) {
    let mut visible_lower_g = 0usize;
    let mut hidden_lower_g = 0usize;
    let mut lower_f = 0usize;
    for_each_clef(measure_text, |clef| {
        if !is_lower_staff_clef(clef) {
            return;
        }
        if clef.contains("<sign>F</sign>") {
            lower_f += 1;
        } else if clef.contains("<sign>G</sign>") {
            if clef_start_tag_has_print_object_no(clef) {
                hidden_lower_g += 1;
            } else {
                visible_lower_g += 1;
            }
        }
    });
    (visible_lower_g, hidden_lower_g, lower_f)
}
fn hide_visible_lower_staff_g_clefs(measure_text: &str) -> (String, usize) {
    let mut out = String::with_capacity(measure_text.len() + 64);
    let mut scan = 0usize;
    let mut hidden = 0usize;
    while let Some(relative_start) = measure_text[scan..].find("<attributes") {
        let start = scan + relative_start;
        let Some(relative_end) = measure_text[start..].find("</attributes>") else {
            break;
        };
        let end = start + relative_end + "</attributes>".len();
        let attributes = &measure_text[start..end];
        let (rewritten_attributes, hidden_in_block) =
            hide_visible_lower_staff_g_clefs_in_attributes(attributes);
        out.push_str(&measure_text[scan..start]);
        out.push_str(&rewritten_attributes);
        hidden += hidden_in_block;
        scan = end;
    }
    out.push_str(&measure_text[scan..]);
    (out, hidden)
}
fn hide_visible_lower_staff_g_clefs_in_attributes(attributes: &str) -> (String, usize) {
    let mut out = String::with_capacity(attributes.len() + 64);
    let mut scan = 0usize;
    let mut hidden = 0usize;
    while let Some(relative_start) = attributes[scan..].find("<clef") {
        let start = scan + relative_start;
        let Some(relative_end) = attributes[start..].find("</clef>") else {
            break;
        };
        let end = start + relative_end + "</clef>".len();
        let clef = &attributes[start..end];
        out.push_str(&attributes[scan..start]);
        if is_lower_staff_g_clef(clef) && !clef_start_tag_has_print_object_no(clef) {
            out.push_str(&add_print_object_no_to_clef_start_tag(clef));
            hidden += 1;
        } else {
            out.push_str(clef);
        }
        scan = end;
    }
    out.push_str(&attributes[scan..]);
    (out, hidden)
}
fn for_each_clef(text: &str, mut visit: impl FnMut(&str)) {
    let mut scan = 0usize;
    while let Some(relative_start) = text[scan..].find("<clef") {
        let start = scan + relative_start;
        let Some(relative_end) = text[start..].find("</clef>") else {
            break;
        };
        let end = start + relative_end + "</clef>".len();
        visit(&text[start..end]);
        scan = end;
    }
}
fn is_lower_staff_clef(clef: &str) -> bool {
    clef.contains("number=\"2\"") || clef.contains("number='2'")
}
fn is_lower_staff_g_clef(clef: &str) -> bool {
    is_lower_staff_clef(clef) && clef.contains("<sign>G</sign>")
}
fn clef_start_tag_has_print_object_no(clef: &str) -> bool {
    let Some(end) = clef.find('>') else {
        return false;
    };
    let start_tag = &clef[..=end];
    start_tag.contains("print-object=\"no\"") || start_tag.contains("print-object='no'")
}

fn add_print_object_no_to_clef_start_tag(clef: &str) -> String {
    let Some(end) = clef.find('>') else {
        return clef.to_string();
    };
    let start_tag = &clef[..end];
    if start_tag.contains("print-object=") {
        return clef.to_string();
    }
    format!("{} print-object=\"no\">{}", start_tag, &clef[end + 1..])
}

fn is_exact_replacement(
    before: &str,
    after: &str,
    source_start: usize,
    source_end: usize,
    output_end: usize,
) -> bool {
    source_start < source_end
        && source_end <= before.len()
        && source_start <= output_end
        && output_end <= after.len()
        && before.is_char_boundary(source_start)
        && before.is_char_boundary(source_end)
        && after.is_char_boundary(source_start)
        && after.is_char_boundary(output_end)
        && before[..source_start] == after[..source_start]
        && before[source_end..] == after[output_end..]
}



pub(crate) struct RenderRow61BlockMint { _private: () }

#[cfg(test)]
mod tests {
    use super::{inspect_clef_surface_span, project_clef_surface_plan, ClefSurfaceInspection};

    fn source(attributes: &str) -> String {
        format!("<score-partwise><part id=\"P1\"><measure number=\"1\">{attributes}<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note></measure></part></score-partwise>")
    }

    fn inspection(xml: &str) -> ClefSurfaceInspection {
        let start = xml.find("<measure").unwrap();
        let end = xml.find("</measure>").unwrap() + "</measure>".len();
        inspect_clef_surface_span(xml, start, end).unwrap()
    }

    fn plan(xml: &str) -> super::ClefSurfaceMaterializationPlan {
        inspection(xml)
            .hide_visible_lower_staff_g_preserve_f_context(xml)
            .unwrap()
    }

    #[test]
    fn prepared_surface_ticket_transports_the_exact_realized_post() {
        let xml = source("<attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>");
        let pending = project_clef_surface_plan(&xml, plan(&xml)).unwrap();
        let expected = pending.candidate_xml().to_string();
        let mut candidate = pending.verify_sounding_invariants(true, 1, 1).unwrap();
        let receipt = candidate.prepare_render_receipt().expect("real #61 proof");
        let (receipt, post) = receipt.split_realized_post_document();
        assert_eq!(post.as_deref(), Some(expected.as_str()));
        assert!(candidate.render_receipt_prepared);
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
        probe.push_block(Some(receipt), &expected);
        probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    }

    #[test]
    fn mixed_attributes_keep_bass_context_and_hide_only_lower_treble_surface() {
        let xml = source("<attributes><divisions>96</divisions><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>");
        let inspected = inspection(&xml);
        assert_eq!((inspected.visible_lower_g_clefs, inspected.hidden_lower_g_clefs, inspected.lower_f_clefs), (1, 0, 1));
        let candidate = project_clef_surface_plan(&xml, plan(&xml)).unwrap();
        assert!(candidate.candidate_xml().contains("<clef number=\"2\" print-object=\"no\"><sign>G</sign><line>2</line></clef>"));
        assert!(candidate.candidate_xml().contains("<clef number=\"2\"><sign>F</sign><line>4</line></clef>"));
        assert!(candidate.candidate_xml().contains("<clef number=\"1\"><sign>G</sign><line>2</line></clef>"));
    }

    #[test]
    fn production_contract_rejects_sounding_or_note_count_invariant_breaks() {
        let xml = source("<attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>");
        let plan = plan(&xml);
        assert!(project_clef_surface_plan(&xml, plan)
            .unwrap()
            .verify_sounding_invariants(false, 1, 1)
            .is_none());
        assert!(project_clef_surface_plan(&xml, plan)
            .unwrap()
            .verify_sounding_invariants(true, 1, 2)
            .is_none());
        assert!(project_clef_surface_plan(&xml, plan)
            .unwrap()
            .verify_sounding_invariants(true, 1, 1)
            .is_some());
    }

    #[test]
    fn inspection_without_bass_context_cannot_mint_a_plan() {
        let xml = source("<attributes><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>");
        assert!(inspection(&xml).hide_visible_lower_staff_g_preserve_f_context(&xml).is_none());
    }
}
