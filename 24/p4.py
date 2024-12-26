import sys
import re


def isxmas(grid, dr, dc, row, col):
    for i, c in enumerate("XMAS"):
        pr = row + i * dr
        pc = col + i * dc
        if pr < 0 or pc < 0 or pr >= len(grid) or pc >= len(grid[pr]):
            return False
        if grid[pr][pc] != c:
            return False
    return True


def find(grid, dr, dc):
    count = 0
    for row in range(len(grid)):
        for col in range(len(grid[row])):
            if isxmas(grid, dr, dc, row, col):
                count += 1
    return count


total = 0
grid = [l.strip() for l in sys.stdin.readlines()]
for dr in [-1, 0, 1]:
    for dc in [-1, 0, 1]:
        if dr == 0 and dc == 0:
            continue
        total += find(grid, dr, dc)
print(total)

######################################################


def hasx(grid, row, col):
    if grid[row][col] != "A":
        return False
    c1 = [grid[row - 1][col - 1], grid[row + 1][col + 1]]
    c2 = [grid[row - 1][col + 1], grid[row + 1][col - 1]]
    c1.sort()
    c2.sort()
    return c1 == ["M", "S"] and c2 == ["M", "S"]


total = 0
for row in range(1, len(grid) - 1):
    for col in range(1, len(grid[row]) - 1):
        if hasx(grid, row, col):
            total += 1
print(total)
