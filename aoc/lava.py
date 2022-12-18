import sys

from aoc import rope

ADJACENTS = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
]


def read_cube(line: str) -> (int, int, int):
    spl = [int(l) for l in line.strip().split(",")]
    return (spl[0], spl[1], spl[2])


class Bounds:
    def __init__(self):
        self.min = 1
        self.max = -1

    def track(self, val):
        if self.min > self.max:
            self.min = val - 1
            self.max = val + 1
            return
        self.min = min(self.min, val - 1)
        self.max = max(self.max, val + 1)

    def __repr__(self):
        return "{}-{}".format(self.min, self.max)

    def rng(self):
        return range(self.min, self.max + 1)

    def inside(self, v):
        return v >= self.min and v <= self.max


class Blob:
    def __init__(self, data):
        self.cubes = {}
        self.bounds = [Bounds(), Bounds(), Bounds()]
        for l in data:
            self.add_cube(read_cube(l))

    def add_cube(self, cube):
        if cube in self.cubes:
            return
        self.cubes[cube] = 6
        for i, b in enumerate(self.bounds):
            b.track(cube[i])
        for adj in ADJACENTS:
            check = rope.pplus(cube, adj)
            if check in self.cubes:
                self.cubes[cube] -= 1
                self.cubes[check] -= 1

    def inside(self, c):
        return all(b.inside(c[i]) for i, b in enumerate(self.bounds))

    def surface_area(self):
        total = 0
        for c in self.cubes:
            total += self.cubes[c]
        return total

    def enclose(self) -> set:
        origin_l = [b.min for b in self.bounds]
        origin = (origin_l[0], origin_l[1], origin_l[2])
        q = [origin]
        enclosure = set(q)
        while q:
            cur = q.pop()
            for adj in ADJACENTS:
                check = rope.pplus(cur, adj)
                if check in enclosure:
                    continue
                if not self.inside(check):
                    continue
                if check in self.cubes:
                    continue
                enclosure.add(check)
                q.append(check)
        return enclosure

    def add_internals(self):
        enc = self.enclose()
        for x in self.bounds[0].rng():
            for y in self.bounds[1].rng():
                for z in self.bounds[2].rng():
                    c = (x, y, z)
                    if c in enc or c in self.cubes:
                        continue
                    self.add_cube(c)
        return self


if __name__ == "__main__":
    b = Blob(sys.stdin)
    print(b.surface_area())
    b.add_internals()
    print(b.surface_area())
