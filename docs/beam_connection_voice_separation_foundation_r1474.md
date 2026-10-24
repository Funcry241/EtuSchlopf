# r1474: beam/voice compatibility foundation

Parent: accepted r1473; delivery SHA-256
`28ffae8143909c1c8005e8307b0ba547242539ba9dc67cedeb9811b75f4560f6`.
Class: RefactorOnly. Scope: ZIP identity, two new contracts, this explanation and
the existing change manifest. Every Rust source, validator, pin, dependency and
runtime option remains byte-identical to the accepted parent. No music effect.

The causal definition covers both an early connection proposal conflicting with
an available sealed assignment and a later assignment separating a written
connection. The implementation rules remain separate from that error class.
The companion document freezes evidence requirements, not a proven implementation.

## Code findings

`run_rest_and_articulation_render_stage` receives canonical structure and the
frozen voice plan before executing the bridge. The initial stage has already
attached producer render-occurrence witnesses. Staccato persistence already uses
a typed receipt to bind writer-owned ranges to canonical source/attack/fragment
identities and advances that evidence with validation.

The bridge currently consumes only hierarchy edits and returns a generic
preserving receipt; it carries no identity-bound extension relation. Later,
`run_final_render_notation_authority_stage` creates the FinalChord occurrence
continuation. M5 subsequently transports the PDF-source continuation. Therefore
extending only the late type would omit the early history: a checked early
companion and explicit handover are required. The complete bridge transport is
not implemented or claimed proven by this take.

The existing voice-free occurrence signature uses sounding MIDI pitch rather
than spelling. This is useful for checked respelling but is not sufficient as a
bridge-connection validator: relevant beam states and other structural properties
need their own checks. Existing validators and authority boundaries remain intact.

## Evaluation plan, not normative selection

Known evaluation cases: E6/T34 splits when the frozen plan is materialized;
E6/T32 splits during later original-group alignment. A corpus diagnostic currently
reports those two locations. It remains a diagnostic, not a gate or source of
write authority; its importer, chord and cross-staff scope must be explicit.
The intended later comparison is no remaining known cases and no newly introduced
ones, alongside semantic XML checks and PDF inspection. That is a hypothesis.
E9/T32's previously explained voice change is not newly authorized here.

The repair may require declared voice changes in addition to beam changes.
A future concrete music-effect scope must state them explicitly; this foundation
neither performs nor pre-approves those writes. Expected preservation covers all
Master XMLs, sounding notes, durations, rests, ties and slurs, subject to the exact
later reviewed scope. No score-specific selector is permitted.

## Native acceptance

Run `.\run.cmd --take --all`. Require the exact r1474 archive accepted, all strict
checks green, all 20 XMLs byte-identical to r1473 and all 50 PDF pages pixel-equal
apart from file metadata. The optional trace is not required for this documents-only
take. A later repair must cite the unchanged causal contract from this accepted
parent, together with its implementation and evidence requirements.

Local validation: all four unchanged parent checks must pass for the clean tree
and extracted ZIP; a complete managed-tree comparison must show no code changes.
No local corpus renderer run or redundant Rust suite is needed for unchanged code.
