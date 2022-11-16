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
        opcode: &Opcode,
        data: Option<Data>,
        address: Option<Address>,
        _memory: &M,
        registers: &Registers,
    ) -> Result<Option<Data>> {
        let mut diss = format!("{:4x} : {:?}", registers.pc, opcode);
        if let Some(a) = address {
            diss += &format!(" &{:x}", a);
        }
        if let Some(d) = data {
            diss += &format!(" #{:x}", d)
        }
        println!("{}", diss);
        Ok(None)
    }
}
