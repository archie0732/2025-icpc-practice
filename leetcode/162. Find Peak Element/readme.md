# 162. Find Peak Element

>[!note]
>- `bin search` 
>- `Medium`

## Question

too easy, skip

### solve

1. `size == 0` or `size == 1`
2. first number of array or last number of array is peak
3. use bin search

### CODE(CPP)

```cpp
class Solution {
public:
    int findPeakElement(vector<int>& nums) {
        int left = 0, right = nums.size(), n = nums.size();

        // size == 1 or size == 0
        if(n == 1 || n == 0)
            return 0;
 
        // peak in first number of array
        if(nums[0] > nums[1])
            return 0;
        // peak in last number of array
        if(nums[n-1] > nums[n-2])
            return n-1;
        
        // bin search 
        // warn careful range!!
        while(left <= right){
            int mid = (left + right)/2;

            if(mid > 0 && mid+1 < n && nums[mid] > nums[mid+1] && nums[mid] > nums[mid - 1])
                return mid;
            else if(mid+1 < n && nums[mid] < nums[mid+1])
                left = mid + 1;
            else if(mid > 0 && nums[mid] < nums[mid - 1])
                right = mid - 1;
        }

        return left;
    }
};
```

arch1e's github @ 2025/06/22