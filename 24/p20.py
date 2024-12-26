import sys
from queue import PriorityQueue

def adj(track, cur):
    r, c = cur
    if r > 0:
        yield (r-1, c)
    if c > 0:
        yield (r, c-1)
    if r < len(track)-1:
        yield (r+1, c)
    if c < len(track[r])-1:
        yield (r, c+1)

def cheatable_from(track, dist, cstart):
    q = PriorityQueue()
    q.put((0, cstart, True))
    v = {}
    while not q.empty():
        cost, cur, last_on_open = q.get()
        if cur in v:
            oldc, oldl = v[cur]
            if not(cost == oldc and oldl and not last_on_open):
                continue
        v[cur] = (cost, last_on_open)
        on_open = track[cur[0]][cur[1]] != '#'
        if on_open and not last_on_open:
            yield (cur, cost)
        if cost == dist:
            continue
        for a in adj(track, cur):
            if cost > 0 or track[a[0]][a[1]] == '#':
                #if cstart == (1, 2) and a == (9, 3):
                #    print("__", cost, cur, last_on_open, on_open, a)
                q.put((cost+1, a, on_open))

def cheatable_front(track, dist, cstart):
    frontier = [cstart]
    v = set(frontier)
    for cost in range(1, dist+1):
        newf = []
        for f in frontier:
            for a in adj(track, f):
                if a in v:
                    continue
                v.add(a)
                tp = track[a[0]][a[1]]
                if cost == 1 and tp != '#':
                    continue
                if tp != '#':
                    yield (a, cost)
                newf.append(a)
        frontier = newf

def mht_dist_cheats(cstart, dist, e2s):
    for e in e2s:
        mhd = abs(cstart[0] - e[0]) + abs(cstart[1] - e[1])
        if mhd != 0 and mhd <= dist:
            yield(e, mhd)

def cheats(track, dist, savings, cstart, s2e, e2s, limit):
    #for cend, ctime in cheatable_from(track, dist, cstart):
    for cend, ctime in mht_dist_cheats(cstart, dist, e2s):
        if cend not in e2s:
            continue
        total = s2e[cstart] + ctime + e2s[cend]
        if total >= limit:
            continue
        #if (limit - total) == 66:
        #    print(limit, total, cstart, cend, s2e[cstart] , ctime , e2s[cend])
        savings[(cstart, cend)] = limit - total



def dijk(track, start):
    q = PriorityQueue()
    dists = {}
    q.put((0, start))
    while not q.empty():
        cost, cur = q.get()
        if cur in dists:
            continue
        dists[cur] = cost
        for a in adj(track, cur):
            if track[a[0]][a[1]] == '#':
                continue
            q.put((cost+1, a))
    return dists

def go():
    track=[[c for c in l.strip()] for l in sys.stdin.readlines()]
    for ri, row in enumerate(track):
        for ci, c in enumerate(row):
            if c == 'S':
                start=(ri, ci)
            if c == 'E':
                end=(ri, ci) 
    s2e = dijk(track, start)
    e2s = dijk(track, end)
    limit = s2e[end]
    savings={}
    savingss={}
    for cstart in s2e:
        if s2e[cstart] >= limit:
            continue
        cheats(track, 2, savings, cstart, s2e, e2s, limit) 
        cheats(track, 20, savingss, cstart, s2e, e2s, limit) 
    def desc(sv):
        gs = {}
        gt100 = 0
        for s in sv:
            s = sv[s]
            if s not in gs:
                gs[s] = 0
            gs[s] += 1
            if s >= 100:
                gt100 += 1
        for s in sorted(gs.keys()):
            print(s, gs[s])
        print(gt100)
    desc(savings)
    desc(savingss)

go()
