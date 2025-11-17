# 並查集 (Disjoint Set Union, DSU)

>[!note]
>用來管理「互不相交的集合」

並查集就是專門用來快速執行這兩件事的：

> 1. 合併 (Union)： 把兩個不同的組別「合併」成同一組。
> 2. 查詢 (Find)： 快速查詢某個人「屬於哪個組別？」

## 用法

- 尋找

```py
def find(i):
    if parent[i] == i:
        return i
    
    # 關鍵：在回程時，把路徑上所有節點都直接連到 root
    parent[i] = find(parent[i]) 
    return parent[i]

```

- 合併

```py
# 假設 size[] 陣列一開始都初始化為 1
def union(i, j):
    root_i = find(i)
    root_j = find(j)
    
    if root_i != root_j:
        # 關鍵：把小樹(i)接在大樹(j)底下
        if size[root_i] < size[root_j]:
            parent[root_i] = root_j
            size[root_j] += size[root_i]
        else:
            # 包含 size[i] >= size[j] 的情況
            parent[root_j] = root_i
            size[root_i] += size[root_j]
```
