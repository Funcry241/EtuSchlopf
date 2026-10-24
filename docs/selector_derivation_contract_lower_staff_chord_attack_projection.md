# Lower-staff chord attack projection: error-class definition

Stage A of `causal_class_first_v1`. Written against accepted r1499/K6. Its acceptance status is determined solely by the lifecycle/acceptance record of the take that carries it. Repository path: `docs/selector_derivation_contract_lower_staff_chord_attack_projection.md`.

This definition may accompany a suitable take that introduces no new selector-sensitive code. It introduces no runtime implementation. Classification or repair must be delivered later, after the definition is present unchanged in the accepted parent. No separate revision is reserved by this document.

SelectorDerivationContract: causal_class_first_v1
SelectorChange: no
SelectorRole: error_class_definition
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: not_yet_measured
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
CausalFeature: a visible pitched lower-staff chord member has a serialized attachment to the head of its own chord before lower-staff visibility projection; filtering out that head must not attach the member to an earlier unrelated projected attack or erase the first lower-staff observation of this attack
CausalFeature: the chord head is the preceding non-member note reached through the member sequence in the same part and measure with no intervening backup, forward or attributes boundary; the head's staff, visibility, rest status and voice label do not replace this attachment relation
CausalFeature: an anchor created from a member's own attack position refers to the chord head rather than to an interior member position; any earlier tie or beam continuation anchor remains subject to its existing independent authority and safety checks
CausalFeature: if the member-to-head relation is unresolved, a surviving earlier tie or beam observation cannot by itself establish that missing relation or grant the member a new clef transition anchor
NonCausalContextExcluded: project identity, etude identity, concrete measure number, part/voice/staff label values, pitch values, register thresholds, source ordinal, absolute byte offset, beam presence, corpus frequency, candidate count, execution order across independent groups, renderer success and any known witness location

## Cause and class boundary

MusicXML distinguishes a note that advances the measure cursor from additional notes marked with `<chord/>`. The latter share the notated attack of their chord; their duration does not advance the cursor again. A local sequence of note carriers therefore has an attachment relation before a reader filters it by presentation staff, visibility or pitch presence.

The class is **loss of that attachment in the lower-staff readability projection**. It is present when a lower member is merged into an unrelated preceding projected attack, when the first relevant lower observation of its own chord is omitted from projected attack accounting, or when its self-position becomes an interior clef insertion anchor. The mechanism requires no beam tags and no particular pitch, register, voice number or surrounding melody.

A physical/notated chord attack is not an assertion of a fresh sounding key strike for every tied member. This definition neither changes tie semantics nor equates notation attacks with audio attacks.

The head is the last preceding non-member in the attached note sequence, not the immediately preceding member in a multi-note chord. An independent note at the same onset is not a chord member merely because its pitch or timing matches. No historical Source/Attack/Fragment identity is reconstructed here: current-document attachment is a local observation, not replacement provenance.

### Supported relation and unresolved relation

A resolved local relation stays within one part and measure and does not cross an intervening `backup`, `forward` or `attributes` element. These are conservative observation boundaries for this class; this is not a claim that the MusicXML specification globally forbids every other serialization.

An absent head, a boundary crossing or an otherwise unsupported relation remains unresolved. The reader may retain pitch pressure as its existing policy requires, but may not borrow the previous lower list entry as a substitute head. An unresolved member forms its own anchorless projected entry; it is never merged into another entry. Whether that entry contributes to passage attack counts must be specified explicitly in the later implementation take. A live tie or beam map entry alone does not resolve the missing chord attachment. No new clef anchor is authorized from that unresolved member, including when both maps still contain earlier origins.

A visible or hidden rest carrier can be the serialized head observed in a pipeline document. Its role here is positional attachment, not a claim that a rest sounds a pitch or that the whole document is schema-valid or faithfully importable. A hidden head and an upper-staff head likewise remain positional facts even when they create no visible pitched lower observation. Hidden members keep their existing visibility treatment.

Grace and cue handling is not extended or repaired by this definition. Existing octave-display, tie and primary-beam policies remain separate. This class does not require a general timeline parser, a new duration policy, or a new rule for accepting malformed documents.

## Consumers of the same observation

Two existing consumers on r1499 are relevant:

1. `scan_lower_pitches` in `src/structural_write_authority/rustig_clef_planning.rs` supplies projected attack starts and clef anchors. `collect_lower_staff_attacks`, passage accounting and `add_readability_passage_attacks` consume those facts.
2. `sustained_lower_staff_measure_clef_preference` in `src/notation.rs` (E054) counts lower attacks and accumulates the duration attached to counted attacks. Dropping a member solely because its head was filtered can affect both quantities.

One fact must mean the same attachment in both consumers. That does not authorize moving clef policy, thresholds, costs or writer ownership into an observation helper. For E054, any later correction must account consistently for attack count and its associated duration; this definition does not select a new maximum/minimum/union duration rule for unequal-duration chord tones.

