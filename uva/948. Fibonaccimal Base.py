board = []


def build_board():
    for i in range(501):
        if i == 0:
            board.append(0)
        elif i == 1 or i == 2:
            board.append(1)
        else:
            board.append(board[i - 1] + board[i - 2])


build_board()
n = int(input())

for _ in range(n):
    x = int(input())

    ans = ""
    flag = False
    print(f"{x} = ", end="")

    for i in range(len(board) - 1, 1, -1):
        if board[i] <= x:
            x -= board[i]
            flag = True
            ans += "1"
        elif flag == True:
            ans += "0"

    print(f"{ans} (fib)")
