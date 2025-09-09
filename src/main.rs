use std::env;
use std::process;

mod commit;
mod object_finder;
mod head;
mod command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <hash>", args[0]);
        process::exit(1);
    }
    if args[1] == "log" {
        command::log::log(&args);
    }
}
