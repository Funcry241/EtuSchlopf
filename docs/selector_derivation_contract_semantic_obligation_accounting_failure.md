# Selector Derivation Contract - Semantic Obligation Accounting Failure Error Class

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
ErrorClassName: semantic_obligation_accounting_failure
CausalFeature: a versioned semantic contract applied to structurally observed facts of an authoritative origin epoch establishes an obligation with its own identity and a domain of lane-time cells
CausalFeature: after genesis the obligation crosses at least one pipeline transformation or representation boundary before the final persisted epoch; provenance-bound continuation of its accounting identity is mandatory across every such boundary and is not an opt-in of an individual transformation, whether or not that transformation changes structural facts, occurrence or lane correspondence or the representation epoch
CausalFeature: provenance-bound accounting is not total on the obligation's origin domain: for at least one origin cell there is neither a complete certified lineage through every subsequent boundary to a fulfilment in the final persisted epoch nor contract-admitted terminal discharge evidence applicable to that cell under the obligation's own lineage; a missing or broken correspondence is itself missing accounting
CausalFeature: fulfilment and discharge are judged under the genesis semantic world of the obligation, consisting of the contract version that derived it, the structural-observation schema version whose facts that contract read and the identity and correspondence schema version that defines its lineage, and under the image rule that contract defines for the obligation kind; a later version of any of these three never decides whether an obligation derived under an earlier world is accounted
CausalFeature: a lineage counts as certified only if it is valid under the identity and correspondence schema of the genesis semantic world; acceptance by a particular checker or proof procedure is a detector property and never establishes accounting by itself
CausalFeature: correspondence used for this accounting is derived from carried identity and certified relations and is never reconstructed from physical coordinates of an intermediate or final representation
CausalFeature: accounting is per obligation lineage and per origin cell; equality of total obligation measure is insufficient because it admits compensation between different cells or lineages
NonCausalContextExcluded: project identity etude identity measure number page number system number human locator source-id value or corpus rarity
NonCausalContextExcluded: exact meter divisions tick values durations beat position number of rests notes or obligations and exact location inside the measure
NonCausalContextExcluded: exact staff number numeric voice label pitch register clef hand label or spelling
NonCausalContextExcluded: which writer owner code path or transformation step removed or altered the representation and whether a per-path guard exists for it
NonCausalContextExcluded: whether the existing cursor sounding pitch onset duration staff voice chord or tie projection gates accept the transformation
NonCausalContextExcluded: the concrete normative meaning of a particular obligation kind including whether a visible rest itself or the rhythmic presence of a voice is the obligation
NonCausalContextExcluded: uniqueness in the current corpus and any feature added only because it keeps or removes a known witness

## Causal definition

This class exists when an obligation that a versioned semantic contract derives from the authoritative origin epoch crosses one or more transformation or representation boundaries and afterwards has no complete account: accounting is not total on its origin domain. Some origin cell is neither continued through a complete certified identity-bound lineage to a fulfilment in the final persisted epoch, judged within the genesis semantic world of the obligation, nor terminally discharged by evidence that world admits for that obligation kind.

The class quantifies over the origin domain, not over the image of the correspondence. A cell whose correspondence is lost has no image at all; it is therefore unaccounted, even when every cell that still has an image is fulfilled. Accounting continuation is mandatory across every boundary after genesis; no individual transformation decides whether it owes continuation.

Meaning and proof procedure are kept apart. The genesis semantic world (contract version, structural-observation schema version, identity and correspondence schema version) fixes what an obligation means and when it is accounted. The proof regime (proof epoch, proof schema, pipeline contracts) only determines how accounting is demonstrated and may change without changing class membership.

The defect is structural. It is not located at a single deletion decision, and it does not depend on which owner or path altered the representation. It exists because the product state lacks a conserved object on which the survival or legitimate release of a semantic obligation must be accounted across representation changes.

Accounting itself is the object of this class. A change of the musical representation is therefore not a precondition: a step that leaves structural facts and correspondence relations untouched but breaks the accounting continuation of an obligation is a member, even if the resulting music happens to remain correct.

The class is norm-neutral. It references the versioned semantic contract but does not define what any particular obligation means. It therefore does not decide whether a visible rest is itself an obligation or only one admissible realization of a higher rhythmic obligation.

