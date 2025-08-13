# 326. Power of Three

>[!note]
>- Medium
>- recursive


## 題目

給一個數字`n` 求有沒有一個數`x`滿足 `3^x == n`

## 解題

就按題意解就行，要改成`long long`


### CPP CODE 


```cpp
class Solution {
public:
    bool isPowerOfThree(int n) {

        for(int i = 0; (long long)pow(3, i) <= n; i++){
            if((long long)pow(3, i) == n)return true;
        }
        return false;
    }
};
```