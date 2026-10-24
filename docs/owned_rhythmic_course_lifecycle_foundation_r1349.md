# r1349 — Constructive rhythmic course lifetime

Parent: accepted r1348, candidate SHA-256
1f215d33761f0f1f88986e8bcc9239e895f089ea7e6cb1dcbb5defdb3c0fd097.
Version: 0.10.1839. ChangeClass: KodexOnly.

## Result and why another definition was necessary

r1348 specified the evidence needed for a rest license but supplied no musical
lifetime rule. This Take defines the deliberate lifetime choice in
owned_rhythmic_course_lifecycle_contract_v1.md: a primary course expressly created
by the arrangement owner covers its planned notation domain, while an independent
branch covers only its expressly declared support components. A gap cannot
create, extend, join or terminate a course. Existing obligations and sounding
source roles are not retroactively reassigned.

The rule is an editorial choice for generated notation, not a theorem that MIDI
reveals melodic voice identity. Its adoption introduces musical authority and
must receive the true change class and complete runtime proof. Documentation
acceptance does not authorize a new issuer or writer. Canonical role and capacity
contracts are unchanged. No productive Rust, output, capability or gate changes.

The read-only investigation preceding this Take found:

- midi_seed.rs::build_musicxml/append_staff_voice emits gap/trailing rests before
  the VisibleRhythmLedger. Its source binding rows are emitted only for pitches.
  The document GENESIS identity does not supply per-rest musical responsibility.
- arrangement.rs::apply_visible_rhythm_ledger fills parsed staff/voice gaps and
  carries source-fragment rows only for surviving normalized notes.
- FrozenSoundingTemporalLanePlan uses temporal interval coloring, which supplies
  no course lifetime. The canonical source-role plan remains pitched-source bound.
- CanonicalFinalChord serialization_run_id is an auxiliary serialization grouping,
  not a negative-rest musical course; tie/chord continuity covers specific material.
- PreSerializationClosedSameLine candidates require contiguous pitched members
  and block on unbound rest occupancy. StructuralSnapshot's VoiceRuns group XML
  labels; HAN03 temporal edges also use existing staff/voice equivalence.

Consequently the prior two-Take target was not a proven implementation plan.
This Take supplies the constructive policy that was missing from r1348. It does
not assert that all producer migrations, role transitions, lineage, readers and
bounded allocation will fit in one next Take. An arbitrary license assertion,
existing XML voice, or Unknown-only transport is not the implementation closure.

## Executable model

Run the following Python 3 block. Creation strings denote hypothetical, already
admitted owner events; the model does not authenticate a product capability or
mint opaque product identity. It does construct each policy's coverage at birth
rather than assuming an unexplained full-coverage license.

Activity is explicitly course-bound. Metric notation, chord/tie hosts, original
import lineage, multiple role/staff segments, physical cursor realization and
actual consumers remain runtime acceptance obligations outside this model.
Continuity is passed from creation, never solved from musical event similarity.

