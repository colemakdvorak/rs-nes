use super::AddressingMode;
use cpu::Cpu;
use cpu::byte_utils::wrapping_add;
use memory::Memory;

pub struct ZeroPageY {
    addr: u16,
    value: u8,
}

impl ZeroPageY {
    pub fn init<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let addr = wrapping_add(cpu.read_pc(&tick_handler), cpu.registers.y) as u16;
        let val = cpu.read_memory(addr, &tick_handler);

        ZeroPageY {
            addr: addr,
            value: val,
        }
    }
}

impl<M: Memory> AddressingMode<M> for ZeroPageY {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
        cpu.write_memory(self.addr, value, &tick_handler);
    }
}