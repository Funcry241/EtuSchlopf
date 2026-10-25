# r1516 - OttavaRecueContextDefinition

Parent: accepted r1515, take SHA-256
15cb8e29f849c4f1d27f431f9d940ec6faf2e5bd323b87f8b59ac10fdf9e848d.
ChangeClass: KodexOnly. Native acceptance is pending.

## Confirmed current parent

The 20261025-140002 export binds the delivered r1515 payload to its acceptance
record. Acceptance, metadata, intent and semantic-snapshot hashes were checked.
The source snapshot matches the delivered managed source tree. Windows reports
2493 passed, 0 failed, 1 ignored across 15 test programs; build/clippy passed,
ten-piece corpus dark-green, final exit 0.

The two clef examples are visibly improved. Five PDF pages change; 45 are
pixel-identical at 144 dpi. All raw pitched-note fragments in all 20 XMLs remain
byte-identical. Other structural changes are clef surfaces and three rest display
rebindings. Nine Masters are byte-identical; the remaining Master moves a clef.
Committed primary clef reentry reports zero changes in all ten pieces.

## Corrected diagnosis of the proposed next topic

The earlier description of the repeated ottava words as a simple duplicate was
premature. The accepted code intentionally emits a matching visible word for
each silent-bridge continuation. The words are not a second octave shift.

Evaluation address only: Etude 4, bar 8, upper staff. At divisions 96, the final
Render contains visible continues at offsets 24 and 120. Immediately preceding
upper-domain gaps are 0..24 and 96..120; pitched lower-domain notes occupy those
intervals. A third lower-domain event occupies 192..216; absence of an identical
cue there is not by itself evidence of a missing word. The final artifact alone
cannot reconstruct all earlier planner or visible-plan decisions.

The PDF shows both words under one visually continued line on that system.
That motivates a reading-context comparison. It does not prove that either
word should be removed. No corpus-wide candidate distribution was used to
derive or narrow the new class.

## Responsible existing paths

- src/registernotation/mod.rs: derive visible persistent spans and record
  RegisternotationsSichtwiederholung at qualifying silent bridges.
- src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs: materialize
  and validate an exact matching visible continuation reminder; generic octave
  word cleanup explicitly retains continuation words.
- src/structural_write_authority/horizontal_layout_plan_materialization.rs:
  project away generated reminders only at supported final layout restarts.
- docs/registernotation_silent_bridge_final_layout_recue_projection_closure_r605.md:
  intentional in-system reminder and final-layout restart distinction.

These accepted decisions and guards remain unchanged. The existing gesture and
common-span error classes concern octave selection; they are not reused as an
implicit license to change this cue-presentation policy.

## Why this is a definition take

src/take_change_control.rs requires a causal error-class contract frozen in an
already accepted parent before selector-sensitive implementation. This take
therefore adds only the separate omitted cue-context definition, this evidence
note and candidate/status metadata. All runtime Rust, tests, dependencies,
gates, prior contracts and musical outputs remain unchanged by the take.

After acceptance, establish the actual visible-plan inputs and a supported
reader-reminder decision, then prove materialization and final-guard closure.
Do not delete words first and adapt validation afterward. No repaired PDF or
future take identity is claimed here.

Apply on accepted r1515 with:

    .\run.cmd --take --all
