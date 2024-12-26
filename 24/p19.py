import sys

class Ptnr:

    def __init__(self, twls):
        self.towels = set(twls)
        self.cable = {}

    def able(self, pattern) -> int:
        if not pattern:
            return 1
        if pattern in self.cable:
            return self.cable[pattern]
        ret = 0
        if pattern in self.towels:
            ret += 1
        for i in range(1, len(pattern)):
            if pattern[:i] not in self.towels:
                continue
            ret += self.able(pattern[i:])
        self.cable[pattern] = ret
        return ret

def go():
    twls = [l.strip() for l in sys.stdin.readline().split(", ")]
    sys.stdin.readline()
    ptrns = [l.strip() for l in sys.stdin.readlines()]
    p = Ptnr(twls)
    tot = 0
    ctot = 0
    for pat in ptrns:
        cut = p.able(pat)
        if cut:
            tot += 1
            ctot += cut
    print(tot)
    print(ctot)

go()
