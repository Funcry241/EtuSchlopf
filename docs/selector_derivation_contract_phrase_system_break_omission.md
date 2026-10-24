# Phrase-slur omission caused by a materialized system boundary

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
ClassDerivationBasis: the existing phrase-slur owner treats a materialized layout boundary between phrase endpoints as a reason to omit the phrase relation; changing the line layout does not change the musical phrase endpoints
CausalFeature: a phrase-notation intent describes an ordered musical relation with a start and an end in the same part
CausalFeature: at least one materialized system boundary lies after the start and at or before the end; a materialized new-page boundary is also a system boundary for this purpose
CausalFeature: the existing final notation owner withholds the phrase slur because that relation crosses the boundary, even when its remaining unchanged eligibility checks would permit materialization
NonCausalContextExcluded: project name, etude identity, printed measure labels, absolute corpus locations, page or system index, pitches chosen to identify a witness, corpus frequency, uniqueness, machine performance, and renderer force flags

## Causal meaning and necessity

A musical phrase relation connects musical anchors. A line or page boundary is
a layout property between those anchors. Treating that property alone as a
reason to suppress the relation makes phrase notation depend on line wrapping.
The defect class is the resulting omission. It is meaningful for unknown
scores and arbitrary labels, positions, durations and numbers of occurrences.

The intent supplies the relation to be represented; this class does not infer
a new phrase. Ordered anchors identify its two ends. The intervening materialized
boundary explains why this existing owner suppresses that relation. Without
that specific suppression the case is outside this class. The definition does
not classify ambiguous anchors, missing notes or unrelated omissions as this
failure merely because they happen near a break.

This definition has been fixed before measuring the distribution of this class
in the current corpus. Existing acceptance comparisons of whole documents do
not serve as class-distribution measurements. No current corpus address or
snapshot is a normative fixture or a productive selector.

## Separate boundary of the one subsequent implementation take

Only the existing final-layout/phrase owner may materialize the relation through
its existing insertion plan, receipt and adoption path. The later implementation
must reference this byte-unchanged contract from its accepted parent. It may
remove the system-crossing veto; it may not infer additional phrase intents.

All other current eligibility and conservation rules remain: existing phrase
policy approval, uniquely resolved visible anchors, ordered distinct endpoints,
single-pitch start and end, parseability, note-event cardinality, sounding/timeline
fingerprints, balanced slur pairs and the existing number-level capacity.
The document-wide existing-slur refusal and the chord-endpoint refusal remain
known limitations, unchanged by this work. Receipt, Custody, temporal checks,
channel allocation, musical selection and renderer commands remain unchanged.

Existing admitted phrase relations, their anchors and their assigned numbers
must be retained. New crossing relations must not displace existing relations
through shared number capacity. A change in capacity, global ranking of
independent candidates, or a new first/unique-winner rule is outside this scope.
The later implementation must prove the dependency handling for genuinely
overlapping number intervals without treating independent occurrences as one
global conflict group.

## Required evidence and bounded native sequence

Normative tests must be synthetic: equivalent anchors with and without a system
boundary, page boundaries, multiple intervening boundaries, non-crossing positive
neighbors, unchanged existing-slur and chord-endpoint refusal, missing/ambiguous
anchor refusal, preservation of note content and existing pairs, and unchanged
local outcomes under renaming, relocation and independent duplication. Capacity
neighbors must show that existing pairs are not lost or renumbered.

The native acceptance comparison against accepted r1450 must list every added
slur with both anchors. The only permitted XML changes are the additional slur
markup for phrases previously withheld by this boundary veto. In particular,
pitch, onset, duration, staff and voice bytes, all previous notation, and layout
directives remain unchanged. All ten projects, unforced -o PDF generation,
collision checks and channel-profile checks must pass. Actual cross-system
appearance must be inspected in the rendered result; successful XML parsing
alone is not visual acceptance.

At most two native takes are allocated to this topic: r1452 freezes this class
without Rust changes, and r1453 is reserved for the single scoped implementation.
The latter is prepared only after acceptance of this parent. If the scoped
implementation cannot satisfy these obligations within that budget, stop and
report the concrete obstacle instead of widening the topic or adding takes.
Custody and the unrelated r1451 preparation remain paused.

This parent is based on the natively accepted r1450 take with SHA-256
bd748654d99640cee15bffa4df646dc424dd3e4a0cb9f13bd9a4b82f65e45ebc.
It introduces no classifier, repair, selector, writer, runtime gate, test,
renderer invocation, profile change, authority change or musical mutation.
