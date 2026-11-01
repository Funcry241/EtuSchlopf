# Source time notation projection: causal error-class definition

Stage A proposed on accepted r1533. Native acceptance remains pending. No runtime implementation, musical write or gate change.

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
CausalFeature: the notated time representation (measure boundaries, written meter and notated value of each source event) is produced from fixed assumptions instead of one explicit projection from the exact source timeline and its meter changes
CausalFeature: a source event whose exact onset and duration cannot be written as plain note values in the bound meter is either silently approximated or crashes instead of receiving an explicit, evidence-bearing notation class or a refusal
NonCausalContextExcluded: project identity, filename, corpus membership, measure number, pitch content, staff, voice, density, solver rank, audio evidence, renderer output


## Frozen scope and source custody

This single projection owns the explicit relation between exact source time,
source meter and notated location/value/event kind. Source PPQ, raw source
pair identity, exact onset and exact duration remain immutable provenance.
Notation is a separate derived coordinate space; no claim of acoustic-duration
identity may be made from a notated grace duration of zero.

Supported meter: 2/4, 3/4, 4/4 with chronological changes at bar boundaries.
Initial missing meter defaults to 4/4. Pickup handling is unchanged. Other
meters, mid-bar changes, tuplets and ambiguous notation are explicitly refused
before destructive preparation. This is an explicit supported scope, not a
rule inferred from the evaluation corpus.

A grace hypothesis is limited to a non-notatable source event shorter than a
sixteenth with exactly one contiguous predecessor and one following host in
the same track/channel, musical gap strictly below 1/192 quarter, a notatable
combined hostfix interval, and no second participating interpretation. More
than one coupled candidate is refused. Reverse encoding and ambiguous
polyphonic assignment are unsupported. Display is always a slashed eighth;
it does not assert original appoggiatura/acciaccatura intent. Source notes
are never discarded or replaced by rewritten MIDI. Existing source identities
must flow from exact import through selection, genesis, custody and lineage.

## Exploration and future implementation

Dachs' labeled experiments and the existing exact-import prototype are prior
exploratory evidence; they are not native acceptance of this definition.
Both planned takes are reviewed together against the unchanged proposed parent.
The definition stays byte-identical in the implementation take. No selector,
writer, gate, global meter variable, side list or count correction is installed
by Stage A. A later admitted implementation must retain all independent product,
source-identity, custody, schema and release checks.
