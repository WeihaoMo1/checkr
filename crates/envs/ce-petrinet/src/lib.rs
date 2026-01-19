use ce_core::{Env, Generate, ValidationResult, define_env, rand};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use stdx::stringify::Stringify;

mod parser;
use parser::parse_pcommands;

mod dot_generator;
use dot_generator::dot;

mod steps;
use steps::steps;

define_env!(PetrinetEnv);

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {}

#[derive(tapi::Tapi, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PCommands(pub Vec<PCommand>);

#[derive(tapi::Tapi, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PCommand {
    Connection {
        from: String,
        to: String,
        amount: String,
        to_transition: bool,
        from_transition: bool,
    },
    Token(String, usize),
}

impl Display for PCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PCommand::Connection {
                from,
                to,
                amount,
                to_transition,
                from_transition,
            } => {
                if *to_transition {
                    write!(f, "{} -> [{}] ({});", from, to, amount)
                } else if *from_transition {
                    write!(f, "[{}] -> {} ({});", from, to, amount)
                } else {
                    // This will not happen(just a placeholder)
                    write!(f, "{} -> {} ({});", from, to, amount)
                }
            }
            PCommand::Token(place, n) => write!(f, "{} ({});", place, *n),
        }
    }
}

impl Display for PCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cmd in &self.0 {
            writeln!(f, "{cmd}")?;
        }
        Ok(())
    }
}

impl FromStr for PCommands {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_pcommands(s)
    }
}

#[derive(tapi::Tapi, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub commands: Stringify<PCommands>,
    pub steps: usize,
}

#[derive(tapi::Tapi, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub dot: String,
    pub map: Vec<HashMap<String, usize>>,
}

impl Env for PetrinetEnv {
    type Input = Input;
    type Output = Output;
    type Meta = ();

    fn run(input: &Self::Input) -> ce_core::Result<Self::Output> {
        let parsed: PCommands =
            input
                .commands
                .try_parse()
                .map_err(ce_core::EnvError::invalid_input_for_program(
                    "failed to parse commands",
                ))?;

        let (map, steps) = steps(parsed, input.steps);

        let dot_str = dot(steps).map_err(ce_core::EnvError::invalid_input_for_program(
            "failed to generate DOT",
        ))?;

        Ok(Output { dot: dot_str, map })
    }

    fn validate(input: &Self::Input, output: &Self::Output) -> ce_core::Result<ValidationResult> {
        let reference = Self::run(input);

        if reference.unwrap().map[0] == output.map[0] {
            Ok(ValidationResult::Correct)
        } else {
            Ok(ValidationResult::Mismatch {
                reason: "Initial state mismatch".to_string(),
            })
        }
    }
}

impl Generate for Input {
    type Context = ();

    fn gn<R: rand::Rng>(_cx: &mut Self::Context, rng: &mut R) -> Self {
        let num_nodes = rng.random_range(3..=6);
        let places: Vec<String> = ('a'..).take(num_nodes).map(String::from).collect();

        let num_transitions = rng.random_range(2..=4);
        let transition_names = ["process", "complete", "check", "send", "receive"];
        let transitions: Vec<String> = transition_names
            .iter()
            .take(num_transitions)
            .map(|s| s.to_string())
            .collect();

        let mut commands = Vec::new();
        let mut used_places = HashSet::new();

        for transition in &transitions {
            let from = places[rng.random_range(0..places.len())].clone();
            let amount = rng.random_range(1..=3).to_string();
            used_places.insert(from.clone());
            commands.push(PCommand::Connection {
                from,
                to: transition.clone(),
                amount,
                to_transition: true,
                from_transition: false,
            });
        }

        for transition in &transitions {
            let to = places[rng.random_range(0..places.len())].clone();
            let amount = rng.random_range(1..=3).to_string();
            used_places.insert(to.clone());
            commands.push(PCommand::Connection {
                from: transition.clone(),
                to,
                amount,
                to_transition: false,
                from_transition: true,
            });
        }

        for _ in 0..rng.random_range(0..=4) {
            if rng.random_range(0..2) == 0 {
                let from = places[rng.random_range(0..places.len())].clone();
                let to = transitions[rng.random_range(0..transitions.len())].clone();
                let amount = rng.random_range(1..=3).to_string();
                used_places.insert(from.clone());
                commands.push(PCommand::Connection {
                    from,
                    to,
                    amount,
                    to_transition: true,
                    from_transition: false,
                });
            } else {
                let from = transitions[rng.random_range(0..transitions.len())].clone();
                let to = places[rng.random_range(0..places.len())].clone();
                let amount = rng.random_range(1..=3).to_string();
                used_places.insert(to.clone());
                commands.push(PCommand::Connection {
                    from,
                    to,
                    amount,
                    to_transition: false,
                    from_transition: true,
                });
            }
        }

        let mut places_for_tokens: Vec<String> = used_places.into_iter().collect();
        places_for_tokens.shuffle(rng);

        let num_tokens = rng.random_range(1..=3.min(places_for_tokens.len()));
        for place in places_for_tokens.iter().take(num_tokens) {
            let token_count = rng.random_range(1..=5);
            commands.push(PCommand::Token(place.clone(), token_count));
        }

        Self {
            commands: Stringify::new(PCommands(commands)),
            steps: 0,
        }
    }
}
