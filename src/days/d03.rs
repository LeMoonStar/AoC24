use super::{Answer, Day, DayImpl};
use regex::Regex;

const CURRENT_DAY: u8 = 3;

#[derive(Clone, Debug)]
pub struct Command {
    command: String,
    parameters: Vec<u64>,
}

impl Command {
    pub fn run(&self) -> u64 {
        if self.command != "mul" || self.parameters.len() < 2 {
            return 0;
        }
        self.parameters[0] * self.parameters[1]
    }
}

#[derive(Clone, Debug)]
pub struct Program(Vec<Command>);

impl Program {
    pub fn do_all_multiplications(&self) -> u64 {
        self.0.iter().map(|v| v.run()).sum()
    }

    pub fn run(&self) -> u64 {
        let mut enabled = true;

        self.0
            .iter()
            .map(|v| {
                if v.command == "don't" {
                    enabled = false;
                } else if v.command == "do" {
                    enabled = true;
                } else if enabled {
                    return v.run();
                }
                0
            })
            .sum()
    }
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let pattern: Regex = Regex::new(r"(do|don't|mul)\(((?:\d+,?)*)\)").unwrap();

        let mut commands = vec![];
        for (_, [command, parameters]) in pattern.captures_iter(value).map(|c| c.extract()) {
            commands.push(Command {
                command: command.to_string(),
                parameters: parameters
                    .split(",")
                    .map(|v| v.parse().unwrap_or(0))
                    .collect(),
            });
        }

        Self(commands)
    }
}

type Data = Program;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test03.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(161), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.do_all_multiplications())
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.run())
    }
}
