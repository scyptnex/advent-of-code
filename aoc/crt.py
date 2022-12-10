import sys


class Crt:
    def __init__(self, t=0, x=1):
        self.t = t
        self.x = x

    def execute(self, i):
        i = i.strip()
        if i == "noop":
            return Crt(self.t + 1, self.x)
        else:
            v = int(i.split()[1])
            return Crt(self.t + 2, self.x + v)


def sequence(data):
    c_new = Crt()
    yield c_new
    for l in data:
        c_new = c_new.execute(l)
        yield c_new

def render(seq):
    cur = next(seq)
    render=[]
    line=""
    for s in seq:
        for t in range(cur.t, s.t):
            if t%40 in [cur.x - 1, cur.x, cur.x+1]:
                line += "#"
            else:
                line += "."
            if t % 40 == 39:
                render.append(line)
                line = ""
        cur = s
    render.append(line)
    return render


def events(seq):
    c_old = None
    c_new = next(seq)
    ev = 20
    for s in seq:
        c_old = c_new
        c_new = s
        if c_old.t < ev and c_new.t >= ev:
            yield ev, c_old.x
            ev += 40


def readin():
    for l in sys.stdin:
        yield l[:-1]


def record_seq(seq):
    total=0
    for s in seq:
       c, x = s
       total += c*x
       if c >= 220:
           break
    return total

if __name__ == "__main__":
    print("\n".join(render(sequence(readin()))))

