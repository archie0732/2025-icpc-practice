import sys


line = sys.stdin.read()

flag = False

for c in line:
    if c == '"':
        if flag == False:
            print("``", end="")
            flag = True
        else:
            print("''", end="")
            flag = False
    else:
        print(c, end="")
