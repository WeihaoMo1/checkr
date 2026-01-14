use ce_core::{Env, Generate, ValidationResult, define_env, rand};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use stdx::stringify::Stringify;

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
    Connection(String, String, String),
    Token(String, usize),
}

impl Display for PCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PCommand::Connection(from, to, transition) => write!(f, "{from} -> {to} [{transition}];"),
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

pub fn parse_pcommands(src: &str) -> Result<PCommands, ParseError> {
    let trimmed = src.trim();

    if !trimmed.ends_with(';') {
        return Err(ParseError::new(format!("Invalid command: '{trimmed}'")));
    }

    let mut pcmds = Vec::new();

    trimmed
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .try_for_each(|pcmd| {
            if let Some((a, rest)) = pcmd.split_once("->") {
                let a = a.trim();

                let (b, bracket_part) = rest.trim()
                    .rsplit_once('[')
                    .ok_or_else(|| {
                        ParseError::new(format!("Expected '[]' after target: '{pcmd}'"))
                    })?;

                let b = b.trim();

                let t = bracket_part
                    .strip_suffix(']')
                    .ok_or_else(|| {
                        ParseError::new(format!("Unclosed bracket in: '{pcmd}'"))
                    })?
                    .trim();

                
                if a.is_empty() || b.is_empty() || a.contains("->") || b.contains("->") {
                    return Err(ParseError::new(format!("Invalid connection: '{pcmd}'")));
                }
                if a.contains("*") || b.contains("*") {
                    return Err(ParseError::new(format!(
                        "Invalid connection syntax: '{pcmd}'"
                    )));
                }

                pcmds.push(PCommand::Connection(a.to_string(), b.to_string(), t.to_string()));
                return Ok(());
            }

            let mut chars = pcmd.chars();
            let mut place = String::new();

            while let Some(c) = chars.next() {
                if c.is_alphanumeric() {
                    place.push(c);
                } else {
                    let stars: String = std::iter::once(c).chain(chars).collect();
                    if !stars.chars().all(|c| c == '*') {
                        return Err(ParseError::new(format!("Invalid token syntax: '{pcmd}")));
                    }
                    pcmds.push(PCommand::Token(place, stars.len()));
                    return Ok(());
                }
            }
            Err(ParseError::new(format!("Unrecognized command: '{pcmd}'")))
        })?;
    Ok(PCommands(pcmds))
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

pub fn dot(pcmds: PCommands) -> String {
    let mut lines = String::new();
    for pcmd in &pcmds.0 {
        match pcmd {
            PCommand::Connection(a, b, t) => {
                lines.push_str(&format!(
                    "  {a:?}[label=\"{a}\"]; {a:?} -> {b:?} [label=\"{t}\"]; {b:?}[label=\"{b}\"];\n"
                ));
            }
            PCommand::Token(place, n) => {
                lines = lines.replace(
                    &format!("[label=\"{place}\"]"),
                    &format!("[label=\"{place}{}\"]", "*".repeat(*n)),
                );
            }
        }
    }
    format!("digraph G {{\n{}}}", lines)
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
