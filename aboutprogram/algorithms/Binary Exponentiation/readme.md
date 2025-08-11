# Binary Exponentiation / powMod 快速冪

## 使用時機 與 條件

>[!note]
> ### 時機
> 如果我們要計算 a^b(mod m)，直接乘的話複雜度是 O(b)，b 如果很大（比如 109109），會超慢。
>快速冪可以用 O(log b) 的時間算完。

### 使用方法

$`{base}^{exp}`$

將`exp` 轉成二進位

例如 為 `13`

$`13 = {1101}_2 ​= 8 + 4 + 1 `$

可以轉換為:

$`a^{13} = a^8 × a^4 × a^1`$

**我們只需要把 a 不斷平方，然後挑選二進制裡是 1 的次方數來相乘。**

這樣快速又方便!!

```cpp
long long modPow(long long base, long long exp, long long mod) {
    long long res = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) { // 當前二進位最低位是 1
            res = (res * base) % mod;
        }
        base = (base * base) % mod; // 平方
        exp >>= 1; // 指數右移一位
    }
    return res;
}
```

要配合前綴積相除取餘，請見 **Fermat’s Little Theorem 費馬小定理**

