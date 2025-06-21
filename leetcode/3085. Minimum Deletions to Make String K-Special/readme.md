# 3085. Minimum Deletions to Make String K-Special


>[!note]
>- `hash table` `greedy`
>- `Medium`


## 題目

懶，略

### 解題

要求: `任意兩點間距小於 k`

重設範圍 -> `fre[i] ~ fre[i] + k` ， i 為任意一點

>- 大於: 刪到滿足
>- 小於: 直接砍點


### CODE(CPP)

```cpp

class Solution {
public:
    int minimumDeletions(string word, int k) {
        map<char, int>fr;
        for(const auto& it : word)
            fr[it]++;
        vector<int>v;

        for(const auto& [c, val] : fr){
            cout<<c<<" "<<val<<endl;v.push_back(val);
        }
        sort(v.begin(), v.end());

        int ans = INT_MAX, n = v.size();

        for(int i = 0; i < n; i++){
            int result = 0, cur = v[i];

            for(int j = 0; j < n; j++){

                if(k + cur < v[j])
                    result += v[j]-cur-k;
                else if(v[j] < cur)
                    result += v[j];
            }
            cout<<result<<endl<<"===="<<endl;

            ans = min(result, ans);
        }

        return ans;
        
    }
};

/*
    n = 1
    w = 1
    o = 2
    v = 4
*/
```