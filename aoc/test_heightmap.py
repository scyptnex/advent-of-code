import unittest

from aoc import heightmap


class TestHeightmap(unittest.TestCase):
    def test_walk(self):
        data = [
            "Sabqponm",
            "abcryxxl",
            "accszExk",
            "acctuvwj",
            "abdefghi",
        ]
        m = heightmap.Terrain(data)
        self.assertEqual(m.w, 8)
        self.assertEqual(m.h, 5)
        self.assertEqual(m.start, (0, 0))
        self.assertSetEqual(m.end, set([(2, 5)]))

        self.assertEqual(m.height(1, 1), 1)
        self.assertEqual(m.height(4, 7), 8)

        self.assertEqual(m.walk(), 31)

        m.invert()
        self.assertEqual(m.start, (2, 5))
        self.assertEqual(len(m.end), 6)
        self.assertEqual(m.walk(), 29)
