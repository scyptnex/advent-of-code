import sys

ROCKS = [
        [(0,1)]*4,
        [(1,1),(0,3),(1,1)],
        [(0,1)]*2+[(0,3)],
        [(0,4)],
        [(0,2)]*2,
]

def generate_rock(r_count, max_height):
    r = ROCKS[r_count%len(ROCKS)]
    rock = []
    for i, y_slice in enumerate(r):
        lowest_y, height = y_slice
        rock += [(i+2, lowest_y + max_height + y + 3) for y in range(height)]
    return rock

def shift(rock, dx, dy):
    return [(x+dx, y+dy) for x, y in rock]

EMPTY=0
ROCK=1

class Tetris:
    def __init__(self, wind):
        self.wind = wind
        self.w_idx = 0
        self.r_count = 0
        self.width = 7
        self.field = []
        self.highest = [0 for _ in range(self.width)]
        self.max_height = 0

    def taken(self, x, y):
        return self.field[y][x] != EMPTY

    def extend(self, rock):
        highest = max(rp[1] for rp in rock)
        field_height = len(self.field)
        if field_height > highest:
            return
        self.field += [[EMPTY for _ in range(self.width)] for _ in range(field_height, highest+1)]

    def valid_rock(self, rock):
        for x, y in rock:
            if x < 0 or x >= self.width or y < 0:
                return False
            if self.taken(x, y):
                return False
        return True

    def maybe_wind(self, rock):
        cur_wind = -1 if self.wind[self.w_idx] == "<" else 1
        self.w_idx = (self.w_idx+1)%len(self.wind)
        new_rock = shift(rock, cur_wind, 0)
        if self.valid_rock(new_rock):
            return new_rock
        return rock

    def maybe_drop(self, rock):
        new_rock = shift(rock, 0, -1)
        if self.valid_rock(new_rock):
            return new_rock
        for x, y in rock:
            self.field[y][x] = ROCK
            self.highest[x] = max(y+1, self.highest[x])
        self.max_height = max(self.highest)
        return None


    def fall(self):
        rock_pos = generate_rock(self.r_count, self.max_height)
        self.r_count += 1
        self.extend(rock_pos)
        while True:
            rock_pos = self.maybe_wind(rock_pos)
            rock_pos = self.maybe_drop(rock_pos)
            if not rock_pos:
                break


    def go(self, rocks=2022):
        for _ in range(rocks):
            self.fall()
        return self.max_height

if __name__ == "__main__":
    t = Tetris(next(sys.stdin).strip())
    print(t.go())
