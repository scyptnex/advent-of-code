import unittest

from aoc import decrypt

class TestDecrypt(unittest.TestCase):

    def test_chain(self):
        data = [str(i) for i in range(20)]
        s = decrypt.Seq(data)
        s.print()
