import sys

switched = False
walls = set()
boxes = set()
bot = None
dirs = ""
wh_w = 0
wh_h = 0
for row, l in enumerate(sys.stdin.readlines()):
    l = l.strip()
    if not l:
        switched = True
    elif switched:
        dirs += l
    else:
        wh_h += 1
        wh_w = len(l * 2)
        for colin, c in enumerate(l):
            col = colin * 2
            if c == "#":
                walls.add((row, col))
                walls.add((row, col + 1))
            elif c == "O":
                boxes.add((row, col))
            elif c == "@":
                bot = (row, col)


def offs(c):
    if c == "<":
        return (0, -1)
    if c == ">":
        return (0, 1)
    if c == "^":
        return (-1, 0)
    return (1, 0)


def move(p, d):
    return (p[0] + d[0], p[1] + d[1])


def push(gap, drc, pushed_boxes):
    if gap in walls:
        return False
    # the"real" box does the pushing
    if gap in boxes:
        pushed_boxes.add(gap)
        if drc == "<":
            return push(move(gap, offs(drc)), drc, pushed_boxes)
        elif drc == ">":
            return push(move(move(gap, offs(drc)), offs(drc)), drc, pushed_boxes)
        elif drc == "^":
            return push(move(gap, offs(drc)), drc, pushed_boxes) and push(
                move(move(gap, offs(drc)), offs(">")), drc, pushed_boxes
            )
        else:
            return push(move(gap, offs(drc)), drc, pushed_boxes) and push(
                move(move(gap, offs(drc)), offs(">")), drc, pushed_boxes
            )
    lft_half = move(gap, offs("<"))
    if lft_half in boxes:
        return push(lft_half, drc, pushed_boxes)
    return True


def trybot(bot, drc):
    nb = move(bot, offs(drc))
    pushed_boxes = set()
    if not push(nb, drc, pushed_boxes):
        return bot
    for pb in pushed_boxes:
        boxes.remove(pb)
    for pb in pushed_boxes:
        boxes.add(move(pb, offs(drc)))
    return nb


def show():
    for r in range(wh_h):
        for c in range(wh_w):
            v = "."
            if (r, c) in walls:
                v = "#"
            elif (r, c) in boxes:
                v = "["
            elif move((r, c), offs("<")) in boxes:
                v = "]"
            elif (r, c) == bot:
                v = "@"
            print(v, end="")
        print()


show()
for drc in dirs:
    bot = trybot(bot, drc)
show()

tot = 0
for br, bc in boxes:
    tot += 100 * br + bc
print(tot)
