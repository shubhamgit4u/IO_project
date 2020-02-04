use std::env;
use std::fs;
use io_project::config;
//use ansi_term:: Colour::Red;
fn main() {
    let args:Vec<String> = env::args().collect();
    let config = config::logic(&args);
   // let red_string = Red.paint("a red string").to_string();
    println!("Searching{}",config.query);
    println!("in file name {}",config.file_name);
    let content = fs::read_to_string(config.file_name).expect("something wrong while rwading file");
    println!("content:-{}",content);
   config::run();
    
}
