# 1353. Maximum Number of Events That Can Be Attended

>[!note]
> - `Medium`
> - `Heap` `greedy`


## 題目

給一個二維陣列

內部 [`start`, `end`] 為活動的開始~結束時間

- 一個活動可以在開始至結束前選擇**一天**參加 ，不得逾一天內參加兩個以上活動

求最多可以參加的活動數

## 解題

用 `Heap`

首先，先排序 -> 由最先開始的天數來排定 (確保第一天就可以參加)

- 因為可以在期間內選`1`天 -> 先選先截止的(`end` 小的)

結束

>[!important]
> priority_queue 內的排序應該使用 <&type, vector<&type>, greater<&type>> ，`greater` 為由小到大(與vector相反)

### CODE

```cpp
class Solution {
public:
    int maxEvents(vector<vector<int>>& events) {
        sort(events.begin(), events.end(), [](vector<int>a, vector<int>b){
            return a[0] == b[0] ? a[1] < b[1] : a[0] < b[0];
        });

        int ans = 0;
        priority_queue<int, vector<int>, greater<int>>queue;

        int day = 0, lastDay = 0, index = 0;

        for(const auto & e : events) lastDay = max(e[1], lastDay);

        while(day <= lastDay){

            // 符合天數的都加入
            while(index < events.size() && events[index][0] == day){
                queue.push(events[index][1]);
                index++;
            }
            // 過期的略過
            while(!queue.empty() && queue.top() < day){
                queue.pop();
            }
            // 將最快截止的選定
            if(!queue.empty()){
                ans++;
                queue.pop();
            }
            day++;
        }

        return ans;
    }


};
```
