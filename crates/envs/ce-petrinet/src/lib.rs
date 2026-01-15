use ce_core::{Env, Generate, ValidationResult, define_env, rand};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use stdx::stringify::Stringify;

mod parser;
use parser::parse_pcommands;

mod dot_generator;
use dot_generator::dot;


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

use std::error::Error;
use std::fmt;

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
            } => write!(
                f,
                "{from} -> {to} ({amount}) ({to_transition}, {from_transition});"
            ),
            PCommand::Token(place, n) => write!(f, "{place}{stars};", stars = "*".repeat(*n)),
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
}

#[derive(tapi::Tapi, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub dot: String,
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

        Ok(Output { dot: dot(parsed) })
    }

    fn validate(_input: &Self::Input, _output: &Self::Output) -> ce_core::Result<ValidationResult> {
        Ok(ValidationResult::Correct)
    }
}

impl Generate for Input {
    type Context = ();

    fn gn<R: rand::Rng>(_cx: &mut Self::Context, _rng: &mut R) -> Self {
        Self {
            commands: Stringify::new(PCommands(vec![])),
        }
    }
}
