# Synthetic support-conservation foundation

This executable model specifies the local cursor mechanism before corpus
distribution. It does not call a productive writer, authenticate origins or
prove runtime transport. The actual-owner custody experiment found a failing
chord-support test; its prototype is withheld. The admitted source remains r1350.

Let an original host/member support start at s and last d. Let sequential head
fragments have positive durations p_i with sum d. The start seen by a member
written after every head fragment is s + sum(p_i except the last). Keeping the
member duration d there changes its support whenever that prefix is positive.
Transporting an authorized member alongside every matching host fragment can
conserve the interval union, subject to separate attack/tie/host authority.

This model deliberately supplies no rule for assigning an imported member to
a musical course, handling unequal support, issuing rests or authorizing XML.

```python
import unittest

def original(start, duration):
    assert duration > 0
    return (start, start + duration)

def trailing_member(start, duration, head_parts):
    assert head_parts and all(p > 0 for p in head_parts)
    assert sum(head_parts) == duration
    resolved = start + sum(head_parts[:-1])
    return (resolved, resolved + duration)

def complete_fragments(start, duration, parts):
    assert parts and all(p > 0 for p in parts)
    assert sum(parts) == duration
    cursor = start
    output = []
    for part in parts:
        output.append((cursor, cursor + part))
        cursor += part
    return output

def union(intervals):
    result = []
    for start, end in sorted(intervals):
        if result and start <= result[-1][1]:
            result[-1] = (result[-1][0], max(result[-1][1], end))
        else:
            result.append((start, end))
    return result

def displaced(start, duration, parts):
    return original(start, duration) != trailing_member(start, duration, parts)

class SupportConservation(unittest.TestCase):
    def test_actual_synthetic_shape(self):
        self.assertEqual(original(0, 384), (0, 384))
        self.assertEqual(trailing_member(0, 384, [96] * 4), (288, 672))
        self.assertTrue(displaced(0, 384, [96] * 4))

    def test_unsplit_host_is_not_this_error(self):
        self.assertFalse(displaced(19, 45, [45]))

    def test_complete_matching_transport_conserves_union(self):
        self.assertEqual(union(complete_fragments(7, 45, [9, 18, 18])), [original(7, 45)])

    def test_translation_preserves_class(self):
        for start in [0, 7, 91, 10003]:
            self.assertTrue(displaced(start, 45, [9, 18, 18]))

    def test_metric_scaling_preserves_class_and_union(self):
        for scale in [1, 2, 3, 5]:
            parts = [9 * scale, 18 * scale, 18 * scale]
            self.assertTrue(displaced(7 * scale, 45 * scale, parts))
            self.assertEqual(union(complete_fragments(7 * scale, 45 * scale, parts)),
                             [original(7 * scale, 45 * scale)])

    def test_names_labels_and_visibility_are_not_inputs(self):
        for name, voice, visible in [('A', '1', True), ('Renamed', '23', False)]:
            self.assertTrue(displaced(7, 45, [9, 18, 18]))

    def test_independent_cohorts_are_additive(self):
        a = displaced(7, 45, [9, 18, 18])
        b = displaced(107, 60, [12, 24, 24])
        self.assertEqual([a], [a, b][:1])
        self.assertEqual([b], [a, b][1:])
        self.assertTrue(a and b)

    def test_invalid_partition_supplies_no_realization(self):
        for parts in [[], [45, 0], [9, 18], [46]]:
            with self.assertRaises(AssertionError):
                complete_fragments(7, 45, parts)

if __name__ == '__main__':
    unittest.main()
```

Runtime proof remains required at the existing generator/arrangement/ledger
boundaries. In particular, a physically conserved union alone does not prove
tie preservation, unchanged attacks, valid hidden-member hosting or musical
role authority. Neither a converter exit code nor this model can supply those
missing premises.
