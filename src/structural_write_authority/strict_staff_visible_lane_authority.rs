//! Typed central authority for strict-staff visible-lane edits.
//!
//! Callers retain every musical and notational selection decision. This module
//! only consumes exact source/range-bound edit plans for already selected
//! visible-lane, rest-ledger, duplicate-note, tie and articulation fragments.
//! It deliberately exposes no general raw-XML writer.

use super::destructive_edit_attribution::{DestructiveEditBatchAttribution, IndividualEditIdentity};
use super::authority_primitives::{
    fingerprint,
    syntactic_exact_note as exact_note,
    syntactic_exact_rest_note as exact_rest_note,
    xml_fragment_parses,
    child_text,
    child_block,
    contains_document_boundary,
};
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisibleLaneEffect {
    BackupAnchor,
    HiddenRestVisibility,
    MissingStaffEdgeRest,
                
                          
                
                          
    DuplicateSamePitchSuppression,
                
                                
    RestBeatMaterialization,
                
                      
    EmptyVoiceRestSuppression,
    LowerVisibleRestSuppression,
                
                               
    UpperDuplicateRestSuppression,
    UpperInitialRestCoalescing,
}

#[derive(Debug)]
struct BoundFragmentEdit {
    identity: IndividualEditIdentity,
    start: usize,
    end: usize,
    original: String,
    original_fingerprint: u64,
    replacement: String,
}

#[derive(Debug)]
struct VisibleLaneEditBatch {
    source: String,
    source_fingerprint: u64,
    effect: VisibleLaneEffect,
    edits_descending: Vec<BoundFragmentEdit>,
    attribution: DestructiveEditBatchAttribution,
}

impl VisibleLaneEditBatch {
    fn issue(
        source: &str,
        edits: Vec<(usize, usize, String)>,
        effect: VisibleLaneEffect,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        let mut bound = Vec::with_capacity(edits.len());
        for (ordinal, (start, end, replacement)) in edits.into_iter().enumerate() {
            let identity = attribution.identity(ordinal)?;
            if start > end || end > source.len() {
                return None;
            }
            let original = source.get(start..end)?.to_string();
            let insertion = start == end;
            if effect == VisibleLaneEffect::EmptyVoiceRestSuppression
                && !insertion
                && exact_rest_note(&original)
                && (replacement.trim().is_empty() || exact_forward(&replacement))
            {
                let following = source.get(end..)?.trim_start();
                if following.starts_with("<note") {
                    let following_end = following.find("</note>")? + "</note>".len();
                    let following_note = following.get(..following_end)?;
                    if exact_note(following_note) && following_note.contains("<chord") {
                        return None;
                    }
                }
            }
            if !edit_allowed(effect, &original, &replacement, insertion) {
                return None;
            }
            bound.push(BoundFragmentEdit {
                identity,
                start,
                end,
                original_fingerprint: fingerprint(&original),
                original,
                replacement,
            });
        }
        bound.sort_by_key(|left| std::cmp::Reverse(left.start));
        for pair in bound.windows(2) {
            let later = &pair[0];
            let earlier = &pair[1];
            let both_replace_nonempty_ranges = later.start < later.end && earlier.start < earlier.end;
            if both_replace_nonempty_ranges && earlier.end > later.start {
                return None;
            }
        }
        for edit in &bound {
            attribution.mark_bound(edit.identity);
        }
        Some(Self {
            source: source.to_string(),
            source_fingerprint: fingerprint(source),
            effect,
            edits_descending: bound,
            attribution,
        })
    }

    fn apply(self, source: &str) -> Option<String> {
        let Self {
            source: bound_source,
            source_fingerprint,
            effect,
            edits_descending,
            attribution,
        } = self;
        if bound_source != source || source_fingerprint != fingerprint(source) {
            return None;
        }
        let identities = edits_descending
            .iter()
            .map(|edit| edit.identity)
            .collect::<Vec<_>>();
        let mut attributed_xml = bound_source;
        for edit in edits_descending {
            let current = attributed_xml.get(edit.start..edit.end)?;
            if current != edit.original
                || fingerprint(current) != edit.original_fingerprint
                || !edit_allowed(
                    effect,
                    current,
                    &edit.replacement,
                    edit.start == edit.end,
                )
            {
                return None;
            }
            attributed_xml.replace_range(edit.start..edit.end, &edit.replacement);
        }
        attribution.mark_applied_batch(&identities, source, &attributed_xml);
        Some(attributed_xml)
    }
}

