# 166. Fraction to Recurring Decimal

>[!note]
>- Medium
>- string & math


## 題目

給一個**除數**與被**除數**，求結果(含小數點與正負號)，如果為無限循環小數，在循環處用`(``)`包起

## 解題

>[!tip]
>- 考慮以下情況
>1. 被除數 < 0
>2. 除數 < 0
>3. 除數 && 被除數 < 0
>4. 被除數 == INT_MIN (如果直接轉正成 `long long` 會暴)


先判斷正負，然後轉正數用正數來計算(程式語言無法處理負數取餘)

```cpp
(n < 0) ^ (d < 0) // 其中一個為true才是true 兩個相同為false
```

然後用 `llabs` 來轉正

分為**整數**部分與**小數點後**部分

- 整數: 正常算
- 小數 跟正常除法公式相同 => 取餘 * 10 在除

### CODE


```cpp
#define ll long long 
class Solution {
public:
    string fractionToDecimal(int numerator, int denominator) {
        if(numerator == 0)return "0";

        string ans = (numerator < 0) ^ (denominator < 0) ? "-" : "";

        ll n = llabs((long long) numerator);
        ll d = llabs((long long) denominator);

        ans += to_string(n/d);
        ll remainder = n%d;

        if(!remainder)return ans;
        ans += ".";

        unordered_map<ll, int>mp;
        while(remainder > 0){
            if(mp.count(remainder)){
                ans.insert(mp[remainder], "(");
                ans += ")";
                break;
            }

            mp[remainder] = ans.size();
            remainder *= 10;
            ans += to_string(remainder / d);
            remainder %= d;
        } 
        return ans; 
    }
};
```
