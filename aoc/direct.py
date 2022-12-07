import sys


class Direct:
    def __init__(self, parent):
        self.parent = parent
        self.files = {}
        self.directs = {}
        self.size = None

    def ls(self, line):
        spl = line.split(maxsplit=1)
        if spl[0] == "dir":
            self.directs[spl[1]] = Direct(self)
        else:
            self.files[spl[1]] = int(spl[0])

    def calc_size(self):
        self.size = 0
        for f in self.files:
            self.size += self.files[f]
        for d in self.directs:
            self.directs[d].calc_size()
            self.size += self.directs[d].size


    def select(self, pred):
        if pred(self.size):
            yield self
        for d in self.directs:
            for td in self.directs[d].select(pred):
                yield td

    def threshold(self, limit):
        for td in self.select(lambda s: s <= limit):
            yield td

    def __str__(self):
        return (
            "{"
            + " ".join([d + ": " + str(self.directs[d]) for d in self.directs])
            + " ".join([f + ": " + str(self.files[f]) for f in self.files])
            + "}"
        )


def read(data):
    root = Direct(None)
    cur_node = root
    for line in data:
        if line[0] != "$":
            cur_node.ls(line)
            continue
        spl = line.split()
        if spl[1] == "ls":
            continue
        # cd
        if spl[2] == "/":
            cur_node = root
        elif spl[2] == "..":
            cur_node = cur_node.parent
        else:
            cur_node = cur_node.directs[spl[2]]
    root.calc_size()
    return root

def get_threshold(root):
    total=0
    for td in root.threshold(100000):
        total += td.size
    return total

def get_deletion(root):
    total_space = 70000000
    required_space = 30000000
    used_space = root.size
    delete_space = used_space - (total_space - required_space)
    
    sizes = [td.size for td in root.select(lambda s : s >= delete_space)]
    sizes.sort()
    return sizes[0]

def readin():
    for l in sys.stdin:
        yield l[:-1]


if __name__ == "__main__":
    print(get_deletion(read(readin())))
