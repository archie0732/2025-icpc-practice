# 2008. Maximum Earnings From Taxi


>[!note]
>- `Medium`
>- `DP` `Hash map`

## 題目

給一個陣列 `rides` 與 `n` 

- `rides` 裡面的`子陣列`代表 每個顧客的 [起點, 目的地, 小費]
- `n` 總共可以移動的站 0 ~ n
- 收費: `目的地 - 起點 + 小費`

>[!important]
> 無法回頭，到目的地後可以立馬接新客人

> 求最多可以收多少`$`


## 解題

用 `dp` (0 ~ n)

- 看 於第`i`個地方時可以獲得最多的`$`是多少

1. dp[i] = dp[i-1]: 上一個站的錢比在這站接客更多錢
2. dp[i] = rides[][2]+ (rides[][1] - rides[][0]) + dp[rides[0]]: 選擇接客，並且去找該客人的起點可以賺多少錢

>[!tip]
>如果照上面的方法找客人，即 for 循環找人會超時。可以用 map<int, vector<vector<int>>> 快速定位 map[i] => 有哪接客人的目的地在 i

### CODE(CPP)

```cpp
#define ll long long
#define pill pair<int, int>
class Solution {
public:
    long long maxTaxiEarnings(int n, vector<vector<int>>& rides) {
        sort(rides.begin(), rides.end(), [](vector<int>a, vector<int>b){
            return a[1] < b[1];
        });

        vector<ll> dp(n+1, 0);
        unordered_map<int,vector<vector<int>>>map;
        for(const auto a : rides){
            map[a[1]].push_back(a);
        }

        for(int i = 1; i <= n; i++){

            for(const auto  j : map[i]){
                dp[i] = max(dp[i], j[2] + (j[1] - j[0]) + dp[j[0]]); // 小費 + 路程費 + dp[j 的前面賺最多的 $]
            }
            dp[i] = max(dp[i-1], dp[i]);
        }

        return dp[n];
    }
};
```

