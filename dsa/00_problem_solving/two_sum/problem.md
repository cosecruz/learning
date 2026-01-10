# Two sum problem

Given an input of a collection of numbers and a target, find the indices where values in the collection sum to the target.

For example:
Input: [2,7,11,5], target = 9
Output: [0,1] (because 2+7 = 9);

Solution by examples:

1. Try all pairs
   inpuut : [3, 1, 4, 2]; target : 3

take elem 3; 3 == target but we need two sum and no 0 to add to give 3
3+1 = 4 //wrong
3+4 = 7
3 + 2 = 5

take next elem = 1
1 + 4 = 5
1+ 2 = 3 correct found it
//but brute force O(n^@)

1. Use target - elem to find reaminder
   Input: [2,3,5,4,6,2] target: 10;

   questions: - what do we do with duplicates

2: target = 10 - 2 = 8; check if 8 exists in the array
3: 10 -3 = 7 does it exist?
5: 10 - 5 = 5 does another 5 exist
4: 10-4=6; does 6 exist? yes return index of 4 and 6
