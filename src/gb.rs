use crate::gb::{cpu::Cpu, memory::{BANK_SIZE, Memory}};

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
            memory: Memory {
                fixed_rom_bank: [0; BANK_SIZE],
                switch_rom_bank: [0; BANK_SIZE],
                io_registers: [0; 0xFF80 - 0xFF00],
                hram: [0; 0xFFFF - 0xFF80],
            },
            cycles: 0,
        }
    }

    pub fn emulate_cycle(&mut self) {
        let instr = self.cpu.fetch_instruction(&mut self.memory);
        println!("Cycle: {:>3}, Instruction: {:02X}",
            self.cycles,
            instr
        );
        self.cycles = self.cycles.wrapping_add(1);
        self.cpu.execute_instruction(instr, &mut self.memory).unwrap();
    }
}