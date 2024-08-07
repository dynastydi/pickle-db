const PAGE_SIZE : u32 = 4096;

pub enum DataType {
    INT, 
    CHAR(u32),
}

pub struct Column {
    pub name : String,
    pub dtype : DataType,
}

struct Table {
    name : String,
    cols : Vec<Column>,
}

pub struct Pager;

impl Pager {
    pub fn create_table(&mut self, name : &str, cols : Vec<Column>) {

        //let table = Table

    }


}
