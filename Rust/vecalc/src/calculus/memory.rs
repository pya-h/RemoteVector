use std::collections::HashMap;

use super::vector::vector::Vector;

pub struct Scope {
    vectors: HashMap<String, Vector>,
    name: String
}

impl Scope {
    fn get(name: &String)-> Scope {
        Scope {name: name.clone(), vectors: HashMap::new()}
    }
}

pub struct Memory {
    scopes: HashMap<String, Scope>
}

impl Memory {
    fn create() -> Memory {
        Memory {scopes: HashMap::new()}
    }
}