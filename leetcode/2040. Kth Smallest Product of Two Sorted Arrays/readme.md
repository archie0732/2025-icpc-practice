# 2040. Kth Smallest Product of Two Sorted Arrays

>[!note]
>- `Hard`
>- `Bin search`

## 題目

給兩個陣列`nums1` `nums2`，將 兩陣列相乘後排序，找到第 `k`個

>[!warn]
> 不能照題意走，會`TLE`

### 解題

- binary search goal (`x`)

`x`用來找到一個值 是**乘法陣列中第n個**


1. 如果 `x` <  `k` => 移向 `r`
2. 如果 `x` >= `k` => 移向 `l`

> 即，符合`k >= x` 中的最小值

- 要找到符合`第一項`，可以將 `nums1` 遍歷的值(`it`)去找到 `nums2` 中有幾個可以符合

- x 為 目標值 ， result 為我們目前算出來的

>1. it == 0   ， 如果我們要找的 `x >= 0` 那麼就直接回傳 `nums2.length`
>2. result < x， 如果 `it > 0` 則代表 `nums2[mid]` 找到的值過小 ， `l = mid + 1` 。 如果 `it < 0` 那麼代表 `r` 過大 =>(往負數去找) `r = mid`
>3. result > x， 如果 `it > 0` 則代表 `nums2[mid]` 找到的值過大 ， `r = mid` 。 如果 `it < 0 ` 那麼代表 |`l`| 過大 `l = mid + 1`

用簡單數學論證

```md
result = it * nums2[mid]

條件: result <= x

- it * nums2[mid] <= x

### 移項

- nums2[mid] <= x / it;

又分以下情況

- it == 0 && x > 0， 因為 0 * nums2[mid] = 0 所以會有 nums2.size() 個符合
- it > 0 ， 正常移項 nums2[mid] <= x / it
- it < 0 ， 需要變號 nums2[mid] >= x / it
```

>[!note]
>注意 開頭的 `binary search` 必須為 `l + (r - l)/2`!!!!<br>
> 因為 |l| == |r| == 1e10 (long long 最大值) `r + l` 會 `overflow` ，所以先`r - l `避免溢出


### CODE(CPP)

![](https://image.fonwall.ru/o/wk/tohsaka-rin-fate-stay-night-wallpaper-shirou-emiya-other.jpeg)

```cpp
#define ll long long
class Solution {
public:
    long long kthSmallestProduct(vector<int>& nums1, vector<int>& nums2, long long k) {
        ll l = -1e10, r = 1e10;

        while(l < r){
            ll mid = l + (r - l) / 2;

            if(countProducts(nums1, nums2, mid) >= k){
                r = mid;
            }
            else{
                l = mid + 1;
            }
        }
        return r;
    }

private:
    ll countProducts(vector<int>& nums1, vector<int>& nums2, ll x){
        ll count = 0;

        for(int it : nums1){
            
            if(it == 0){
                if(x >= 0) count += nums2.size();
                continue;
            }

            // bin search
            int l = 0, r = nums2.size();

            // upper_boound 找值
            while(l < r){
                int mid = (l + r) / 2;
                ll result = 1LL * it * nums2[mid];

                if(result <= x){
                    if(it > 0) l = mid + 1;
                    else r = mid;
                }
                else{
                    if(it > 0)r = mid;
                    else l = mid + 1;
                }
            }
            count += it > 0 ? l : (nums2.size() - l);
        }
        return count;
    }
};
```
