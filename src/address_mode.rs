/// See more about the AddressMode here:
/// http://www.6502.org/users/obelisk/index.html
#[derive(Debug, PartialEq)]
pub enum AddressMode {
  Implicit,
  Accumulator,
  Immediate,
  ZeroPage,
  ZeroPageX,
  ZeroPageY,
  Relative,
}
