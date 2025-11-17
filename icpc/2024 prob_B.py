n = int(input())

a = [int(x) for x in input().split()]

start, end = 0, 0
mx, cur = 0, 0
mp = [0] * len(a)


for i, x in enumerate(a):
    if x > (cur + x):
        print(f"debug: {x}, {cur+x}")
        mp[i] = i
        cur = x
    else:
        cur += x
        mp[i] = mp[i - 1] if i > 0 else 0

    if mx < cur:
        mx = cur
        start = mp[i]
        end = i


print(f"{start}, {end}, {mx}")

ans = mx * 2
for i, x in enumerate(a):
    if i >= start and i <= end:
        continue
    ans += abs(x)

print(ans)
