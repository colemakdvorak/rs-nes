#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::compare_base::compare;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Cpy;

impl OpCode for Cpy {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let val = cpu.registers.y;
        compare(cpu, am, val);
    }
}