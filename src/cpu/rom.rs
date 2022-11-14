use super::{Address, Byte, Error, Memory, Result, Word};

pub struct Rom {
    memory: Vec<u8>,
    base: Address,
}

impl Rom {
    pub fn new(image: Vec<u8>, base: Address) -> Self {
        Rom {
            memory: image,
            base,
        }
    }
}

impl Memory for Rom {
    fn read_byte(&self, address: Address) -> Result<Byte> {
        if address < self.base || address > self.base + self.memory.len() as u16 {
            return Err(Error::AddressOutOfRange(address));
        }

        Ok(self.memory[(address - self.base) as usize])
    }

    fn read_word(&self, address: Address) -> Result<Word> {
        if address & 1 != 0 {
            return Err(Error::InvalidAddress(address));
        }
        if address < self.base || address > self.base + self.memory.len() as u16 {
            return Err(Error::AddressOutOfRange(address));
        }

        let base = (address - self.base) as usize;
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
        let _ = Rom::new(vec![0; 32], 0);
    }

    #[test]
    fn construct_from_image() {
        let _ = Rom::new(test_rom1(), 0);
    }
}
