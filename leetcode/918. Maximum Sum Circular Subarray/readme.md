# 918. Maximum Sum Circular Subarray


[918. Maximum Sum Circular Subarray](https://leetcode.com/problem-list/monotonic-queue/)

## 解題

> 看到 最大陣列(連續)最大值想到 `Kadane` (`dp` 的一種)

但是這一題有一個問題，就是他是循環陣列

```py
[1, 2, 4, 1] + [1, 2, 4, 1] + .....
```

但是有限制長度只能是 `n  = len(arr)`

就是同個位置的值不可重複加

例如

```py
[1, 1, 3, 4] # = 9 是最大不可以在加 1 (index = 0) 被加過了
```

這個時候就可以想到一個招

把陣列開看

```py
[1, 1, 1, -1, -1, 1]
```

把 1 當成正數的值， -1 是 < 0 的值

>根據規則，我們可以把頭尾拼接，略過中間的值

- total - (< 0 的值)

所以可以把這當成:

```py
max(全部的值 - 最小的區間, Kadane 找到的最大值)
```

至於找到最大跟最小的陣列值可以用 `Kadane`

### CODE

```py

class Solution:
    def maxSubarraySumCircular(self, nums: List[int]) -> int:
        
        cur_max, cur_min = 0, 0
        mn, mx = nums[0], nums[0]

        total = 0

        for i in nums:
            cur_max = max(i, cur_max + i)
            cur_min = min(i, cur_min + i)
            mn = min(mn, cur_min)
            mx = max(mx, cur_max)

            total += i
        # print(f'{mx}, {mn}, {total}')
        
        return max(mx, total - mn) if mx > 0 else mx
```

