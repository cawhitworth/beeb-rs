use std::marker::PhantomData;

use crate::cpu::{Memory, Writeback, Data, Address, registers::Registers, Result, Error};

pub struct WritebackUnit<M>
{
    phantom: PhantomData<M>
}

impl<M> WritebackUnit<M>
where M: Memory {
    pub fn new() -> Self {
        WritebackUnit { phantom: PhantomData }
    }
}

impl<M> crate::cpu::WritebackUnit<M> for WritebackUnit<M>
where M: Memory {
    fn writeback(&self, writeback: &Writeback, data: Data, address: Address, memory: &mut M, registers: &mut Registers) -> Result<()> {
        todo!()
    }
}