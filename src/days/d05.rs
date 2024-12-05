use std::collections::BTreeSet;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 5;

// This would be much more efficient as a Map<u64, Vec<u64>>
// But I don't know what Part 2 will require, so I'll keep this.
#[derive(Debug, Clone)]
pub struct PageOrderingRule {
    first: u64,
    second: u64,
}

impl From<&str> for PageOrderingRule {
    fn from(value: &str) -> Self {
        let (first, second) = value.split_once('|').unwrap();

        Self {
            first: first.parse().unwrap(),
            second: second.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdateSequence(Vec<u64>);

impl UpdateSequence {
    fn matches_rules(&self, rules: &Vec<PageOrderingRule>) -> bool {
        let mut contained: BTreeSet<u64> = self.0.iter().map(|v| *v).collect();
        let mut seen = BTreeSet::new();

        for page_number in &self.0 {
            if rules
                .iter()
                .filter(|v| {
                    v.second == *page_number
                        && !seen.contains(&v.first)
                        && contained.contains(&v.first)
                })
                .inspect(|v| println!("{:?}", v))
                .count()
                > 0
            {
                return false;
            }
            seen.insert(page_number);
        }

        true
    }

    fn get_middle_page_number(&self) -> u64 {
        self.0[self.0.len() / 2]
    }
}

impl From<&str> for UpdateSequence {
    fn from(value: &str) -> Self {
        Self(value.split(',').map(|v| v.parse().unwrap()).collect())
    }
}

#[derive(Debug, Clone)]
pub struct SafetyManualPrintOrder {
    rules: Vec<PageOrderingRule>,
    updates: Vec<UpdateSequence>,
}

impl From<&str> for SafetyManualPrintOrder {
    fn from(value: &str) -> Self {
        let (rules_string, updates_string) = value.split_once("\n\n").unwrap();

        Self {
            rules: rules_string.lines().map(|v| v.into()).collect(),
            updates: updates_string.lines().map(|v| v.into()).collect(),
        }
    }
}

type Data = SafetyManualPrintOrder;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test05.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(143), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(
            data.updates
                .iter()
                .filter(|v| v.matches_rules(&data.rules))
                .map(|v| v.get_middle_page_number())
                .sum(),
        )
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0)
    }
}
