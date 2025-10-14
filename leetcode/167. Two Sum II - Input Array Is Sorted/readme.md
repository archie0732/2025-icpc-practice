# 167. Two Sum II - Input Array Is Sorted


>[!note]
> Medium
> binary search

## question

給一個遞增的陣列與數字k，求將兩個數字相加等於k，回傳那兩個數的位置

>[!important]
> 位置為 1 ~ len(numbers)


## solution

for跑整個陣列

對每個數去做 binary search 去找

## CODE

```py


class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        def binarySearch(i: int) -> int:
            left, right = 0, len(numbers) - 1
            a = numbers[i]
            while left <= right:
                mid = (left + right) // 2
                b = numbers[mid]
                if mid != i and a + b == target:
                    return mid
                elif a + b < target:
                    left = mid + 1
                else:
                    right = mid - 1
            return -1

        for i in range(len(numbers)):
            j = binarySearch(i)
            if j != -1:
                
                return sorted([i + 1, j + 1])

        return []

```
