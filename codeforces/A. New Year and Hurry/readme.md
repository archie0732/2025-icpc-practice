# New Year and Hurry

>[!note]
> 800
> binary search

[Qustion Link](https://codeforces.com/problemset/problem/750/A)

## 題目

給一個題目數量k與走路到派對的時間，比賽於20:00開始，如果要24:00前到派對請問最多可以寫幾題（注意！1~k題中，每題要花的時間為 i X 5，例如要寫兩題則要花 5 X 1+ 5 X 2 = 15 分鐘）

## 解題

1. binary search
2. 等差級數公式 total = (1 + n) X n/2，又因為要花5分鐘 totol X 5

## CODE

```py
def main() -> int:
    p, m = map(int, input().split())
    allCanUse = 4 * 60 - m

    # print(f"debug: all can use: {allCanUse}")

    def binarySearch() -> int:
        l, r = 0, p + 1

        while l < r:
            mid = (l + r) // 2
            canUse = 5 * mid * (mid + 1) // 2

            if canUse <= allCanUse:
                l = mid + 1
            else:
                r = mid

        return l - 1

    print(binarySearch())
    return 0


main()

```
