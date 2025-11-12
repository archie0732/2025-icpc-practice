# 2654. Minimum Number of Operations to Make All Array Elements Equal to 1


[2654. Minimum Number of Operations to Make All Array Elements Equal to 1](https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/description/?envType=daily-question&envId=2025-11-12)

## 解題

先將可能的情況做分析

1. 陣列已經有 1
2. 陣列沒有 1 但是可以透過 `gcd(i, i+1)` 制做出至少 1 組
3. 陣列沒有 1 但是可以透過多個 gcd 製造出 (即 g = gcd(i,i+1) g = gcd(g, i+2).....g==1)
4. 做不出 1

根據以下情況

>1. 陣列已經有 1

因為任何數跟 1 做 gcd 一定是 1，所以可以擴散到整個陣列

如果有多個 1 就可以減少一次

>2. 陣列沒有 1 但是可以透過 `gcd(i, i+1)` 制做出至少 1 組

找到後即可用第1條=> 先做 1 次 + 陣列長度 - 1 (那個被做出來的 1 不用)

>3. 陣列沒有 1 但是可以透過多個 gcd 製造出

```py
g = gcd(i,i+1), g = gcd(g, i+2).....g==1
```

即為:

花費 res 次 做出 1 + 陣列長度(1的擴散次數) - 1 (減去 res 做出的 1)

做出後

>4. 做不出 1

執行完 1 2 3 後都沒有 就是 4 回傳 -1


### CODE

```py

class Solution:
    def minOperations(self, nums: List[int]) -> int:
        once = nums.count(1)

        n = len(nums)
        if once > 0:
            return n - once
        
        res = inf
        for i in range(n - 1):
            g = nums[i]
            for j in range(i+1, n):
                g = gcd(g, nums[j])

                if g == 1:
                    res = min(res, j - i)
                    break
        
        if res == inf:
            return -1
        return res + n - 1
```
