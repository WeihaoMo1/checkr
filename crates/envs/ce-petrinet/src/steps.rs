use crate::{PCommand, PCommands};
use std::{
    collections::{HashMap},
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
        let mut transition_outputs: HashMap<String, Vec<(String, usize)>> = HashMap::new();

        for pcmd in &pcmds.0 {
            if let PCommand::Connection {
                from,
                to,
                amount,
                from_transition,
                to_transition,
            } = pcmd
            {
                let cost = amount.parse::<usize>().unwrap_or(0);

                if *to_transition {
                    transition_inputs
                        .entry(to.clone())
                        .or_default()
                        .push((from.clone(), cost));
                }

                if *from_transition {
                    transition_outputs
                        .entry(from.clone())
                        .or_default()
                        .push((to.clone(), cost));
                }

                if *to_transition {
                    transition_inputs.entry(to.clone()).or_default();
                    transition_outputs.entry(to.clone()).or_default();
                }

                if *from_transition {
                    transition_inputs.entry(from.clone()).or_default();
                    transition_outputs.entry(from.clone()).or_default();
                }
            }
        }

        println!("Input: {:?}", transition_inputs);
        println!("Output: {:?}", transition_outputs);

        let mut fire_list = Vec::new();

        for (transition, places) in transition_inputs {
            let can_fire = {
                let mut needed = HashMap::new();

                for (place, cost) in &places {
                    *needed.entry(place).or_insert(0) += *cost;
                }

                needed
                    .into_iter()
                    .all(|(place, total)| tokens.get(place).copied().unwrap_or(0) >= total)
            };

            if can_fire {
                for (place, cost) in &places {
                    *tokens.get_mut(place).unwrap() -= *cost;
                }
                fire_list.push(transition);
            }
        }

        for transition in fire_list {
            for (to, produced) in &transition_outputs[&transition] {
                *tokens.entry(to.clone()).or_insert(0) += *produced;
            }
        }
    }

    let mut new_cmds = Vec::new();

    for pcmd in &pcmds.0 {
        if let PCommand::Connection { .. } = pcmd {
            new_cmds.push(pcmd.clone());
        }
    }

    for (place, amount) in tokens {
        if amount > 0 {
            new_cmds.push(PCommand::Token(place, amount));
        }
    }

    PCommands(new_cmds)
}
