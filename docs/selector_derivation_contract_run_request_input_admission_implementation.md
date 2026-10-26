# Run request input admission implementation

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
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
ParentErrorClassContract: docs/selector_derivation_contract_run_request_input_admission.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: effective required sources and destination paths must be resolved and admitted before their controller removes previous outputs
CausalFeature: the established MIDI reader and WAV sample reader must accept the source data before destructive preparation; jobs and trace options must be validated before the corresponding controller edge
CausalFeature: ambiguous sources, canonical source/output aliases, repeated destination paths, symbolic output links, non-file targets and conservatively equal input/output bytes cannot authorize destructive preparation
NonCausalContextExcluded: project identity, corpus membership, concrete measure, musical content, note density, solver rank, elapsed time, filename chosen by a known witness

## Frozen parent and scope

The Stage-A definition is consumed byte-unchanged from natively accepted r1526.
The earlier prototype and synthetic probes were already known before its freeze;
they are exploratory evidence, not a claim of new post-freeze discovery.

This implementation moves the existing early checks before output deletion.
It changes no decoder, musical selection, audio-preservation policy, scoring,
writer authority, receipt, adoption, gate, recovery owner or output identity.
File arguments resolve their actual source from the stem while keeping the
complete caller argument as output identity, matching the launcher backup set.
Established project-name calls retain all source and output paths.

Batch admission covers every discovered MIDI/WAV stem, rejects incomplete pairs,
and checks all selected sources against all project and aggregate destinations
before the manifest or previous outputs are replaced. Those resources form one
shared filesystem transaction scope: a foreign input/output collision is not
an independent candidate. Unrelated projects are admitted by the same local rule;
there is no first-project, ranking or unique-candidate permission.

## Conservative boundaries

Canonical path comparison recognizes resolvable aliases. Equal input/output
bytes cause a conservative refusal; that is neither native file identity nor
proof that two files are hard links. Equal independent output files are allowed.
Windows uses case-insensitive path comparison conservatively. Native Windows
locks, ACLs, reparse points, concurrent changes and process termination require
separate native evaluation and the existing recovery owner's later extension.
No read-only admission can guarantee a future write will succeed.

MIDI without WAV remains unsupported. Missing audio is not replaced by invented
evidence. Missing-renderer preflight, late export failure, crash recovery and
custom/batch result transactions remain open. This bounded change cannot close
A01, A02, A05, A07 or A08 in their entirety.

## Proof obligations

Synthetic file tests exercise positive admission, source ambiguity, aliases,
unrelated equal outputs, independent duplication and relocation, missing inputs
and output directories, disguised sources and symbolic output links. Process
probes check invalid MIDI/WAV/options and batch failure before deletion by hashing
all prior inputs and outputs. Build, canonical Strict Clippy, full all-target
tests and the actual incoming gates are required before delivery. Real corpus
and native Windows acceptance remain separate evidence and are never inferred
from synthetic tests or from the parent run.
