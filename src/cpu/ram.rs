use crate::cpu::{Error, ErrorType, Memory};

use crate::cpu::Result;
use crate::cpu::{Address, Byte, Word};

pub struct Ram {
    memory: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Ram {
            memory: vec![0; size],
        }
    }
}

impl Memory for Ram {
    fn length(&self) -> usize {
        self.memory.len()
    }

    fn read_byte(&self, address: Address) -> Result<Byte> {
        let uaddr = address as usize;
        if uaddr >= self.memory.len() {
            return Err(Error::without_pc(ErrorType::AddressOutOfRange(address)));
        }
        Ok(self.memory[uaddr])
    }

    fn read_word(&self, address: Address) -> Result<Word> {
        let uaddr = address as usize;

        if uaddr >= self.memory.len() {
            return Err(Error::without_pc(ErrorType::AddressOutOfRange(address)));
        }

        let mut word: u16 = self.memory[uaddr] as u16;
        word += (self.memory[uaddr + 1] as u16) << 8;

        Ok(word)
    }

    fn write_byte(&mut self, address: Address, data: Byte) -> Result<()> {
        let uaddr = address as usize;
        if uaddr >= self.memory.len() {
            return Err(Error::without_pc(ErrorType::AddressOutOfRange(address)));
        }
        self.memory[uaddr] = data;

        Ok(())
    }

    fn write_word(&mut self, address: Address, data: Word) -> Result<()> {
        let uaddr = address as usize;

        if uaddr >= self.memory.len() {
            return Err(Error::without_pc(ErrorType::AddressOutOfRange(address)));
        }

        self.memory[uaddr] = (data & 0xff) as u8;
        self.memory[uaddr + 1] = (data >> 8) as u8;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let _ = Ram::new(10);
    }

    #[test]
    fn read_bytes_in_range_succeeds() -> Result<()> {
        let memory = Ram::new(10);

        memory.read_byte(0)?;
        memory.read_byte(1)?;

        Ok(())
    }

    #[test]
    fn read_byte_out_of_range_fails() {
        let memory = Ram::new(10);

        let address: Address = 11;
        let result = memory.read_byte(address);

        assert_eq!(
            result,
            Err(Error::without_pc(ErrorType::AddressOutOfRange(address)))
        )
    }

    #[test]
    fn write_bytes_in_range_succeeds() -> Result<()> {
        let mut memory = Ram::new(10);

        memory.write_byte(0, 0xde)?;
        memory.write_byte(1, 0xad)?;

        Ok(())
    }

    #[test]
    fn write_byte_out_of_range_fails() {
        let mut memory = Ram::new(10);

        let address: Address = 11;
        let result = memory.write_byte(address, 0);
        assert_eq!(
            result,
            Err(Error::without_pc(ErrorType::AddressOutOfRange(address)))
        )
    }

    #[test]
    fn read_words_in_range_succeeds() -> Result<()> {
        let memory = Ram::new(10);

        memory.read_word(0)?;
        memory.read_word(2)?;

        Ok(())
    }

    #[test]
    fn read_word_out_of_range_fails() {
        let memory = Ram::new(10);

        let address: Address = 12;
        let result = memory.read_word(address);

        assert_eq!(
            result,
            Err(Error::without_pc(ErrorType::AddressOutOfRange(address)))
        )
    }

    #[test]
    fn write_words_in_range_succeeds() -> Result<()> {
        let mut memory = Ram::new(10);

        memory.write_word(0, 0xdead)?;
        memory.write_word(2, 0xbeef)?;

        Ok(())
    }

    #[test]
    fn write_word_out_of_range_fails() {
        let mut memory = Ram::new(10);

        let address: Address = 12;
        let result = memory.write_word(address, 0);
        assert_eq!(
            result,
            Err(Error::without_pc(ErrorType::AddressOutOfRange(address)))
        )
    }

    #[test]
    fn write_read_byte_succeeds() -> Result<()> {
        let mut memory = Ram::new(10);
        let address: Address = 0;
        let value = 0xde;
        memory.write_byte(address, value)?;

        let result = memory.read_byte(address);
        match result {
            Ok(r) => assert_eq!(r, value),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    #[test]
    fn write_read_word_succeeds() -> Result<()> {
        let mut memory = Ram::new(10);
        let address: Address = 0;
        let value = 0xdead;
        memory.write_word(address, value)?;

        let result = memory.read_word(address);
        match result {
            Ok(r) => assert_eq!(r, value),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    #[test]
    fn endianness_is_correct() -> Result<()> {
        let mut memory = Ram::new(16);

        memory.write_byte(0, 0xad)?;
        memory.write_byte(1, 0xde)?;
        match memory.read_word(0) {
            Ok(r) => assert_eq!(r, 0xdead),
            Err(e) => return Err(e),
        }

        memory.write_word(0, 0xbeef)?;
        match memory.read_byte(0) {
            Ok(r) => assert_eq!(r, 0xef),
            Err(e) => return Err(e),
        }

        match memory.read_byte(1) {
            Ok(r) => assert_eq!(r, 0xbe),
            Err(e) => return Err(e),
        }

        Ok(())
    }
}
