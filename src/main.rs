use std::env;

// this is a comment to force a change
// this is a second comment just to introduce a change into the source code
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
