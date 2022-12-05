import sys


def main():
    cur_elf = 0
    elves = []
    for l in sys.stdin:
        line = l[:-1]
        if not line:
            elves.append(cur_elf)
            cur_elf = 0
        else:
            cur_elf += int(line)
    elves.append(cur_elf)

    elves.sort()
    print(elves)
    print(elves[-1])
    print(elves[-3] + elves[-2] + elves[-1])


if __name__ == "__main__":
    main()
