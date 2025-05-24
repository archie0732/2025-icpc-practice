# 2942. Find Words Containing Character

>[!note]
>- `easy`
>- `string` `method: find`


## 題目

給一個包含字串的陣列與一個字元

回傳那些位置有包含



### 解題

>[!note]
> ## find method
> "string".find("char") 即在"string"中尋找"char"，沒找到回傳 -1，有回傳位置


#### code (CPP)


```cpp
class Solution {
public:
    vector<int> findWordsContaining(vector<string>& words, char x) {
        vector<int> ans;
        int n = words.size();


        for(int i = 0; i < n; i++){
            if(words[i].find(x) != -1){
                ans.push_back(i);
            }
        }

        return ans;
    }
};
```