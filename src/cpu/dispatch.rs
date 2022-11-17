use crate::cpu::registers::Registers;
use crate::cpu::{AddressDataDispatcher, InstructionDecoder, Memory, Opcode, Result};

use crate::cpu::ExecutionUnit;

use super::{Error, ErrorType, ExecutionResult, WritebackUnit};

pub struct Dispatcher<I, A, M, E, W>
where
    I: InstructionDecoder,
    M: Memory,
    A: AddressDataDispatcher<M>,
    E: ExecutionUnit<M>,
    W: WritebackUnit<M>,
{
    instruction_decoder: I,
    address_dispatcher: A,
    memory: M,
    registers: Registers,
    execution_unit: E,
    writeback_unit: W,
}

impl<I, A, M, E, W> Dispatcher<I, A, M, E, W>
where
    I: InstructionDecoder,
    M: Memory,
    A: AddressDataDispatcher<M>,
    E: ExecutionUnit<M>,
    W: WritebackUnit<M>,
{
    pub fn new(
        registers: Registers,
        memory: M,
        instruction_decoder: I,
        address_dispatcher: A,
        execution_unit: E,
        writeback_unit: W,
    ) -> Self {
        Dispatcher {
            instruction_decoder,
            address_dispatcher,
            memory,
            registers,
            execution_unit,
            writeback_unit,
        }
    }

    pub fn dispatch(&mut self) -> Result<()> {
        let opcode = self.memory.read_byte(self.registers.pc)?;
        let instruction = self.instruction_decoder.decode(opcode)?;

        match instruction.opcode {
            Opcode::Invalid(o) => {
                self.registers.pc_next = self.registers.pc + 1;

                let result = self.execution_unit.execute(
                    &instruction.opcode,
                    None,
                    None,
                    &self.memory,
                    &mut self.registers,
                );

                self.registers.pc = self.registers.pc_next;

                if let Err(e) = result {
                    Err(e)
                } else {
                    Ok(())
                }
            }
            _ => {
                self.registers.pc_next = self.registers.pc + instruction.byte_length as u16;

                let address = self.address_dispatcher.get_address(
                    &instruction.addressing_mode,
                    &self.memory,
                    &self.registers,
                )?;

                let data = self.address_dispatcher.get_data(
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

                if result != ExecutionResult::None {
                    self.writeback_unit.writeback(
                        &instruction.writeback,
                        result,
                        address,
                        &mut self.memory,
                        &mut self.registers,
                    )?;
                }

                self.registers.pc = self.registers.pc_next;
                Ok(())
            }
        }
    }

    pub fn registers(&mut self) -> &mut Registers {
        &mut self.registers
    }
}
