# 2785. Sort Vowels in a String


>[!note]
> - Medium
> - queue 


## 題目

給一個單字，要求將單字的母音挑出並更具`ASCII`排序並放回

## 解題

>[!tip]
>### priority_queue
> 預設由大到小，用 priority_queue<T, vector<T>, greater<T>> 為由大到小


### CPP CODE


```cpp
class Solution {
public:
    string sortVowels(string s) {
        priority_queue<char,vector<char>, greater<char>> v;
        int n = s.size();

        for(int i = 0; i < n ; i++){
            if(isVowel(tolower(s[i]))){
                v.push(s[i]);
                s[i] = '8';
            }
        }

        

        for(int i = 0; i < n; i++){
            if(s[i] == '8'){
                s[i] = v.top();
                v.pop();
            }
        }

        return s;
    }

private:
    bool isVowel(char a){
        
        if(a == 'a' || a == 'e' || a == 'i' || a == 'o' || a == 'u')
            return true;
        return false;
    }
};
```