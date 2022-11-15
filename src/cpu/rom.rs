use crate::cpu::{Address, Byte, Error, Memory, Result, Word};

pub struct Rom {
    memory: Vec<u8>,
}

impl Rom {
    pub fn new(image: Vec<u8>) -> Self {
        Rom { memory: image }
    }
}

impl Memory for Rom {
    fn read_byte(&self, address: Address) -> Result<Byte> {
        if address > self.memory.len() as u16 {
            return Err(Error::AddressOutOfRange(address));
        }

        Ok(self.memory[address as usize])
    }

    fn read_word(&self, address: Address) -> Result<Word> {
        if address & 1 != 0 {
            return Err(Error::InvalidAddress(address));
        }
        if address > self.memory.len() as u16 {
            return Err(Error::AddressOutOfRange(address));
        }

        let base = address as usize;
        let mut word = self.memory[base] as u16;
        word += (self.memory[base + 1] as u16) << 8;
        Ok(word)
    }

    fn write_byte(&mut self, _: Address, _: Byte) -> Result<()> {
        Ok(())
    }

    fn write_word(&mut self, _: Address, _: Word) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::roms::test_rom1;

    use super::*;

    #[test]
    fn construct() {
        let _ = Rom::new(vec![0; 32]);
    }

    #[test]
    fn construct_from_image() {
        let _ = Rom::new(test_rom1());
    }

    #[test]
    fn read_bytes_in_range_succeeds() -> Result<()> {
        let memory = Rom::new(vec![0; 32]);

        memory.read_byte(0)?;
        memory.read_byte(1)?;

        Ok(())
    }

    #[test]
    fn read_byte_out_of_range_fails() {
        let memory = Rom::new(vec![0; 32]);

        let address: Address = 33;
        let result = memory.read_byte(address);

        assert_eq!(result, Err(Error::AddressOutOfRange(address)))
    }

    #[test]
    fn read_words_in_range_succeeds() -> Result<()> {
        let memory = Rom::new(vec![0; 32]);

        memory.read_word(0)?;
        memory.read_word(2)?;

        Ok(())
    }

    #[test]
    fn read_word_wrong_offset_fails() {
        let memory = Rom::new(vec![0; 32]);

        let address: Address = 1;
        let result = memory.read_word(address);

        assert_eq!(result, Err(Error::InvalidAddress(address)))
    }

    #[test]
    fn read_word_out_of_range_fails() {
        let memory = Rom::new(vec![0; 32]);

        let address: Address = 34;
        let result = memory.read_word(address);

        assert_eq!(result, Err(Error::AddressOutOfRange(address)))
    }

    #[test]
    fn write_bytes_in_range_succeeds() -> Result<()> {
        let mut memory = Rom::new(vec![0; 32]);

        memory.write_byte(0, 0xde)?;
        memory.write_byte(1, 0xad)?;

        Ok(())
    }

    #[test]
    fn write_byte_out_of_range_succeeds() -> Result<()> {
        let mut memory = Rom::new(vec![0; 32]);

        memory.write_byte(33, 0)?;
        Ok(())
    }

    #[test]
    fn write_words_in_range_succeeds() -> Result<()> {
        let mut memory = Rom::new(vec![0; 32]);

        memory.write_word(0, 0xdead)?;
        memory.write_word(2, 0xbeef)?;

        Ok(())
    }

    #[test]
    fn write_word_out_of_range_fails() -> Result<()> {
        let mut memory = Rom::new(vec![0; 32]);

        memory.write_word(34, 0)?;
        Ok(())
    }

    #[test]
    fn write_read_byte_succeeds_memory_not_altered() -> Result<()> {
        let address: Address = 0;
        let init = 0x00;
        let value = 0xde;

        let mut memory = Rom::new(vec![init; 32]);
        memory.write_byte(address, value)?;

        let result = memory.read_byte(address);
        match result {
            Ok(r) => assert_eq!(r, init),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    #[test]
    fn write_read_word_succeeds_memory_not_altered() -> Result<()> {
        let address: Address = 0;
        let init = 0x00;
        let value = 0xde;

        let mut memory = Rom::new(vec![init; 32]);

        memory.write_word(address, value)?;

        let result = memory.read_word(address);
        match result {
            Ok(r) => assert_eq!(r, init as u16),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    #[test]
    fn check_endiannness_correct() -> Result<()> {
        let memory = Rom::new(vec![0xad, 0xde]);

        match memory.read_word(0) {
            Ok(r) => assert_eq!(r, 0xdead),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}
