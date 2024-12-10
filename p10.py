import sys

lns = [[int(c) for c in l.strip()] for l in sys.stdin.readlines() if l.strip()]

def trails(lns, rx, cx):
    frontier = set([(rx, cx)])
    for h in range(1,10):
        new_frontier = set()
        for ri, ci in frontier:
            for ra, ca in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                (rn, cn) = (ri+ra, ci+ca)
                if rn < 0 or cn < 0 or rn >= len(lns) or cn >= len(lns[rn]):
                    continue
                if lns[rn][cn] == h:
                    new_frontier.add((rn, cn))
        frontier = new_frontier
    return len(frontier)

def trails2(lns, ri, ci):
    cur = lns[ri][ci]
    if cur == 9:
        return 1
    trls = 0
    for ra, ca in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        (rn, cn) = (ri+ra, ci+ca)
        if rn < 0 or cn < 0 or rn >= len(lns) or cn >= len(lns[rn]):
            continue
        if lns[rn][cn] != cur+1:
            continue
        trls += trails2(lns, rn, cn)
    return trls

tot=0
tot2=0
for ri, r in enumerate(lns):
    for ci, c in enumerate(r):
        if c == 0:
            tot += trails(lns, ri, ci)
            tot2 += trails2(lns, ri, ci)
print(tot)
print(tot2)
