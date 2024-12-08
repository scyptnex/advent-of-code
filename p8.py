import sys

def anti(a, b, w, h, antis):
    ar, ac = a
    br, bc = b
    xr, xc = (ar-(br-ar), ac-(bc-ac))
    if xr < 0 or xc < 0 or xr >= h or xc >= w:
        return
    antis.add((xr, xc))

def outside(xr, xc, w, h):
    return xr < 0 or xc < 0 or xr >= h or xc >= w

def anti2(a, b, w, h, antis):
    ar, ac = a
    br, bc = b
    dr, dc = (br-ar, bc-ac)
    if dr == 0:
        # share row, row is anti except between
        dc = dc // abs(dc)
    if dc == 0:
        dr = dr // abs(dr)
    jmp = 0
    while True:
        xar, xac = (ar - dr*jmp, ac-dc*jmp)
        xbr, xbc = (br + dr*jmp, bc+dc*jmp)
        ao = outside(xar, xac, w, h)
        bo = outside(xbr, xbc, w, h)
        if ao and bo:
            return
        if not ao:
            antis.add((xar, xac))
        if not bo:
            antis.add((xbr, xbc))
        jmp += 1

antenna={}
height=0
width=0
for r, line in enumerate(sys.stdin.readlines()):
    line = line.strip()
    height+=1
    width=0
    for c, ch in enumerate(line.strip()):
        width+=1
        if ch == '.':
            continue
        if ch not in antenna:
            antenna[ch] = []
        antenna[ch].append((r, c))
antis=set()
antis2=set()
for lbl in antenna:
    cur = antenna[lbl]
    for i in range(0, len(cur)):
        for j in range(i+1, len(cur)):
            anti(cur[i], cur[j], width, height, antis)
            anti(cur[j], cur[i], width, height, antis)
            anti2(cur[i], cur[j], width, height, antis2)
print(len(antis))
print(len(antis2))
