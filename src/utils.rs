use std::io::stdin;

pub fn readln() -> String {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    String::from(input.trim())
}
