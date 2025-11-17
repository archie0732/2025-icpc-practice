# 3228. Maximum Number of Operations to Move Ones to the End

>[!note]
>- Medium 

## 解題

其實很簡單，看以下例子

```md
1 0 0 1 1 0 1
```
移動 index = 0

```md
0 0 1 1 1 0 1 # 移動 1 次
```

移動 index = {3, 4, 5}

```md
0 0 0 1 1 1 1 # 移動 1+2 次
```

由以上可以看出越右邊遇到 0 要移動的步數就是 **在這個 0 左邊的 1 的次數**

>[!tip]
>需要注意！要處理連續遇到 0 的狀況，不處理會連加

- ex:

```md
1 0 0 1 1 0 1 # 移動第一位不管連續0會變成 +1 +1 但其實只移動一次
```


### CODE

```py
class Solution:
    def maxOperations(self, s: str) -> int:
        ans = 0
        oneces = 0
        last = '1'
        for i in s:
            if i == '1':
                oneces += 1
            elif i == '0' and last != '0':
                ans += oneces
            last = i
        return ans
```


