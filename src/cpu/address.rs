use std::marker::PhantomData;

use crate::cpu::{Address, AddressingMode, Error, ErrorType, Memory, Registers, Result};

use super::Data;

pub struct AddressAndDataDispatch<M> {
    phantom: PhantomData<M>,
}

impl<M> AddressAndDataDispatch<M>
where
    M: Memory,
{
    pub fn new() -> Self {
        AddressAndDataDispatch {
            phantom: PhantomData,
        }
    }

    fn implicit(&self, _memory: &M, _registers: &Registers) -> Result<Option<Address>> {
        Ok(None)
    }

    fn accumulator(&self, _memory: &M, _registers: &Registers) -> Result<Option<Address>> {
        Ok(None)
    }

    fn immediate(&self, _memory: &M, registers: &Registers) -> Result<Option<Address>> {
        Ok(Some(registers.pc + 1))
    }

    fn zero_page(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        let zero_page = memory.read_byte(registers.pc + 1)?;
        Ok(Some(zero_page as u16))
    }

    fn zero_page_x(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        let zero_page = memory.read_byte(registers.pc + 1)? as u16;
        let zero_page_x = zero_page + registers.x as u16;
        Ok(Some(zero_page_x & 0x00ff))
    }

    fn zero_page_y(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        let zero_page = memory.read_byte(registers.pc + 1)? as u16;
        let zero_page_y = zero_page + registers.y as u16;
        Ok(Some(zero_page_y & 0x00ff))
    }

    fn relative(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        let offset = memory.read_byte(registers.pc + 1)? as i8;
        let address = (registers.pc_next).wrapping_add(offset as u16);

        Ok(Some(address))
    }

    fn absolute(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        let address = memory.read_word(registers.pc + 1)?;

        Ok(Some(address))
    }

    fn absolute_x(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        let address = memory.read_word(registers.pc + 1)?;
        Ok(Some(address + registers.x as u16))
    }

    fn absolute_y(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        let address = memory.read_word(registers.pc + 1)?;
        Ok(Some(address + registers.y as u16))
    }

    fn indirect(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        let indir_address = memory.read_word(registers.pc + 1)?;
        let address = memory.read_word(indir_address as Address)?;
        Ok(Some(address))
    }

    fn indirect_x(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        let mut indir_address = memory.read_word(registers.pc + 1)?;
        indir_address += registers.x as u16;
        indir_address &= 0xff;
        let address = memory.read_word(indir_address as Address)?;
        Ok(Some(address))
    }

    fn indirect_y(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        let indir_address = memory.read_word(registers.pc + 1)?;
        let address = memory.read_word(indir_address as Address)?;
        Ok(Some(address + registers.y as u16))
    }
}

impl<M> crate::cpu::AddressDataDispatcher<M> for AddressAndDataDispatch<M>
where
    M: Memory,
{
    fn get_address(
        &self,
        mode: &AddressingMode,
        memory: &M,
        registers: &Registers,
    ) -> Result<Option<Address>> {
        match mode {
            AddressingMode::Implicit => self.implicit(memory, registers),
            AddressingMode::Accumulator => self.accumulator(memory, registers),
            AddressingMode::Immediate => self.immediate(memory, registers),
            AddressingMode::ZeroPage => self.zero_page(memory, registers),
            AddressingMode::ZeroPageX => self.zero_page_x(memory, registers),
            AddressingMode::ZeroPageY => self.zero_page_y(memory, registers),
            AddressingMode::Relative => self.relative(memory, registers),
            AddressingMode::Absolute => self.absolute(memory, registers),
            AddressingMode::AbsoluteX => self.absolute_x(memory, registers),
            AddressingMode::AbsoluteY => self.absolute_y(memory, registers),
            AddressingMode::Indirect => self.indirect(memory, registers),
            AddressingMode::IndirectX => self.indirect_x(memory, registers),
            AddressingMode::IndirectY => self.indirect_y(memory, registers),
            AddressingMode::None => Err(Error::with_pc(
                registers.pc,
                ErrorType::InvalidAddressingMode,
            )),
        }
    }

    fn get_data(
        &self,
        mode: &AddressingMode,
        memory: &M,
        registers: &Registers,
    ) -> Result<Option<Data>> {
        match mode {
            AddressingMode::Implicit => Ok(None),
            AddressingMode::Accumulator => Ok(Some(registers.a)),
            AddressingMode::Immediate => {
                let address = self.immediate(memory, registers)?;
                let b = memory.read_byte(address.unwrap())?;
                Ok(Some(b))
            }
            AddressingMode::ZeroPage => {
                let address = self.zero_page(memory, registers)?;
                let b = memory.read_byte(address.unwrap())?;
                Ok(Some(b))
            }
            AddressingMode::ZeroPageX => {
                let address = self.zero_page_x(memory, registers)?;
                let b = memory.read_byte(address.unwrap())?;
                Ok(Some(b))
            }
            AddressingMode::ZeroPageY => {
                let address = self.zero_page_y(memory, registers)?;
                let b = memory.read_byte(address.unwrap())?;
                Ok(Some(b))
            }
            AddressingMode::Relative => {
                let address = self.relative(memory, registers)?;
                let b = memory.read_byte(address.unwrap())?;
                Ok(Some(b))
            }
            AddressingMode::Absolute => {
                let address = self.absolute(memory, registers)?;
                let b = memory.read_byte(address.unwrap())?;
                Ok(Some(b))
            }
            AddressingMode::AbsoluteX => {
                let address = self.absolute_x(memory, registers)?;
                let b = memory.read_byte(address.unwrap())?;
                Ok(Some(b))
            }
            AddressingMode::AbsoluteY => {
                let address = self.absolute_y(memory, registers)?;
                let b = memory.read_byte(address.unwrap())?;
                Ok(Some(b))
            }
            AddressingMode::Indirect => {
                let address = self.indirect(memory, registers)?;
                let b = memory.read_byte(address.unwrap())?;
                Ok(Some(b))
            }
            AddressingMode::IndirectX => {
                let address = self.indirect_x(memory, registers)?;
                let b = memory.read_byte(address.unwrap())?;
                Ok(Some(b))
            }
            AddressingMode::IndirectY => {
                let address = self.indirect_y(memory, registers)?;
                let b = memory.read_byte(address.unwrap())?;
                Ok(Some(b))
            }
            AddressingMode::None => Err(Error::with_pc(
                registers.pc,
                ErrorType::InvalidAddressingMode,
            )),
        }
    }
}

