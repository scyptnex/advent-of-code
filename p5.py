import sys

not_allowed_after = {}
lists = []
for line in sys.stdin.readlines():
    if not line.strip():
        continue
    if "|" in line:
        rule = line.strip().split("|")
        if rule[1] not in not_allowed_after:
            not_allowed_after[rule[1]] = set()
        not_allowed_after[rule[1]].add(rule[0])
    else:
        lists.append(line.strip().split(","))


def excluded(l, not_allowed_after):
    for i, before in enumerate(l):
        for after in l[i + 1 :]:
            if before in not_allowed_after and after in not_allowed_after[before]:
                return True
    return False


def reorder(l, not_allowed_after):
    ret = [l[0]]
    for e in l[1:]:
        placement = len(ret)
        for pl in range(len(ret)):
            r = ret[pl]
            if r in not_allowed_after and e in not_allowed_after[r]:
                placement = pl
                break
        ret.insert(placement, e)
    return ret


totl = 0
tx = 0
for l in lists:
    if not excluded(l, not_allowed_after):
        totl += int(l[len(l) // 2])
    else:
        l2 = reorder(l, not_allowed_after)
        tx += int(l2[len(l2) // 2])
print(totl)
print(tx)
