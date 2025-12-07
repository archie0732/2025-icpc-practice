s = str(input())

dicts = ["h", "e", "l", "l", "o"]

flag = 0


for x in s:
    if x.lower() == dicts[flag]:
        flag += 1
        # print(x, end=" ")
    if flag > 4:
        break

print("YES" if flag > 4 else "NO")
