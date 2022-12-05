import unittest

from aoc import crate_stack


class TestCrateStack(unittest.TestCase):
    def test_problem(self):
        data = [
            "    [D]",
            "[N] [C]",
            "[Z] [M] [P]",
            " 1   2   3",
        ]
        self.assertEqual(
            crate_stack.problem(data).stacks, [["Z", "N"], ["M", "C", "D"], ["P"]]
        )

    def test_move(self):
        cs = crate_stack.CrateStack()
        cs.stacks = [["Z", "N"], ["M", "C", "D"], ["P"]]
        cs.move("move 1 from 2 to 1")
        self.assertEqual(cs.stacks, [["Z", "N", "D"], ["M", "C"], ["P"]])

    def test_crate_stack(self):
        data = [
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
        self.assertEqual(crate_stack.read_in(data), "MCD")
