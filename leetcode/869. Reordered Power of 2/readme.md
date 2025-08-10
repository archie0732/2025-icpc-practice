# 869. Reordered Power of 2

[question link](https://leetcode.com/problems/reordered-power-of-2/description/?envType=daily-question&envId=2025-08-10)


>[!note]
> `Medium`
> `bit` `HasH Table`

## 解題

1. 先列出所有`2` 的次方 `2^0  ~ 2^30`
2. 然後去看有沒有一樣

>[!tip]
> 可以用 `sort` 如果 整理結果一樣那麼就找得到答案了！！！

## cpp code

```cpp
class Solution {
public:
    bool reorderedPowerOf2(int n) {
        
        string s = to_string(n);
        sort(s.begin(), s.end());

        
        for(int i = 0; i <= 30; i++){
           string  ans = to_string(1 << i);

           sort(ans.begin(), ans.end());

           if(ans == s)
            return true;
        }

        return false;
    }
};
```