#[cfg(test)]
mod address_tests {
    use super::*;
    use crate::cpu::{ram::Ram, AddressDataDispatcher};

    #[test]
    fn implicit() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let m = Ram::new(65536);
        let r = Registers::new();

        let address = address_dispatcher.get_address(&AddressingMode::Implicit, &m, &r)?;
        assert_eq!(address, None);

        Ok(())
    }

    #[test]
    fn accumulator() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let m = Ram::new(65536);
        let mut r = Registers::new();
        r.a = 0x10;

        let address = address_dispatcher.get_address(&AddressingMode::Accumulator, &m, &r)?;
        assert_eq!(address, None);

        Ok(())
    }

    #[test]
    fn immediate() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let mut _m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x10;

        let address = address_dispatcher.get_address(&AddressingMode::Immediate, &_m, &r)?;
        assert_eq!(address, Some(0x11));

        Ok(())
    }

    #[test]
    fn zero_page() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        let expected_address = 0x7f;
        r.pc = 0x10;
        m.write_byte(r.pc + 1, expected_address)?;

        let address = address_dispatcher.get_address(&AddressingMode::ZeroPage, &m, &r)?;
        assert_eq!(address, Some(expected_address as u16));

        Ok(())
    }

    #[test]
    fn zero_page_x() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x10;
        r.x = 0x80;
        m.write_byte(r.pc + 1, 0x81)?;

        let address = address_dispatcher.get_address(&AddressingMode::ZeroPageX, &m, &r)?;
        assert_eq!(address, Some(0x0001 as u16));

        Ok(())
    }

    #[test]
    fn zero_page_y() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x10;
        r.y = 0x80;
        m.write_byte(r.pc + 1, 0x81)?;

        let address = address_dispatcher.get_address(&AddressingMode::ZeroPageY, &m, &r)?;
        assert_eq!(address, Some(0x0001 as u16));

        Ok(())
    }

    #[test]
    fn relative() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x10;
        r.pc_next = 0x10;
        m.write_byte(0x11, 0xff)?;

        let address = address_dispatcher.get_address(&AddressingMode::Relative, &m, &r)?;
        assert_eq!(address, Some(0x0f));

        Ok(())
    }

    #[test]
    fn absolute() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x00;
        m.write_word(0x01, 0x01234)?;

        let address = address_dispatcher.get_address(&AddressingMode::Absolute, &m, &r)?;
        assert_eq!(address, Some(0x1234));

        Ok(())
    }

    #[test]
    fn absolute_x() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x00;
        r.x = 0x10;
        m.write_word(0x01, 0x01234)?;

        let address = address_dispatcher.get_address(&AddressingMode::AbsoluteX, &m, &r)?;
        assert_eq!(address, Some(0x1244));

        Ok(())
    }

    #[test]
    fn absolute_y() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x00;
        r.y = 0x10;
        m.write_word(0x01, 0x01234)?;

        let address = address_dispatcher.get_address(&AddressingMode::AbsoluteY, &m, &r)?;
        assert_eq!(address, Some(0x1244));

        Ok(())
    }

    #[test]
    fn indirect() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x00;
        m.write_word(0x01, 0x1234)?;
        m.write_word(0x1234, 0x4567)?;

        let address = address_dispatcher.get_address(&AddressingMode::Indirect, &m, &r)?;

        assert_eq!(address, Some(0x4567));

        Ok(())
    }

    #[test]
    fn indirect_x() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x00;
        r.x = 0x10;
        m.write_word(0x01, 0x1234)?;
        m.write_word(0x44, 0x4567)?;

        let address = address_dispatcher.get_address(&AddressingMode::IndirectX, &m, &r)?;

        assert_eq!(address, Some(0x4567));

        Ok(())
    }

    #[test]
    fn indirect_y() -> Result<()> {
        let address_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x00;
        r.y = 0x10;
        m.write_word(0x01, 0x1234)?;
        m.write_word(0x1234, 0x4567)?;

        let address = address_dispatcher.get_address(&AddressingMode::IndirectY, &m, &r)?;

        assert_eq!(address, Some(0x4577));

        Ok(())
    }
}

