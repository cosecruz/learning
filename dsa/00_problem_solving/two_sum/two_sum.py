def two_sum_brute(arr, target):
    n = len(arr)

    for i in range(n):
        for j in range(i + 1, n):
            if arr[i] + arr[j] == target:
                return [i, j]
    return None


def two_sum(arr, target):
    seen = {}

    for i, n in enumerate(arr):
        complement = target - n
        if complement in seen:
            return [seen[complement], i]
        seen[n] = i
    return None


# val = two_sum_brute([1, 2, 3, 4], 3)
val = two_sum([1, 2, 3, 4, 5], 7)
print(val)
