use std::marker::PhantomData;

use crate::cpu::{
    registers::Registers, registers::StatusBits, Address, Data, Error, ErrorType, ExecutionResult,
    Memory, Opcode, Result,
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
    ) -> Result<ExecutionResult> {
        match opcode {
            Opcode::ADC => {
                if let Some(d) = data {
                    let result: u16 = registers.a as u16 + d as u16 + u16::from(registers.carry());
                    registers.write_flag(StatusBits::Neg, result & 0x80 == 0x80);
                    registers.write_flag(StatusBits::Zero, result & 0xff == 0);
                    registers.write_flag(StatusBits::Carry, result > 255);

                    Ok(ExecutionResult::Data((result & 0xff) as u8))
                } else {
                    Err(Error::with_pc(registers.pc, ErrorType::MissingData))
                }
            }
            Opcode::AND => {
                if let Some(d) = data {
                    let result = registers.a & d;
                    registers.write_flag(StatusBits::Neg, result & 0x80 == 0x80);
                    registers.write_flag(StatusBits::Zero, result == 0);
                    Ok(ExecutionResult::Data(result))
                } else {
                    Err(Error::with_pc(registers.pc, ErrorType::MissingData))
                }
            }
            Opcode::ASL => {
                if let Some(d) = data {
                    registers.write_flag(StatusBits::Carry, d & 0x80 == 0x80);

                    let result = d << 1;
                    registers.write_flag(StatusBits::Neg, result & 0x80 == 0x80);
                    registers.write_flag(StatusBits::Zero, result == 0);
                    Ok(ExecutionResult::Data(result))
                } else {
                    Err(Error::with_pc(registers.pc, ErrorType::MissingData))
                }
            }
            Opcode::BCC => {
                if !registers.carry() {
                    if let Some(a) = address {
                        Ok(ExecutionResult::Address(a))
                    } else {
                        Err(Error::with_pc(registers.pc, ErrorType::MissingAddress))
                    }
                } else {
                    Ok(ExecutionResult::None)
                }
            }
            Opcode::BCS => {
                if registers.carry() {
                    if let Some(a) = address {
                        Ok(ExecutionResult::Address(a))
                    } else {
                        Err(Error::with_pc(registers.pc, ErrorType::MissingAddress))
                    }
                } else {
                    Ok(ExecutionResult::None)
                }
            }
            Opcode::BEQ => {
                if registers.zero() {
                    if let Some(a) = address {
                        Ok(ExecutionResult::Address(a))
                    } else {
                        Err(Error::with_pc(registers.pc, ErrorType::MissingAddress))
                    }
                } else {
                    Ok(ExecutionResult::None)
                }
            }
            Opcode::BIT => {
                if let Some(d) = data {
                    let result = registers.a & d;
                    registers.write_flag(StatusBits::Neg, d & 0x80 == 0x80);
                    registers.write_flag(StatusBits::Ovf, d & 0x40 == 0x40);
                    registers.write_flag(StatusBits::Zero, result == 0);

                    Ok(ExecutionResult::None)
                } else {
                    Err(Error::with_pc(registers.pc, ErrorType::MissingData))
                }
            }
            Opcode::BMI => {
                if registers.negative() {
                    if let Some(a) = address {
                        Ok(ExecutionResult::Address(a))
                    } else {
                        Err(Error::with_pc(registers.pc, ErrorType::MissingAddress))
                    }
                } else {
                    Ok(ExecutionResult::None)
                }
            }
            Opcode::BNE => {
                if !registers.zero() {
                    if let Some(a) = address {
                        Ok(ExecutionResult::Address(a))
                    } else {
                        Err(Error::with_pc(registers.pc, ErrorType::MissingAddress))
                    }
                } else {
                    Ok(ExecutionResult::None)
                }
            }
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
            Opcode::Invalid(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{ram::Ram, ExecutionUnit, Memory};

    #[test]
    fn construct() {
        let _execution_unit: super::ExecutionUnit<Ram> = super::ExecutionUnit::new();
    }

    #[test]
    fn adc() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // Acc, Data, Carry, Result, N, Z, C
            (0x00, 0x00, false, 0x00, false, true, false),
            (0x00, 0x01, false, 0x01, false, false, false),
            (0xff, 0x01, false, 0x00, false, true, true),
            (0x7f, 0x01, false, 0x80, true, false, false),
        ];

        for (acc, data, carry_in, expected_result, neg, zero, carry) in test_cases {
            let case = format!(
                "A:{} + D:{} + C:{} = {} {}{}{}",
                acc,
                data,
                if carry_in { 1 } else { 0 },
                expected_result,
                if neg { "N" } else { "" },
                if zero { "Z" } else { "" },
                if carry { "C" } else { "" }
            );

            registers.write_flag(StatusBits::Carry, carry_in);
            registers.a = acc;

            let result =
                execution_unit.execute(&Opcode::ADC, Some(data), None, &memory, &mut registers)?;

            assert_eq!(result, ExecutionResult::Data(expected_result), "{}", case);
            assert_eq!(registers.carry(), carry, "C: {}", case);
            assert_eq!(registers.zero(), zero, "Z: {}", case);
            assert_eq!(registers.negative(), neg, "N: {}", case);
        }

        Ok(())
    }

    #[test]
    fn and() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // Acc, Data, Result, N, Z
            (0x00, 0x00, 0x00, false, true),
            (0x00, 0x01, 0x00, false, true),
            (0xff, 0xff, 0xff, true, false),
            (0xff, 0x01, 0x01, false, false),
        ];

        for (acc, data, expected_result, neg, zero) in test_cases {
            let case = format!(
                "A:{} & D:{} = {} {}{}",
                acc,
                data,
                expected_result,
                if neg { "N" } else { "" },
                if zero { "Z" } else { "" }
            );

            registers.a = acc;

            let result =
                execution_unit.execute(&Opcode::AND, Some(data), None, &memory, &mut registers)?;

            assert_eq!(result, ExecutionResult::Data(expected_result), "{}", case);
            assert_eq!(registers.zero(), zero, "{}", case);
            assert_eq!(registers.negative(), neg, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn asl() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // Data, Result, N, Z, C
            (0x00, 0x00, false, true, false),
            (0x01, 0x02, false, false, false),
            (0xff, 0xfe, true, false, true),
            (0x80, 0x00, false, true, true),
        ];

        for (data, expected_result, neg, zero, carry) in test_cases {
            let case = format!(
                "ASL D:{} = {} {}{}{}",
                data,
                expected_result,
                if neg { "N" } else { "" },
                if zero { "Z" } else { "" },
                if carry { "C" } else { "" }
            );

            let result =
                execution_unit.execute(&Opcode::ASL, Some(data), None, &memory, &mut registers)?;

            assert_eq!(result, ExecutionResult::Data(expected_result), "{}", case);
            assert_eq!(registers.carry(), carry, "{}", case);
            assert_eq!(registers.zero(), zero, "{}", case);
            assert_eq!(registers.negative(), neg, "{}", case);
        }
        Ok(())
    }

    #[test]
    fn bcc() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // addr, carry, expected result
            (0x1234, true, ExecutionResult::None),
            (0x1234, false, ExecutionResult::Address(0x1234)),
        ];

        for (address, carry, expected_result) in test_cases {
            let case = format!(
                "BCC {} (carry {}) = {:?}",
                address,
                if carry { "set" } else { "unset" },
                expected_result
            );
            registers.write_flag(StatusBits::Carry, carry);

            let result = execution_unit.execute(
                &Opcode::BCC,
                None,
                Some(address),
                &memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn bcs() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // addr, carry, expected result
            (0x1234, false, ExecutionResult::None),
            (0x1234, true, ExecutionResult::Address(0x1234)),
        ];

        for (address, carry, expected_result) in test_cases {
            let case = format!(
                "BCS {} (carry {}) = {:?}",
                address,
                if carry { "set" } else { "unset" },
                expected_result
            );
            registers.write_flag(StatusBits::Carry, carry);

            let result = execution_unit.execute(
                &Opcode::BCS,
                None,
                Some(address),
                &memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn beq() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // addr, zero, expected result
            (0x1234, false, ExecutionResult::None),
            (0x1234, true, ExecutionResult::Address(0x1234)),
        ];

        for (address, zero, expected_result) in test_cases {
            let case = format!(
                "BEQ {} (Z {}) = {:?}",
                address,
                if zero { "set" } else { "unset" },
                expected_result
            );
            registers.write_flag(StatusBits::Zero, zero);

            let result = execution_unit.execute(
                &Opcode::BEQ,
                None,
                Some(address),
                &memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn bit() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // acc, data, N, O, Z
            (0x00, 0x00, false, false, true),
            (0x01, 0x01, false, false, false),
            (0x00, 0xff, true, true, true),
            (0x80, 0x80, true, false, false),
            (0x00, 0x40, false, true, true),
        ];

        for (acc, data, neg, ovf, zero) in test_cases {
            let case = format!("BIT {} A {} => N {} O {} Z {}", data, acc, neg, ovf, zero);

            registers.a = acc;
            execution_unit.execute(&Opcode::BIT, Some(data), None, &memory, &mut registers)?;

            assert_eq!(registers.negative(), neg, "N: {}", case);
            assert_eq!(registers.overflow(), ovf, "O: {}", case);
            assert_eq!(registers.zero(), zero, "Z: {}", case);
        }

        Ok(())
    }

    #[test]
    fn bmi() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // addr, neg, expected result
            (0x1234, false, ExecutionResult::None),
            (0x1234, true, ExecutionResult::Address(0x1234)),
        ];

        for (address, neg, expected_result) in test_cases {
            let case = format!(
                "BMI {} (N {}) = {:?}",
                address,
                if neg { "set" } else { "unset" },
                expected_result
            );
            registers.write_flag(StatusBits::Neg, neg);

            let result = execution_unit.execute(
                &Opcode::BMI,
                None,
                Some(address),
                &memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn bne() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // addr, zero, expected result
            (0x1234, true, ExecutionResult::None),
            (0x1234, false, ExecutionResult::Address(0x1234)),
        ];

        for (address, zero, expected_result) in test_cases {
            let case = format!(
                "BNE {} (Z {}) = {:?}",
                address,
                if zero { "set" } else { "unset" },
                expected_result
            );
            registers.write_flag(StatusBits::Zero, zero);

            let result = execution_unit.execute(
                &Opcode::BNE,
                None,
                Some(address),
                &memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }
}
