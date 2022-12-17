import unittest

from aoc import tetris

WIND = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"

class TestTetris(unittest.TestCase):

    def test_tower(self):
        self.assertEqual(tetris.Tetris(WIND).go(rocks=1), 1)
        self.assertEqual(tetris.Tetris(WIND).go(rocks=5), 9)
        self.assertEqual(tetris.Tetris(WIND).go(), 3068)
