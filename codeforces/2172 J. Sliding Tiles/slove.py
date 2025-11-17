n = int(input())

borad = [int(x) for x in input().split()]

walls = [int(x) for x in input().split()]


high = borad


for i in range(len(borad) - 1, -1, -1):
    block = borad[i]
    j = i - 1
    while j >= 0:
        block = max(high[i], walls[j])
        if high[j] > block:
            move = high[j] - block
            print(f"move {move}, No.{i} get {move} current high is {high[j]}")
            borad[i] += move
            high[i] = high[j]
            high[j] = block
        else:
            print("can not move anything")
        j -= 1

        print(f"high[{i}]: {high[i]}")
        print(high)
        print("=" * 5)
    print("*" * 5)
    print(f"get next {i-1}")

print(borad)
