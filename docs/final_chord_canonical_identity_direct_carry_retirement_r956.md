# r956 – FinalChordCanonicalIdentityDirectCarryRetirement

## Authority

- Version: `0.10.1839`
- Revision: `r956`
- Authority parent: `r953 FinalChordExactPruneDispositionRetirement`
- Change class: `MusicMutation`
- Rejected `r954` and failed `r955` are Failure Evidence only and are not parents of this Take.
- The r955 non-tie-notation guard experiment is not carried forward; r956 starts from the exact r953 source state.

## Problem

The productive FinalChord path already created a complete canonical structural plan while deciding the projection, but discarded that plan after byte projection. Freeze then rebuilt FinalChord structure from projected XML by transporting a second projected identity receipt, parsing the projection again, classifying host/chord topology again, and sealing a second `CanonicalFinalChordStructuralPlan`.

That is the inverse-information pattern r953 set out to retire:

```text
precise authority exists
-> discard
-> reconstruct from weaker/downstream representation
-> duplicate ownership and proof surface
```

The failed Etüde-1 witness at measure 11 / onset 192 also showed that chasing another local guard would continue the same pattern instead of reducing ownership.

## Cut

r956 binds the existing `CandidateMaterialLineageSpine` opaque identity (`FragmentId`, `SourceEventId`, `SoundingAttackId`) directly into `plan_canonical_final_chord_structure_once(...)` before FinalChord projection.

The resulting canonical structural plan and the encoding plan are produced in the same decision pass. The encoding plan is projected through the existing sole byte projector. The canonical structural plan is carried directly into final sounding freeze and then into `build_frozen_serialization_voice_channels_for_canonical(...)`.

Removed from the productive authority corridor:

- `CandidateFinalChordProjectionIdentityReceiptRow`
- projected-XML -> CandidateMaterial representation reconstruction
- projected chord-host reconstruction
- projected tie-continuation reconstruction
- `ProjectedFinalChordRepresentationRow`
- `seal_final_chord_structure_from_projection(...)`
- the wrapper that rebuilt canonical structure before frozen voice-channel construction
- tests that existed only to exercise that retired reverse-seal path

The read-only Final Report convenience planner remains; it is diagnostic and does not own freeze/write authority.

## Existing fail-closed boundaries retained

No validator or gate is relaxed.

`project_final_chord_encoding_plan_with_provenance(...)` remains the sole FinalChord byte projection and still verifies:

- source plan fingerprint/length binding through the underlying projector,
- every projected pitched note has exactly one projection provenance,
- generated note spans remain exact,
- no projected pitched occurrence has duplicate provenance,
- generated fragment counts match the planned materialization,
- generated fragment ordinals are complete.

The orchestrator still resolves every provenance row back to the exact CandidateMaterial fragment and checks SourceEvent/SoundingAttack agreement.

`freeze_final_sounding_material(...)` still compares the complete sounding projection derived from CandidateMaterialState with the actual projected XML and fails closed on any difference before freezing.

The temporal validator is unchanged. No duration is forced from 192 to 96. No writer, repair authority, recovery authority, receipt class, fingerprint class, or new proof layer is added.

## Why MusicMutation

The cut is primarily retirement, but it is deliberately classified `MusicMutation` rather than `RefactorOnly`: the first FinalChord classification now receives the real CandidateMaterial opaque identity instead of synthetic XML-declassified identities. Existing same-pitch reattack logic may therefore distinguish or unify attacks differently when CandidateMaterial already knows they are the same/different sounding event. That semantic possibility is explicit and must be tested by the real Windows gate.

## Source balance

Before Take metadata/documentation, Rust source delta versus r953:

```text
+21
-1365
-----
-1344 lines
```

The dominant deletion is the downstream projected-representation reconstruction and second canonical seal.

## Acceptance intent

The first acceptance target should be the known-green Etüde 6. This establishes whether the authority retirement preserves the accepted FinalChord behavior without making Etüde 1 responsible for accepting the architecture change.

Only after r956 is accepted from Etüde 6 should Etüde 1 be rerun to see whether real opaque identity plus single-pass canonical carry changes the measure-11/onset-192 witness.

If r956 fails before or during the known-green Etüde-6 gate, it is Failure Evidence and r953 remains Authority. Do not stack a local repair on top of the rejected candidate.
