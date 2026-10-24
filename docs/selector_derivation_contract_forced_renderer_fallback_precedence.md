# Forced renderer fallback precedence over unforced conversion

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
ClassDerivationBasis: documented converter semantics of the force option and the existing ordered compatibility list; corpus witnesses do not define the predicate
CausalFeature: the renderer offers an ordered list of invocation variants in which some variants carry a force option that makes the consumer ignore import corruption or version warnings, while the other variants do not
CausalFeature: a process-local or shadow-scoped reuse mechanism remembers a previously winning invocation variant and moves it ahead of the remaining list for a later conversion of a different or changed source document
CausalFeature: the remembered winner is a forced variant, so the later conversion of the actual final source is attempted forced first and no unforced attempt on that exact source byte state is made or recorded
CausalFeature: a feasibility or acceptance consumer treats any successful conversion as renderer-feasible without distinguishing whether the success required the force option
NonCausalContextExcluded: project etude measure page system pitch voice label staff index source or occurrence identifiers revision corpus rarity worker count and execution timing
NonCausalContextExcluded: the particular executable path installation source operating system converter exit code value PDF byte size and whether the final forced output happens to look acceptable

## Class meaning

The force option is a compatibility fallback. It hides the consumer's own
judgement that the imported score is corrupt. A conversion that only succeeds
forced is therefore not evidence that the serialized MusicXML is importable,
and it is not evidence of renderer feasibility.

The defect is the loss of the unforced judgement for the exact source byte state
that becomes the product, or the conflation of forced and unforced success in a
feasibility decision. It is independent of whether the unforced attempt would
have succeeded. Both outcomes are possible members: an unforced success that was
never tried, and an unforced failure that was masked.

## Necessity review

- Without a forced variant in the list there is nothing to hide.
- Without reuse or reordering the existing list already tries every unforced
  variant before any forced variant; the defect then requires a consumer that
  accepts forced success as feasibility.
- A remembered unforced winner moved first is benign: it does not suppress the
  consumer's corruption judgement.
- Reuse for the byte-identical source with the identical outcome record is not a
  member, because the unforced judgement for that state already exists.

## Invariance

Renaming projects, measures, labels or executables, permuting unrelated list
entries while keeping forced variants after unforced ones, changing timing or
worker counts, and uniform duration scaling do not change membership.

## Authority boundary

This definition grants no selector, renderer command change, cache change,
release decision, red exit or force prohibition. A later implementation must
cite this file as `ParentErrorClassContract`, keep the complete compatibility
list available, and report per conversion whether it was forced and whether an
unforced attempt on the exact final source was made, with that attempt's
consumer evidence. A successful PDF is never material proof by itself.
