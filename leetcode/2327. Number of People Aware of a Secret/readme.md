# 2327. Number of People Aware of a Secret


>[!note]
> - `Medium`

## 題目

有個 秘密，開始只有一個人知道

- 過了 `delay` 天 知道這秘密的人會分享給一個人
- 過了 `forget`天 知道秘密的人會忘記

求過了n天有多少人知道秘密

## 解題

- dp 紀錄這一天有多少人分享祕密(在今天知道秘密)


1. 過了 `delay`天，會有 `dp[day - delay]` 人 **分享秘密**
2. 過了 `forget`天，在 `dp[day - forget]` 以前知道秘密的人會忘記

所以只要求 `dp[n - forget + 1] ~ dp[n]` 的總和就是在第`n`還知道秘密的人數


### CPP CODE


```cpp
class Solution {
public:
    int peopleAwareOfSecret(int n, int delay, int forget) {
        vector<int>dp(n+1, 0);
        dp[1] = 1;

        int share = 0;
        for(int i = 2; i <= n ; i++){
            if(i > delay){
                share = (share + dp[i - delay] + MOD) % MOD;
            }

            if(i > forget){
                share = (share - dp[i - forget] + MOD) % MOD;
            }

            dp[i] = share;
        }

        int ans = 0;

        for(int i = n - forget + 1; i <= n; i++){
            ans = (ans + dp[i]) % MOD;
        }

        return ans;
    }

private:
    long long MOD = 1000000007;
};
```