// QUERY PARSING MODULE

use crate::{
    bplus,
    pager,
};


// unit struct
pub struct Parser {
    tree : Option<bplus::Tree>,
}

impl Parser {

    pub fn new() -> Parser {
        Parser { 
            tree : None,
            //tree : bplus::Tree::new(4, 32, "test".to_string()) 
        }
    }

    // master parse function
    pub fn parse(&mut self, query : String) {
        
        // tokenise query into a vector for iteration
        let strvec : Vec<&str> = query.split(' ').collect();

        match strvec[0] {
            


            "insert" => {
                let key = strvec[1].parse::<u32>();
                if strvec.len() == 3 && key.is_ok() 
                    {
                        if let Some(ref mut t) = &mut self.tree {
                            t.insert(key.unwrap(), String::from(strvec[2]));
                        }
                        else {
                            self.tree = Some(bplus::Tree::new(4, key.unwrap(), String::from(strvec[2])))
                        }
                    }
                else { println!("failed to insert.") }
            },
            


            "create" => {
                if strvec[1] == "table" {
                    let new_query = query.replace("create table", "");
                    let column_vec : Vec<&str> = new_query.split(',').collect();
                    for column in column_vec {
                        let word_vec : Vec<&str> = column.trim().split(' ').collect();
                        if word_vec.len() == 2 {
                            let n = word_vec[0];
                            let t = word_vec[1];
                            match t {
                                "int" => {
                                    let column = pager::Column {
                                        name : n.to_string(),
                                        dtype : pager::DataType::INT,
                                    };
                                },
                                _ if t.starts_with("char") => {
                                    let start_bytes = t.find("(").unwrap();
                                    let end_bytes = t.find(")").unwrap();
                                    let size = t[start_bytes + 1..end_bytes].parse::<u32>().unwrap();
                                    let column = pager::Column {
                                        name : n.to_string(),
                                        dtype : pager::DataType::CHAR(size),
                                    };
                                },
                                &_ => {println!("poor syntax")},
                            }
                        }
                    }
                }
            },
            


            &_ => println!("not understood."),
        


        }
    }

}
