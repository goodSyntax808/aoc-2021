use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
pub fn count_increases(input: &[i32]) -> u32 {
    if input.len() == 0 {
        return 0;
    }
    let mut iter = input.iter();
    let mut prev_val = iter.next().unwrap();
    let mut i = 0;
    for val in iter {
        if prev_val < val {
            i += 1;
        }
        prev_val = val;
    }

    return i;
}

pub fn count_increases_windowed(input: &[i32], window_size: u32) -> u32 {
    let mut num_increases = 0;
    let mut prev_sum = None;
    let len = input.len();

    for (i, val) in input.into_iter().enumerate() {
        if len - i < window_size as usize {
            break;
        }
        let mut sum = val.clone();
        for j in 1..window_size {
            // ideally would handle overflow errors with the casting
            sum += input[i + j as usize];
        }

        if prev_sum.is_some() && prev_sum.unwrap() < sum {
            num_increases += 1;
        }
        prev_sum = Some(sum);
    }

    num_increases
}

#[aoc(day1, part2)]
pub fn count_increases_3day_window(input: &[i32]) -> u32 {
    count_increases_windowed(input, 3)
}

#[cfg(test)]
mod tests {
    mod part1 {
        use crate::day1::count_increases;
        #[test]
        fn empty_inputs_return_0() {
            assert_eq!(count_increases(&[]), 0);
        }

        #[test]
        fn no_increases_return_0() {
            assert_eq!(count_increases(&[5, 5, 3, 2, 1, 0, -1]), 0);
        }

        #[test]
        fn increases_are_counted() {
            assert_eq!(count_increases(&[-1, 0, 1, 1, 2]), 3);
        }
    }

    mod part2 {
        use crate::day1::count_increases_3day_window;
        #[test]
        fn inputs_less_than_window_size_return_0() {
            assert_eq!(count_increases_3day_window(&[]), 0);
            assert_eq!(count_increases_3day_window(&[1, 2]), 0);
            assert_eq!(count_increases_3day_window(&[1, 2, 3]), 0);
        }

        #[test]
        fn no_increases_return_0() {
            assert_eq!(count_increases_3day_window(&[5, 5, 3, 2, 1, -1]), 0);
        }

        #[test]
        fn increases_are_counted() {
            assert_eq!(count_increases_3day_window(&[1, 2, 3, 4]), 1);
            assert_eq!(count_increases_3day_window(&[-1, 0, 1, 1, 2, 1]), 2);
        }
    }
}
