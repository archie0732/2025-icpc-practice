# 240. Search a 2D Matrix II

[question](https://leetcode.com/problems/search-a-2d-matrix-ii/?envType=problem-list-v2&envId=binary-search)

## 解題

做一個二分搜

>再那之前先看邊界，matrix[i][0] <= target <= matrix[i][-1]。超出邊界就可以跳過了


### CODE

```py
class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        
        for i in range(len(matrix)):
            if matrix[i][0] > target or matrix[i][-1] < target:
                continue
            
            l = 0; r = len(matrix[i]) -1

            while l <= r:
                mid = l + (r - l) // 2

                x = matrix[i][mid]

                if x == target:
                    return True
                elif x < target:
                    l = mid + 1
                else:
                    r = mid - 1
                
        return False
```
