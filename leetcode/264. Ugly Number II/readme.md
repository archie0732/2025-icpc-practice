# 264. Ugly Number II


>[!note]
>- `Medium`
>- `Heap` `Math`


## 題目 

只能由 `2` `3` `5` 組成 求 第 `n` 個

## 解題

- 用 `heap` 來確定大小 -> 由小到大
- 依序 乘 `2` `3` `5`

>[!important]
> 注意要用 `set` `map` 來確定不會重複<br>
> 要用 `long long`


### CODE

```cpp
#define ll long long
class Solution {
public:
    int nthUglyNumber(int n) {
        priority_queue<ll, vector<ll>, greater<ll>> heap;

        heap.push(1);

        int index = 1;

        set<ll> set;

        ll arr[3] = {2, 3, 5};

        while(index < n){
            
            ll top = heap.top();

            for(ll a : arr){
                ll temp = a * top;
                if(set.count(temp)) continue;

                set.insert(temp);
                heap.push(temp);
            }
            

            heap.pop();
            index++;
        }

        return heap.top();
    }
};
```