```python
from dataclasses import dataclass, replace
import unittest

@dataclass(frozen=True)
class Course:
    identity: str
    kind: str
    role: str
    staff: str
    coverage: tuple
    parent: str = ''

@dataclass(frozen=True)
class Activity:
    occurrence: str
    start: int
    end: int
    kind: str
    role: str
    staff: str
    course: str

# Synthetic admitted owner inputs stand in for product capabilities. These
# constructors model a deliberate coverage choice, not authentication or IDs.
def interval_union(intervals):
    result = []
    for start, end in sorted(intervals):
        if end <= start:
            raise ValueError('invalid support')
        if result and start <= result[-1][1]:
            result[-1] = (result[-1][0], max(result[-1][1], end))
        else:
            result.append((start, end))
    return tuple(result)

def born_primary(creation, planned_domain, role, staff):
    if not creation or not role or not staff:
        raise ValueError('unbound primary decision')
    support = interval_union((planned_domain,))
    return Course(creation, 'primary', role, staff, support)

def born_branch(creation, parent, declared_support, role, staff):
    if not creation or not parent or creation == parent or not role or not staff:
        raise ValueError('unbound branch decision')
    coverage = interval_union(declared_support)
    if not coverage:
        raise ValueError('branch has no declared support')
    return Course(creation, 'branch', role, staff, coverage, parent)

def continued(course):
    # Metric fragments, relabels and measure cuts do not change the license.
    return replace(course)

def extended(course, explicit_decision, additional_support):
    if not explicit_decision or not additional_support:
        raise ValueError('missing coverage-extension decision')
    return replace(course, coverage=interval_union(course.coverage + tuple(additional_support)))

def ended_at(course, explicit_decision, boundary):
    if not explicit_decision:
        raise ValueError('missing early-termination decision')
    coverage = tuple((a, min(b, boundary)) for a,b in course.coverage if a < boundary)
    return replace(course, coverage=interval_union(coverage))

def rests(course, activity, complete=True):
    if course is None or not complete:
        raise ValueError('unbound course or incomplete inventory')
    rows = {}
    for row in activity:
        if (not row.occurrence or row.start >= row.end
                or row.kind not in ('pitched', 'hidden-pitched', 'existing-rest')
                or row.course != course.identity
                or row.role != course.role or row.staff != course.staff
                or not any(a <= row.start and row.end <= b for a,b in course.coverage)):
            raise ValueError('unbound course transition or activity outside support')
        if row.occurrence in rows and rows[row.occurrence] != row:
            raise ValueError('conflicting occurrence')
        rows[row.occurrence] = row
    rows = list(rows.values())
    for i, left in enumerate(rows):
        for right in rows[i+1:]:
            if (left.start < right.end and right.start < left.end
                    and 'existing-rest' in (left.kind, right.kind)):
                raise ValueError('existing rest conflict needs separate proof')
    result = []
    for a,b in course.coverage:
        occupied = interval_union((x.start,x.end) for x in rows if a <= x.start and x.end <= b)
        cursor = a
        for start,end in occupied:
            if cursor < start:
                result.append((cursor,start))
            cursor = end
        if cursor < b:
            result.append((cursor,b))
    return tuple(result)

class CourseLifecycleContract(unittest.TestCase):
    def setUp(self):
        self.course = born_primary('creation-a', (0,12), 'UpperPrimary', 'upper')
        self.activity = (Activity('attack-a',2,5,'pitched','UpperPrimary','upper','creation-a'),
                         Activity('attack-b',8,10,'hidden-pitched','UpperPrimary','upper','creation-a'))
    def test_primary_lifetime_before_activity(self):
        self.assertEqual(self.course.coverage, ((0,12),))
        self.assertEqual(rests(self.course,self.activity), ((0,2),(5,8),(10,12)))
    def test_empty_primary_remains_responsible(self):
        self.assertEqual(rests(self.course,()), ((0,12),))
    def test_sound_on_each_side_does_not_authorize_a_course(self):
        with self.assertRaises(ValueError): rests(None,self.activity)
    def test_bounded_branch_has_no_outside_rest(self):
        course = born_branch('branch-a','creation-a',((2,5),),'UpperAuxiliary','upper')
        rows = (Activity('a',2,5,'pitched','UpperAuxiliary','upper','branch-a'),)
        self.assertEqual(rests(course,rows), ())
    def test_disconnected_branch_never_takes_hull(self):
        course = born_branch('branch-a','creation-a',((2,5),(8,10)),'UpperAuxiliary','upper')
        self.assertEqual(course.coverage, ((2,5),(8,10)))
        self.assertEqual(rests(course,()), ((2,5),(8,10)))
    def test_branch_can_explicitly_license_internal_silence(self):
        course = born_branch('branch-a','creation-a',((2,10),),'UpperAuxiliary','upper')
        rows = tuple(replace(x,role='UpperAuxiliary',course='branch-a') for x in self.activity)
        self.assertEqual(rests(course,rows), ((5,8),))
    def test_no_implicit_extension(self):
        with self.assertRaises(ValueError): extended(self.course,'',((12,15),))
    def test_explicit_extension(self):
        self.assertEqual(extended(self.course,'extension-event',((12,15),)).coverage, ((0,15),))
    def test_declared_end_and_explicit_early_end(self):
        self.assertEqual(rests(self.course,())[-1][1],12)
        self.assertEqual(ended_at(self.course,'termination-event',6).coverage, ((0,6),))
        with self.assertRaises(ValueError): ended_at(self.course,'',6)
    def test_split_continuation_does_not_extend(self):
        split = (replace(self.activity[0],occurrence='fragment-a',end=3),
                 replace(self.activity[0],occurrence='fragment-b',start=3),self.activity[1])
        self.assertEqual(rests(continued(self.course),split), rests(self.course,self.activity))
    def test_existing_rest_is_not_new_creation(self):
        rows = self.activity + (Activity('rest-a',5,8,'existing-rest','UpperPrimary','upper','creation-a'),)
        self.assertEqual(rests(self.course,rows), ((0,2),(10,12)))
    def test_restification_keeps_licensed_support(self):
        rows = (replace(self.activity[0],kind='existing-rest'),self.activity[1])
        self.assertEqual(rests(self.course,rows), rests(self.course,self.activity))
        self.assertEqual(continued(self.course).identity,self.course.identity)
    def test_hidden_pitch_still_occupies_time(self):
        self.assertFalse(any(a < 10 and 8 < b for a,b in rests(self.course,self.activity)))
    def test_role_staff_divergence_requires_transition(self):
        for row in (replace(self.activity[0],role='UpperSecondary'),replace(self.activity[0],staff='lower')):
            with self.assertRaises(ValueError): rests(self.course,(row,))
    def test_observation_duplication(self):
        self.assertEqual(rests(self.course,self.activity*3),rests(self.course,self.activity))
    def test_independent_course_creation_stays_distinct(self):
        other = born_primary('creation-b',(0,12),'UpperPrimary','upper')
        self.assertNotEqual(self.course.identity,other.identity)
        other_rows = tuple(replace(x,occurrence=x.occurrence+'-b',course='creation-b') for x in self.activity)
        self.assertEqual(rests(other,other_rows),rests(self.course,self.activity))
        with self.assertRaises(ValueError): rests(other,self.activity)
        self.assertEqual(rests(self.course,self.activity), ((0,2),(5,8),(10,12)))
    def test_opaque_identity_bijection(self):
        course = replace(self.course,identity='opaque-course')
        rows = tuple(replace(x,occurrence='opaque-'+str(i),course='opaque-course') for i,x in enumerate(self.activity))
        self.assertEqual(rests(course,rows),rests(self.course,self.activity))
    def test_staff_and_role_transport(self):
        course = replace(self.course,role='LowerPrimary',staff='lower')
        rows = tuple(replace(x,role='LowerPrimary',staff='lower') for x in self.activity)
        self.assertEqual(rests(course,rows),rests(self.course,self.activity))
    def test_uniform_time_scaling(self):
        course = replace(self.course,coverage=((0,84),))
        rows = tuple(replace(x,start=x.start*7,end=x.end*7) for x in self.activity)
        self.assertEqual(rests(course,rows),tuple((a*7,b*7) for a,b in rests(self.course,self.activity)))
    def test_incomplete_conflicting_inventory(self):
        with self.assertRaises(ValueError): rests(self.course,self.activity,False)
        with self.assertRaises(ValueError): rests(self.course,self.activity+(replace(self.activity[0],end=6),))
    def test_invalid_creation_and_outside_support(self):
        with self.assertRaises(ValueError): born_primary('',(0,12),'UpperPrimary','upper')
        with self.assertRaises(ValueError): born_branch('b','p',(),'UpperAuxiliary','upper')
        with self.assertRaises(ValueError): rests(self.course,(replace(self.activity[0],end=15),))

if __name__ == '__main__': unittest.main()
```