fn edit_allowed(
    effect: VisibleLaneEffect,
    original: &str,
    replacement: &str,
    insertion: bool,
) -> bool {
    if contains_document_boundary(replacement)
        || (!replacement.is_empty() && !xml_fragment_parses(replacement))
    {
        return false;
    }
    match effect {
        VisibleLaneEffect::BackupAnchor => {
            !insertion
                && exact_backup(original)
                && exact_backup(replacement)
                && child_text(original, "duration").is_some()
                && child_text(replacement, "duration").is_some()
        }
        VisibleLaneEffect::HiddenRestVisibility => {
            !insertion
                && exact_rest_note(original)
                && exact_rest_note(replacement)
                && preserved_note_ledger(original, replacement)
        }
        VisibleLaneEffect::MissingStaffEdgeRest => {
            insertion
                && !replacement.trim().is_empty()
                && replacement.contains("<rest")
                && (replacement.contains("<backup") || replacement.contains("<note"))
        }
        VisibleLaneEffect::EmptyVoiceRestSuppression
        | VisibleLaneEffect::LowerVisibleRestSuppression
        | VisibleLaneEffect::UpperDuplicateRestSuppression => {
            !insertion
                && exact_rest_note(original)
                && (replacement.is_empty()
                    || exact_forward(replacement)
                    || exact_rest_note(replacement))
        }
                    
                                                
                                                             
                      
                                            
                                          
                                                 
                                                    
         
                    
                                                     
                      
                                       
                                          
                                                                        
         
        VisibleLaneEffect::DuplicateSamePitchSuppression => {
            !insertion
                && exact_note(original)
                && exact_note(replacement)
                && preserved_sounding_note_ledger(original, replacement)
        }
        VisibleLaneEffect::RestBeatMaterialization => {
            !insertion
                && rest_timeline_fragment(original)
                && (replacement.is_empty() || rest_timeline_fragment(replacement))
        }
                    
                                                          
                      
                                                   
                                                                                  
         
        VisibleLaneEffect::UpperInitialRestCoalescing => {
            if insertion {
                !replacement.trim().is_empty()
                    && replacement.contains("<rest")
                    && rest_timeline_fragment(replacement)
            } else {
                rest_timeline_fragment(original)
                    && (replacement.is_empty() || rest_timeline_fragment(replacement))
            }
        }
                    
                                                 
                      
                                       
                                          
                                               
                                                                               
         
    }
}





fn exact_forward(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.starts_with("<forward")
        && trimmed.ends_with("</forward>")
        && trimmed.contains("<duration>")
        && trimmed.matches("</forward>").count() == 1
}

fn exact_backup(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.starts_with("<backup")
        && trimmed.ends_with("</backup>")
        && trimmed.contains("<duration>")
        && trimmed.matches("</backup>").count() == 1
}

fn rest_timeline_fragment(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.is_empty()
        || exact_rest_note(trimmed)
        || exact_forward(trimmed)
        || exact_backup(trimmed)
        || (xml_fragment_parses(trimmed)
            && (trimmed.contains("<rest")
                || trimmed.contains("<forward")
                || trimmed.contains("<backup")))
}

fn preserved_note_ledger(before: &str, after: &str) -> bool {
    ["duration", "voice", "staff", "type"]
        .iter()
        .all(|tag| child_text(before, tag) == child_text(after, tag))
}

fn preserved_pitch_voice_staff(before: &str, after: &str) -> bool {
    child_block(before, "pitch") == child_block(after, "pitch")
        && child_text(before, "voice") == child_text(after, "voice")
        && child_text(before, "staff") == child_text(after, "staff")
}

