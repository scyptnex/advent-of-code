import sys

def common_item(a, b):
    a_set = set(a)
    for b_char in b:
        if b_char in a_set:
            return b_char
    raise Exception("Couldnt find duplicate")

def common_sack(s):
    return common_item(s[:len(s)//2], s[len(s)//2:])

def priority(l):
    return (ord(l) - ord("A") + 27) if l.isupper() else (ord(l) - ord("a") + 1);


def badge(triple):
    set_a = set(triple[0])
    set_b = set(triple[1])
    for c in triple[2]:
        if c in set_a and c in set_b:
            return c

def triples_to_sack(data):
    while True:
        d1 = next(data, None)
        if d1 == None:
            break
        d2 = next(data)
        d3 = next(data)
        b = badge([d1, d2, d3])
        yield b+b


def score(data):
    total = 0
    for d in data:
        total += priority(common_sack(d))
    return total


def readin():
    for l in sys.stdin:
        yield l[:-1]


if __name__=="__main__":
    print(score(triples_to_sack(readin())))

