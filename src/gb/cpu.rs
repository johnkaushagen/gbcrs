use std::{mem, result};

use crate::gb::{memory, mmu::Mmu};

const ZMASK: u8 = 0b1000_0000;
const NMASK: u8 = 0b0100_0000;
const HMASK: u8 = 0b0010_0000;
const CMASK: u8 = 0b0001_0000;

pub struct Cpu {
    // Registers
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Cpu {
    pub fn new() -> Self {
        println!("I'm working!");
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn print_registers(&self) {
        println!("Registers:");
        println!("-------------------------");
        println!("a:  0x{:04X}  |  f:  0x{:04X}", self.a, self.f);
        println!("b:  0x{:04X}  |  c:  0x{:04X}", self.b, self.c);
        println!("d:  0x{:04X}  |  e:  0x{:04X}", self.d, self.e);
        println!("h:  0x{:04X}  |  l:  0x{:04X}", self.h, self.l);
        println!("pc: 0x{:04X}  |  sp: 0x{:04X}", self.pc, self.sp);
    }

    pub fn fetch_instruction<M: Mmu>(&mut self, memory: &mut M) -> u8 {
        let instr = memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        instr
    }

    pub fn execute_instruction<M: Mmu>(&mut self, instr: u8, memory: &mut M) -> Result<(), String> {
        match instr {
            0x00 => Ok(()),
            0x05 => self.dec_b(),
            0x06 => self.ld_b_n8(memory),
            0x0C => self.inc_c(),
            0x0E => self.ld_c_n8(memory),
            0x11 => self.ld_de_n16(memory),
            0x13 => self.inc_de(),
            0x17 => self.rla(),
            0x1A => self.ld_a_de(memory),
            0x20 => self.jr_nz_e8(memory),
            0x21 => self.ld_hl_n16(memory),
            0x22 => self.ld_hli_a(memory),
            0x23 => self.inc_hl(),
            0x31 => self.ld_sp_n16(memory),
            0x32 => self.ld_hld_a(memory),
            0x3E => self.ld_a_n8(memory),
            0x4F => self.ld_c_a(),
            0x77 => self.ld_hl_a(memory),
            0x78 => self.ld_a_b(),
            0x7D => self.ld_a_l(),
            0x86 => self.add_a_hl(memory),
            0xAF => self.xor_a_a(),
            0xBE => self.cp_a_hl(memory),
            0xC1 => self.pop_bc(memory),
            0xC5 => self.push_bc(memory),
            0xC9 => self.ret(memory),
            0xCB => {
                // Prefix instructions,
                let prefix = self.read_and_increment(memory);
                match prefix {
                    0x11 => self.rlc(),
                    0x7C => self.bit_7h(),
                    _ => {
                        println!("Unknown opcode 0xCB{:02X}", prefix);
                        Ok(())
                    }
                }
            }
            0xCD => self.call_a16(memory),
            0xD1 => self.pop_de(memory),
            0xE0 => self.ldh_a8_a(memory),
            0xE2 => self.ldh_c_a(memory),
            0xE5 => self.push_hl(memory),
            0xFE => self.cp_a_n8(memory),
            _ => {
                println!("Unknown opcode 0x{:02X}", instr);
                Ok(())
            }
        }
    }
    // 0x00 NOP
    // 0x05 DEC B
    pub fn dec_b(&mut self) -> Result<(), String> {
        if self.b & 0x0F == 0 {
            // We have to borrow from bit 4
            self.f = self.f | (1 << 5);
        }
        self.b = self.b.wrapping_add_signed(-1);
        if self.b == 0 {
            self.f = self.f | (1 << 7);
        }
        self.f = self.f | (1 << 6);
        Ok(())
    }
    // 0x06 LD B, n8
    pub fn ld_b_n8<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let n8 = self.read_and_increment(memory);
        self.b = n8;
        Ok(())
    }
    // 0x0C INC C
    pub fn inc_c(&mut self) -> Result<(), String> {
        self.c = self.c.wrapping_add(1);
        if self.c == 0 {
            self.f = self.f | 0x80;
        }
        self.f = self.f & 0xBF;
        if self.c & 0b00010000 != 0 {
            self.f = self.f | 0b00100000;
        }
        Ok(())
    }
    // 0x0E LD C, n8
    pub fn ld_c_n8<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let value = self.read_and_increment(memory);
        self.c = value;
        Ok(())
    }
    // 0x11 LD DE, n16
    pub fn ld_de_n16<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        self.e = self.read_and_increment(memory);
        self.d = self.read_and_increment(memory);
        Ok(())
    }
    // 0x13 INC DE
    pub fn inc_de(&mut self) -> Result<(), String> {
        let val = self.de().wrapping_add(1);
        self.e = (val | &0x00FF) as u8;
        self.d = ((val & 0xFF00) >> 8) as u8;
        Ok(())
    }
    // 0x17 RL A
    pub fn rla(&mut self) -> Result<(), String> {
        self.a = self.a.rotate_left(1);
        self.f = (self.f & (0 << 5)) | ((self.a & 1) << 5);
        Ok(())
    }
    // 0x1A LD A, [DE]
    pub fn ld_a_de<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        self.a = memory.read_byte(self.de());
        Ok(())
    }
    // 0x20 JR NZ, e8
    pub fn jr_nz_e8<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let nz = (self.f & 0x80) >> 7;
        let e8 = self.read_and_increment(memory) as i8;
        if nz != 0 {
            self.pc = self.pc.wrapping_add_signed(e8 as i16);
        }
        Ok(())
    }
    // 0x21 LD HL n16
    pub fn ld_hl_n16<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        self.h = self.read_and_increment(memory);
        self.l = self.read_and_increment(memory);
        Ok(())
    }
    // 0x22 LD [HLI], A
    pub fn ld_hli_a<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        self.a = memory.read_byte(self.read_hl());
        self.inc_hl();
        Ok(())
    }
    // 0x23 INC HL
    pub fn inc_hl(&mut self) -> Result<(), String> {
        let val = self.read_hl().wrapping_add(1);
        self.l = (val & 0x00FF) as u8;
        self.h = ((val & 0xFF00) >> 8) as u8;
        Ok(())
    }
    // 0x31 LD SP n16
    pub fn ld_sp_n16<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let lsb = self.read_and_increment(memory) as u16;
        let msb = self.read_and_increment(memory) as u16;
        self.sp = (msb << 8) | lsb;
        Ok(())
    }
    // 0x32 LD HL- A
    pub fn ld_hld_a<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let value = memory.read_byte(self.read_hl());
        self.decrement_hl();
        self.a = value;
        Ok(())
    }
    // 0x3E LD A, n8
    pub fn ld_a_n8<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let value = self.read_and_increment(memory);
        self.a = value;
        Ok(())
    }
    // 0x4F LD C, A
    pub fn ld_c_a(&mut self) -> Result<(), String> {
        self.c = self.a;
        Ok(())
    }
    // 0x77 LD [HL], A
    pub fn ld_hl_a<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let addr = self.read_hl();
        self.a = memory.read_byte(addr);
        Ok(())
    }
    // 0x78 LD A, B
    pub fn ld_a_b(&mut self) -> Result<(), String> {
        self.a = self.b;
        Ok(())
    }
    // 0x7D LD A, L
    pub fn ld_a_l(&mut self) -> Result<(), String> {
        self.a = self.l;
        Ok(())
    }
    // 0x86 ADD A, [HL]
    pub fn add_a_hl<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let value = memory.read_byte(self.read_hl());
        self.a = self.a.wrapping_add(value);
        Ok(())
    }
    // 0xAF XOR, A, A
    pub fn xor_a_a(&mut self) -> Result<(), String> {
        self.a = self.a ^ self.a; // This is always 0
        self.f &= 0x80;
        Ok(())
    }
    // 0xBE CP A,[HL]
    pub fn cp_a_hl<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let addr = self.read_hl();
        let value = memory.read_byte(addr);
        if self.a == value {
            self.f = self.f | (1 << 7);
        }
        self.f = self.f | (1 << 6);
        if value > self.a {
            self.f = self.f | (1 << 5);
        }
        if (value & 0x0F) > (self.a & 0x0F) {
            self.f = self.f | (1 << 6);
        }
        Ok(())
    }
    // 0xC1 POP BC
    pub fn pop_bc<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let bc = self.pop_stack(memory);
        self.b = ((bc & 0xFF00) >> 8) as u8;
        self.c = (bc & 0x00FF) as u8;
        Ok(())
    }
    // 0xC5 PUSH BC
    pub fn push_bc<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        self.sp = self.sp.wrapping_add_signed(-1);
        memory.write_byte(self.sp, self.c);
        self.sp = self.sp.wrapping_add_signed(-1);
        memory.write_byte(self.sp, self.b);
        Ok(())
    }
    // 0xC9 RET
    pub fn ret<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        self.pc = self.pop_stack(memory);
        Ok(())
    }
    // 0xCB Prefix instructions
    // 0xCB11 RL C
    pub fn rlc(&mut self) -> Result<(), String> {
        self.c = self.c.rotate_left(1);
        self.f = (self.f & (0 << 5)) | ((self.c & 1) << 5);
        Ok(())
    }
    // 0xCB7C Bit 7 H
    pub fn bit_7h(&mut self) -> Result<(), String> {
        let result = ((1 << 7) & self.h) >> 7;
        let z = result ^ 1;
        let n = 0 as u8;
        let h = 1 as u8;
        self.f = self.f | (z << 7);
        self.f = self.f & (0xAF);
        self.f = self.f | (0x40);
        Ok(())
    }
    // 0xCD CALL a16
    pub fn call_a16<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let nn_lsb = self.read_and_increment(memory);
        let nn_msb = self.read_and_increment(memory);
        self.sp = self.sp.wrapping_add_signed(-1);
        memory.write_byte(self.sp, nn_lsb);
        self.sp = self.sp.wrapping_add_signed(-1);
        memory.write_byte(self.sp, nn_msb);
        self.pc = ((nn_msb as u16) << 8) | (nn_lsb as u16);
        Ok(())
    }
    // 0xD1 POP DE
    pub fn pop_de<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        self.e = memory.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        self.d = memory.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        Ok(())
    }
    // 0xE0 LDH [a8], A
    pub fn ldh_a8_a<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let lo = self.read_and_increment(memory);
        let addr = 0xFF00 | (lo as u16);
        self.a = memory.read_byte(addr);
        Ok(())
    }
    // 0xE2 LDH [C], A
    pub fn ldh_c_a<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let addr: u16 = 0xFF00 + (self.c as u16);
        let value = memory.read_byte(addr);
        self.a = value;
        Ok(())
    }
    // 0xE5 PUSH HL
    pub fn push_hl<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        self.sp = self.sp.wrapping_add_signed(-1);
        memory.write_byte(self.sp, self.h);
        self.sp = self.sp.wrapping_add_signed(-1);
        memory.write_byte(self.sp, self.l);
        Ok(())
    }
    // 0xFE CP A, n8
    pub fn cp_a_n8<M: Mmu>(&mut self, memory: &mut M) -> Result<(), String> {
        let value = self.read_and_increment(memory);
        if self.a == value {
            self.f = self.f | (1 << 7);
        }
        self.f = self.f | (1 << 6);
        if value > self.a {
            self.f = self.f | (1 << 5);
        }
        if (value & 0x0F) > (self.a & 0x0F) {
            self.f = self.f | (1 << 6);
        }
        Ok(())
    }

    fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    fn pop_stack<M: Mmu>(&mut self, memory: &mut M) -> u16 {
        let msb = memory.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let lsb = memory.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        ((msb as u16) << 8) | (lsb as u16)
    }

    fn push_stack<M: Mmu>(&mut self, memory: &mut M, value: u16) {
        let lsb = (value & 0x00FF) as u8;
        let msb = ((value & 0xFF00) >> 8) as u8;
        memory.write_byte(self.sp, lsb);
        self.sp = self.sp.wrapping_add_signed(-1);
        memory.write_byte(self.sp, msb);
        self.sp = self.sp.wrapping_add_signed(-1);
    }
    fn read_hl(&mut self) -> u16 {
        let msb = self.h as u16;
        let lsb = self.l as u16;
        (msb << 8) | lsb
    }

    fn decrement_hl(&mut self) {
        let mut hl = self.read_hl();
        hl = hl.wrapping_add_signed(-1);
        self.l = (hl & 0x00FF) as u8;
        self.h = ((hl & 0xFF00) >> 8) as u8;
    }
    fn read_and_increment<M: Mmu>(&mut self, memory: &mut M) -> u8 {
        let value = memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    fn set_z(&mut self) {
        self.f |= ZMASK;
    }
    fn unset_z(&mut self) {
        self.f &= !ZMASK;
    }
    fn set_n(&mut self) {
        self.f |= NMASK;
    }
    fn unset_n(&mut self) {
        self.f &= !NMASK;
    }
    fn set_h(&mut self) {
        self.f |= HMASK;
    }
    fn unset_h(&mut self) {
        self.f &= !HMASK;
    }
    fn set_c(&mut self) {
        self.f |= CMASK;
    }
    fn unset_c(&mut self) {
        self.f &= !CMASK;
    }
}

