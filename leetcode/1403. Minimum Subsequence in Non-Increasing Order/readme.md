# 1403. Minimum Subsequence in Non-Increasing Order

>[!note]
>- Easy
>- greedy sorting


## 題目

給一個陣列，求取出幾個數字後的總合能大於被取出數字後的原陣列

- 如果有多個解答，求子陣列越短且排序為遞減(只有唯一解)

## 解題

sort => sum(array) => for by end => sum - array[i] < array[i] + array[i+1].... array[n] => ans


### CPP CODE

```cpp
class Solution {
public:
    vector<int> minSubsequence(vector<int>& nums) {
        sort(nums.begin(), nums.end());

        int sum = accumulate(nums.begin(), nums.end(), 0);
        int n = nums.size();

        int temp = 0;
        vector<int> ans;
        for(int i = n - 1; i >= 0 && temp <= sum; i--){
            temp += nums[i];
            sum -= nums[i];
            ans.push_back(nums[i]);
        }

        return ans;
    }
};
```
