use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
    let message = String::from(
        "Hello, World! Edited form HX! How ong can a message be before the format starts looking weird?",
    );
    let stdout = stdout();
    let width = message.len();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
