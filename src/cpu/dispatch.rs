use crate::cpu::registers::Registers;
use crate::cpu::{
    Address, AddressDispatcher, AddressingMode, Data, DataDispatcher, Error, InstructionDecoder,
    Memory, Result,
};

pub struct Dispatcher<I, A, D, M>
where
    I: InstructionDecoder,
    A: AddressDispatcher,
    D: DataDispatcher,
    M: Memory,
{
    instruction_decoder: I,
    address_dispatcher: A,
    data_dispatcher: D,
    memory: M,
    registers: Registers,
}

impl<I, A, D, M> Dispatcher<I, A, D, M>
where
    I: InstructionDecoder,
    A: AddressDispatcher,
    D: DataDispatcher,
    M: Memory,
{
    pub fn new(
        registers: Registers,
        memory: M,
        instruction_decoder: I,
        address_dispatcher: A,
        data_dispatcher: D,
    ) -> Self {
        Dispatcher {
            instruction_decoder,
            address_dispatcher,
            data_dispatcher,
            memory,
            registers,
        }
    }

    fn dispatch_address(&self, mode: &AddressingMode) -> Result<Address> {
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
            AddressingMode::None => Err(Error::InvalidAddressingMode),
        }
    }

    fn dispatch_data(&self, mode: &AddressingMode) -> Result<Data> {
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
            AddressingMode::None => Err(Error::InvalidAddressingMode),
        }
    }

    pub fn dispatch(&mut self) -> Result<()> {
        let opcode = self.memory.read_byte(self.registers.pc)?;
        let instruction = self.instruction_decoder.decode(opcode)?;

        let address = self.dispatch_address(&instruction.addressing_mode)?;
        let data = self.dispatch_data(&instruction.addressing_mode)?;

        println!("{:?} {:04x} {:04x}", instruction.opcode, address, data);

        Ok(())
    }
}
