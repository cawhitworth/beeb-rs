use crate::cpu::registers::Registers;
use crate::cpu::{
    AddressDispatcher, DataDispatcher, InstructionDecoder,
    Memory, Result,
};

use crate::cpu::ExecutionUnit;

use super::WritebackUnit;

pub struct Dispatcher<'a, I, A, D, M, E, W>
where
    I: InstructionDecoder,
    M: Memory,
    A: AddressDispatcher<M>,
    D: DataDispatcher<M>,
    E: ExecutionUnit<M>,
    W: WritebackUnit<M>,
{
    instruction_decoder: &'a I,
    address_dispatcher: &'a A,
    data_dispatcher: &'a D,
    memory: &'a mut M,
    registers: &'a mut Registers,
    execution_unit: &'a E,
    writeback_unit: &'a W,
}

impl<'a, I, A, D, M, E, W> Dispatcher<'a, I, A, D, M, E, W>
where
    I: InstructionDecoder,
    M: Memory,
    A: AddressDispatcher<M>,
    D: DataDispatcher<M>,
    E: ExecutionUnit<M>,
    W: WritebackUnit<M>,
{
    pub fn new(
        registers: &'a mut Registers,
        memory: &'a mut M,
        instruction_decoder: &'a I,
        address_dispatcher: &'a A,
        data_dispatcher: &'a D,
        execution_unit: &'a E,
        writeback_unit: &'a W,
    ) -> Self {
        Dispatcher {
            instruction_decoder,
            address_dispatcher,
            data_dispatcher,
            memory,
            registers,
            execution_unit,
            writeback_unit,
        }
    }

    pub fn dispatch(&mut self) -> Result<()> {
        let opcode = self.memory.read_byte(self.registers.pc)?;
        let instruction = self.instruction_decoder.decode(opcode)?;

        if instruction.opcode != crate::cpu::Opcode::Invalid {
        self.registers.pc_next = self.registers.pc + instruction.byte_length as u16;

        let address = self.address_dispatcher.get_address(
            &instruction.addressing_mode,
            &self.memory,
            &self.registers,
        )?;

        let data = self.data_dispatcher.get_data(
            &instruction.addressing_mode,
            &self.memory,
            &self.registers,
        )?;

        let result = self.execution_unit.execute(
            &instruction.opcode,
            data,
            address,
            &self.memory,
            &mut self.registers,
        )?;

        if result.is_some() {
            self.writeback_unit.writeback(
                &instruction.writeback,
                result,
                address,
                &mut self.memory,
                &mut self.registers,
            )?;
        }

        self.registers.pc = self.registers.pc_next;
    } else {
        self.registers.pc += 1;
    }

        Ok(())
    }
}
