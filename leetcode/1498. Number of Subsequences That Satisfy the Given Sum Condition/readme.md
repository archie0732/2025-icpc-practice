# 1498. Number of Subsequences That Satisfy the Given Sum Condition

>[!note]
>- `Medium`
>- `bin search` `two point`

## 題目

給一個陣列與一個數字`target`


子陣列規範
- 子陣列中最大(`sub[sub.size() - 1]`) + 最小(`sub[0]`) <= `target`

回傳子陣列**數量**

>[!tip]
> 因為數字很大`leetcode`要求需要將答案`mod` `1e9 + 7`!


## 解題

>[!important]
> ## 子陣列組合
> 由`i`起頭，包含`nums[i] ~ nums[r]`之間有 `2^(r - i)` 個!

```md
nums = [2, 3, 4, 5]

# 除了 2(index = 0) 以外，其他的都有選擇`要加入` 或 `不加入` 兩種方法

[2]
[2,3]
[2,4]
[2,5]
[2,3,4]
[2,3,5]
[2,4,5]
[2,3,4,5]

```

>[!tip]
> 記得所有計算都要`mod`

- 二元搜目標

>指定起點，要找到他可以填入的最大右極值


```cpp
class Solution {
public:
    int numSubseq(vector<int>& nums, int target) {
        sort(nums.begin(), nums.end());
        int n = nums.size();
        int MOD = 1e9 + 7;

        // 預算 2^i mod MOD
        vector<int> pow2(n, 1);
        for (int i = 1; i < n; i++) {
            pow2[i] = (pow2[i - 1] * 2) % MOD;
        }

        int ans = 0;

        for (int i = 0; i < n; i++) {
            int r = upperBound(nums, target - nums[i], i); // 找最大合法 right
            if (r >= i) {
                ans = (ans + pow2[r - i]) % MOD;
            }
        }

        return ans;
    }

private:
    // 找最大 right，使得 nums[i] + nums[right] <= target
    int upperBound(const vector<int>& nums, int goal, int i) {
        int l = i, r = nums.size() - 1, ans = i - 1;
        while (l <= r) {
            int mid = (l + r) / 2;
            if (nums[mid] <= goal) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        return ans;
    }
};
```