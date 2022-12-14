import unittest

from aoc import sand

class TestSand(unittest.TestCase):

    def test_chains(self):
        self.assertEqual(sand.coords("1,1"), (1, 1))
        self.assertEqual(sand.chain("498,4 -> 498,6"), [(498, 4), (498, 6)])

    def test_sand(self):
        data = ["498,4 -> 498,6 -> 496,6", "503,4 -> 502,4 -> 502,9 -> 494,9"]
        sp = sand.Problem(data)
        self.assertTrue(sp.orig, (494, 0))

        self.assertTrue(sp.floor, 11)

        self.assertEqual(sp.all_sand(), 24)


        sp2 = sand.Problem(data)
        sp2.bottom()
        self.assertEqual(sp2.all_sand(), 93)
