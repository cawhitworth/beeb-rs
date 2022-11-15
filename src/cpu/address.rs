use std::{marker::PhantomData, mem};

use crate::cpu::{Address, AddressingMode, Error, Memory, Registers, Result};

pub struct AddressDispatcher<M> {
    phantom: PhantomData<M>,
}

impl<M> AddressDispatcher<M>
where
    M: Memory,
{
    pub fn new() -> Self {
        AddressDispatcher {
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
        let offset = memory.read_byte(registers.pc + 1)? as u16;
        let address = (registers.pc).wrapping_add(offset);

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
        todo!()
    }

    fn indirect_x(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        todo!()
    }

    fn indirect_y(&self, memory: &M, registers: &Registers) -> Result<Option<Address>> {
        todo!()
    }
}

impl<M> crate::cpu::AddressDispatcher<M> for AddressDispatcher<M>
where
    M: Memory,
{
    fn dispatch(
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
            AddressingMode::None => Err(Error::InvalidAddressingMode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::ram::Ram;

    #[test]
    fn immediate() -> Result<()> {
        let address_dispatcher: AddressDispatcher<Ram> = AddressDispatcher::new();
        let mut _m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x10;

        let address = address_dispatcher.immediate(&_m, &r)?;
        assert_eq!(address, Some(0x11));
        
        Ok(())
    }

    #[test]
    fn zero_page() -> Result<()> {
        let address_dispatcher: AddressDispatcher<Ram> = AddressDispatcher::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        let expected_address = 0x7f;
        r.pc = 0x10;
        m.write_byte(r.pc + 1, expected_address)?;
        
        let address = address_dispatcher.zero_page(&m, &r)?;
        assert_eq!(address, Some(expected_address as u16));
        
        Ok(())
    }

    #[test]
    fn zero_page_x() -> Result<()> {
        let address_dispatcher: AddressDispatcher<Ram> = AddressDispatcher::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x10;
        r.x = 0x80;
        m.write_byte(r.pc + 1, 0x81)?;
        
        let address = address_dispatcher.zero_page_x(&m, &r)?;
        assert_eq!(address, Some(0x0001 as u16));
        
        Ok(())
    }

    #[test]
    fn zero_page_y() -> Result<()> {
        let address_dispatcher: AddressDispatcher<Ram> = AddressDispatcher::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x10;
        r.y = 0x80;
        m.write_byte(r.pc + 1, 0x81)?;
        
        let address = address_dispatcher.zero_page_y(&m, &r)?;
        assert_eq!(address, Some(0x0001 as u16));
        
        Ok(())
    }

    #[test]
    fn relative() -> Result<()> {
        let address_dispatcher: AddressDispatcher<Ram> = AddressDispatcher::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x00;
        m.write_byte(0x01, 0x10)?;

        let address = address_dispatcher.relative(&m, &r)?;
        assert_eq!(address, Some(0x10));

        Ok(())
    }

    #[test]
    fn absolute() -> Result<()> {
        let address_dispatcher: AddressDispatcher<Ram> = AddressDispatcher::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x00;
        m.write_word(0x01, 0x01234)?;
        
        let address = address_dispatcher.absolute(&m, &r)?;
        assert_eq!(address, Some(0x1234));

        Ok(())
    }

    #[test]
    fn absolute_x() -> Result<()> {
        let address_dispatcher: AddressDispatcher<Ram> = AddressDispatcher::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x00;
        r.x = 0x10;
        m.write_word(0x01, 0x01234)?;

        let address = address_dispatcher.absolute_x(&m, &r)?;
        assert_eq!(address, Some(0x1244));

        Ok(())
    }

    #[test]
    fn absolute_y() -> Result<()> {
        let address_dispatcher: AddressDispatcher<Ram> = AddressDispatcher::new();
        let mut m = Ram::new(65536);
        let mut r = Registers::new();

        r.pc = 0x00;
        r.y = 0x10;
        m.write_word(0x01, 0x01234)?;

        let address = address_dispatcher.absolute_y(&m, &r)?;
        assert_eq!(address, Some(0x1244));

        Ok(())
    }
}