fn preserved_sounding_note_ledger(before: &str, after: &str) -> bool {
    preserved_pitch_voice_staff(before, after)
        && child_text(before, "duration") == child_text(after, "duration")
        && child_text(before, "type") == child_text(after, "type")
        && before_has_same_chord_role(before, after)
}

fn before_has_same_chord_role(before: &str, after: &str) -> bool {
    before.contains("<chord") == after.contains("<chord")
}




            
                                           
                                             
                       
                              
                         
                                               
                             
                             
                                                                                          
                                
                  
                                                                                           
         
      
 

#[track_caller]
pub fn apply_backup_anchor_rewrite_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> (Option<String>, Option<crate::master_state_identity::ExactTransformationReceipt>) {
    let geometry = edits.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
        let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(edits.len());
        VisibleLaneEditBatch::issue(source, edits, const { VisibleLaneEffect::BackupAnchor }, attribution)?.apply(source)
    })();
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(super::VisibleLane41RenderExactMint, source, post, geometry).expect("EXACT owner changed bytes without receipt"));
    (output, receipt)
}
#[track_caller]
pub fn apply_hidden_rest_visibility_rewrite_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> (Option<String>, Option<crate::master_state_identity::ExactTransformationReceipt>) {
    let geometry = edits.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
        let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(edits.len());
        VisibleLaneEditBatch::issue(source, edits, VisibleLaneEffect::HiddenRestVisibility, attribution)?.apply(source)
    })();
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(super::VisibleLane44RenderExactMint, source, post, geometry).expect("EXACT owner changed bytes without receipt"));
    (output, receipt)
}
            
                              
                                                    
                                            
  
            
                              
                                              
                                            
  
#[track_caller]
pub fn apply_duplicate_same_pitch_suppression_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> (Option<String>, Option<crate::master_state_identity::ExactTransformationReceipt>) {
    let geometry = edits.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
        let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(edits.len());
        VisibleLaneEditBatch::issue(source, edits, VisibleLaneEffect::DuplicateSamePitchSuppression, attribution)?.apply(source)
    })();
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(super::VisibleLane42RenderExactMint, source, post, geometry).expect("EXACT owner changed bytes without receipt"));
    (output, receipt)
}
            
                              
                                                  
                                                  
  
#[track_caller]
pub fn apply_rest_beat_materialization_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> (Option<String>, Option<crate::master_state_identity::ExactTransformationReceipt>) {
    let geometry = edits.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
        let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(edits.len());
        VisibleLaneEditBatch::issue(source, edits, VisibleLaneEffect::RestBeatMaterialization, attribution)?.apply(source)
    })();
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(super::VisibleLane46RenderExactMint, source, post, geometry).expect("EXACT owner changed bytes without receipt"));
    (output, receipt)
}
            
                              
                                          
                                        
  
#[track_caller]
pub fn apply_empty_voice_rest_suppression_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> (Option<String>, Option<crate::master_state_identity::ExactTransformationReceipt>) {
    let geometry = edits.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
        let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(edits.len());
        VisibleLaneEditBatch::issue(source, edits, VisibleLaneEffect::EmptyVoiceRestSuppression, attribution)?.apply(source)
    })();
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(super::VisibleLane43RenderExactMint, source, post, geometry).expect("EXACT owner changed bytes without receipt"));
    (output, receipt)
}
#[track_caller]
pub fn apply_lower_visible_rest_suppression_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> (Option<String>, Option<crate::master_state_identity::ExactTransformationReceipt>) {
    let geometry = edits.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
        let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(edits.len());
        VisibleLaneEditBatch::issue(source, edits, VisibleLaneEffect::LowerVisibleRestSuppression, attribution)?.apply(source)
    })();
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(super::VisibleLane45RenderExactMint, source, post, geometry).expect("EXACT owner changed bytes without receipt"));
    (output, receipt)
}
            
                              
                                                    
                                                 
  
