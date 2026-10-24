# r756 FrozenSoundingTemporalMaterializationTransportCutover

Parent: accepted/released r755.
ChangeClass: RefactorOnly.

Purpose: close the final data edge needed by the next atomic temporal MusicMutation without adding a writer or changing MusicXML. After `FinalSoundingMaterialFreeze` has sealed CandidateMaterialLineage and bound `FrozenSoundingTemporalLanePlan`, the existing `InitialRenderTransaction` now carries both the immutable lane plan and the final CandidateMaterial attack snapshot to the already-existing late FinalNotation/StructuralWriteAuthority frontier.

The transported state is populated only after the freeze succeeds. The pre-freeze transaction initializer contains empty/default transport values, then `complete_initial_render_transaction_stage` replaces them from the sealed lineage. The existing FinalRenderNotationAuthority input receives read-only references to both values. No new XML writer, no write call, no duration/tie/voice materialization, no selection and no threshold are introduced.

This take intentionally does not claim MusicXML VoiceAuthority or TieWriterAuthority. Its only productive change is the explicit immutable transport required so the next MusicMutation can materialize the already-bound 1,274-attack temporal truth without reaching back into private CandidateMaterialLineage state or inventing another audit bridge.
