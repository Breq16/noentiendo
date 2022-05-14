use crate::registers::ProgramCounter;
use crate::system::{MemoryIO, System};

pub trait Fetch {
  // Fetch immediate values
  fn fetch(&mut self) -> Result<u8, ()>;
  fn fetch_word(&mut self) -> Result<u16, ()>;

  // Fetch an 8-bit pointer and return the value at that address
  fn fetch_zero_page(&mut self) -> Result<u8, ()>;
  fn fetch_zero_page_x(&mut self) -> Result<u8, ()>;
  fn fetch_zero_page_y(&mut self) -> Result<u8, ()>;

  // Fetch a 16-bit pointer and return the value at that address
  fn fetch_absolute(&mut self) -> Result<u8, ()>;
  fn fetch_absolute_x(&mut self) -> Result<u8, ()>;
  fn fetch_absolute_y(&mut self) -> Result<u8, ()>;

  // Fetch a 16-bit pointer by adding the X register to the instruction argument
  // and return the value at that address
  fn fetch_indirect_x(&mut self) -> Result<u8, ()>;

  // Fetch a 16-bit pointer, add the Y register to it, and return the value at
  // that address
  fn fetch_indirect_y(&mut self) -> Result<u8, ()>;
}

impl Fetch for System {
  fn fetch(&mut self) -> Result<u8, ()> {
    let result = self.read(self.registers.pc_address());
    self.registers.pc_increment();
    result
  }

  fn fetch_word(&mut self) -> Result<u16, ()> {
    let lo = self.fetch()?;
    let hi = self.fetch()?;
    Ok((hi as u16) << 8 | lo as u16)
  }

  fn fetch_zero_page(&mut self) -> Result<u8, ()> {
    let address = self.fetch()? as u16;
    let result = self.read(address);
    result
  }

  fn fetch_zero_page_x(&mut self) -> Result<u8, ()> {
    let address = self.fetch()?;
    let result = self.read((address + self.registers.x_index) as u16);
    result
  }

  fn fetch_zero_page_y(&mut self) -> Result<u8, ()> {
    let address = self.fetch()?;
    let result = self.read((address + self.registers.y_index) as u16);
    result
  }

  fn fetch_absolute(&mut self) -> Result<u8, ()> {
    let address = self.fetch_word()?;
    let result = self.read(address);
    result
  }

  fn fetch_absolute_x(&mut self) -> Result<u8, ()> {
    let address = self.fetch_word()?;
    let result = self.read(address + self.registers.x_index as u16);
    result
  }

  fn fetch_absolute_y(&mut self) -> Result<u8, ()> {
    let address = self.fetch_word()?;
    let result = self.read(address + self.registers.y_index as u16);
    result
  }

  fn fetch_indirect_x(&mut self) -> Result<u8, ()> {
    let base = self.fetch()?;
    let address = (base + self.registers.x_index) as u16;
    let pointer = self.read_word(address)?;
    let result = self.read(pointer);
    result
  }

  fn fetch_indirect_y(&mut self) -> Result<u8, ()> {
    let base = self.fetch()?;
    let address = self.read_word(base as u16)?;
    let result = self.read(address + self.registers.y_index as u16);
    result
  }
}