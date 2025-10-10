# 3147. Taking Maximum Energy From the Mystic Dungeon


>[!note]
>- Medium
>- prefix


## 題目

太麻煩，略

## 解題

因為如果能夠跳一定要跳，所以反過來就可以

### SLOVE

```cpp
class Solution {
public:
    int maximumEnergy(vector<int>& energy, int k) {
        int n = energy.size();

        vector<int>dp(n, 0);

        int ans = INT_MIN;
        for(int i = n-1; i >= 0; i--){
            if(i + k >= n){
                dp[i] = energy[i];
            }
            else{
                dp[i] = dp[i+k] + energy[i];
            }
        }

        for(auto i : dp)
            ans = max(ans, i);

        return ans;
    }
};
```
