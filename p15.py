import sys

switched = False
walls=set()
boxes=set()
bot=None
dirs=""
wh_w=0
wh_h=0
for row, l in enumerate(sys.stdin.readlines()):
    l = l.strip()
    if not l:
        switched = True
    elif switched:
        dirs += l
    else:
        wh_h+=1
        wh_w=len(l)
        for col, c in enumerate(l):
            if c == '#':
                walls.add((row, col))
            elif c == 'O':
                boxes.add((row, col))
            elif c == '@':
                bot = (row, col)

def offs(c):
    if c == '<':
        return (0, -1)
    if c == '>':
        return (0, 1)
    if c == '^':
        return (-1, 0)
    return (1, 0)

def move(p, d):
    return (p[0]+d[0], p[1]+d[1])

def makegap(gap, off):
    if gap in walls:
        return False
    if gap in boxes:
        ng = move(gap, off)
        if not makegap(ng, off):
            return False
        boxes.remove(gap)
        boxes.add(ng)
        return True
    return True

def trybot(bot, off):
    nb = move(bot, off)
    if makegap(nb, off):
        return nb
    return bot

def show():
    for r in range(wh_h):
        for c in range(wh_w):
            v = '.'
            if (r, c) in walls:
                v = '#'
            elif (r, c) in boxes:
                v = 'O'
            elif (r, c) == bot:
                v = '@'
            print(v, end='')
        print()

show()
for drc in dirs:
    o = offs(drc)
    bot = trybot(bot, o)
show()

tot = 0
for br, bc in boxes:
    tot += (100*br + bc)
print(tot)

