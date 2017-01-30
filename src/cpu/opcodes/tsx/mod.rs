#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Tsx;

impl OpCode for Tsx {
    type Input = ();

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, _: AM) {
        cpu.registers.x = cpu.registers.sp;
        let x = cpu.registers.x;
        cpu.registers.set_sign_and_zero_flag(x);
        cpu.tick()
    }
}
