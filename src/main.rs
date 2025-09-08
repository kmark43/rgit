use std::env;
use std::process;

mod commit;
mod object_finder;
mod head;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <hash>", args[0]);
        process::exit(1);
    }
    if args[1] == "log" {
        if args.len() != 2 {
            println!("Usage: {} log", args[0]);
            process::exit(1);
        }
        let head = head::Head::from_head();
        println!("Head: {}", head.ref_path.display());
        println!("Hash: {}", head.head_hash);
    }
}
