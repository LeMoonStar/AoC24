use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 2;

pub enum Direction {
    Increasing,
    Decreasing,
}

#[derive(Clone, Debug)]
pub struct Report(Vec<i8>);

impl Report {
    pub fn is_safe(&self, mut tolerance: usize) -> bool {
        let mut direction = None;
        let mut skip = false;

        for i in 1..self.0.len() {
            if skip {
                skip = false;
                continue;
            }

            if !Self::safe_comparison(self.0[i - 1], self.0[i], &mut direction) {
                if tolerance > 0 && i > 1 {
                    tolerance -= 1;
                    if !Self::safe_comparison(self.0[i - 2], self.0[i], &mut direction) {
                        skip = true;
                        continue;
                    }
                }

                return false;
            }
        }

        true
    }

    fn safe_comparison(a: i8, b: i8, direction: &mut Option<Direction>) -> bool {
        match (b - a, &direction) {
            /* Over limits */
            (..=-4 | 4.., _) => return false,

            /* Decide on direction */
            (..=-1, None) => *direction = Some(Direction::Decreasing),
            (1.., None) => *direction = Some(Direction::Increasing),

            /* Wrong Direction */
            (1.., Some(Direction::Decreasing)) => return false,
            (..=-1, Some(Direction::Increasing)) => return false,

            /* No Change */
            (0, _) => return false,
            _ => {}
        }

        return true;
    }
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        Self(
            value
                .split_whitespace()
                .map(|v| v.parse::<i8>().expect("error while parsing input."))
                .collect(),
        )
    }
}

type Data = Vec<Report>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test02.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(2), Answer::Number(4))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.lines().map(|l| l.into()).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut safe_count = 0;
        for report in data {
            if report.is_safe(0) {
                safe_count += 1;
            }
        }

        return Answer::Number(safe_count);
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut safe_count = 0;
        for report in data {
            if report.is_safe(1) {
                safe_count += 1;
            }
        }

        return Answer::Number(safe_count);
    }
}
