use std::marker::PhantomData;

use crate::cpu::{registers::Registers, Address, Data, Memory, Result, Writeback};

pub struct WritebackUnit<M> {
    phantom: PhantomData<M>,
}

impl<M> WritebackUnit<M>
where
    M: Memory,
{
    pub fn new() -> Self {
        WritebackUnit {
            phantom: PhantomData,
        }
    }
}

impl<M> crate::cpu::WritebackUnit<M> for WritebackUnit<M>
where
    M: Memory,
{
    fn writeback(
        &self,
        _writeback: &Writeback,
        _data: Option<Data>,
        _address: Option<Address>,
        _memory: &mut M,
        _registers: &mut Registers,
    ) -> Result<()> {
        todo!()
    }
}
