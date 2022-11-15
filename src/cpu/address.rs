use std::marker::PhantomData;

use crate::cpu::{Address, Result, AddressingMode, Memory, Registers, Error};

pub struct AddressDispatcher<M> {
    phantom: PhantomData<M>
}

impl<M> AddressDispatcher<M>
where M: Memory {
    pub fn new() -> Self {
        AddressDispatcher { phantom: PhantomData }
    }

    fn implicit(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn accumulator(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn immediate(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn zero_page(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn zero_page_x(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn zero_page_y(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn relative(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn absolute(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn absolute_x(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn absolute_y(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn indirect(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn indirect_x(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }

    fn indirect_y(&self, memory: &M, registers: &Registers) -> Result<Address> {
        todo!()
    }
}

impl<M> crate::cpu::AddressDispatcher<M> for AddressDispatcher<M>
where M: Memory {
    fn dispatch(&self, mode: &AddressingMode, memory: &M, registers: &Registers) -> Result<Address> {
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