# 2419. Longest Subarray With Maximum Bitwise AND

>[!note]
>- `Medium`


## question

找最大的數字的子陣列最常長度

## solution

```cpp

class Solution {
public:
    int longestSubarray(vector<int>& nums) {
        int mx = 0;

        for(int num : nums)
            mx = max(num, mx);

        int ans = 0, cur = 0;

        for(int num : nums){
            if(mx == num)
                cur++;
            else{
                ans = max(ans, cur);
                cur = 0;
            }
        }


        return max(cur, ans);
    }
};
```
