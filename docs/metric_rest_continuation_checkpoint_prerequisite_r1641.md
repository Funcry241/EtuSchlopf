# Metric rest continuation checkpoint prerequisite

This KodexOnly revision has no runtime source changes. It binds the three
protected Checkpoint58 successor files and the exact removal of the E050
containment envelope to the exact images below. The
parent-compiled transition accepts them together only for r1641 to r1642
(MetricRestContinuationCutover, GovernedChange). The earlier historical
authorization retains its own identity and exact images.

The proposed checkpoint persists a verified post only through sovereign
adoption. It accepts one original E053 metric-rest-bridge receipt. The
ordinary adoption chain verifies the current source and the post before
persistence, then witnesses and commits them. The physical sink can only be
called with the adoption boundary capability. The old exported checkpoint,
legacy owner and E050 mint must be absent from runtime sources. The
successor declares the following narrow change for the removed envelope.
`govern=envelope_narrow|STRUCTURAL_REST_BRIDGE_VARIANT_A_PRESERVING|Retire the independent E050 owner`.
The usual
admission checks still govern the rest of the successor.

Successor corpus acceptance requires all ten Master and ten Render MusicXML
documents to equal accepted r1640 byte for byte. The evidence must show
coverage of the entire previous E050 marker-changing set of 46 note rows,
including the four formerly provisional bridges, under the surviving
metric owner. The native corpus must also preserve the 683 original edge
proofs and contain no committed LowerDualEdge candidate. Corpus identities
are verification witnesses only.

The reproducible protected source diff follows. The remaining changes for
the successor need their own implementation and verification.

