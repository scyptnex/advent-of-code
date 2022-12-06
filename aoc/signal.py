import sys

def signal(stream, plen=4):
    seen=['\0']*plen
    count = 0
    for c in stream:
        count += 1
        seen = seen[1:] + [c]
        if seen[0] == '\0':
            continue
        if len(set(seen)) == plen:
            return count
    return -1

def read_by_char():
    while True:
        c = sys.stdin.read(1)
        if not c:
            break
        yield c

def main():
    print(signal(read_by_char(), plen=14))

if __name__ == "__main__":
    main()
