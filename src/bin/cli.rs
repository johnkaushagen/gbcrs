use gbcrs::gb::cpu::Cpu;
use gbcrs::gb::memory::{BANK_SIZE, Memory};


fn main() {
    println!("Hello World!");
    let mut cpu = Cpu::new();
    let mut memory = Memory {
        fixed_rom_bank: [0; BANK_SIZE],
        switch_rom_bank: [0; BANK_SIZE],
    };
    memory.fixed_rom_bank[0] = 0x12;
    memory.fixed_rom_bank[1] = 0x34;
    let instr = cpu.fetch_instruction(&mut memory);
    println!("Instruction 0x{:02X}", instr);
    let instr = cpu.fetch_instruction(&mut memory);
    println!("Instruction 0x{:02X}", instr);
}