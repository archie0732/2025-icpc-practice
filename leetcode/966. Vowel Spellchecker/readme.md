# 966. Vowel Spellchecker

>[!note]
>- Medium
>- hashmap



## 題目

給一個字典與一個單字集

- 如果與字典一模一樣: 返回自己
- 如果出現大小寫問題: 返回字典中找到的第一個一樣的單字(僅改變大小寫)
- 如果出現更改母音後語字典相同: 返回返回字典中找到的第一個一樣的單字
- 其他情況: 返回""


>[!important]
>`2` `3` 可以同時滿足


## 解題

因為自滿足第一個調整後相同的字，所以將前面的字變小寫且將母音變為特殊符號，只要將單字集的字也變成小寫母音為同樣的特殊符號 => 符合!!


### CPP CODE

```cpp
class Solution {
public:
    vector<string> spellchecker(vector<string>& wordlist, vector<string>& queries) {
        set<string> st;
        map<string, string> mp;

        for(string s : wordlist){
            st.insert(s);

            string lower = toLower(s);
            string devowel = deVowel(lower);

            if(!mp.count(lower))
                mp[lower] = s;
            if(!mp.count(devowel))
                mp[devowel] = s;
        }

        /*
        for(auto it : mp){
            cout<< it.first << " " << it.second << endl;
        }
        */

        vector<string> ans;
        for(string s : queries){

            string l = toLower(s);
            string d = deVowel(l);

            if(st.count(s))
                ans.push_back(s);
            else if(mp.count(l))
                ans.push_back(mp[l]);
            else if(mp.count(d))
                ans.push_back(mp[d]);
            else
                ans.push_back("");
        }

        return ans;
    }

private:
    string toLower(string s){
        for(auto &c : s)
            c = tolower(c);
        return s;
    }

    string deVowel(string s){
        char vowels[] = {'a', 'e', 'i', 'o', 'u'};
        for(char &c : s){
            if(ranges::any_of(vowels, [c](char a){ return a == c; })){
                c = '1';
            }
        }
        return s;
    }
    
};
```

