use std::marker::PhantomData;

use crate::cpu::{
    registers::Registers, registers::StatusBits, Address, Data, Error, ErrorType, Memory, Opcode,
    Result,
};

pub struct ExecutionUnit<M> {
    phantom: PhantomData<M>,
}

impl<M> ExecutionUnit<M>
where
    M: Memory,
{
    pub fn new() -> Self {
        ExecutionUnit {
            phantom: PhantomData,
        }
    }
}

impl<M> crate::cpu::ExecutionUnit<M> for ExecutionUnit<M>
where
    M: Memory,
{
    fn execute(
        &self,
        opcode: &Opcode,
        data: Option<Data>,
        address: Option<Address>,
        memory: &M,
        registers: &mut Registers,
    ) -> Result<Option<Data>> {
        match opcode {
            Opcode::ADC => {
                if let Some(d) = data {
                    let result = registers.a + d + if registers.carry() { 1 } else { 0 };
                    registers.write_flag(StatusBits::Neg, result & 0x80 == 1);

                    Ok(Some(result))
                } else {
                    Err(Error::with_pc(registers.pc, ErrorType::MissingData))
                }
            }
            Opcode::AND => todo!(),
            Opcode::ASL => todo!(),
            Opcode::BCC => todo!(),
            Opcode::BCS => todo!(),
            Opcode::BEQ => todo!(),
            Opcode::BIT => todo!(),
            Opcode::BMI => todo!(),
            Opcode::BNE => todo!(),
            Opcode::BPL => todo!(),
            Opcode::BRK => todo!(),
            Opcode::BVC => todo!(),
            Opcode::BVS => todo!(),
            Opcode::CLC => todo!(),
            Opcode::CLD => todo!(),
            Opcode::CLI => todo!(),
            Opcode::CLV => todo!(),
            Opcode::CMP => todo!(),
            Opcode::CPX => todo!(),
            Opcode::CPY => todo!(),
            Opcode::DEC => todo!(),
            Opcode::DEX => todo!(),
            Opcode::DEY => todo!(),
            Opcode::EOR => todo!(),
            Opcode::INC => todo!(),
            Opcode::INX => todo!(),
            Opcode::INY => todo!(),
            Opcode::JMP => todo!(),
            Opcode::JSR => todo!(),
            Opcode::LDA => todo!(),
            Opcode::LDX => todo!(),
            Opcode::LDY => todo!(),
            Opcode::LSR => todo!(),
            Opcode::NOP => todo!(),
            Opcode::ORA => todo!(),
            Opcode::PHA => todo!(),
            Opcode::PHP => todo!(),
            Opcode::PLA => todo!(),
            Opcode::PLP => todo!(),
            Opcode::ROL => todo!(),
            Opcode::ROR => todo!(),
            Opcode::RTI => todo!(),
            Opcode::RTS => todo!(),
            Opcode::SBC => todo!(),
            Opcode::SEC => todo!(),
            Opcode::SED => todo!(),
            Opcode::SEI => todo!(),
            Opcode::STA => todo!(),
            Opcode::STX => todo!(),
            Opcode::STY => todo!(),
            Opcode::TAX => todo!(),
            Opcode::TAY => todo!(),
            Opcode::TSX => todo!(),
            Opcode::TXA => todo!(),
            Opcode::TXS => todo!(),
            Opcode::TYA => todo!(),
            Opcode::Invalid(o) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::ram::Ram;

    #[test]
    fn construct() {
        let _execution_unit: ExecutionUnit<Ram> = ExecutionUnit::new();
    }
}
