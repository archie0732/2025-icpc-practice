# 3541. Find Most Frequent Vowel and Consonant

>[!note]
>- Easy
>- func



## 題目

給一個字串，求裡面出現頻率最多母音與字音的出現次數總和

## 解題

很簡單，不過可以用一個比較炫的方法

>[!tip]
>- renages::any_of(\<Array>, []\(){})


### CPP CODE

```cpp
class Solution {
public:
    int maxFreqSum(string s) {
        int mxV = 0, mxC = 0;
        map<char, int>mp;

        char vowel[] = {'a', 'e', 'i', 'o', 'u'};


        for(auto c : s){
            if(std::ranges::any_of(vowel, [c](char a){
                return c == a;
            })){
                mp[c]++;


                mxV = max(mxV, mp[c]);
            }
            else{
                mp[c]++;
                mxC = max(mxC, mp[c]);
            }
        }


        return mxV + mxC;

    }


};
```
