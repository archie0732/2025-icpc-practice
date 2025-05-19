# 5. Longest Palindromic Substring


>[!note]
>- Medium
>- dp、string


## 題意

給一個字串，回傳最長的`對稱字串`(例如`aba` `aca` `aaaa`)


## 解題

用`dp` -> 紀錄前面一個是不是對稱

```
X 對稱 X
```

因為中間是對稱的，所以當**頭的X == 尾巴的X**時就會產生新的對稱

**!注意!**

- 單個字為對稱
- 如果是連續的字`aa` `bb` 也是對稱 -> 要另外去設
- 剩下用`dp`陣列

### dp 陣列


`typeof dp[j][i] -> boolean` 即`j`到`i`之間的字串為對稱

所以，`dp[i][i] -> true`，如果`X1 == X2`且`dp[X1+1][X2-1] == true`即成立!

需要注意，在長度為二(例: `aa`)時，不用符合`dp[X1+1][X2-1]`(因為我們還未將`dp[X2][X2]`設為`true`)!


### code (CPP)


![](/image/5.jpg)


```cpp

class Solution {
public:
    string longestPalindrome(string s) {
        if(s.size() <= 1) return s;

        vector<vector<bool>>dp(s.size(), vector<bool>(s.size(), false));


        int start = 0, end = 0;
        int max_len = 1;
        

        for(int i = 0; i < s.size(); i++){
            dp[i][i] = true;
            for(int j = 0; j < i; j++){
                if(s[i] == s[j] && (i - j <= 2 || dp[j + 1][i - 1])){
                    dp[j][i] = true;

                    if(i - j + 1 > max_len){
                        max_len = i - j + 1;
                        start = j;
                        end = i;
                    }
                }
            }
        }

        return s.substr(start, end - start + 1);
    }
};
```