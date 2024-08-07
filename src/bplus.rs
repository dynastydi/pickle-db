// TODO - change to indexing after pager implementation.

#[derive(Clone, PartialEq)]
pub struct KVP {
    key : u32,
    val : String,
}

#[derive(Clone, PartialEq)]
struct LeafStruct {
    payload : Vec<KVP>,
    link : Option<usize>,
}

#[derive(Clone, PartialEq)]
struct BranchStruct {
    keys : Vec<u32>,
    children : Vec<Box<Node>>,
}

#[derive(Clone, PartialEq)]
enum Node {
    Branch(BranchStruct),
    Leaf(LeafStruct),
}

impl Node {
    fn search<'a>(&'a mut self, key : u32) -> &mut Node {
        
        if let Node::Branch(ref mut branch_struct) = self {
            
            let children = &mut branch_struct.children;

            let keys = &branch_struct.keys;
            
            for i in 0..keys.len()-1 {
                if key <= keys[i] {
                    return &mut *children[i].search(key)
                }
            }
            // no if-check if got this far.
            return &mut *children[keys.len()].search(key)
        }
        
        // if leaf is found, return it
        else { return self }

    }
}

pub struct Tree {
    m : usize,
    root : Node,
}

impl Tree {
    pub fn new(m : usize, key : u32, val : String) -> Tree {
        let payload = vec![KVP {key, val}];
        let root = Node::Leaf(LeafStruct {
            payload : payload,
            link : None,
        });
        println!("new tree with root {}", key);
        Tree {m, root}
    }

    pub fn insert(&mut self, key : u32, val : String) {
        //let mut kvp = KVP { key, val };
        // check root capacity
        let root_clone = self.root.clone();
        match self.root {
            Node::Leaf(ref mut leaf_struct) => {
                let leaf_clone = leaf_struct.clone();
                let len = leaf_clone.payload.len();

                if len == self.m {
                    println!("time to split root");
                    // create two new leaves:
                    // A has link to B and first half of kvps
                    // B has link to original's link and rest of kvps
                    let mut load_a = leaf_clone.payload;
                    let load_b = load_a.split_off(len - (len / 2));

                    let mut splitter = load_a.last().unwrap().key;

                    let child_b = Box::new(Node::Leaf(LeafStruct {
                        payload : load_b,
                        link : leaf_clone.link,
                    }));

                    let child_a = Box::new(Node::Leaf(LeafStruct {
                        payload : load_a,
                        link : None,//Some(child_b),
                    }));


                    self.root = Node::Branch(BranchStruct {
                        keys : vec!(splitter),
                        children : vec!(child_a, child_b),
                    });
                    // new root is a branch
                    // one key - the last of A
                    // two children, A & B

                }
            },
            Node::Branch(ref mut branch_struct) => {
                let branch_clone = branch_struct.clone();
                let len = branch_clone.children.len();

                if len == self.m {
                    println!("time to split root");
                    // create two new branches:
                    // A has first half of children
                    // B has second half of children
                    let mut children_a = branch_clone.children;
                    let mut children_b = children_a.split_off(len - (len / 2));
                    
                    let mut keys_a = branch_clone.keys;
                    let mut keys_b = keys_a.split_off(len - (len / 2));
                    
                    let mut splitter = keys_a.pop().unwrap();

                    let child_a = Box::new(Node::Branch(BranchStruct {
                        keys : keys_a,
                        children : children_a,
                    }));

                    let child_b = Box::new(Node::Branch(BranchStruct {
                        keys : keys_b,
                        children : children_b,
                    }));

                    self.root = Node::Branch(BranchStruct {
                        keys : vec!(splitter),
                        children : vec!(child_a, child_b),
                    });
                }
            },
        }


        let mut node = self.root.search(key);
        
        
        if let Node::Leaf(ref mut leaf_struct) = node {
            let payload = leaf_struct.payload.clone();
            let pos = payload.iter().position(|x| x.key == key);
            // if key is in vector
            if let Some(index) = pos {
                leaf_struct.payload[index].val = val;
                println!("reassigned {}", key);
            }
            
            else { 
                leaf_struct.payload.push(KVP {key, val});
                println!("inserted {}", key);
            }
            
            //else if payload.len() == m - 1 {
            // split (put [m/2:] in another)
            // if parent exists:
            //  if parent is not full:
            //      copy last key of leaf A to parent
            //      make leaves A & B children of parent
            //  if parent is full:
            //      recur
            //  if no parent:
            //      create
            //}
        }
            
        else { panic!() }
        
    }
}

/*
// final data-holding node
struct Leaf {
    content : Vec<KVP>,         // keys & values
    next : Option<Box<Leaf>>,   // optional next link
}

// internal traversal node
struct Branch {
    // TODO - avoid empty pointers?
    keys : Vec<usize>,          // vector of keys
    branches : Option<Vec<Branch>>,     // optional vector of child branches
    leaves : Option<Vec<Leaf>>,         // vector of child
}

*/

