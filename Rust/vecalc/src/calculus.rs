pub mod memory;
pub mod vector;
pub mod matrix;
pub enum Token {
    Vector(vector::Vector),
    Matrix(matrix::Matrix),
    Scalar(f64),
    Operator(String),
    Null,
    Wtf(String)
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

    fn evaluate(str_tokens: Vec<&str>) -> Token {
        let mut lhs: Vec<Token>  = Vec::new();
        let mut rhs: Vec<Token> = Vec::new();

        let mut reading_vector: bool = false;
        let mut vector_elements: Vec<f64>;
        for tk in str_tokens {
            match tk {
                "(" => {
                    reading_vector = true;
                    vector_elements = Vec::new();
                },
                "[" => {
                    // TODO: extract matrix
                },
                ")" => {
                    if reading_vector {
                        vector::Vector v = vector::Vector::new(St0ing::from("#inline-temp"), vector_elements);
                        rhs.push(value);
                        reading_vector = false;
                    } else {
                        // parenthesis or error
                    }
                },
                _ => {
                    // TODO: probably a pre-defined vector
                }
            }

            if reading_vector {
                let value: f64 = tk.parse().unwrap();
                vector_elements.push(value);
            }
        }
        Token::Null
    }
    pub fn analyze(instruction: &String) {
        let tokens: Vec<&str> = instruction.split_whitespace().collect();
        let terms_count: usize = tokens.len();

        if terms_count > 2 {
            if tokens[1] == "=" {

            }
        }
    }
}