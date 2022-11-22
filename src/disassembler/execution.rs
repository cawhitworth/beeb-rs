use std::marker::PhantomData;

use crate::cpu::{registers::Registers, Address, Data, ExecutionResult, Memory, Opcode, Result};

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
        _memory: &mut M,
        registers: &mut Registers,
    ) -> Result<ExecutionResult> {
        let mut diss = format!("{:04x} : ", registers.pc);

        match opcode {
            Opcode::Invalid(o) => {
                diss += &format!("({:02x})", o);
            }
            _ => {
                diss += &format!("{:?}", opcode);
            }
        }

        if let Some(a) = address {
            diss += &format!(" &{:x}", a);
        }
        if let Some(d) = data {
            diss += &format!(" #{:x}", d)
        }
        println!("{}", diss);
        Ok(ExecutionResult::None)
    }
}
