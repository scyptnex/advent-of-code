import sys

class rbt:

    def __init__(self, l):
        [[self.x, self.y], [self.vx, self.vy]] = [[int(j) for j in x[2:].split(',')] for x in l.split(' ')]
        
    def adv(self, t, w, h):
        nx = self.x + self.vx*t
        ny = self.y + self.vy*t
        if nx < 0:
            nx = nx + (nx//-w + 2)*w
        if ny < 0:
            ny = ny + (ny//-h + 2)*h
        nx = nx%w
        ny = ny%h

        ww = w//2
        hh = h//2
        ret = 0
        if nx == ww:
            return -1
        if ny == hh:
            return -1
        if nx < ww:
            ret += 1
        if ny < hh:
            ret += 2
        return ret


rbts = [rbt(l.strip()) for l in sys.stdin.readlines()]

quads = [0]*5
for r in rbts:
    quads[r.adv(100, 101, 103)] += 1

print(quads)
print(quads[0]*quads[1]*quads[2]*quads[3])
