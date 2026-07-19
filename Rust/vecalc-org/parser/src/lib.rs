extern crate calculus;
use std::thread::scope;

use calculus::matrix::Matrix;
use calculus::memory::Token::Identifier;
use calculus::memory::*;
use calculus::vector::Vector;
use calculus::*;

trait MathOperations {
    // Use this: https://chatgpt.com/share/6737beff-489c-8003-b6fe-34b207ab028e
}

pub struct Analyzer {
    memory: Memory,
}

struct Statement {
    pub left: Vec<Token>,
    pub right: Vec<Token>,
    pub has_equal: bool,
}

impl Statement {
    pub fn new() -> Self {
        Self {
            left: vec![],
            right: vec![],
            has_equal: false,
        }
    }

    pub fn equal_passed(&mut self) {
        self.has_equal = true;
    }

    pub fn next(&mut self, token: Token) {
        if self.has_equal {
            self.right.push(token);
        } else {
            self.left.push(token);
        }
    }

    pub fn finalize(&mut self) {
        if !self.has_equal {
            self.right = std::mem::take(&mut self.left);
        }
    }

    pub fn is_resolved(&self) -> bool {
        self.right.len() == 1 && self.left.len() == 1
    }

    pub fn merge_adjacents(&mut self, i: &mut usize, result: Token, tokens_count: &mut usize) {
        self.right[*i] = result;
        self.right.remove(*i + 1);
        self.right.remove(*i - 1);
        *tokens_count -= 2;
        *i -= 1;
    }

    pub fn at(&self, index: usize, scope: &Scope) -> Token {
        // only will be called when index is valid
        let token = &self.right[index];
        if let Token::Identifier(symbol) = token {
            return scope.evaluate_identifier(&symbol);
        }
        token.clone()
    }
}

impl Analyzer {
    pub fn init() -> Analyzer {
        Analyzer {
            memory: Memory::create(),
        }
    }

    pub fn get(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn analyze(&mut self, instruction: &String, scope: &String) -> String {
        let tokens_as_string: Vec<&str> = instruction.split_whitespace().collect();

        let mut inside_sth: bool = false;
        let mut vector_reading_cache: Vec<f64> = Vec::new();
        let mut matrix_rading_cache: Vec<Vector> = Vec::new();
        let mut statement = Statement::new();
        let scope_memory = self.get().get(scope);

        for tk in tokens_as_string {
            if inside_sth && tk != "]" {
                if matrix_rading_cache.len() > 0 {
                    // TODO: matrix calcs
                } else if vector_reading_cache.len() > 0 {
                    let v = scope_memory.evaluate_identifier(tk);
                    if let Token::Scalar(v) = v {
                        vector_reading_cache.push(v);
                    } else {
                        // Throw type error ...
                    }
                } else {
                    let v_as_token = scope_memory.evaluate_identifier(tk);
                    match v_as_token {
                        Token::Scalar(v) => {
                            vector_reading_cache.push(v);
                        }
                        Token::Vector(v) => {
                            matrix_rading_cache.push(v);
                        }
                        _ => {
                            // throw error.
                        }
                    }
                }
                // TODO: this only support single element extracting; add scalaer math inside vecore/matrix definition...
            } else {
                match tk {
                    "[" => {
                        if inside_sth {
                            // throw error:
                        }
                        inside_sth = true;
                    }
                    "]" => {
                        if inside_sth {
                            if matrix_rading_cache.len() > 0 {
                                let m: Matrix =
                                    Matrix::new(scope_memory.name(), matrix_rading_cache.clone());
                                statement.next(Token::Matrix(m));
                                matrix_rading_cache.clear();
                            } else if vector_reading_cache.len() > 0 {
                                let v: Vector =
                                    Vector::new(scope_memory.name(), vector_reading_cache.clone());
                                statement.next(Token::Vector(v));
                                vector_reading_cache.clear();
                            } else {
                                // throw error: Empty vector (remember to falsify flag)
                            }
                            inside_sth = false;
                        } else {
                            // throw error: Unopened vector...
                        }
                    }
                    "=" => {
                        statement.equal_passed();
                        continue;
                    }
                    "+" | "-" | "." | "*" => statement.next(Token::Operator(tk.to_string())),

                    _ => statement.next(Token::Identifier(tk.to_string())),
                }
            }
        }
        statement.finalize();

        // now calculate based on priorities
        let mut rhs_tokens_count = statement.right.len();
        let mut i = 0;

        while i < rhs_tokens_count {
            if let Token::Operator(op) = &statement.right[i] {
                if i == 0 || i == rhs_tokens_count - 1 {
                    // ERROR
                }
                if op != "*" && op != "/" && op != "." {
                    i += 1;
                    continue;
                }
                match (
                    statement.at(i - 1, &scope_memory),
                    statement.at(i + 1, &scope_memory),
                ) {
                    (Token::Scalar(s1), Token::Scalar(s2)) => {
                        statement.merge_adjacents(
                            &mut i,
                            match op.as_str() {
                                "*" => Token::Scalar(s1 * s2),
                                "/" => Token::Scalar(s1 / s2),
                                _ => Token::Wtf(format!("Invalid Operator: {}", op)),
                            },
                            &mut rhs_tokens_count,
                        );
                    }
                    (Token::Vector(v1), Token::Vector(v2)) => {
                        statement.merge_adjacents(
                            &mut i,
                            match op.as_str() {
                                "*" => Token::Matrix(v1.cross(&v2)),
                                "." => {
                                    if let Some(product) = v1.dot(&v2) {
                                        Token::Scalar(product)
                                    } else {
                                        Token::Wtf(format!(
                                            "{} has different dimension than {}",
                                            v1.to_string(),
                                            v2.to_string()
                                        ))
                                    }
                                }
                                _ => Token::Wtf(format!("Invalid Operator: {}", op)),
                            },
                            &mut rhs_tokens_count,
                        );
                    }
                    (Token::Scalar(c), Token::Vector(v)) => {
                        statement.merge_adjacents(
                            &mut i,
                            match op.as_str() {
                                "." | "*" => Token::Vector(v.map(c, 0.0)),
                                _ => Token::Wtf(format!("Invalid Operator: {}", op)),
                            },
                            &mut rhs_tokens_count,
                        );
                    }
                    (Token::Vector(v), Token::Scalar(c)) => {
                        statement.merge_adjacents(
                            &mut i,
                            match op.as_str() {
                                "." | "*" => Token::Vector(v.map(c, 0.0)),
                                _ => Token::Wtf(format!("Invalid Operator: {}", op)),
                            },
                            &mut rhs_tokens_count,
                        );
                    }
                    _ => {
                        // ERROR
                    }
                }
            }

            i += 1
        }

        for i in 0..statement.right.len() {
            // TODO: Second order priorities...
        }
        if !statement.is_resolved() {
            // ERROR:
        }
        if statement.has_equal {
            if let Token::Identifier(ident) = &statement.left[0] {
                scope_memory.define(ident.clone(), statement.at(0, &scope_memory));
            }
        }
        statement.at(0, &scope_memory).to_string()
    }
}
