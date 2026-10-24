# Selector Derivation Contract - Render Epoch Path-Scoped Continuity r1184

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
ParentErrorClassContract: docs/selector_derivation_contract_semantic_obligation_accounting_failure.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: the accepted parent class treats continuity across representation boundaries as an accounting question and forbids reconstructing semantic identity from physical coordinates, so this measurement-only classifier must first keep independent persisted Render-MusicXML byte streams separate before it can report continuity or drift between observations
CausalFeature: continuity membership is determined only by the normalized persisted file path of the already-observed Render-MusicXML write; that path scopes one physical byte stream and is never promoted to musical carrier identity semantic occurrence identity repair target or product authority
CausalFeature: the first observed boundary of each physical render stream is a seed because no prior after-fingerprint exists for that same stream; later continuity is reported only when the prior complete-byte after-fingerprint of that stream equals the current complete-byte before-fingerprint
CausalFeature: a mismatch between consecutive observations of the same physical render stream is measurement evidence of an unobserved transition or epoch reset only; it does not identify an owner, prove a semantic violation, select a repair, authorize a write, or discharge an obligation
CausalFeature: observations of an independent render path cannot change the continuity classification of another path; adding or interleaving an independent stream creates its own seed and leaves every pre-existing stream result unchanged
CausalFeature: the persisted report exposes only an opaque stream fingerprint and aggregate stream count, not the physical path, and carrier metrics remain explanatory measurements that never decide byte-stream continuity
NonCausalContextExcluded: project identity etude identity measure number page system staff voice onset duration pitch register hand label clef spelling known witness location and corpus rarity
NonCausalContextExcluded: exact physical path text stream ordinal boundary ordinal semantic kind name and opaque stream fingerprint as musical selector repair selector or writer authority
NonCausalContextExcluded: visible-rest counts hidden-rest counts note counts forward counts sounding-signature outcome and silence metrics as continuity selectors; complete-byte same-stream fingerprint continuity alone determines this measurement
NonCausalContextExcluded: scorewide stream count global ranking first-wins selection uniqueness of one stream in the current corpus and any feature chosen only to isolate a known r1182 drift row

## Derivation from the accepted parent class

The accepted `semantic_obligation_accounting_failure` contract already requires later accounting to preserve identity and correspondence across boundaries without rediscovering them from incidental physical coordinates. r1182 added read-only render-epoch measurement, but its continuity state used one previous-after fingerprint for all accepted render paths inside a project. The r1182 full-corpus measurement therefore could not distinguish an actual same-file unobserved transition from an ordinary observation of a different persisted render file.

r1184 narrows only that measurement binding. It does not define a new semantic obligation and does not alter the parent error class. A normalized persisted path is used solely to partition physical byte-stream continuity. It is intentionally weaker than semantic identity: equal musical content on different paths remains two streams, and a stream path never proves that two musical carriers correspond.

The first observation per stream has no same-stream predecessor and is therefore classified as a seed. For later observations, complete-byte fingerprints are compared only against the previous observation of that same stream. This eliminates cross-file contamination without interpreting the musical contents, project, measure, voice, staff or known witness.

## Multiplicity and independent-stream invariance

Interleaving an independent accepted render path cannot change the stored previous-after fingerprint of another path. Duplicating an independent stream adds only its own seed and later same-stream observations. No scorewide stream count, uniqueness rule or ranking changes whether an existing stream is continuous.

The synthetic regression control exercises the existing audit recorder directly with two accepted render paths and no productive writer or sink. Existing same-path positive and negative continuity controls remain the measurement oracle.

## Authority boundary

This contract covers selector-sensitive audit classification only. r1184 changes no music, productive selector, writer family, StructuralWriteAuthority surface, repair scope, threshold, MasterApply, proof, permit or release authority. The r1182 static destructive-effect inventory, generated transport, carrier census, observability projection and PrimaryBeamWriterDelta remain unchanged.

A `DRIFT_OR_EPOCH_RESET` result remains observation only. It cannot identify which transformation occurred between boundaries, cannot distinguish a legitimate reset from an unobserved mutation, and cannot be converted into a repair decision without a later independently governed take.
