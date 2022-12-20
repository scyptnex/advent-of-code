import sys

class DllNode:

    def __init__(self, val:int):
        self.val = val
        self.next = None
        self.prev = None

    def before(self, other):
        op = other.prev
        self.next = other
        self.prev = op
        other.prev = self
        op.next = self

    def after(self, other):
        self.before(other.next)

class Seq:

    def __init__(self, data):
        self.zero = None
        for d in data:
            n = DllNode(int(d.strip()))
            if self.zero is None:
                n.prev = n
                n.next = n
                self.zero = n
            else:
                n.before(self.zero)

    def gen(self):
        if self.zero is None:
            return
        s = self.zero
        while True:
            yield s
            s = s.next
            if s == self.zero:
                return

    def print(self):
        for n in self.gen():
            print("{} -> {} -> {}".format(n.prev.val, n.val, n.next.val))


if __name__=="__main__":
    pass
