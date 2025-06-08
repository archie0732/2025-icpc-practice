# 2131. Longest Palindrome by Concatenating Two Letter Words

>[!note]
>- `medium`
>- `string` `hash map`


## 題目

- 給一個包含字串(**長度一定為二**)，求最長的`對稱字串`的**長度**


## 解題

>[!note]
> 因為自串長度一定為2，所以 如果 xy yx 一定可以加入(長度+4)

1. 先解決本體對稱的(不包含兩個字元一樣的，例如xx、yy)
2. 如果只有一個**相同`字元`**(xx)，那麼可以把它放在中間(ab + xx + ba) ，長度+2
3. 如果有兩組 -> 就跟上面的`1.`一樣，但是根據`2.`應該要把剛剛多加的 `2` 還回來，結束!


### code (CPP)


```cpp
class Solution {
public:
    int longestPalindrome(vector<string>& words) {
        int count = 0;
        int mid = 0;

        vector<vector<int>> v(26, vector<int>(26, 0));

        for(const auto& it : words){
            int x = it[0] - 'a', y = it[1] - 'a';

            if(v[y][x] > 0){
                v[y][x]--;
                count += 4;

                if(x == y)mid--;
            }
            else {
                v[x][y]++;

                if(x == y)mid++;
            }
        }

        return mid > 0 ? count + 2 : count;
        
    }
}; 
```
