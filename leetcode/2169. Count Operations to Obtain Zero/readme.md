# 2169. Count Operations to Obtain Zero


[link](https://leetcode.com/problems/count-operations-to-obtain-zero/description/?envType=daily-question&envId=2025-11-09)


## 解題

相減並紀錄次數

### CODE

```py
class Solution:
    def countOperations(self, num1: int, num2: int) -> int:
        ans = 0

        while num1 > 0 and num2 > 0:
            if num1 < num2:
                num1, num2 = num2, num1
            
            num1 -= num2
            ans+=1

        return ans
```
