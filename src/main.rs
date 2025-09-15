use std::env;
use std::process;

mod object;
mod object_finder;
mod head;
mod command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        process::exit(1);
    }
    if args[1] == "log" {
        command::log::log(&args);
    }
    if args[1] == "checkout" {
        command::checkout::checkout(&args);
    }
}
