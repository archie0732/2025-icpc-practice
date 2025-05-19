# 3024. Type of Triangle


>[!note]
>- easy
>- 基本常識


## 題意

給一個三角形三邊長

- 如果為等腰回傳`isosceles`
- 為正三角回傳`equilateral`
- 三邊不同長回傳`scalene`

**如果不能組成三角形**回傳`none`

## 解題

`sort` -> 結束!

設排序後結果 a 、 b 、 c

正: a == c
等腰: a == b || b == c

>[!note]
> # 注意
> 如果兩個短邊相加不大於第三邊，就不能組成三角形!


### code (CPP)

```cpp
class Solution {
public:
    string triangleType(vector<int>& nums) {
        sort(nums.begin(), nums.end());

        int a = nums[0], b = nums[1], c = nums[2];

        if(a + b <= c)return "none";
        else if(a == c)return "equilateral";
        else if(a == b || c == b)return "isosceles";
        else return "scalene";


        return "scalene";
    }
};
```

