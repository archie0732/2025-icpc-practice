# 386. Lexicographical Numbers

>[!note]
>- `Medium`
>- `dfs`


## 題目

給一個`range`要求: **`由數字的第一個數字由小到大(如果地一個數字一樣則先排數字較小的)**

- `range = 13` return: `[1,10,11,12,13,2,3,4,5,6,7,8,9]`

## 解題

- `dfs`的題目都是設立`樹幹`與`樹枝`

- 樹幹: 1 ..... 2 ..... 3 ....之間的點為分支(例如1的分支10,100,11,12)
- 樹枝: 單個點之間的分之延伸 1 -> 10 -> 100 -> 1000 -> ....


#### 試想

- 如果 range 為 101 那麼`樹枝 1`的延伸為何

```md
1 -> 10 -> 100 -> 101
```

- 那如果範圍為 13 呢

```md
1 -> 10 -> 11 -> 12 -> 13
```


那麼我們就可以透過 `遞迴` 的方法來實現上面的

```py
# 僅流程，並不是完整程式碼!!

def dfs()
  # 先判斷是否超過 range
  if cur > range:
    return;

  # 由 +1 -> +9 之遞迴
  for i in range(1,9):
    next = cur + i
    # 如果 超過 range 就跳出
    if next > range: break;

    # 尋找 *10 後之結果
    dfs(next, range)

```


最後補齊`樹幹`與`紀錄的陣列`就結束了!

## CODE (CPP)

```cpp
class Solution {
public:
    void dfs(int cur, int range, vector<int>&dp){
        if(cur > range)return;

        dp.push_back(cur);

        for(int i = 0; i <= 9; i++){
            int next = cur * 10 + i;

            if(next > range)break;
            dfs(next, range, dp);
        }
        
    }
    vector<int> lexicalOrder(int n) {
        vector<int>dp;
        
        for(int i = 1; i <= n && i <= 9; i++)
            dfs(i, n, dp);

        return dp;
    }
};
```