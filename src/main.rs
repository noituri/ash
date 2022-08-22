mod cli;
mod code;
mod failure;

fn main() {
    if let Err(why) = cli::init() {
        eprintln!("Error occurred: {why}")
    }
}
