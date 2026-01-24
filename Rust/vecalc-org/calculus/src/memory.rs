use std::collections::HashMap;

use super::vector::Vector;

pub struct Scope {
    vectors: HashMap<String, Vector>,
    name: String,
}

impl Scope {
    fn new(name: &String) -> Scope {
        Scope {
            name: name.clone(),
            vectors: HashMap::new(),
        }
    }

    pub fn get(&self, vector_name: &String) -> Option<&Vector> {
        self.vectors.get(vector_name)
    }

    pub fn define_vector(&mut self, name: String, vector_components: Vec<f64>) {
        self.vectors
            .entry(name.to_string())
            .and_modify(|v| v.update(vector_components.clone()))
            .or_insert(Vector::new(&name, vector_components));
    }
}

pub struct Memory {
    scopes: HashMap<String, Scope>,
}

impl Memory {
    pub fn create() -> Memory {
        Memory {
            scopes: HashMap::new(),
        }
    }

    pub fn get(&mut self, scope: &String) -> &mut Scope {
        self.scopes
            .entry(scope.clone())
            .or_insert(Scope::new(&scope))
    }
}
