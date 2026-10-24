# Batch project logs (K6)

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
ParentErrorClassContract: docs/selector_derivation_contract_final_primary_beam_serialization_voice_partition.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: the existing diagnostic stderr of every corpus project process is kept in that project's own log file and copied unchanged into the console log as one framed block after the process ends
CausalFeature: the existing opt-in console status records are read from the already piped project stdout and forwarded unchanged when they decode as valid records; the page-count preference transport on the same stdout is untouched
NonCausalContextExcluded: project identity, etude identity, source-id values, pitch, staff, voice label, corpus rarity, independent candidate multiplicity and log content as class membership or productive authority

As in the accepted original-beam diagnostic compaction contract, SelectorChange
records the generic gate's treatment of diagnostic control flow in new code.
The new branches only decide where already produced diagnostic bytes are kept
(project log file or the inherited stream when no file can be created), how a
log file is named (path-safe characters) and whether a stdout line is a valid
console status record. This is not a new musical error class, classifier,
candidate decision, repair, rejection, threshold or writer.

The frozen parent remains byte-identical. Project processes, their arguments,
musical decisions, MusicXML/PDF outputs, exit codes and the batch verdict are
unchanged; the batch still checks exit codes and output files exactly as before.
A missing or unreadable project log only loses a diagnostic copy. Duplicating
an independent project adds its own log and cannot change any productive
decision.

Synthetic tests cover safe log file names, block framing and record forwarding.
Native corpus evaluation must compare all Master/Render XMLs byte-for-byte and
PDFs against the accepted parent. No new productive write is authorized by this
diagnostic take.
