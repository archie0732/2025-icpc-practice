# 1323. Maximum 69 Number

>[!note]
>- Medium
>- greedy


## 題目

給一個只包含 `6` 與 `9` 的數字，只改一個字的情況下求最大的數字為多少

## 解題

把最靠左邊的 `6` 變成 `9` 就可以了

### CPP CODE


```cpp
class Solution {
public:
    int maximum69Number (int num) {
        string s = to_string(num);

        for(char &c : s){
            if(c == '6'){
                c = '9';
                
                int ans = 0;
                for(char a : s){
                    ans = ans * 10 + (a - '0');
                }
                return ans;
            }
        }
        return num;
        
    }
};
```
