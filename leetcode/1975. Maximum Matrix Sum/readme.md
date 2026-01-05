# 1975. Maximum Matrix Sum


>[!note]
>Midum greedy

## Question

[qustion link](https://leetcode.com/problems/maximum-matrix-sum/description/?envType=daily-question&envId=2026-01-05)

## 解題

可以無限的把兩個**相鄰**的數字都乘`-1`(正負顛倒)

>表示可以把任意位置的兩個數正負顛倒

舉例

```md
1. -1 1 1 1 -1 # 把頭尾的 -1 變為 1
2. 1 -1 1 1 -1 # 把 1 2 同乘 -1
3. 1 1 -1 1 -1 # 把 2 3 同乘 -1
4. 1 1 1 -1 -1
5. 1 1 1 1 1
```

同樣的 `cloumn` 也可以做到，由此可知斜角也是可以(看是要先上下在左右或是先左右在上下)

所以如果board上有偶數的負數可以直接轉換成**正數總和相加**

如果是基數個負數那麼一定會有一個數字是負數 => 找 board 中絕對值最小的數字把他變成負

### CODE

```python

class Solution:
    def maxMatrixSum(self, matrix: List[List[int]]) -> int:
        
        count = 0
        mn = 100000
        total = 0

        for x in matrix:
            for i in x:
                if i < 0:
                    count += 1
                total += abs(i)
                mn = min(abs(i), mn)
        
        print(total, mn)
        if count % 2 == 0:
            return total
        else:
            return total - mn*2


```
