# aoc-template

My template for solving the [advent-of-code](https://adventofcode.com) puzzles with rust.


## How to get the most out of this template

### Preparation

1. Install [just](https://just.systems)
2. Log-in on https://adventofcode.com/
3. Inspect your cookies and copy content of the `AOC_SESSION` cookie
4. Export an `AOC_SESSION` environment variable with the content of the `AOC_COOKIE`
5. Update the `year` variable in the `justfile` to the year you would like to solve


### Each day

*(replace `$DAY` by the current day of the month)*

1. get your personal puzzle input with `just get-input $DAY`
2. start the tests with `just watch $DAY"`
   (it'll automatically rerun the tests when the source change)
3. Write your tests and solve the puzzle in `src/day$DAY/mod.rs`
5. Once you think you're done, enable the `input` test case and watch it fail.
   The failure message contains your puzzle answer.
6. If your answer is correct, write it as the expected value for that test. (so that all the tests pass)
7. Refactor as you please

