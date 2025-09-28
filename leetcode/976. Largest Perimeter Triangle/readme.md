# 976. Largest Perimeter Triangle

>[!note]
>- Easy
>- greedy sort


## 題目

給一個陣列 => 用裡面的三個長度組成最大的三角形
- 回傳三角形`週長`

> 如果沒有符合的，回傳`0`

## 解題

>[!important]
>三角形: 兩短邊相加 `>` 長邊


- `sort` 後去找有沒有符合的(由長往短找)

### CODE

```cpp
class Solution {
public:
    int largestPerimeter(vector<int>& nums) {
        sort(nums.begin(), nums.end());

        for(int i = nums.size()-1; i > 1; i--){
            if(nums[i] < nums[i-1] + nums[i-2])
                return nums[i] + nums[i-1] + nums[i-2];
        }

        return 0;
    }
};
```
