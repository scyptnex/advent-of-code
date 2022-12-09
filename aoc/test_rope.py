import unittest

from aoc import rope


class TestRope(unittest.TestCase):
    def test_pdiff(self):
        self.assertEqual(rope.pplus((1, 1), (1, 1)), (2, 2))
        self.assertEqual(rope.pplus((1, 1), (-1, -1)), (0, 0))
        self.assertEqual(rope.pdiff((1, 1), (-1, -1)), (2, 2))

    def test_move(self):
        self.assertEqual(rope.move((0, 0)), (0, 0))
        self.assertEqual(rope.move((1, 0)), (0, 0))
        self.assertEqual(rope.move((0, -1)), (0, 0))

        self.assertEqual(rope.move((0, 2)), (0, 1))
        self.assertEqual(rope.move((-2, 0)), (-1, 0))

        self.assertEqual(rope.move((2, 1)), (1, 1))
        self.assertEqual(rope.move((2, -1)), (1, -1))
        self.assertEqual(rope.move((-2, 1)), (-1, 1))

        self.assertEqual(rope.move((2,2)), (1,1))

    def test_new_pos(self):
        h2, t2 = rope.get_new_pos((0, 0), (0, 0), (1, 0))
        self.assertEqual(h2, (1, 0))
        self.assertEqual(t2, (0, 0))

        h2, t2 = rope.get_new_pos((0, 0), (-1, 0), (1, 0))
        self.assertEqual(h2, (1, 0))
        self.assertEqual(t2, (0, 0))

        h2, t2 = rope.get_new_pos((0, 0), (-1, -1), (1, 0))
        self.assertEqual(h2, (1, 0))
        self.assertEqual(t2, (0, 0))

    def test_snake(self):
        data = [
            "R 4",
            "U 4",
            "L 3",
            "D 1",
            "R 4",
            "D 1",
            "L 5",
            "R 2",
        ]
        self.assertEqual(rope.count_tail_pos(data), 13)
        self.assertEqual(rope.count_tail_pos(data, 10), 1)
        data = [
            "R 5",
            "U 8",
            "L 8",
            "D 3",
            "R 17",
            "D 10",
            "L 25",
            "U 20",
        ]
        self.assertEqual(rope.count_tail_pos(data, 10), 36)
