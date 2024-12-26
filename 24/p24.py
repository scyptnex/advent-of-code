import sys
import queue
import re


def addl(nets, lhs, rhs, op, out):
    if lhs not in nets:
        nets[lhs] = []
    nets[lhs].append((rhs, op, out))




def do_det(lhs, op, rhs, out):
    if op == "AND":
        return [out, lhs and rhs]
    elif op == "OR":
        return [out, lhs or rhs]
    else:
        return [out, lhs ^ rhs]


def det(netw, q: queue.deque, wires, val):
    if val[0] in wires:
        return
    wires[val[0]] = val[1]
    if val[0] not in netw:
        return
    for rhs, op, out in netw[val[0]]:
        if rhs not in wires:
            continue
        rv = wires[rhs]
        q.append(do_det(val[1], op, rv, out))


def wr(wrs, n):
    if n not in wrs:
        return ''
    if wrs[n]:
        return ' [label="1"]'
    return ' [label="0"]'


def dotg(netw, wrs):
    print("digraph foo {")
    for s, s_adj in netw.items():
        for t, op, res in s_adj:
            t_adj = (op, res)
            if s > t:
                continue
            node = s + t_adj[0] + t
            print("  {} -> {}{};".format(s, node, wr(wrs, s)))
            print("  {} -> {}{};".format(t, node, wr(wrs, t)))
            print("  {} -> {}{};".format(node, t_adj[1], wr(wrs, t_adj[1])))
    print("}")

def solvewires(netw, initial):
    q = queue.deque(initial)
    wires = {}
    while q:
        v = q.popleft()
        det(netw, q, wires, v)
    return wires

def solve(netw, initial):
    wires = solvewires(netw, initial)
    wl = [(n, wires[n]) for n in wires if n[0] == "z"]
    wl.sort()
    wl.reverse()
    res = 0
    for w in wl:
        res = res * 2
        if w[1]:
            res += 1
    return res

def tc(adj, n):
    f = [n]
    ret = set(f)
    while f:
        cur = f.pop()
        if cur not in adj:
            continue
        for a in adj[cur]:
            if a not in ret:
                ret.add(a)
                f.append(a)
    return ret

def fmt(ltr, nbr):
    return "{}{:02d}".format(ltr, nbr)

def simul(netw, nbr, ones):
    initial = []
    for i in range(0, nbr):
        for l in ['x', 'y']:
            ini = fmt(l, i)
            initial.append((ini, ini in ones))
    wrs = solvewires(netw, initial)
    onez = []
    zerz = []
    nonz = []
    for i in range(0, nbr):
        z = fmt('z', i)
        if z in wrs:
            if wrs[z]:
                onez.append(z)
            else:
                zerz.append(z)
        else:
            nonz.append(z)
    return (onez, zerz, nonz)

def worked(ozn, expected):
    o, z, n = ozn
    return o == expected and not n

def check(netw, nbr, cur):
    x = fmt('x', cur)
    y = fmt('y', cur)
    z = fmt('z', cur)
    zc = fmt('z', cur+1)
    srcs = []
    dsts = [z]
    if zc in netw:
        dsts.append(zc)
    if x in netw:
        srcs += [x, y]
        if not worked(simul(netw, nbr, set([x])), [z]):
            return ([x], [z])
        if not worked(simul(netw, nbr, set([y])), [z]):
            return ([y], [z])
        if not worked(simul(netw, nbr, set([x, y])), [zc]):
            return ([x, y], [zc])
    if cur > 0:
        xp = fmt('x', cur-1)
        yp = fmt('y', cur-1)
        srcs += [xp, yp]
        if not worked(simul(netw, nbr, set([xp, yp])), [z]):
            return ([xp, yp], [z])
        if x in netw:
            if not worked(simul(netw, nbr, set([xp, yp, x])), [zc]):
                return ([xp, yp, x], [zc])
            if not worked(simul(netw, nbr, set([xp, yp, y])), [zc]):
                return ([xp, yp, y], [zc])
            if not worked(simul(netw, nbr, set([xp, yp, x, y])), [z, zc]):
                return ([xp, yp, y], [zc])
    return ([], [], srcs, dsts)

def swapn(net_in, a, b):
    net_out = {}
    for src in net_in:
        net_out[src] = []
        for dst, op, res in net_in[src]:
            if res == a:
                net_out[src].append((dst, op, b))
            elif res == b:
                net_out[src].append((dst, op, a))
            else:
                net_out[src].append((dst, op, res))
    return net_out


def determine_wrong(netw, known_good, fwd, bwd, nbr, cur, inp, oup):
    onz, ofz, noz = simul(netw, nbr, set(inp))
    suspa = tc(bwd, oup[0]).difference(known_good)
    suspb = tc(bwd, onz[0]).difference(known_good)
    for susa in suspa:
        for susb in suspb:
            net2 = swapn(netw, susa, susb)
            c = check(net2, nbr, cur)
            print(susa, susb, c)
            if c[0] or c[1]:
                continue
            #return (net2, susa, susb)
    raise Exception("IDK!!!")


def analyze(netw):
    fwd = {}
    bwd = {}
    for n in netw:
        for m, o, x in netw[n]:
            if n not in fwd:
                fwd[n] = set()
            if m not in fwd:
                fwd[m] = set()
            fwd[n].add(x)
            fwd[m].add(x)
            if x not in bwd:
                bwd[x] = set()
            bwd[x].add(m)
            bwd[x].add(n)
    pat = re.compile("^z[0-9][0-9]$")
    zs = [x for x in bwd if pat.match(x)]
    zs.sort()
    known_good = set()
    bads = []
    for i in range(len(zs)):
        c = check(netw, len(zs), i)
        if c[0] or c[1]:
            print(i, c)
            raise("BAD!")
        else:
            for kgs in c[2]:
                for kgd in c[3]:
                    known_good |= tc(fwd, kgs) & tc(bwd, kgd)

def go():
    inits = []
    nets = {}
    initing = True
    for line in sys.stdin.readlines():
        l = line.strip()
        if not l:
            initing = False
            continue
        if initing:
            sp = l.split(": ")
            inits.append((sp[0], True if sp[1] == "1" else False))
        else:
            sp = l.split(" ")
            addl(nets, sp[0], sp[2], sp[1], sp[4])
            addl(nets, sp[2], sp[0], sp[1], sp[4])
    print(solve(nets, inits))
    #dotg(nets, {})
    swaps=[
           ['z07', 'gmt'],
           ['qjj', 'cbj'],            
           ['dmn', 'z18'],
           ['cfk', 'z35'],
            ]
    l = []
    for s in swaps:
        nets = swapn(nets, s[0], s[1])
        l += s
    analyze(nets)
    print(",".join(sorted(l)))

go()
