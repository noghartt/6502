use crate::{address_mode::AddressMode, cpu::Cpu};

#[derive(Debug, PartialEq)]
pub enum Opcode {
  LDA,       // LDA -> LoaD Accumulator
  LDX,       // LDX -> LoaD X register
  LDY,       // LDY -> LoaD Y register
  STA,       // STA -> STore Accumulator
  STX,       // STX -> STore X register
  STY,       // STY -> STore Y register
  BRK,       // BRK -> BReaK
  NOP,       // NOP -> No OPeration
}

#[derive(Debug)]
pub struct Instruction {
  pub opcode: Opcode,
  pub address_mode: AddressMode,
  pub operands: (Option<u8>, Option<u8>),
}

impl Instruction {
  pub fn new(opcode: Opcode, mode: AddressMode, operands: (Option<u8>, Option<u8>)) -> Self {
    Self {
      opcode,
      address_mode: mode,
      operands,
    }
  }

  pub fn decode(op: u8, a: u8, b: u8) -> Self {
    match op {
      0x00 => Self::new(Opcode::BRK, AddressMode::Implicit, (None, None)),
      0xA9 => Self::new(Opcode::LDA, AddressMode::Immediate, (Some(a), None)),
      _ => todo!()
    }
  }

  pub fn resolve_operand_value(&self, cpu: &Cpu) -> u8 {
    match self.address_mode {
      AddressMode::Immediate => self.operands.0.unwrap(),
      _ => todo!()
    }
  }
}
