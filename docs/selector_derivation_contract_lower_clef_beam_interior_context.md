# Lower-clef boundaries excluded by unconditional primary-beam anchor closure

Stage A on accepted r1512. This is a causal decision-context definition, not an
implementation, classifier, permission to insert a clef, or musical release.
Native hash-bound acceptance is pending. A later implementation must reference
these bytes unchanged from an already accepted parent.

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
ErrorClassName: lower_clef_internal_register_context_excluded_by_primary_beam_anchor_coalescence
ClassDerivationBasis: a primary beam establishes rhythmic membership, whereas a clef establishes the written pitch reference; preserving membership does not itself establish whether a reading-reference boundary at an independent attack is safe, useful, or materializable
CausalFeature: visible lower-domain attacks in an independently established primary-beam component have differing clef preferences under the existing written-pitch observation and existing supported clef pressure function
CausalFeature: the decision maps those internal preferences to a shared earlier structural origin solely through primary-beam closure, including its existing transitive tie-anchor composition, and therefore omits their own attack boundaries from the clef-course comparison
CausalFeature: no independently bound decision at those omitted boundaries establishes whether a complete clef-only alternative would preserve the rhythmic component, tie and chord semantics, surrounding clef carry, and materialization safety
NonCausalContextExcluded: project or file identity, printed measure number, exact pitch spelling or MIDI value, numeric staff or voice label, event ordinal or byte offset value, note count, fixed pitch-jump or ledger threshold, corpus frequency, number of candidates, discovery order, worker count, renderer success and resemblance to a known PDF

## Meaning and necessity

This class describes an omitted comparison context, not proof that the current
clef course is musically wrong. Opposing preferences alone do not justify a clef
change. Summing all pitches at a shared origin may be the best feasible choice.
The causal question is whether beam membership alone must erase an independently
attacked reading boundary before its complete safety and cost can be evaluated.

Each feature is necessary. Without conflicting preferences there is no missing
internal register decision in this class. Without anchor coalescence, a missing
candidate may have another cause. If an independently bound boundary decision
already considers and rejects the alternative for actual safety, continuity,
cost or materialization reasons, that is not this omission.

The cause follows from the distinct meanings of rhythmic grouping and pitch
reference, not from a particular low tail, leap size, pair of durations, accidental,
number of notes, or number of ledger lines. Exploration of known output prompted
the question; these observations are not claimed to have been unseen. No new
governed classifier has measured the distribution of this definition.

## Positive neighbors and UNKNOWN

- An ordinary same-register beam can remain in one clef without this conflict.
- A tied continuation is not an independent new-attack boundary. A tie origin
  must not be discarded merely to place a desired clef later.
- A chord member is not a new attack. The complete simultaneous chord and all
  its incoming/outgoing ties remain the observation unit.
- A beam beginning at a tied continuation can legitimately inherit an earlier
  anchor. This definition does not declare that inheritance defective.
- A real clef change inside a beam can be distracting or geometrically unsafe.
  Its preference is not decided by this definition.
- A full-course comparison can reject an internal change because it worsens
  aggregate pressure, transition cost, short excursions, or the following carry.
- Cross-staff groups require independently established part, attack and group
  ownership; equal voice text or PDF adjacency is insufficient.
- Missing, stale or unmatched beam/tie/chord evidence is UNKNOWN. Neither an
  unknown relation nor a high ledger count can mint an insertion permission.
- Octave-display boundaries retain their accepted scope semantics. An upper-only
  direction cannot reset a lower decision scope.

## Repair scope must be established separately

The existing owner is ClefReadabilityPrimaryCourse in
src/structural_write_authority/rustig_clef_planning.rs. Any future clef-only
alternative must be decided there, using the actual phase input and the existing
cost function over the full affected course, including incoming and outgoing
state. It must preserve sounding pitches, spelling, onset, duration, visibility,
staff/voice, beam membership and levels, ties, slurs, rests and octave directions.
There is no new writer, feedback loop, cleanup pass or renderer correction.

The accepted primary-beam closure currently forbids internal clef surfaces at
several independent layers. This document does not retire any of them. A later
repair must identify and reconcile candidate formation, solver admission,
normalization, materialization and postwrite checks with one independently proved
boundary policy; bypassing just one guard is forbidden. Existing tie/chord and
scope protections must remain effective. If this cannot be proved, preserve the
baseline and expose the unresolved boundary. An alternative involving an octave
change, beam splitting or relocation needs its own owner and authority; it is
not implicitly authorized by this clef-only definition.

The comparison must not use a hard three-ledger cutoff or retune pressure,
transition or short-excursion weights. Nor may the absence of a permitted clef
boundary be repaired by inventing a primary-beam beginning or by changing notes.
Strict improvement under unchanged costs is necessary for the intended later
repair, but never sufficient without all semantic and writer-safety proofs.

## Required independent evidence before productive change

Synthetic tests must distinguish a safe fresh attack from a tied continuation,
chord member, incoming tie on another chord pitch, unsupported/hidden carrier,
unmatched beam continuation and cross-staff ambiguity. Include both directions
of register change, ties linking successive beam groups, existing internal clefs,
octave start/stop scopes, conflicting simultaneous pitches, equal/worse complete
courses and an unchanged ordinary group. A safe alternative must survive the
real planner, normalization, original materializer, receipt/adoption, committed
readback and reentry without altering beam/tie bytes or unrelated music.

Rename labels, relocate synthetic components, perturb unrelated input and duplicate
independent components. Each original local decision must remain unchanged; no
global winner, candidate cap or scorewide uniqueness can grant authority. Real
corpus examples remain evaluation only. Distribution may falsify this definition
or reveal missing evidence; it cannot be used to narrow it toward one example.
