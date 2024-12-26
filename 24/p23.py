import sys
from collections import defaultdict


def readn():
    res = defaultdict(set)
    for l in sys.stdin.readlines():
        l = l.strip().split("-")
        res[l[0]].add(l[1])
        res[l[1]].add(l[0])
    return res


def dotty(ntw):
    print("graph foo{")
    for a in ntw:
        for b in ntw[a]:
            if b <= a:
                continue
            print(" {} -- {};".format(a, b))
    print("}")


def bk1(ntw, r, p, x):
    if not p and not x:
        return r
    lgst = set()
    for v in p:
        vs = set([v])
        vn = ntw[v]
        chk = bk1(ntw, r | vs, p & vn, x & vn)
        if len(chk) > len(lgst):
            lgst = chk
        p = p - vs
        x = x | vs
    return lgst


def go():
    ntw = readn()
    triples = []
    for a in ntw:
        cons = ntw[a]
        for b in cons:
            if b < a:
                continue
            for c in cons:
                if c < b:
                    continue
                if c not in ntw[b]:
                    continue
                triples.append((a, b, c))
    print(
        len([t for t in triples if t[0][0] == "t" or t[1][0] == "t" or t[2][0] == "t"])
    )
    print(",".join(k for k in sorted(bk1(ntw, set(), set(ntw.keys()), set()))))


go()
