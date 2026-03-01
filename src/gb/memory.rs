use crate::gb::mmu::Mmu;

pub const BANK00_OFFSET: usize = 0x0000;
pub const BANKNN_OFFSET: usize = 0x4000;
pub const VRAM_OFFSET: usize = 0x8000;
pub const EXRAM_OFFSET: usize = 0xA000;
pub const WRAM1_OFFSET: usize = 0xC000;
pub const WRAM2_OFFSET: usize = 0xD000;
pub const ECHORAM_OFFSET: usize = 0xE000;
pub const OAM_OFFSET: usize = 0xFE00;
pub const NOUSE_OFFSET: usize = 0xFEA0;
pub const IOREG_OFFSET: usize = 0xFF00;
pub const HRAM_OFFSET: usize = 0xFF80;
pub const INTRRPT_OFFSET: usize = 0xFFFF;

pub const BANKSIZE: usize = 0x4000;
pub const VRAMSIZE: usize = 0x2000;
pub const EXRAMSIZE: usize = 0x2000;
pub const WRAMSIZE: usize = 0x1000;
pub const ECHORAMSIZE: usize = 0x1E00;
pub const OAMSIZE: usize = 0x00A0;
pub const NOUSESIZE: usize = 0x0060;
pub const IOREGSIZE: usize = 0x0080;
pub const HRAMSIZE: usize = 0x007F;

pub struct Memory {
    pub fixbank: [u8; BANKSIZE],
    pub switchbank: [u8; BANKSIZE],
    pub vram: [u8; VRAMSIZE],
    pub exram: [u8; EXRAMSIZE],
    pub wram1: [u8; WRAMSIZE],
    pub wram2: [u8; WRAMSIZE],
    pub echoram: [u8; ECHORAMSIZE],
    pub oam: [u8; OAMSIZE],
    pub nouse: [u8; NOUSESIZE],
    pub ioreg: [u8; IOREGSIZE],
    pub hram: [u8; HRAMSIZE],
    pub interrupt_enable: u8,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            fixbank: [0; BANKSIZE],
            switchbank: [0; BANKSIZE],
            vram: [0; VRAMSIZE],
            exram: [0; EXRAMSIZE],
            wram1: [0; WRAMSIZE],
            wram2: [0; WRAMSIZE],
            echoram: [0; ECHORAMSIZE],
            oam: [0; OAMSIZE],
            nouse: [0; NOUSESIZE],
            ioreg: [0; IOREGSIZE],
            hram: [0; HRAMSIZE],
            interrupt_enable: 0,
        }
    }
}

impl Mmu for Memory {
    fn read_byte(&mut self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            BANK00_OFFSET..BANKNN_OFFSET => return self.fixbank[addr],
            BANKNN_OFFSET..VRAM_OFFSET => return self.switchbank[addr - BANKNN_OFFSET],
            VRAM_OFFSET..EXRAM_OFFSET => return self.vram[addr - VRAM_OFFSET],
            EXRAM_OFFSET..WRAM1_OFFSET => return self.exram[addr - EXRAM_OFFSET],
            WRAM1_OFFSET..WRAM2_OFFSET => return self.wram1[addr - WRAM1_OFFSET],
            WRAM2_OFFSET..ECHORAM_OFFSET => return self.wram2[addr - WRAM2_OFFSET],
            ECHORAM_OFFSET..OAM_OFFSET => return self.echoram[addr - ECHORAM_OFFSET],
            OAM_OFFSET..NOUSE_OFFSET => return self.oam[addr - OAM_OFFSET],
            NOUSE_OFFSET..IOREG_OFFSET => return self.nouse[addr - NOUSE_OFFSET], // Maybe error? Don't know yet
            IOREG_OFFSET..HRAM_OFFSET => return self.ioreg[addr - IOREG_OFFSET],
            HRAM_OFFSET..INTRRPT_OFFSET => return self.hram[addr - HRAM_OFFSET],
            INTRRPT_OFFSET => return self.interrupt_enable,
            _ => panic!("Invalid address 0x{:04X}", addr), // This should never happen if addr is u16
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        let addr = addr as usize;
        match addr {
            BANK00_OFFSET..BANKNN_OFFSET => self.fixbank[addr] = value,
            BANKNN_OFFSET..VRAM_OFFSET => self.switchbank[addr - BANKNN_OFFSET] = value,
            VRAM_OFFSET..EXRAM_OFFSET => self.vram[addr - VRAM_OFFSET] = value,
            EXRAM_OFFSET..WRAM1_OFFSET => self.exram[addr - EXRAM_OFFSET] = value,
            WRAM1_OFFSET..WRAM2_OFFSET => self.wram1[addr - WRAM1_OFFSET] = value,
            WRAM2_OFFSET..ECHORAM_OFFSET => self.wram2[addr - WRAM2_OFFSET] = value,
            ECHORAM_OFFSET..OAM_OFFSET => self.echoram[addr - ECHORAM_OFFSET] = value,
            OAM_OFFSET..NOUSE_OFFSET => self.oam[addr - OAM_OFFSET] = value,
            NOUSE_OFFSET..IOREG_OFFSET => self.nouse[addr - NOUSE_OFFSET] = value, // Maybe error? Don't know yet
            IOREG_OFFSET..HRAM_OFFSET => self.ioreg[addr - IOREG_OFFSET] = value,
            HRAM_OFFSET..INTRRPT_OFFSET => self.hram[addr - HRAM_OFFSET] = value,
            INTRRPT_OFFSET => self.interrupt_enable = value,
            _ => panic!("Invalid address 0x{:04X}", addr), // This should never happen if addr is u16
        }
    }
}
