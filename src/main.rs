use std::env;

// this is a comment to force a change
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
