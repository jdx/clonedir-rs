extern crate clonedir_lib;

use clonedir_lib::clonedir;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("USAGE: clonedir FROM TO");
    }
    clonedir(&args[1], &args[2]).unwrap();
}
