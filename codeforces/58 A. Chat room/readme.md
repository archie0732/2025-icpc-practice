# A. Chat room

[qustion](https://codeforces.com/problemset/problem/58/A)

>[!note]
> 1000

## 題目

給一個字串 s 如果可以**順序正確的**組成 hello 輸出 YES 反之 NO

## 解題

用一個陣列 dists 存 hello 然後遍歷 s 如果s[i] == dists[flag]，則 flag++

>如果遍歷完後 flag > 4 說明順序全對，輸出 YES 反之 NO

### CODE

```py
s = str(input())

dicts = ["h", "e", "l", "l", "o"]

flag = 0


for x in s:
    if x.lower() == dicts[flag]:
        flag += 1
    if flag > 4:
        break

print("YES" if flag > 4 else "NO")
```
