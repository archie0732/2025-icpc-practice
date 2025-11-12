# 53. Maximum Subarray


[53. Maximum Subarray](https://leetcode.com/problems/maximum-subarray/description/)

## 解題

典型的 Kadane 題目

### CODE 

```py
class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        mx, cur = nums[0], nums[0]

        for x in nums[1:]:
            cur = max(x, cur + x)
            # print(f'{cur}, {mx}')
            mx = max(cur, mx)
        
        return mx
```
