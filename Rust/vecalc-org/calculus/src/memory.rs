use std::collections::HashMap;

use super::matrix::Matrix;
use super::vector::Vector;

#[derive(Clone, PartialEq)]
pub enum Token {
    Vector(Vector),
    Matrix(Matrix),
    Scalar(f64),
    Operator(String),
    Identifier(String),
    Null,
    Wtf(String),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Vector(v) => v.to_string(),
            Token::Matrix(m) => m.to_string(),
            Token::Null => String::from("()"),
            Token::Scalar(s) => s.to_string(),
            Token::Operator(s) => s.clone(),
            Token::Identifier(s) => s.clone(),
            Token::Wtf(s) => s.clone(),
        }
    }

    pub fn is_valid_term(&self) -> bool {
        match self {
            Token::Wtf(_) | Token::Operator(_) => false,
            _ => true,
        }
    }

    pub fn is_operator(&self) -> bool {
        match self {
            Token::Operator(_) => true,
            _ => false,
        }
    }
}

pub struct Scope {
    pub members: HashMap<String, Token>,
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

    pub fn name(&self) -> &String {
        &self.name
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

    pub fn define(&mut self, name: String, token: Token) {
        self.members.insert(name, token);
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
    pub scopes: HashMap<String, Scope>,
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
