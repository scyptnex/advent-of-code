import sys
from queue import PriorityQueue

# N E S W
drcs = [(-1, 0), (0, 1), (1, 0), (0, -1)]
EAST = 1

field = [l.strip() for l in sys.stdin.readlines() if l.strip()]

for ri, row in enumerate(field):
    for ci, c in enumerate(row):
        if c == "S":
            start = (ri, ci, EAST, ri, ci, EAST)

q = PriorityQueue()
v = {}
q.put((0, start))
while not q.empty():
    cost, cur = q.get()
    (cur_r, cur_c, cur_h, last_r, last_c, last_h) = cur
    vcur = (cur_r, cur_c, cur_h)
    if vcur in v:
      if v[vcur][0] == cost:
        v[vcur][1].append((last_r, last_c, last_h))
      continue
    v[vcur] = (cost, [(last_r, last_c, last_h)])
    if field[cur_r][cur_c] == "E":
        sol = cost
        solv = vcur
        break
    fr, fc = drcs[cur_h]
    nxts = [
        (cost + 1, (cur_r + fr, cur_c + fc, cur_h, cur_r, cur_c, cur_h)),
        (cost + 1000, (cur_r, cur_c, (cur_h + 1) % 4, cur_r, cur_c, cur_h)),
        (cost + 1000, (cur_r, cur_c, (cur_h + 3) % 4, cur_r, cur_c, cur_h)),
    ]
    for n in nxts:
        nr = n[1][0]
        nc = n[1][1]
        if field[nr][nc] == '#':
            continue
        q.put(n)
print(sol)

front = [solv]
seen = set(front)
while front:
    cur = front.pop()
    for prev in v[cur][1]:
        if prev in seen:
            continue
        seen.add(prev)
        front.append(prev)
print(len(set((r, c) for r, c, _ in seen)))
