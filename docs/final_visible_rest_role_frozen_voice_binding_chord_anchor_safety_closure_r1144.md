# r1144 FinalVisibleRestRoleFrozenVoiceBindingChordAnchorSafetyClosure

Parent: accepted v0.10.1839 r1139 `FinalVisibleRestRoleFrozenVoiceBindingErrorClassAdmissionClosure`. ChangeClass: `MusicMutation`.

## Purpose

Accepted r1139 freezes the causal class `final_visible_rest_role_decision_diverges_across_final_serialized_voice_binding`. Rejected r1143 then reached productive execution and proved that the late FrozenVoice-bound replay removes the previously committed shadow candidate (`candidates 1→0`) through the existing StructuralWriteAuthority, with parse, sounding and raw-cursor invariants green. The following existing FinalChord occurrence-continuation verifier nevertheless stopped fail-closed because the removed rest was also a physical MusicXML anchor for a following `<chord/>` member.

This is not a new causal class. Accepted r1136 already established the general safety rule that a non-chord rest which physically anchors any parsed `<chord/>` member must not be replaced by a sequence containing `<forward>`, because that resets parser anchor state and can move or re-identify a pitched occurrence.

## Productive change

r1144 retains the r1143 late FrozenVoice-bound replay architecture but aligns its local selector with the already accepted r1136 chord-anchor safety rule. A visible rest is eligible for the bound shadow purge only when it is not the physical anchor of an immediately following parsed `<chord/>` note.

The same predicate is consumed by both the late replacement selector and the committed FinalPdfSource shadow-rest readback. Therefore a structurally obligated chord-anchor carrier is neither rewritten nor falsely reported as an unresolved purge candidate.

No project, etude, real measure, pitch, staff number, duration or corpus candidate count participates in the selector.

## Authority boundary

The r1143 authority closure is preserved: selection is pure; replacement materialization remains exclusively inside existing `StructuralWriteAuthority::apply_final_visible_rest_role_rewrite_authorized()`; persistence remains the existing `write_final_notation_checkpoint_authorized()` sink; the committed veto is not bypassed. No noncanonical writer root is introduced.

## Invariants

All r1143 pre-checkpoint invariants remain unchanged. In addition, chord-anchor topology is now a selector precondition inherited from r1136. The existing FinalChord occurrence-continuation verifier remains unchanged and independently fail-closed.

## Synthetic regression

The existing relocatable symbolic-measure test keeps its positive non-anchor case and adds a second synthetic case in the same test body: a secondary bound visible rest immediately followed by a `<chord/>` pitched member yields zero replacements. The fixture uses measure identifier `x` and synthetic labels only.

## Expected runtime consequence

The previously observed bound candidate that is structurally obligated as a chord carrier is classified as non-purgeable before any write. Safe non-anchor candidates remain eligible for the r1143 replay. The committed detector uses the same safety-complete predicate, so it cannot demand a transformation that the accepted r1136 topology rule forbids.
