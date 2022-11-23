use std::marker::PhantomData;

use crate::cpu::{
    registers::Registers, registers::StatusBits, Byte, Word, Address, Data, Error, ErrorType, ExecutionResult,
    Memory, Opcode, Result,
};

pub struct ExecutionUnit<M> {
    phantom: PhantomData<M>,
}

const STACK_BASE: Address = 0x100;

impl<M> ExecutionUnit<M>
where
    M: Memory,
{
    pub fn new() -> Self {
        ExecutionUnit {
            phantom: PhantomData,
        }
    }

    fn push_byte(
        &self, value: Byte, memory: &mut M, registers: &mut Registers
    ) -> Result<()> {
        memory.write_byte(registers.sp as Address + STACK_BASE, value)?;
        registers.sp = registers.sp.wrapping_sub(1);
        Ok(())
    }

    fn push_word(
        &self, value: Word, memory: &mut M, registers: &mut Registers
    ) -> Result<()> {
        self.push_byte((value >> 8) as Byte, memory, registers)?;
        self.push_byte((value & 0xff) as Byte, memory, registers)?;
        Ok(())
    }

    fn pop_byte(
        &self, memory: &mut M, registers: &mut Registers
    ) -> Result<Byte> {
        registers.sp = registers.sp.wrapping_add(1);
        memory.read_byte(registers.sp as Address + STACK_BASE)
    }

    fn pop_word(
        &self, memory: &mut M, registers: &mut Registers
    ) -> Result<Word> {
        let lsb = self.pop_byte(memory, registers)? as Word;
        let msb = self.pop_byte(memory, registers)? as Word;
        Ok(lsb + (msb << 8))
    }

    fn compare(&self, lhs: Byte, rhs: Byte, registers: &mut Registers) {
        let result = rhs.wrapping_sub(lhs);
        registers.write_flag(StatusBits::Neg, (result & 0x80) == 0x80);
        registers.write_flag(StatusBits::Zero, result == 0x00);

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
        memory: &mut M,
        registers: &mut Registers,
    ) -> Result<ExecutionResult> {
        match opcode {
            Opcode::ADC => {
                if let Some(d) = data {
                    let result: u16 = registers.a as u16 + d as u16 + u16::from(registers.carry());
                    let sign_a = registers.a & 0x80 == 0x80;
                    let sign_d = d & 0x80 == 0x80;
                    let sign_r = result & 0x80 == 0x80;

                    registers.write_flag(StatusBits::Neg, sign_r);
                    registers.write_flag(StatusBits::Zero, result & 0xff == 0);
                    registers.write_flag(StatusBits::Carry, result > 255);
                    registers.write_flag(StatusBits::Ovf, (sign_a == sign_d) && (sign_a != sign_r));
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
            Opcode::BPL => {
                if !registers.negative() {
                    if let Some(a) = address {
                        Ok(ExecutionResult::Address(a))
                    } else {
                        Err(Error::with_pc(registers.pc, ErrorType::MissingAddress))
                    }
                } else {
                    Ok(ExecutionResult::None)
                }
            }
            Opcode::BRK => {
                registers.write_flag(StatusBits::Brk, true);
                self.push_word(registers.pc + 2, memory, registers)?;
                self.push_byte(registers.ps, memory, registers)?;
                let a = memory.read_word(0xfffe)?;
                Ok(ExecutionResult::Address(a))
            }
            Opcode::BVC => 
            {
                if !registers.overflow() {
                    if let Some(a) = address {
                        Ok(ExecutionResult::Address(a))
                    } else {
                        Err(Error::with_pc(registers.pc, ErrorType::MissingAddress))
                    }
                } else {
                    Ok(ExecutionResult::None)
                }
            }
            Opcode::BVS => {
                if registers.overflow() {
                    if let Some(a) = address {
                        Ok(ExecutionResult::Address(a))
                    } else {
                        Err(Error::with_pc(registers.pc, ErrorType::MissingAddress))
                    }
                } else {
                    Ok(ExecutionResult::None)
                }
            }
            Opcode::CLC => {
                registers.clear_flag(StatusBits::Carry);
                Ok(ExecutionResult::None)
            }
            Opcode::CLD => {
                registers.clear_flag(StatusBits::Dec);
                Ok(ExecutionResult::None)
            }
            Opcode::CLI => {
                registers.clear_flag(StatusBits::Int);
                Ok(ExecutionResult::None)
            }
            Opcode::CLV => {
                registers.clear_flag(StatusBits::Ovf);
                Ok(ExecutionResult::None)
            }
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
            Opcode::SBC => todo!(), // take two's complement of data and ADC
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
    fn push_byte_succeeds() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(0x10000);
        let mut registers = Registers::new();
       
        registers.sp = 0xff;
        let _ = execution_unit.push_byte(0xde, &mut memory, &mut registers)?;

        assert_eq!(registers.sp, 0xfe);
        let b = memory.read_byte(0x1ff)?;

        assert_eq!(b, 0xde);

        Ok(())
    }

    #[test]
    fn push_byte_overflow_succeeds() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(0x10000);
        let mut registers = Registers::new();
       
        registers.sp = 0x00;
        let _ = execution_unit.push_byte(0xde, &mut memory, &mut registers)?;

        assert_eq!(registers.sp, 0xff);
        let b = memory.read_byte(0x100)?;

        assert_eq!(b, 0xde);

        Ok(())
    }

    #[test]
    fn push_word_succeeds() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(0x10000);
        let mut registers = Registers::new();
        
        registers.sp = 0xff;
        let _ = execution_unit.push_word(0xdead, &mut memory, &mut registers)?;

        assert_eq!(registers.sp, 0xfd);
        let lsb = memory.read_byte(0x1ff)?;
        let msb = memory.read_byte(0x1fe)?;

        assert_eq!(lsb, 0xde);
        assert_eq!(msb, 0xad);

        Ok(())
    }
    
    #[test]
    fn push_word_overflow_succeeds() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(0x10000);
        let mut registers = Registers::new();
        
        registers.sp = 0x01;
        let _ = execution_unit.push_word(0xdead, &mut memory, &mut registers)?;

        assert_eq!(registers.sp, 0xff);
        let lsb = memory.read_byte(0x101)?;
        let msb = memory.read_byte(0x100)?;

        assert_eq!(lsb, 0xde);
        assert_eq!(msb, 0xad);

        Ok(())
    }

    #[test]
    fn pop_byte_succeeds() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(0x10000);
        let mut registers = Registers::new();
        memory.write_byte(0x1ff, 0xde)?;
        registers.sp = 0xfe;
        let b = execution_unit.pop_byte(&mut memory, &mut registers)?;

        assert_eq!(registers.sp, 0xff);
        assert_eq!(b, 0xde);

        Ok(())
    }
    
    #[test]
    fn pop_byte_underflow_succeeds() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(0x10000);
        let mut registers = Registers::new();

        memory.write_byte(0x100, 0xde)?;
        
        registers.sp = 0xff;
        let b = execution_unit.pop_byte(&mut memory, &mut registers)?;

        assert_eq!(registers.sp, 0x00);
        assert_eq!(b, 0xde);

        Ok(())
    }
    
    #[test]
    fn pop_word_succeeds() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(0x10000);
        let mut registers = Registers::new();
       
        memory.write_byte(0x1ff, 0xde)?;
        memory.write_byte(0x1fe, 0xad)?;

        registers.sp = 0xfd;
        let r = execution_unit.pop_word(&mut memory, &mut registers)?;

        assert_eq!(registers.sp, 0xff);
        assert_eq!(r, 0xdead);

        Ok(())
    }
    
    #[test]
    fn pop_word_underflow_succeeds() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(0x10000);
        let mut registers = Registers::new();
       
        memory.write_byte(0x100, 0xde)?;
        memory.write_byte(0x1ff, 0xad)?;

        registers.sp = 0xfe;
        let r = execution_unit.pop_word(&mut memory, &mut registers)?;

        assert_eq!(registers.sp, 0x00);
        assert_eq!(r, 0xdead);

        Ok(())
    }

    #[test]
    fn adc() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // Acc, Data, Carry, Result, N, Z, C, V
            (0x00, 0x00, false, 0x00, false, true, false, false),
            (0x00, 0x01, false, 0x01, false, false, false, false),
            (0xff, 0x01, false, 0x00, false, true, true, false),
            (0x7f, 0x01, false, 0x80, true, false, false, true),
            (0x70, 0x70, false, 0xe0, true, false, false, true),
            (0xff, 0x80, false, 0x7f, false, false, true, true),
        ];

        for (acc, data, carry_in, expected_result, neg, zero, carry, overflow) in test_cases {
            let case = format!(
                "A:{} + D:{} + C:{} = {} {}{}{}{}",
                acc,
                data,
                if carry_in { 1 } else { 0 },
                expected_result,
                if neg { "N" } else { "" },
                if zero { "Z" } else { "" },
                if carry { "C" } else { "" },
                if overflow { "V" } else { "" },
            );

            registers.write_flag(StatusBits::Carry, carry_in);
            registers.a = acc;

            let result =
                execution_unit.execute(&Opcode::ADC, Some(data), None, &mut memory, &mut registers)?;

            assert_eq!(result, ExecutionResult::Data(expected_result), "{}", case);
            assert_eq!(registers.carry(), carry, "C: {}", case);
            assert_eq!(registers.zero(), zero, "Z: {}", case);
            assert_eq!(registers.negative(), neg, "N: {}", case);
            assert_eq!(registers.overflow(), overflow, "V: {}", case);
        }

        Ok(())
    }

    #[test]
    fn and() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
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
                execution_unit.execute(&Opcode::AND, Some(data), None, &mut memory, &mut registers)?;

            assert_eq!(result, ExecutionResult::Data(expected_result), "{}", case);
            assert_eq!(registers.zero(), zero, "{}", case);
            assert_eq!(registers.negative(), neg, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn asl() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
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
                execution_unit.execute(&Opcode::ASL, Some(data), None, &mut memory, &mut registers)?;

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
        let mut memory = Ram::new(1);
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
                &mut memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn bcs() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
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
                &mut memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn beq() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
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
                &mut memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn bit() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
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
            execution_unit.execute(&Opcode::BIT, Some(data), None, &mut memory, &mut registers)?;

            assert_eq!(registers.negative(), neg, "N: {}", case);
            assert_eq!(registers.overflow(), ovf, "O: {}", case);
            assert_eq!(registers.zero(), zero, "Z: {}", case);
        }

        Ok(())
    }

    #[test]
    fn bmi() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
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
                &mut memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn bne() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
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
                &mut memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }
    #[test]

    fn bpl() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // addr, neg, expected result
            (0x1234, true, ExecutionResult::None),
            (0x1234, false, ExecutionResult::Address(0x1234)),
        ];

        for (address, neg, expected_result) in test_cases {
            let case = format!(
                "BPL {} (N {}) = {:?}",
                address,
                if neg { "set" } else { "unset" },
                expected_result
            );
            registers.write_flag(StatusBits::Neg, neg);

            let result = execution_unit.execute(
                &Opcode::BPL,
                None,
                Some(address),
                &mut memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn brk() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(0x10000);
        let mut registers = Registers::new();

        registers.write_flag(StatusBits::Brk, false);
        registers.sp = 0xff;
        registers.pc = 0xff00;
        registers.write_flag(StatusBits::Neg, true);
        memory.write_word(0xfffe, 0x1234)?;

        let expected_ps = registers.ps | StatusBits::Brk as u8;
        let expected_pc = registers.pc + 2;
        let expected_sp = registers.sp.wrapping_sub(3);

        let r = execution_unit.execute(&Opcode::BRK, None, None, &mut memory, &mut registers)?;

        assert!(registers.brk());
        assert_eq!(registers.sp, expected_sp);
        assert_eq!(r, ExecutionResult::Address(0x1234));

        let ps = execution_unit.pop_byte(&mut memory, &mut registers)?;
        assert_eq!(ps, expected_ps);
        let pc = execution_unit.pop_word(&mut memory, &mut registers)?;
        assert_eq!(pc, expected_pc);

        Ok(())

    }

    #[test]
    fn bvc() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // addr, ovf, expected result
            (0x1234, true, ExecutionResult::None),
            (0x1234, false, ExecutionResult::Address(0x1234)),
        ];

        for (address, ovf, expected_result) in test_cases {
            let case = format!(
                "BVC {} (N {}) = {:?}",
                address,
                if ovf { "set" } else { "unset" },
                expected_result
            );
            registers.write_flag(StatusBits::Ovf, ovf);

            let result = execution_unit.execute(
                &Opcode::BVC,
                None,
                Some(address),
                &mut memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }
    
    #[test]
    fn bvs() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
        let mut registers = Registers::new();

        let test_cases = vec![
            // addr, ovf, expected result
            (0x1234, false, ExecutionResult::None),
            (0x1234, true, ExecutionResult::Address(0x1234)),
        ];

        for (address, ovf, expected_result) in test_cases {
            let case = format!(
                "BVS {} (N {}) = {:?}",
                address,
                if ovf { "set" } else { "unset" },
                expected_result
            );
            registers.write_flag(StatusBits::Ovf, ovf);

            let result = execution_unit.execute(
                &Opcode::BVS,
                None,
                Some(address),
                &mut memory,
                &mut registers,
            )?;

            assert_eq!(result, expected_result, "{}", case);
        }

        Ok(())
    }

    #[test]
    fn clc() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
        let mut registers = Registers::new();

        registers.write_flag(StatusBits::Carry, true);
        let _ = execution_unit.execute(&Opcode::CLC, None, None, &mut memory, &mut registers)?;

        assert!(!registers.carry(), "Carry should be clear");
        Ok(())
    }

    #[test]
    fn cld() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
        let mut registers = Registers::new();

        registers.write_flag(StatusBits::Dec, true);
        let _ = execution_unit.execute(&Opcode::CLD, None, None, &mut memory, &mut registers)?;

        assert!(!registers.dec(), "Dec should be clear");
        Ok(())
    }

    #[test]
    fn cli() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
        let mut registers = Registers::new();

        registers.write_flag(StatusBits::Int, true);
        let _ = execution_unit.execute(&Opcode::CLI, None, None, &mut memory, &mut registers)?;

        assert!(!registers.int(), "Int should be clear");
        Ok(())
    }
    
    #[test]
    fn clv() -> Result<()> {
        let execution_unit = super::ExecutionUnit::new();
        let mut memory = Ram::new(1);
        let mut registers = Registers::new();

        registers.write_flag(StatusBits::Ovf, true);
        let _ = execution_unit.execute(&Opcode::CLV, None, None, &mut memory, &mut registers)?;

        assert!(!registers.overflow(), "Overflow should be clear");
        Ok(())
    }
}
