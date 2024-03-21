#[derive(Default, Debug)]
struct CpuStatusRegister {
  /// Carry flag
  c: bool,
  /// Zero flag
  z: bool,
  /// Interrupt disable
  i: bool,
  /// Decimal mode
  d: bool,
  /// Break command
  b: bool,
  /// Overflow flag
  v: bool,
  /// Negative flag
  n: bool,
}

#[derive(Debug)]
pub struct Cpu {
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
      pc: 0,
      sp: 0,
      acc: 0,
      x: 0,
      y: 0,
      status: CpuStatusRegister::default(),
    }
  }
}
