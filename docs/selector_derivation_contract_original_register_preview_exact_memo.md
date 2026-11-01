# Original register preview: exact same-input result reuse

SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: repair_scope
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: measurement_only
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: docs/selector_derivation_contract_joint_register_reading_context.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
NonCausalContextExcluded: corpus identity, printed labels, pitch identity as selector, rarity, discovery order and elapsed time

CausalFeature: a full proposal exactly equal to a previously evaluated proposal reads the same immutable original XML within one borrowed comparison context
CausalFeature: the concrete preview computes only a complete cost map or UNKNOWN from that input and proposal; it has no productive write authority

r1577 is performance-only on accepted r1576. The original comparison owner,
candidate visitation order, strict improvement predicates, tie handling, costs,
musical policy, protection rules, productive writers and C3 integration remain
byte-identical. No arbitrary callback is memoized. The concrete read-only
preview reuses a result only within its original immutable source context.
The context is dropped before source materialization.

The visible fingerprint locates a bucket but grants no equality authority.
Every hit compares the entire typed proposal with Eq: ordered plans, parts,
staffs, intervals, octave representations, repeats, both fingerprints and all
metadata. Collisions stay separate. Cached UNKNOWN, an empty successful cost
map and a cache miss are distinct. No entry crosses input epochs.

Storage is bounded to 4096 entries and 16 MiB of retained logical payload.
The payload counter includes proposal and result headers, plan and repeat
arrays, strings and cost entries; it is not an allocator or RSS measurement.
The entry bound additionally bounds tree/bucket overhead. When either limit
is reached, the full unchanged preview still runs. Capacity does not prune,
reorder, admit or reject a musical candidate. No clock enters the decision.

One console-only summary binds the original input SHA-256 and reports requests, exact hits, actual evaluations,
stored entries, logical payload and storage bypasses. These counters have no
verdict or selection authority. They are deliberately not added to result.txt,
whose existing musical evidence remains unchanged.

Differential tests compare complete maps with the original preview and actual
selected full proposals with the uncached original comparison. They exercise
colliding fingerprints, changed metadata, repeated/reversed visits, G/F input
epochs, nonempty valid costs, invalid XML, UNKNOWN, empty success, zero/one
entry capacity and zero payload capacity. A manual synthetic benchmark reports
timing and evaluator counts without asserting a timing threshold.

Native acceptance requires all twenty Master/Render MusicXML files byte-equal
to r1576, unchanged Live/C3 closure and successful completion of all ten etudes.
Measure the original comparison stage and total runtime on the same machine,
with the same job count and build profile. Local synthetic timing does not
establish a corpus speedup; a low native hit rate can make the improvement small.
