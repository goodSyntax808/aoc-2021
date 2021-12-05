use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
pub fn count_increases(input: &[i32]) -> usize {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

#[aoc(day1, part2)]
pub fn count_increases_3day_window(input: &[i32]) -> usize {
    input
        .windows(4)
        .filter(|window| window[0] < window[3])
        .count()
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
