use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 4;

#[derive(Debug, Clone,Copy)]
enum Direction {
    East,
    West,
    North,
    South,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest
}

#[derive(Debug, Clone)]
pub struct LetterWall (Vec<Vec<char>>);

impl LetterWall {
    fn at(&self, x: usize, y: usize) -> Option<&char> {
        let row = self.0.get(y);

        if let Some(row) = row {
            row.get(x)
        } else {
            None
        }
    }

    fn is_upper_left_of_xmas(&self, x: usize, y: usize) -> Option<Direction> {
        match(
            self.at( x, y), self.at( x+1, y), self.at( x+2, y), self.at( x+3, y),
            self.at( x, y+1), self.at( x+1, y+1), self.at( x+2, y+1), self.at( x+3, y+1),
            self.at( x, y+2), self.at( x+1, y+2), self.at( x+2, y+2), self.at( x+3, y+2),
            self.at( x, y+3), self.at( x+1, y+3), self.at( x+2, y+3), self.at( x+3, y+3)
        ) {
            (
                Some('X'), Some('M'), Some('A'), Some('S'),
                _, _, _, _,
                _, _, _, _,
                _, _, _, _
            ) => Some(Direction::East),
            (
                Some('S'), Some('A'), Some('M'), Some('X'),
                _, _, _, _,
                _, _, _, _,
                _, _, _, _
            ) => Some(Direction::West),
            (
                Some('X'), _, _, _,
                Some('M'), _, _, _,
                Some('A'), _, _, _,
                Some('S'), _, _, _
            ) => Some(Direction::South),
            (
                Some('S'), _, _, _,
                Some('A'), _, _, _,
                Some('M'), _, _, _,
                Some('X'), _, _, _
            ) => Some(Direction::North),
            (
                Some('X'), _, _, _,
                _, Some('M'), _, _,
                _, _, Some('A'), _,
                _, _, _, Some('S')
            ) => Some(Direction::SouthEast),
            (
                Some('S'), _, _, _,
                _, Some('A'), _, _,
                _, _, Some('M'), _,
                _, _, _, Some('X')
            ) => Some(Direction::NorthWest),
            (
                _, _, _, Some('X'),
                _, _, Some('M'), _,
                _, Some('A'), _, _,
                Some('S'), _, _, _
            ) => Some(Direction::SouthWest),
            (
                _, _, _, Some('S'),
                _, _, Some('A'), _,
                _, Some('M'), _, _,
                Some('X'), _, _, _
            ) => Some(Direction::NorthEast),
            _ => None
        }
    }

    pub fn count_xmas(&self) -> u64 {
        let mut count = 0;
        for y in 0..self.0.len(){
            for x in 0..self.0[0].len() {
                if let Some(_dir) = self.is_upper_left_of_xmas(x, y) {
                    dprintln!("FOUND AT {} {} '{}' - DIR: {:?}", x, y, self.0[y][x], _dir);
                    count += 1;
                }
            }
        }
        return count;
    }
}

impl From<&str> for LetterWall {
    fn from(value: &str) -> Self {
        Self (
            value
                .lines()
                .map(|v| v.chars().collect())
                .collect()
        )
    }
}

type Data = LetterWall;

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test04.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(18), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input.into(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        // NOTE TO SELF: Actual result above 2334
        Answer::Number(data.count_xmas())
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0)
    }
}
