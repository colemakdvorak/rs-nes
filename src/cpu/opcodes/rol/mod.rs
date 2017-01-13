#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::shift_base::shift_left;
use super::OpCode;

pub struct Rol;

impl OpCode for Rol {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let carry_set = cpu.registers.carry_flag();
        shift_left(cpu, am, carry_set, &tick_handler)
    }
}