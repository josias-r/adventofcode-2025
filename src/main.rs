use ferris_says::say;
use std::io::{BufWriter, stdout};

mod day_01;
mod day_02;

fn main() {
    let message = String::from(
        "Hello, World! Edited from HX! How long can a message be before the format starts looking weird?",
    );
    let stdout = stdout();
    let width = message.len();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
    day_01::solve().unwrap();

    day_02::solve();
}
