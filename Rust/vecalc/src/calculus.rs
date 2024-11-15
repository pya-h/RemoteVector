pub mod matrix;
pub mod memory;
pub mod vector;
pub enum Token {
    Vector(vector::Vector),
    Matrix(matrix::Matrix),
    Scalar(f64),
    Operator(String),
    Null,
    Wtf(String),
}

trait MathOperations {
    // Use this: https://chatgpt.com/share/6737beff-489c-8003-b6fe-34b207ab028e
}

pub struct Analyzer {
    memory: memory::Memory,
}

impl Analyzer {
    pub fn init() -> Analyzer {
        Analyzer {
            memory: memory::Memory::create(),
        }
    }

    pub fn get(&mut self) -> &mut memory::Memory {
        &mut self.memory
    }

    pub fn analyze(&mut self, instruction: &String, scope: &String) {
        let tokens: Vec<&str> = instruction.split_whitespace().collect();

        let mut lhs: Vec<Token> = Vec::new();
        let mut rhs: Vec<Token> = Vec::new();
        let mut passed_equal: bool = false;
        let mut reading_vector: bool = false;
        let mut vector_elements: Vec<f64> = Vec::new();
        
        for tk in tokens {
            if reading_vector {
                let value: f64 = tk.parse().unwrap();
                vector_elements.push(value);
            } else {
                match tk {
                    "(" => {
                        reading_vector = true;
                        vector_elements.clear();
                    },
                    "[" => {
                        // TODO: extract matrix
                    },
                    ")" => {
                        if reading_vector && vector_elements.len() > 0 {
                            let v: vector::Vector =
                                vector::Vector::new(&String::from("#inline-temp"), vector_elements.clone());
                            if passed_equal {
                                rhs.push(Token::Vector(v));
                            } else {
                                lhs.push(Token::Vector(v));
                            }
                            reading_vector = false;
                        } else {
                            // parenthesis or error
                        }
                    },
                    "=" => {
                        passed_equal = true;
                        continue;
                    },
                    "+" | "-" | "." | "*" | "x" => {
                        if passed_equal {
                            rhs.push(Token::Operator(tk.to_string()));
                        } else {
                            lhs.push(Token::Operator(tk.to_string()));
                        } 
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
