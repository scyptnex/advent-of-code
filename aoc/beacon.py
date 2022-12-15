
import sys

from aoc import rope


def clean(s):
    if s[-1] in (",", ":"):
        s = s[:-1]
    return s[2:]

def parse(s):
    sp = s.split()
    spx = [sp[2], sp[3], sp[-2], sp[-1]]
    spx = [int(clean(c)) for c in spx]
    return ((spx[0], spx[1]), (spx[2], spx[3]))

def merge(ranges):
    end = None
    sta = None
    ret = []
    for r in sorted(ranges):
        rs, rt = r
        if sta is None:
            sta = rs
            end = rt
            continue
        if rs <= end+1:
            end = max(end, rt)
            continue
        ret.append((sta, end))
        sta = rs
        end = rt

    if sta != None:
        ret.append((sta, end))
    return ret



class Sense:
    def __init__(self, sensor, beacon):
        self.s_loc = sensor
        self.b_loc = beacon
        xd, yd = rope.pdiff(sensor, beacon)
        self.dist = abs(xd) + abs(yd)

    def influence(self, y):
        sx, sy = self.s_loc
        diff = abs(y - sy)
        if diff > self.dist:
            return None
        rdr = self.dist - diff
        return (sx - rdr, sx + rdr)



class Beacons:

    def __init__(self, data):
        self.beacons = set()
        self.sensors = []
        for l in data:
            sens = Sense(*parse(l.strip()))
            self.beacons.add(sens.b_loc)
            self.sensors.append(sens)

    def excluded(self, y):
        influences = [s.influence(y) for s in self.sensors]
        influences = [i for i in influences if i != None]
        return merge(influences)

    def count_excluded(self, y):
        excl = self.excluded(y)
        total = 0
        for x in excl:
            x1, x2 = x
            total += x2 - x1
        return total

    def find_beacon(self, limit, progress=False):
        for y in range(limit):
            if y%100000 == 0 and progress:
                print(y)
            ex = self.excluded(y)
            if len(ex) > 1:
                return ex[0][1] + 1, y


if __name__=="__main__":
    b = Beacons(sys.stdin)
    print(b.count_excluded(2000000))
    x = b.find_beacon(4000000, progress = True)
    print(x)
    print(x[0]*4000000 + x[1])
