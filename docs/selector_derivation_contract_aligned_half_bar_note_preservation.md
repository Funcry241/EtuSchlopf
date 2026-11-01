# Preserve an existing plain value at an ordinary 4/4 half-bar onset

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
CausalFeature: the received occurrence is already one visible pitched half or dotted quarter starting at a half-bar onset and contained in the same two-beat group of ordinary 4/4 on the existing canonical 96-division basis
CausalFeature: existing ties note-bound notations grace tuplets lyrics chord coupling or overlapping pitched support in the same staff refuse this preservation scope
CausalFeature: complete nonimplicit measured support and every direct note backup forward cursor operation remain inside the received measure
NonCausalContextExcluded: input identity corpus membership pitch register hand staff or voice label value measure label ordinal candidate count rarity rank renderer success page count and historical audit priority

## Parent definition and metric scope

The unchanged error-class definition was accepted before implementation and is
inherited byte-identically from r1539. The previous whole-bar repair remains
unchanged. This additional repair scope follows the two-beat grouping of the
received ordinary 4/4 context: onsets 0 and 192, with durations 192 or 144, do
not conceal the half-bar boundary. These are representable half and dotted
quarter values. Other positions and durations retain their previous partition.
This is a conservative first scope, not a universal rule for all meters or
syncopations. The numeric basis is the existing serializer's 96 divisions.

Nonoverlapping earlier or later notes in the same staff are independent. Any
pitched support overlapping the candidate in that staff refuses this initial
scope, including hidden support. A following chord member also refuses it.
This prevents detaching a hidden or visible chord member from its host. Other
staffs and measures do not provide ranking or singleton authority.

## Existing owner and invariants

Only the existing normalize_visible_note_values partition decision changes.
Admitted occurrences follow its existing unsplit/type-sync/continuation path.
No occurrence or existing tie chain is merged. Existing source, root, course,
staff, voice, onset and received notated interval remain bound to the original
owner. Cursor movement, rests, independent directions and pedal offsets retain
their support. There is no new writer, source identity or mutation capability.

The received notated interval is not a claim that a MIDI key-release duration
has the same length. The seed generator's existing source-to-notation duration
policy is unchanged. Separate articulation, outer-tie and unowned-chord defects
remain outside this take.

## Proof obligations and independent validation

Synthetic source-bound regressions cover both piano staffs, both half-bar
onsets, both values, repeated same-pitch attacks, complete chord membership,
independent duplication across staffs and measures, normalization stability,
source/root/course support and rejection of corrupted bound documents.
Counterexamples retain parent output for off-beat or half-bar-crossing support,
note-bound obligations, hidden chord coupling, overlapping voices, other
meters and overfilled cursors. Legacy output equality is a noninterference
check, not an endorsement of separately documented legacy defects.

The existing pause-separated-attack test now explicitly requires the retained
half plus the independent post-rest quarter. Its two-attack and sounding-event
preservation assertions remain mandatory; it no longer treats redundant
splitting of the first occurrence as the expected output.

A real-input single-project validation established byte-exact reproduction of
the accepted parent's master and render MusicXML, a local two-quarters-to-half
change, unchanged selected source plan and complete sounding-event projections,
and successful persisted MusicXML-4.0 and PDF release gates. This observation is
validation only; it supplies no selector constant or write authority. The
remaining corpus and Windows acceptance still belong to the user's take run.

The physical-birth handoff regression now receives 120 units, which genuinely
requires a quarter-plus-sixteenth partition. Its split, inserted-rest, birth
count and exact-binding assertions remain unchanged; an aligned 144-unit input
no longer supplies that split witness under the admitted policy.
