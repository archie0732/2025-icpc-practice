# 1138A - Sushi for Two

>[!note]
> 900

[question](https://codeforces.com/problemset/problem/1138/A)

## 解題

略

```py
x = int(input(""))
susi = list(map(int, input().split()))

last = susi[0]
a = [0] * 3
ans = 0
a[last] += 1
for i in range(1, x):
    cur = susi[i]
    # print(f"cur: {cur}, last: {last}")
    if last == cur:
        a[cur] += 1
    else:
        a[cur] = 1
    temp = min(a[1], a[2])
    ans = max(temp, ans)
    # print(f"1: {a[1]}, 2: {a[2]}")
    last = cur

print(ans * 2)
```
