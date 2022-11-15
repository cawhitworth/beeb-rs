use crate::cpu::registers::Registers;
use crate::cpu::{
    Address, AddressDispatcher, AddressingMode, Data, DataDispatcher, Error, InstructionDecoder,
    Memory, Result,
};

use crate::cpu::ExecutionUnit;

use super::WritebackUnit;

pub struct Dispatcher<I, A, D, M, E, W>
where
    I: InstructionDecoder,
    M: Memory,
    A: AddressDispatcher<M>,
    D: DataDispatcher<M>,
    E: ExecutionUnit<M>,
    W: WritebackUnit<M>
{
    instruction_decoder: I,
    address_dispatcher: A,
    data_dispatcher: D,
    memory: M,
    registers: Registers,
    execution_unit: E,
    writeback_unit: W
}

impl<I, A, D, M, E, W> Dispatcher<I, A, D, M, E, W>
where
    I: InstructionDecoder,
    M: Memory,
    A: AddressDispatcher<M>,
    D: DataDispatcher<M>,
    E: ExecutionUnit<M>,
    W: WritebackUnit<M>
{
    pub fn new(
        registers: Registers,
        memory: M,
        instruction_decoder: I,
        address_dispatcher: A,
        data_dispatcher: D,
        execution_unit: E,
        writeback_unit: W
    ) -> Self {
        Dispatcher {
            instruction_decoder,
            address_dispatcher,
            data_dispatcher,
            memory,
            registers,
            execution_unit,
            writeback_unit
        }
    }

    pub fn dispatch(&mut self) -> Result<()> {
        let opcode = self.memory.read_byte(self.registers.pc)?;
        let instruction = self.instruction_decoder.decode(opcode)?;

        let address = self.address_dispatcher.dispatch(&instruction.addressing_mode, &self.memory, &self.registers)?;
        let data = self.data_dispatcher.dispatch(&instruction.addressing_mode, &self.memory, &self.registers)?;

        let result = self.execution_unit.execute(&instruction.opcode, data, address, &self.memory, &mut self.registers)?;

        if result.is_some() {
            self.writeback_unit.writeback(&instruction.writeback, result, address, &mut self.memory, &mut self.registers)?;
        }

        Ok(())
    }
}
