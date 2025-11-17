# 1996 A. Legs

>[!note]
> 800

## CODE

```py
t = int(input())

while t > 0:
    t -= 1
    n = int(input())

    print(((n // 4) + (1 if n % 4 > 0 else 0)))

```
