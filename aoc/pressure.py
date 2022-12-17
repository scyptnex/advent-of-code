import sys


class Valve:
    def __init__(self, txt):
        strings = txt.strip().split(" ")
        self.location = strings[1]
        self.flow_rate = int(strings[4].split("=")[-1][:-1])
        self.tunnels = [l[:-1] if "," in l else l for l in strings[9:]]


class XValve:
    def __init__(self, v, mapping):
        self.location = mapping[v.location]
        self.flow_rate = v.flow_rate
        self.tunnels = [mapping[t] for t in v.tunnels]


def valves(data):
    return [Valve(l) for l in data]


def xvalves(valves):
    v_good = [v for v in valves if v.flow_rate > 0]
    v_bad = [v for v in valves if v.flow_rate <= 0]
    mapping = {}
    for i, e in enumerate(v_good):
        mapping[e.location] = i
    for i, e in enumerate(v_bad):
        mapping[e.location] = len(v_good) + i
    return mapping, len(v_good)


def open_valve(cur_open, new_valve):
    return cur_open | (1 << new_valve)


def is_open(cur_open, valve):
    return (cur_open >> valve) & 1


def t_len(n):
    return ((n+1)*n)//2

def t_idx(x, y):
    return t_len(x)+y


class SSpace:
    def __init__(self, in_valves):
        self.mapping, self.useful_count = xvalves(in_valves)
        self.valves = {
            v.location: v for v in [XValve(iv, self.mapping) for iv in in_valves]
        }
        self.tbl_pos_count = t_len(len(in_valves))
        self.tbl_val_count = 2**self.useful_count
        self.tbl_lim_count = 26
        tblen = self.tbl_lim_count*self.tbl_val_count*self.tbl_pos_count
        print(tblen)
        print((tblen*4)//(1024*1024), "M")
        self.tbl = [-1 for i in range(tblen)]

    def memo_idx(self, h, e, open_set, limit):
        return (t_idx(h, e)*self.tbl_val_count*self.tbl_lim_count) + (open_set*self.tbl_lim_count) + limit


    def best_move_e(self, h_loc: str, e_loc: str, open_set: int, limit: int) -> int:
        if limit <= 1:
            return 0

        if e_loc > h_loc:
            return self.best_move_e(e_loc, h_loc, open_set, limit)

        key = self.memo_idx(h_loc, e_loc, open_set, limit)
        if self.tbl[key] != -1:
            return self.tbl[key]

        limit -= 1
        moves = []
        h_valve = self.valves[h_loc]
        e_valve = self.valves[e_loc]
        # neither open
        for h in h_valve.tunnels:
            for e in e_valve.tunnels:
                moves.append(self.best_move_e(h, e, open_set, limit))
        # human opens
        if (not is_open(open_set, h_loc)) and h_valve.flow_rate > 0:
            for e in e_valve.tunnels:
                moves.append(
                    self.best_move_e(h_loc, e, open_valve(open_set, h_loc), limit)
                    + limit * h_valve.flow_rate
                )
        # elephant opens
        if (not is_open(open_set, e_loc)) and e_valve.flow_rate > 0:
            for h in h_valve.tunnels:
                moves.append(
                    self.best_move_e(h, e_loc, open_valve(open_set, e_loc), limit)
                    + limit * e_valve.flow_rate
                )
        # both open
        if (
            (not is_open(open_set, h_loc))
            and h_valve.flow_rate > 0
            and (not is_open(open_set, e_loc))
            and e_valve.flow_rate > 0
            and e_loc != h_loc
        ):
            new_open = open_valve(open_valve(open_set, h_loc), e_loc)
            moves.append(
                self.best_move_e(h_loc, e_loc, new_open, limit)
                + (limit * e_valve.flow_rate)
                + (limit * h_valve.flow_rate)
            )
        mx = max(moves)
        self.tbl[key] = mx
        return mx

    def solve_e(self):
        aa = self.mapping["AA"]
        return self.best_move_e(aa, aa, 0, 26)


if __name__ == "__main__":
    print(SSpace(valves(sys.stdin)).solve_e())
