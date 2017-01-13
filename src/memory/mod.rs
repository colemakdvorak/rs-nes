pub mod nes_memory;

use seahash;
use std::io::Write;

#[cfg(test)]
mod spec_tests;

pub const ADDRESSABLE_MEMORY: usize = 65536;

pub trait Memory: 'static + Send + Clone {
    fn store(&mut self, u16, u8);
    fn load(&self, u16) -> u8;
    fn hash(&self) -> u64;
    fn dump<T: Write>(&self, writer: &mut T);
}

pub struct SimpleMemory {
    addr: [u8; ADDRESSABLE_MEMORY],
}

impl SimpleMemory {
    pub fn new() -> Self {
        SimpleMemory { addr: [0; ADDRESSABLE_MEMORY] }
    }

    pub fn store_many(&mut self, addr: u16, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.store(addr + i as u16, *byte);
        }
    }
}

impl Clone for SimpleMemory {
    fn clone(&self) -> Self {
        SimpleMemory { addr: self.addr }
    }
}

impl Default for SimpleMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for SimpleMemory {
    fn store(&mut self, addr: u16, data: u8) {
        let addr = addr as usize;
        self.addr[addr] = data;
    }

    fn load(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        self.addr[addr]
    }

    fn dump<T: Write>(&self, writer: &mut T) {
        writer.write_all(&self.addr).unwrap();
    }

    fn hash(&self) -> u64 {
        seahash::hash(&self.addr)
    }
}
