# Staff notation context evaluated in serial order instead of musical time

Stage A defines a causal error class only. It introduces no classifier, repair
scope, comparison policy, boundary permission, writer or musical change. A later
implementation must reference this document unchanged from an accepted parent.

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
ErrorClassName: staff_notation_context_evaluated_in_serial_order_instead_of_musical_time
ClassDerivationBasis: staff notation state and simultaneous musical protection relations have effective musical times; XML positions locate representation bytes but do not establish those times across cursor movements or independently serialized streams
CausalFeature: the current input contains a notation reference or protected musical relation whose effective time domain and a relevant visible note or proposed notation boundary can be independently resolved in the same part and staff domain
CausalFeature: the consumer assigns that reference or protection context through serialized position intervals or prefix traversal rather than the independently resolved effective musical-time relation
CausalFeature: those two relations actually disagree for a relevant observation, causing an incorrect written-pitch reference, incorrect cost contribution, or incorrect presence or absence of a protection relation in a notation decision
NonCausalContextExcluded: project or file identity, printed measure number, pitch spelling or exact MIDI value, numeric staff or voice label, absolute byte position, event ordinal, fixed ledger count, fixed leap or duration threshold, note count and known output appearance
NonCausalContextExcluded: discovery order, worker count, independent candidate count, corpus frequency, scorewide uniqueness, renderer success and any specific desired clef or octave course

## Meaning and causal necessity

The error is substituting representation order for musical applicability, not
using byte coordinates at all. Byte coordinates remain legitimate source-bound
locators for materialization. Multiple streams, backup/forward or a direction
offset can expose the distinction; none of their particular syntactic shapes
or labels is required as a target-discriminating feature.

Each feature is necessary. Without independently resolved musical-time evidence,
a disagreement cannot be established and the case is UNKNOWN. Without a
serial-order substitution the cause may be a different observation or policy
error. Without an actual disagreement, a serial traversal can be a correct and
sufficient implementation for that input. The existence of several voices or
of a high ledger count alone does not establish this class.

Clef and octave display state determine the written pitch reference of the
staff at the note's effective onset. A tie continuation, chord membership,
primary-beam relation or simultaneous/sustaining event may also constrain a
proposed notation boundary. Their precise ownership and established scope rules
remain with the current owners. This definition does not replace those rules
with a new universal boundary prohibition.

Known output observations prompted the investigation. They are evaluation
examples, not feature-selection criteria. The features follow from the distinct
semantics of representation order and musical time and remain meaningful if all
known pieces are removed. Earlier independent diagnostic measurements are not a
governed classifier or a claim that this newly frozen class has been measured.
No implementation may narrow this class using those measurements.

## Positive neighbors and UNKNOWN

- A single monotonic stream can have identical serialized and effective order.
- Several streams can still yield the same effective staff context; multiple
  voices alone are not an error.
- An isolated ledger excursion can be an intentional winner under the existing
  aggregate costs. Correct context does not require a different clef.
- A correctly time-bound boundary can be rejected for ties, beams, chords,
  simultaneous occupancy, materialization safety or aggregate cost.
- Directions for a different staff do not establish local display state. Staff
  and part membership must be independently resolved before comparison.
- Conflicting simultaneous state declarations, unresolved onset or duration
  units, unmatched tie/beam evidence and unresolved chord attachment remain
  UNKNOWN. Their absence from a serialized prefix does not prove their absence
  at the effective musical time.
- Reordering independent serialization streams while retaining their musical
  events and effective state timeline must not change a musical decision merely
  because a different stream appears first. If the rewrite also changes musical
  applicability, that is not an equivalent-order counterexample.

## Separate obligations for a later repair scope

The existing clef-course decision owner is ClefReadabilityPrimaryCourse in
src/structural_write_authority/rustig_clef_planning.rs. The related observation
primitives, boundary observer, materializers and receipt/adoption paths retain
their current authority. This document grants none of them new write reachability.

A later repair must establish one coherent current-input time relation for
observation, candidate formation, course ordering, admissibility and all affected
normalization and postwrite checks. Fixing a pitch cost while a protection guard
still observes a different time domain is insufficient. Likewise, sorting notes
alone is insufficient if state intervals, chord attack counts, surrounding
carry or transition/excursion accounting remain tied to an incompatible order.

The implementation must independently bound its supported temporal and
materialization scope. Onsets, endings, cursor moves, duration units, direction
offsets, chord attachment, staff/part ownership, simultaneous state and
cross-measure carry require evidence appropriate to that scope. Equal-time state
and boundary decisions must not acquire contradictory independent permissions.
Unsupported evidence grants no new course or write; it must not be silently
reported as a temporally proved baseline.

Existing pressure, transition, redundancy and short-excursion weights are not
changed by this definition. Any later complete baseline/alternative comparison
must evaluate both courses in the same proven time domain, with their incoming
and outgoing state. A smaller isolated note cost or ledger maximum grants no
repair permission. An unchanged musical result is a legitimate outcome.

Sounding pitch, spelling, onset, duration, rests, visibility, staff/voice, chord
membership, beam levels and membership, ties and slurs acquire no alteration
permission. Joint octave/clef optimization, changing octave directions, beam
splitting, voice relocation or musical rewriting require their own existing or
separately established authority; they are not implicitly authorized here.

## Independent evidence required before productive change

Future normative tests must use synthetic music and current source-bound
observations. Cover both reference directions, state events in another serialized
stream, starts/stops and offsets, complete chord members, simultaneous and
sustaining foreign-voice events, matched and unmatched ties, complete and
incomplete beams, duration-unit changes, measure carry and contradictory or
unsupported state. Test safe and unsafe boundaries and equal/worse alternatives.

Metamorphic probes must preserve the event multiset and effective state while
renaming labels, relocating components, reordering equivalent stream encodings,
perturbing unrelated input and duplicating independent parts. Independent
candidates retain local decisions; scorewide ranking or uniqueness cannot mint
write authority. A complete part course may retain its explicitly shared carried
state conflict group without coupling independent parts.

The actual planner, normalization, original materialization, receipt/adoption,
persisted readback and reentry must agree on the same supported relation. Real
captured inputs and PDFs remain evaluation evidence only. Runtime acceptance
and rendered outcomes are separate proofs; this Stage-A document claims neither
a repaired course nor a changed PDF.
