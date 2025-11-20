def main():
    n = int(input())
    luck_number_list = [x for x in range(1, 1001) if set(str(x)) <= {"4", "7"}]
    for x in luck_number_list:
        if n % x == 0:
            return "YES"
    return "NO"


print(main())
