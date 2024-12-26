import sys
import itertools

def mix(a, b):
    return a^b

def prune(a):
    return a%16777216

def gens(seed):
    cur = seed
    while True:
        yield cur
        cur = prune(mix(cur, cur*64))
        cur = prune(mix(cur, cur//32))
        cur = prune(mix(cur, cur*2048))

def amt(seq, i):
    return seq[i]%10

def diff(seq, i):
    return amt(seq, i) - amt(seq, i-1)

def best_bids(seq):
    bid={}
    cur = [diff(seq, i) for i in range(1, 4)]
    cur = (0, cur[0], cur[1], cur[2])
    for i in range(4, len(seq)):
        cx, ca, cb, cc = cur
        cd = diff(seq, i)
        cur = (ca, cb, cc, cd)
        if cur not in bid:
            bid[cur] = amt(seq, i)
    return bid

def total_bid(bid, bests):
    return sum(b[bid] for b in bests if bid in b)


def go():
    tot = 0
    seeds = [int(l.strip()) for l in sys.stdin.readlines() if l.strip()]
    seqs = [[q for q in itertools.islice(gens(x), 2001)] for x in seeds]
    bids = [best_bids(s) for s in seqs]
    bst = (0, 0, 0, 0)
    bbest = total_bid(bst, bids)
    print(sum(s[-1] for s in seqs))
    for a in range(-9, 10):
        for b in range(-9, 10):
            for c in range(-9, 10):
                for d in range(-9, 10):
                    cur = (a, b, c, d)
                    bcur = total_bid(cur, bids)
                    if bcur > bbest:
                        bbest = bcur
                        bst = cur
    print(bbest)

go()
