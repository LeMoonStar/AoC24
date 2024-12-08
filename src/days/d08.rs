use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Sub},
};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vector {
    pub x: i64,
    pub y: i64,
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AntennaMap {
    antenna: HashMap<char, Vec<Vector>>,
    limits: Vector,
}

impl AntennaMap {
    fn is_in_limits(&self, point: Vector) -> bool {
        !(point.x < 0 || point.y < 0 || point.x >= self.limits.x || point.y >= self.limits.y)
    }

    fn get_antinodes(&self, frequency: char) -> Option<Vec<Vector>> {
        let antennas = self.antenna.get(&frequency)?;
        let antenna_count = antennas.len();

        if antenna_count < 2 {
            return Some(vec![]);
        }

        let mut antinodes = Vec::with_capacity(antenna_count.pow(2) - antenna_count);

        for a in 0..antenna_count {
            for b in (0..antenna_count).filter(|v| *v != a) {
                let new_position =
                    antennas[a] + (antennas[b] - antennas[a]) * Vector { x: 2, y: 2 };
                if self.is_in_limits(new_position) {
                    antinodes.push(new_position);
                }
            }
        }

        Some(antinodes)
    }

    fn get_resonances(&self, frequency: char) -> Option<Vec<Vector>> {
        let antennas = self.antenna.get(&frequency)?;
        let antenna_count = antennas.len();

        if antenna_count < 2 {
            return Some(vec![]);
        }

        let mut antinodes = Vec::with_capacity(antenna_count.pow(2) - antenna_count);

        for a in 0..antenna_count {
            for b in (0..antenna_count).filter(|v| *v != a) {
                let distance = antennas[b] - antennas[a];

                let mut multiplier = 1; // WHYYYY does this have to be 1.... that's just dumb

                loop {
                    let new_position = antennas[a]
                        + distance
                            * Vector {
                                x: multiplier,
                                y: multiplier,
                            };
                    if !self.is_in_limits(new_position) {
                        break;
                    }
                    antinodes.push(new_position);
                    multiplier += 1;
                }
            }
        }

        Some(antinodes)
    }

    fn get_all_antinodes(&self) -> Vec<Vector> {
        self.antenna
            .keys()
            .filter_map(|frequency| self.get_antinodes(*frequency))
            .flatten()
            .collect()
    }

    fn get_all_resonances(&self) -> Vec<Vector> {
        self.antenna
            .keys()
            .filter_map(|frequency| self.get_resonances(*frequency))
            .flatten()
            .collect()
    }
}

impl From<&str> for AntennaMap {
    fn from(value: &str) -> Self {
        let lines = value.lines();

        let limits = Vector {
            y: lines.clone().count() as i64,
            x: lines.clone().next().unwrap().len() as i64,
        };

        Self {
            antenna: lines
                .enumerate()
                .map(|(y, line)| {
                    (
                        y,
                        line.chars()
                            .enumerate()
                            .filter(|(_, char)| *char != '.')
                            .collect::<Vec<_>>(),
                    )
                })
                .filter(|(_, line)| line.len() > 0)
                .fold(HashMap::new(), |mut map, (y, line)| {
                    line.into_iter().for_each(|(x, frequency)| {
                        if let Some(antenna) = map.get_mut(&frequency) {
                            antenna.push(Vector {
                                x: x as i64,
                                y: y as i64,
                            });
                        } else {
                            map.insert(frequency, vec![Vector {
                                x: x as i64,
                                y: y as i64,
                            }]);
                        }
                    });
                    map
                }),
            limits: limits,
        }
    }
}

type Data = AntennaMap;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test08.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(14), Answer::Number(34))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(
            data.get_all_antinodes()
                .into_iter()
                .collect::<HashSet<Vector>>()
                .len() as u64,
        )
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(
            data.get_all_resonances()
                .into_iter()
                .collect::<HashSet<Vector>>()
                .len() as u64,
        )
    }
}
