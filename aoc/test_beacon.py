import unittest

from aoc import beacon


class TestBeacon(unittest.TestCase):
    def test_parse(self):
        s = beacon.Sense(*beacon.parse("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"))
        self.assertEqual(s.s_loc, (2, 18))
        self.assertEqual(s.b_loc, (-2, 15))
        self.assertEqual(s.dist, 7)

    def test_merge(self):
        self.assertEqual(beacon.merge([]), [])
        self.assertEqual(beacon.merge([(1, 3), (2,4)]), [(1, 4)])
        self.assertEqual(beacon.merge([(1, 2), (3,4)]), [(1,4)])
        self.assertEqual(beacon.merge([(1,4), (3, 5), (7,11)]), [(1, 5), (7,11)])

    def test_signal(self):
        s = beacon.Sense((8, 7), (2, 10))
        self.assertEqual(s.dist, 9)
        self.assertEqual(s.influence(7), (-1, 17))
        self.assertEqual(s.influence(5), (1, 15))
        self.assertEqual(s.influence(22), None)

    def test_beacons(self):
        data = [
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15 ",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16 ",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3  ",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10   ",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10   ",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10  ",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3  ",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3  ",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3  ",
        ]
        b = beacon.Beacons(data)
        self.assertEqual(len(b.sensors), 14)
        self.assertEqual(len(b.beacons), 6)

        self.assertEqual(b.count_excluded(10), 26)

        self.assertEqual(b.find_beacon(20), (14,11))
