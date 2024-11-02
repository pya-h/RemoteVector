pub mod memory;
pub mod vector;
pub mod matrix;
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

    fn anaalyze(instruction: &String) {
        let mut tokens = instruction.split_whitespace();
        let first = match tokens.next() {
            Some(token) => token,
            None => ""
        };
        if first.is_empty() {
            print!("Invalid Instruction!");
            return ();
        }
        if let Some(first_char)  = first.chars().next() {
            if first_char.is_alphabetic() {
                // define a new vector
            }
            
        }
        
    }
}