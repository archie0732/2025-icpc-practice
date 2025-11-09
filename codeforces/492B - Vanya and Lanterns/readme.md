# 492B - Vanya and Lanterns

[question](https://codeforces.com/problemset/problem/492/B)

## 解題

按照題目來解即可

>[!note]
> 注意街道開始與第一個燈的距離，還有最後一個燈與街道尾端的距離是否大於燈之間的間隙
>[!tip]
>注意!當輸入很多時切記python不要使用

```py
a = [int(x) for x in input().split()] # 會 runtime error
```

- 使用

```py
line = sys.stdin.redline().split()

arr = [int(x) for x in line]
```

## CODE

```py
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
```
