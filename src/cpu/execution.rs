use std::marker::PhantomData;

use crate::cpu::{Memory, Opcode, Data, Address, registers::Registers, Result, Error};

pub struct ExecutionUnit<M>
{
    phantom: PhantomData<M>
}

impl<M> ExecutionUnit<M>
where M: Memory {
    pub fn new() -> Self {
        ExecutionUnit { phantom: PhantomData }
    }
}

impl<M> crate::cpu::ExecutionUnit<M> for ExecutionUnit<M>
where M: Memory {
    fn execute(&self, opcode: &Opcode, data: Option<Data>, address: Option<Address>, memory: &M, registers: &Registers) -> Result<Option<Data>> {
        todo!()
    }
}