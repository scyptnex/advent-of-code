import sys


class Cpu:
    def __init__(self, ar, br, cr):
        self.a = ar
        self.b = br
        self.c = cr
        self.pc = 0
        self.out = []

    def run(self, pgm):
        self.pc = 0
        self.out = []
        #self.dump()
        while self.pc < len(pgm):
            #print(pgm[self.pc], end="")
            r = [
                self.adv,
                self.bxl,
                self.bst,
                self.jnz,
                self.bxc,
                self.outt,
                self.bdv,
                self.cdv,
            ][pgm[self.pc]](pgm[self.pc + 1])
            if r is None:
                self.pc += 2
            else:
                self.pc = r
            #self.dump()
        return self.out

    def dump(self):
        print(repr(self.__dict__))

    def combo(self, v):
        if v == 4:
            return self.a
        elif v == 5:
            return self.b
        elif v == 6:
            return self.c
        return v

    def adv(self, v):
        self.a = (self.a//(2**self.combo(v)))

    def bxl(self, v):
        self.b = self.b ^ v

    def bst(self, v):
        self.b = self.combo(v)%8

    def jnz(self, v):
        if self.a:
            return v

    def bxc(self, v):
        self.b = self.b ^ self.c

    def outt(self, v):
        self.out.append(self.combo(v)%8)

    def bdv(self, v):
        self.b = (self.a//(2**self.combo(v)))

    def cdv(self, v):
        self.c = (self.a//(2**self.combo(v)))

def do(a, pgm):
    return ",".join(str(i) for i in Cpu(a, 0, 0).run(pgm))

def can_apply(cur, cmask, new, nmask):
    xmask = cmask&nmask
    return cur&xmask == new&xmask

def pb(b):
    return "".join('1' if b&(1<<i) else '0' for i in range(63, -1, -1))

def search(pgm, idx, cur, mask):
    if idx == len(pgm):
        return cur
    tgt = pgm[idx]
    best = -1
    for i in range(0, 8):
        ni = i << (3*idx)
        mi = 7 << (3*idx)
        if not can_apply(cur, mask, ni, mi):
            continue
        ccur = cur | ni
        cmask = mask |  mi
        bo = i^1
        bx = bo^5
        need = tgt^bx
        need <<= (bo + 3*idx)
        nmask = 7 << (bo + 3*idx)
        if not can_apply(ccur, cmask, need, nmask):
            continue
        nxt = ccur | need
        nxtmask = cmask | nmask
        check = search(pgm, idx+1, nxt, nxtmask)
        if check != -1 and (check < best or best == -1):
            best = check
    return best



def go():
    a = int(sys.stdin.readline().split(": ")[1].strip())
    b = int(sys.stdin.readline().split(": ")[1].strip())
    c = int(sys.stdin.readline().split(": ")[1].strip())
    sys.stdin.readline()
    pgm = [int(x) for x in sys.stdin.readline().split(": ")[1].strip().split(",")]
    print(do(a, pgm))

    start_mask = (2**16-1) << 48
    print(search(pgm, 0, 0, start_mask))

go()
