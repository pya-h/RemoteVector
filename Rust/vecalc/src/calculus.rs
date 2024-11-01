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
}
