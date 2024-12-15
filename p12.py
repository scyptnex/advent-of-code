import sys


def find(ds, v):
    if v not in ds:
        ds[v] = v
    if ds[v] == v:
        return v
    ds[v] = find(ds, ds[v])
    return ds[v]


def union(ds, a, b):
    ra = find(ds, a)
    rb = find(ds, b)
    ds[ra] = rb


def group_fncs(fncs: set[tuple[int, int]]) -> int:
    ds = {}
    for fr, fc in fncs:
        if int(fr * 4) % 4 == 0:
            if (fr + 1, fc) in fncs:
                union(ds, (fr, fc), (fr + 1, fc))
            if (fr - 1, fc) in fncs:
                union(ds, (fr, fc), (fr - 1, fc))
        else:
            if (fr, fc + 1) in fncs:
                union(ds, (fr, fc), (fr, fc + 1))
            if (fr, fc - 1) in fncs:
                union(ds, (fr, fc), (fr, fc - 1))
    fcs = set()
    for f in fncs:
        fcs.add(find(ds, f))
    return len(fcs)


def getplot(start, plts):
    perim = 0
    q = [start]
    v = set(q)
    fncs = set()
    while q:
        cur = q.pop()
        for dr, dc in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            nr, nc = (cur[0] + dr, cur[1] + dc)
            if (nr, nc) in v:
                continue
            if (nr, nc) in plts:
                plts.remove((nr, nc))
                v.add((nr, nc))
                q.append((nr, nc))
            else:
                perim += 1
                f = (cur[0] + (dr / 4), cur[1] + (dc / 4))
                fncs.add(f)
    return (perim * len(v), group_fncs(fncs) * len(v))


plots = {}
for ri, r in enumerate(sys.stdin.readlines()):
    for ci, c in enumerate(r.strip()):
        if c not in plots:
            plots[c] = set()
        plots[c].add((ri, ci))

total = 0
dtotal = 0
for p in plots:
    cur = plots[p].copy()
    while cur:
        t, d = getplot(cur.pop(), cur)
        total += t
        dtotal += d

print(total)
print(dtotal)
