use crate::instructions::{Instruction, Opcode};
use crate::memory::{
  Memory,
  MEMORY_ADDRESS_RAM_START,
  MEMORY_RAM_SIZE, MEMORY_STACK_START,
};

#[derive(Debug)]
pub struct CpuStatusRegister {
  pub carry: bool,
  pub zero: bool,
  pub interrupt_disable: bool,
  pub decimal_mode: bool,
  pub break_command: bool,
  pub overflow: bool,
  pub negative: bool,
}

impl CpuStatusRegister {
  pub fn new() -> Self {
    Self {
      carry: false,
      zero: false,
      interrupt_disable: true,
      decimal_mode: false,
      break_command: false,
      overflow: false,
      negative: false,
    }
  }

  pub fn from_byte(byte: u8) -> Self {
    Self {
      carry: (byte & 0b0000_0001) != 0,
      zero: (byte & 0b0000_0010) != 0,
      interrupt_disable: (byte & 0b0000_0100) != 0,
      decimal_mode: (byte & 0b0000_1000) != 0,
      break_command: (byte & 0b0001_0000) != 0,
      overflow: (byte & 0b0100_0000) != 0,
      negative: (byte & 0b1000_0000) != 0,
    }
  }

  pub fn as_byte(&self) -> u8 {
    let mut byte = 0;
    byte |= if self.carry { 1 } else { 0 } << 0;
    byte |= if self.zero { 1 } else { 0 } << 1;
    byte |= if self.interrupt_disable { 1 } else { 0 } << 2;
    byte |= if self.decimal_mode { 1 } else { 0 } << 3;
    byte |= if self.break_command { 1 } else { 0 } << 4;
    byte |= if self.overflow { 1 } else { 0 } << 6;
    byte |= if self.negative { 1 } else { 0 } << 7;
    byte
  }
}

const IRQ_ADDRESS: u16 = 0xFFFE;

#[derive(Debug)]
pub struct Cpu {
  memory: Memory,
  /// The program counter
  pc: u16,
  /// The stack pointer
  sp: u8,
  /// The accumulator
  acc: u8,
  /// The X register
  x: u8,
  /// The Y register
  y: u8,
  /// The status register
  status: CpuStatusRegister,
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      memory: Memory::new(),
      pc: MEMORY_ADDRESS_RAM_START,
      sp: 0x00FF,
      acc: 0,
      x: 0,
      y: 0,
      status: CpuStatusRegister::new(),
    }
  }

  pub fn load_program(&mut self, program: Vec<u8>) -> Result<(), &'static str> {
    if program.len() > MEMORY_RAM_SIZE as usize {
      return Err("Program is too large to fit in memory");
    }

    for (i, byte) in program.iter().enumerate() {
      let address = MEMORY_ADDRESS_RAM_START as usize + i;
      self.memory.write_byte(address as u16, *byte);
    }

    Ok(())
  }

  pub fn tick(&mut self) {
    loop {
      let instruction = self.fetch_and_decode();
      self.execute(instruction);
      // TODO: Just halt when reach IRQ address for now
      if self.pc == IRQ_ADDRESS {
        break;
      }
    }
  }

  fn fetch_and_decode(&mut self) -> Instruction {
    let opcode = self.memory.read_byte(self.pc);
    let a = self.memory.read_byte(self.pc + 1);
    let b = self.memory.read_byte(self.pc + 2);
    Instruction::decode(opcode, a, b)
  }

  fn execute(&mut self, instruction: Instruction) {
    match instruction.opcode {
      Opcode::BRK => self.execute_opcode_brk(),
      Opcode::LDA => self.execute_opcode_lda(instruction),
      _ => todo!("instruction not implemented yet"),
    }
  }

  fn execute_opcode_brk(&mut self) {
    self.pc += 2;
    self.status_flag_update_break(true);
    self.status_flag_update_interrupt(true);
    self.stack_push(((self.pc >> 8) & 0xFF) as u8);
    self.stack_push((self.pc & 0xFF) as u8);
    self.stack_push(self.status.as_byte());
    self.pc = IRQ_ADDRESS;
  }

  fn execute_opcode_lda(&mut self, instruction: Instruction) {
    let value = instruction.resolve_operand_value(self);
    self.acc = value;
    self.pc += 2;
    self.status_flag_update_negative(value);
    self.status_flag_update_zero(value);
  }

  fn stack_push(&mut self, value: u8) {
    self.memory.write_byte(MEMORY_STACK_START + self.sp as u16, value);
    self.sp = self.sp.wrapping_sub(1);
  }

  fn status_flag_update_negative(&mut self, value: u8) {
    self.status.negative = value == 0;
  }
  
  fn status_flag_update_zero(&mut self, value: u8) {
    self.status.zero = value == 0;
  }

  fn status_flag_update_interrupt(&mut self, value: bool) {
    self.status.interrupt_disable = value;
  }

  fn status_flag_update_break(&mut self, value: bool) {
    self.status.break_command = value;
  }
}

#[cfg(test)]
mod tests {
  use crate::address_mode::AddressMode;
  use super::*;

  #[test]
  fn should_create_a_new_cpu() {
    let cpu = Cpu::new();

    assert_eq!(cpu.pc, 0x0200);
    assert_eq!(cpu.sp, 0x00FF);
    assert_eq!(cpu.acc, 0);
    assert_eq!(cpu.x, 0);
    assert_eq!(cpu.y, 0);
    assert_eq!(cpu.status.as_byte(), 0b0000_0100);
  }

  #[test]
  fn should_load_a_program_into_memory() {
    let mut cpu = Cpu::new();
    let program = vec![0x42, 0x43, 0x44];

    cpu.load_program(program).unwrap();

    assert_eq!(cpu.memory.read_byte(0x0200), 0x42);
    assert_eq!(cpu.memory.read_byte(0x0201), 0x43);
    assert_eq!(cpu.memory.read_byte(0x0202), 0x44);
  }

  #[test]
  fn should_not_load_a_program_into_memory_if_it_is_too_large() {
    let mut cpu = Cpu::new();
    let program = vec![0x42; MEMORY_RAM_SIZE as usize + 1];

    let result = cpu.load_program(program);

    assert_eq!(result, Err("Program is too large to fit in memory"));
  }

  #[test]
  fn should_fetch_and_decode_an_instruction() {
    let mut cpu = Cpu::new();
    let program = vec![0xA9, 0x42, 0x00];
    cpu.load_program(program).unwrap();

    let instruction = cpu.fetch_and_decode();

    assert_eq!(instruction.opcode, Opcode::LDA);
    assert_eq!(instruction.address_mode, AddressMode::Immediate);
    assert_eq!(instruction.operands, (Some(0x42), None));
  }

  #[test]
  fn should_execute_an_instruction() {
    let mut cpu = Cpu::new();
    let program = vec![0xA9, 0x42, 0x00];
    cpu.load_program(program).unwrap();
    cpu.tick();

    assert_eq!(cpu.acc, 0x42);
    assert_eq!(cpu.pc, IRQ_ADDRESS); 
  }
}
