# 1744 C. Traffic Light

[link](https://codeforces.com/problemset/problem/1744/C)

## 解題

aaa

### CODEE

```py


t = int(input(""))

while t > 0:
    t -= 1
    [a, b] = input("").split()
    s = input("")
    n = len(s)
    s += s

    ans = 0

    goal = []
    for i in range(len(s)):

        if i < n and s[i] == b:
            goal.append(i)
        if s[i] == "g" and len(goal) > 0:
            ans = max(ans, i - goal.pop(0))
            goal = []
    print(ans)

```
