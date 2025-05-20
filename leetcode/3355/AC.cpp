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