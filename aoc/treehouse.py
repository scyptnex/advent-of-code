import sys

def get_row(data, r):
    return data[r]

def get_col(data, c):
    for r in range(0, len(data)):
        yield data[r][c]

def yield_visible_forwards(seq: str):
    cur = '\0'
    for i, e in enumerate(seq):
        if e > cur:
            yield i
            cur = e

def yield_visible(seq: str):
    for v in yield_visible_forwards(seq):
        yield v
    l = len(seq) - 1
    for v in yield_visible_forwards(seq[::-1]):
        yield l - v

def num_visible(data):
    visible = [[False for c in r] for r in data]
    visible_count = 0
    n_rows = len(data)
    n_cols = len(data[0])
    for r in range(0,n_rows):
        for c in yield_visible(get_row(data, r)):
            visible_count += 0 if visible[r][c] else 1
            visible[r][c] = True
    for c in range(0,n_cols):
        for r in yield_visible("".join(get_col(data, c))):
            visible_count += 0 if visible[r][c] else 1
            visible[r][c] = True
    return visible_count

def get_score(data, ir, ic, n_rows, n_cols):
    score = 1
    t_height = data[ir][ic]
    for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
        dr, dc = direction
        r = ir+dr
        c = ic+dc
        dscore = 0
        while r < n_rows and r >= 0 and c < n_cols and c >= 0:
            dscore += 1
            if data[r][c] >= t_height:
                break
            r += dr
            c += dc
        if dscore == 0:
            return 0
        score *= dscore
    return score


def sceinic_score(data):
    n_rows = len(data)
    n_cols = len(data[0])
    max_score = 0
    for r in range(n_rows):
        for c in range(n_cols):
            score = get_score(data, r, c, n_rows, n_cols)
            if score > max_score:
                max_score = score
    return max_score



if __name__=="__main__":
    data = [l[:-1] for l in sys.stdin]
    print(num_visible(data))
    print(sceinic_score(data))
