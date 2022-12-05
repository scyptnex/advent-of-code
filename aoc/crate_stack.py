import sys


class CrateStack:
    def __init__(self):
        self.stacks = []

    def move(self, command):
        cs = command.split()
        count = int(cs[1])
        orig = int(cs[3]) - 1
        dest = int(cs[5]) - 1
        for i in range(0, count):
            self.stacks[dest].append(self.stacks[orig].pop())

    def move2(self, command):
        cs = command.split()
        count = int(cs[1])
        orig = int(cs[3]) - 1
        dest = int(cs[5]) - 1
        crane = []

        for i in range(0, count):
            crane.append(self.stacks[orig].pop())

        for i in range(0, count):
            self.stacks[dest].append(crane.pop())


def problem(lines):
    c = CrateStack()
    num_stacks = int(lines[-1][-1])
    stacks = [[] for i in range(0, num_stacks)]
    for l in lines[:-1]:
        for idx in range(0, num_stacks):
            charr = 4 * idx + 1
            if charr >= len(l):
                continue
            if l[charr] == " ":
                continue
            stacks[idx].append(l[charr])
    [s.reverse() for s in stacks]
    c.stacks = stacks
    return c


def read_in(data):
    p = None
    stacks = []
    for d in data:
        d = d.rstrip()
        if not d:
            p = problem(stacks)
        elif p != None:
            p.move2(d)
        else:
            stacks.append(d)
    return "".join([s[-1] for s in p.stacks])


if __name__ == "__main__":
    print(read_in(sys.stdin))
