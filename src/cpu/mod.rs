use std::fmt;

use self::registers::Registers;

pub mod address;
pub mod dispatch;
pub mod execution;
pub mod instruction_decode;
pub mod memory;
pub mod ram;
pub mod registers;
pub mod rom;
pub mod writeback;

pub type Byte = u8;
pub type Word = u16;
pub type Address = Word;
pub type Data = Byte;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorType {
    AddressOutOfRange(Address),
    InvalidAddressingMode,
    InvalidInstruction(Byte),
    MissingData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub pc: Option<Address>,
    pub error_type: ErrorType,
}

impl Error {
    fn with_pc(pc: Address, error_type: ErrorType) -> Self {
        Error {
            pc: Some(pc),
            error_type,
        }
    }

    fn without_pc(error_type: ErrorType) -> Self {
        Error {
            pc: None,
            error_type,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error_type {
            ErrorType::AddressOutOfRange(addr) => {
                f.write_fmt(format_args!("Address out of range (0x{:04x})", addr));
            }
            ErrorType::InvalidAddressingMode => {
                f.write_fmt(format_args!("Invalid addressing mode"));
            }
            ErrorType::InvalidInstruction(opcode) => {
                f.write_fmt(format_args!("Invalid instruction ({:02x})", opcode));
            }
            ErrorType::MissingData => {
                f.write_fmt(format_args!("Missing data"));
            }
        }

        if let Some(pc) = self.pc {
            f.write_fmt(format_args!(" at 0x{:04x}", pc));
        }

        Ok(())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Memory {
    fn length(&self) -> usize;

    fn read_byte(&self, address: Address) -> Result<Byte>;
    fn read_word(&self, address: Address) -> Result<Word>;

    fn write_byte(&mut self, address: Address, data: Byte) -> Result<()>;
    fn write_word(&mut self, address: Address, data: Word) -> Result<()>;
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    Invalid(Byte),
}

pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    None = -1,
}

pub enum Writeback {
    Accumulator,
    X,
    Y,
    Memory,
    PC,
    SP,
    PS,
    NoWriteback = -1,
}

pub struct Instruction {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
    pub writeback: Writeback,
    pub byte_length: usize,
    pub ticks: usize,
}

impl Instruction {
    pub fn new(
        opcode: Opcode,
        addressing_mode: AddressingMode,
        writeback: Writeback,
        byte_length: usize,
        ticks: usize,
    ) -> Self {
        Instruction {
            opcode,
            addressing_mode,
            writeback,
            byte_length,
            ticks,
        }
    }
}

pub trait InstructionDecoder {
    fn decode(&self, opcode: Byte) -> Result<&Instruction>;
}

pub trait AddressDataDispatcher<M>
where
    M: Memory,
{
    fn get_address(
        &self,
        mode: &AddressingMode,
        memory: &M,
        registers: &Registers,
    ) -> Result<Option<Address>>;

    fn get_data(
        &self,
        mode: &AddressingMode,
        memory: &M,
        registers: &Registers,
    ) -> Result<Option<Data>>;
}

pub trait ExecutionUnit<M>
where
    M: Memory,
{
    fn execute(
        &self,
        opcode: &Opcode,
        data: Option<Data>,
        address: Option<Address>,
        memory: &M,
        registers: &mut Registers,
    ) -> Result<Option<Data>>;
}

pub trait WritebackUnit<M>
where
    M: Memory,
{
    fn writeback(
        &self,
        target: &Writeback,
        data: Option<Data>,
        address: Option<Address>,
        memory: &mut M,
        registers: &mut Registers,
    ) -> Result<()>;
}
