pub const MEMORY_SIZE: usize = 0xFFFF; // 65535
pub const MEMORY_ADDRESS_RAM_START: u16 = 0x0200;
pub const MEMORY_ADDRESS_RAM_END: u16 = 0xFFFF;
// TODO: Implement the last memory address to target the BRK and other related instructions
pub const MEMORY_RAM_SIZE: u16 = MEMORY_ADDRESS_RAM_END - MEMORY_ADDRESS_RAM_START;

#[derive(Debug)]
pub struct Memory([u8; MEMORY_SIZE]);

impl Memory {
  pub fn new() -> Self {
    Self([0x00; MEMORY_SIZE])
  }

  pub fn read_byte(&self, address: u16) -> u8 {
    self.0[address as usize]
  }

  pub fn write_byte(&mut self, address: u16, value: u8) {
    self.0[address as usize] = value;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_write_an_address_on_memory() {
    let mut memory = Memory::new();

    memory.write_byte(0x0000, 0x42);
    memory.write_byte(0x0001, 0x43);

    assert_eq!(memory.read_byte(0x0000), 0x42);
    assert_eq!(memory.read_byte(0x0001), 0x43);
  }
}
