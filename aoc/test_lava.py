import unittest

from aoc import lava


class TestLava(unittest.TestCase):
    def test_cube(self):
        self.assertEqual(lava.read_cube("1,2,3\n"), (1, 2, 3))

    def test_blob(self):
        data = [
            "2,2,2",
            "1,2,2",
            "3,2,2",
            "2,1,2",
            "2,3,2",
            "2,2,1",
            "2,2,3",
            "2,2,4",
            "2,2,6",
            "1,2,5",
            "3,2,5",
            "2,1,5",
            "2,3,5",
        ]
        self.assertEqual(lava.Blob(data).surface_area(), 64)
        self.assertEqual(lava.Blob(data).add_internals().surface_area(), 58)
