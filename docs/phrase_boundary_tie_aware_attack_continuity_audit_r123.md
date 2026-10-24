# Phrase Boundary Tie-Aware Attack Continuity Audit r123

## Parent and scope

Parent: EtuSchlopf v0.10.1839 r122 `PhraseBoundaryEvidenceFoundationAuditNoMutationRecovery`.

r122 established the audit-only phrase-boundary evidence foundation and was accepted by the real Windows full-corpus strict run. r123 keeps that architecture and corrects one semantic projection error discovered only after reviewing the complete 10-etude corpus.

This take remains `AuditOnly`.

It introduces no phrase classification, no PhraseSpan, no SlurIntent, no NotationPolicy, no AnchorResolver, no writer authority, no MusicXML mutation, no sounding-event mutation, no master mutation, no external data and no learned weights.

## Corpus finding that motivates r123

The r122 10-etude corpus contains:

- 7,623 onset groups currently called attacks;
- 7,594 boundary observations;
- 679 visible pitched tie-stop members;
- 645 onset groups consisting only of tie-stop continuation members;
- 34 mixed onset groups containing both tied continuation members and genuinely newly attacked pitches.

A tie stop is a continuation of an already sounding note. It is not a new musical attack merely because MusicXML must spell the continuation as another `<note>` element.

Therefore r122 over-counts true attacks.

This matters before any further phrase theory is added. Phrase evidence based on inter-onset distance, duration, melodic motion or continuity must not treat notation-only tie segmentation as re-articulation.

## r123 attack contract

The phrase audit owner now distinguishes:

1. **newly attacked pitch members**: visible pitched members without `tie-stop`;
2. **tie continuation members**: visible pitched members with `tie-stop`;
3. **mixed onsets**: one onset containing both categories.

Chord grouping remains onset-based inside one exact `part/staff/voice` lane.

A tie-stop-only onset:

- does not create a new AttackPoint;
- does not create a new BoundaryObservation;
- does not enter the melodic attack descriptor as a new pitch;
- remains continuity evidence.

A mixed onset:

- remains one true AttackPoint because at least one pitch is newly attacked;
- excludes tied continuation pitches from the melodic attack descriptor;
- retains the sounding continuation at that onset when determining local lane continuity.

## Same-measure sounding continuation

Within one measure, a tie-stop-only continuation extends the `sounding_end` of the most recent true attack in the same lane.

Consequently a notated tie segment cannot create a false positive temporal gap.

This is deliberately limited to same-measure time in r123.

Cross-measure time remains unresolved exactly as in r122 because the phrase owner still has no canonical cross-measure temporal span. r123 does not invent one.

## Why cross-measure time is not r123

The first corpus review suggested that 700 cross-measure observations were the next obvious gap.

A deeper audit found the more fundamental attack-projection error above. Building cross-measure rhythm before correcting tie continuations would have propagated a false AttackPoint model into the next layer.

The order is therefore:

`r122 Boundary Evidence Foundation`
-> `r123 Tie-Aware True Attack / Continuity`
-> later `Canonical Cross-Measure Time`
-> only then richer evidence families.

## Expected corpus effect

An independent read-only projection over the r122 full corpus predicts:

- true attacks: 7,623 -> 6,978;
- boundary observations: 7,594 -> 6,949;
- tie-stop-only onsets excluded: 645;
- mixed onsets retained as true attacks: 34;
- positive gap support remains 181;
- explicit rest support remains 145;
- overlap opposition remains 23;
- cross-measure unresolved remains 700.

This expected effect is diagnostic, not an acceptance oracle. The real Windows run remains authoritative.

The important property is that the reduction is concentrated in notation-only tie segmentation while existing positive rhythm/rest evidence remains unchanged.

## Evidence and authority contracts

Still true after r123:

- `PhraseClassification = none`
- `PhraseSpan = none`
- `SlurIntent = none`
- `NotationPolicy = none`
- `AnchorResolver = none`
- `Scalarization = nein`
- `PhraseConfidence = none`
- `PromotedBoundaryCandidates = 0`
- `PauseGeneratedPositions = 0`
- `RenderXmlWrite = nein`
- `RenderNotationMutation = nein`
- `SoundingEventMutation = nein`
- `MasterMusicXmlMutation = nein`

## Hard stop after r123

Do not proceed directly to phrase spans or slurs.

First inspect the real 10-etude r123 corpus and verify:

1. tie-only attack reduction is stable and explained;
2. positive gap/rest evidence is not accidentally lost;
3. mixed tied/new onsets remain represented as attacks;
4. cross-measure unresolved remains visible rather than silently guessed;
5. no voice collapse or mutation occurs.

If green, the likely next foundation step is canonical cross-measure temporal continuity using an already authoritative metric/time owner rather than a new parallel timeline.

The architectural order remains:

`Evidence -> Structure -> Intent -> Notation`.
