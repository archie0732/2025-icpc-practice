import sys


def main():
    line1 = sys.stdin.readline().split()

    n = int(line1[0])
    roads = int(line1[-1])

    line2 = sys.stdin.readline().split()

    lanterns = [int(x) for x in line2]
    lanterns.sort()

    if n == 1:
        print(f"{max(lanterns[0], roads - lanterns[0]):.10f}")
        return

    max_gap = 0.0

    last = lanterns[0]

    for lantern in lanterns[1:]:
        gap = lantern - last
        if gap > max_gap:
            max_gap = float(gap)
        last = lantern

    ans = max(lanterns[0], roads - lanterns[-1], max_gap / 2.0)

    print(f"{ans:.10f}")


main()
