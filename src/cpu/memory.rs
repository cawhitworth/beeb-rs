use crate::cpu::{Address, Byte, Memory, Result, Word};

pub struct OverlayMemory<M, O>
where
    M: Memory,
    O: Memory,
{
    pub base_memory: M,

    pub overlay_memory: O,
    overlay_offset: Address,
}

impl<M, O> OverlayMemory<M, O>
where
    M: Memory,
    O: Memory,
{
    pub fn new(base_memory: M, overlay_memory: O, overlay_offset: Address) -> Self {
        let base_end = base_memory.length();
        let overlay_end = overlay_offset as usize + overlay_memory.length();

        if overlay_offset as usize > base_end || overlay_end > base_end {
            panic!(
                "Overlay ({:4x}-{:4x}) cannot be outside of base ({:4x}-{:4x})",
                overlay_offset, overlay_end, 0x00, base_end
            );
        }
        OverlayMemory {
            base_memory,
            overlay_memory,
            overlay_offset,
        }
    }

    fn in_overlay(&self, address: Address) -> bool {
        address >= self.overlay_offset
            && address < self.overlay_offset + self.overlay_memory.length() as u16
    }

    fn get_in_overlay(&self, address: Address) -> Option<Address> {
        if !self.in_overlay(address) {
            return None;
        }

        Some(address - self.overlay_offset)
    }
}

impl<M, O> Memory for OverlayMemory<M, O>
where
    M: Memory,
    O: Memory,
{
    fn length(&self) -> usize {
        self.base_memory.length()
    }

    fn read_byte(&self, address: Address) -> Result<Byte> {
        if let Some(overlay_addr) = self.get_in_overlay(address) {
            self.overlay_memory.read_byte(overlay_addr)
        } else {
            self.base_memory.read_byte(address)
        }
    }

    fn read_word(&self, address: Address) -> Result<Word> {
        if let Some(overlay_addr) = self.get_in_overlay(address) {
            self.overlay_memory.read_word(overlay_addr)
        } else {
            self.base_memory.read_word(address)
        }
    }

    fn write_byte(&mut self, address: Address, data: Byte) -> Result<()> {
        if let Some(overlay_addr) = self.get_in_overlay(address) {
            self.overlay_memory.write_byte(overlay_addr, data)
        } else {
            self.base_memory.write_byte(address, data)
        }
    }

    fn write_word(&mut self, address: Address, data: Word) -> Result<()> {
        if let Some(overlay_addr) = self.get_in_overlay(address) {
            self.overlay_memory.write_word(overlay_addr, data)
        } else {
            self.base_memory.write_word(address, data)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cpu::ram::Ram;
    use crate::cpu::rom::Rom;

    #[test]
    fn construct() {
        let ram = Ram::new(0x10000);
        let rom = Rom::new(vec![0; 0x100]);

        let overlay_mem = OverlayMemory::new(ram, rom, 0xff00);
    }

    #[test]
    #[should_panic]
    fn construct_out_of_bounds_panics() {
        let ram = Ram::new(0x10000);
        let rom = Rom::new(vec![0; 0x101]);

        let overlay_mem = OverlayMemory::new(ram, rom, 0xff00);
    }

    #[test]
    fn passthrough_of_base_succeeds() -> Result<()> {
        let base_rom = Rom::new(vec![0xde; 0x100]);
        let overlay_rom = Rom::new(vec![0xed; 0x10]);
        let overlay_mem = OverlayMemory::new(base_rom, overlay_rom, 0x00);

        let b = overlay_mem.read_byte(0x10)?;
        assert_eq!(b, 0xde);

        Ok(())
    }

    #[test]
    fn overlay_succeeds() -> Result<()> {
        let base_rom = Rom::new(vec![0xde; 0x100]);
        let overlay_rom = Rom::new(vec![0xed; 0x10]);
        let overlay_mem = OverlayMemory::new(base_rom, overlay_rom, 0x00);

        let b = overlay_mem.read_byte(0x00)?;
        assert_eq!(b, 0xed);

        Ok(())
    }

    #[test]
    fn write_to_base_succeeds() -> Result<()> {
        let base_ram = Ram::new(0x100);
        let overlay_ram = Ram::new(0x10);
        let mut overlay_mem = OverlayMemory::new(base_ram, overlay_ram, 0x00);

        overlay_mem.write_byte(0x10, 0xde)?;
        let base_read = overlay_mem.base_memory.read_byte(0x10)?;
        assert_eq!(base_read, 0xde);

        let overlay_read = overlay_mem.read_byte(0x10)?;
        assert_eq!(overlay_read, 0xde);

        Ok(())
    }

    #[test]
    fn write_to_overlay_succeeds_does_not_affect_base() -> Result<()> {
        let mut base_ram = Ram::new(0x100);
        base_ram.write_byte(0x00, 0xba)?;

        let overlay_ram = Ram::new(0x10);
        let mut overlay_mem = OverlayMemory::new(base_ram, overlay_ram, 0x00);

        overlay_mem.write_byte(0x00, 0xde)?;

        let overlay_read = overlay_mem.read_byte(0x00)?;
        assert_eq!(overlay_read, 0xde);

        let base_read = overlay_mem.base_memory.read_byte(0x00)?;
        assert_eq!(base_read, 0xba);
        Ok(())
    }
}
