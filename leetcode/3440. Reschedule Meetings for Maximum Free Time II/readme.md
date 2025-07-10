# 3440. Reschedule Meetings for Maximum Free Time II

>[!note]
>- `Medium`
>- `array`


## 題目 

給兩個陣列 `startTime` `endTime` ，`endTime[i] - startTime[i]` 為第 `i` 個會議的時間，其他時間為休息時間。還有 `eventTime` 表示最大的時間

> 求 在移動一段會議的情況下，`最大的連續休息時間`。注意! 只能平移會議不得縮減或增加時間長度


## 解題

- 實作

1. 先算所有的`空閒時間段`
2. 嘗試將會議移動到那些 `空閒時間段`來獲得最長時間段

移法有兩種

### 1. 後退 or 前進

- 原本
```md
# O: free time, X: meet time, i: cur time

XXXOOOIIIOOXX
```
- 移動後
```md
XXXOOOOOIIIXX # 往後移兩隔
```

### 2. 丟到一段(非前後相接的)空隙

- 原本
```md
XOOXXOOOOIIXX
```
- 移動後
```md
XIIXXOOOOOOXX # 丟到最前面的空隙
```

接下來就實作就行了

## CODE(CPP)

```cpp
class Solution {
public:
    int maxFreeTime(int eventTime, vector<int>& startTime, vector<int>& endTime) {
        int n = startTime.size();

        vector<int> free;

        // list all free time
        free.push_back(startTime[0] - 0);
        for(int i = 1; i < n; i++){
            free.push_back(startTime[i] - endTime[i-1]);
        }
        free.push_back(eventTime - endTime[n-1]);

        int m = free.size();
        vector<int> prevMax(m,0), lastMax(m,0);

        // i 號之前最大的 free time
        prevMax[0] = free[0];
        for(int i = 1; i < m; i++){
            prevMax[i] = max(prevMax[i-1], free[i]);
        }

        // i 號之後最大的 free time
        lastMax[m-1] = free[m-1];
        for(int i = m-2; i >= 0; i--){
            lastMax[i] = max(lastMax[i+1], free[i]);
        }


        //
        int ans = 0;
        for(int i = 0; i < n; i++){

            int meetTime = endTime[i] - startTime[i];
            
            // 在這個開會前面的空閒時間，與後面的空閒時間
            int prevFree = (i == 0) ? startTime[i] - 0 : startTime[i] - endTime[i-1];
            int lastFree = (i == n-1) ? eventTime - endTime[i] : startTime[i+1] - endTime[i];
            
            ans = max(ans, prevFree + lastFree); // 把這段時間一到空閒時間的【最前】或 【最後】，last & pre 就會變連續

            // 如果再 i 會議前找到【一個比他時間長的空隙(不包含接觸的空隙)】
            if(i > 0 && prevMax[i-1] >= meetTime){
                ans = max(meetTime + prevFree + lastFree, ans); // 將他移到那個空隙並去找 前面 + 自己 + 後面 的總長度
            }
            if(i + 2 < m && lastMax[i+2] >= meetTime){
                ans = max(meetTime + prevFree + lastFree, ans); // 同上
            }
        }

        return ans;
        
    }
};
```
