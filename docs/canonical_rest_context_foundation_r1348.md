# r1348 — Canonical rest-context definition

Parent: accepted r1347, candidate SHA-256
4656e57e6cbee41065d49d6486a0f9c1498aa343bebee15553bb4b4086746909.
Version: 0.10.1839. ChangeClass: KodexOnly.

## Concrete result and implementation boundary

canonical_rest_context_contract_v1.md separates existing-rest ancestry from new
licensed rest obligations, and musical course identity from occurrence identity
and presentation role. The causal dependency class is frozen separately in
selector_derivation_contract_unbound_rest_context.md. Both are definition only.
There is no Rust, capability, writer, musical selector or output change.
All previously accepted contracts remain byte-identical.

The current producer/role phase gap is grounded in arrangement.rs:
apply_visible_rhythm_ledger groups parsed events by staff/voice, generates XML
strings for uncovered time and omits creation lineage rows for inserted rests.
The role owner in structural_write_authority/serialization_plan.rs forms pitched
source roles after canonical written evidence; HARD SAME chords and temporal
lanes do not establish negative-rest course ownership. The lineage spine transports
source-bound existing fragments and cannot authorize the inserted unknown origins.
The r1126 rest-ownership shadow explicitly leaves negative responsibility unbound.
The r1136 interval projection protects an already-existing visible obligation;
it is not a new-rest license issuer and is not replaced by this definition.

A complete next implementation must supply an admitted musical course/coverage
issuer, show the real acyclic role/rest dependency path and close all producers
and consumers. That issuer is absent in r1347. This Take does not assert the
planned full closure fits in one implementation Take or that compaction is feasible.
An identity-only Unknown transport is not the promised implementation closure.

## Executable conditional obligation model

Run the following block with Python 3. It is documentation, not product code or
a Rust test. It assumes independently authorized licenses and complete bound
inventories; it neither authenticates these inputs nor creates their authority.
It models positive-duration interval subtraction and role/staff segmentation.
Metric realization, restified ancestry, grace/cursor/host/tie proofs, source
creation identity and actual consumer behavior remain outside this model.

