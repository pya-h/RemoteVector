use std::collections::HashMap;

use super::vector::Vector;
use super::matrix::Matrix;

pub enum Token {
    Vector(Vector),
    Matrix(Matrix),
    Scalar(f64),
    Operator(String),
    Null,
    Wtf(String),
}

pub struct Scope {
    members: HashMap<String, Token>,
    name: String,
}

impl Scope {
    fn new(name: &String) -> Scope {
        Scope {
            name: name.clone(),
            members: HashMap::new(),
        }
    }

    pub fn get(&self, name: &String) -> Option<&Token> {
        &self.members.get(name)
    }

    pub fn get_vec(&self, vector_name: &String) -> Option<&Vector> {
        if let Some(v) = self.get(vector_name) {
            if let Token::Vector(v) = v {
                return v;
            }
        }
        None
    }

    pub fn get_mat(&self, matrix_name: &String) -> Option<&Matrix> {
        if let Some(mat) = self.get(matrix_name) {
            if let Token::Matrix(mat) = mat {
                return mat;
            }
        }
        None
    }

    
    pub fn define_vector(&mut self, name: String, vector_components: Vec<f64>) {
        self.members
            .entry(name.to_string())
            .and_modify(|v| v.update(vector_components.clone()))
            .or_insert(Token::Vector(Vector::new(&name, vector_components)));
    }

    pub fn define_matrix(&mut self, name: String, column_vecs: Vec<Vector>) {
        self.members
            .entry(name.to_string())
            .and_modify(|v| v.update(column_vecs.clone()))
            .or_insert(Token::Matrix(Matrix::new(&name, column_vecs)));
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
