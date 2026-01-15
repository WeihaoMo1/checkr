use crate::ParseError;
use super::PCommand;
use super::PCommands;


pub trait PetrinetParsing {
    fn extract_transition(&self, pcmd: &str) -> Result<&str, ParseError>;
    fn extract_amount(&self, pcmd: &str) -> Result<&str, ParseError>;
    fn separate_target_and_amount(&self, pcmd: &str) -> Result<(&str, &str), ParseError>;
}

impl PetrinetParsing for &str {
    fn extract_transition(&self, pcmd: &str) -> Result<&str, ParseError> {
        let transition = self
            .trim()
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .ok_or_else(|| ParseError::new(format!("OK Unclosed sqr_bracket in: '{pcmd}'")))?;
        Ok(transition.trim())
    }

    fn extract_amount(&self, pcmd: &str) -> Result<&str, ParseError> {
        let amount = self
            .strip_suffix(')')
            .ok_or_else(|| ParseError::new(format!("Unclosed bracket in: '{pcmd}'")))?;
        Ok(amount.trim())
    }

    fn separate_target_and_amount(&self, pcmd: &str) -> Result<(&str, &str), ParseError> {
        let (target, amount) = self
            .trim()
            .rsplit_once('(')
            .ok_or_else(|| ParseError::new(format!("Expected '()' after target: '{pcmd}'")))?;
        Ok((target, amount))
    }
}


fn check_syntax(target: &str, amount: &str, pcmd: &str) -> Result<(), ParseError> {
    if target.is_empty() || amount.is_empty() || target.contains("->") || amount.contains("->") {
        return Err(ParseError::new(format!("Invalid connection: '{pcmd}'")));
    };
    if target.contains("*") || amount.contains("*") {
        return Err(ParseError::new(format!(
            "Invalid connection syntax: '{pcmd}'"
        )));
    };
    Ok(())
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
                if a.contains("[") {
                    let from = a.extract_transition(pcmd)?;

                    let (target, amount) = rest.separate_target_and_amount(pcmd)?;

                    let target = target.trim();
                    let amount = amount.extract_amount(pcmd)?;

                    check_syntax(target, amount, pcmd)?;
                
                    pcmds.push(PCommand::Connection {
                        from: from.to_string(),
                        to: target.to_string(),
                        amount: amount.to_string(),
                        to_transition: false,
                        from_transition: true,
                    });
                } else {
                    let from = a.trim();

                    let (target, amount) = rest.separate_target_and_amount(pcmd)?;

                    let target = target.extract_transition(pcmd)?;

                    let amount = amount.extract_amount(pcmd)?;
                    check_syntax(target, amount, pcmd)?;

                    pcmds.push(PCommand::Connection {
                        from: from.to_string(),
                        to: target.to_string(),
                        amount: amount.to_string(),
                        to_transition: true,
                        from_transition: false,
                    });
                }
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