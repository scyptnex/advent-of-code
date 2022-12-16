import unittest

from aoc import pressure


class TestPressure(unittest.TestCase):
    def test_valve(self):
        t = "Valve EE has flow rate=3; tunnels lead to valves FF, DD"
        v = pressure.Valve(t)
        self.assertEqual(v.location, "EE")
        self.assertEqual(v.flow_rate, 3)
        self.assertEqual(v.tunnels, ["FF", "DD"])

    def test_solve(self):
        data = [
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA   ",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB    ",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD    ",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG    ",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH    ",
            "Valve HH has flow rate=22; tunnel leads to valve GG        ",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ    ",
            "Valve JJ has flow rate=21; tunnel leads to valve II        ",
        ]
        x = pressure.solve(pressure.valves(data))
        self.assertEqual(x, 1651)
