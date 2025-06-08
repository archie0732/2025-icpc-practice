# 3362. Zero Array Transformation III

>[!note]
>- `medium`
>- `prefix` `heap (priority_queue)`

## 題目

- 本題屬於`leetcode 3355`的延伸(進階)，請先確保先學會該題再來


繼 `3355` 改求只要幾個 `queries` 就可以滿足(刪掉無用的`query`區間)


### 解題

繼上次的 `prefix_sum` 後 

- 提供想法

如果另 `i = 0` 且 `nums[0] = 1` 且有以下 `queries` 應該優先選哪個

```md
1. `[0, 1]`
2. `[0, 2]`
3. `[0, 3]`
4. `[1, 3]`
```

`4.` 不符合， 而 `1.` `2.` `3.` 中 選擇**範圍**更大的 `3.` 更有性價比


#### 重點 

而在 `nums = [0,0,1,1,0]` 與 `queries = [[3,4],[0,2],[2,3]]` 中 **比起選擇 `[0, 2]` 選擇 `[2, 3]` 能直接滿足要求**

由此可以得到以下結論

>[!note]
>query 選擇
>1. 起頭位置有需要減的 (`queries[] = [l, r]` ， `l` = `i` &&  `nums[i] > 0`)
>2. 在符合第一點的情況下選擇 **包含範圍更大的**


所以使用 `priority_queue`

>[!note]
> ## priority_queue
> - 性質: 由大到小的 `queue`
> - 在這裡的功用:  在符合先上面 `1.` 的情況下選擇 範圍更大的 (`push(queries[][1])`)

然後用 `prefix_sum` 紀錄 `i` ~ `queries[][1]` 的 -> 如果後面有值要加 **就直接加在 prefix_sum 就好**

然後 `delta` 可以記錄 剛剛找到的 `query` 範圍的尾巴 (告訴 `prefix` 要 `-1` )



最後要 `sort` `queries` 但由於 `queries` 是**二維陣列**且我們有自己要排序的規定

**按造`l`範圍由小到大，且在`l`相同時先排`r`較大的(因為範圍較大)**


>[!note]
> # lambda
> 在 function 裡定義 function


```cpp

v = [1, 2, 3, 4];

sort(v.begin(), v.end(), [](int a, int b){
  return a > b;
}) // 在 sort 這個 function 裡面定義一個比較邏輯 (要求 **由大到小排列**)


// ans = 4, 3, 2, 1
```

如果覺得太過抽象，也可以


```cpp

bool cmp(int a, int b){return a > b} // 由大到小

v = [1, 2, 3, 4]
sort(v.begin(), v.end(), cmp);

// 結果與上面一致
```


而在這裡

```cpp

sort(queries.begin(), queries.end(), [](const vector<int>&a, const vector<int>&b){      
            return a[0] == b[0] ? a[1] > b[1] : a[0] < b[0];
        });

```

是為了要先排 `queries` 的 `l` (由小到大)

如果 `l` 一樣大 -> **先排`r`較大的**



### code (CPP)


恭喜你看到這! 辛苦了!!

![](../../image/3362.gif)



```cpp

class Solution {
public:

    int maxRemoval(vector<int>& nums, vector<vector<int>>& queries) {
        int flag = 0;
        int n = nums.size();

        
        // const vector<int>& 一定要有，實測不加會導致 TLE
        sort(queries.begin(), queries.end(), [](const vector<int>&a, const vector<int>&b){      
            return a[0] == b[0] ? a[1] > b[1] : a[0] < b[0];
        });

        // 由大到小的 queue
        priority_queue<int>queue;

        int j = 0, prefixSum = 0;

        vector<int>delta(n + 1, 0);

        /*
            - nums[i]: 在 i 要減幾次
            - i: queries[] 的左極與又極

            想法:
                同一個符合 i 的 query 
                選擇 範圍更大的 quey 是不是更好 (因為存在可能 i+1, i+2, i+... 些需要減)
                
                - 舉裡: 比起 i ~ i + 1 選擇 i ~ i + 2 會更好
        */
        for(int i = 0; i < n; i++){

            // 1. 減去 prefix 的尾數
            prefixSum += delta[i];

            // 先推入符合i的值
            // 標準 只要 query 的範圍 == i 都可以
            while(j < queries.size() && queries[j][0] == i){
                queue.push(queries[j][1]);
                j++;
            }

            // queue.top() -> 表示目前最大的 距離 (i ~ queue.top())
            // 這裡先不管 只要超過 i 的值就累加在 operation 上 -> 前綴和(prefix sum)
            while(prefixSum < nums[i] && !queue.empty() && queue.top() >= i){
                prefixSum++;
                // 因為 範圍為 i ~ queue.top() ，不過這裡是超過 i 就+1所以要還回 queue.top() 後面多加的 ，減去部分在 1.
                delta[queue.top() + 1]--;
                queue.pop();
            }

            //拚盡全力也無法滿足 -> 無解
            if(prefixSum < nums[i])return -1;
        }

        // 剩下未使用的陣列
        return queue.size();
    }
};
```