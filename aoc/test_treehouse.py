import unittest

from aoc import treehouse


class TestTreehouse(unittest.TestCase):
    def test_yield_visible(self):
        self.assertEqual([i for i in treehouse.yield_visible_forwards("30373")], [0,3])
        self.assertEqual([i for i in treehouse.yield_visible("30373")], [0,3,4,3])

    def test_visible(self):
        data = [
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ]
        self.assertEqual(treehouse.num_visible(data), 21)
        self.assertEqual(treehouse.sceinic_score(data), 8)
