use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 7;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn calculate(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Multiply => a * b,
            Operator::Add => a + b,
        }
    }
}

#[derive(Debug, Clone)]
struct CombinationIterator<'a, T>
where
    T: Sized + Clone,
{
    iteration: usize,
    length: u32,
    possibilities: &'a [T],
}

impl<'a, T> CombinationIterator<'a, T>
where
    T: Sized + Clone,
{
    fn get_index_on_position(&self, position: u32) -> usize {
        (if position == 0 {
            self.iteration
        } else {
            self.iteration / (self.possibilities.len().pow(position))
        }) % self.possibilities.len()
    }

    fn new(possibilities: &'a [T], length: u32) -> Self {
        Self {
            iteration: 0,
            length,
            possibilities,
        }
    }
}

impl<'a, T> Iterator for CombinationIterator<'a, T>
where
    T: Sized + Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.get_index_on_position(self.length) != 0 {
            return None;
        }

        let values = (0..self.length)
            .map(|p| self.possibilities[self.get_index_on_position(p)].clone())
            .collect();
        self.iteration += 1;
        return Some(values);
    }
}

#[derive(Debug, Clone)]
pub struct CalibrationEquation {
    pub result: u64,
    pub parts: Vec<u64>,
}

impl CalibrationEquation {
    fn calculate(&self, operations: Vec<Operator>) -> u64 {
        let mut value = self.parts[0];

        for i in 1..(self.parts.len()) {
            value = operations[i - 1].calculate(value, self.parts[i]);
        }

        value
    }

    fn can_be_valid(&self) -> bool {
        CombinationIterator::new(
            &[Operator::Add, Operator::Multiply],
            (self.parts.len() - 1) as u32,
        )
        .map(|o| self.calculate(o))
        .collect::<Vec<_>>()
        .contains(&self.result)
    }
}

impl From<&str> for CalibrationEquation {
    fn from(value: &str) -> Self {
        let (first, second) = value.split_once(':').unwrap();

        Self {
            result: first.parse().unwrap(),
            parts: second
                .trim()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Calibrator {
    equations: Vec<CalibrationEquation>,
}

impl Calibrator {
    pub fn get_valid_sum(&self) -> u64 {
        self.equations
            .iter()
            .filter(|v| v.can_be_valid())
            .map(|v| v.result)
            .sum()
    }
}

impl From<&str> for Calibrator {
    fn from(value: &str) -> Self {
        Self {
            equations: value.lines().map(|v| v.into()).collect(),
        }
    }
}

type Data = Calibrator;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test07.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(3749), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_valid_sum())
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0)
    }
}
