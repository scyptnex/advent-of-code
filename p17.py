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
        self.dump()
        while self.pc < len(pgm):
            print(pgm[self.pc], end="")
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
            self.dump()
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
        self.a = (self.a//(2**self.combo(v)))&(2**32 - 1)

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
        self.b = (self.a//(2**self.combo(v)))&(2**32 - 1)

    def cdv(self, v):
        self.c = (self.a//(2**self.combo(v)))&(2**32 - 1)


def go():
    a = int(sys.stdin.readline().split(": ")[1].strip())
    b = int(sys.stdin.readline().split(": ")[1].strip())
    c = int(sys.stdin.readline().split(": ")[1].strip())
    sys.stdin.readline()
    pgm = [int(x) for x in sys.stdin.readline().split(": ")[1].strip().split(",")]
    print(",".join(str(i) for i in Cpu(a, b, c).run(pgm)))
    print(",".join(str(i) for i in Cpu(117440, b, c).run(pgm)))


go()
