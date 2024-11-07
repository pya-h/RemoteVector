pub mod memory;
pub mod vector;
pub mod matrix;
pub enum Term {
    Vector(vector::Vector),
    Matrix(matrix::Matrix),
    Scalar(f64),
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

    fn evaluate(tokens: Vec<&str>) -> Term {
        for token in &tokens {
            if token == &"(" {
                // TODO: extract vector
            } else if token == &"[" {
                // TODO: extract matrix
            } else {
                // TODO: probably a pre-defined vector
            }
        }
        Term::Null
    }
    pub fn anaalyze(instruction: &String) {
        let tokens: Vec<&str> = instruction.split_whitespace().collect();
        let terms_count: usize = tokens.len();

        if terms_count > 2 {
            if tokens[1] == "=" {

            }
        }
    }
}