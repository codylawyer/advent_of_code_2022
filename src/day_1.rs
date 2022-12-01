pub fn part_1(input: &Vec<i32>) -> i32 {
    // iterate over input, fold takes initial value and a closure
    input
        .iter()
        .fold(0, |accum, item| if accum >= *item { accum } else { *item })
}

pub fn part_2(input: &mut Vec<i32>) -> i32 {
    input.sort();
    input.reverse();
    input[0] + input[1] + input[2]
}
