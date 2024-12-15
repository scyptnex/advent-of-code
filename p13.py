import sys


def prs(s: str):
    return [int(h.strip()[2:]) for h in s.split(": ")[1].split(",")]


def val(x, a, b, n):
    return (a * n + b * x) / a


def slv_old(ax, ay, bx, by, sx, sy):
    ret = -1
    for a in range(0, 101):
        rx, ry = (sx - a * ax, sy - a * ay)
        if rx < 0 or ry < 0:
            continue
        if rx % bx != 0:
            continue
        b = rx // bx
        if b * by != ry:
            continue
        sol = a * 3 + b * 1
        print("SOL", a, b, sol)
        if ret == -1 or sol < ret:
            ret = sol
    if ret == -1:
        return 0
    return ret


def slv(ax, ay, bx, by, sx, sy):
    print(ax, ay, bx, by, sx, sy)
    n = bx * sy - by * sx
    n /= bx

    x_icpt = (ax * bx * n) / (bx * ay - ax * by)
    x_i = int(round(x_icpt))

    a_press = x_i // ax
    print(n, x_icpt, x_i, a_press)

    rx = sx - a_press * ax
    ry = sy - a_press * ay

    if rx % bx != 0:
        return 0

    b = rx // bx

    if b * by != ry:
        return 0

    return a_press * 3 + b


lns = [l.strip() for l in sys.stdin.readlines()]

tot = 0
tot2 = 0
for i in range(0, len(lns), 4):
    print()
    ax, ay = prs(lns[i])
    bx, by = prs(lns[i + 1])
    sx, sy = prs(lns[i + 2])
    tot += slv(ax, ay, bx, by, sx, sy)
    tot2 += slv(ax, ay, bx, by, sx + 10000000000000, sy + 10000000000000)
print(tot)
print(tot2)
