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
                if valid_places.contains(to) && *to_transition
                    || valid_places.contains(from) && *from_transition
                    || valid_transitions.contains(from) && *to_transition
                    || valid_transitions.contains(to) && *from_transition
                {
                    return Err(ParseError::new(
                        "Invalid connection: Transition can not have the same label as a place",
                    ));
                }
                if *to_transition && !*from_transition {
                    lines.push_str(&format!(
                        "{from:?}[label=\"{from}\",group=\"place\"]; {from:?} -> {to:?} [label=\"{amount}\"]; {to:?}[label=\"{to}\",group=\"transition\"];\n"
                    ));
                    valid_places.push(from.clone());
                    valid_transitions.push(to.clone());
                } else if !*to_transition && *from_transition {
                    lines.push_str(&format!(
                            "{from:?}[label=\"{from}\",group=\"transition\"]; {from:?} -> {to:?} [label=\"{amount}\"]; {to:?}[label=\"{to}\",group=\"place\"];\n"
                        ));
                    valid_places.push(to.clone());
                    valid_transitions.push(from.clone());
                } else {
                    return Err(ParseError::new(
                        "Invalid connection: both ends cannot be transitions or places",
                    ));
                }
            }
            PCommand::Token(place, n) => {
                if valid_places.contains(place) {
                    lines = lines.replace(
                        &format!("[label=\"{place}\",group=\"place\"]"),
                        &format!("[label=\"{place}\n\n{}\",group=\"place\"]", *n),
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
