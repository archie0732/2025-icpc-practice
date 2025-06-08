# 3356. Zero Array Transformation II

>[!note]
> `medium`
> `prefix sum` `bin search`


## 題目

本題為 `leetcode. 3355` 的延伸 請先做完 `3355` 再來完成這題


接續上題，`query` 除了 `[l, r]` 外， `query` 新增 `query[2] = [l, r, val]` `val` 為可以加的值


這題問: 回傳一個 **最小** `k` 為 `queries[0]` ~ `queries[k]` 即可完成 將 `nums` **全部歸零**


### 解題

>[!note]
> 因為題目為 **保留 0 ~ k** 的 `queries`
> - 推斷能使用 `binary search`


- 想法

1. 建立一個確認`function` -> 判斷 **由0切到mid能否完成題目要求**
2. 用 `binary search` 去找答案


>[!note]
>## binary search 判斷標準
>- 可以: 再多砍一點範圍
>- 不可以: 不符合標準，只好擴大範圍



### code (CPP)

![](../../image/lt3356.jpg)

```cpp
class Solution {
public:
    int minZeroArray(vector<int>& nums, vector<vector<int>>& queries) {
        // 題目有說 **找到最小的 queries[0] ~ queries[k]** -> 用 bin_search


        int l = 0, r = queries.size();

        // 先判斷不任何值刪的會不會過 -> 不會過那就 直接 -1
        if(!canWork(nums, queries, r))return -1;

        

        while(l <= r){
            int mid = (r + l)/2;

            bool result = canWork(nums, queries, mid);

            if(result){
                r = mid - 1;
            }
            else {
                l = mid + 1;
            }
        }

        return l;
    }


    bool canWork(vector<int>& nums, vector<vector<int>>& queries, int k){

        int prefixSum = 0, n = nums.size();

        vector<int>delta(n + 1, 0);

        for(int i = 0; i < k; i++){
            delta[queries[i][0]] += queries[i][2];
            delta[queries[i][1] + 1] -= queries[i][2];
        }

        int index = 0;
        for(const auto& it : nums){
            prefixSum+=delta[index];
            index++;

            if(prefixSum < it)return false;
        }

        return true;
    }
};
```