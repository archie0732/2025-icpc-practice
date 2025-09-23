# 165. Compare Version Numbers

>[!note]
>- Meduim
>- string


## 題目

給兩個版本號，比大小

- version1 > version2 => 1
- version1 < version2 => -1
- version1 == version2 => 0

>[!important]
>- 1.10 > 1.2, 因為 10 \> 2
>- 1.001 == 1.1 因為 1 == 1


## 解題

根據 `.`  把數字分開並比大小(`.`後沒有數字就是0)

### CPP CODE

```cpp
class Solution {
public:
    int compareVersion(string version1, string version2) {
        int n1 = version1.size(), n2 = version2.size();

        for(int i = 0, j = 0; i < n1 || j <= n2; i++, j++){
            int v1 = 0, v2 = 0;

            while(i < n1 && version1[i] != '.'){
                v1 = v1*10 + (version1[i++] - '0');
            }
            while(j < n2 && version2[j] != '.'){
                v2 = v2*10 + (version2[j++] - '0');
            }

            cout<< v1 << ", " << v2 << endl;
            
            // cmp
            if(v1 < v2)return -1;
            if(v1 > v2)return 1;
        }
        return 0;
    }
};
```
