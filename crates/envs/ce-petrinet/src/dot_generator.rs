use crate::{PCommand, PCommands, ParseError};

pub fn dot(pcmds: PCommands) -> Result<String, ParseError> {
    let mut lines = String::new();
    let mut valid_transitions: Vec<String> = Vec::new();
    let mut valid_places: Vec<String> = Vec::new();
    for pcmd in &pcmds.0 {
        match pcmd {
            PCommand::Connection {
                from,
                to,
                amount,
                to_transition,
                from_transition,
            } => {
                if *to_transition && !*from_transition {
                    lines.push_str(&format!(
                        "{from:?}[label=\"{from}\"]; {from:?} -> {to:?} [label=\"{amount}\"]; {to:?}[label=\"{to}\",group=\"transition\"];\n"
                    ));
                    valid_places.push(from.clone());
                    valid_transitions.push(to.clone());
                } else if !*to_transition && *from_transition {
                    if valid_transitions.contains(from) {
                        lines.push_str(&format!(
                            "{from:?}[label=\"{from}\",group=\"transition\"]; {from:?} -> {to:?} [label=\"{amount}\"]; {to:?}[label=\"{to}\"];\n"
                        ));
                    } else {
                        return Err(ParseError::new(format!(
                            "Transition '{from}' doesn't have an entry"
                        )));
                    }
                } else {
                    return Err(ParseError::new(
                        "Invalid connection: both ends cannot be transitions",
                    ));
                }
            }
            PCommand::Token(place, n) => {
                if valid_places.contains(place) {
                    lines = lines.replace(
                        &format!("[label=\"{place}\"]"),
                        &format!("[label=\"{place}{}\"]", "*".repeat(*n)),
                    );
                } else {
                    return Err(ParseError::new(format!(
                        "Place '{place}' doesn't exist in the petrinet"
                    )));
                }
            }
        }
    }
    Ok(format!("digraph G {{\n{}}}", lines))
}
