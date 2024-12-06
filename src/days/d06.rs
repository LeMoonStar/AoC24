use super::{Answer, Day, DayImpl};
use std::{collections::HashSet, hash::Hash, path::Iter};

const CURRENT_DAY: u8 = 6;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

/**
A 2D position
```
   y-
x- + x+
   y+
```
*/
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn move_in_direction(
        &self,
        dir: Direction,
        steps: u8,
        limit_x: u8,
        limit_y: u8,
    ) -> Option<Self> {
        match dir {
            Direction::North => Some(Self {
                x: self.x,
                y: self.y.checked_sub(steps)?,
            }),
            Direction::South => {
                if self.y + 1 >= limit_y {
                    None
                } else {
                    Some(Self {
                        x: self.x,
                        y: self.y.checked_add(steps)?,
                    })
                }
            }
            Direction::East => Some(Self {
                x: self.x.checked_add(steps)?,
                y: self.y,
            }),
            Direction::West => {
                if self.x + 1 >= limit_x {
                    None
                } else {
                    Some(Self {
                        x: self.x.checked_sub(steps)?,
                        y: self.y,
                    })
                }
            }
        }
    }
}

pub struct PatrolPathIterator<'a> {
    position: Option<Position>,
    direction: Direction,
    map: &'a PatrollingMap,
}

impl<'a> Iterator for PatrolPathIterator<'a> {
    type Item = (Position, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_pos =
            self.position?
                .move_in_direction(self.direction, 1, self.map.width, self.map.height);

        if self.map.is_obstacle(next_pos?) {
            self.direction = self.direction.turn_right();
            next_pos = self.position?.move_in_direction(
                self.direction,
                1,
                self.map.width,
                self.map.height,
            );
        }
        self.position = next_pos;
        Some((self.position?, self.direction))
    }
}

#[derive(Debug, Clone)]
pub struct PatrollingMap {
    pub width: u8,
    pub height: u8,
    obstacles: HashSet<Position>,
    start_position: Position,
}

impl PatrollingMap {
    pub fn get_visiting_positions(&self) -> HashSet<Position> {
        let mut visited = HashSet::new();

        visited
    }

    pub fn is_obstacle(&self, pos: Position) -> bool {
        self.obstacles.contains(&pos)
    }

    pub fn iter(&self) -> PatrolPathIterator {
        PatrolPathIterator {
            position: Some(self.start_position),
            direction: Direction::North,
            map: self,
        }
    }
}

impl From<&str> for PatrollingMap {
    fn from(value: &str) -> Self {
        let mut start_position: Option<Position> = None;

        let lines = value.lines();

        let height = lines.clone().count() as u8;
        let width = lines.clone().next().unwrap().len() as u8;

        let obstacles = lines
            .map(|line| line.chars().enumerate().collect::<Vec<(usize, char)>>())
            .enumerate()
            .inspect(|(y, chars)| {
                chars.iter().for_each(|(x, c)| {
                    if *c == '^' {
                        start_position = Some(Position {
                            x: *x as u8,
                            y: *y as u8,
                        });
                    }
                });
            })
            .map(|(y, chars)| {
                (
                    y,
                    chars
                        .iter()
                        .filter(|(_, c)| *c == '#')
                        .map(|(x, _)| *x)
                        .collect::<Vec<usize>>(),
                )
            })
            .fold(vec![], |mut acc, (y, obstacles_in_row)| {
                obstacles_in_row.iter().for_each(|x| {
                    acc.push(Position {
                        x: *x as u8,
                        y: y as u8,
                    })
                });
                acc
            })
            .into_iter()
            .collect();

        Self {
            start_position: start_position.expect("Couldn't find start position"),
            obstacles,
            width,
            height,
        }
    }
}

type Data = PatrollingMap;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test06.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(41), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        println!("{:?}", data);
        let visited: HashSet<Position> = data
            .iter()
            //.inspect(|v| println!("{:?}", v))
            .map(|v| v.0)
            .collect();
        Answer::Number(visited.len() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0)
    }
}