```diff
--- a/src/build_gate/ssc_containment_envelopes_v1.tsv
+++ b/src/build_gate/ssc_containment_envelopes_v1.tsv
@@ -11,7 +11,6 @@
 row=PARTIAL_BEAT_BEAM_PRESERVING|Preserving|Projection|beam|-|change,insert,remove|owner_conserved_projection|no|E053 rebuilds partial-beat post-rest beams only
 row=PARTIAL_BEAT_LANE_TRANSFER_PRESERVING|Preserving|Projection|note,forward,backup|-|change,insert,remove|owner_conserved_projection|yes|content: E057 exchanges forward and note serialization between cursor lanes with zero event multiset delta
 row=COMMITTED_CROSS_STAFF_BEAM_PRESERVING|Preserving|Projection|beam|-|change,insert,remove|owner_conserved_projection|no|E049 closes committed cross-staff beam gestures only
-row=STRUCTURAL_REST_BRIDGE_VARIANT_A_PRESERVING|Preserving|Projection|beam|-|change,insert,remove|owner_conserved_projection|no|E050 beam-only render write across structural rest bridges
 row=OCTAVE_WORD_LABEL_PRESERVING|Preserving|Projection|direction,direction-type,words|-|change,remove|owner_conserved_projection|no|E055 removes generated 8va/15ma word labels and emptied directions
 row=VISUAL_LAYOUT_DEFAULTS_PRESERVING|Preserving|Projection|defaults,scaling,millimeters,tenths,page-layout,page-height,page-width,page-margins,left-margin,right-margin,top-margin,bottom-margin,system-layout,system-margins,system-distance,top-system-distance,staff-layout,staff-distance,appearance,line-width,note-size,distance,music-font,word-font,lyric-font|-|change,insert|owner_conserved_projection|no|E044 inserts the visual layout defaults block only
 row=LOCAL_STAFF_DISTANCE_PRESERVING|Preserving|Projection|print,staff-layout,staff-distance|-|change,insert|owner_conserved_projection|no|E013 local staff-distance layout only
--- a/src/structural_write_authority/master_adoption/render_adopted_state.rs
+++ b/src/structural_write_authority/master_adoption/render_adopted_state.rs
@@ -55,7 +55,7 @@
     VisibleRestRole,
     FinalPianoRestGlyph,
 
-    SisterFeedbackBeam,
+    MetricRestContinuation,
     ExternalFeedbackClosure,
     KeySignatureSpelling,
     DirectionRelocation,
@@ -645,6 +645,33 @@
         let verified = verified.extend_with_sink_owner(&COMMIT_BOUNDARY, sink_receipt, &label);
         self.witness(Some(verified.candidate()), &label);
         self.commit(verified, sink, &label);
+        Ok(())
+    }
+
+    /// Persists only a replayable, owner-authored metric rest continuation.
+    pub(crate) fn adopt_metric_rest_continuation(
+        &mut self,
+        root: RenderTrialRoot,
+        candidate: &str,
+        receipt: crate::master_state_identity::PreservingTransformationReceipt,
+    ) -> anyhow::Result<()> {
+        let mut receipts = RenderTransitionReceipts::new();
+        let is_metric = receipt.is_metric_rest_bridge()
+            && receipt.binds_recorded_representation(root.document(), candidate).is_ok();
+        receipts.push_preserving(Some(receipt), candidate);
+        if !is_metric {
+            receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
+            anyhow::bail!("Metric rest continuation requires its original recorded metric receipt");
+        }
+        self.adopt_transition(
+            root, candidate, receipts,
+            RenderSinkKind::Checkpoint(RenderCheckpointSink::MetricRestContinuation),
+            |path, _pre, verified| {
+                super::super::orchestrator_artifact_sink_authority::write_metric_rest_continuation_checkpoint(
+                    &COMMIT_BOUNDARY, path, verified,
+                )
+            },
+        )?;
         Ok(())
     }
 
--- a/src/structural_write_authority/mod.rs
+++ b/src/structural_write_authority/mod.rs
@@ -464,7 +464,6 @@
     write_master_musicxml_output_authorized,
     write_result_report_output_authorized,
     write_retention_return_checkpoint_authorized,
-    write_sister_feedback_beam_checkpoint_authorized,
     write_visible_rest_role_checkpoint_authorized,
 
 };
--- a/src/structural_write_authority/orchestrator_artifact_sink_authority.rs
+++ b/src/structural_write_authority/orchestrator_artifact_sink_authority.rs
@@ -154,7 +154,7 @@
     FinalPianoRestGlyph,
 
     
-    SisterFeedbackBeam,
+    MetricRestContinuation,
     ExternalFeedbackClosure,
     KeySignatureSpelling,
     DirectionRelocation,
@@ -172,7 +172,7 @@
             Self::FinalPianoRestGlyph => "final_piano_rest_glyph",
 
             
-            Self::SisterFeedbackBeam => "sister_feedback_beam",
+            Self::MetricRestContinuation => "metric_rest_continuation",
             Self::ExternalFeedbackClosure => "external_feedback_closure",
             Self::KeySignatureSpelling => "key_signature_spelling",
             Self::DirectionRelocation => "direction_relocation",
@@ -540,7 +540,15 @@
 render_checkpoint_entry!(write_final_piano_rest_glyph_checkpoint_authorized, FinalPianoRestGlyph);
 
 
-render_checkpoint_entry!(write_sister_feedback_beam_checkpoint_authorized, SisterFeedbackBeam);
+pub(super) fn write_metric_rest_continuation_checkpoint(
+    _boundary: &super::RenderAdoptionCommitBoundary,
+    path: &Path,
+    verified: &str,
+) -> (io::Result<()>, Option<ExactTransformationReceipt>) {
+    let result = OrchestratedRenderCheckpointCapability::issue(path, verified, RenderCheckpointStage::MetricRestContinuation)
+        .and_then(|capability| capability.write());
+    (result, None)
+}
 render_checkpoint_entry!(write_external_feedback_closure_checkpoint_authorized, ExternalFeedbackClosure);
 render_checkpoint_entry!(write_key_signature_spelling_checkpoint_authorized, KeySignatureSpelling);
 render_checkpoint_entry!(write_direction_relocation_checkpoint_authorized, DirectionRelocation);
```
