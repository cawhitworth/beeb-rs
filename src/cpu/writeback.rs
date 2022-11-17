use std::marker::PhantomData;

use crate::cpu::{registers::Registers, Address, Memory, Result, Writeback};

use super::ExecutionResult;

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
        _data: ExecutionResult,
        _address: Option<Address>,
        _memory: &mut M,
        _registers: &mut Registers,
    ) -> Result<()> {
        todo!()
    }
}
