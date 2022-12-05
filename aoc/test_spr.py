import unittest

import aoc.spr as spr


class TestSpr(unittest.TestCase):
    def test_outcome(self):
        self.assertEqual(spr.outcome_score("R", "R"), 3)
        self.assertEqual(spr.outcome_score("R", "P"), 6)
        self.assertEqual(spr.outcome_score("R", "S"), 0)
        self.assertEqual(spr.outcome_score("P", "R"), 0)
        self.assertEqual(spr.outcome_score("P", "P"), 3)
        self.assertEqual(spr.outcome_score("P", "S"), 6)
        self.assertEqual(spr.outcome_score("S", "R"), 6)
        self.assertEqual(spr.outcome_score("S", "P"), 0)
        self.assertEqual(spr.outcome_score("S", "S"), 3)

    def test_score(self):
        self.assertEqual(spr.score("R", "P"), 8)
        self.assertEqual(spr.score("P", "R"), 1)

    def test_total(self):
        seq = [["A", "Y"], ["B", "X"], ["C", "Z"]]
        self.assertEqual(spr.total(seq), 15)

    def test_determine_play(self):
        self.assertEqual(spr.determine_play("A", "Y"), "X")
        self.assertEqual(spr.determine_play("B", "X"), "X")
        self.assertEqual(spr.determine_play("C", "Z"), "X")

        self.assertEqual(spr.determine_play("A", "X"), "Z")
        self.assertEqual(spr.determine_play("A", "Z"), "Y")
