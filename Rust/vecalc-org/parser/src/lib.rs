extern crate calculus;
use calculus::*;
use calculus::memory::*;

trait MathOperations {
    // Use this: https://chatgpt.com/share/6737beff-489c-8003-b6fe-34b207ab028e
}

pub struct Analyzer {
    memory: Memory,
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
        let tokens: Vec<&str> = instruction.split_whitespace().collect();

        let mut lhs: Vec<Token> = Vec::new();
        let mut rhs: Vec<Token> = Vec::new();
        let mut passed_equal: bool = false;
        let mut inside_sth: bool = false;
        let mut vector_reading_cache: Vec<f64> = Vec::new();
        let mut matrix_rading_cache: Vec<Token> = Vec::new();

        let scope_memory = self.get().get(scope);

        let mut parse_next = |next| {
            if passed_equal {
                rhs.push(next);
            } else {
                lhs.push(next);
            }
        }
        for tk in tokens {
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
                        Token::Vector | Token::Matrix => {
                            matrix_rading_cache.push(v_as_token); // starts extracting matrix
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
                    },
                    "]" => {
                        if inside_sth {
                            if matrix_rading_cache.len() > 0 {
                                let v: matrix::Matrix =
                                    matrix::Matrix::new(&String::from("#inline-temp"), matrix_rading_cache.clone());
                                parse_next(v);
                                matrix_rading_cache.clear();
                            } else if vector_reading_cache.len() > 0 {
                                let v: vector::Vector =
                                    vector::Vector::new(&String::from("#inline-temp"), vector_reading_cache.clone());
                                parse_next(v);
                            } else {
                                // throw error: Empty vector (remember to falsify flag)
                            }
                            inside_sth = false;
                        } else {
                            // throw error: Unopened vector...
                        }
                    },
                    "=" => {
                        passed_equal = true;
                        continue;
                    },
                    "+" | "-" | "." | "*" | "x" => {
                        parse_next(Token::Operator(tk.to_string()));
                    },
                    _ => {
                        parse_next(scope_memory.evaluate_identifier(tk))
                    }
                }
            }

        }
        // now calculate based on priorities
        let rhs_tokens_count = rhs.len();
        for  mut i in 0..rhs_tokens_count {
            if let Token::Operator(operator) = &rhs[i] {
                if i > 0 && i < rhs_tokens_count - 1 {
                    match operator.as_str() {
                        "x" => {
                            match &rhs[i - 1] {
                                Token::Vector(v) => {
                                    if let Token::Vector(u) = &rhs[i + 1] {
                                        let r = v.cross(u);
                                        rhs[i - 1] = Token::Matrix(r);
                                        rhs.remove(i + 1);
                                        rhs.remove(i);
                                        i -= 1;
                                    }
                                }
                                Token::Matrix(m) => {

                                }
                                _ => {

                                }
                            }
                        }
                        _ => {
                            
                        }
                    }
                }
            }
        }
    }
}
