import sys

def blocks(i):
    cur=[]
    for l in i:
        l = l.strip()
        if l:
            cur.append(l)
        else:
            yield cur
            cur = []
    yield cur

def parsel(b):
    ret=[]
    for i in range(len(b[0])):
        h = 0
        for hc in range(1, len(b)):
            if b[hc][i] == '.':
                break
            h += 1
        ret.append(h)
    return ret

def tkl(ks, ls, b):
    if b[0] == '#'*len(b[0]):
        ls.append(parsel(b))
    else:
        ks.append(parsel([l for l in reversed(b)]))

def overlaps(k, l):
    for i in range(len(k)):
        if k[i] + l[i] > 5:
            return True
    return False

def go():
    ks = []
    ls = []
    for b in blocks(sys.stdin.readlines()):
        tkl(ks, ls, b)
    count=0
    for k in ks:
        for l in ls:
            if not overlaps(k, l):
                count += 1
    print(count)
go()
