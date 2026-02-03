use std::collections::HashMap;

use super::matrix::Matrix;
use super::vector::Vector;

#[derive(Clone)]
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
        self.members.get(name)
    }

    pub fn get_vec(&self, vector_name: &String) -> Option<&Vector> {
        if let Some(v) = self.get(vector_name) {
            if let Token::Vector(v) = v {
                return Some(v);
            }
        }
        None
    }

    pub fn get_mat(&self, matrix_name: &String) -> Option<&Matrix> {
        if let Some(mat) = self.get(matrix_name) {
            if let Token::Matrix(mat) = mat {
                return Some(mat);
            }
        }
        None
    }

    pub fn evaluate_identifier(&self, token: &str) -> Token {
        if let Ok(v) = token.parse::<f64>() {
            return Token::Scalar(v);
        }
        match self.get(&token.to_string()) {
            Some(v) => v.clone(),
            _ => Token::Null,
        }
    }

    pub fn define_vector(&mut self, name: String, vector_components: Vec<f64>) {
        self.members.insert(
            name.to_string(),
            Token::Vector(Vector::new(&name, vector_components)),
        );
    }

    pub fn define_matrix(&mut self, name: String, column_vecs: Vec<Vector>) {
        self.members.insert(
            name.to_string(),
            Token::Matrix(Matrix::new(&name, column_vecs)),
        );
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
