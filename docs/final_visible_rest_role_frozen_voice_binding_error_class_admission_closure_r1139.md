# r1139 FinalVisibleRestRoleFrozenVoiceBindingErrorClassAdmissionClosure

Parent: accepted v0.10.1839 r1137 `FinalSerializedVoiceRestObligationWarningDebtClosure`. ChangeClass: `KodexOnly`.

## Purpose

r1139 reapplies only the required `causal_class_first_v1` foundation for the newly isolated FinalVisibleRestRole / final serialized voice-binding divergence.

The frozen contract is:

`docs/selector_derivation_contract_final_visible_rest_role_frozen_voice_binding_divergence.md`

This take changes no Rust product code, no MusicXML, no Render-MusicXML, no PDF output, no productive selector, no writer reachability, no threshold, no scoring, no release predicate and no committed-state veto.

## Recovery from rejected r1138

r1138 carried the same causal-class foundation and passed the take bootstrap integrity path through:

- dead-code integrity;
- change dependency preflight;
- semantic-capability diff;
- incoming take validation;
- PRE-TAKE-DIFF-GATE;
- transactional apply and verification.

It was rejected only at `reexec_exit` because the reexecution target was the still-failing countercheck whose defect r1138 intentionally did not repair. The transaction rolled back to accepted r1137. r1138 is therefore Failure Evidence and is not a parent.

r1139 is rebuilt directly from accepted r1137 with the same error-class semantics. Its acceptance run must use an already-green parent witness because this KodexOnly take is intentionally non-reparative. Acceptance proves admission and no regression; it does not claim the failing countercheck is repaired.

## Fresh runtime evidence

The countercheck on accepted r1137 reaches Build and strict Clippy successfully but fails closed before MuseScore/PDF in `FinalPdfSourceCommit`.

The important evidence is not the corpus coordinate. The causal sequence is:

- the later SilentVoice writer sees candidate materializations;
- all newly planned materializations are correctly withheld by the accepted canonical-shadow conflict protection;
- nevertheless the final persisted-source readback contains a pre-existing `CommittedShadowRestCandidate`;
- the persisted source is otherwise parse/cursor/pitched-timeline stable;
- therefore the residual candidate was not created by the SilentVoice materializations governed by that protection.

Static readback of the accepted r1137 source exposes the authority mismatch:

- `apply_final_visible_rest_role_purge_and_vertical_placement_write()` classifies productive rest roles with `atomic_visible_rest_is_shadow_bookkeeping_candidate()`, which consumes the physical XML voice labels available at that earlier stage;
- the committed FinalPdfSource veto classifies with `atomic_visible_rest_is_shadow_bookkeeping_candidate_with_voice_binding()`, which consumes the bound `FrozenSerializedVoicePlan`;
- later canonical serialized-voice materialization is allowed to change the relevant physical voice relation without changing pitched semantics.

Thus one rest can survive the productive role owner and only later become a positive canonical shadow-rest candidate under the authoritative final serialized voice relation.

## Frozen causal error class

The class is named:

`final_visible_rest_role_decision_diverges_across_final_serialized_voice_binding`

A member requires one surviving visible rest whose productive role decision occurs without the final authoritative voice relation, followed by an already-authorized final serialized voice materialization that preserves the pitched semantic timeline but changes the channel relation relevant to the canonical shadow-rest predicate, after which the committed bound predicate positively classifies the surviving rest as shadow bookkeeping.

This is distinct from the accepted earlier classes:

- visible-rest rhythmic-obligation loss concerns removal of an already-visible rhythmic obligation without sufficient dispensability proof;
- SilentVoice canonical-shadow conflict concerns creation of new visible rests from cursor carriers that immediately contradict canonical shadow truth;
- this class concerns an already-surviving visible rest whose canonical role truth changes across the final serialized-voice binding boundary because the productive owner and committed readback do not consume one identical authoritative voice relation.

## Scope boundary

r1139 does not repair the issue and does not decide whether the eventual closure should rebind the existing role owner, move/replay the existing canonical role decision after final serialized voice materialization, or use another already-authorized in-place projection point.

A future repair must keep the existing committed shadow-rest veto unchanged and must not add a parallel role classifier, a corpus selector, a new writer root, or a blanket secondary-rest purge.

## No corpus authority

No Etude identity, measure number, staff number, numeric voice label, pitch, exact duration, page/system position, candidate count or current-corpus uniqueness is part of the error class or future selector authority.

`CorpusDistributionPurpose` remains `not_yet_measured`.

## Intended next step after acceptance

After r1139 is accepted, derive the smallest repair scope that makes the productive FinalVisibleRestRole decision and final committed readback consume one authoritative final serialized voice relation. The preferred search space is the existing owner/order boundary; the committed veto stays fail-closed and the final pitched semantic timeline remains invariant.
