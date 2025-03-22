use std::{collections::HashMap };

struct LocalStorage{
    m: HashMap<String, String>
}

impl LocalStorage{
    pub fn new() -> Self{
        Self {m: HashMap::new()}
    }    
}


