import sys
from queue import PriorityQueue


def cardinal_directions():
    yield (-1, 0, "^")
    yield (0, -1, "<")
    yield (1, 0, "v")
    yield (0, 1, ">")


def adj(pos):
    for dr, dc, x in cardinal_directions():
        yield (pos[0] + dr, pos[1] + dc, x)


def keyfor(kys, pos):
    for ar, ac, x in adj(pos):
        if ar < 0 or ac < 0:
            continue
        if ar >= len(kys) or ac >= len(kys[ar]):
            continue
        if kys[ar][ac] == " ":
            continue
        yield (kys[ar][ac], x)


def graphy(ptn):
    crs = [[c for c in l] for l in ptn.split("\n")]
    ret = {}
    for ri, row in enumerate(crs):
        for ci, c in enumerate(row):
            if c == " ":
                continue
            ret[c] = [k for k in keyfor(crs, (ri, ci))]
    return ret

KEYP = graphy(
    """789
456
123
 0A"""
)

ARRW = graphy(
    """ ^A
<v>"""
)


class Me:
    def best(self, s, t):
        return 1


class Keyp:
    def __init__(self, subp, adjs):
        self.subp = subp
        self.mem = {}
        self.adjs = adjs

    def cbest(self, s, t):
        q = PriorityQueue()
        v = set()
        q.put((0, s, 'A'))
        while not q.empty():
            cost, cur, subloc = q.get()
            # if s == '2' and t == '9':
            #     print(cost, cur, subloc, seq)
            if (cur, subloc) in v:
                continue
            v.add((cur, subloc))
            if cur == t:
                if subloc == 'A':
                    return cost
                pac = self.subp.best(subloc, 'A')
                q.put((cost + pac, cur, 'A'))
                continue
            for adj, ax in self.adjs[cur]:
                pxc = self.subp.best(subloc, ax)
                q.put((cost + pxc, adj, ax))

    def best(self, s, t):
        if s == t:
            return 1
        if (s, t) not in self.mem:
            self.mem[(s, t)] = self.cbest(s, t)
        return self.mem[(s, t)]


def slv(pzl, n):
    cur = Me()
    for _ in range(0, n):
        cur = Keyp(cur, ARRW)
    cur = Keyp(cur, KEYP)
    tot = 0
    for p in pzl:
        last = "A"
        cplx = 0
        for c in p:
            cplx += cur.best(last, c)
            last = c
        tot += cplx * int("".join(c for c in p if c.isdigit()))
    print(tot)

pzl = [l.strip() for l in sys.stdin.readlines()]
slv(pzl, 2)
slv(pzl, 25)
