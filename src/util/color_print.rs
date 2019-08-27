// see https://en.wikipedia.org/wiki/ANSI_escape_code

const RED: &str = "\x1B[1;31m";
const BLUE: &str = "\x1B[1;34m";
const CYAN: &str = "\x1B[1;36m";
const GREEN: &str = "\x1B[0;32m";
const YELLOW: &str = "\x1B[0;33m";
const RESET: &str = "\x1B[0;0m";

#[allow(dead_code)]
pub fn red(text: &str) -> String {
    format!("{}{}{}", RED, text, RESET)
}

#[allow(dead_code)]
pub fn blue(text: &str) -> String {
    format!("{}{}{}", BLUE, text, RESET)
}

#[allow(dead_code)]
pub fn cyan(text: &str) -> String {
    format!("{}{}{}", CYAN, text, RESET)
}

#[allow(dead_code)]
pub fn green(text: &str) -> String {
    format!("{}{}{}", GREEN, text, RESET)
}

#[allow(dead_code)]
pub fn yellow(text: &str) -> String {
    format!("{}{}{}", YELLOW, text, RESET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors() {
        println!("Here is some {} text.", red("red colored"));
        println!("Here is some {} text.", blue("blue colored"));
        println!("Here is some {} text.", cyan("cyan colored"));
        println!("Here is some {} text.", green("green colored"));
    }
}
