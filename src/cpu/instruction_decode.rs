use crate::cpu::{AddressingMode, Byte, Instruction, Opcode, Result, Writeback};

pub struct InstructionDecoder {
    decode_table: [Instruction; 256],
}

// See https://www.masswerk.at/6502/6502_instruction_set.html
impl InstructionDecoder {
    pub fn new() -> Self {
        InstructionDecoder {
            decode_table: [
                /* 00 */
                Instruction::new(Opcode::BRK, AddressingMode::Implicit, Writeback::PC, 1, 7),
                /* 01 */
                Instruction::new(
                    Opcode::ORA,
                    AddressingMode::IndirectX,
                    Writeback::Accumulator,
                    2,
                    6,
                ),
                /* 02 */
                Instruction::new(
                    Opcode::Invalid(0x02),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 03 */
                Instruction::new(
                    Opcode::Invalid(0x03),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 04 */
                Instruction::new(
                    Opcode::Invalid(0x04),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 05 */
                Instruction::new(
                    Opcode::ORA,
                    AddressingMode::ZeroPage,
                    Writeback::Accumulator,
                    2,
                    3,
                ),
                /* 06 */
                Instruction::new(
                    Opcode::ASL,
                    AddressingMode::ZeroPage,
                    Writeback::Memory,
                    2,
                    5,
                ),
                /* 07 */
                Instruction::new(
                    Opcode::Invalid(0x07),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 08 */
                Instruction::new(Opcode::PHP, AddressingMode::Implicit, Writeback::SP, 1, 3),
                /* 09 */
                Instruction::new(
                    Opcode::ORA,
                    AddressingMode::Immediate,
                    Writeback::Accumulator,
                    2,
                    2,
                ),
                /* 0A */
                Instruction::new(
                    Opcode::ASL,
                    AddressingMode::Accumulator,
                    Writeback::Accumulator,
                    1,
                    2,
                ),
                /* 0B */
                Instruction::new(
                    Opcode::Invalid(0x0B),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 0C */
                Instruction::new(
                    Opcode::Invalid(0x0C),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 0D */
                Instruction::new(
                    Opcode::ORA,
                    AddressingMode::Absolute,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 0E */
                Instruction::new(
                    Opcode::ASL,
                    AddressingMode::Absolute,
                    Writeback::Memory,
                    3,
                    6,
                ),
                /* 0F */
                Instruction::new(
                    Opcode::Invalid(0x0F),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 10 */
                Instruction::new(Opcode::BPL, AddressingMode::Relative, Writeback::PC, 2, 2),
                /* 11 */
                Instruction::new(
                    Opcode::ORA,
                    AddressingMode::IndirectY,
                    Writeback::Accumulator,
                    2,
                    5,
                ),
                /* 12 */
                Instruction::new(
                    Opcode::Invalid(0x12),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 13 */
                Instruction::new(
                    Opcode::Invalid(0x13),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 14 */
                Instruction::new(
                    Opcode::Invalid(0x14),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 15 */
                Instruction::new(
                    Opcode::ORA,
                    AddressingMode::ZeroPageX,
                    Writeback::Accumulator,
                    2,
                    4,
                ),
                /* 16 */
                Instruction::new(
                    Opcode::ASL,
                    AddressingMode::ZeroPageX,
                    Writeback::Memory,
                    2,
                    6,
                ),
                /* 17 */
                Instruction::new(
                    Opcode::Invalid(0x17),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 18 */
                Instruction::new(
                    Opcode::CLC,
                    AddressingMode::Implicit,
                    Writeback::NoWriteback,
                    1,
                    2,
                ),
                /* 19 */
                Instruction::new(
                    Opcode::ORA,
                    AddressingMode::AbsoluteY,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 1A */
                Instruction::new(
                    Opcode::Invalid(0x1A),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 1B */
                Instruction::new(
                    Opcode::Invalid(0x1B),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 1C */
                Instruction::new(
                    Opcode::Invalid(0x1C),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 1D */
                Instruction::new(
                    Opcode::ORA,
                    AddressingMode::AbsoluteX,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 1E */
                Instruction::new(
                    Opcode::ASL,
                    AddressingMode::AbsoluteX,
                    Writeback::Memory,
                    3,
                    7,
                ),
                /* 1F */
                Instruction::new(
                    Opcode::Invalid(0x1F),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 20 */
                Instruction::new(Opcode::JSR, AddressingMode::Absolute, Writeback::PC, 3, 6),
                /* 21 */
                Instruction::new(
                    Opcode::AND,
                    AddressingMode::IndirectX,
                    Writeback::Accumulator,
                    2,
                    6,
                ),
                /* 22 */
                Instruction::new(
                    Opcode::Invalid(0x22),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 23 */
                Instruction::new(
                    Opcode::Invalid(0x23),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 24 */
                Instruction::new(
                    Opcode::BIT,
                    AddressingMode::ZeroPage,
                    Writeback::NoWriteback,
                    2,
                    3,
                ),
                /* 25 */
                Instruction::new(
                    Opcode::AND,
                    AddressingMode::ZeroPage,
                    Writeback::Accumulator,
                    2,
                    3,
                ),
                /* 26 */
                Instruction::new(
                    Opcode::ROL,
                    AddressingMode::ZeroPage,
                    Writeback::Memory,
                    2,
                    5,
                ),
                /* 27 */
                Instruction::new(
                    Opcode::Invalid(0x27),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 28 */
                Instruction::new(Opcode::PLP, AddressingMode::Implicit, Writeback::PS, 1, 4),
                /* 29 */
                Instruction::new(
                    Opcode::AND,
                    AddressingMode::Immediate,
                    Writeback::Accumulator,
                    2,
                    2,
                ),
                /* 2A */
                Instruction::new(
                    Opcode::ROL,
                    AddressingMode::Accumulator,
                    Writeback::Accumulator,
                    1,
                    2,
                ),
                /* 2B */
                Instruction::new(
                    Opcode::Invalid(0x2B),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 2C */
                Instruction::new(
                    Opcode::BIT,
                    AddressingMode::Absolute,
                    Writeback::NoWriteback,
                    3,
                    4,
                ),
                /* 2D */
                Instruction::new(
                    Opcode::AND,
                    AddressingMode::Absolute,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 2E */
                Instruction::new(
                    Opcode::ROL,
                    AddressingMode::Absolute,
                    Writeback::Memory,
                    3,
                    6,
                ),
                /* 2F */
                Instruction::new(
                    Opcode::Invalid(0x2F),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 30 */
                Instruction::new(Opcode::BMI, AddressingMode::Relative, Writeback::PC, 2, 2),
                /* 31 */
                Instruction::new(
                    Opcode::AND,
                    AddressingMode::IndirectY,
                    Writeback::Accumulator,
                    2,
                    5,
                ),
                /* 32 */
                Instruction::new(
                    Opcode::Invalid(0x32),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 33 */
                Instruction::new(
                    Opcode::Invalid(0x33),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 34 */
                Instruction::new(
                    Opcode::Invalid(0x34),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 35 */
                Instruction::new(
                    Opcode::AND,
                    AddressingMode::ZeroPageX,
                    Writeback::Accumulator,
                    2,
                    4,
                ),
                /* 36 */
                Instruction::new(
                    Opcode::ROL,
                    AddressingMode::ZeroPageX,
                    Writeback::Memory,
                    2,
                    6,
                ),
                /* 37 */
                Instruction::new(
                    Opcode::Invalid(0x37),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 38 */
                Instruction::new(
                    Opcode::SEC,
                    AddressingMode::Implicit,
                    Writeback::NoWriteback,
                    1,
                    2,
                ),
                /* 39 */
                Instruction::new(
                    Opcode::AND,
                    AddressingMode::AbsoluteY,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 3A */
                Instruction::new(
                    Opcode::Invalid(0x3A),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 3B */
                Instruction::new(
                    Opcode::Invalid(0x3B),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 3C */
                Instruction::new(
                    Opcode::Invalid(0x3C),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 3D */
                Instruction::new(
                    Opcode::AND,
                    AddressingMode::AbsoluteX,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 3E */
                Instruction::new(
                    Opcode::ROL,
                    AddressingMode::AbsoluteX,
                    Writeback::Memory,
                    3,
                    7,
                ),
                /* 3F */
                Instruction::new(
                    Opcode::Invalid(0x3F),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 40 */
                Instruction::new(Opcode::RTI, AddressingMode::Implicit, Writeback::PC, 1, 6),
                /* 41 */
                Instruction::new(
                    Opcode::EOR,
                    AddressingMode::IndirectX,
                    Writeback::Accumulator,
                    2,
                    6,
                ),
                /* 42 */
                Instruction::new(
                    Opcode::Invalid(0x42),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 43 */
                Instruction::new(
                    Opcode::Invalid(0x43),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 44 */
                Instruction::new(
                    Opcode::Invalid(0x44),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 45 */
                Instruction::new(
                    Opcode::EOR,
                    AddressingMode::ZeroPage,
                    Writeback::Accumulator,
                    2,
                    3,
                ),
                /* 46 */
                Instruction::new(
                    Opcode::LSR,
                    AddressingMode::ZeroPage,
                    Writeback::Memory,
                    2,
                    5,
                ),
                /* 47 */
                Instruction::new(
                    Opcode::Invalid(0x47),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 48 */
                Instruction::new(Opcode::PHA, AddressingMode::Implicit, Writeback::SP, 1, 3),
                /* 49 */
                Instruction::new(
                    Opcode::EOR,
                    AddressingMode::Immediate,
                    Writeback::Accumulator,
                    2,
                    2,
                ),
                /* 4A */
                Instruction::new(
                    Opcode::LSR,
                    AddressingMode::Accumulator,
                    Writeback::Accumulator,
                    1,
                    2,
                ),
                /* 4B */
                Instruction::new(
                    Opcode::Invalid(0x4B),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 4C */
                Instruction::new(Opcode::JMP, AddressingMode::Absolute, Writeback::PC, 3, 3),
                /* 4D */
                Instruction::new(
                    Opcode::EOR,
                    AddressingMode::Absolute,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 4E */
                Instruction::new(
                    Opcode::LSR,
                    AddressingMode::Absolute,
                    Writeback::Memory,
                    3,
                    6,
                ),
                /* 4F */
                Instruction::new(
                    Opcode::Invalid(0x4F),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 50 */
                Instruction::new(Opcode::BVC, AddressingMode::Relative, Writeback::PC, 2, 2),
                /* 51 */
                Instruction::new(
                    Opcode::EOR,
                    AddressingMode::IndirectY,
                    Writeback::Accumulator,
                    2,
                    5,
                ),
                /* 52 */
                Instruction::new(
                    Opcode::Invalid(0x52),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 53 */
                Instruction::new(
                    Opcode::Invalid(0x53),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 54 */
                Instruction::new(
                    Opcode::Invalid(0x54),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 55 */
                Instruction::new(
                    Opcode::EOR,
                    AddressingMode::ZeroPageX,
                    Writeback::Accumulator,
                    2,
                    4,
                ),
                /* 56 */
                Instruction::new(
                    Opcode::LSR,
                    AddressingMode::ZeroPageX,
                    Writeback::Memory,
                    2,
                    6,
                ),
                /* 57 */
                Instruction::new(
                    Opcode::Invalid(0x57),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 58 */
                Instruction::new(
                    Opcode::CLI,
                    AddressingMode::Implicit,
                    Writeback::NoWriteback,
                    1,
                    2,
                ),
                /* 59 */
                Instruction::new(
                    Opcode::EOR,
                    AddressingMode::AbsoluteY,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 5A */
                Instruction::new(
                    Opcode::Invalid(0x5A),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 5B */
                Instruction::new(
                    Opcode::Invalid(0x5B),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 5C */
                Instruction::new(
                    Opcode::Invalid(0x5C),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 5D */
                Instruction::new(
                    Opcode::EOR,
                    AddressingMode::AbsoluteX,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 5E */
                Instruction::new(
                    Opcode::LSR,
                    AddressingMode::AbsoluteX,
                    Writeback::Memory,
                    3,
                    7,
                ),
                /* 5F */
                Instruction::new(
                    Opcode::Invalid(0x5F),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 60 */
                Instruction::new(Opcode::RTS, AddressingMode::Implicit, Writeback::PC, 1, 6),
                /* 61 */
                Instruction::new(
                    Opcode::ADC,
                    AddressingMode::IndirectX,
                    Writeback::Accumulator,
                    2,
                    6,
                ),
                /* 62 */
                Instruction::new(
                    Opcode::Invalid(0x62),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 63 */
                Instruction::new(
                    Opcode::Invalid(0x63),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 64 */
                Instruction::new(
                    Opcode::Invalid(0x64),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 65 */
                Instruction::new(
                    Opcode::ADC,
                    AddressingMode::ZeroPage,
                    Writeback::Accumulator,
                    2,
                    3,
                ),
                /* 66 */
                Instruction::new(
                    Opcode::ROR,
                    AddressingMode::ZeroPage,
                    Writeback::Memory,
                    2,
                    5,
                ),
                /* 67 */
                Instruction::new(
                    Opcode::Invalid(0x67),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 68 */
                Instruction::new(
                    Opcode::PLA,
                    AddressingMode::Implicit,
                    Writeback::Accumulator,
                    1,
                    4,
                ),
                /* 69 */
                Instruction::new(
                    Opcode::ADC,
                    AddressingMode::Immediate,
                    Writeback::Accumulator,
                    2,
                    2,
                ),
                /* 6A */
                Instruction::new(
                    Opcode::ROR,
                    AddressingMode::Accumulator,
                    Writeback::Accumulator,
                    1,
                    2,
                ),
                /* 6B */
                Instruction::new(
                    Opcode::Invalid(0x6B),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 6C */
                Instruction::new(Opcode::JMP, AddressingMode::Indirect, Writeback::PC, 3, 5),
                /* 6D */
                Instruction::new(
                    Opcode::ADC,
                    AddressingMode::Absolute,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 6E */
                Instruction::new(
                    Opcode::ROR,
                    AddressingMode::Absolute,
                    Writeback::Memory,
                    3,
                    6,
                ),
                /* 6F */
                Instruction::new(
                    Opcode::Invalid(0x6F),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 70 */
                Instruction::new(Opcode::BVS, AddressingMode::Relative, Writeback::PC, 2, 2),
                /* 71 */
                Instruction::new(
                    Opcode::ADC,
                    AddressingMode::IndirectY,
                    Writeback::Accumulator,
                    2,
                    5,
                ),
                /* 72 */
                Instruction::new(
                    Opcode::Invalid(0x72),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 73 */
                Instruction::new(
                    Opcode::Invalid(0x73),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 74 */
                Instruction::new(
                    Opcode::Invalid(0x74),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 75 */
                Instruction::new(
                    Opcode::ADC,
                    AddressingMode::ZeroPageX,
                    Writeback::Accumulator,
                    2,
                    4,
                ),
                /* 76 */
                Instruction::new(
                    Opcode::ROR,
                    AddressingMode::ZeroPageX,
                    Writeback::Memory,
                    2,
                    6,
                ),
                /* 77 */
                Instruction::new(
                    Opcode::Invalid(0x77),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 78 */
                Instruction::new(
                    Opcode::SEI,
                    AddressingMode::Implicit,
                    Writeback::NoWriteback,
                    1,
                    2,
                ),
                /* 79 */
                Instruction::new(
                    Opcode::ADC,
                    AddressingMode::AbsoluteY,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 7A */
                Instruction::new(
                    Opcode::Invalid(0x7A),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 7B */
                Instruction::new(
                    Opcode::Invalid(0x7B),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 7C */
                Instruction::new(
                    Opcode::Invalid(0x7C),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 7D */
                Instruction::new(
                    Opcode::ADC,
                    AddressingMode::AbsoluteX,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* 7E */
                Instruction::new(
                    Opcode::ROR,
                    AddressingMode::AbsoluteX,
                    Writeback::Memory,
                    3,
                    7,
                ),
                /* 7F */
                Instruction::new(
                    Opcode::Invalid(0x7F),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 80 */
                Instruction::new(
                    Opcode::Invalid(0x80),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 81 */
                Instruction::new(
                    Opcode::STA,
                    AddressingMode::IndirectX,
                    Writeback::Memory,
                    2,
                    6,
                ),
                /* 82 */
                Instruction::new(
                    Opcode::Invalid(0x82),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 83 */
                Instruction::new(
                    Opcode::Invalid(0x83),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 84 */
                Instruction::new(
                    Opcode::STY,
                    AddressingMode::ZeroPage,
                    Writeback::Memory,
                    2,
                    3,
                ),
                /* 85 */
                Instruction::new(
                    Opcode::STA,
                    AddressingMode::ZeroPage,
                    Writeback::Memory,
                    2,
                    3,
                ),
                /* 86 */
                Instruction::new(
                    Opcode::STX,
                    AddressingMode::ZeroPage,
                    Writeback::Memory,
                    2,
                    3,
                ),
                /* 87 */
                Instruction::new(
                    Opcode::Invalid(0x87),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 88 */
                Instruction::new(Opcode::DEY, AddressingMode::Implicit, Writeback::Y, 1, 2),
                /* 89 */
                Instruction::new(
                    Opcode::Invalid(0x89),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 8A */
                Instruction::new(
                    Opcode::TXA,
                    AddressingMode::Implicit,
                    Writeback::Accumulator,
                    1,
                    2,
                ),
                /* 8B */
                Instruction::new(
                    Opcode::Invalid(0x8B),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 8C */
                Instruction::new(
                    Opcode::STY,
                    AddressingMode::Absolute,
                    Writeback::Memory,
                    3,
                    4,
                ),
                /* 8D */
                Instruction::new(
                    Opcode::STA,
                    AddressingMode::Absolute,
                    Writeback::Memory,
                    3,
                    4,
                ),
                /* 8E */
                Instruction::new(
                    Opcode::STX,
                    AddressingMode::Absolute,
                    Writeback::Memory,
                    3,
                    4,
                ),
                /* 8F */
                Instruction::new(
                    Opcode::Invalid(0x8F),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 90 */
                Instruction::new(Opcode::BCC, AddressingMode::Relative, Writeback::PC, 2, 2),
                /* 91 */
                Instruction::new(
                    Opcode::STA,
                    AddressingMode::IndirectY,
                    Writeback::Memory,
                    2,
                    6,
                ),
                /* 92 */
                Instruction::new(
                    Opcode::Invalid(0x92),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 93 */
                Instruction::new(
                    Opcode::Invalid(0x93),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 94 */
                Instruction::new(
                    Opcode::STY,
                    AddressingMode::ZeroPageX,
                    Writeback::Memory,
                    2,
                    3,
                ),
                /* 95 */
                Instruction::new(
                    Opcode::STA,
                    AddressingMode::ZeroPageX,
                    Writeback::Memory,
                    2,
                    4,
                ),
                /* 96 */
                Instruction::new(
                    Opcode::STX,
                    AddressingMode::ZeroPageY,
                    Writeback::Memory,
                    2,
                    4,
                ),
                /* 97 */
                Instruction::new(
                    Opcode::Invalid(0x97),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 98 */
                Instruction::new(
                    Opcode::TYA,
                    AddressingMode::Implicit,
                    Writeback::Accumulator,
                    1,
                    2,
                ),
                /* 99 */
                Instruction::new(
                    Opcode::STA,
                    AddressingMode::AbsoluteY,
                    Writeback::Memory,
                    3,
                    5,
                ),
                /* 9A */
                Instruction::new(Opcode::TXS, AddressingMode::Implicit, Writeback::SP, 1, 2),
                /* 9B */
                Instruction::new(
                    Opcode::Invalid(0x9B),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 9C */
                Instruction::new(
                    Opcode::Invalid(0x9C),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 9D */
                Instruction::new(
                    Opcode::STA,
                    AddressingMode::AbsoluteX,
                    Writeback::Memory,
                    3,
                    5,
                ),
                /* 9E */
                Instruction::new(
                    Opcode::Invalid(0x9E),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* 9F */
                Instruction::new(
                    Opcode::Invalid(0x9F),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* A0 */
                Instruction::new(Opcode::LDY, AddressingMode::Immediate, Writeback::Y, 2, 2),
                /* A1 */
                Instruction::new(
                    Opcode::LDA,
                    AddressingMode::IndirectX,
                    Writeback::Accumulator,
                    2,
                    6,
                ),
                /* A2 */
                Instruction::new(Opcode::LDX, AddressingMode::Immediate, Writeback::X, 2, 2),
                /* A3 */
                Instruction::new(
                    Opcode::Invalid(0xA3),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* A4 */
                Instruction::new(Opcode::LDY, AddressingMode::ZeroPage, Writeback::Y, 2, 3),
                /* A5 */
                Instruction::new(
                    Opcode::LDA,
                    AddressingMode::ZeroPage,
                    Writeback::Accumulator,
                    2,
                    3,
                ),
                /* A6 */
                Instruction::new(Opcode::LDX, AddressingMode::ZeroPage, Writeback::X, 2, 3),
                /* A7 */
                Instruction::new(
                    Opcode::Invalid(0xA7),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* A8 */
                Instruction::new(Opcode::TAY, AddressingMode::Implicit, Writeback::Y, 1, 2),
                /* A9 */
                Instruction::new(
                    Opcode::LDA,
                    AddressingMode::Immediate,
                    Writeback::Accumulator,
                    2,
                    2,
                ),
                /* AA */
                Instruction::new(Opcode::TAX, AddressingMode::Implicit, Writeback::X, 1, 2),
                /* AB */
                Instruction::new(
                    Opcode::Invalid(0xAB),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* AC */
                Instruction::new(Opcode::LDY, AddressingMode::Absolute, Writeback::Y, 3, 4),
                /* AD */
                Instruction::new(
                    Opcode::LDA,
                    AddressingMode::Absolute,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* AE */
                Instruction::new(Opcode::LDX, AddressingMode::Absolute, Writeback::X, 3, 4),
                /* AF */
                Instruction::new(
                    Opcode::Invalid(0xAF),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* B0 */
                Instruction::new(Opcode::BCS, AddressingMode::Relative, Writeback::PC, 2, 2),
                /* B1 */
                Instruction::new(
                    Opcode::LDA,
                    AddressingMode::IndirectY,
                    Writeback::Accumulator,
                    2,
                    5,
                ),
                /* B2 */
                Instruction::new(
                    Opcode::Invalid(0xB2),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* B3 */
                Instruction::new(
                    Opcode::Invalid(0xB3),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* B4 */
                Instruction::new(Opcode::LDY, AddressingMode::ZeroPageX, Writeback::Y, 2, 4),
                /* B5 */
                Instruction::new(
                    Opcode::LDA,
                    AddressingMode::ZeroPageX,
                    Writeback::Accumulator,
                    2,
                    4,
                ),
                /* B6 */
                Instruction::new(Opcode::LDX, AddressingMode::ZeroPageY, Writeback::X, 2, 4),
                /* B7 */
                Instruction::new(
                    Opcode::Invalid(0xB7),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* B8 */
                Instruction::new(
                    Opcode::CLV,
                    AddressingMode::Implicit,
                    Writeback::NoWriteback,
                    1,
                    2,
                ),
                /* B9 */
                Instruction::new(
                    Opcode::LDA,
                    AddressingMode::AbsoluteY,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* BA */
                Instruction::new(Opcode::TSX, AddressingMode::Implicit, Writeback::X, 1, 2),
                /* BB */
                Instruction::new(
                    Opcode::Invalid(0xBB),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* BC */
                Instruction::new(Opcode::LDY, AddressingMode::AbsoluteX, Writeback::Y, 3, 4),
                /* BD */
                Instruction::new(
                    Opcode::LDA,
                    AddressingMode::AbsoluteX,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* BE */
                Instruction::new(Opcode::LDX, AddressingMode::AbsoluteY, Writeback::X, 3, 4),
                /* BF */
                Instruction::new(
                    Opcode::Invalid(0xBF),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* C0 */
                Instruction::new(
                    Opcode::CPY,
                    AddressingMode::Immediate,
                    Writeback::NoWriteback,
                    2,
                    2,
                ),
                /* C1 */
                Instruction::new(
                    Opcode::CMP,
                    AddressingMode::IndirectX,
                    Writeback::NoWriteback,
                    2,
                    6,
                ),
                /* C2 */
                Instruction::new(
                    Opcode::Invalid(0xC2),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* C3 */
                Instruction::new(
                    Opcode::Invalid(0xC3),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* C4 */
                Instruction::new(
                    Opcode::CPY,
                    AddressingMode::ZeroPage,
                    Writeback::NoWriteback,
                    2,
                    3,
                ),
                /* C5 */
                Instruction::new(
                    Opcode::CMP,
                    AddressingMode::ZeroPage,
                    Writeback::NoWriteback,
                    2,
                    3,
                ),
                /* C6 */
                Instruction::new(
                    Opcode::DEC,
                    AddressingMode::ZeroPage,
                    Writeback::Memory,
                    2,
                    5,
                ),
                /* C7 */
                Instruction::new(
                    Opcode::Invalid(0xC7),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* C8 */
                Instruction::new(Opcode::INY, AddressingMode::Implicit, Writeback::Y, 1, 2),
                /* C9 */
                Instruction::new(
                    Opcode::CMP,
                    AddressingMode::Immediate,
                    Writeback::NoWriteback,
                    2,
                    3,
                ),
                /* CA */
                Instruction::new(Opcode::DEX, AddressingMode::Implicit, Writeback::X, 1, 2),
                /* CB */
                Instruction::new(
                    Opcode::Invalid(0xCB),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* CC */
                Instruction::new(
                    Opcode::CPY,
                    AddressingMode::Absolute,
                    Writeback::NoWriteback,
                    3,
                    4,
                ),
                /* CD */
                Instruction::new(
                    Opcode::CMP,
                    AddressingMode::Absolute,
                    Writeback::NoWriteback,
                    3,
                    4,
                ),
                /* CE */
                Instruction::new(
                    Opcode::DEC,
                    AddressingMode::Absolute,
                    Writeback::Memory,
                    3,
                    6,
                ),
                /* CF */
                Instruction::new(
                    Opcode::Invalid(0xCF),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* D0 */
                Instruction::new(Opcode::BNE, AddressingMode::Relative, Writeback::PC, 2, 2),
                /* D1 */
                Instruction::new(
                    Opcode::CMP,
                    AddressingMode::IndirectY,
                    Writeback::NoWriteback,
                    2,
                    5,
                ),
                /* D2 */
                Instruction::new(
                    Opcode::Invalid(0xD2),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* D3 */
                Instruction::new(
                    Opcode::Invalid(0xD3),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* D4 */
                Instruction::new(
                    Opcode::Invalid(0xD4),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* D5 */
                Instruction::new(
                    Opcode::CMP,
                    AddressingMode::ZeroPageX,
                    Writeback::NoWriteback,
                    2,
                    4,
                ),
                /* D6 */
                Instruction::new(
                    Opcode::DEC,
                    AddressingMode::ZeroPageX,
                    Writeback::Memory,
                    2,
                    6,
                ),
                /* D7 */
                Instruction::new(
                    Opcode::Invalid(0xD7),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* D8 */
                Instruction::new(
                    Opcode::CLD,
                    AddressingMode::Implicit,
                    Writeback::NoWriteback,
                    1,
                    2,
                ),
                /* D9 */
                Instruction::new(
                    Opcode::CMP,
                    AddressingMode::AbsoluteY,
                    Writeback::NoWriteback,
                    3,
                    4,
                ),
                /* DA */
                Instruction::new(
                    Opcode::Invalid(0xDA),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* DB */
                Instruction::new(
                    Opcode::Invalid(0xDB),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* DC */
                Instruction::new(
                    Opcode::Invalid(0xDC),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* DD */
                Instruction::new(
                    Opcode::CMP,
                    AddressingMode::AbsoluteX,
                    Writeback::NoWriteback,
                    3,
                    4,
                ),
                /* DE */
                Instruction::new(
                    Opcode::DEC,
                    AddressingMode::AbsoluteX,
                    Writeback::Memory,
                    3,
                    7,
                ),
                /* DF */
                Instruction::new(
                    Opcode::Invalid(0xDF),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* E0 */
                Instruction::new(
                    Opcode::CPX,
                    AddressingMode::Immediate,
                    Writeback::NoWriteback,
                    2,
                    2,
                ),
                /* E1 */
                Instruction::new(
                    Opcode::SBC,
                    AddressingMode::IndirectX,
                    Writeback::Accumulator,
                    2,
                    6,
                ),
                /* E2 */
                Instruction::new(
                    Opcode::Invalid(0xE2),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* E3 */
                Instruction::new(
                    Opcode::Invalid(0xE3),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* E4 */
                Instruction::new(
                    Opcode::CPX,
                    AddressingMode::ZeroPage,
                    Writeback::NoWriteback,
                    2,
                    3,
                ),
                /* E5 */
                Instruction::new(
                    Opcode::SBC,
                    AddressingMode::ZeroPage,
                    Writeback::Accumulator,
                    2,
                    3,
                ),
                /* E6 */
                Instruction::new(
                    Opcode::INC,
                    AddressingMode::ZeroPage,
                    Writeback::Memory,
                    2,
                    5,
                ),
                /* E7 */
                Instruction::new(
                    Opcode::Invalid(0xE7),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* E8 */
                Instruction::new(Opcode::INX, AddressingMode::Implicit, Writeback::X, 1, 2),
                /* E9 */
                Instruction::new(
                    Opcode::SBC,
                    AddressingMode::Immediate,
                    Writeback::Accumulator,
                    2,
                    2,
                ),
                /* EA */
                Instruction::new(
                    Opcode::NOP,
                    AddressingMode::Implicit,
                    Writeback::NoWriteback,
                    1,
                    2,
                ),
                /* EB */
                Instruction::new(
                    Opcode::Invalid(0xEB),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* EC */
                Instruction::new(
                    Opcode::CPX,
                    AddressingMode::Absolute,
                    Writeback::NoWriteback,
                    3,
                    4,
                ),
                /* ED */
                Instruction::new(
                    Opcode::SBC,
                    AddressingMode::Absolute,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* EE */
                Instruction::new(
                    Opcode::INC,
                    AddressingMode::Absolute,
                    Writeback::Memory,
                    3,
                    6,
                ),
                /* EF */
                Instruction::new(
                    Opcode::Invalid(0xEF),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* F0 */
                Instruction::new(Opcode::BEQ, AddressingMode::Relative, Writeback::PC, 2, 2),
                /* F1 */
                Instruction::new(
                    Opcode::SBC,
                    AddressingMode::IndirectY,
                    Writeback::Accumulator,
                    2,
                    5,
                ),
                /* F2 */
                Instruction::new(
                    Opcode::Invalid(0xF2),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* F3 */
                Instruction::new(
                    Opcode::Invalid(0xF3),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* F4 */
                Instruction::new(
                    Opcode::Invalid(0xF4),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* F5 */
                Instruction::new(
                    Opcode::SBC,
                    AddressingMode::ZeroPageX,
                    Writeback::Accumulator,
                    2,
                    4,
                ),
                /* F6 */
                Instruction::new(
                    Opcode::INC,
                    AddressingMode::ZeroPageX,
                    Writeback::Memory,
                    2,
                    6,
                ),
                /* F7 */
                Instruction::new(
                    Opcode::Invalid(0xF7),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* F8 */
                Instruction::new(
                    Opcode::SED,
                    AddressingMode::Implicit,
                    Writeback::NoWriteback,
                    1,
                    2,
                ),
                /* F9 */
                Instruction::new(
                    Opcode::SBC,
                    AddressingMode::AbsoluteY,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* FA */
                Instruction::new(
                    Opcode::Invalid(0xFA),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* FB */
                Instruction::new(
                    Opcode::Invalid(0xFB),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* FC */
                Instruction::new(
                    Opcode::Invalid(0xFC),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
                /* FD */
                Instruction::new(
                    Opcode::SBC,
                    AddressingMode::AbsoluteX,
                    Writeback::Accumulator,
                    3,
                    4,
                ),
                /* FE */
                Instruction::new(
                    Opcode::INC,
                    AddressingMode::AbsoluteX,
                    Writeback::Memory,
                    3,
                    7,
                ),
                /* FF */
                Instruction::new(
                    Opcode::Invalid(0xFF),
                    AddressingMode::None,
                    Writeback::NoWriteback,
                    0,
                    0,
                ),
            ],
        }
    }
}

impl super::InstructionDecoder for InstructionDecoder {
    fn decode(&self, opcode: Byte) -> Result<&Instruction> {
        Ok(&self.decode_table[opcode as usize])
    }
}
