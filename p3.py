import sys
import re

text =  "".join(sys.stdin.readlines())
pres=text.split("mul(")
pres=[p[:p.find(')')] for p in pres if ')' in p]
pattern = re.compile('^[0-9]+,[0-9]+$')
pres=[[int(s) for s in p.split(',')] for p in pres if pattern.match(p)]
sums=[l[0]*l[1] for l in pres]
total = 0
for s in sums:
    total+=s
print(total)

matchr = re.compile('''do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)''')
doing = True
t2 = 0
for match in matchr.finditer(text):
    m = match.group(0)
    if m == 'do()':
        doing = True
    elif m == 'don\'t()':
        doing = False
    elif doing:
        t2 += (int(match.group(1)) * int(match.group(2)))
print(t2)
