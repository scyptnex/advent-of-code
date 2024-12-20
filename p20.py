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

def dijk(track, start, end, lim):
    q = PriorityQueue()
    v = set()
    q.put((0, start))
    while not q.empty():
        cost, cur = q.get()
        if cur in v or cost > lim:
            continue
        v.add(cur)
        if cur == end:
            return cost
        for a in adj(track, cur):
            if track[a[0]][a[1]] == '#':
                continue
            if track[cur[0]][cur[1]] == '1' and track[a[0]][a[1]] != '2':
                continue
            q.put((cost+1, a))
    return -1

def go():
    track=[[c for c in l.strip()] for l in sys.stdin.readlines()]
    
    for ri, row in enumerate(track):
        for ci, c in enumerate(row):
            if c == 'S':
                start=(ri, ci)
            if c == 'E':
                end=(ri, ci) 
    normal = dijk(track, start, end, len(track)*len(track[0]))
    savings = {}
    for ri, row in enumerate(track):
        print('.')
        for ci, c in enumerate(row):
            if track[ri][ci] != '#':
                continue
            for pa in adj(track, (ri, ci)):
                if track[pa[0]][pa[1]] == '#':
                    continue
                track[ri][ci] = '1'
                track[pa[0]][pa[1]] = '2'
                new = dijk(track, start, end, normal)
                track[ri][ci] = '#'
                track[pa[0]][pa[1]] = '.'
                if new != -1 and new < normal:
                    sav = normal - new
                    if sav not in savings:
                        savings[sav] = 0
                    savings[sav] += 1
    totsav = 0
    for s in savings:
        if s < 100:
            continue
        totsav += savings[s]
    print(totsav)

go()
