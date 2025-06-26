# 2311. Longest Binary Subsequence Less Than or Equal to K

>[!note]
>- `Medium`
>- `dp` `binary`

## 題目

給一個陣列(**binary**)與一個數字`k`(**decimal**)，求小於等於`k`的子陣列的最長長度

>[!important]
>- 可以刪除陣列中的數字去組裝，但不能更改順序
>- 子陣列可以有前導`0`


## 解題

因為要小於`k`，我們可以有一個想法

- 陣列由右邊往左，遇到`1`時去算是否小於等於`k`
- 遇到`0`就直接增加長度(因為可以包含**前導0**)

>[!tip]
> **!!注意!! 在計算比較大小的`二進位轉十進位`時位數過長導致算不出來**<br>
> 我們可以先去計算`k`的`二進位長度`直接去比較子陣列的長度

### CODE(CPP)

```cpp
class Solution {
public:
    int longestSubsequence(string s, int k) {
        int ans = 0, cur = 0, n = s.size();

        int maxLength = 0, copy = k;

        while(copy > 0){
            copy /= 2;
            maxLength++;
        }
        
        for(int i = 0; i < n; i++){
            char c = s[n-1-i];

            //cout<<"c: "<<c<<endl;

            if(c == '1' && maxLength >= i && isLessK(cur, i, k)){
                ans++;
            }
            else if(c == '0')
                ans++;
            //cout<<cur<<endl;
        }
        return ans;
    }

private:
    bool isLessK(int& cur, int index, int k){
        cur += (1 << index);
        return cur <= k;
    }
};
```