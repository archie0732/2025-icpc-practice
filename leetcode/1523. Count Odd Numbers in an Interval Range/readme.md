# 1523. Count Odd Numbers in an Interval Range

[qustion link](https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/description/?envType=daily-question&envId=2025-12-07)

>[!note]
> Easy
> Math

## 題目

給 `low` 跟 `high` 求這區間一共有幾個基數解(頭尾都算)

## 解題

1. 直接用 `for` 開跳 (沒試過不是到能不能過)
2. 用數學算

>這裡用方法2

```md
1 2 3 4 5 6 7
長度為 7 共有 4 個基數
```

可以看到基數是**間隔一格出現**(廢話)

那麼就可以用除的來看

>[!tip]
>要注意頭部事蹟數還是偶數

- 由於這裡是 用 `(頭 - 尾) // 2` 來算的基本會 (會少算頭)
>所以可以直接 先定位頭為基數(偶數就+1) 然後算玩數量後 + 1 (補算頭)

### CODE

```py
class Solution:
    def countOdds(self, low: int, high: int) -> int:

        if low % 2 == 0:
            low += 1

        return 1 + (high - low) // 2
```
