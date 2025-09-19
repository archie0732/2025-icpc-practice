# 1827. Minimum Operations to Make the Array Increasing


>[!note]
>- Easy
>- greedy


## 題目

給一個陣列可以對其做以下操作

- 對陣列裡的某個元素`+1`

求最少要操作幾次才可以讓陣列變成**嚴格遞增**


### CPP CODE

```
class Solution {
public:
    int minOperations(vector<int>& nums) {
        int n = nums.size();

        if(n == 0 || n == 1) return 0;


        int last = 0, ans = 0;

        for(int& i : nums){
            if(i <= last){
                int t = last - i + 1;
                ans += t;
                i += t;
            }

            last = i;

        }

        return ans;
    }
};
```
