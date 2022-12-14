import sys

from aoc import rope

def coords(s):
    sp = s.split(",")
    return (int(sp[0]), int(sp[1]))


def chain(l):
    return [coords(s) for s in l.split(" -> ")]


ROCK = 1
SAND = 2
FLOOR = 3

class Problem:
    def __init__(self, data):
        self.coords = [chain(l.strip()) for l in data]
        min_x = max_x = 500
        min_y = max_y = 0
        for c in self.coords:
            for l in c:
                x, y = l
                min_x = min(min_x, x)
                max_x = max(max_x, x)
                min_y = min(min_y, y)
                max_y = max(max_y, y)
        self.orig = (min_x, min_y)

        self.field={}

        self.floor = 2
        for c in self.coords:
            for i in range(1, len(c)):
                x1, y1 = c[i-1]
                x2, y2 = c[i]
                xl = min(x1, x2)
                xh = max(x1, x2)
                yl = min(y1, y2)
                yh = max(y1, y2)
                for x in range(xl, xh+1):
                    for y in range(yl, yh+1):
                        self.field[(x, y)] = ROCK
                self.floor = max(self.floor, yh+2)
        self.sand = (500, 0)

    def inside(self, s):
        _, sy = s
        return sy <= self.floor

    def bottom(self):
        for f in range(500-(2*self.floor), 500+(2*self.floor)):
            self.field[(f, self.floor)] = FLOOR

    def add_sand(self):
        s = self.sand
        while self.inside(s):
            new_s = None
            for m in [(0, 1), (-1, 1), (1, 1)]:
                ps = rope.pplus(m, s)
                if ps not in self.field:
                    new_s = ps
                    break
            if new_s is not None:
                s = new_s
                continue
            if s in self.field and self.field[s] == SAND:
                return None
            self.field[s] = SAND
            return s
        return None
    
    def all_sand(self):
        total = 0
        while True:
            s = self.add_sand()
            if s is None:
                return total
            total += 1

if __name__=="__main__":
    p = Problem(sys.stdin)
    p.bottom()
    print(p.all_sand())

