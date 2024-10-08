mod cpu;
mod disassembler;
mod roms;

use cpu::address::AddressAndDataDispatch;
use cpu::dispatch::Dispatcher;
use cpu::instruction_decode::InstructionDecoder;
use cpu::writeback::WritebackUnit;
use disassembler::execution::ExecutionUnit;

use cpu::memory::OverlayMemory;
use cpu::ram::Ram;
use cpu::rom::Rom;

fn main() -> cpu::Result<()> {
    let mut registers = cpu::registers::Registers::new();

    let address_data_dispatch = AddressAndDataDispatch::new();

    let ram = Ram::new(64 * 1024);
    let rom = Rom::new(roms::test_rom1());
    let overlay_memory = OverlayMemory::new(ram, rom, 0xff00);

    let instruction_decoder = InstructionDecoder::new();

    let execution_unit = ExecutionUnit::new();
    let writeback_unit = WritebackUnit::new();

    registers.pc = 0xff00;

    let mut cpu = Dispatcher::new(
        registers,
        overlay_memory,
        instruction_decoder,
        address_data_dispatch,
        execution_unit,
        writeback_unit,
    );

    loop {
        cpu.dispatch()?;
    }
}
