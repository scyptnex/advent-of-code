import sys


def safe(l):
    gradient = 0
    for i in range(1, len(l)):
        diff = l[i] - l[i - 1]
        if abs(diff) < 1 or abs(diff) > 3:
            return False
        if gradient == 0:
            gradient = diff
            continue
        if gradient / diff < 0:
            return False
    return True


def safel(l, i):
    if safe(l[:i] + l[i + 1 :]):
        return True
    if safe(l[: i - 1] + l[i:]):
        return True
    if i > 1 and safe(l[: i - 2] + l[i - 1 :]):
        return True
    return False


def safe2(l):
    gradient = 0
    errs = 0
    for i in range(1, len(l)):
        diff = l[i] - l[i - 1]
        if abs(diff) < 1 or abs(diff) > 3:
            return safel(l, i)
        if gradient == 0:
            gradient = diff
            continue
        if gradient / diff < 0:
            return safel(l, i)
    return True


levels = [
    [int(spl.strip()) for spl in line.split() if spl] for line in sys.stdin.readlines()
]
safe_count = 0
for l in levels:
    if safe(l):
        safe_count += 1
print(safe_count)

safe_count = 0
for l in levels:
    if safe2(l):
        safe_count += 1
print(safe_count)
