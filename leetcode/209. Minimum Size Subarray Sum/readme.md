# 209. Minimum Size Subarray Sum

[link](https://leetcode.com/problems/minimum-size-subarray-sum/description/?envType=problem-list-v2&envId=binary-search)

## 解題

先往右擴建`prefix`，當大於 `target` 時 

1. 先看範圍
2. 再嘗試減去 `nums[start]`
3. start++

>[!note]
>## 注意! 一定要按上述方式
> 不然再範圍為 `0 ~ n` 時會漏算



### CODE

```py
class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        n = len(nums)

        ans = 10000000
        prefix = 0
        start = 0
        for i in range(n):
            prefix += nums[i]

            while prefix >= target:
                ans = min(ans, i - start + 1)
                prefix -= nums[start]
                start += 1
        
        return ans if ans != 10000000 else 0
```
