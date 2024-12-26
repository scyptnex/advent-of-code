import sys
from queue import PriorityQueue

crds = [[int(c) for c in l.strip().split(',')] for l in sys.stdin.readlines() if l.strip()]

# FIRST=12
# WID=6
FIRST=1024
WID=70
HEI=WID

def adj(cur):
    x, y = cur
    if x > 0:
        yield (x-1, y)
    if y > 0:
        yield (x, y-1)
    if x < WID:
        yield (x+1, y)
    if y < HEI:
        yield (x, y+1)

def dijk(lim):
    fallen = set((c[0], c[1]) for c in crds[:lim])
    q = PriorityQueue()
    v = set()
    q.put((0, (0, 0)))
    while not q.empty():
        cost, cur = q.get()
        if cur in v:
            continue
        v.add(cur)
        #print(cur)
        if cur == (WID, HEI):
            return cost
        for a in adj(cur):
            if a in fallen:
                continue
            q.put((cost+1, a))
    return -1

def dd(good, bad):
    chk = (good+bad)//2
    if good +1 >= bad:
        return crds[good]
    if dijk(chk) == -1:
        return dd(good, chk)
    else:
        return dd(chk, bad)

print(dijk(FIRST))
ans = dd(0, len(crds))
print("{},{}".format(ans[0], ans[1]))
