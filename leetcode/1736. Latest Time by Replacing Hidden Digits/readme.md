# 1736. Latest Time by Replacing Hidden Digits


>[!note]
>- Easy
>- string


## question

給一個時間 `00:00` ~ `23:59` 其中包含 `?`，求修改 `?` 後最大的時間

## 解題

略

### CPP CODE

```cpp
class Solution {
public:
    string maximumTime(string time) {
        
        // 1
        if(time[0] == '?' && time[1] != '?' && time[1] - '0' >= 4) time[0] = '1';
        else if(time[0] == '?') time[0] = '2';

        // 2
        if(time[1] == '?' && (time[0] == '1' || time[0] == '0')) time[1] = '9';
        if(time[1] == '?' && time[0] == '2') time[1] = '3';

        // 3
        if(time[3] == '?') time[3] = '5';
        
        // 4
        if(time[4] == '?') time[4] = '9';

        return time;
    }
};
```
