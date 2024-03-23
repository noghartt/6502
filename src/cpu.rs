use core::fmt;
use std::fmt::{Debug, Formatter};
use std::process;

use crate::instructions::{Instruction, Opcode};
use crate::memory::{
  Memory,
  MEMORY_ADDRESS_RAM_START,
  MEMORY_RAM_SIZE,
};


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
  status: u8,
}

const CPU_FLAG_MASK: u8 = 0b00110000;

impl Cpu {
  pub fn new() -> Self {
    Self {
      memory: Memory::new(),
      pc: MEMORY_ADDRESS_RAM_START,
      sp: 0x00FF,
      acc: 0,
      x: 0,
      y: 0,
      status: CPU_FLAG_MASK,
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
      let should_break =  self.execute(instruction);
      if should_break {
        break;
      }
    }
  }

  fn execute(&mut self, instruction: Instruction) -> bool {
    return match instruction.opcode {
      Opcode::BRK => self.execute_opcode_brk(),
      Opcode::LDA => self.execute_opcode_lda(instruction),
      _ => todo!("instruction not implemented yet"),
    }
  }

  fn fetch_and_decode(&mut self) -> Instruction {
    let opcode = self.memory.read_byte(self.pc);
    let a = self.memory.read_byte(self.pc + 1);
    let b = self.memory.read_byte(self.pc + 2);
    Instruction::decode(opcode, a, b)
  }

  fn update_flags_neg_zero(&mut self, value: u8) {
    if value == 0 {
      self.status |= 0b00000010;
    } else {
      self.status &= 0b11111101;
    }

    if value & 0b10000000 != 0 {
      self.status |= 0b10000000;
    } else {
      self.status &= 0b01111111;
    }
  }

  fn execute_opcode_brk(&mut self) -> bool {
    true
  }

  fn execute_opcode_lda(&mut self, instruction: Instruction) -> bool {
    let value = instruction.resolve_operand_value(self);
    self.acc = value;
    self.update_flags_neg_zero(value);
    self.pc += 2;
    false
  }
}

#[cfg(test)]
mod tests {
  use crate::address_mode::AddressMode;

use super::*;

  #[test]
  fn should_create_a_new_cpu() {
    let cpu = Cpu::new();

    assert_eq!(cpu.pc, 0xFFFC);
    assert_eq!(cpu.sp, 0x00FF);
    assert_eq!(cpu.acc, 0);
    assert_eq!(cpu.x, 0);
    assert_eq!(cpu.y, 0);
    assert_eq!(cpu.status, 0b00110000);
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
    assert_eq!(cpu.pc, 0x0202);    
  }
}
