use std::fmt;

use crate::cpu::{Address, Data};

pub struct Registers {
    pub pc: Address,
    pub sp: Data,
    pub a: Data,
    pub x: Data,
    pub y: Data,

    pc_next: Address,
    ps: Data,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum StatusBits {
    Carry = 1 << 0,
    Zero = 1 << 1,
    Int = 1 << 2,
    Dec = 1 << 3,
    Brk = 1 << 4,
    Ovf = 1 << 5,
    Neg = 1 << 6,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            pc: 0x0000,
            sp: 0xff,
            a: 0x00,
            x: 0x00,
            y: 0x00,
            pc_next: 0x000,
            ps: 0x00,
        }
    }

    pub fn get_flag(&self, bit: StatusBits) -> bool {
        (self.ps & bit as u8) != 0
    }

    pub fn set_flag(&mut self, bit: StatusBits) {
        self.ps |= bit as u8;
    }

    pub fn clear_flag(&mut self, bit: StatusBits) {
        self.ps &= 0xff ^ bit as u8;
    }

    pub fn carry(&self) -> bool {
        self.get_flag(StatusBits::Carry)
    }

    pub fn zero(&self) -> bool {
        self.get_flag(StatusBits::Zero)
    }

    pub fn int(&self) -> bool {
        self.get_flag(StatusBits::Int)
    }

    pub fn dec(&self) -> bool {
        self.get_flag(StatusBits::Dec)
    }

    pub fn brk(&self) -> bool {
        self.get_flag(StatusBits::Brk)
    }

    pub fn overflow(&self) -> bool {
        self.get_flag(StatusBits::Ovf)
    }

    pub fn negative(&self) -> bool {
        self.get_flag(StatusBits::Neg)
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PC: {:04x} ", self.pc))?;
        f.write_fmt(format_args!("SP: {:02x} ", self.sp))?;
        f.write_fmt(format_args!("A: {:02x} ", self.a))?;
        f.write_fmt(format_args!("X: {:02x} ", self.x))?;
        f.write_fmt(format_args!("Y: {:02x} ", self.y))?;
        f.write_fmt(format_args!("PCnext: {:04x} ", self.pc_next))?;

        if self.carry() {
            f.write_str("C")?;
        }
        if self.zero() {
            f.write_str("Z")?;
        }
        if self.int() {
            f.write_str("I")?;
        }
        if self.dec() {
            f.write_str("D")?;
        }
        if self.brk() {
            f.write_str("B")?;
        }
        if self.overflow() {
            f.write_str("O")?;
        }
        if self.negative() {
            f.write_str("V")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let _ = Registers::new();
    }

    #[test]
    fn get_set() {
        let mut r = Registers::new();
        let map: [(fn(&Registers) -> bool, StatusBits); 7] = [
            (Registers::carry, StatusBits::Carry),
            (Registers::zero, StatusBits::Zero),
            (Registers::int, StatusBits::Int),
            (Registers::dec, StatusBits::Dec),
            (Registers::brk, StatusBits::Brk),
            (Registers::overflow, StatusBits::Ovf),
            (Registers::negative, StatusBits::Neg),
        ];

        for (get_test_flag, flag) in map {
            r.set_flag(flag);
            assert!(get_test_flag(&r));
            assert_eq!(get_test_flag(&r), r.get_flag(flag));

            r.clear_flag(flag);
            assert!(!get_test_flag(&r));
            assert_eq!(get_test_flag(&r), r.get_flag(flag));
        }
    }
}
