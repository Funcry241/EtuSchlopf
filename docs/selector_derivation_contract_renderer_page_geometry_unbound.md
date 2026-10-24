# Renderer page geometry is unbound

SelectorDerivationContract: causal_class_first_v1
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionMayRefineClassToKnownWitness: no
SelectorChange: no
SelectorRole: error_class_definition
CorpusDistributionPurpose: not_yet_measured
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
ErrorClassName: renderer_page_geometry_unbound
CausalFeature: a product document is consumed by a renderer under a fixed declared physical paper, notation scaling and margin policy
CausalFeature: the consumed source does not bind a complete unambiguous geometry consistent with that policy, so consumer defaults or contradictory source fields can determine physical page layout
NonCausalContextExcluded: project identity, measure labels, pitch, voice, staff, piece length, corpus distribution, process load, invocation path and renderer success

The defect class concerns a document header and physical presentation, not
selection among musical occurrences or inference from rendered success.
The product paper decision is A4 portrait. Native configuration observation
supports scaling 7 mm per 40 tenths, page width 1200.33 and height 1696.72
tenths, and margins 85.7143 tenths on every side and both page parities.
Those exported values express nominal A4 in the installed consumer's units;
no independent calibration or hidden force/locale correction is admitted.

A later implementation may extend the existing E044 visual-layout-defaults
owner only: bind scaling and page-layout before system/staff layout, preserve
the same receipt mint and exact one-insertion journal, reuse matching complete
geometry, refuse incomplete, duplicate or contradictory geometry before writing,
and remain byte-identical without receipt on an already complete marked header.
Matching compares parsed finite numeric values exactly to the frozen decimal
constants; no tolerance, rounding-based admission or guessed conversion.
Singularity checks are confined to fields of one document header transaction,
never a scorewide occurrence count. Independent documents do not compete.

All Master, render, primary-beam/voice shadows, oracle candidates and final
renderer sources must inherit this binding before their first consumer call.
No new file writer, renderer option, font, appearance, music selector, SSC
verdict, quarantine, #56 operation, audit acceptance, gate or toolchain changes.
The child does not grant its own transition. This parent installs no code.

Native evaluation must expose changed layout-dependent voice labels, octave
continuation cues and phrase decisions. Tone pitches, semantic starts and
semantic durations remain invariant. A system-crossing phrase is governed by
the unchanged E007 policy, not a new slur deletion. Previously implicit A4
measurements are a diagnostic prediction, never the new accepted baseline.
The new baseline requires a fresh native qualified run and readback; later
byte comparisons bind that accepted source and its freshly persisted products.
