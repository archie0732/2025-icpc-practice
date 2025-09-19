# 1903. Largest Odd Number in String

>[!note]
>- Easy
>- string greedy


## 題目

給一個字串求剪切後的**最大奇數**


## 解題

由右往左切，遇到奇數位直接切

### CPP CODE

>[!tip]
>- substr(起始位置, 長度)

```cpp
class Solution {
public:
    string largestOddNumber(string num) {
        int n = num.size();
        string ans = "";
        for(int i = n - 1; i >= 0; i--){
            if((num[i] - '0') % 2){

                cout << "find it! " << num[i] << endl;
                ans = num.substr(0, i-2); 
            }
        }

        return ans;
    }
};
```
