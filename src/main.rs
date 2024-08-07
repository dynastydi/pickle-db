use std::io::{
    stdin,
    stdout,
    Write,
};

mod queries;
mod bplus;
mod pager;

fn main() {
    
    let mut parser = queries::Parser::new();
   
    let mut query : String;
    

    query = "create table test ( id int, name char(8) )".to_string();
    
    parser.parse(query);


    /*
    loop {
        
        // set up input
        print!("pickle > ");
        stdout().flush().unwrap();

        // accept input
        query = String::new();
        stdin().read_line(&mut query).unwrap();
        query = query.to_lowercase();
        
        if query.starts_with('.') {
            // parse input
            match query.to_lowercase().trim() {
                ".q" | ".quit" => break,
                ".h" | ".help" => println!("this would be the help page!"),
                ".b" => { 
                    //tree.add(8, "test".to_string());
                    //println!("success!");
                }

                &_ => println!("not understood."),
            }
        } else {
            parser.parse(query);
        }

    }
    */
    println!("quitting...")
}
