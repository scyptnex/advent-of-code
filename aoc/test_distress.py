import unittest

from aoc import distress


class TestDistress(unittest.TestCase):
    def test_compare(self):
        self.assertTrue(distress.correct([1, 1, 3, 1, 1], [1, 1, 5, 1, 1]))
        self.assertFalse(distress.correct([[[]]], [[]]))

    def test_parse(self):
        self.assertEqual(
            distress.parse("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            [1, [2, [3, [4, [5, 6, 7]]]], 8, 9],
        )

    def test_corrects(self):
        data = [
            "[1,1,3,1,1]                ",
            "[1,1,5,1,1]                ",
            "                           ",
            "[[1],[2,3,4]]              ",
            "[[1],4]                    ",
            "                           ",
            "[9]                        ",
            "[[8,7,6]]                  ",
            "                           ",
            "[[4,4],4,4]                ",
            "[[4,4],4,4,4]              ",
            "                           ",
            "[7,7,7,7]                  ",
            "[7,7,7]                    ",
            "                           ",
            "[]                         ",
            "[3]                        ",
            "                           ",
            "[[[]]]                     ",
            "[[]]                       ",
            "                           ",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        ]
        self.assertEqual(distress.sum_of_indicies_in_order(iter(data)), 13)
        self.assertEqual(distress.order_distress(data), 140)
