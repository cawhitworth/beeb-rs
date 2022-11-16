use std::marker::PhantomData;

use crate::cpu::{registers::Registers, Address, Data, Memory, Opcode, Result};

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
        _opcode: &Opcode,
        _data: Option<Data>,
        _address: Option<Address>,
        _memory: &M,
        _registers: &Registers,
    ) -> Result<Option<Data>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let _execution_unit: ExecutionUnit<crate::cpu::ram::Ram> = ExecutionUnit::new();
    }
}