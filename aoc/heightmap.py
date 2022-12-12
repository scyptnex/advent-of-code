import sys

from aoc import rope


class Terrain:
    def __init__(self, data):
        self.t = []
        self.w = 0
        self.h = 0
        for l in data:
            row = l.strip()
            if "S" in row:
                self.start = (len(self.t), row.index("S"))
            if "E" in row:
                self.end = set([(len(self.t), row.index("E"))])
            self.t.append(row)
        self.w = len(self.t[0])
        self.h = len(self.t)

    def height(self, r, c):
        h = self.t[r][c]
        if h == "S":
            h = "a"
        elif h == "E":
            h = "z"
        return ord(h) - ord("a")

    def inside(self, r, c):
        return r >= 0 and c >= 0 and r < self.h and c < self.w

    def can_step(self, old_loc, new_loc):
        old_h = self.height(*old_loc)
        new_h = self.height(*new_loc)
        return new_h <= (old_h + 1)

    def invert_h(self, h):
        if h == "S":
            h = "a"
        if h == "E":
            return "S"
        h_idx = ord(h) - ord("a")
        return chr(ord("z") - h_idx)

    def invert(self):
        self.start = self.end.pop()
        self.end = set()
        for r, row in enumerate(self.t):
            for c, h in enumerate(row):
                if h == "a" or h == "S":
                    coord = (r, c)
                    self.end.add(coord)
        new_t = [[self.invert_h(h) for h in row] for row in self.t]
        self.t = new_t

    def walk(self):
        distance = 0
        fronteir = [self.start]
        visited = set(fronteir)
        while fronteir:
            old_front = fronteir
            fronteir = []
            for loc in old_front:
                if loc in self.end:
                    return distance
                for step in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
                    new_loc = rope.pplus(loc, step)
                    if (
                        not self.inside(*new_loc)
                        or new_loc in visited
                        or not self.can_step(loc, new_loc)
                    ):
                        continue
                    fronteir.append(new_loc)
                    visited.add(new_loc)
            distance += 1
        return -1


if __name__ == "__main__":
    hm = Terrain(sys.stdin)
    print(hm.walk())
    hm.invert()
    print(hm.walk())
