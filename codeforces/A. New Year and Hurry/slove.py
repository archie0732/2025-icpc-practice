def main() -> int:
    p, m = map(int, input().split())
    allCanUse = 4 * 60 - m

    # print(f"debug: all can use: {allCanUse}")

    def binarySearch() -> int:
        l, r = 0, p + 1

        while l < r:
            mid = (l + r) // 2
            canUse = 5 * mid * (mid + 1) // 2

            if canUse <= allCanUse:
                l = mid + 1
            else:
                r = mid

        return l - 1

    print(binarySearch())
    return 0


main()
