a = sorted([int(x) for x in input().split()])


if a[2] - 10 >= a[0]:
    print("check again")
else:
    print(f"final {a[1]}")
