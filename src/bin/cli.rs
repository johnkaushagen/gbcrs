use gbcrs::gb::Gb;

fn main() {
    println!("Hello World!");
    let mut emu = Gb::new();
    emu.memory.fixed_rom_bank[0] = 0x12;
    emu.memory.fixed_rom_bank[1] = 0x34;
    emu.memory.fixed_rom_bank[2] = 0x56;
    emu.memory.fixed_rom_bank[3] = 0x78;
    emu.memory.fixed_rom_bank[4] = 0x90;
    emu.memory.fixed_rom_bank[5] = 0xAB;
    emu.memory.fixed_rom_bank[6] = 0xCD;
    emu.memory.fixed_rom_bank[7] = 0xEF;
    for _ in 1..=8 {
        emu.emulate_cycle();
    }
}