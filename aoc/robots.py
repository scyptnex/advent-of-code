import sys

from aoc import rope


def bot(desc: str) -> (int, int, int, int):
    l = desc.split()
    ore = int(l[4])
    if len(l) < 7:
        return (ore, 0, 0, 0)
    other = int(l[7])
    return (ore, other, 0, 0) if l[-1] == "clay" else (ore, 0, other, 0)

def can_afford(wallet, bot) -> bool:
    return all(w >= b for w, b in zip(wallet, bot))

class Problem:
    def __init__(self, bots):
        self.tbl = {}
        self.bots = bots

    def solve(self, t, wallet, fleet):
        print(t, wallet, fleet, len(self.tbl))
        if t == 0:
            return wallet[3]

        key = (t, wallet, fleet)
        if key in self.tbl:
            return self.tbl[key]

        t -= 1
        n_wallet = rope.pplus(wallet, fleet)

        # do nothing
        moves = [self.solve(t, n_wallet, fleet)]
        for i, bot in enumerate(self.bots):
            if not can_afford(wallet, bot):
                continue
            new_bot = [0, 0, 0, 0]
            new_bot[i] = 1
            new_bot = (new_bot[0],new_bot[1],new_bot[2],new_bot[3])
            moves.append(self.solve(t, rope.pdiff(n_wallet, bot), rope.pplus(fleet, new_bot)))
        mx = max(moves)
        self.tbl[key] = mx
        return mx

    def start(self):
        return self.solve(24, (0, 0, 0, 0), (1, 0, 0, 0))


class Blueprint:
    def __init__(self, line):
        bp = line.strip().split(":", 2)
        self.idx = int(bp[0].split()[1])
        bp = bp[1].split(".")
        self.bots = [bot(l.strip()) for l in bp if l]

    def geodes(self):
        return Problem(self.bots).start()
