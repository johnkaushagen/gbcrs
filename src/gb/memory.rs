use crate::gb::mmu::Mmu;

pub const BANK_SIZE: usize = 0x4000; // 16 KiB ROM Banks

pub struct Memory {
    pub fixed_rom_bank: [u8; BANK_SIZE],
    pub switch_rom_bank: [u8; BANK_SIZE],
    pub io_registers: [u8; 0xFF80 - 0xFF00],
    pub hram: [u8; 0xFFFF - 0xFF80],
}

impl Mmu for Memory {
    fn read_byte(&mut self, addr: u16) -> u8 {
        match addr as usize {
            0..BANK_SIZE => return self.fixed_rom_bank[addr as usize],
            BANK_SIZE..0x8000 => return self.switch_rom_bank[(addr as usize) - BANK_SIZE],
            0xFF00..0xFF80 => return self.io_registers[(addr as usize) - 0xFF00],
            0xFF80..0xFFFF => return self.io_registers[(addr as usize) - 0xFF80],
            _ => panic!("Invalid address 0x{:04X}", addr),
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        if (addr as usize) < BANK_SIZE {
            self.fixed_rom_bank[addr as usize] = value;
        } else if (addr as usize) < 2 * BANK_SIZE {
            self.switch_rom_bank[(addr as usize) - BANK_SIZE] = value;
        } else if let 0xFF80..0xFFFF = (addr as usize) {
            self.hram[(addr as usize) - 0xFF80] = value;
        } else {
            println!("Invalid adress 0x{:04X}", addr);
        }
    }
}