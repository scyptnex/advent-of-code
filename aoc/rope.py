import sys

UP = (0, 1)
DOWN = (0, -1)
LEFT = (-1, 0)
RIGHT = (1, 0)

DIRS = {
    "U": UP,
    "D": DOWN,
    "L": LEFT,
    "R": RIGHT,
}


def pwise(l, a, b):
    return tuple(l(x, y) for (x, y) in zip(a, b))


def pplus(a, b):
    return pwise(lambda x, y: x + y, a, b)


def pdiff(a, b):
    return pwise(lambda x, y: x - y, a, b)


def move1d(d, dp):
    if d < 0:
        return -move1d(-d, dp)
    return d // 2 if abs(dp) < 2 else (d // abs(d) if d != 0 else 0)


def move(diff):
    x, y = diff
    return (move1d(x, y), move1d(y, x))


def get_new_pos(head, tail, direction):
    h2 = pplus(head, direction)
    diff = pdiff(h2, tail)
    return h2, pplus(move(diff), tail)


def count_tail_pos(data, rlen=2):
    snake = [(0, 0) for _ in range(rlen)]
    t_locs = set([snake[-1]])
    for l in data:
        d = DIRS[l[0]]
        for _ in range(int(l[2:])):
            snake[0] = pplus(snake[0], d)
            for i in range(1, rlen):
                diff = pdiff(snake[i - 1], snake[i])
                snake[i] = pplus(move(diff), snake[i])
            t_locs.add(snake[-1])
    return len(t_locs)


def readin():
    for l in sys.stdin:
        yield l[:-1]


if __name__ == "__main__":
    print(count_tail_pos(readin(), 10))
