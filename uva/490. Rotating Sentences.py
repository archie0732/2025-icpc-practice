import sys

lines = []

for i in sys.stdin:
    lines.append(i.replace("\n", "").replace("\r", ""))

max_len = max(len(line) for line in lines)

lines.reverse()


for i in range(max_len):
    row = ""
    for line in lines:
        if i < len(line):
            row += line[i]
        else:
            row += " "
    print(row)
