# 2172 A. ASCII Art Contest

## 解題

一共就三個數，sort完後就可以確定中位數了，接下來就看上下限是否超過10

### CODE

```py
a = sorted([int(x) for x in input().split()])


if a[2] - 10 >= a[0]:
    print("check again")
else:
    print(f"final {a[1]}")
```
