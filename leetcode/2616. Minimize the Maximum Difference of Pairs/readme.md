# 2616. Minimize the Maximum Difference of Pairs


## 題目

在一個排序後的陣列 `nums` 中，找出 `p` 個不重疊的 `pair`（兩兩配對，**不能重複使用元素**），使得這 `p` 組 `pair` 中的差值最大值最小化（也就是找到 p 組差值最小的 pair，然後回傳其中最大的那一個差值）。


### 解題

- 想法 1

用`貪心`去找倆倆一組的最小，然後依序找到`p`組

- 想法 2

用`二分搜尋`去猜第`p`個值是多少 ，又有以下兩種情況

我們先將 二分搜的答案假定為 `mid`

>[!note]
>- 找到的值: 有**超過** `p` 個答案滿足(小於 `mid`) -> 縮小範圍
>- 找到的值: **未能滿足** `p` 個 (找到 符合 `mid` 的個數 **小於** p) -> 擴大範圍


這裡是用**想法二**


>[!note]
> ### 二分搜尋法
> - 有一個左極與右極
> - 當右 >= 左 時未找到答案
> - 符合答案時，答案為 左


### CODE(CPP)

```cpp
class Solution {
public:
    // 是否能滿足 超過 p 個
    bool canThrowTheTest(const vector<int>&nums, int p, int max){
        int count = 0, n = nums.size();

        for(int i = 0; i < n-1; i++){
            if(abs(nums[i] - nums[i+1]) <= max){
                count++;
                i++;
            }
        }

        return  count >= p;
    }
    int minimizeMax(vector<int>& nums, int p) {
        int right = INT_MAX, left = 0;

        sort(nums.begin(), nums.end());

        while(right >= left){
            int mid = (right + left)/2;

            if(canThrowTheTest(nums, p, mid)){
                right = mid - 1;
            }
            else{
                left = mid + 1; 
            }
        }

        return left;
    }
};
```


