# 2200. Find All K-Distant Indices in an Array

>[!note]
>- `Easy`
>- `array`

## 題目

太簡單了，略

## 解題

- 照著題目的意思去寫就行

- 下面有進階版寫法

### CODE(CPP)


```cpp
class Solution {
public:
    vector<int> findKDistantIndices(vector<int>& nums, int key, int k) {
        vector<int>ans, dp;
        int n = nums.size();

        for(int i = 0; i <n; i++)
            if(nums[i] == key)
                dp.push_back(i);

        for(int i = 0; i < n; i++){
            for(int j = 0; j < dp.size(); j++){
                if(abs(dp[j] - i) <= k){
                    ans.push_back(i);
                    break;
                }
            }
        }
        return ans;
    }
};
```

- 進階 O(n)

>由左至右->符合就去找範圍，固定新的`l` = 舊的`r`(r+1)，這樣就不會重複加入點


```md
# 示意圖

X X X i X X X
  <---O--->

# when i == key(O), l = i-k, r = i+k
```

```cpp
class Solution {
public:
    vector<int> findKDistantIndices(vector<int>& nums, int key, int k) {
        int n = nums.size();
        vector<int> ans;
        int r = 0, l = 0;

        for(int i = 0; i < n; i++){
            if(nums[i] == key){
              // 如果舊的比較大(已經加入，就用舊的)不然就使用新的範圍(i - k)
                l = max(r, i - k);
                r = min(n - 1, i + k) + 1;

                for(; l < r; l++)
                    ans.push_back(l);
            }
        }

        return ans;
    }
};
```