mod cpu;
mod roms;

use roms::test_rom1;

use cpu::ram::Ram;
use cpu::rom::Rom;

use crate::cpu::Memory;

fn main() -> cpu::Result<()> {
    let mut registers = cpu::registers::Registers::new();
    let _ = Ram::new(65536);

    let rom = Rom::new(test_rom1(), 0x0000);

    registers.set_flag(cpu::registers::StatusBits::Brk);
    registers.clear_flag(cpu::registers::StatusBits::Carry);

    println!("Registers: {}", registers);

    for inst in 0..10u16 {
        let b = rom.read_byte(inst)?;
        println!("Decode: {:02x}", b);
    }

    Ok(())
}
