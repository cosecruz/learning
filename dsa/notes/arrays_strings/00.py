# two pointers
def example(arr):
    left, right = 0, len(arr) - 1

    while left < right:
        # process
        left += 1
        right -= 1


# sliding window
def example2(arr):
    k = 2  # window size

    for i in range(len(arr) - k + 1):
        window = arr[i : k + i]

        # process window


# reverse iteration


def example3(arr):
    for i in range(len(arr) - 1, -1, -1):
        print(arr[i])


print(example3([1, 2, 3, 4]))

# prefix sums
# for example to sum all elements in an array from left to right
# Array:     [3, 1, 4, 1, 5]
# Prefix:    [3, 4, 8, 9, 14]
#            ↑  ↑  ↑  ↑  ↑
#            3  3+1 3+1+4 ... sum of all so far

# simulation
# start:
# arr = [1,2,3,4]
# prefix = []
# 0 prefix = arr[n]
# 1 prefix = arr[n]+ arr[n+1]
# 2 prefix = arr[n] + arr[n+1] = arr[n+2]  and so on


def example4(arr):
    if not arr:
        return []

    prefix = [arr[0]]

    for i in range(1, len(arr)):
        prefix.append(arr[i] + prefix[i - 1])
    return prefix


def range_sum(prefix, L, R):
    """Get sum from index L to R (inclusive)"""
    if L == 0:
        return prefix[R]
    return prefix[R] - prefix[L - 1]


# sum(l..r) = prefix[r] - prefix[l - 1]
# arr = [3, 1, 4, 1, 5]
# prefix = [3, 4, 8, 9, 14]
#
# sum from index 1 to 3 → [1, 4, 1]
# result = prefix[3] - prefix[0]  # 9 - 3 = 6
# special case
# sum(0..r) = prefix[r]


print(example4([1, 2, 3, 4, 5]))

# problem:
# given [3,4,5,2,1,6,7] and k =7
# count how many continuous chunk add up to k

# eg:
# arr = [1,1,1]; k+ 2
#  valid chunks: [1,1] (indew 0->1) and [1,1](index 1-> 2)
#  result 2

# simulation for any number of continuous elements that sums up to k
# start
# count = 0
#


# **todo
# simulation **come back
# start
# count = 0
# loop
# if arr[n] + arr[n+1] == k: count+=1
def count_continuous(arr, k):
    count = 0
    current_total = 0
    total_seen = {0: 1}

    for n in range(len(arr) - 1):
        current_total += arr[n]

        if current_total - k in total_seen:
            count += total_seen[current_total - k]

        total_seen[current_total] = total_seen.get(current_total - k, 0) + 1

    return count


def count_adjacent_chunks_k(arr, k):
    if not arr:
        return 0
    count = 0
    for n in range(len(arr) - 1):
        if arr[n] + arr[n + 1] == k:
            count += 1
    return count


print(count_adjacent_chunks_k([3, 4, 5, 2, 1, 6, 7], 7))
