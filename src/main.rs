mod cpu;
mod roms;

use cpu::address::AddressDispatcher;
use cpu::data::DataDispatcher;
use cpu::instruction_decode::InstructionDecoder;
use cpu::AddressingMode;
use roms::test_rom1;

use cpu::ram::Ram;
use cpu::rom::Rom;

struct Dispatcher<I, A, D, M>
where
    I: cpu::InstructionDecoder,
    A: cpu::AddressDispatcher,
    D: cpu::DataDispatcher,
    M: cpu::Memory,
{
    instruction_decoder: I,
    address_dispatcher: A,
    data_dispatcher: D,
    memory: M,
    registers: cpu::registers::Registers,
}

impl<I, A, D, M> Dispatcher<I, A, D, M>
where
    I: cpu::InstructionDecoder,
    A: cpu::AddressDispatcher,
    D: cpu::DataDispatcher,
    M: cpu::Memory,
{
    fn dispatch_address(&self, mode: &AddressingMode) -> cpu::Result<cpu::Address> {
        match mode {
            AddressingMode::Implicit => self.address_dispatcher.implicit(),
            AddressingMode::Accumulator => self.address_dispatcher.accumulator(),
            AddressingMode::Immediate => self.address_dispatcher.immediate(),
            AddressingMode::ZeroPage => self.address_dispatcher.zero_page(),
            AddressingMode::ZeroPageX => self.address_dispatcher.zero_page_x(),
            AddressingMode::ZeroPageY => self.address_dispatcher.zero_page_y(),
            AddressingMode::Relative => self.address_dispatcher.relative(),
            AddressingMode::Absolute => self.address_dispatcher.absolute(),
            AddressingMode::AbsoluteX => self.address_dispatcher.absolute_x(),
            AddressingMode::AbsoluteY => self.address_dispatcher.absolute_y(),
            AddressingMode::Indirect => self.address_dispatcher.indirect(),
            AddressingMode::IndirectX => self.address_dispatcher.indirect_x(),
            AddressingMode::IndirectY => self.address_dispatcher.indirect_y(),
            AddressingMode::None => Err(cpu::Error::InvalidAddressingMode),
        }
    }

    fn dispatch_data(&self, mode: &AddressingMode) -> cpu::Result<cpu::Data> {
        match mode {
            AddressingMode::Implicit => self.data_dispatcher.implicit(),
            AddressingMode::Accumulator => self.data_dispatcher.accumulator(),
            AddressingMode::Immediate => self.data_dispatcher.immediate(),
            AddressingMode::ZeroPage => self.data_dispatcher.zero_page(),
            AddressingMode::ZeroPageX => self.data_dispatcher.zero_page_x(),
            AddressingMode::ZeroPageY => self.data_dispatcher.zero_page_y(),
            AddressingMode::Relative => self.data_dispatcher.relative(),
            AddressingMode::Absolute => self.data_dispatcher.absolute(),
            AddressingMode::AbsoluteX => self.data_dispatcher.absolute_x(),
            AddressingMode::AbsoluteY => self.data_dispatcher.absolute_y(),
            AddressingMode::Indirect => self.data_dispatcher.indirect(),
            AddressingMode::IndirectX => self.data_dispatcher.indirect_x(),
            AddressingMode::IndirectY => self.data_dispatcher.indirect_y(),
            AddressingMode::None => Err(cpu::Error::InvalidAddressingMode),
        }
    }

    fn dispatch(&mut self) -> cpu::Result<()> {
        let opcode = self.memory.read_byte(self.registers.pc)?;
        let instruction = self.instruction_decoder.decode(opcode)?;

        let address = self.dispatch_address(&instruction.addressing_mode)?;
        let data = self.dispatch_data(&instruction.addressing_mode)?;

        println!("{:?} {:04x} {:04x}", instruction.opcode, address, data);

        Ok(())
    }
}

fn main() -> cpu::Result<()> {
    let mut registers = cpu::registers::Registers::new();
    let _ = Ram::new(65536);

    let _rom = Rom::new(test_rom1(), 0x0000);

    registers.set_flag(cpu::registers::StatusBits::Brk);
    registers.clear_flag(cpu::registers::StatusBits::Carry);

    let mut cpu = Dispatcher {
        instruction_decoder: InstructionDecoder::new(),
        address_dispatcher: AddressDispatcher::new(),
        data_dispatcher: DataDispatcher::new(),
        memory: Ram::new(64 * 1024),
        registers,
    };

    cpu.dispatch()?;

    Ok(())
}
