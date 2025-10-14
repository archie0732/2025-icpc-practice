# 3349. Adjacent Increasing Subarrays Detection I


>[!note]
> Easy


## 題目

>給一個陣列，與數字 ｋ。求2個長度為`k`的子陣列，並且要是原陣列間相連且子陣列要**嚴格遞增**

## 解題

too easy, skip

## CODE

```python

class Solution:
    def hasIncreasingSubarrays(self, nums: List[int], k: int) -> bool:
        
        def isIncreasing(start: int) -> bool:
            count = 0

            if start >= len(nums) or start+k > len(nums):
                return False

            last = nums[start]

            # print(f'start at {start}')
            for x in nums[start+1:(start+k)]:
                if x <= last:
                    return False
                last = x
            return True
        
        for i in range(0, len(nums)):
            # print('======')
            first = False 
            second = False
            if(isIncreasing(i) and isIncreasing(i+k)):
                return True
        return False
```
