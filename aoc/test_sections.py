import unittest

import aoc.sections as sections


class TestSections(unittest.TestCase):
    def test_contains(self):
        self.assertTrue(sections.contains((1, 4), (2, 3)))
        self.assertTrue(sections.contains((2, 3), (1, 4)))
        self.assertTrue(sections.contains((1, 1), (1, 1)))

    def test_range_to_pairs(self):
        self.assertEqual(sections.range_to_pairs("2-4,6-8"), ((2, 4), (6, 8)))

    def test_count_contains(self):
        data = [
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ]
        self.assertEqual(sections.count_contained(data), 2)

    def test_overlaps(self):
        self.assertTrue(sections.overlaps((1, 3), (2, 4)))
        self.assertTrue(sections.overlaps((3, 5), (2, 4)))
        self.assertTrue(sections.overlaps((3, 3), (2, 4)))
        self.assertFalse(sections.overlaps((1, 2), (3, 4)))

        self.assertTrue(sections.overlaps((2, 4), (1, 3)))
        self.assertTrue(sections.overlaps((2, 4), (3, 5)))
        self.assertTrue(sections.overlaps((2, 4), (3, 3)))
        self.assertFalse(sections.overlaps((3, 4), (1, 2)))
