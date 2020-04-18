use std::env;

pub fn print_vars() {
    for (key, value) in env::vars() {
        println!("\x1b[32m{}:\x1b[0m {}", key, value);
    }
}
