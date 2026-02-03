extern crate calculus;
use calculus::matrix::Matrix;
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
    has_equal: bool,
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

    pub fn analyze(&mut self, instruction: &String, scope: &String) {
        let tokens_as_string: Vec<&str> = instruction.split_whitespace().collect();

        let mut inside_sth: bool = false;
        let mut vector_reading_cache: Vec<f64> = Vec::new();
        let mut matrix_rading_cache: Vec<Vector> = Vec::new();
        let mut statement = Statement::new();
        let scope_memory = self.get().get(scope);

        for tk in tokens_as_string {
            if inside_sth {
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
                            vector_reading_cache.push(v); // starts extracting vector
                        }
                        Token::Vector(v) => {
                            matrix_rading_cache.push(v); // starts extracting matrix
                        }
                        _ => {
                            // throw error.
                        }
                    }
                }
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
                                let m: Matrix = Matrix::new(
                                    &String::from("#inline-temp"),
                                    matrix_rading_cache.clone(),
                                );
                                statement.next(Token::Matrix(m));
                                matrix_rading_cache.clear();
                            } else if vector_reading_cache.len() > 0 {
                                let v: Vector = Vector::new(
                                    &String::from("#inline-temp"),
                                    vector_reading_cache.clone(),
                                );
                                statement.next(Token::Vector(v));
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
                    "+" | "-" | "." | "*" | "x" => {
                        statement.next(Token::Operator(tk.to_string()));
                    }
                    _ => statement.next(scope_memory.evaluate_identifier(tk)),
                }
            }
        }
        statement.finalize();
        // now calculate based on priorities
        let rhs_tokens_count = statement.right.len();
        for mut i in 0..rhs_tokens_count {
            if let Token::Operator(operator) = &statement.right[i] {
                if i > 0 && i < rhs_tokens_count - 1 {
                    match operator.as_str() {
                        "x" => match &statement.right[i - 1] {
                            Token::Vector(v) => {
                                if let Token::Vector(u) = &statement.right[i + 1] {
                                    let r = v.cross(u);
                                    statement.right[i - 1] = Token::Matrix(r);
                                    statement.right.remove(i + 1);
                                    statement.right.remove(i);
                                    i -= 1;
                                }
                            }
                            Token::Matrix(m) => {}
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}
