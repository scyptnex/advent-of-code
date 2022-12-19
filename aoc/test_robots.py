import unittest

from aoc import robots


DATA = [
    "Blueprint 1:  Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.",
    "Blueprint 2:  Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
]


class TestRobots(unittest.TestCase):
    def test_parse(self):
        self.assertEqual(
            robots.bot("Each geode robot costs 3 ore and 12 obsidian"), (3, 0, 12, 0)
        )

        bp = robots.Blueprint(DATA[0])
        self.assertEqual(bp.idx, 1)
        self.assertEqual(bp.bots[0], (4, 0, 0, 0))
        self.assertEqual(bp.bots[1], (2, 0, 0, 0))
        self.assertEqual(bp.bots[2], (3, 14, 0, 0))
        self.assertEqual(bp.bots[3], (2, 0, 7, 0))

    def test_geodes(self):

        self.assertEqual(robots.Blueprint(DATA[0]).geodes(), 9)
        self.assertEqual(robots.Blueprint(DATA[1]).geodes(), 12)
