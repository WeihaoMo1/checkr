use ce_core::{Env, Generate, ValidationResult, define_env, rand::{self}};
use serde::{Deserialize, Serialize};

define_env!(PlaygroundEnv);

#[derive(tapi::Tapi, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub text: String,                   
}

#[derive(tapi::Tapi, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub result: String,                
}


fn capitalize_every_other_word(text: &str) -> String {
    let mut result = String::new();
    let mut current_word = String::new();
    let mut word_count = 0;

    for ch in (text.to_string() + " ").chars() {
        if ch.is_whitespace() {
            if !current_word.is_empty() {
                if word_count % 2 == 1 {
                    result.push_str(&current_word.to_uppercase());
                } else {
                    result.push_str(&current_word);
                }
                current_word.clear();
                word_count += 1;
            }
            result.push(ch);
        } else {
            current_word.push(ch);
        }
    }

    result.pop();
    result
} 

impl Env for PlaygroundEnv {
    type Input = Input;

    type Output = Output;

    type Meta = ();

    fn run(input: &Self::Input) -> ce_core::Result<Self::Output> {
        Ok(Output {                     
            result: capitalize_every_other_word(&input.text),
        })
    }

    fn validate(input: &Self::Input, output: &Self::Output) -> ce_core::Result<ValidationResult> {
        let reference = Self::run(input)?;
        
        Ok(
            match (
                &reference.result,
                &output.result
            ) {
                (r, o) if r == o => ValidationResult::Correct,
                (_, _) => {
                    ValidationResult::Mismatch {
                        reason: format!("Did not produce same as reference."),
                    }
                }
            },
        )
    }
}

impl Generate for Input {
    type Context = ();

    fn gn<R: rand::Rng>(_cx: &mut Self::Context, rng: &mut R) -> Self {
        let words = vec![
            "Hello", "world", "🌎", "This", "is", "my", "input",
            "Rust", "programming", "language", "testing", "example",
            "foo", "bar", "baz", "qux", "😉",
        ];

        let num_words = rng.random_range(3..=8);
     /*    let text = (0..num_words)
            .map(|_| words[rng.random_range(0..words.len())])
            .collect::<Vec<_>>()
            .join(" ");
        Self{ text } */
        let mut text = String::new();
        for i in 0..num_words {
            text.push_str(words[rng.random_range(0..words.len())]);
            
            if i < num_words - 1 {
                let num_spaces = rng.random_range(1..=3);
                for _ in 0..num_spaces {
                    text.push(' ');
                }
                let new_line_space = rng.random_range(0..=1);
                for _ in 0..new_line_space {
                    text.push('\n');
                }
            }
        }
        
        Self { text }
    }
}
 