import math

t = int(input())

while t > 0:
    t -= 1
    n = int(input())

    a = list(map(int, input().split()))

    sum = 0

    for x in a:
        sum += x

    print(
        "YES"
        if math.floor(math.sqrt(sum)) * math.floor(math.sqrt(sum)) == sum
        else "NO"
    )
