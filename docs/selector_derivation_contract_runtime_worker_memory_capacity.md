# Automatic worker count exceeding measured memory capacity

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
ClassDerivationBasis: resource arithmetic of concurrent process or thread workers on a host with bounded memory; no corpus or host witness defines the predicate
CausalFeature: the program starts an automatically chosen number of concurrent pipeline workers when the user gives no explicit worker count
CausalFeature: each concurrent worker has a positive peak memory demand that is not negligible relative to the memory the host can provide to the program
CausalFeature: the automatic default is derived from processor parallelism or from fixed constants alone, without a measurement of the physical and currently available memory of the executing host or its memory limit
CausalFeature: the product of the default worker count and the per-worker peak demand can exceed the measured available memory, so the default itself can cause paging, out-of-memory termination or timing-dependent failures
NonCausalContextExcluded: project etude measure pitch voice staff source or occurrence identifiers revision corpus rarity and musical content of any run
NonCausalContextExcluded: a particular processor model thread count memory size operating system container limit or historical host configuration, and any fixed default such as a remembered thread or worker number

## Class meaning

Processor parallelism bounds useful concurrency. Memory bounds safe
concurrency. A default that only sees the first can be correct on one host and
harmful on another with the same thread count and less memory. The defect is
the missing measurement in the default path, not a particular number.

An explicit user override is outside the class: the user owns that decision.
A measurement that is unavailable on a platform is not a proof of safety; it
must be reported as unknown and must not silently fall back to a fixed
memory assumption presented as measured.

## Necessity review

- Without automatic concurrency there is no default to size.
- Without non-negligible per-worker memory the processor bound suffices.
- With a memory measurement in the default path the predicate is false.
- Without the possibility of exceeding available memory the default is safe.

## Invariance

Renaming projects or files, changing musical input, permuting worker order and
scaling durations do not change membership. Membership depends only on how the
default is derived and on the measured resource relation.

## Authority boundary

This definition grants no scheduling change, worker selection, candidate budget
change or musical selection change. Memory must never alter which musical
candidate is chosen; it may only bound concurrency. A later implementation must
cite this file as `ParentErrorClassContract`, report measured processor
parallelism, physical and available memory, the derived default and any explicit
override separately, and keep the explicit override authoritative.
