# Newly coincident distinct keyboard attacks: error-class definition

Stage A on accepted r1560. This take freezes the semantic relation only. It adds
no runtime classifier, placement predicate, candidate policy, musical write,
proof activation or release gate. A later implementation must name this unchanged
file from its actually accepted parent.

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
ErrorClassName: newly_coincident_distinct_keyboard_attacks
CausalFeature: distinct active regular attacks have independently carried canonical source and attack identities within one authoritatively bound physical keyboard domain
CausalFeature: their exact raw onset and physical key pairs differ within that source domain
CausalFeature: an onset or key placement maps those distinct raw pairs to the same exact placed onset and physical key pair on that keyboard while retaining both attack obligations
CausalFeature: written staff, technical voice, visibility and serialization order are presentation facts and do not partition the physical keyboard equality relation
NonCausalContextExcluded: project or etude identity, measure number, absolute pitch or register, spelling, numeric source or attack identity values, numeric staff or voice labels, profile names, iteration numbers, owner names, corpus rarity, candidate counts, rendered appearance and renderer success

## Semantic relation

For two distinct active regular source attacks a and b on the same bound physical
keyboard, let R(a) be the exact pair (raw absolute onset, raw physical key), and
P(a) the exact pair (placed absolute onset, placed physical key). The requirement is:

    P(a) = P(b) implies R(a) = R(b).

The defect is the existence of a pair for which P(a) = P(b) and R(a) != R(b).
Both attacks remain separately identified obligations; preserving their total
count does not preserve their distinct temporal/key relation.

The raw and placed coordinates need not use the same units. Equality of raw
pairs is evaluated inside the common exact source domain; equality of placed
pairs is evaluated inside the common exact placement domain. They must not be
compared numerically across unrelated grids. A local partition such as a measure
is admissible only with a proof that its key denotes the same absolute-time
relation for the supported operations. Equal local offsets in different measures
alone do not establish equality of placed times.

Independent instruments are separate physical domains. Conversely, two staves,
voices, MIDI track labels or serialization channels are not evidence of separate
keyboards. The implementation must establish the actual supported domain rather
than infer it from those presentation labels.

## What is allowed and what is outside this class

Two source attacks with the same raw onset/key pair are source unisons. They are
not members of this class merely because they remain simultaneous after
placement. This exception is equality of raw pairs, not overlap of raw intervals.
For example, distinct raw onsets with overlapping durations still remain distinct
onsets and may not be collapsed under an overlap exception.

This permission is only absence of this R6 defect. It does not override another
existing safety condition, authorize a particular destination, or require all
source-unison participants to remain immovable. No blanket source-unison filter
or special corpus list follows from this definition.

The class is pointwise in onset and physical key. It defines no new general rule
for overlapping positive-duration intervals at different onsets. Existing A7
interval, tie, channel and source-binding protections remain unchanged. A later
implementation may not claim that an interval predicate with a raw-overlap
exception subsumes A7 without proving that no previously refused case is newly
permitted.

A tied continuation is a representation of an already identified attack, not
an additional source attack. Its exclusion requires its carried continuation
relation. Missing source_event_id or attack_id is unknown identity or role,
not proof that a pitched fragment is a continuation. A tie-stop marker alone is
not a replacement for a valid carried relation.

Grace-event placement is outside the currently supported regular-attack domain.
Excluding it does not certify its correctness or grant a new placement authority.
Unknown domain, source identity, attack role, raw key or time binding cannot yield
positive admissibility for a proposed move whose safety depends on that fact.

## Necessity review

1. Distinct carried attack identities are necessary because physical note heads,
   tied fragments and copied representation bytes do not themselves establish
   separate attack obligations. Identity may not be reconstructed from pitch,
   onset, document order or final XML similarity.
2. One bound physical keyboard is necessary because equal keys and onsets on
   independent instruments do not establish the physical coalescence in this
   class. Staff and voice alone do not establish that independence.
3. Different raw pairs are necessary to distinguish newly created coincidence
   from genuine simultaneous source unisons. Overlapping source intervals do not
   establish equality of their attack positions.
4. Equal exact placed pairs are necessary: same pitch at different onsets, or
   different keys at one onset, is not this defect. General interval safety is
   independently governed.
5. Considering every unordered participating pair is necessary: checking one
   representative of a group can miss a different raw class inside that group.
6. Carried source truth and bound time domains are necessary because matching
   later coordinates cannot supply the provenance or timing premise of the rule.

## Separate obligations of a later implementation

The future MusicMutation must carry a concrete implementation/repair contract
referencing this unchanged accepted definition. It must prove the data and
execution boundaries of all actual affected placement paths, including paths
that happened not to win in the observed corpus.

Each alternative must begin from its own actual immutable phase seed and its
carried identities. For the existing sequential edit model, earlier confirmed
planned moves reserve their actual proposed destinations and vacate their former
positions before later moves are checked. An unconfirmed future move supplies no
speculative vacancy. Rejected alternatives must not reserve positions in another
alternative. This requirement does not mandate a new matching algorithm or a new
ordering of independent alternatives.

The future implementation must prove the same binding, admissibility predicate
and occupancy progression for the corresponding evaluated and executed proposal.
The checked geometry must be the geometry actually offered to the existing
writer, including chord attachment, onset, duration, key, staff and voice where
relevant. A slot/atomic mismatch is not permission to check one interval and
write another.

This definition authorizes no new score, penalty constant, tie-break rule,
search budget, tone-set selection, source-unison freeze, visibility change,
source omission or writer path. Existing historical differences in treatment of
unplaced tones may remain explicitly documented; any deliberate scoring or
ordering change needs its own concrete justification in the implementation scope.
The definition does not claim full residual or ranking equivalence merely from
shared admissibility and simulation.

No admissible target must not discharge an attack. Where the existing operation
supports skipping that move, the attack stays at its current phase-seed position,
which is not necessarily its raw position. Preservation of that path must be
shown without a new removal, clone, restification or identity mint. An already
unrelated defect elsewhere supplies no local write authority.

Tests must be synthetic and independent of corpus pitches, source ID values and
measure locators. They must include genuine source unisons, distinct onsets with
raw interval overlap, different presentation channels, already reserved targets,
released old positions, absent or contradictory bindings, a phase seed previously
moved by an earlier owner, actual proposed chord geometry and retained A7 refusals.
Relabeling, valid relocation, irrelevant perturbation and independent-domain
candidate duplication must not change an unrelated local decision. Invariance
of the pair relation does not claim order independence of an existing greedy
complete plan.

The final R6 report remains a read-only observation in r1560. This definition
neither turns it into a release veto nor repairs any of its measured defects.
Later proof, receipt, identity and writer obligations remain in force.

## Prior knowledge and ordering disclosure

The r1555/r1556 comparisons, A7, the three remaining introduced pairs, the source
unisons and the r1560 R6 reader were known before this definition. Dachs' 060/061
analysis and Maus' counterexample to the raw-overlap exception were also known.
This text does not claim those historical explorations occurred after its freeze.

The Stage-A ordering fields describe a future distribution measurement of this
frozen general class, not retrospective ignorance of the corpus. No fresh class
measurement or executable selector is included in this take. The relation is
derived from distinct source attack obligations and physical key/time equality;
its membership does not change if a known witness disappears or additional
independent examples are found.
