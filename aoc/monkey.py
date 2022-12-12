import sys


def parse_directive(data):
    l = next(data)
    return l.split(":", 2)[1].strip()


class Monkey:
    def __init__(self, data):
        self.items = [int(i.strip()) for i in parse_directive(data).split(",")]
        self.op = parse_directive(data).split("=", 2)[1]
        self.div = int(parse_directive(data).split()[-1])
        self.true_t = int(parse_directive(data).split()[-1])
        self.false_t = int(parse_directive(data).split()[-1])
        self.inspections = 0
        self.panic = 0

    def __str__(self):
        return "i={} op={} div={} panic={} t={} f={}".format(
            self.items, self.op, self.div, self.panic, self.true_t, self.false_t
        )

    def do_op(self, old):
        return eval(self.op)

    def business(self):
        for i in self.items:
            r = self.do_op(i)
            self.inspections += 1
            if self.panic == 0:
                r = r // 3
            else:
                r = r % self.panic
            if r % self.div == 0:
                yield self.true_t, r
            else:
                yield self.false_t, r
        self.items = []

    def catch(self, v):
        self.items.append(v)


class MonkeyBusiness:
    def __init__(self):
        self.monkeys = []

    def read_in(self, data):
        while True:
            try:
                l = next(data)
            except:
                break
            l = l.strip()
            if not l:
                continue
            self.monkeys.append(Monkey(data))

    def business(self, rounds=20):
        for r in range(rounds):
            for m in self.monkeys:
                for t, v in m.business():
                    self.monkeys[t].catch(v)
        ins = [m.inspections for m in self.monkeys]
        ins.sort()
        return ins[-2] * ins[-1]

    def panic(self):
        p = 1
        for m in self.monkeys:
            p *= m.div
        for m in self.monkeys:
            m.panic = p


if __name__ == "__main__":
    mb = MonkeyBusiness()
    mb.read_in(sys.stdin)
    mb.panic()
    print(mb.business(rounds=10000))
