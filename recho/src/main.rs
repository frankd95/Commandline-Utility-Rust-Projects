// recho - this is a very simple version of the echo command in unix made in rust. 
//This is a part of a collection of projects of me re-creating common unix command tools in rust to learn the language. 

//This program will function like echo, `echo "Hello, World!"` will out put whats in " "

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let output = &args[1];
    let trimmed = output.trim_matches('"');

    print!("{}",trimmed);

}