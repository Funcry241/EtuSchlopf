# r1067 - PrimaryBeamSerializationVoicePartitionLateRebindMaterialization

## Parent and authority

Parent is the canonically accepted r1066 `PrimaryBeamVoicePartitionWriterOutcomeProjectionClosure` candidate with SHA-256 `d533fc75195917608db9efd254cac377b85a2aa738bb2648fddb2f93cc27f83f`.

r1066 is AuditOnly and adds only the typed manifest projection `PrimaryBeamVoiceElementsWritten`; it adds no writer and changes no music. Its accepted full-corpus run leaves that field absent/zero and proves the projection remains backward-compatible.

The productive musical scope is still the already accepted r1061 repair scope, with the accepted r1065 late-rebind evidence as the current identity proof. All rejected r1062 and rejected r1066 MusicMutation candidates are runtime evidence only; none is parent or selector authority.

## Productive rule

Selection is recomputed forward from current typed state. A metric line may enter only when all of the following independently hold:

1. it binds to exactly one current closed same-line candidate with the same staff and members;
2. the existing exact pre-final primary-beam proof-chain identity is present;
3. exactly one current canonical primary-beam owner receipt binds that proof chain and its canonical primary states match the current members;
4. the current MaterializationSafetyConjunction proves that opaque metric-line identity safe and supplies one exact existing-primary source-event/attack target plus its already existing channel;
5. the current transaction-local FinalChord occurrence continuation binds every selected member by exact source-event plus attack identity;
6. a clean run-local baseline renderer shadow shows more than one visible primary-beam fragment for those exact current occurrences;
7. current serialized roles contain only the already sealed same-staff primary/secondary relation with exactly one existing primary target channel.

No project identity, known witness, concrete source-id value, measure number, score-wide candidate count, voice-number convention or persisted result row can authorize selection.

## Late physical rebind

The complete pre-existing FinalNotation writer bundle runs first on the original frozen serialized voice relation. The primary-beam plan cannot steer any earlier FinalNotation writer.

Only after that bundle closes are selected members and the existing-primary target rebound again:

`source_event_id + attack_id -> current FinalPdfSource occurrence -> current (measure, pitched_ordinal) -> current <voice> scalar`.

Every selected member and every physical footprint must remain unique. Only members whose current scalar differs from the exact existing-primary target scalar enter the final edit batch.

The exact scalar ranges are delegated to the already accepted central `apply_atomic_staff_voice_scalar_batch_authorized` authority immediately before the existing `write_final_notation_checkpoint_authorized` sink. The transaction-local voice plan and occurrence continuation advance across the exact same voice-only delta.

## Runtime closures carried forward

The rejected r1066 MusicMutation attempts exposed three independent integration boundaries, all retained only as runtime evidence:

- the renderer baseline probe must use the already authorized target-scoped renderer-feasibility shadow namespace;
- the run-local baseline layout probe must not publish a second copy of globally unique page-count/joint-planner console evidence;
- the final writer projection needs an independent typed PrimaryBeam write cause rather than reinterpreting legacy writer counters.

r1067 therefore uses the accepted renderer-feasibility shadow root with a distinct PrimaryBeam project slot, uses `RunLocalProbeSilent` for the baseline layout computation and `AuthoritativeConsole` for the committed-source owner, and reports the accepted r1066 field `PrimaryBeamVoiceElementsWritten`.

## Writer-outcome binding

`PrimaryBeamVoiceElementsWritten` is projected from the exact late-rebind edit count only after `FinalNotationCommitOutcome::Committed`. A rejected or unrequested sink projects zero.

The field is additive evidence only. It does not replace, alter or reinterpret `MeasuresWritten`, Forward/Rest/Clef counters, legacy `MusicMutation` diagnostics, renderer outcome or release authority.

`src/corpus_run_manifest.rs` is unchanged in r1067; all parsing and aggregate-writer authority comes from the accepted r1066 parent.

## Invariants

ChangeClass: `MusicMutation`.

Master MusicXML: unchanged.

Primary-beam tags: unchanged.

Pitch: unchanged.

Onset and duration: unchanged.

Staff: unchanged.

Chord syntax: unchanged.

Ties: unchanged.

Voice/channel inventory: unchanged; only already existing sealed primary labels are reused.

New writer root: none.

New external productive write surface: none.

Persisted proof/report reentry: none.

Candidate width, solver semantics, renderer success criteria and fail-closed policy: unchanged.

## Validation expectation

The accepted r1066 parent has 1,390 strict all-target tests. r1067 adds the five generic late-rebind materialization tests, one run-local evidence-projection regression test and one post-commit writer-outcome regression test. Expected strict all-target count: 1,397.

The full corpus must remain 10/10. Any source/attack ambiguity, physical-footprint collision, target-channel drift, unauthorized shadow path, duplicate global console evidence, sink rejection, continuation/readback failure or manifest inconsistency remains fail-closed.
