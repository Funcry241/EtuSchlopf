# Run request input admission: causal error-class definition

Stage A of `causal_class_first_v1`, written against accepted r1525.
This definition introduces no executable runtime classifier or repair. A later
implementation must reference these unchanged bytes in an accepted parent.

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
CausalFeature: a run request identifies source files and destination files; deleting or replacing a destination before resolving the effective sources and checking source/destination separation can destroy an input or a previous result before request admissibility is known
CausalFeature: distinct path spellings can designate the same filesystem target; source protection therefore cannot depend only on textual inequality of arguments
CausalFeature: missing, unreadable, structurally unsupported or ambiguous required sources cannot authorize destruction of previous result files; admission uses the supported source reader rather than a success claim inferred from filename existence
CausalFeature: a batch controller can remove output files before its child starts; admission in that child alone does not protect the controller's earlier destructive edge
CausalFeature: the invocation identity used for source resolution, output naming, report binding and backup selection must remain consistent; resolving a file argument must not silently move writes outside the backed-up output set
NonCausalContextExcluded: project identity, corpus membership, etude identity, concrete measure number, pitch, voice or staff labels, musical density, particular melody, solver ranking, corpus frequency, elapsed runtime and a known failing filename

## Cause and boundary

The class is destructive preparation before a run request is admitted. Admission
means that the effective required sources can be resolved and read under their
existing supported contracts, destination paths are separated from protected
sources and from each other, and request options needed before destructive
preparation are valid. It grants no musical selector, replacement provenance,
source-event mutation, output overwrite permission or audio-derived truth.

A source file is not established by its extension alone. A valid container can
still contain unsupported or unusable data. Existing source decoders remain
responsible for that distinction. This definition does not add a new decoder,
expand supported musical formats, authorize a missing WAV to change note
retention or claim that two recordings describe the same performance.

Filesystem paths are locators. A canonical path or native file identity, where
available, can demonstrate aliasing. Content equality is not file identity;
a conservative refusal based on equal source/output bytes must say so and
must not be reported as proof that two paths are hard links. Distinct output
files with equal content are not automatically the same destination.

Admission must precede the first destructive edge in each controller or child
that owns such an edge. This does not remove the independent requirement for
transactional protection against later failures, process termination, output
locks, disk errors or changes after admission. Early validation alone is not
crash recovery and does not establish that a future commit will succeed.

## Positive and negative neighbors

- Missing or invalid required source: reject before destructive preparation;
  a supported, readable source with disjoint output paths passes admission.
- Destination aliases a protected source or repeats another destination:
  reject; an unrelated destination remains admissible.
- Multiple matching sources without an unambiguous request: reject with an
  explanation; a genuinely unambiguous request can be resolved.
- File-argument source resolution changes the effective source path: preserve
  the output/backup identity or migrate both together; a valid source choice
  alone does not authorize unprotected output renaming.
- Batch child validation exists but the parent deletes first: the parent's
  edge remains unprotected; parent admission before that edge is necessary.

Renaming a project, moving it to another directory, replacing its music with
unrelated supported source data or adding independent projects does not alter
the local source/destination safety rule. No first-project exception, score
size threshold or corpus-specific list is justified.

## Prior knowledge and authority limit

The initial synthetic failure probes, the source-read findings and an unbuilt
runtime prototype with seven passing isolated source-module tests were known
before this text was frozen. The prototype was rejected by the local parent
dependency gate because this definition was not already installed. Those
experiments are exploratory evidence, not a retrospective claim of post-freeze
measurement. No musical corpus distribution for this frozen class is supplied.

Future normative tests must use synthetic file resources and byte-preservation
assertions. The native lifecycle record, not this document, determines whether
the introducing take is accepted. This file creates no writer, recovery,
release, calibration or gate exception.