## Membership is not violation and not escape

Three states are deliberately kept apart:

- Accounting failure (this class): a complete, contract-conformant, identity-bound account is missing for at least one cell.
- Semantic violation: the final persisted state actually fails the contract for that obligation. It is decidable only where correspondence is reconstructible from carried identity; otherwise it is unknown, and unknown is never resolved through physical coordinates.
- Existing gate escape: the current projection gates accept the transformation. This is an observed property of the gate inventory and is not part of class membership, so a better future detector cannot reclassify the cause.

Missing proof is not proven falsity. After a later explicit activation of a proof regime, missing accounting may nevertheless block release fail-closed.

## Relation to the frozen visible-rest elision class

`visible_rest_elision_without_bound_rhythmic_dispensability_proof` (`docs/selector_derivation_contract_visible_rest_elision_rhythmic_obligation.md`) remains unchanged and valid. It is a decision-local authority defect at one productive elision owner.

This class is broader in two independent directions. First, it covers composition: every individual decision may be locally authorized while the composition of transformations still leaves a cell unaccounted, for example when correspondence is lost before a later step observes an apparently empty lane. Second, it covers obligation kinds beyond visible rests. Neither class replaces the other; a case may belong to both.

## Necessity review

1. Genesis through a versioned contract is necessary because observation yields facts, not obligations; without the contract the class would turn every observed structure into a norm.
2. Crossing at least one boundary after genesis is necessary because the class concerns loss of accounting continuation, not defects already present in the origin epoch. Requiring an actual change of the musical representation would wrongly exclude a broken accounting chain over unchanged music, and letting a transformation declare whether it requires continuation would give every step an opt-out.
3. Quantifying over the origin domain is necessary because quantifying over the image of the correspondence silently drops exactly those cells whose correspondence was lost.
4. Binding fulfilment and discharge to the genesis semantic world is necessary because the contract reads structural facts through an observation schema and lineage through an identity schema. A byte-identical contract evaluated over a changed observation or identity schema is a different semantic world; any of the three changing later could make a lost obligation look accounted without any visible normative change.
5. Defining certification by validity under the genesis identity schema is necessary so that a weak or changed checker cannot turn missing accounting into apparent accounting; otherwise class membership would again depend on the detector inventory.
6. Per-cell accounting under the obligation's own lineage is necessary because evidence is a shareable fact while obligations are not; lineage binding prevents one obligation from being paid with evidence belonging to another region.
7. Identity-derived correspondence is necessary because reconstructing identity from staff, voice, onset or pitch of an intermediate representation reproduces the very failure mode the class describes.
8. Rejecting measure equality is necessary because equal totals can hide a lost cell compensated by an unrelated one.
9. Excluding gate acceptance from membership is necessary so that the class stays causal and stable under changes to the detector inventory.

## Not measurable on the current product state

The current product state carries no certified lane or occurrence correspondence relations for accounting purposes. Applied directly, every contract-derived obligation would be unaccounted. Class membership therefore does not discriminate on the current product state, and a corpus distribution becomes meaningful only for a concrete, frozen instance of the semantic contract.

`CorpusDistributionPurpose` stays `not_yet_measured`.

## Evidence requirements for later takes

A later structural-observation take may introduce a read-only reader of structural facts and persisted correspondence relations without veto authority. It must not interpret facts as obligations. Its observation schema and the identity and correspondence schema must carry explicit versions, because they become part of every genesis semantic world.

A later normative take may freeze a concrete semantic contract. Its rules must have derivable, complete read dependencies; a rule whose dependencies cannot be derived is inadmissible. A change to the semantic contract is a normative change and must not share a take with a MusicMutation.

A later accounting or gate take must reference this contract unchanged from an accepted parent. Pipeline steps may only transport, partition, reparametrize through certified relations or annotate evidence; terminal discharge belongs exclusively to the final gate on persisted final bytes. Artifacts produced before activation remain unproven rather than retroactively red or retroactively green.

Normative tests derived later must be synthetic and corpus-independent. Renaming, relocation, irrelevant musical perturbation and independent duplication must not change the accounting result of an unrelated obligation.
