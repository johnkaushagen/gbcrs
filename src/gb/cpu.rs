use crate::gb::mmu::Mmu;

pub struct Cpu {
    // Registers
    pub a: u8, pub f: u8,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Cpu {
    pub fn new() -> Self {
        println!("I'm working!");
        Self {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn fetch_instruction<M: Mmu>(&mut self, memory: &mut M) -> u8{
        let instr = memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        instr
    }
}