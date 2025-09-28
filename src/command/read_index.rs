use crate::git::index;

pub fn read_index(args: &Vec<String>) {
    if args.len() != 2 {
        println!("Usage: {} read-index", args[0]);
        std::process::exit(1);
    }
    let index = index::Index::read_index();
    println!("{:?}", index);
}