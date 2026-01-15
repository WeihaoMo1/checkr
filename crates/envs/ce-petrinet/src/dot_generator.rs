use crate::{PCommand, PCommands};

pub fn dot(pcmds: PCommands) -> String {
    let mut lines = String::new();
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
                } else {
                    lines.push_str(&format!(
                        "{from:?}[label=\"{from}\",group=\"transition\"]; {from:?} -> {to:?} [label=\"{amount}\"]; {to:?}[label=\"{to}\"];\n"
                    ));
                }
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