import sys

def cansolve(target, terms, cur, nxt):
    if nxt == len(terms):
        return cur == target
    if cansolve(target, terms, cur+terms[nxt], nxt+1):
        return True
    return cansolve(target, terms, cur*terms[nxt], nxt+1)

def solve(sln, terms):
    print(sln, terms)
    if cansolve(sln, terms, terms[0], 1):
        return sln
    return 0

def joined(a, b):
    return int(str(a) + str(b))

def cansolve2(target, terms, cur, nxt):
    if nxt == len(terms):
        return cur == target
    if cansolve2(target, terms, cur+terms[nxt], nxt+1):
        return True
    if cansolve2(target, terms, joined(cur, terms[nxt]), nxt+1):
        return True
    return cansolve2(target, terms, cur*terms[nxt], nxt+1)

def solve2(sln, terms):
    print(sln, terms)
    if cansolve2(sln, terms, terms[0], 1):
        return sln
    return 0

tot = 0
tot2 = 0
for line in sys.stdin.readlines():
    line = line.strip()
    if not line:
        continue
    spl = [int(s.split(':')[0]) for s in line.split(' ')]
    tot += solve(spl[0], spl[1:])
    tot2 += solve2(spl[0], spl[1:])
print(tot)
print(tot2)
