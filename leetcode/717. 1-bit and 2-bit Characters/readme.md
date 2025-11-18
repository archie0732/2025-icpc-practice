# 717. 1-bit and 2-bit Characters

>[!note]
> Easy array

[717. 1-bit and 2-bit Characters](https://leetcode.com/problems/1-bit-and-2-bit-characters/?envType=daily-question&envId=2025-11-18)

## 解題

可以看出一個規律

> `1` 開頭一定要兩個一組 因為 `10` `11`

如果 `0` 開頭的只能自己一組

>[!tip]
>所以就遍歷整個陣列，開頭遇到`1`就跳兩格，`0`就跳一格

最後觀看最右邊的`0`在不在（有沒有被`1`吃掉）

### CODE

```py
class Solution:
    def isOneBitCharacter(self, bits: List[int]) -> bool:
        
        while len(bits) > 1:
            if bits[0] == 1:
                bits.pop(0)
                bits.pop(0)
            elif bits[0] == 0:
                bits.pop(0)
            #print(len(bits))
        
        return True if len(bits) == 1 else False

```