#[track_caller]
pub fn apply_upper_duplicate_rest_suppression_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> (Option<String>, Option<crate::master_state_identity::ExactTransformationReceipt>) {
    let geometry = edits.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
        let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(edits.len());
        VisibleLaneEditBatch::issue(source, edits, VisibleLaneEffect::UpperDuplicateRestSuppression, attribution)?.apply(source)
    })();
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(super::VisibleLane47RenderExactMint, source, post, geometry).expect("EXACT owner changed bytes without receipt"));
    (output, receipt)
}
#[track_caller]
pub fn apply_upper_initial_rest_coalescing_authorized(source: &str, edits: Vec<(usize, usize, String)>) -> (Option<String>, Option<crate::master_state_identity::ExactTransformationReceipt>) {
    let geometry = edits.iter().map(|(start, end, replacement)| crate::master_state_identity::ExactEdit::bound(*start, *end, replacement.clone())).collect();
    let output = (|| {
        let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(edits.len());
        VisibleLaneEditBatch::issue(source, edits, VisibleLaneEffect::UpperInitialRestCoalescing, attribution)?.apply(source)
    })();
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(super::VisibleLane48RenderExactMint, source, post, geometry).expect("EXACT owner changed bytes without receipt"));
    (output, receipt)
}

use crate::master_state_identity::ExactTransformationReceipt;

