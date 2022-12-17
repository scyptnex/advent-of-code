import unittest

from aoc import pressure


class TestPressure(unittest.TestCase):

    def test_tri(self):
        self.assertEqual(pressure.t_len(1), 1)
        self.assertEqual(pressure.t_len(2), 3)
        self.assertEqual(pressure.t_len(5), 15)

        self.assertEqual(pressure.t_idx(0, 0), 0)
        self.assertEqual(pressure.t_idx(1, 0), 1)
        self.assertEqual(pressure.t_idx(1, 1), 2)
        self.assertEqual(pressure.t_idx(4, 3), 13)

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
        ss = pressure.SSpace(pressure.valves(data))
        # self.assertEqual(ss.solve(), 1651)
        self.assertEqual(ss.solve_e(), 1707)
