t = int(input())

for i in range(t):
    n = int(input())
    a = list(map(int, (input().split())))

    ans = 0

    for i in range(n - 1, 0, -1):
        for j in range(0, i):
            if a[j] > a[j + 1]:
                a[j], a[j + 1] = a[j], a[j + 1]
                ans += 1
    print(f"Optimal train swapping takes {ans} swaps.")
