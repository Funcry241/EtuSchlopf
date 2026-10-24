# Frozen channel allocation admits only consumer-reservable partitions (implementation scope)

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
ParentErrorClassContract: docs/selector_derivation_contract_partwide_renderer_channel_capacity.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
AdditionalAcceptedErrorClassContract: docs/selector_derivation_contract_chord_scoped_staff_projection.md
AdditionalAcceptedErrorClassContract: docs/selector_derivation_contract_headless_renderer_import_channel.md
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
ScorewideMultiplicityAsWriteAuthority: forbidden
CausalFeature: the consumer reserves at most four whole-part voices per staff; a regular voice that does not fit is re-selected forever (no-op setStaff(-2)), so it and every smaller regular voice stay unallocated and their notes are lost
CausalFeature: the greedy first-fit over the accepted class order can need more channels per role than the existing SemanticDifferent, WrittenDifferent and role edges require
CausalFeature: non-pitched carriers keep their unchanged written voice bytes; their label joins the channel owning the identical reserved canonical label or forms a voice of its own; they receive no role
CausalFeature: complete canonical production evidence binds a participating member to a different staff from its sounding physical host; one consumer chord object cannot preserve both presentation obligations, so the existing independent auxiliary representation is planned before HARD SAME sealing
NonCausalContextExcluded: piece identity measure label pitch register voice label value of pitched notes staff number as identity source identity value corpus distribution ranking and independent candidate count

## Scope
Owner `serialization_plan.rs::build_frozen_serialization_voice_channels_for_canonical`. The greedy
assignment is the first candidate of a deterministic search and stays byte-identical whenever the
consumer admits it. Only on overflow: one deterministic DSATUR colouring pass over the existing graph
for the roles of the failing staves, under exactly the existing edges; recheck; otherwise the first
candidate is kept and a checked registering-clique witness or `Undetermined` is recorded.
Transport: `FinalChordWrittenOccurrence.projected_note_start` (carried from the projector provenance row).
Not in scope: edge relaxation, slot extension, rest changes, writer reachability, label binding,
legacy preferences, later owners.

## Bound staff representation before canonical sealing

The existing canonical planning owner `plan_auxiliary_lane` admits its existing
source-bound independent AuxiliaryLane representation when a participating
pitched member has an independently bound staff different from its sounding
physical host. The complete existing production evidence binds SourceEventId,
AttackId, FragmentId, onset, duration, pitch, staff and ties before this decision.
The old same-staff head/member relation remains HARD SAME. Already removed or
rematerialized members and already sealed canonical plans/edges are untouched.
No physical staff or pitch changes. This closes the contradiction before a new
canonical HARD SAME class is created, rather than relaxing a frozen class.

For this staff representation, full original duration and every tie/notation
byte are retained; the legacy reattack shortening and tie/notation exclusion do
not apply. The existing non-overlapping complete-measure lane proof remains
mandatory, as do exact generated provenance, canonical fragment binding and
transaction-local Output-Alpha. There is no new writer, receipt, omission or
post-render cleaner. Cursor-only carriers do not qualify. This independent
scope relies on the unchanged accepted chord-scoped staff-projection definition.

Independent original physical cohorts are never joined solely by equal onset.
Within the new bound staff-separation representation, their positive-duration groups are partitioned into
non-overlapping serialization runs. A joint auxiliary chord requires the same
original physical host and bound staff; its actual generated head receives the
positive pre-seal HARD SAME edges. Existing orphan and same-staff long-duration auxiliary rules retain their prior
planning and rejection behavior in separate runs. The
unchanged materializer and binder consume the allocated run identities. The
legacy path without canonical evidence retains its existing behavior.

## Consumer-model limits

Feasibility is evidence for the complete projected planning epoch, not a proof
of the later final Render or native output. All later canonical constraints,
receipts, role bindings and final occurrence gates remain mandatory. No success
label grants release or musical completeness. Independent final native comparison
of pitch/onset/duration/staff/ties/chord obligations is required for acceptance of
the repair result; zero predicted slot loss alone is insufficient.

The current source/role carrier has no independent part-resource field. The
consumer search therefore retains the first assignment as Undetermined for
multi-part input; it never combines independent part slot budgets. Unsupported
or noncanonical numeric labels (including possible numeric aliases), unbound
note ranges and unreadable projection likewise retain the first assignment.
Unknown evidence is not permission. Every required pitched channel must own a
registering pitched head on its occurrence's staff; unchanged rest labels never
supply this positive guarantee. Search remains one explicit bound part/staff
resource group, never a piece/measure locator or cross-resource ranking.

## Finite technical alternative, no calibration change

The recursive 400,000-node Dachs prototype is not used. There is no new search
budget, timeout, musical score threshold or slot expansion. Each DSATUR iteration
colours exactly one existing class; saturation and remaining degree select the
next vertex within this explicit resource graph. Existing classes, roles and all
SemanticDifferent/WrittenDifferent edges remain unchanged. A candidate with more
colours than the first assignment is discarded. The complete candidate is then
checked against the same projected consumer obligations before it can replace
the first assignment. This polynomial pass is neither minimum-colouring proof
nor exhaustive search. Failure remains Undetermined unless an independently
checked registering clique of pinned roles itself exceeds the fixed consumer
capacity. No exhaustive-unsatisfiability verdict is produced.

## Material identity continuation

The production source-range binder carries exact CandidateMaterial fragments into
the existing forward-onset bridge. An unchanged representation occurrence returns
no attack translation after checking live source/attack identity, pitch, positive
fragment duration and exact bound measure/raw-index/onset. A tie-stop fragment may
retain its attack's earlier onset or measure; its SourceEventId and AttackId remain
mandatory. Non-continuations and actual translations retain the original attack
head measure/onset checks. Distinct occurrences of one tied attack do not create
duplicate attack translations. Generated provenance, final freeze equality and all
canonical/Output-Alpha obligations remain mandatory. No guard is bypassed for a
changed onset. Existing reconciliation still records the no-change material epoch.
