class Solution {
public:
    bool isZeroArray(vector<int>& nums, vector<vector<int>>& queries) {
        int n = nums.size();
        vector<int>dp(n, 0);

        for(const auto& it : queries){
            int l = it[0], r = it[1];

            for(int i = l; i <= r; i++){
                dp[i]++;
            }
        }

        for(int i = 0; i < n; i++){
            if(nums[i] > dp[i])return false;
        }

        return true;
    }
};