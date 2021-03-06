use crate::memory::Memory;

pub struct NullMemory {}

impl NullMemory {
  pub fn new() -> Self {
    Self {}
  }
}

impl Memory for NullMemory {
  fn read(&self, _address: u16) -> u8 {
    0
  }

  fn write(&mut self, _address: u16, _value: u8) {}

  fn tick(&mut self) {}

  fn reset(&mut self) {}
}
