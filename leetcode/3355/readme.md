# 3355. Zero Array Transformation I


>[!note]
>- Medium
>- prefixSum

## 題意

給一個`nums`陣列與一個2D陣列`queries`

而在 `queries[i] = [l, r]`下可以將`nums`裡面的`l到r`範圍中的數字**減一**

求在跑完`queries`後能否將`nums`裡面的數字都變為`0`


- 舉例 `queries` 為 `[[0, 2], [1, 3]]` ，而`queries[0] = [0, 2]` 表示可以將 `nums`中的 0 ~ 2 間的數字`-1`!


### 解題

其實簡單的解法就是將`queries`遍歷的範圍都記錄在`dp`內(`dp[i]++`)但是會超時 -> 請見 `TLE.cpp`

所以這裡用的方法為建一個`prefix`數，當到`queries`內的`l`時`prefix`就加，這樣就可以確保`l->尾`都有被加過

那`r`要停止的部分就讓`prefix`的`r+1`去減去這樣到`r+1`時就會將`l->r`的部分還回來



- 例子

```md
0 0 1 0 0 -1 # 指範圍 2 ~ 4

# prefix 變化

0 -> 0 -> 1 -> 1 -> 1 -> 0  # 只有 範圍 2~4 有加到
```



### code (CPP) 可見 AC.cpp


```cpp

class Solution {
public:
    bool isZeroArray(vector<int>& nums, vector<vector<int>>& queries) {
        int n = nums.size();
        vector<int>dp(n + 1, 0);

        for(const auto& it : queries){
            int l = it[0], r = it[1];

            dp[l]++;
            dp[r + 1]--;
        }

        int preSum = 0;
        for(int i = 0; i < n; i++){
            preSum += dp[i];
            if(preSum < nums[i])return false;
        }

        return true;
    }
};
```