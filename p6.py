import sys

grid=[[c for c in l.strip()] for l in sys.stdin.readlines() if l.strip()]

nxt_dir={
        (-1,0): (0,1),
        (0,1): (1,0),
        (1,0): (0,-1),
        (0,-1): (-1,0),
        }

for ri, r in enumerate(grid):
    for ci, c in enumerate(r):
        if c == '^':
            gs = (ri, ci)

def simulate(guard_start, start_dir):
    visited=set()
    vis_dir=set()
    guard_loc = guard_start
    guard_dir = start_dir
    while True:
        visited.add(guard_loc)
        if (guard_loc, guard_dir) in vis_dir:
            return 0 # trapped
        vis_dir.add((guard_loc, guard_dir))
        gx, gy = guard_loc
        dx, dy = guard_dir
        nx, ny = gx+dx, gy+dy
        if nx < 0 or ny < 0 or nx >= len(grid) or ny >=len(grid[nx]):
            return len(visited)
        elif grid[nx][ny] == '#':
            guard_dir = nxt_dir[guard_dir]
        else:
            guard_loc = (nx, ny)

def simulate2(guard_start, start_dir):
    visited=set()
    guard_loc = guard_start
    guard_dir = start_dir
    utraps = set()
    while True:
        visited.add(guard_loc)
        gx, gy = guard_loc
        dx, dy = guard_dir
        nx, ny = gx+dx, gy+dy
        if nx < 0 or ny < 0 or nx >= len(grid) or ny >=len(grid[nx]):
            return len(utraps)
        elif grid[nx][ny] == '#':
            guard_dir = nxt_dir[guard_dir]
        else:
            grid[nx][ny] = '#'
            if (nx, ny) not in visited and simulate(guard_loc, guard_dir) == 0:
                utraps.add((nx, ny))
            grid[nx][ny] = '.'
            guard_loc = (nx, ny)

print(simulate(gs, (-1,0)))
print(simulate2(gs, (-1,0)))