#[track_caller]
pub fn insert_missing_staff_edge_rest_authorized(
    source: &str,
    insertions: Vec<(usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    
    let geometry = insertions.iter().map(|(position, replacement)| crate::master_state_identity::ExactEdit::bound(*position, *position, replacement.clone())).collect();
    let output = (|| {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        insertions.len(),
    );
    let edits = insertions
        .into_iter()
        .map(|(position, replacement)| (position, position, replacement))
        .collect();
    VisibleLaneEditBatch::issue(
        source,
        edits,
        VisibleLaneEffect::MissingStaffEdgeRest,
        attribution,
    )?
    .apply(source)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow51ExactMint { _private: () }, source, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

#[derive(Debug)]
struct CrossStaffHandRoleTrialSink {
    source_fingerprint: u64,
    candidate: String,
    candidate_fingerprint: u64,
    output_path: PathBuf,
}

impl CrossStaffHandRoleTrialSink {
    fn issue(source: &str, candidate: &str, output_path: &Path) -> Result<Self> {
        if candidate.trim().is_empty() || roxmltree::Document::parse(candidate).is_err() {
            bail!("cross_staff_hand_role_trial: candidate is not a complete parseable XML document");
        }
        Ok(Self {
            source_fingerprint: fingerprint(source),
            candidate: candidate.to_string(),
            candidate_fingerprint: fingerprint(candidate),
            output_path: output_path.to_path_buf(),
        })
    }

    fn commit(self, source: &str, candidate: &str, output_path: &Path) -> Result<()> {
        if self.source_fingerprint != fingerprint(source)
            || self.candidate != candidate
            || self.candidate_fingerprint != fingerprint(candidate)
            || self.output_path.as_path() != output_path
        {
            bail!("cross_staff_hand_role_trial: source, candidate or output path binding changed");
        }
        let candidate_xml = self.candidate;
        let writer_trace_before = crate::audit_only::writer_trace::capture_before(&self.output_path);
        std::fs::write(&self.output_path, candidate_xml.as_bytes()).with_context(|| {
            format!(
                "cross_staff_hand_role_trial: could not write {}",
                self.output_path.display()
            )
        })?;
        let readback = std::fs::read(&self.output_path).with_context(|| {
            format!(
                "cross_staff_hand_role_trial: could not read back {}",
                self.output_path.display()
            )
        })?;
        if readback != candidate_xml.as_bytes()
            || fingerprint(std::str::from_utf8(&readback).unwrap_or_default())
                != self.candidate_fingerprint
        {
            bail!("cross_staff_hand_role_trial: exact file readback mismatch");
        }
        crate::audit_only::writer_trace::record_after(
            "cross_staff_hand_role_trial",
            &self.output_path,
            writer_trace_before.as_deref(),
            &candidate_xml,
        );
        Ok(())
    }
}

pub fn write_cross_staff_hand_role_trial_authorized(
    source: &str,
    candidate: &str,
    output_path: &Path,
) -> Result<()> {
    CrossStaffHandRoleTrialSink::issue(source, candidate, output_path)?
        .commit(source, candidate, output_path)
}

pub(crate) struct RenderRow51ExactMint { _private: () }

#[cfg(test)]
mod normalized_macro_receipt_tests {
    use super::*;
    type Owner = fn(&str, Vec<(usize, usize, String)>) -> (Option<String>, Option<crate::master_state_identity::ExactTransformationReceipt>);
    fn legacy(source: &str, edits: Vec<(usize, usize, String)>, effect: VisibleLaneEffect) -> Option<String> {
        let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(edits.len());
        VisibleLaneEditBatch::issue(source, edits, effect, attribution)?.apply(source)
    }
           
                                                                    
                                                                                                  
                                                                                                      
                      
                                                                                                                       
                                                      
                                                          
                                                  
                                                            
           
                                                                           
                                                                                                  
                                                                                              
         
     
    #[test]
    fn all_eight_owners_replay_their_own_unchanged_legacy_event() {
        let rest = "<note><rest/><duration>4</duration><voice>1</voice><staff>1</staff></note>";
        let hidden_rest = "<note print-object=\"no\"><rest/><duration>4</duration><voice>1</voice><staff>1</staff></note>";
        let note = "<note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>";
        let hidden_note = "<note print-object=\"no\"><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>";
        let forward = "<forward><duration>4</duration><voice>1</voice><staff>1</staff></forward>";
        let backup = "<backup><duration>4</duration></backup>";
        let new_backup = "<backup><duration>8</duration></backup>";
        for (row, owner, effect, original, replacement) in [(41, apply_backup_anchor_rewrite_authorized as Owner, VisibleLaneEffect::BackupAnchor, backup, new_backup),
(42, apply_duplicate_same_pitch_suppression_authorized as Owner, VisibleLaneEffect::DuplicateSamePitchSuppression, note, hidden_note),
(43, apply_empty_voice_rest_suppression_authorized as Owner, VisibleLaneEffect::EmptyVoiceRestSuppression, rest, forward),
(44, apply_hidden_rest_visibility_rewrite_authorized as Owner, VisibleLaneEffect::HiddenRestVisibility, rest, hidden_rest),
(45, apply_lower_visible_rest_suppression_authorized as Owner, VisibleLaneEffect::LowerVisibleRestSuppression, rest, forward),
(46, apply_rest_beat_materialization_authorized as Owner, VisibleLaneEffect::RestBeatMaterialization, rest, forward),
(47, apply_upper_duplicate_rest_suppression_authorized as Owner, VisibleLaneEffect::UpperDuplicateRestSuppression, rest, forward),
(48, apply_upper_initial_rest_coalescing_authorized as Owner, VisibleLaneEffect::UpperInitialRestCoalescing, rest, forward)] {
            let source = format!("é{original}終");
            let begin = "é".len();
            let end = begin + original.len();
            for edits in [Vec::new(), vec![(begin,end,original.to_string())], vec![(begin,end,replacement.to_string())], vec![(source.len()+1,source.len()+2,replacement.to_string())], vec![(end,begin,replacement.to_string())]] {
                let expected = legacy(&source, edits.clone(), effect);
                let (post, receipt) = owner(&source, edits);
                assert_eq!(post, expected, "row {row}");
                assert_eq!(receipt.is_some(), post.as_deref().is_some_and(|p| p != source));
                if let Some(receipt) = receipt {
                    assert_eq!(receipt.descriptor_row(), row);
                    assert_eq!(receipt.realize_post_document(&source), post);
                    assert!(receipt.realize_post_document("wrong pre").is_none());
                }
            }
            let (_, receipt) = owner(&source, vec![(begin,end,replacement.to_string())]);
            assert!(receipt.is_some(), "positive write row {row}");
        }
    }
}
