import unittest

from geometry import (
    Beam, Contact, Head, Stem, attach_beams, attach_heads, classify_contacts,
    group_stems,
)


def classify(heads, beams, stems, space):
    head_stems = attach_heads(heads, stems, space)
    beam_stems = attach_beams(beams, stems, space, heads, head_stems)
    return classify_contacts(heads, beams, head_stems, beam_stems, space)


class GeometryTest(unittest.TestCase):
    def test_beam_crosses_foreign_head_with_own_flag_stem(self):
        space = 5.0
        beam = Beam(10, 40, 19, 21, 19, 21)
        own_ends = [Stem(10, 19, 37, .45), Stem(40, 19, 38, .45)]
        foreign = Head(22, 17.5, 28, 22.5)
        flag_stem = Stem(22, 20, 37, .45)
        self.assertEqual(classify([foreign], [beam], own_ends + [flag_stem], space), [(0, 0, Contact.FOREIGN_HEAD)])

    def test_explicit_own_head_is_visible_but_not_foreign(self):
        space = 5.0
        beam = Beam(10, 40, 19, 21, 19, 21)
        chord = Head(22, 17.5, 28, 22.5)
        self.assertEqual(classify_contacts([chord], [beam], [{3}], [{2, 3}], space), [(0, 0, Contact.OWN_HEAD)])

    def test_unknown_head_is_not_claimed_foreign(self):
        beam = Beam(10, 40, 19, 21, 19, 21)
        floating = Head(22, 17.5, 28, 22.5)
        stems = [Stem(10, 19, 37, .45)]
        self.assertEqual(classify([floating], [beam], stems, 5), [(0, 0, Contact.UNKNOWN_ATTACHMENT)])

    def test_long_hals_without_collision(self):
        space = 5.0
        beam = Beam(10, 40, 19, 21, 16, 18)
        stems = [Stem(10, 19, 74, .45), Stem(40, 16, 66, .45)]
        self.assertGreater(stems[0].length(space), 10)
        self.assertEqual(classify([Head(4, 72, 10, 77), Head(34, 63, 40, 68)], [beam], stems, space), [])
        self.assertEqual(attach_beams([beam], stems, space, [], []), [{0, 1}])

    def test_connected_primary_and_secondary_bands_count_once(self):
        self.assertEqual(group_stems([{0, 1}, {1, 2}, {2}, {3, 4}]), [{0, 1, 2}, {3, 4}])

    def test_sloping_beam_clears_center_despite_rectangle_overlap(self):
        head = Head(22, 22, 28, 26)
        beam = Beam(10, 40, 25, 27, 13, 15)
        stems = [Stem(10, 25, 52, .45), Stem(40, 13, 50, .45), Stem(22, 24, 47, .45)]
        self.assertEqual(classify([head], [beam], stems, 5), [])


if __name__ == "__main__":
    unittest.main()
