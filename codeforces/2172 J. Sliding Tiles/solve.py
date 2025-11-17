import sys
import heapq


def solve():
    input = sys.stdin.readline
    n = int(input())

    col_height = list(map(int, input().split()))
    wall = list(map(int, input().split()))
    wall.append(n)

    falling = {}
    pq = []
    events = [dict() for _ in range(n)]

    def update_falling(x, v):
        """falling[x] += v，同時更新 heap"""
        if v == 0:
            return
        old = falling.get(x, 0)
        new = old + v

        if new == 0:
            falling.pop(x, None)
        else:
            falling[x] = new
            if old == 0:
                heapq.heappush(pq, x)

    for r in range(n):
        update_falling(0, 1)
        update_falling(col_height[r], -1)

        stop = wall[r]
        absorbed = 0

        while pq:
            x = pq[0]
            if x not in falling:
                heapq.heappop(pq)
                continue

            if x >= stop:
                break

            heapq.heappop(pq)
            cnt = falling.pop(x)
            absorbed += cnt
            events[r][x] = events[r].get(x, 0) + cnt

        if absorbed:
            events[r][stop] = events[r].get(stop, 0) - absorbed
            update_falling(stop, absorbed)

    ans = [0] * n
    for r in range(n - 1, -1, -1):
        level = 0
        prev = -1

        for x in sorted(events[r]):
            if prev != -1:
                width = x - prev
                ans[r] += width
                if r - level >= 0:
                    ans[r - level] -= width

            level += events[r][x]
            prev = x

        if r + 1 < n:
            ans[r] += ans[r + 1]

    print(*ans)


solve()
