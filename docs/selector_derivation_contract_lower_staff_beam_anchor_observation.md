# Existing chord/beam anchor observation (r1504)

SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: error_classification
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: measurement_only
CorpusDistributionMayRefineClassToKnownWitness: no
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
ParentErrorClassContract: docs/selector_derivation_contract_lower_staff_chord_attack_projection.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: the accepted chord-attachment contract requires self-position anchors to reference the resolved serialized head and forbids a surviving older tie or beam origin from granting an unresolved member a clef-transition anchor
CausalFeature: the existing scanner already carries the head-derived self-anchor, tie-derived anchor, stored per-voice primary-beam origin, masked beam anchor and combined effective anchor; the diagnostic projects these exact facts without changing their computation or replacing missing attachment
CausalFeature: part-local serialized note and measure distances plus intervening real XML cursor/state elements describe the context of those existing anchors; a stored origin can predate the beam begin due to the existing tie policy, so stored origin and effective anchor are reported separately
NonCausalContextExcluded: project identity, etude identity, printed measure labels, absolute byte values, voice/staff label values, pitch, register, rarity, discovery order, corpus frequency, output count and independent multiplicity as class membership or writing authority

The unchanged accepted r1500 definition remains byte-identical. This contract
records the generic gate's treatment of new diagnostic report control, following
the accepted diagnostic-projection pattern. It implements no new error classifier
for remote beams. It neither extends the frozen chord class to beam continuity
nor declares any reported distance a violation. All independent beam/tie authority
and safety checks remain exactly as before; no chord-class membership is inferred
from beam presence, distance, XML-gap count or absence of a final anchor.

The observation census includes every visible lower pitched event carrying a
recognized primary beam, not just members or putative failures. This is context
for the existing anchor invariants, never a productive selection or claim that
these events all belong to the frozen error class. Further chord members and
positive neighbors are retained. No report row can alter the scanner's return
value, solver, ranking, insertion, receipt or materialization. Existing no-beam,
invisible and upper events keep their original effects on scanner state.

All rows are preserved without top-N truncation or multiplicity veto. Independent
parts have independent scan state. Duplicating a part duplicates its observations
with only the descriptive part index changed and leaves the original facts and
plans unchanged. Synthetic tests cover this independence, unresolved member with
an older live beam origin, foreign head metadata, inherited tie origins, missing
origins, octave reset and observation-free versus observed scan equality.

The output is final-document evidence only, with explicit unknown and unavailable
states. A remote-beam error-class definition and any eventual productive repair
remain separate future work. This take grants neither.
