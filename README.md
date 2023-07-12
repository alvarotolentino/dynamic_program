### Example of dynamic programming

The following are some algorithms that implement the dynamic programming technique.

* bestsum.rs: Given an array of numbers and a target sum, return the shortest combination of numbers that add up to exactly the target sum.

* cansum.rs: Given an array of numbers and a target sum, return true if any combination of numbers in the array can add up to exactly the target sum.

* countconstruct.rs: Given a target string and an array of strings, return the number of ways that the target can be constructed by concatenating elements of the array.

* gridtraveler.rs: Given a grid of size m * n, return the number of ways you can travel from the top-left to the bottom-right of the grid.

* howcansum.rs: Given an array of numbers and a target sum, return an array containing any combination of elements that add up to exactly the target sum.

All of them implement memoization in order to improve the performance of the algorithms.

## How to execute test
```bash
cargo test
```