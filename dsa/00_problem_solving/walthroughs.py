# find maximum element
# index = 0
# max = arr[index]
# if arr[index +1]>max
# update max
# else return max


def find_max(arr):
    if len(arr) == 1:
        return arr[0]

    max = arr[0]

    for n in arr:
        if n > max:
            max = n

    return max


arr = [
    7,
    7,
    7,
    7,
]
print(find_max(arr))

# with python api
print(max(arr))

# Count elements > average

#  Input: [1,2,3,4,5]
#  avg = 3.0
#  elem = [4,5]
#  output = 2

# arr[] of length n = (1 ≤ n ≤ 10⁵)

# expected complexity at worst case = O(n log n)


# edge case
# is arr an ordered list ot not
#  does it inlude negative numbers
# what if len(arr) is 1

# simulation
# start:
# if len(arr) = 1; return None
# calculate sum of all ell elem in arr
#  sum = 0
#  loop array: sum += elem of arr
#  avg = sum/len(arr)
#  gt_avg_arr will contain all nums in array gt avg
# loop through arr: push n > avg
#  return len(gt_avg_arr)


def find_elem_gt_avg(arr):
    if len(arr) <= 1:
        return 0

    total = 0

    for num in arr:
        total += num

    avg = total / len(arr)

    count = 0
    for n in arr:
        if n > avg:
            count += 1

    return count


arr2 = [1, 2, 3, 4, 5]
print(find_elem_gt_avg(arr2))


def find_elem_gt_avg2(arr):
    if len(arr) <= 1:
        return 0

    avg = sum(arr) / len(arr)
    return sum(1 for x in arr if x > avg)


# count negative numbers
def count_negataive(arr):
    if len(arr) < 1:
        return None

    count = 0
    for num in arr:
        if num < 0:
            count += 1
    return count


print(count_negataive([-1, 5, -3, 0, -2]))


# final second maximum
# Input: Array of at least 2 distinct integers
# Output: Second largest element
# Example: [3, 1, 4, 1, 5] → 4

# start: max ,  next_max = arr[0]


# loop arr: if num>max
# next_max = max
# max = num
#
# return nextmax
#
def find_2nd_largest(arr):
    if len(arr) <= 1:
        return None

    max, next_max = arr[0], arr[0]

    for num in arr:
        if num > max:
            next_max = max
            max = num
        elif num != max and num > next_max:
            next_max = num

    return next_max


print(find_2nd_largest([3, 1, 4, 1, 5]))

# **Exercise 4: Reverse Array**
#
# Input: Array arr[]
# Output: Array in reverse order
# Example: [1, 2, 3] → [3, 2, 1]

# the easiest way brute force
# start:
# new arr = []
#  i = len(arr)-1
# while i <0
#   append to new arr elem at i
#  i --
# return new arr


def reverse_arr(arr):
    i = len(arr)

    if i <= 1:
        return arr

    new_arr = []
    while i > 0:
        new_arr.append(arr[i - 1])
        i -= 1

    return new_arr


print(reverse_arr([1, 2, 3]))


def in_place_reversal(arr):
    left, right = 0, len(arr) - 1

    while right > left:
        arr[left], arr[right] = arr[right], arr[left]
        left += 1
        right -= 1
    return arr


# check if sorted in ascending order


def check_sorted(arr):
    if len(arr) <= 1:
        return True

    n = 0
    while n < len(arr) - 1:
        if arr[n] > arr[n + 1]:
            return False
        n += 1
    return True


print(check_sorted([1, 4, 3]))


# **Exercise 6: Find Missing Number**
#
# ```
# Input: Array of n-1 integers from range [1, n] (one missing)
# Output: The missing number
# Example: [1, 2, 4, 5] (n=5) → 3


def missing_num(arr, n):
    if len(arr) != n - 1:
        return None

    sum_of_n_elem = n * (n + 1) // 2

    sum_of_elem_in_arr = 0

    for num in arr:
        sum_of_elem_in_arr += num

    return sum_of_n_elem - sum_of_elem_in_arr


print(missing_num([1, 2, 4, 5], 5))


def missing_num_xor(arr, n):
    xor_all = 0
    for i in range(1, n + 1):
        xor_all ^= i

    for num in arr:
        xor_all ^= num

    return xor_all
