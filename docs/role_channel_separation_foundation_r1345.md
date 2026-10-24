# r1345 — Occurrence-bound presentation role foundation

Parent: accepted r1344, candidate SHA-256
ae6da7f7aeda1338c7c0e35ab425ea6e754cef3c48295d2dcc3a197849c3a404.
Version: 0.10.1839 (same-version Take). ChangeClass: KodexOnly.

The source snapshot in the later 20261011-081200 assistant export has SHA-256
e40564ba892ea280559405ae02745811ab4dac22d00b6645ce97b16eb10db250,
identical to the previously examined accepted source. The accepted capacity
class from r1326 remains unchanged. This candidate defines preservation
obligations for the planned role/channel separation without changing a selector.

## Mechanism and limit of the evidence

The accepted `serialization_plan.rs` records one presentation role per channel
and label. `channel_incompatible` conflicts every pair of classes with different
roles, irrespective of interval overlap; reusable channels also require equal
roles. This is consistent with current downstream consumers, which read role
through a voice label. Simply removing that edge could change rest and beam
decisions. Current canonical role derivation is not changed or discredited here.

The corresponding frozen temporal plan reports four upper/four lower lanes in
one diagnostic corpus part and four upper/three lower lanes in another, while
their final exports use nine and ten distinct labels. These are observations,
not independently established musical voice counts or bounded assignment proofs.
Existing written/sounding/HARD SAME constraints may still obstruct compaction.
Historical same-role clique obstructions cannot be bypassed by this foundation.

## Executable, corpus-independent binding specification

The following self-contained Python 3 example models a frozen occurrence-role
relation and a separate mutable channel assignment. It is deliberately not a
product allocator, classifier, reader or writer. Tuple keys below stand for
already authorized exact occurrence identities, not a derivation from XML.
Changing a channel cannot change a role. A channel-only lookup cannot represent
two distinct roles, whereas an exact occurrence lookup can. An unknown or
conflicting binding is rejected. A duplicate observation is idempotent;
distinct fragments remain distinct material.

```python
from dataclasses import dataclass, replace
import unittest

@dataclass(frozen=True)
class Fact:
    occurrence: tuple
    role: str

def freeze(facts, required):
    table = {}
    for fact in facts:
        if not fact.occurrence or not fact.role:
            raise ValueError('unknown provenance')
        if fact.occurrence in table and table[fact.occurrence] != fact.role:
            raise ValueError('conflicting binding')
        table[fact.occurrence] = fact.role
    if set(table) != set(required):
        raise ValueError('incomplete or unexpected occurrence coverage')
    return table

def observed_roles(table, channels):
    if set(table) != set(channels):
        raise ValueError('unbound channel occurrence')
    return {key: table[key] for key in channels}

class BindingContract(unittest.TestCase):
    def setUp(self):
        self.a = ('source-a', 'attack-a', 'fragment-a', 'pitched')
        self.b = ('source-b', 'attack-b', 'fragment-b', 'rest-carrier')
        self.facts = [Fact(self.a, 'UpperPrimary'), Fact(self.b, 'UpperSecondary')]
        self.table = freeze(self.facts, [self.a, self.b])

    def test_distinct_roles_survive_shared_channel(self):
        self.assertEqual(observed_roles(self.table, {self.a: 1, self.b: 1}),
                         observed_roles(self.table, {self.a: 8, self.b: 9}))
        self.assertEqual(len(set(self.table.values())), 2)

    def test_label_renaming_and_order(self):
        self.assertEqual(observed_roles(self.table, {self.a: 1, self.b: 1}),
                         observed_roles(self.table, {self.b: 823, self.a: 823}))

    def test_conflict_rejected(self):
        with self.assertRaises(ValueError):
            freeze(self.facts + [Fact(self.a, 'LowerPrimary')], [self.a, self.b])

    def test_missing_and_unexpected_rejected(self):
        for required in ([self.a], [self.a, self.b, ('missing',)]):
            with self.assertRaises(ValueError):
                freeze(self.facts, required)

    def test_duplicate_observation_invariance(self):
        self.assertEqual(freeze(self.facts * 3, [self.a, self.b]), self.table)

    def test_fragments_of_same_source_remain_distinct(self):
        c = ('source-a', 'attack-a', 'fragment-c', 'continuation')
        facts = [self.facts[0], Fact(c, 'UpperSecondary')]
        table = freeze(facts, [self.a, c])
        self.assertEqual(len(table), 2)
        self.assertNotEqual(table[self.a], table[c])

    def test_opaque_identity_bijection(self):
        renamed = {key: ('opaque', str(i)) for i, key in enumerate(self.table)}
        facts = [replace(fact, occurrence=renamed[fact.occurrence]) for fact in self.facts]
        table = freeze(facts, renamed.values())
        self.assertEqual({key: table[renamed[key]] for key in self.table}, self.table)

    def test_staff_permutation_transports_roles(self):
        swap = {'UpperPrimary': 'LowerPrimary', 'UpperSecondary': 'LowerSecondary'}
        facts = [replace(fact, role=swap[fact.role]) for fact in self.facts]
        self.assertEqual(freeze(facts, self.table),
                         {key: swap[role] for key, role in self.table.items()})

    def test_channel_coverage_rejected(self):
        with self.assertRaises(ValueError):
            observed_roles(self.table, {self.a: 1})

    def test_unknown_provenance_rejected(self):
        for fact in (Fact((), 'UpperPrimary'), Fact(self.a, '')):
            with self.assertRaises(ValueError):
                freeze([fact], [fact.occurrence])

if __name__ == '__main__':
    unittest.main()
```

These ten checks prove only the independence and fail-closed properties of the
stated binding relation. They do not prove product lineage completeness,
musical channel compatibility, consumer capacity or unchanged materialization.
Those remain explicit acceptance obligations in the new preservation contract.

## Additional consumer evidence

The independent two-measure capacity witness documented in r1326 was reproduced
before this candidate: ordinary MuseScore Studio 4.7.5 import rejects the expanded
channel form; compatible compaction imports and returns all eight pitched
pitch/onset/duration/staff tuples. This confirms technical reuse can resolve a
synthetic capacity failure. It does not prove this proposed role migration or
authorize any rewrite of the corpus. The r1331 correction concerning incomplete
historical readback reconstruction remains applicable.

## Validation and delivery

Run the executable Python block independently of the product. Check exact source
continuity against r1344 and call the unchanged accepted parent's
`take_change_control::validate_incoming_take` on complete parent/candidate trees.
No productive Rust changes are present; no new Rust tests or weakened gates are
included. The incoming archive excludes local root Cargo.lock and retains nested
tool locks. Windows acceptance and full-corpus results must come from the normal
Take transaction, not from these local foundation checks.

Put the complete candidate archive into the existing Take input and run:

    ./run.cmd --take --all

Only after acceptance may the implementation Take consume this foundation.