For the planned implementation, the proposed E054 convention is that the first visible pitched lower observation of a resolved chord contributes its own duration, matching the current treatment of a visible lower head. This proposal is not an executable policy or a class-membership condition; the implementation take must state and test it explicitly, including its treatment of unresolved entries.

The existing `src/clef_readability_observation.rs` is a candidate owner for a shared writer-free attachment primitive. That ownership proposal is not an implemented API or permission to change additional consumers.

## Necessity review and positive neighbors

| Feature | Why needed independently of a known witness | Positive or limiting neighbor |
| --- | --- | --- |
| Member-to-head attachment before projection | A filtered head is still the positional origin of its members | Visible lower head with several lower members remains one projected attack |
| First lower observation of each resolved chord | Otherwise an upper, hidden or rest head can make all lower members disappear from attack accounting | Later lower members of the same chord do not create additional attacks |
| Head-based self-anchor | An interior member is not the beginning of the chord | Existing legal tie-chain origin may precede the current head |
| Explicit unresolved state | Missing attachment cannot be repaired by list proximity or a surviving map entry | Following ordinary non-member note starts its own resolvable sequence |

Changing the preceding unrelated voice, duplicating the group in an independent location, or renaming measure labels does not change these arguments. A leading rest or a particular number of chord notes is not a required class feature.

## Independence from adjacent classes

- Beam-fragment continuity, part-wide beam-map scope and phantom remote binding are separate causes. This definition does not change `open_primary_beams`, tie identity, beam-group membership or the global normalization fallback.
- Consumer-side shared-chord staff conflicts in `selector_derivation_contract_chord_scoped_staff_projection.md` concern import representation, not this reader's lost attachment.
- The r1252 source-envelope and local #57 repair-scope contracts concern safe relocation envelopes and current coordinates. They do not supply the missing attack-projection definition.
- Original chord/beam owner-evidence transport is provenance transport, not permission for a new clef mutation.
- The local register conflict illustrated by E6/T22 remains outside this class's repair. No beam splitting, clef-inside-beam exception or new short octave-shift rule is introduced.

## Prior knowledge disclosure and future measurement

The r1494 synthetic experiments, including beam-free swapped-head pairs, were known before this text was written. Dachs also reported an existing rest-headed lower-member occurrence at E4/T30 in the r1499 end states. Earlier corpus and prototype measurements are historical exploratory evidence; this document does not claim they happened after acceptance of this definition or that the author had not seen them.

The features above are derived from serialized chord attachment and projection loss, not selected to isolate that occurrence. No witness pitch, measure, note count, preceding-rest pattern or corpus uniqueness enters class membership. The definition would remain the same if that occurrence were removed.

The Stage-A fields `ClassDefinitionPrecedesCorpusDistribution` and `CorpusDistributionPurpose` describe the required ordering for a future measurement of this frozen class. They must not be read as a retrospective claim that no prior corpus exploration occurred. This file supplies no new class-distribution measurement and no executable classifier. The receiving take's review must retain this disclosure.

Future normative regression fixtures must be synthetic. They should cover the two consumers, swapped head/member order where equivalence is independently established, hidden/rest heads, multiple member voices, missing heads at each stated boundary, and unresolved relations despite surviving tie/beam state. Positive tie-continuation controls must remain. Relocation and independent duplication preserve local decisions; no score-wide cap or first-winner rule is justified.

The full productive E024 and #58 Prepared/receipt/adoption paths remain later proof obligations. Existing green tests or successful rendering alone do not prove this class repaired.

## Stage-A authority limit and later repair

This file grants no classifier, selector, clef insertion, note reordering, staff transfer, beam or tie mutation, receipt minting, adoption, release or SSC authority. It changes no Rust source, test, registry, threshold, launcher, Cargo file or runtime output.

The next suitable non-selector-sensitive take may carry this definition as metadata, with its actual final diff and change class checked by the existing admission rules. The subsequent implementation remains MusicMutation even if the current corpus stays byte-identical. Any implementation contract must refer to this unchanged definition in an accepted parent, as required by the existing gates. Code wording must not be tuned to evade lexical detection.

For the later attack-projection take, byte-identical 20/20 corpus XMLs, pixel-identical Windows PDFs apart from metadata, unchanged beam/tie tags and zero clef reentry are the agreed expectations to test, not results proved by this Stage-A document. An unexpected difference requires diagnosis and renewed scope agreement.

## Primary semantic reference

[MusicXML 4.0: chord](https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/chord/) describes attachment to the preceding note and the cursor behavior of the first preceding non-chord note. [MusicXML 4.0: duration](https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/duration/) distinguishes cursor advancement for ordinary notes, backup and forward. These references support the semantic derivation; they do not replace EtuSchlopf's existing authorities or certify an individual input document.
