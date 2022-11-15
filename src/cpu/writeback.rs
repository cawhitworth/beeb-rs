use std::marker::PhantomData;

use crate::cpu::{registers::Registers, Address, Data, Error, Memory, Result, Writeback};

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
        writeback: &Writeback,
        data: Option<Data>,
        address: Option<Address>,
        memory: &mut M,
        registers: &mut Registers,
    ) -> Result<()> {
        todo!()
    }
}
