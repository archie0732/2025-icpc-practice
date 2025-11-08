# 1611. Minimum One Bit Operations to Make Integers Zero

[link](https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/submissions/1823920512/?envType=daily-question&envId=2025-11-08)


## 解題

1. 先簡單一點判斷 `n = 2^k`
2. `10` -> `11` -> `01` -> `00` (3次)
3. `100` -> `101` -> `111` -> `110` -> `010` (`010` 即為剛剛的 10的變法)

可歸納出 `2^k` 要變成 `0`，首先 `2^(k-1)` 要先變成 1 而在那之前的都要是0

>所以總共會進行 `k = 2 * (k-1) + 1` 次 (因為 k-1次為 0 變 1，然後會再需要 1 變回 0。 + 1 為第 k 項由 1 變 0)

4. 最後 如果不是 `2^k` 類型 則取後一位(x = `n ^(1 << k)`) 相減 (因為不用做 0 變 1的動作，所以會少 2^x 次)

> 1010 => 1000 - 0010
 



### CODE

```cpp
class Solution:
    def minimumOneBitOperations(self, n: int) -> int:
        if n == 0:
            return 0
        k = len(bin(n)) - 3

        def f(k) -> int: # f(2^k) = 2 * (f(2^(n-1)) + 1
            if k == 0:
                return 1
            return 2 * f(k-1) + 1

        # 由遞迴反推 cost = 2^k-1 
        
        cost = f(k)

        n = n^(1 << (k))

        return cost-self.minimumOneBitOperations(n)
```
