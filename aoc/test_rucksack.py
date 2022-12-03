import unittest

import aoc.rucksack as ruck


class TestRucksack(unittest.TestCase):
    data = [
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ]

    def test_common_item(self):
        self.assertEqual(ruck.common_item("ac", "ab"), "a")

    def test_common_sack(self):
        self.assertEqual(ruck.common_sack("vJrwpWtwJgWrhcsFMMfFFhFp"), "p")

    def test_score(self):
        self.assertEqual(ruck.priority("a"), 1)
        self.assertEqual(ruck.priority("z"), 26)
        self.assertEqual(ruck.priority("A"), 27)
        self.assertEqual(ruck.priority("Z"), 52)
        self.assertEqual(ruck.score(self.data), 157)

    def test_badge(self):
        self.assertEqual(ruck.badge(self.data[:3]), "r")
        self.assertEqual(ruck.badge(self.data[3:]), "Z")

        converted = [r for r in ruck.triples_to_sack(iter(self.data))]
        self.assertEqual(converted, ["rr", "ZZ"])

        self.assertEqual(ruck.score(converted), 70)
