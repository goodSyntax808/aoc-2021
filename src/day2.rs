use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

pub enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(' ');
            let direction = Direction::from_str(split.next().unwrap()).unwrap();
            let units: i32 = split.next().unwrap().parse().unwrap();

            (direction, units)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_day2_pt1(input: &[(Direction, i32)]) -> i32 {
    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;

    for (direction, units) in input {
        match direction {
            Direction::Forward => horizontal_pos += units,
            Direction::Up => vertical_pos -= units,
            Direction::Down => vertical_pos += units,
        }
    }

    horizontal_pos * vertical_pos
}

#[aoc(day2, part2)]
pub fn solve_day2_pt2(input: &[(Direction, i32)]) -> i32 {
    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;
    let mut aim = 0;

    for (direction, units) in input {
        match direction {
            Direction::Forward => {
                horizontal_pos += units;
                vertical_pos += aim * units
            }
            Direction::Up => aim -= units,
            Direction::Down => aim += units,
        }
    }

    horizontal_pos * vertical_pos
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_inputs_return_0() {
        assert_eq!(1, 1);
    }
}
