# Same-beat partial overwrite of an existing beam group: causal definition

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
ErrorClassName: same_beat_partial_beam_group_overwrite
CausalFeature: an existing writer proposes replacement beam states on a proper subset of a local input beam connection within one metric beat
CausalFeature: the writer input has a claimed primary connection across the replacement membership boundary; the earlier note claims an outgoing connection with begin or continue, or the later note claims an incoming connection with continue or end
CausalFeature: replacing only the selected subset can remove or contradict that claimed connection while leaving the musical events and an adjacent member outside the replacement unchanged
CausalFeature: the connection and replacement boundary are local facts of the exact writer input and proposed write, independent of a final renderer result or a later compensating writer
NonCausalContextExcluded: project or etude identity, measure label, pitch, register, staff number, numeric voice label, clef location, leading or trailing rest presence, exact group size, exact duration, rarity, writer order, final PDF appearance and previously reported witness counts
NonCausalContextExcluded: independent occurrences elsewhere in the score, their discovery order and their number do not change local class membership

## Definition and necessity

This class concerns destructive partial replacement of an input primary beam
connection inside one metric beat. The primary state is evidence of a claimed
edge, not proof that the input is a consistent or correct complete beam group.
One-sided claims are sufficient: inconsistent begin/begin/end or end/end input
must not be mistaken for an absence of prior connectivity.

1. A proposed subset write is necessary: without a membership boundary there
   is no partial-group overwrite of this class.
2. A claimed input edge crossing that boundary is necessary: without it the
   local group may be newly beamable or already separate.
3. Both adjacent pitched events must lie in the same metric beat. A writer
   whose existing policy separates beats may retain that prior behavior.
4. A rest gap inside the beat does not itself erase a claimed edge. Chord
   members are attached resources, not additional primary beam steps.
5. Input and proposal must belong to the same actual writer execution. Final
   XML resemblance, report location or a Master comparison cannot substitute
   for this immediate binding.

The earlier and later event refer to the existing writer's onset/index order
inside its established staff/voice lane. This definition grants no new lane,
voice selection, measure selection, ranking or write authority. Independent
measures have disjoint mutation domains; duplicating or relabeling one cannot
change another's class membership. Existing multi-group transaction selection
is not broadened by this definition.

## Excluded extensions

An exactly matching replacement group without a crossing input edge is outside
this class. Loss of higher beam levels during such full-group normalization is
an explicitly separate open issue. Cross-beat and cross-measure edges are also
outside this definition. Protecting them would require separate justification.

## No implementation or music effect

This take freezes the definition only. No classifier, productive protection,
writer, test fixture derived from the corpus, threshold or renderer policy is
changed. A later implementation must consume this contract byte-unchanged from
an accepted parent and demonstrate its local safety scope using independent
synthetic cases. Corpus output changes and visible improvements remain
hypotheses until that implementation is evaluated.
