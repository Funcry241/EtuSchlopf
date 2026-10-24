# WavObservation Legacy Oracle Qualification r147

Estimated WavPerception plan position: Take 1/4.

Parent authority: accepted full-corpus r146 `PhraseSlurExistingWriterRootProductiveRecovery`.

Parent acceptance evidence:

- Assistant export: `assistant_export_latest(20260821-134114).zip`
- Version: `0.10.1839`
- Revision: `r146`
- Full corpus: `10/10`
- FinalExit: `0`
- BuildExit: `0`
- ClippyExit: `0`
- RunSeverity: `dark_green`
- ReleaseDecision: `released`
- Complete source snapshot SHA-256: `013b6bc5dd71d8434ec5e13b1903ba3bfb194c611f9c934b7d207c1c507c8db6`

Frozen phrase baseline from that accepted corpus run:

- SlurIntent `allow_structural_pending_visual`: `71`
- SlurIntent withhold: `9`
- Withhold single attack: `5`
- Withhold voice overlap: `4`
- Withhold internal conflict: `0`
- Withhold unexplained parallel role divergence: `0`
- Master MusicXML slur pairs across corpus: `0`
- Final render MusicXML slur pairs across corpus: `28`
- Phrase-report baseline SHA-256 over the ordered per-project phrase-projection fingerprints: `17f09bd24eaa4f815d591fcd7b5588af85e578fd56e4387a6b4829abef8d31cf`

## Change class

`AuditOnly`.

No productive source authority changes. FLP remains the only productive source/seed authority in r147. Existing WAV articulation and audio-salience consumers remain behaviorally untouched.

## Purpose

Introduce a source-safe immutable WAV observation audit and a frozen read-only view of the legacy FLP source events before hand/staff/playability transformations.

This take does **not** perform pitch transcription, onset promotion, beat tracking, metric reconstruction, MIDI authority, source cutover, phrase modification or writer mutation.

## WavObservation contract

The exact WAV file bytes own source identity.

The canonical physical time coordinate is `OriginalSampleIndex`, defined here as the original interleaved WAV frame ordinal. All channels at one frame share that physical time coordinate. Seconds are a derived projection only.

The audit records:

- exact source-file SHA-256;
- decoded-signal FNV-64 over channel-preserving decoded sample values;
- audio format;
- sample rate;
- channel count;
- bits per sample;
- block alignment;
- original frame count;
- decoded sample-value count;
- exact first/last non-zero frame without a musical activity threshold;
- partial-frame tail status.

Original channels are not collapsed by this owner. The existing mono average remains a derived consumer view and receives no new authority.

## FrozenLegacySourceOracle contract

The legacy oracle is an audit-only parser of the same FLP note-event format. It does not call or modify the productive FLP seed builder. It mirrors the raw event and PPQ normalization contract, normalizes to the existing canonical `96` divisions when needed, and freezes only pre-hand/pre-staff source-event facts:

- stable ordinal;
- pitch MIDI;
- metric onset ticks;
- source extent hint ticks.

The fingerprint intentionally includes the stable ordinal, so two otherwise identical pitch/onset events remain two events. No set semantics are permitted.

The legacy oracle is evaluation evidence only. It may not become a WAV runtime feature, fallback, repair source or production selection input after source cutover.

## Runtime report gate

The existing result report gains one compact section:

`WAV-OBSERVATION-LEGACY-ORACLE-QUALIFICATION`

It ends in either:

- `WAV OBSERVATION QUALIFIED`, or
- `WAV OBSERVATION NOT QUALIFIED`.

This gate is audit-only and does not change the current release decision.

## Hard invariants

r147 must not change:

- FLP productive source authority;
- generated Master MusicXML semantics;
- existing WavTruth articulation decisions;
- existing audio-salience decisions;
- phrase classification or SlurIntent policy;
- phrase-slur writer root;
- StructuralWriteAuthority root set;
- sounding/timeline truth;
- PDF music/layout behavior.

No external model/runtime dependency is added. No ONNX, Python, Torch, Basic Pitch, TransKun or MIDI product path is introduced.

## Expected Windows acceptance

Run:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Expected:

- PRE-TAKE-GATE GREEN from accepted r146;
- semantic capability/security gates green;
- Rust compile/tests/strict Clippy green;
- normal Etüde Nr. 6 musical result unchanged;
- new audit section present and `WAV OBSERVATION QUALIFIED` for the real WAV/FLP pair;
- no new writer root and no master/sounding mutation.

After a green single-target commit, a full-corpus run is the qualification evidence for Take 1/4. Take 2/4 may begin only after that corpus evidence is reviewed.
