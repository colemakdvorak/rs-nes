#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use memory::Memory;
use screen::Screen;

pub struct Eor;

impl OpCode for Eor {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let rhs = am.read();
        let lhs = cpu.registers.acc;
        let res = lhs ^ rhs;
        cpu.registers.set_acc(res);
    }
}
