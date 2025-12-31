# 1970. Last Day Where You Can Still Cross


>[!note]
>Hard
>Dsu(Union find)

## 題目

[題目連結](https://leetcode.com/problems/last-day-where-you-can-still-cross/description/?envType=daily-question&envId=2025-12-31)

## 解題

一開始板子都在->板子會依序消失根據天數->最後板子都不見

所以我們可以反推: 一開始都沒有板子 -> 回退一天多一塊板子，將板子連接起來 -> 直到可以重頭部連置底部

>[!tip]
>## DUS (Union)
>將板子設一個歸屬 (連通)
>當 START == END 就代表連通


### CODE

```python

class DSU:
    def __init__(self, n):
        self.parent = list(range(n))
        self.rank = [1] * n
    
    def find(self, x):
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x]) 
        return self.parent[x]
    
    def union(self, x, y):
        rootX = self.find(x)
        rootY = self.find(y)
        
        if rootX != rootY:
            if self.rank[rootX] > self.rank[rootY]:
                self.parent[rootY] = rootX
            elif self.rank[rootX] < self.rank[rootY]:
                self.parent[rootX] = rootY
            else:
                self.parent[rootY] = rootX
                self.rank[rootX] += 1
            return True
        return False

    def is_connected(self, x, y):
        return self.find(x) == self.find(y)

class Solution:
    def latestDayToCross(self, row: int, col: int, cells: List[List[int]]) -> int:
        
        grids = [[0] * col for _ in range(row)]
        START = col * row
        END = col * row + 1
        dsu = DSU(col * row + 2)


        for i in range(len(cells) - 1, -1, -1):
            r, c = cells[i][0] - 1, cells[i][1] - 1
            grids[r][c] = 1

            curIdx = r * col + c

            if r == 0:
                dsu.union(END, curIdx)
            if r == row - 1:
                dsu.union(START, curIdx)
            
            dir = [(0, 1), (0, -1), (1, 0), (-1, 0)]

            for dr, dc in dir:
                nr = dr + r
                nc = dc + c

                nIdx = nr * col + nc

                if 0 <= nr < row and 0 <= nc < col and grids[nr][nc] == 1:
                    dsu.union(nIdx, curIdx)

                if dsu.is_connected(START, END):
                    return i

        return 0  

```
