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
        let mut matrix_rading_cache: Vec<Vector> = Vec::new();

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
                    match tk.parse::<f64>() {
                        Ok(v) => {
                            vector_reading_cache.push(v);
                        }
                        Err(_) => {
                            if let Some(v) = self.get().get(scope).get(&tk.to_string()) {
                                vector_reading_cache.push(v);
                            } else {
                                // TODO: Throw error
                            }
                        }
                    }
                } else {

                }
            } else {
                match tk {
                    "[" => {
                        inside_sth = true;
                        vector_reading_cache.clear();
                    },
                    "]" => {
                        if inside_sth {
                            if matrix_rading_cache.len() > 0 {

                            }
                            let v: vector::Vector =
                                vector::Vector::new(&String::from("#inline-temp"), vector_reading_cache.clone());
  
                            inside_sth = false;
                        } else {
                            // parenthesis or error
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
                        match tk.parse::<f64>() {
                            Ok(v) => {
                                if passed_equal {
                                    rhs.push(Token::Scalar(v));
                                } else {
                                    lhs.push(Token::Scalar(v));
                                }
                            }
                            Err(_) => {
                                let v = self.get().get(scope).get(&tk.to_string());
                            }
                        }
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
