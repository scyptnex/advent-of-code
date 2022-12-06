import unittest

from aoc import signal

class TestSignal(unittest.TestCase):

    def test_signal(self):
        self.assertEqual(signal.signal("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7)
        self.assertEqual(signal.signal("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5)
        self.assertEqual(signal.signal("nppdvjthqldpwncqszvftbrmjlhg"), 6)
        self.assertEqual(signal.signal("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10)
        self.assertEqual(signal.signal("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11)
