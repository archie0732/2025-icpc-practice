# 1751. Maximum Number of Events That Can Be Attended II

>[!note]
>- `Hard`
>- `DP` `binary search`


## 題目

給一個陣列`events`與一個數字`k`

- `events` => `[[start, end, value]]`
- `k` 可以 參加的活動數量上限

在選定一個活動後，**該期間內不得選擇其他活動**

>求最大`value`

## 解題

用 `dp` 反推:

- 當我選擇一個活動後，我可以在`開始日期之前` 在選擇在`該開始日期前截止的活動`
- 然後後面的活動只要再去找前面已經計算過的就可以了

- ex
```md
[1, 2, 1], [3, 4, 2], [4, 7, 6]

- 以index = 0 來看 -> 最佳選擇就是選自己(沒有其他活動可選)

- 以index = 1 來看 -> 1. 選自己 + 挑選前面的(index = 0) 2. 跳過自己(只選index = 0) ，1. > 2. 所以 填入 2.

- 以 index = 2 來看 -> 1. 選自己 + 挑選前面的(index = 1 不行。只有 index = 0) 2. 跳過自己 (選 index = 1 的選擇，即 index[0+1]) 2. > 1. 。填入 2

```

再來是優化 `找到前面的有效活動`

用 `binary search`

### CODE(CPP)

```cpp

class Solution {
public:
    int maxValue(vector<vector<int>>& events, int k) {
        int n = events.size();
        vector<vector<int>>dp (n+1, vector<int>(k+1, 0));
        
        sort(events.begin(), events.end(), [](vector<int>a, vector<int>b){
            return a[1] < b[1]; // 由結束日先的決定
        });

        for(int i = 0; i < n; i++){
            int prev = -1, left = 0, right = events.size();

            while(left <= right){
                int mid = (left + right ) / 2;

                if(events[mid][1] < events[i][0]){
                    prev = mid;
                    left = mid + 1;
                }
                else
                    right = mid - 1;
            }

            for(int j = 1; j <= k ; j++){

                dp[i+1][j] = max(dp[i][j], dp[prev + 1][j - 1] + events[i][2]);
            }
        }

        return dp[n][k];
    }
};
```
