# 3350. Adjacent Increasing Subarrays Detection II

>[!note]
> Medium

## 題目

略

## 解題

略

### CODE

```py
class Solution:
    def maxIncreasingSubarrays(self, nums: List[int]) -> int:
        n = len(nums)
        ans, pre , cur = 0, 0, 1
        for i in range(1, n):
            if nums[i-1] < nums[i]:
                cur += 1
            else:
                pre = cur
                cur = 1
            a = min(cur, pre)
            candidate = max(cur//2, a)
            ans = max(ans, candidate)
        return ans

```
