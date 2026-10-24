# Required pitched material in a headless renderer import channel

SelectorDerivationContract: causal_class_first_v1
SelectorChange: no
SelectorRole: error_class_definition
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
ClassDerivationBasis: independent native import-pass registration and lookup semantics; corpus witnesses do not define the predicate
ErrorClassDefinitionIndependentOfKnownWitness: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: not_yet_measured
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: none
CausalFeature: an exact complete written projection binds every represented pitched occurrence and its actual serialized import key to existing canonical source fragment attack and presentation authority
CausalFeature: at least one independently required participating pitched occurrence has positive written duration but its import key has no registering non-chord head or rest anywhere in the complete consumer part
CausalFeature: the independently established consumer pass registers channels only from participating positive-duration non-chord heads or rests and later requires a registered channel for every participating pitched member
CausalFeature: the resulting required pitched occurrence has no registered import channel before bounded slot admission is evaluated; this is distinct from a registered channel rejected by capacity exhaustion
NonCausalContextExcluded: project etude measure index pitch spelling register hand inferred from pitch source and occurrence identifiers raw voice label magnitude revision warning count corpus rarity worker count and historical intended repair direction
NonCausalContextExcluded: renderer force flags converter exit status PDF existence and a chosen witness's desired staff or voice destination

## Independent derivation and domain

MuseScore's first MusicXML import pass suppresses chord/grace cursor duration
before registering a voice descriptor. Participating ordinary heads and rests
with positive duration register a descriptor and increment its head/rest counts.
Its second pass resolves a pitched note's explicit voice to an import key and
requires successful staff/voice mapping. A positive-duration chord member can
therefore name a key that the first pass never registered. This registration
failure exists independently of four-slot capacity exhaustion.

The class requires exact complete origin coverage, complete head/rest/cursor
evidence over the part, and an independently established consumer profile.
Incomplete or unsupported timing evidence remains unknown. No local fragment,
maximum numeric voice label or missing head in one measure proves partwide
headlessness. A head elsewhere in the part excludes the headless predicate.
Existing print-spacing=no material is cursor-only in both passes and registers
no channel; it supplies no new permission to hide or omit required material.

A synchronous chord with a required pitched member is not intrinsically wrong.
Neither absence of a head nor a coincident onset authorizes a new musical chord,
rest host, transfer, omission or source identity. Another key's physical pitched
head cannot be adopted solely because it has matching pitch, duration, staff,
geometry or convenient capacity. Positive canonical joint-chord authority remains
mandatory. A rest can register a consumer key without becoming a pitched host.

## Necessity and invariance

1. Exact production binding identifies independently required represented material.
2. A participating positive-duration pitched occurrence distinguishes musical loss
   from unused labels and already authorized cursor-only representations.
3. Complete absence of registering heads/rests establishes registration failure;
   a sparse but registered channel is outside this class.
4. Native registration and later lookup semantics establish the causal mismatch
   without depending on conversion success, a particular label or corpus counts.

Project/measure/source renaming, uniform time scaling, corresponding staff/profile
permutation and injective import-key renaming preserve membership. Distinct lexical
labels aliasing one consumer key require complete alias-group evaluation rather
than label-string counting; this definition grants no permission to introduce
aliases or merge canonical roles. Adding an unrelated independent part or a
compatible occurrence that introduces no registering head cannot change the
original group's headless membership. Adding a registering head changes the
explicitly shared registration group and is not independent duplication.

## Authority boundary

This definition introduces no productive selector, allocator, writer, caller,
label binding, canonical role change, chord split, rest edit, visibility edit,
source/fragment/attack receipt change, SSC change or renderer command change.
All Rust remains byte-identical to accepted r1326. The accepted written-channel
and partwide-capacity classes remain unchanged.

A later implementation must reference this already accepted class separately
from its own repair contract. It must consume existing canonical occurrence,
chord, cursor, presentation and materialization authorities. A required source
may be materialized as its own ordinary pitched head only when an existing
positive source-bound structural authority proves that representation and its
full written onset/duration/tie/cursor obligations. This definition does not
create that authority. A canonical chord member may use another existing head
only under its existing positive joint-chord witness. Required rests and other
independent obligations remain mandatory. Unknown or unsatisfiable obligations
must fail closed; successful native conversion is not occurrence coverage.

Primary independently inspected source:

- [MuseScore v4.7.4 pass 1](https://github.com/musescore/MuseScore/blob/v4.7.4/src/importexport/musicxml/internal/import/importmusicxmlpass1.cpp): `note`, `notePrintSpacingNo`, voice registration after chord/grace duration suppression, `allocateStaves`.
- [MuseScore v4.7.4 pass 2](https://github.com/musescore/MuseScore/blob/v4.7.4/src/importexport/musicxml/internal/import/importmusicxmlpass2.cpp): `note`, explicit chord-member voice lookup, `determineStaffMoveVoice` failure before final chord duration suppression, `notePrintSpacingNo`.
