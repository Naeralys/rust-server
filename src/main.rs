mod server;
mod client;

use std::{env};

struct RunArgs;
impl RunArgs {
    const CLIENT: &str = "client";
    const SERVER: &str = "server";
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Args cannot be null | client or server");
    }

    println!("Starting service with: {}", &args[1]);
    
    match String::from(&args[1]).as_str() {
        RunArgs::CLIENT => client::run(),
        RunArgs::SERVER => server::run(),
        _ => println!("Args must be either client or server")
    }
}
