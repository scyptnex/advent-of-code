import sys

def digits(i: int) -> list[int]:
    s = str(i)
    if len(s)%2 == 1:
        return None
    return [int(s[:len(s)//2]), int(s[len(s)//2:])]

def evolve(stns: list[int]) -> list[int]:
    ret = []
    for s in stns:
        if s == 0:
            ret.append(1)
            continue;
        hlvs = digits(s)
        if hlvs:
            ret += hlvs
        else:
            ret.append(s*2024)
    return ret

def slv(mem, val, iters):
    if iters == 0:
        return 1
    if (val, iters) not in mem:
        ev = evolve([val])
        ret = 0
        for e in ev:
            ret += slv(mem, e, iters-1)
        mem[(val, iters)] = ret
    return mem[(val, iters)]
    
mem={}
stones = [int(v) for v in sys.stdin.readline().strip().split()]

t25 = 0
t75 = 0
for s in stones:
    t25 += slv(mem, s, 25)
    t75 += slv(mem, s, 75)
print(t25)
print(t75)
