use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 2;

pub enum Direction {
    Increasing,
    Decreasing,
}

#[derive(Clone, Debug)]
pub struct Report(Vec<i8>);

impl Report {
    pub fn is_safe(&self, tolerance: usize) -> bool {
        let mut direction = None;
        let mut failures = 0;
        let mut skip_last = false;

        for i in 1..self.0.len() {
            match (
                self.0[i]
                    - self.0[i - if skip_last {
                        skip_last = false;
                        2
                    } else {
                        1
                    }],
                &direction,
            ) {
                /* Over limits */
                (..=-4 | 4.., _) => {
                    failures += 1;
                    skip_last = true;
                }

                /* Decide on direction */
                (..=-1, None) => direction = Some(Direction::Decreasing),
                (1.., None) => direction = Some(Direction::Increasing),

                /* Wrong Direction */
                (1.., Some(Direction::Decreasing)) => {
                    failures += 1;
                    skip_last = true;
                }
                (..=-1, Some(Direction::Increasing)) => {
                    failures += 1;
                    skip_last = true;
                }

                /* No Change */
                (0, _) => {
                    failures += 1;
                    skip_last = true;
                }
                _ => {}
            }

            if failures > tolerance {
                return false;
            }

            if skip_last {
                println!("SKIPPING {}", i);
            }
        }
        true
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
            println!("{:?}", report);
            if report.is_safe(0) {
                safe_count += 1;
            }
        }

        return Answer::Number(safe_count);
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut safe_count = 0;
        for report in data {
            println!("{:?}", report);
            if report.is_safe(1) {
                println!("Safe!");
                safe_count += 1;
            }
        }

        return Answer::Number(safe_count);
    }
}
