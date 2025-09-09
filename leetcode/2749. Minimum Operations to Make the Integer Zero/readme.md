# 2749. Minimum Operations to Make the Integer Zero

>[!note]
>- `Medium`
>- `binary`

## 題目

轉換為公式:

```
num1 = k*num2 + [2^(i_1) + 2^(i_2)......2^(i_k)]
```

求`k`

## slove

照著公式寫

>[!important]
>先 x = num1-num2 * k　如果 x < k 則一定組不成，因為 2^i 最小為 1 。k*1 > x 一定不成立

### CPP CODE

```cpp
class Solution {
public:
    int makeTheIntegerZero(int num1, int num2) {
        int ans = 1;

        while(true){
            long long x = num1 - static_cast<long long>(num2) * ans;

            if(x < ans){
                return -1;
            }

            if(ans >= countOne(x)){
                return ans;
            }
            ans++;
        }

        return false;
    }

private:
    int countOne(long long x){
        int count = 0;

        while(x > 0){
            if(x & 1){
                count++;
            }
            x = x >> 1;
        }

        return count;
    }
};
```