```python
from dataclasses import dataclass, replace
import unittest

@dataclass(frozen=True)
class Segment:
    start: int
    end: int
    role: str
    staff: str
    visible: bool = True

@dataclass(frozen=True)
class License:
    decision: str
    course: str
    start: int
    end: int
    segments: tuple

@dataclass(frozen=True)
class Activity:
    occurrence: str
    course: str
    start: int
    end: int
    kind: str

@dataclass(frozen=True)
class Obligation:
    decision: str
    course: str
    start: int
    end: int
    role: str
    staff: str
    visible: bool

# Inputs assume an admitted owner already authorized the license and the exact
# inventory. This model cannot authenticate an issuer or derive a musical course.
def obligations(license, inventory, complete):
    if license is None or not complete:
        raise ValueError('missing authority or incomplete inventory')
    if not license.decision or not license.course or license.start >= license.end:
        raise ValueError('invalid license')
    segments = sorted(license.segments, key=lambda x: x.start)
    cursor = license.start
    for segment in segments:
        if (segment.start != cursor or segment.end <= segment.start
                or segment.end > license.end or not segment.role or not segment.staff):
            raise ValueError('unbound or conflicting segments')
        cursor = segment.end
    if cursor != license.end:
        raise ValueError('incomplete segment coverage')
    rows = {}
    for row in inventory:
        if (not row.occurrence or row.course != license.course
                or row.start >= row.end or row.start < license.start
                or row.end > license.end
                or row.kind not in ('pitched', 'hidden-pitched', 'existing-rest')):
            raise ValueError('unbound activity')
        if row.occurrence in rows and rows[row.occurrence] != row:
            raise ValueError('conflicting occurrence')
        rows[row.occurrence] = row
    rows = list(rows.values())
    for i, left in enumerate(rows):
        for right in rows[i + 1:]:
            if (left.start < right.end and right.start < left.end
                    and 'existing-rest' in (left.kind, right.kind)):
                raise ValueError('existing obligation conflict needs independent proof')
    result = []
    for segment in segments:
        cuts = sorted({segment.start, segment.end} | {
            bound for row in rows for bound in (row.start, row.end)
            if segment.start < bound < segment.end})
        pending = None
        for start, end in zip(cuts, cuts[1:]):
            occupied = any(row.start < end and start < row.end for row in rows)
            if occupied:
                if pending is not None:
                    result.append(Obligation(license.decision, license.course, pending,
                        start, segment.role, segment.staff, segment.visible))
                    pending = None
            elif pending is None:
                pending = start
        if pending is not None:
            result.append(Obligation(license.decision, license.course, pending,
                segment.end, segment.role, segment.staff, segment.visible))
    return result

class RestContextContract(unittest.TestCase):
    def setUp(self):
        self.license = License('owner-decision', 'course-a', 0, 12,
            (Segment(0, 12, 'UpperPrimary', 'upper'),))
        self.activity = (Activity('attack-a', 'course-a', 2, 5, 'pitched'),
                         Activity('attack-b', 'course-a', 8, 10, 'hidden-pitched'))
    def run_model(self, license=None, activity=None):
        return obligations(self.license if license is None else license,
            self.activity if activity is None else activity, True)
    def test_licensed_gaps_only(self):
        self.assertEqual([(x.start, x.end) for x in self.run_model()], [(0, 2), (5, 8), (10, 12)])
    def test_no_license_no_permission(self):
        with self.assertRaises(ValueError):
            obligations(None, self.activity, True)
    def test_incomplete_inventory_no_permission(self):
        with self.assertRaises(ValueError):
            obligations(self.license, self.activity, False)
    def test_hidden_pitch_not_silent(self):
        self.assertFalse(any(x.start < 10 and 8 < x.end for x in self.run_model()))
    def test_existing_rest_not_recreated(self):
        rows = self.activity + (Activity('rest-a', 'course-a', 5, 8, 'existing-rest'),)
        self.assertEqual([(x.start, x.end) for x in self.run_model(activity=rows)], [(0, 2), (10, 12)])
    def test_observation_duplication_idempotent(self):
        self.assertEqual(self.run_model(), self.run_model(activity=self.activity * 3))
    def test_conflicting_identity_refused(self):
        with self.assertRaises(ValueError):
            self.run_model(activity=self.activity + (replace(self.activity[0], end=6),))
    def test_equal_geometry_distinct_activity(self):
        duplicate = replace(self.activity[0], occurrence='independent-attack')
        self.assertEqual(self.run_model(), self.run_model(activity=self.activity + (duplicate,)))
        self.assertNotEqual(duplicate.occurrence, self.activity[0].occurrence)
    def test_role_staff_segments_explicit(self):
        license = replace(self.license, segments=(Segment(0, 6, 'UpperPrimary', 'upper'),
            Segment(6, 12, 'LowerSecondary', 'lower', False)))
        result = self.run_model(license=license)
        self.assertEqual([(x.start, x.end, x.role, x.staff, x.visible) for x in result],
            [(0, 2, 'UpperPrimary', 'upper', True), (5, 6, 'UpperPrimary', 'upper', True),
             (6, 8, 'LowerSecondary', 'lower', False), (10, 12, 'LowerSecondary', 'lower', False)])
    def test_missing_overlapping_unknown_segments_refused(self):
        for segments in ((), (Segment(0, 11, 'r', 's'),),
            (Segment(0, 8, 'r', 's'), Segment(7, 12, 'r', 's')),
            (Segment(0, 12, '', 's'),)):
            with self.assertRaises(ValueError):
                self.run_model(license=replace(self.license, segments=segments))
    def test_foreign_course_and_unbound_disposition_refused(self):
        for row in (replace(self.activity[0], course='other'),
                    replace(self.activity[0], kind='unknown'),
                    replace(self.activity[0], occurrence='')):
            with self.assertRaises(ValueError):
                self.run_model(activity=(row,))
    def test_existing_rest_conflict_refused(self):
        with self.assertRaises(ValueError):
            self.run_model(activity=self.activity + (Activity('rest-a', 'course-a', 3, 6, 'existing-rest'),))
    def test_uniform_time_scaling(self):
        factor = 7
        license = replace(self.license, start=0, end=84,
            segments=tuple(replace(s, start=s.start*factor, end=s.end*factor) for s in self.license.segments))
        rows = tuple(replace(x, start=x.start*factor, end=x.end*factor) for x in self.activity)
        self.assertEqual(self.run_model(license, rows),
            [replace(x, start=x.start*factor, end=x.end*factor) for x in self.run_model()])
    def test_opaque_identity_bijection(self):
        license = replace(self.license, decision='opaque-d', course='opaque-c')
        rows = tuple(replace(x, occurrence='opaque-'+str(i), course='opaque-c') for i,x in enumerate(self.activity))
        self.assertEqual(self.run_model(license, rows),
            [replace(x, decision='opaque-d', course='opaque-c') for x in self.run_model()])
    def test_role_staff_permutation(self):
        license = replace(self.license, segments=(Segment(0, 12, 'LowerPrimary', 'lower'),))
        self.assertEqual(self.run_model(license),
            [replace(x, role='LowerPrimary', staff='lower') for x in self.run_model()])
    def test_independent_group_duplication(self):
        before = self.run_model()
        license = replace(self.license, decision='decision-b', course='course-b')
        rows = tuple(replace(x, occurrence=x.occurrence+'-b', course='course-b') for x in self.activity)
        other = self.run_model(license, rows)
        self.assertEqual(before, self.run_model())
        self.assertEqual(other, [replace(x, decision='decision-b', course='course-b') for x in before])
    def test_fully_occupied_course_has_no_new_rest(self):
        self.assertEqual(self.run_model(activity=(Activity('whole', 'course-a', 0, 12, 'pitched'),)), [])
    def test_licensed_silent_course(self):
        self.assertEqual([(x.start,x.end) for x in self.run_model(activity=())], [(0,12)])

if __name__ == '__main__':
    unittest.main()
```

The eighteen checks cover licensed gaps, absence of a license, incomplete inventory,
hidden sounding time, existing rests, duplicate observations, conflicting identity,
distinct coincident activity, explicit role/staff segments, invalid segments,
foreign/unbound activity, rest conflicts, time scaling, opaque identity bijections,
role/staff transport, independent groups, fully occupied coverage and licensed
silence without sounding events. Test premises are entirely synthetic.
Project, measure and channel labels are absent from the model, so it does not
prove their actual product transport. Those checks remain required at real owner
and consumer boundaries. The supplied decision/course strings stand for already
issued opaque identity; no product identity is derived from their values.

## Validation and handoff

Local validation must include extraction/execution of this exact block, the
unchanged parent's incoming admission gate, complete managed-tree byte comparison,
root-lock exclusion and unchanged nested tool locks. Rust line balance is +0/-0,
net 0. All 336 Rust source files and Cargo.toml are byte-identical to r1347.
Measured local check outcomes accompany delivery; Windows acceptance remains pending.

Install the complete Take through the existing transaction and run:

    ./run.cmd --take --all

Only user-side acceptance advances the green parent. The delivered candidate does
not remove -f, compact channels or change rests; that original objective remains
open until actual admitted implementation and unforced exact readback succeed.
