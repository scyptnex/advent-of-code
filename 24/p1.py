#! /usr/bin/env python3

import sys

l1 = []
l2 = []

for line in sys.stdin:
    l = line.strip().split()
    if not l:
        continue
    l1.append(int(l[0]))
    l2.append(int(l[1]))

l1.sort()
l2.sort()

total = 0
for i, e in enumerate(l1):
    total += abs(e - l2[i])

print(total)

hist = {}

for e in l2:
    if e not in hist:
        hist[e] = 0
    hist[e] += 1

sc = 0
for e in l1:
    if e in hist:
        sc += e * hist[e]

print(sc)
