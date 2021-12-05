use std::collections::HashMap;
use std::env;

mod one;
mod two;
mod three;
mod four;
mod five;

fn main() {
    let args: Vec<String> = env::args().collect();
    let challenge = args
        .get(1)
        .expect("Add the challenge name e.g. 'one' after the command");

    let challenges = HashMap::from([
        ("1", one::challenge_one as fn()),
        ("2", two::challenge_two as fn()),
        ("3", three::challenge_three as fn()),
        ("4", four::challenge_four as fn()),
        ("5", five::challenge_five as fn())
    ]);

    let challenge_fn = challenges
        .get(challenge.as_str())
        .expect(&format!("Unknown event name '{}'", &challenge));

    challenge_fn();
}
