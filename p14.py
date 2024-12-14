import sys
import time

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

    def move(self, w, h):
        self.x = (self.x + w + self.vx)%w
        self.y = (self.y + h + self.vy)%h


rbts = [rbt(l.strip()) for l in sys.stdin.readlines()]
wi = 101
he = 103

quads = [0]*5
for r in rbts:
    quads[r.adv(100, wi, he)] += 1

print(quads[0]*quads[1]*quads[2]*quads[3])

def show(i, m, wi, he):
    for y in range(he):
        for x in range(wi):
            print('X ' if m[x][y] else '  ', end='')
        print()
    print(i, '----------------------')

def oddsof(rbts, wi, he):
    sw = wi//5
    sh = he//5
    counts = [[0 for _ in range(6)]for _ in range(6)]
    for r in rbts:
        counts[r.x//sw][r.y//sh] += 1
    expected = len(rbts)/25
    sm = 0
    for a in range(5):
        for b in range(5):
            nom = counts[a][b] - expected
            nom = nom*nom
            sm += nom / expected
    return sm


biggest = 0
for i in range(0, 99999999):
    e = oddsof(rbts, wi, he)
    if e > biggest:
        biggest = e
        m = [[0 for _ in range(he)] for _ in range(wi)]
        for r in rbts:
            m[r.x][r.y] += 1
        show(i, m, wi, he)
    for r in rbts:
        r.move(wi, he)
