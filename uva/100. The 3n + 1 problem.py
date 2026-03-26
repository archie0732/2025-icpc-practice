import sys

cache = {1: 1}


def alg(n):
    if n in cache:
        return cache[n]

    if n % 2 == 0:
        next = n // 2
    else:
        next = 3 * n + 1
    cache[n] = 1 + alg(next)
    return cache[n]


for line in sys.stdin:
    parts = line.split()

    if not parts:
        break

    a, b = map(int, parts)
    print(f"{a} {b}", end=" ")
    if a > b:
        a, b = b, a
    ans = 0
    for i in range(a, b + 1):
        ans = max(ans, alg(i))

    print(ans)
