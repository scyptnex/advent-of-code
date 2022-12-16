import sys


class Valve:
    def __init__(self, txt):
        strings = txt.strip().split(" ")
        self.location = strings[1]
        self.flow_rate = int(strings[4].split("=")[-1][:-1])
        self.tunnels = [l[:-1] if "," in l else l for l in strings[9:]]


def valves(data):
    return [Valve(l) for l in data]


best_move_tbl = {}


def best_move(loc: str, valves: dict, open_set: frozenset, limit: int) -> int:
    if limit <= 0:
        return 0

    key = (loc, open_set, limit)
    if key in best_move_tbl:
        return best_move_tbl[key]

    limit -= 1
    v_loc = valves[loc]
    moves = [best_move(adj, valves, open_set, limit) for adj in v_loc.tunnels]
    if (v_loc.location not in open_set) and v_loc.flow_rate > 0:
        m = limit * v_loc.flow_rate + best_move(
            loc, valves, open_set.union([loc]), limit
        )
        moves.append(m)
    mx = max(moves)
    best_move_tbl[key] = mx
    return mx


def solve(in_valves, limit=30):
    valves = {v.location: v for v in in_valves}
    return best_move("AA", valves, frozenset(), limit)


if __name__ == "__main__":
    print(solve(valves(sys.stdin)))
