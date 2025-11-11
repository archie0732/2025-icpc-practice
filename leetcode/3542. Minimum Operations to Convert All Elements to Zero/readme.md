# 3542. Minimum Operations to Convert All Elements to Zero


[question](https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/description/?envType=daily-question&envId=2025-10-16)


## 解題

>找把最高(低)位，單調棧

- 舉例

```md
1 3 2 2 3 2 2 1
```

- 可以先拆分全部(1)

```md
X 3 2 2 3 2 2 Ｘ
```

- 接著拆分 (2)

```md
X 3 O O 3 O O X
```

剩下的兩要分兩次

用以上範例可以推裡出

> 1 2 3 這種遞增會需要 `+1` `+1`
> 
> 1 3 2 1 由 1 經過山峰後下降到 1，1 不用再增加次數
>
> 1 2 2 3 2 2 1 因為山谷 1=1 2=2 由此可以看出只需使用3次

所以我們只要管比前面更大就可以了
不過當下降的時候比較的值也要下降

因為

```
1 3 1 2 3
```
因為把 1 變成 0 會截斷最後面的 (2, 3)
所以把 2 設為最低點 3 > 2 所以要多做一次

### CODE

```py
class Solution:
    def minOperations(self, nums: List[int]) -> int:
        stack = [-1]
        ans = 0
        for x in nums:
            while x < stack[-1]:
                stack.pop()
            if stack[-1] < x and x != 0:
                stack.append(x)
                ans += 1
        
        return ans
```
