# 1221. Split a String in Balanced Strings

>[!note]
>- Esye
>- string greedy


## questuin

>[!tip}
> 平衡字串表示字串中的字元數量都一樣，注意題目中只有L與Ｒ兩個字元


給一個`平衡字串`求最多可以拆成幾個`平衡子字串`

- 注意只能照順序拆分，不得更換順序


## 解題

由左往右拆 => 結束

### CPP CODE

```cpp
class Solution {
public:
    int balancedStringSplit(string s) {
        int ans = 0;

        int l = 0, r = 0;
        for(char c : s){
            if(c == 'L')l++;
            else r++;

            if(r == l){
                ans++;
                l = 0;
                r = 0;
            }
        }
        return ans;
    }
};
```
