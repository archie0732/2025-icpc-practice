import sys

data = sys.stdin.read().split()

n = int(data[0])

l = [int(i) for i in data[1 : 1 + n * n]]

board = [l[i * n : (i + 1) * n] for i in range(n)]

mx = -100000000

for top in range(1, n):
    temp = [0] * n
    for i in range(top, n):

        for col in range(n):
            temp[col] += board[i][col]

        current = 0

        for j in temp:
            current = max(j, current + j)
            mx = max(current, mx)


print(mx)
