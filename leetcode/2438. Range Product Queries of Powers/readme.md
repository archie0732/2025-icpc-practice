# 2438. Range Product Queries of Powers


>[!note]
>- Medium
>- `prefix` `bit`

## 題目

給一個 `n` 與一個陣列 `queries`

你需要找出 `n` 的最大二進位組成陣列 `pow`，且根據 `queries[i]` 裡面的 [$`{i}_0, {i}_1`$] 在 `pow` 裡面找到符合的範圍的乘積

$` {i}_0 <= pow <= {i}_1 `$

回傳一個滿足 `queries` 裡所有區間的陣列


## 解題

1. 先將`n` 分解並找出 `pow` => 用 二進位 拆

```
9 = 1001 => 8 + 1
```

然後根據 `queries[i]` 的區間來 算就好

- 簡單版 (較耗時，但是也能過)

```cpp

for(auto i : queries){
  for(int j = i[0] ; j < i[1] ; j++){
    answer.push(pow[j]);
  }
}

```

這樣有兩個`for` 且會一直重複運算，所以也可以用 `prefixSum`

但是會涉及 除法取餘數 所以會需要 **小費馬定理** (且見`algorithm`篇)

### CPP CODE 

```cpp

class Solution {
public:
    vector<int> productQueries(int n, vector<vector<int>>& q) {
        vector<int> prefix;

        prefix = makeTable(n);

        vector<int> answer;


        for(auto & i : q){
            
            // prefix[r] / prefix[l] % modulo => prefix[r] % modulo * prefix[l]^-1 % modulo
            // !!!! prefix[l]^-1 % modulo => prefix[l] ^ (modulo - 2) !!!!!

           int ans;
            if (i[0] == 0) {
                ans = prefix[i[1]];
            } 
            else {
                ans = 1LL * prefix[i[1]] * calculateModulo(prefix[i[0] - 1], modulo - 2) % modulo;
            }


            answer.push_back(ans);
        }

        return answer;
    }

private:
    const int modulo = 1000000007;
    
    vector<int> makeTable(int n){
        vector<int>prefix;

        long long dp = 1;
        while(n > 0){
            int lowst = n & -n;
            dp = 1LL * lowst * dp % modulo;
            prefix.push_back(dp);
            n -= lowst;
        }

        return prefix;
    }

    // a ^ (m - 2) % m
    // 快速冪
    int calculateModulo (long long base, long long exp){
        long long res = 1;
        
        while(exp > 0){
            if(exp & 1){
                res = res * base % modulo;
            }

            base = base * base % modulo;

            exp = exp >> 1;
        }

        return res;
    }
};
```

