import sys

def addgap(gaps, o, w):
    if w not in gaps:
        gaps[w] = []
    gaps[w].append(o)

cur = 0
disk = []
is_file=True
disk2 = []
gaps={}

for c in sys.stdin.readline().strip():
    w = int(c)
    if is_file:
        disk2.append((cur, len(disk), w))
        disk += [cur]*w
        cur += 1
    else:
        addgap(gaps, len(disk), w)
        disk += [-1]*w
    is_file = not is_file

end = len(disk)-1
start = 0
while start < end:
    if disk[start] != -1:
        start += 1
    elif disk[end] == -1:
        end -= 1
    else:
        disk[start] = disk[end]
        disk[end] = -1
        start += 1
        end -= 1

tot = 0
for i, e in enumerate(disk):
    if e == -1:
        break
    tot += i*e

print(tot)

def find_and_fill(gaps, fi, fw):
    ming = -1
    mino = -1
    for w in range(fw, 10):
        if w in gaps and gaps[w] and (ming == -1 or gaps[w][0] < mino):
            ming = w
            mino = gaps[w][0]
    if ming == -1:
        return -1
    if mino >= fo:
        return -1
    newg = ming - fw
    if newg > 0:
        if newg not in gaps:
            gaps[newg] = []
        gaps[newg].append(mino + fw)
        gaps[newg].sort()
    gaps[ming] = gaps[ming][1:]
    return mino

f = len(disk2)-1
while f > 0:
    fi, fo, fw = disk2[f]
    no = find_and_fill(gaps, fo, fw)
    if no == -1:
        f -= 1
        continue
    disk2[f] = (fi, no, fw)
    f -= 1

tot = 0
for fi, fo, fw in disk2:
    for i in range(0, fw):
        tot += fi*(fo+i)
print(tot)
