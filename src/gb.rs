use crate::gb::{cpu::Cpu, memory::Memory};

pub mod cpu;
pub mod memory;
pub mod mmu;

pub struct Gb {
    pub cpu: Cpu,
    pub memory: Memory,

    pub cycles: usize,
}

impl Gb {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            cycles: 0,
        }
    }

    pub fn emulate_cycle(&mut self) {
        let instr = self.cpu.fetch_instruction(&mut self.memory);
        println!("Cycle: {:>3}, Instruction: {:02X}", self.cycles, instr);
        self.cycles = self.cycles.wrapping_add(1);
        self.cpu
            .execute_instruction(instr, &mut self.memory)
            .unwrap();
    }
}
