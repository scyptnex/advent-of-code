import functools
import sys

LESS=-1
CONT=0
MORE=1

DIV_2 = [[2]]
DIV_6 = [[6]]

def correct_helper(left, right):
    if type(left) == list and type(right) == list:
        ll = len(left)
        rl = len(right)
        for i in range(min(ll, rl)):
            sub = correct_helper(left[i], right[i])
            if sub != CONT:
                return sub
        return correct_helper(ll, rl)
    if type(left) == list:
        return correct_helper(left, [right])
    if type(right) == list:
        return correct_helper([left], right)
    if left < right:
        return LESS
    if right < left:
        return MORE
    return CONT

def correct(left, right):
    return correct_helper(left, right) == LESS

def parse(s):
    return eval(s)

def sum_of_indicies_in_order(data):
    total = 0
    idx = 1
    while True:
        try:
            l1 = next(data).strip()
        except:
            return total
        if not l1:
            continue
        l2 = next(data).strip()
        if correct(parse(l1), parse(l2)):
            total += idx
        idx += 1

def order_distress(data):
    parsed = [parse(l.strip()) for l in data if l.strip()]
    parsed.append(DIV_2)
    parsed.append(DIV_6)
    parsed.sort(key=functools.cmp_to_key(correct_helper))
    return (parsed.index(DIV_2)+1) * (parsed.index(DIV_6)+1)

if __name__=="__main__":
    print(order_distress(sys.stdin))
