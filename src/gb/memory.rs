use crate::gb::mmu::Mmu;

pub const BANK_SIZE: usize = 0x4000; // 16 KiB ROM Banks

pub struct Memory {
    pub fixed_rom_bank: [u8; BANK_SIZE],
    pub switch_rom_bank: [u8; BANK_SIZE],
}

impl Mmu for Memory {
    fn read_byte(&mut self, addr: u16) -> u8 {
        if (addr as usize) < BANK_SIZE {
            self.fixed_rom_bank[addr as usize]
        } else if (addr as usize) < 2*BANK_SIZE {
            self.switch_rom_bank[(addr as usize) - BANK_SIZE]
        } else {
            panic!("Invalid addres 0x{:04X}", addr);
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        if (addr as usize) < BANK_SIZE {
            self.fixed_rom_bank[addr as usize] = value;
        } else if (addr as usize) < 2 * BANK_SIZE {
            self.switch_rom_bank[(addr as usize) - BANK_SIZE] = value;
        } else {
            panic!("Invalid adress 0x{:04X}", addr);
        }
    }
}