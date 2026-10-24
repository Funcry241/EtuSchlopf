# Written occurrence incompatibility in a serialization voice channel

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
CausalFeature: two represented pitched occurrences have independently complete production provenance from the canonical FinalChord projection and positive written durations on a common measure-local time axis
CausalFeature: their actual written intervals overlap with positive length; sounding attack intervals alone do not establish or refute this written relation
CausalFeature: they share a technical serialization voice channel although the canonical occurrence and chord authorities do not authorize their joint representation as one synchronous chord event
CausalFeature: the resulting written obligations cannot be represented within that channel as a single legal sequence of sequential notes and positively authorized synchronous chord members without changing an independently required written onset or duration
NonCausalContextExcluded: project, etude, measure number, absolute pitch, spelling, register, page, system, numeric staff or voice label values, occurrence ID values, historical candidate revision, corpus counts, rarity, worker count and execution timing
NonCausalContextExcluded: renderer force flags, warning totals, foreign-voice-note totals, hand preference inferred from pitch, and a known witness's desired transfer direction

## Class meaning

A technical voice is a serialization container. Independently required written
obligations cannot share it when their represented intervals overlap and no bound
canonical chord witness authorizes the combination. This relation is about written
notation after FinalChord materialization, not merely sounding attack durations.
The staff value is presentation context; equal or different staff numbers alone
neither prove a chord nor make an incompatible shared voice legal.

A common onset and positive durations bounded by the actual pitched chord head
are admissible geometry for synchronous MusicXML chord encoding. Equal member
and head durations are not required: a chord member may be shorter, but must not
extend beyond its head. Geometry alone never proves a canonical chord relationship.
Multiple pitches within
one positively bound legal chord are not independent sequential obligations and
must not be split solely because their intervals coincide. Technical backup runs,
legacy voice labels and common pitch cannot replace canonical occurrence identity
or chord authority. Rests and forwards remain governed by their existing owners;
this class does not grant permission to hide, insert or relocate them.

Missing production provenance is an unknown/evidence-closure failure, not evidence
that two notes are compatible. Every represented pitched occurrence must be bound
to its exact canonical source/fragment and projected note span, with its actual
written onset, duration, chord membership and presentation context. Explicitly
omitted material remains outside the represented-occurrence domain only under its
existing omission witness. Unknown IDs, missing rows and nonpositive durations
must never silently downgrade a productive check to sounding-only compatibility.

## Necessity review

- Exact represented occurrence provenance prevents geometry, pitch or voice-label
  matching from fabricating continuity between different source events.
- A common time axis and positive written intervals are necessary to establish
  actual overlap. Disjoint intervals and end-to-start adjacency are outside it.
- Shared technical channel is necessary for this defect: independent compatible
  serialization containers may legally represent overlapping musical lines.
- Lack of a positive canonical joint-chord witness distinguishes incompatible
  independent obligations from valid simultaneous chord tones.
- Inability to preserve required written obligations within the shared channel
  distinguishes a causal conflict from mere foreign-note or backup-run counts.

This class remains unchanged regardless of how many projects or occurrences are
later measured. Independent occurrences in another channel or measure do not
alter membership of an already bound local pair. Connected channel conflicts may
form explicit local conflict groups; scorewide candidate multiplicity is neither
an eligibility condition nor repair authority.

## Authority boundary

Classification grants no write permission and no hand or presentation-role change.
A later repair scope must be justified separately by existing canonical role,
chord, voice and identity authorities. In particular, channel conflict never proves
that either occurrence belongs to the other hand. Existing HARD SAME requirements
cannot be split to eliminate a counterexample. A contradictory HARD SAME class
must be reported or rejected under the existing fail-closed sovereignty rules.

No productive detector, channel allocation, label preference, caller, verifier,
SSC envelope or music writer is introduced or changed by this definition take.

## Standard-based geometry clarification

MusicXML 4.0 permits a `<chord/>` member to have a shorter `<duration>` than the
first preceding non-chord note. The member does not advance the measure cursor.
The positive chord witness must bind the actual pitched head, its onset/end and
the represented member to the same canonical joint-chord authority. A missing or
non-pitched host provides no positive chord authorization. Coincident intervals
without that authority remain independent obligations. A member exceeding its
head's actual written extent is not authorized by this duration rule.

Reference: [W3C MusicXML 4.0 chord element](https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/chord/).

This clarifies the general legal-encoding exclusion, independent of project,
measure, pitch, duration ratio or corpus distribution. It introduces no repair,
selector, writer or runtime fallback. No HARD SAME class may be split merely to
avoid a counterexample. A later implementation must establish chord legality from
complete head/occurrence provenance, never from equal-duration heuristics alone.
