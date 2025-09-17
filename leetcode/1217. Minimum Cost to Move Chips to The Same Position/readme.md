1217. Minimum Cost to Move Chips to The Same Position


>[!note]
>- Medium
>- greedy


## 題目

給一個陣列 `position` , `position[i]` 表示有一個硬幣在該位置上

- 將一個硬幣往左或往右移**1**格需要花費**1**
- 將一個硬幣往左或往右移**2**格需要花費**2**

求將所有硬幣都集中到一個位置上的`最小花費`

## 解題

首先在`基數位置的硬幣`要移動到`某個偶數位置`需要花費`1`點

```
2 * n + 1 # 先移到偶數位置的左或右(每兩個一動, 不花點數), 再移動到基數位置上
```

反之偶數位置也一樣


那麼我們就去看看基數跟偶數的樹兩誰較少就要移到另一個位置上了


### CPP CODE

![](https://wallup.net/wp-content/uploads/2017/10/24/477835-Fate-Stay_Night-anime_girls-Tohsaka_Rin-748x421.jpg)

```cpp
class Solution {
public:
    int minCostToMoveChips(vector<int>& position) {
        int odd = 0, even = 0;

        for(int i : position){
            if(i % 2) odd++;
            else even++;
        }

        return odd > even ? even : odd;
    }
};
```
