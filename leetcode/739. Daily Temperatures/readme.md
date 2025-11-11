# 739. Daily Temperatures

[739. Daily Temperatures](https://leetcode.com/problems/daily-temperatures/description/)

## 解題

典型單調棧問題

>求下一個比他大的值

>[!note]
>注意！他是找下一個跟自己的相對位置

### CODE

```py
class Solution:
    def dailyTemperatures(self, t: List[int]) -> List[int]:
        n = len(t)
        stack = []
        stackIndex = []
        ans = [0]*n

        for i in range(n):
            x = t[i]
            while len(stack) > 0 and stack[-1] < x:
                index = stackIndex.pop()
                # print(f'test-1, {index} ans[index] = {ans[index]}')
                if ans[index] == 0:
                    #print('test-2')
                    ans[index] = i - index
                stack.pop()
            stack.append(x)
            #print(f'push {x} and index {i}')
            stackIndex.append(i)
        return ans


```

2025 @ arch1e & copyright
