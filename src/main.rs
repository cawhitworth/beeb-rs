mod cpu;
mod roms;

use cpu::data::DataDispatcher;
use cpu::address::AddressDispatcher;
use cpu::instruction_decode::InstructionDecoder;
use roms::test_rom1;

use cpu::ram::Ram;
use cpu::rom::Rom;

struct Dispatcher<I, A, D, M>
where I: cpu::InstructionDecoder,
A: cpu::AddressDispatcher,
D: cpu::DataDispatcher,
M: cpu::Memory {
    instruction_decoder: I,
    address_dispatcher: A,
    data_dispatcher: D,
    memory: M,
    registers: cpu::registers::Registers
}

impl<I,A,D,M> Dispatcher<I,A,D,M>
where I: cpu::InstructionDecoder,
A: cpu::AddressDispatcher,
D: cpu::DataDispatcher,
M: cpu::Memory {
    fn dispatch(&mut self) -> cpu::Result<()> {
        let opcode = self.memory.read_byte(self.registers.pc)?;
        let instruction = self.instruction_decoder.decode(opcode)?;
        println!("{:?}", instruction.opcode);

        Ok(())
    }
}

fn main() -> cpu::Result<()> {
    let mut registers = cpu::registers::Registers::new();
    let _ = Ram::new(65536);

    let rom = Rom::new(test_rom1(), 0x0000);

    registers.set_flag(cpu::registers::StatusBits::Brk);
    registers.clear_flag(cpu::registers::StatusBits::Carry);

    let mut cpu = Dispatcher {
        instruction_decoder: InstructionDecoder::new(),
        address_dispatcher: AddressDispatcher::new(),
        data_dispatcher: DataDispatcher::new(),
        memory: Ram::new(64 * 1024),
        registers
    };

    cpu.dispatch()?;

    Ok(())
}
