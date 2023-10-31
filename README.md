# aoc-template

My template for solving the [advent-of-code](https://adventofcode.com) puzzles with rust.


## How to get the most out of this template

### Preparation

1. Install [just](https://just.systems)
2. Log-in on https://adventofcode.com/
3. Inspect your cookies and copy content of the `AOC_SESSION` cookie
4. Export an `AOC_SESSION` environment variable with the content of the `AOC_COOKIE`


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


## Unlicense

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
