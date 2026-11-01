# Preserve an existing unambiguous whole-bar single tone before metric splitting

SelectorDerivationContract: causal_class_first_v1
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionMayRefineClassToKnownWitness: no
SelectorChange: yes
SelectorRole: repair_scope
ParentErrorClassContract: docs/selector_derivation_contract_redundant_metric_note_fragmentation.md
CorpusDistributionPurpose: measurement_only
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
ScorewideMultiplicityAsWriteAuthority: forbidden
CausalFeature: the received pitched occurrence already fills the complete measure from its start and is representable by one whole note in ordinary 4/4 on the existing canonical 96-division basis
CausalFeature: chord members outer ties note-bound notation obligations or additional visible pitched material in the same staff introduce unsupported coupling and therefore refuse this initial repair scope
CausalFeature: actual note intervals and MusicXML note backup forward cursor movement must remain inside the complete nonimplicit measure
NonCausalContextExcluded: input identity pitch value register hand staff or voice label value measure label candidate count rarity solver rank renderer success page count and corpus membership

The unchanged Stage A parent defines the error class. These narrower conditions
limit this first preservation to authority and representation cases actually
proved. 4/4 supplies an ordinary full-bar whole value; 96 divisions is the current
normalizer's value-to-type basis, not a witness identity. Unsupported time bases
and meters keep their existing handling. Candidate identification does not grant
permission to merge existing attacks or substitute source custody.

## Existing owner and invariants

Only normalize_visible_note_values changes its partition decision. Eligible
material follows its existing unsplit/type-sync/continuation path. The original
source occurrence, interval, course, staff, voice and all receipt/custody handoffs
are kept. No new writer, source identity, rest license or mutation authority is
introduced. Both the source-bound arrangement path and later normalization use
the same decision. The existing split paths stay in place.

An eligible tone is visible, pitched, not a chord or rest, starts at 0 and spans
384 units in an explicit complete 4/4 measure. Grace, time-modification, lyrics,
existing ties and all note-bound notations are excluded. Any additional visible
pitched material in the same staff refuses that local conflict group. This is
conservative protection of unsupported same-staff relationships, not a claim
that polyphony requires quarter-note splitting. Other staves and other measures
remain independent. Hidden material retains its existing obligations unchanged.

All parsed note support and direct MusicXML note/backup/forward cursor movement
must fit inside the measure. Implicit and unmeasured measures are excluded.
Independent pedal directions retain their exact serialized offsets and cursor
positions; this does not implement new pedal import or interpretation.

## Proof obligations

Test-only differential fixtures include the unchanged r1537 normalizer as an
explicit historical oracle. Separate attacks, existing chains, rests, other
meters, chords, same-staff extra voices and note-bound obligations retain the
parent output. Independent duplication across staves/measures and label/order
changes cannot create ranking, singleton or scorewide-count authority.
The actual source-bound owner must validate its complete resulting binding.

Six endpoint-cleanup tests in two modules previously relied on a full-bar
occurrence being split. Their source inputs now start 24 units after the measure
boundary, so real beat crossing still produces the necessary chains. Every
assertion stays unchanged.
The separate parent articulation, outer-tie and unowned-chord defects remain
excluded and remain recorded in the separate local investigation; they are not
fixed here or made normative success criteria by this take.
