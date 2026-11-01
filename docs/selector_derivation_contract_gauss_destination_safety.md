# Gauss single-note destination safety

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
ParentErrorClassContract: docs/selector_derivation_contract_semantic_obligation_accounting_failure.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: the existing single-note gesture operation cannot independently rewrite a surrounding sounding continuation or authorize a second overlapping same-key obligation in the same target serialization channel
CausalFeature: before selecting a Gauss destination its exact existing physical anchor and the moved source are bound to the unchanged physical note parser; absent binding grants no destination
CausalFeature: a different physical note of equal sounding pitch in that target measure staff and voice occupies every positively overlapping half-open interval, including hidden chord members and already planned insertions
CausalFeature: rejecting that destination before the original writer runs preserves the surrounding notes and their existing tie obligations; it does not discharge an obligation or create a new representation authority
NonCausalContextExcluded: project identity measure label absolute pitch register enharmonic spelling numeric staff or voice values source ID value corpus rarity renderer success and number of independent candidates

## Scope

MusicMutation, accepted parent r1555. The frozen parent error-class contract
above stays byte-identical. This is a conservative safety restriction of the
already existing single-note Gauss operation, following the accepted consumer
safety boundary in selector_derivation_contract_initial_chord_course_consumer_safety.md.
It neither introduces a new semantic norm nor implements a general detector or
accounting gate for that parent class. Coordinate matching locates physical input
notes for refusal only; it is never certified SourceEvent lineage or a discharge.

The existing operation copies target duration, staff and voice into the inserted
note. Destination occupancy therefore uses that actual proposed written interval,
not the source's former duration. Source range exclusion prevents a note from
blocking its own move. All other members are examined irrespective of visibility,
whether they start or stop a tie, and their spelling. Pitch equality uses the
existing parsed MIDI value, with no fixed pitch or register threshold.

Only equal staff and voice relationships matter, never their particular labels.
End-to-start adjacency is not overlap. Another measure, another serialization
channel, or a different key does not establish this safety refusal. The existing
Gauss distance, score, rank and window policies remain unchanged; their minimum
is taken over remaining admissible destinations. No source is deleted merely
because it has no admissible destination.

The physical note list is updated after each planned move, before the existing
edit batch is materialized. Earlier insertions reserve their actual new intervals,
and an earlier moved source vacates its former interval. Future unexecuted moves
do not provide speculative vacancies. This is one explicit local conflict group,
not a scorewide quota or rarity condition.

## File scope and validation

Product changes are confined to the existing gesture decision owner,
tests/unit/gauss_destination.rs, this implementation contract and Take metadata.
The exact parent requirement for a selector contract was found in early admission;
the file list was explicitly replanned before adding this document. No gate,
writer, source identity owner, execution parser or frozen error definition changes.

Synthetic pure tests exercise both actual selection and destination refusal,
start/stop/independent and hidden enharmonic peers, overlapping holds, adjacency,
other scopes/channels/pitches, missing bindings and planned insertion conflicts.
Independent duplication and relocation must preserve unrelated decisions.
They invoke no music writer. The ten-piece corpus is evaluation only.

Delivery additionally requires all-target tests, Clippy, product build, exact ZIP
admission against r1555 and a complete original-input ten-piece run with exit 0.
Native output and image acceptance remain a separate verification after delivery.
