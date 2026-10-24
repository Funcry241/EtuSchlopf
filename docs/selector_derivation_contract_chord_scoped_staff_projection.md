# Chord-scoped consumer staff projection cannot preserve distinct occurrence staves

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
ClassDerivationBasis: independently inspected consumer data ownership and import grouping semantics; synthetic documents and positive controls establish the mechanism without corpus identity
CausalFeature: a complete exact written projection binds each independently required represented pitched occurrence and its presentation staff to existing production and role authority before consumer import
CausalFeature: an independently established consumer profile groups two or more of these occurrences into one imported chord object using its actual resolved part time and track key
CausalFeature: the participating occurrences require at least two distinct presentation staves under their carried production authority
CausalFeature: the consumer owns presentation staff movement at that shared chord object rather than at each participating occurrence and applies one such value to all its notes
CausalFeature: at least one required occurrence therefore cannot retain its independently bound presentation staff under that shared consumer object; overwriting the object's staff movement with a later member is one independently documented realization
NonCausalContextExcluded: project etude measure number pitch spelling register raw voice or staff label source and occurrence identifier values page system revision corpus rarity and execution order across independent groups
NonCausalContextExcluded: tie presence octave-shift presence note count beyond the distinct-staff contradiction head count rest count majority staff converter exit code force flags PDF existence and a known witness's desired assignment

## Definition and causal derivation

This is a consumer representation incompatibility, not a statement that every
cross-staff chord is intrinsically invalid. A format can express per-note staff
intent while a particular consumer stores staff movement on the shared chord.
If that consumer groups required notes with different bound presentation staves
into one object, one shared staff value cannot satisfy all those obligations.
At least one occurrence changes presentation staff even when pitch, onset,
duration, tie markers and total note count remain intact.

For MuseScore 4.7.4, pass 2 computes each note's time and resolved track through
the pass-1 allocation. Ordinary pitched notes enter `findOrCreateChord`, which
finds or creates a chord at that time and track and calls
`c->setStaffMove(move)` on the shared object for each participating note.
The final assignment applies to the chord, including previously inserted notes.
The mechanism follows from the location of that property and the grouping key;
it does not follow from any particular score's distribution.

Raw `<voice>` equality is insufficient to prove class membership. Consumer key
resolution can involve partwide allocation and aliases. Conversely, a local
head/member relationship in the source is not sufficient if the consumer maps
those carriers to independent objects. Classification must use the complete
consumer grouping semantics and the carried occurrence/staff obligations.
Unknown profile, incomplete cursor evidence or missing occurrence provenance
remains unknown, not permission to rewrite or omit notes.

## Independence from existing classes

`partwide_renderer_channel_capacity` remains byte-unchanged and owns excluded
occurrences caused by bounded slot exhaustion. This class requires a shared
imported object with contradictory staff obligations even when every channel
has a slot. Eliminating exhaustion alone does not eliminate this contradiction.

`octave_display_boundary_incoming_tie` remains byte-unchanged. Ties and octave
spans are not necessary features here: the minimal synthetic examples have
neither. Retaining all tie markers does not prove presentation staff retention.
The existing staff-relocation notation-integrity definition remains unchanged;
its repair boundary withholds individual staff relocation offers and does not
authorize repairing an already imported consumer grouping collision.

## Necessity and exclusions

1. Existing production and role binding separates required presentation staff
   from an unlicensed preference inferred from pitch, geometry or majority.
2. The actual resolved consumer grouping key establishes a shared state object;
   equal onset, an XML chord marker or a coincident pitch alone does not.
3. Distinct required staff assignments create the contradiction. A homogeneous
   shared chord is the positive neighboring case and is outside this class.
4. Shared chord-level ownership explains why one member affects other notes.
   A consumer with independently preserved per-note staff assignment is outside
   this class even when its input notation looks identical.
5. Pitch, note count, ties and successful conversion do not discharge a staff
   obligation. They are separate evidence dimensions.

A later member's order may determine which staff value survives. It is not an
independent repair feature or an authorization to reorder notes until a known
case looks correct. Homogeneous groups remain valid regardless of member order.

## Invariance and shared-group boundary

Project/measure/source renaming, relocation in time, uniform divisions scaling,
and order-preserving label renaming preserve the contradiction when the resolved
consumer grouping is preserved. Presentation-staff permutation with the matching
profile and existing role bindings also preserves it.

Each distinct imported chord object is one explicit shared conflict group.
Duplicating it at another time or in an independent part preserves the decision
for every original group. No global first winner, scorewide cardinality, ranking
or action cap grants repair authority. Adding notes to the same imported chord
changes that group's obligations and is not independent duplication.

## Stage-A scope and future repair boundary

This Take freezes only the definition. It introduces no classifier, allocator,
writer, label binding, chord split, staff or hand transfer, omission, receipt,
SSC envelope, adoption change, consumer command, release gate or runtime branch.
All Rust and musical outputs remain byte-identical to accepted r1397.

A later implementation must reference this unchanged accepted definition and
separate its repair scope from class membership. It must use existing canonical
source/fragment/attack, presentation, chord-host, HARD SAME, rest and cursor
authorities. A convenient free import slot does not authorize merging unrelated
musical events; a shared consumer property does not authorize moving a note's
staff, splitting a canonical chord or relaxing HARD SAME.

The existing serialization owner is the first place to assess a complete
consumer-compatible technical assignment. If existing canonical constraints
require a grouping the consumer cannot represent, that conflict must be
resolved by the independently authorized representation/decision owner or left
open. It cannot be discharged by a new post-render cleaner or an inferred role.
Successful PDF output, a zero-loss capacity model, octave pairing or removing
staff from the comparator is not a complete conservation proof.

No selective real-score fixture enters the repository. External evaluations may
locate a known occurrence, but any future normative test must be synthetic and
must exercise renaming, relocation, scaling, staff permutation, a homogeneous
positive control and independent duplication before productive adoption.

## Primary implementation reference

- [MuseScore v4.7.4 MusicXML pass 2](https://github.com/musescore/MuseScore/blob/v4.7.4/src/importexport/musicxml/internal/import/importmusicxmlpass2.cpp): `note`, consumer track resolution, `findOrCreateChord`, shared staff-move assignment.

The source reference establishes consumer behavior, not EtuSchlopf mutation
permission. The current repository's canonical authorities and gates retain
exclusive control over any later implementation.
