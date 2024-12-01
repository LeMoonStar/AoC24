use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 1;

type Data = Vec<[u64; 2]>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test01.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(0), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| {
                    v.split_whitespace()
                        .map(|v| v.trim().parse::<u64>().expect("Error during parsing"))
                        .collect::<Vec<u64>>()
                        .try_into()
                        .expect("Error during parsing")
                })
                .collect::<Vec<[u64; 2]>>(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut left = data.iter().map(|v| v[0]).collect::<Vec<u64>>();
        let mut right = data.iter().map(|v| v[1]).collect::<Vec<u64>>();

        left.sort();
        right.sort();

        Answer::Number(
            left.into_iter()
                .zip(right.into_iter())
                .map(|(a, b)| a.abs_diff(b))
                .sum(),
        )
    }

    fn two(&self, data: &mut Data) -> Answer {
        let left = data.iter().map(|v| v[0]).collect::<Vec<u64>>();
        let right = data.iter().map(|v| v[1]).collect::<Vec<u64>>();

        Answer::Number(
            left.iter()
                .map(|v| v * right.iter().filter(|n| *n == v).count() as u64)
                .sum(),
        )
    }
}
