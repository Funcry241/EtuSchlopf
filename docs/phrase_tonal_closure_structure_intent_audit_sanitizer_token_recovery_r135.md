# Phrase Tonal Closure Structure Intent Audit Sanitizer Token Recovery r135

## Estimated plan position

Take 6/7 (estimated) on the compressed phrasing path.

## Parent

Accepted DARK-GREEN parent: EtuSchlopf v0.10.1839 r133
`PhraseBoundaryTonalContextReadOnlyProjectionPrewire`.

r134 proved that the combined tonal/closure/structure/intent source compiles and passes strict Clippy,
but its normal program run was rejected by the existing Behind-Bars sanitizer because one newly added
diagnostic sentence contained a reserved human-inspection vocabulary fragment. r135 is rebuilt from
the accepted r133 parent and changes only that diagnostic vocabulary plus revision metadata.

## Purpose

r135 is the final analysis/intent take before any productive slur writer. It combines the same three
read-only stages as the rejected r134 candidate:

`Tonal/Closure Evidence -> Phrase Structure -> Slur Intent / Notation Policy`

No MusicXML, sounding-event or master mutation is permitted.

## Recovery delta

The musical and structural logic is byte-identical to the r134 candidate except for one diagnostic
literal:

the rejected diagnostic literal becomes

`context_requires_inspection_not_write_authority`

The replacement preserves the meaning but does not contain the Behind-Bars sanitizer's reserved
human-authority vocabulary fragments.

No Security policy, sanitizer rule, writer authority or productive pipeline code is changed.

## Tonal and anti-closure evidence

The phrase audit consumes only the existing score-wide best/second tonal profile projection. The
profile name, tonic pitch class, major/minor mode, best score, second score and exact margin remain
visible. The audit adds local observations at existing boundary positions:

- tonic/dominant presence in left and right lane attacks;
- score-wide tonic/dominant presence at the arrival tick;
- bass pitch class immediately before and at arrival;
- bass arrival to tonic;
- bass dominant-to-tonic motion;
- same-lane leading-tone-to-tonic resolution;
- same-lane unresolved leading tone as explicit anti-closure evidence.

There is no local-key claim, Roman-numeral analysis, cadence label, probability or learned weight.

## Qualitative phrase boundary roles

No scalarization is introduced. The audit derives one explainable role from explicit conjunctions:

- `closure_supported`
- `structural_joint`
- `continuation_supported`
- `conflict`
- `unresolved`

A tonal cue alone never creates `closure_supported`. Closure requires both a tonal closure cue and an
independent structural gate.

## Relative hierarchy and intent

Voice and score structure remain separate.

- Grouping level 0: local groups segmented by the conservative structural gate.
- Grouping level 1: same-lane spans closed by `closure_supported` boundaries.
- Open tails remain explicit.
- Score regions group coincident structural/closure observations by canonical time without majority
  voting.

Only the level-0 local group ending at a `closure_supported` boundary can receive
`allow_structural_pending_visual`. It is withheld for a single-attack group, overlap evidence or an
internal boundary conflict. Anchor resolution, overlap numbering, cross-staff feasibility, visual
collision checks and MusicXML writing remain absent.

Default remains `no slur`.

## Static 10-etude replay expectation

The latest known canonical corpus preserves 6,978 true attacks and 6,949 boundary observations. The
r135 musical projection is intentionally identical to r134:

- tonal descriptors known: 6,949/6,949;
- raw tonal closure support: 941 positions;
- explicit tonal anti-closure: 415 positions;
- qualitative `closure_supported`: 84 positions;
- `structural_joint`: 2,647;
- `continuation_supported`: 456;
- `conflict`: 1;
- `unresolved`: 3,761;
- score closure regions: 78;
- multi-lane closure regions: 6;
- score closure regions with independently continuing context: 7;
- grouping-level-1 closed spans: 84;
- open tails: 29;
- structural slur intents pending visual: 74;
- structural slur intents withheld: 10.

## Hard stop after r135

Run the full 10-etude corpus after the single-project gate is green. Inspect closure density,
low-margin tonal contexts, phrase-span lengths and nesting, parallel-pattern role divergences,
continuing voices across score closure, and structural SlurIntent coverage/withhold reasons.

Only a convincing full-corpus hard stop may authorize Take 7/7.
