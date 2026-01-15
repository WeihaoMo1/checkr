use crate::{PCommand, PCommands};
use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub fn steps(pcmds: PCommands, steps: usize) -> PCommands {
    let pcmds = pcmds.clone();

    let mut tokens: HashMap<String, usize> = pcmds
        .0
        .iter()
        .filter_map(|pcmd| {
            if let PCommand::Token(place, amount) = pcmd {
                Some((place.clone(), *amount))
            } else {
                None
            }
        })
        .collect();

    for _ in 0..steps {
        let mut transition_inputs: HashMap<String, Vec<(String, usize)>> = HashMap::new();

        for pcmd in &pcmds.0 {
            if let PCommand::Connection {
                from,
                to,
                amount,
                to_transition,
                ..
            } = pcmd
            {
                if *to_transition {
                    let cost = amount.parse::<usize>().unwrap_or(0);
                    transition_inputs
                        .entry(to.clone())
                        .or_default()
                        .push((from.clone(), cost));
                }
            }
        }

        let mut fire_places: Vec<(String, Vec<(String, usize)>)> = Vec::new();
        let mut reserved: HashMap<String, usize> = HashMap::new();

        for (transition, places) in transition_inputs {
            let can_fire = places.iter().all(|(place, cost)| {
                let available = tokens.get(place).copied().unwrap_or(0);
                let already_reserved = reserved.get(place).copied().unwrap_or(0);
                available >= already_reserved + *cost
            });

            if can_fire {
                for (place, cost) in &places {
                    *reserved.entry(place.clone()).or_insert(0) += *cost;
                }
                fire_places.push((transition, places));
            }
        }

        for (_, places) in &fire_places {
            for (place, cost) in places {
                if let Some(t) = tokens.get_mut(place) {
                    *t -= *cost;
                }
            }
        }

        for (transition, _) in &fire_places {
            for pcmd in &pcmds.0 {
                if let PCommand::Connection {
                    from,
                    to,
                    amount,
                    from_transition,
                    ..
                } = pcmd
                {
                    if *from_transition && from == transition {
                        let produced = amount.parse::<usize>().unwrap_or(0);
                        *tokens.entry(to.clone()).or_insert(0) += produced;
                    }
                }
            }
        }
    }

    let mut new_cmds = Vec::new();

    for pcmd in pcmds.0 {
        match pcmd {
            PCommand::Token(place, _) => {
                if let Some(&t) = tokens.get(&place) {
                    new_cmds.push(PCommand::Token(place, t));
                }
            }
            other => new_cmds.push(other),
        }
    }

    let existing_places: HashSet<String> = new_cmds
        .iter()
        .filter_map(|cmd| {
            if let PCommand::Token(place, _) = cmd {
                Some(place.clone())
            } else {
                None
            }
        })
        .collect();

    for (place, amount) in tokens {
        if !existing_places.contains(&place) {
            new_cmds.push(PCommand::Token(place, amount));
        }
    }

    new_cmds.retain(|cmd| match cmd {
        PCommand::Token(_, amount) => *amount > 0,
        _ => true,
    });

    PCommands(new_cmds)
}