#[cfg(test)]
mod data_tests {
    use crate::cpu::{ram::Ram, AddressDataDispatcher};

    use super::*;

    #[test]
    fn implicit() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let m = Ram::new(65536);
        let r = Registers::new();

        let data = data_dispatcher.get_data(&AddressingMode::Implicit, &m, &r)?;
        assert_eq!(data, None);

        Ok(())
    }

    #[test]
    fn accumulator() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let m = Ram::new(65536);
        let mut r = Registers::new();

        r.a = 0x10;
        let data = data_dispatcher.get_data(&AddressingMode::Accumulator, &m, &r)?;
        assert_eq!(data, Some(0x10));

        Ok(())
    }

    #[test]
    fn immediate() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x00;
        m.write_byte(0x0001, 0x7f)?;
        let data = data_dispatcher.get_data(&AddressingMode::Immediate, &m, &r)?;

        assert_eq!(data, Some(0x7f));

        Ok(())
    }

    #[test]
    fn zero_page() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();
        r.pc = 0x00;
        m.write_byte(0x0001, 0x7f)?;
        m.write_byte(0x007f, 0xaa)?;

        let data = data_dispatcher.get_data(&AddressingMode::ZeroPage, &m, &r)?;
        assert_eq!(data, Some(0xaa));

        Ok(())
    }

    #[test]
    fn zero_page_x() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();
        r.pc = 0x00;
        r.x = 0x10;
        m.write_byte(0x0001, 0xff)?;
        m.write_byte(0x000f, 0xaa)?;

        let data = data_dispatcher.get_data(&AddressingMode::ZeroPageX, &m, &r)?;
        assert_eq!(data, Some(0xaa));

        Ok(())
    }

    #[test]
    fn zero_page_y() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();
        r.pc = 0x00;
        r.y = 0x10;
        m.write_byte(0x0001, 0xff)?;
        m.write_byte(0x000f, 0xaa)?;

        let data = data_dispatcher.get_data(&AddressingMode::ZeroPageY, &m, &r)?;
        assert_eq!(data, Some(0xaa));

        Ok(())
    }

    #[test]
    fn relative() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x10;
        r.pc_next = 0x10;
        m.write_byte(0x11, 0xff)?;
        m.write_byte(0x0f, 0xaa)?;

        let data = data_dispatcher.get_data(&AddressingMode::Relative, &m, &r)?;
        assert_eq!(data, Some(0xaa));

        Ok(())
    }

    #[test]
    fn absolute() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x0010;
        m.write_word(0x0011, 0x1234)?;
        m.write_byte(0x1234, 0xaa)?;

        let data = data_dispatcher.get_data(&AddressingMode::Absolute, &m, &r)?;
        assert_eq!(data, Some(0xaa));

        Ok(())
    }

    #[test]
    fn absolute_x() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x0010;
        r.x = 0x10;
        m.write_word(0x0011, 0x1234)?;
        m.write_byte(0x1244, 0xaa)?;

        let data = data_dispatcher.get_data(&AddressingMode::AbsoluteX, &m, &r)?;
        assert_eq!(data, Some(0xaa));

        Ok(())
    }

    #[test]
    fn absolute_y() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x0010;
        r.y = 0x10;
        m.write_word(0x0011, 0x1234)?;
        m.write_byte(0x1244, 0xaa)?;

        let data = data_dispatcher.get_data(&AddressingMode::AbsoluteY, &m, &r)?;
        assert_eq!(data, Some(0xaa));

        Ok(())
    }

    #[test]
    fn indirect() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x0010;
        m.write_word(0x0011, 0x1234)?;
        m.write_word(0x1234, 0x5678)?;
        m.write_byte(0x5678, 0xaa)?;

        let data = data_dispatcher.get_data(&AddressingMode::Indirect, &m, &r)?;
        assert_eq!(data, Some(0xaa));

        Ok(())
    }

    #[test]
    fn indirect_x() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x0010;
        r.x = 0x01;
        m.write_byte(0x0011, 0xff)?;
        m.write_word(0x0000, 0x1234)?;
        m.write_byte(0x1234, 0xaa)?;

        let data = data_dispatcher.get_data(&AddressingMode::IndirectX, &m, &r)?;
        assert_eq!(data, Some(0xaa));

        Ok(())
    }

    #[test]
    fn indirect_y() -> Result<()> {
        let data_dispatcher = AddressAndDataDispatch::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x0010;
        r.y = 0x10;
        m.write_byte(0x0011, 0xff)?;
        m.write_word(0x00ff, 0x1234)?;
        m.write_byte(0x1244, 0xaa)?;

        let data = data_dispatcher.get_data(&AddressingMode::IndirectY, &m, &r)?;
        assert_eq!(data, Some(0xaa));

        Ok(())
    }
}
