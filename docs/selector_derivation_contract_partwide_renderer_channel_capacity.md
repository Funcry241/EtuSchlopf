# Partwide renderer serialization-channel capacity failure

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
CausalFeature: a complete, exact written projection binds every represented pitched occurrence, actual pitched chord head, rest, and cursor operation to its existing production and role authority
CausalFeature: the intended renderer has an independently established bounded, partwide serialization-channel allocation profile; its slot demands are evaluated over the complete part, rather than separately for each measure
CausalFeature: the projected channel obligations within an explicitly shared staff-allocation resource group exceed that profile's available slots, leaving at least one independently required represented pitched occurrence without an admitted import channel
CausalFeature: rests or cross-staff heads may change a channel's import allocation mode and slot demands; this is a dependency of the complete consumer projection, not evidence that any rest or cross-staff notation is intrinsically wrong
NonCausalContextExcluded: project, etude, measure number, pitch, spelling, register, page, system, particular numeric staff or voice labels, source or occurrence ID values, candidate revision, rarity, corpus distribution, worker count and execution timing
NonCausalContextExcluded: renderer force flags, PDF existence, converter exit code, warning totals, foreign-note totals, preferred hand inferred from pitch, desired historical transfer direction and a majority vote as musical repair authority

## Class meaning

A serialization channel is a technical resource, not a musical voice identity.
Legal measure-local pitched interval allocation alone does not establish that a
consumer can import the complete part. Some consumers reserve only a bounded
number of channels for each staff over the entire part. Serializing more distinct
slot demands than the consumer can admit can discard required occurrences even
when every individual written duration is legal and no local pitched pair overlaps.

For the independently documented MuseScore profile, each staff has four import
voice slots. Its first import pass reserves ordinary voices on a preferred staff
and overlapping cross-staff voices separately on each affected staff. It counts
non-chord heads and rests over the whole part. A rest on another staff can therefore
change the mode of an otherwise stable channel and its resource demands. Legitimate
cross-staff notation is not itself a class member. Neither a raw maximum voice
label nor the number of labels in one measure proves the defect.

Classification requires complete resource demands and at least one required
pitched occurrence excluded by the bounded allocation. Missing production
provenance, unknown renderer profile or incomplete rest/cursor evidence is an
unknown closure failure, not a proof of safety and not permission to repair.
Consumer allocation counts describe technical behavior only: the consumer's
preferred-staff heuristic cannot grant canonical staff, hand or rest-role authority.

The class also covers resource exhaustion without a fatal import error. In a
consumer that returns early on a rejected chord member before suppressing that
member's cursor advance, exhaustion can additionally shift subsequent written
events and produce an overfilled imported measure. This downstream symptom is
neither necessary for class membership nor sufficient to identify it. A successful
conversion, schema validation, zero local collision count or existing PDF cannot
substitute for represented-occurrence coverage.

## Necessity review and invariance

- Exact production binding separates required represented material from material
  omitted under an independently accepted omission witness.
- The actual consumer profile establishes the bounded resource; an arbitrary
  threshold or a corpus-specific count cannot establish capacity.
- The complete part establishes all demands sharing that resource. A per-measure
  scan cannot prove a partwide reservation property.
- Excess demand and exclusion of required pitched material distinguish the defect
  from benign supplemental labels or sparse voices.
- Complete rest and head evidence establishes allocation dependencies without
  declaring rests, cross-staff chords or staff changes inherently invalid.

Renaming project, measure and occurrence identifiers or order-preserving numeric
channel labels cannot change resource infeasibility. Which occurrence a consumer
drops may depend on its priority rules; that choice is not musical repair authority.
Presentation-staff permutation with the corresponding profile and role bindings,
and uniform time-unit scaling, preserve class membership.

Multiplicity is evaluated within the explicitly shared bounded-resource group.
Adding obligations that require the same exhausted slots changes that group's
demands and is not an independent-candidate duplication. Unrelated parts, resources
or compatible obligations introducing no new slot demand cannot change membership
of an existing group. Corpus totals and global ranking of independent repair
candidates are forbidden as write authority. A later implementation must separately
prove the accepted duplication invariant and universal compatibility for every
reuse, including rest and chord-host obligations.

## Authority boundary

This definition grants no writer, selector, label rewrite, rest relocation,
rest suppression, hand transfer, canonical chord split or force fallback.
The existing written pitched-pair class remains unchanged. HARD SAME, canonical
chord heads, presentation roles, visible rest intervals, cursor semantics and
source/fragment/attack continuity remain independently required.

A later repair must use existing canonical role and structural write authorities
to provide a complete, compatible and consumer-admissible channel assignment.
It must preserve every required pitched occurrence's onset, duration, pitch and
tie continuity, and every independently required rest obligation. If bounded
assignment or provenance cannot be proved, it must fail closed instead of dropping
material, fabricating a pitched host or adopting a successful PDF as proof.

No productive detector, allocator, writer, caller, SSC envelope, adoption receipt,
renderer command or runtime behavior changes in this definition take.

Primary implementation references, independently of any project:

- [MuseScore v4.7.4 import allocation](https://github.com/musescore/MuseScore/blob/v4.7.4/src/importexport/musicxml/internal/import/importmusicxmlpass1.cpp): `allocateStaves`, `allocateVoices`, `determineStaffMoveVoice`, `note`.
- [MuseScore v4.7.4 import cursor](https://github.com/musescore/MuseScore/blob/v4.7.4/src/importexport/musicxml/internal/import/importmusicxmlpass2.cpp): `note` returns on unavailable channel before the final chord/grace duration suppression; `measure` advances positive returned duration.
- [MuseScore v4.7.4 score check](https://github.com/musescore/MuseScore/blob/v4.7.4/src/engraving/dom/check.cpp): `Score::sanityCheckLocal`.
