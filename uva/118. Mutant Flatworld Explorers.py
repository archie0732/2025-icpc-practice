import sys


def walk(x, y, d, c):
    if c == "F":
        if d == "N":
            y += 1
        elif d == "W":
            x -= 1
        elif d == "E":
            x += 1
        elif d == "S":
            y -= 1
    elif c == "L":
        if d == "N":
            d = "W"
        elif d == "W":
            d = "S"
        elif d == "S":
            d = "E"
        elif d == "E":
            d = "N"
    elif c == "R":
        if d == "N":
            d = "E"
        elif d == "E":
            d = "S"
        elif d == "S":
            d = "W"
        elif d == "W":
            d = "N"
    return x, y, d


line = sys.stdin.read().split()

lost_x = int(line[0])
lost_y = int(line[1])

index = 2
check = set()

while index < len(line):
    if index + 3 > len(line):
        break  ## ????
    x, y = int(line[index]), int(line[index + 1])
    d = line[index + 2]
    cmd = line[index + 3]

    index += 4
    is_lost = False
    for c in cmd:
        backup_x, backup_y, backup_d = x, y, d

        x, y, d = walk(x, y, d, c)

        if (x < 0 or x > lost_x) or (y < 0 or y > lost_y):
            if (backup_x, backup_y) in check:
                x, y, d = backup_x, backup_y, backup_d
            else:
                is_lost = True
                check.add((backup_x, backup_y))
                x, y, d = backup_x, backup_y, backup_d
                break

    if is_lost:
        print(f"{x} {y} {d} LOST")
    else:
        print(f"{x} {y} {d}")
