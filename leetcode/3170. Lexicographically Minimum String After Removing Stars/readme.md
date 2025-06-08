# 3170. Lexicographically Minimum String After Removing Stars


>[!note]
>- `Medium`
>- `Hash Table` `string` `greedy`

## 題目

給一個字串(包含`*`)

遇到`*`時:

- 刪掉 `*` 並且刪掉在`*`前面的任意一個字元(**須注意必須是`Lexicographically Smaller`: 字元越小越好**)


### 解題

用 `priority_queue`加上自訂排序

```cpp
//priority_queue<type, 跟前面的type同樣的vector, 自訂排序(struct)>
priority_queue<pair<char, int>, vector<char, int>, CMP> queue;
```

排序方法(**要用struct**)

```cpp
struct CMP{
  // 要兩個 ()
  bool operator()(type a,type b){
    // cmp
  }
};
```


### CODE (CPP)

```cpp
class Solution {
public:
    struct CMP1{
        // 要多一個括號!
        bool operator()(pair<char,int>a, pair<char, int>b){
            return a.first == b.first ? a.second < b.second : a.first > b.first;
        }
    };

    string clearStars(string s) {
        int n = s.size();
        string test = "";
        
        priority_queue<pair<char, int>, vector<pair<char, int>>, CMP1> queue;
        vector<int>rmIndex(n, false);

        for(int i = 0 ; i < n; i++){
            if(s[i] != '*')queue.push({s[i], i});
            else{
                auto it = queue.top();
                rmIndex[it.second] = true;
                queue.pop();
            }
        }

        for(int i = 0; i < n; i++){
            if(s[i] != '*' && !rmIndex[i])
                test+=s[i];
        }        


        return test;
    }
};
```