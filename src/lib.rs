use std::fs;
use std::env;
//extern crate ansi_term;
//use ansi_term:: Colour::Red;
pub struct config{
   pub query:String,
    pub file_name:String,
}
impl  config{
    pub fn logic(args:&[String])->config{
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
        println!("lenghth {}",args.len());
        
        config {query , file_name} 
    }
    pub fn run(){
        let args: Vec<String> = env::args().collect();
        let file_name= &args[2];
        let query= &args[1];
         let mut counter =1;
        let contents = fs::read_to_string(file_name).expect("something wrong while rwading file");
        for line in contents.lines(){
            
            if line.contains(query)
            {
            println!("Line which contain {} is =---{}--- at no. {} line",query,line,counter);
            }
            counter +=1;
            }
        } 
    
}