use ferris_says::say;
use std::io::{stdout, BufWriter};

fn greet() -> String {
    String::from("Hello, world!")
}

fn main() {
    let stdout = stdout();
    let message = greet();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, world!");
    }
}