#[cfg(test)]
mod tests {
    use crate::gb::memory::{BANK_SIZE, Memory};

    use super::*;

    #[test]
    fn test_dec_b() {
        // Decrements the register B by 1
        // Sets Z if the result is 0
        // Sets N
        // Sets H if borrow from bit 4
        let mut cpu = Cpu::new();
        cpu.b = 0x02;
        cpu.dec_b().unwrap();
        assert_eq!(cpu.b, 0x01);
        assert_eq!((cpu.f >> 7) & 1, 0); // Z should NOT be set
        assert_eq!((cpu.f >> 6) & 1, 1); // N should be set
        assert_eq!((cpu.f >> 5) & 1, 0); // H should NOT be set

        cpu.f = 0x00;
        cpu.dec_b().unwrap();
        assert_eq!(cpu.b, 0x00);
        assert_eq!((cpu.f >> 7) & 1, 1); // Z should be set
        assert_eq!((cpu.f >> 6) & 1, 1); // N should be set
        assert_eq!((cpu.f >> 5) & 1, 0); // H should NOT be set

        cpu.f = 0x00;
        cpu.dec_b().unwrap();
        assert_eq!(cpu.b, 0xFF);
        assert_eq!((cpu.f >> 7) & 1, 0);
        assert_eq!((cpu.f >> 6) & 1, 1);
        assert_eq!((cpu.f >> 5) & 1, 1); // H should be set
    }
    #[test]
    fn test_ld_b_n8() {
        let mut cpu = Cpu::new();
        cpu.f = 0xA0;
        let mut memory = Memory {
            fixed_rom_bank: [0; BANK_SIZE],
            switch_rom_bank: [0; BANK_SIZE],
            io_registers: [0; 0x80],
            hram: [0; 0x7F],
        };
        memory.fixed_rom_bank[0] = 0xAB;
        cpu.ld_b_n8(&mut memory).unwrap();
        assert_eq!(cpu.b, 0xAB);
        assert_eq!(cpu.f, 0xA0); // Flags are unaffected
    }
    #[test]
    fn test_inc_c() {
        let mut cpu = Cpu::new();
        cpu.inc_c().unwrap();
        assert_eq!(cpu.c, 0x01);
        assert_eq!((cpu.f >> 7) & 1, 0); // Z should NOT be set
        assert_eq!((cpu.f >> 6) & 1, 0); // N should NOT be set (even if it previously was)
        assert_eq!((cpu.f >> 5) & 1, 0); // H should NOT be set

        // Tests that N is set to 0
        cpu.f = 0b0100_0000; // Set N
        cpu.inc_c().unwrap();
        assert_eq!(cpu.c, 0x02);
        assert_eq!((cpu.f >> 7) & 1, 0); // Z should NOT be set
        assert_eq!((cpu.f >> 6) & 1, 0); // N should NOT be set (even if it previously was)
        assert_eq!((cpu.f >> 5) & 1, 0); // H should NOT be set

        // Test half-carry flag
        cpu.c = 0x0F;
        cpu.inc_c().unwrap();
        assert_eq!(cpu.c, 0x10);
        assert_eq!((cpu.f >> 7) & 1, 0); // Z should NOT be set
        assert_eq!((cpu.f >> 6) & 1, 0); // N should NOT be set (even if it previously was)
        assert_eq!((cpu.f >> 5) & 1, 1); // H should be set

        cpu.inc_c();
        assert_eq!(cpu.c, 0x11);
        assert_eq!((cpu.f >> 7) & 1, 0); // Z should NOT be set
        assert_eq!((cpu.f >> 6) & 1, 0); // N should NOT be set (even if it previously was)
        assert_eq!((cpu.f >> 5) & 1, 0); // H should NOT be set
    }
}
