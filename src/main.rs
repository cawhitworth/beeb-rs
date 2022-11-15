mod cpu;
mod roms;
mod disassembler;

use cpu::writeback::WritebackUnit;
use cpu::address::AddressDispatcher;
use cpu::data::DataDispatcher;
use cpu::dispatch::Dispatcher;
use disassembler::execution::ExecutionUnit;
use cpu::instruction_decode::InstructionDecoder;

use cpu::ram::Ram;

fn main() -> cpu::Result<()> {
    let mut registers = cpu::registers::Registers::new();
    let _ = Ram::new(65536);

    registers.set_flag(cpu::registers::StatusBits::Brk);
    registers.clear_flag(cpu::registers::StatusBits::Carry);

    let mut cpu = Dispatcher::new(
        registers,
        Ram::new(64 * 1024),
        InstructionDecoder::new(),
        AddressDispatcher::new(),
        DataDispatcher::new(),
        ExecutionUnit::new(),
        WritebackUnit::new()
    );

    cpu.dispatch()?;

    Ok(())
}
