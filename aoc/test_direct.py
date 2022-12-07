import unittest

from aoc import direct


class TestDirect(unittest.TestCase):
    def test_ls(self):
        d = direct.Direct(None)
        d.ls("dir x")
        d.ls("42 a")
        d.ls("99 b")
        self.assertEqual(d.directs.keys(), set("x"))
        self.assertEqual(d.files, {"a": 42, "b":99})

    def test_read(self):
        data = [
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ]
        self.assertEqual(direct.get_threshold(direct.read(data)), 95437)
        self.assertEqual(direct.get_deletion(direct.read(data)), 24933642)
