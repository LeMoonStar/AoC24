use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 4;

#[derive(Debug, Clone, Copy)]
enum Direction {
    East,
    West,
    North,
    South,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug, Clone)]
pub struct LetterWall(Vec<Vec<char>>);

impl LetterWall {
    fn at(&self, x: usize, y: usize) -> Option<&char> {
        let row = self.0.get(y);

        if let Some(row) = row {
            row.get(x)
        } else {
            None
        }
    }

    #[rustfmt::skip]
    fn get_xmas_directions(&self, x: usize, y: usize) -> Vec<Direction> {
        let mut directions = vec!();

        let map = (
            self.at(x, y), self.at(x+1, y), self.at(x+2, y), self.at(x+3, y),
            self.at(x, y+1), self.at(x+1, y+1), self.at(x+2, y+1), self.at(x+3, y+1),
            self.at(x, y+2), self.at(x+1, y+2), self.at(x+2, y+2), self.at(x+3, y+2),
            self.at(x, y+3), self.at(x+1, y+3), self.at(x+2, y+3), self.at(x+3, y+3)
        );
        
        if let (
            Some(&'X'), Some(&'M'), Some(&'A'), Some(&'S'),
            _, _, _, _,
            _, _, _, _,
            _, _, _, _
        ) = map {
            directions.push(Direction::East);
        }
        
        if let (
            Some('S'), Some('A'), Some('M'), Some('X'),
            _, _, _, _,
            _, _, _, _,
            _, _, _, _
        ) = map {
            directions.push(Direction::West);
        }
        
        if let (
            Some('X'), _, _, _,
            Some('M'), _, _, _,
            Some('A'), _, _, _,
            Some('S'), _, _, _
        ) = map {
            directions.push(Direction::South);
        }
        
        if let (
            Some('S'), _, _, _,
            Some('A'), _, _, _,
            Some('M'), _, _, _,
            Some('X'), _, _, _
        ) = map {
            directions.push(Direction::North);
        }
        
        if let (
            Some('X'), _, _, _,
            _, Some('M'), _, _,
            _, _, Some('A'), _,
            _, _, _, Some('S')
        ) = map {
            directions.push(Direction::SouthEast);
        }
        
        if let (
            Some('S'), _, _, _,
            _, Some('A'), _, _,
            _, _, Some('M'), _,
            _, _, _, Some('X')
        ) = map {
            directions.push(Direction::NorthWest);
        }
        
        if let (
            _, _, _, Some('X'),
            _, _, Some('M'), _,
            _, Some('A'), _, _,
            Some('S'), _, _, _
        ) = map {
            directions.push(Direction::SouthWest);
        }
        
        if let (
            _, _, _, Some('S'),
            _, _, Some('A'), _,
            _, Some('M'), _, _,
            Some('X'), _, _, _
        ) = map {
            directions.push(Direction::SouthEast);
        }

        directions
    }

    pub fn count_xmas(&self) -> u64 {
        let mut count = 0;
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                count += self.get_xmas_directions(x, y).len();
            }
        }
        return count as u64;
    }
}

impl From<&str> for LetterWall {
    fn from(value: &str) -> Self {
        Self(value.lines().map(|v| v.chars().collect()).collect())
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
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        // NOTE TO SELF: Actual result above 2334
        Answer::Number(data.count_xmas())
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0)
    }
}
