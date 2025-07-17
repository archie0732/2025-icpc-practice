# 3202. Find the Maximum Length of Valid Subsequence II


```cpp
class Solution {
public:
    int maximumLength(vector<int>& nums, int k) {
        int ans = 2, n = nums.size();

        for(int i = 0; i < k; i++){
            vector<int> dp(k, 0);
            for(int j = 0; j < n; j++){
                int mod = nums[j] % k; // (nums[j] + prev) % k == i || prev =  i % k - mod 
                int prev = (i - mod + k) % k;
                dp[mod] = dp[prev] + 1;
            }
            
            for(int it : dp){
                ans = max(ans, it);
            }
        }

        return ans;
    }
};
```