The 21 checks cover primary birth before activity; silent primary coverage;
missing course authority; bounded/disconnected branch support; deliberately licensed
internal silence; absent/explicit extension; declared/early termination; conserved
split coverage; existing/restified obligations; hidden sounding occupancy;
unbound role/staff changes; duplicate observations; distinct independent creations
and rejected foreign-course activity; identity renaming; staff/role transport;
uniform time scaling; incomplete/conflicting inventories; invalid/out-of-support
inputs. No real corpus fixture or pitch heuristic participates.

The model has no project/measure/channel labels. Their real transport, role
segment changes and producer capability admission must be tested at actual owner
boundaries in the implementation. Synthetic interval checks do not prove playable
notation, preservation of the corpus or renderer capacity feasibility.

## Validation and delivery

Local checks: extract and execute this exact block; validate the final full tree
with the unchanged accepted parent admission gate; compare complete managed bytes;
verify archive integrity and exact payload; exclude the local root Cargo.lock and
retain all managed tool locks. Rust line balance +0/-0, net 0; all 336 Rust files
and Cargo.toml remain byte-identical to r1348. Full Rust checks and their measured
outcomes accompany delivery. Windows acceptance remains pending at delivery.

Place the complete candidate in the existing Take input and run:

    ./run.cmd --take --all

Only acceptance advances the green parent. The next productive work must consume
the frozen policy, establish admitted creation at genesis and each branch producer,
then transport exact responsibility through real source-role transitions and all
later readers/writers. The channel repair and completion without -f remain open.
