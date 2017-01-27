use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct ZeroPage {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl ZeroPage {
    pub fn init<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let addr = cpu.read_pc(&tick_handler) as u16;
        let val = cpu.read_memory(addr, &tick_handler);

        ZeroPage {
            addr: addr,
            value: val,
            is_store: false,
        }
    }

    pub fn init_store<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let addr = cpu.read_pc(&tick_handler) as u16;

        // Read must consume a cycle for stores, so we call cpu.memory.load() directly
        let val = cpu.memory.load(addr);

        ZeroPage {
            addr: addr,
            value: val,
            is_store: true,
        }
    }
}

impl<M: Memory> AddressingMode<M> for ZeroPage {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
        if !self.is_store {
            // Dummy write cycle
            tick_handler(cpu);
        }
        cpu.write_memory(self.addr, value, &tick_handler)
    }
}
