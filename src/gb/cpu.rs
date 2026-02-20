pub struct Cpu {
    pub pc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        println!("I'm working!");
        Self {
            pc: 0,
        }
    }
}