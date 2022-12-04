import sys


def contains(a, b):
    a1, a2 = a
    b1, b2 = b
    return (a1 >= b1 and a2 <= b2) or (b1 >= a1 and b2 <= a2)


def overlaps(a, b):
    a1, a2 = a
    b1, b2 = b
    return (
        (b1 >= a1 and b1 <= a2)
        or (b2 >= a1 and b2 <= a2)
        or (a1 >= b1 and a1 <= b2)
        or (a2 >= b1 and a2 <= b2)
    )


def range_to_pairs(r):
    x = r.split(",")
    x = x[0].split("-") + x[1].split("-")
    return ((int(x[0]), int(x[1])), (int(x[2]), int(x[3])))


def count_contained(data):
    total = 0
    for l in data:
        if contains(*range_to_pairs(l)):
            total += 1
    return total


def count_overlaps(data):
    total = 0
    for l in data:
        if overlaps(*range_to_pairs(l)):
            total += 1
    return total


def readin():
    for l in sys.stdin:
        yield l[:-1]


if __name__ == "__main__":
    print(count_overlaps(readin()))
