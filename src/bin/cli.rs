use std::{fs::File, io::Read};

use gbcrs::gb::Gb;

fn main() {
    println!("Hello World!");
    let mut emu = Gb::new();
    let path = "dmg0_boot.bin";
    let mut f = File::open(path).expect("File not found");
    f.read(&mut emu.memory.fixbank).expect("Failed to read");
    for _ in 0..=1000 {
        emu.emulate_cycle();
        if emu.cpu.pc == 0x100 {
            break;
        }
    }
    emu.cpu.print_registers();
}
