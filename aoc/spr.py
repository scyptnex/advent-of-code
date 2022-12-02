import sys

THEM = {
    "A": "R",
    "B": "P",
    "C": "S",
}

YOU = {
    "X": "R",
    "Y": "P",
    "Z": "S",
}

PROP = {
    "X": 0,
    "Y": 3,
    "Z": 6,
}

SCORE = {
    "R": 1,
    "P": 2,
    "S": 3,
}


def outcome_score(them, you):
    diff = SCORE[you] - SCORE[them]
    diff = -diff // abs(diff) if abs(diff) > 1 else diff
    return 3 + 3 * diff


def score(them, you):
    return SCORE[you] + outcome_score(them, you)


def total(seq):
    t = 0
    for s in seq:
        t += score(THEM[s[0]], YOU[s[1]])
    return t

def determine_play(them, outcome):
    for k in YOU:
        if PROP[outcome] == outcome_score(THEM[them], YOU[k]):
            return k
    raise "ROFL"

def recode(seq):
    for s in seq:
        yield [s[0], determine_play(s[0], s[1])]

def parse(gen):
    for l in gen:
        yield l.strip().split()


def main():
    print(total(recode(parse(sys.stdin))))


if __name__ == "__main__":
    main()
