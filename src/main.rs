use std::env;
use std::process;

mod git;
mod object_finder;
mod command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        process::exit(1);
    }
    if args[1] == "log" {
        command::log::log(&args);
    } else if args[1] == "checkout" {
        command::checkout::checkout(&args);
    } else if args[1] == "status" {
        command::status::status(&args);
    } else if args[1] == "read-index" {
        command::read_index::read_index(&args);
    } else if args[1] == "read-tree" {
        command::read_tree::read_tree(&args);
    } else {
        println!("Unknown command: {}", args[1]);
        process::exit(1);
    }
}
