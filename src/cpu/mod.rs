pub mod address;
pub mod ram;
pub mod registers;
pub mod rom;
pub mod instruction_decode;
pub mod data;

pub type Byte = u8;
pub type Word = u16;
pub type Address = Word;
pub type Data = Byte;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    AddressOutOfRange(Address),
    InvalidAddress(Address),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Memory {
    fn read_byte(&self, address: Address) -> Result<Byte>;
    fn read_word(&self, address: Address) -> Result<Word>;

    fn write_byte(&mut self, address: Address, data: Byte) -> Result<()>;
    fn write_word(&mut self, address: Address, data: Word) -> Result<()>;
}

#[derive(Debug)]
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
    Invalid = -1,
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
    pub fn new(opcode: Opcode, addressing_mode: AddressingMode, writeback: Writeback, byte_length: usize, ticks: usize) -> Self {
        Instruction { opcode, addressing_mode, writeback, byte_length, ticks }
    }
}

pub trait InstructionDecoder {
    fn decode(&self, opcode: Byte) -> Result<&Instruction>;
}

pub trait AddressDispatcher {
    fn implicit(&self) -> Result<Address>;
    fn accumulator(&self) -> Result<Address>;
    fn immediate(&self) -> Result<Address>;
    fn zero_page(&self) -> Result<Address>;
    fn zero_page_x(&self) -> Result<Address>;
    fn zero_page_y(&self) -> Result<Address>;
    fn relative(&self) -> Result<Address>;
    fn absolute(&self) -> Result<Address>;
    fn absolute_x(&self) -> Result<Address>;
    fn absolute_y(&self) -> Result<Address>;
    fn indirect(&self) -> Result<Address>;
    fn indirect_x(&self) -> Result<Address>;
    fn indirect_y(&self) -> Result<Address>;
}

pub trait DataDispatcher {
    fn implicit(&self) -> Result<Data>;
    fn accumulator(&self) -> Result<Data>;
    fn immediate(&self) -> Result<Data>;
    fn zero_page(&self) -> Result<Data>;
    fn zero_page_x(&self) -> Result<Data>;
    fn zero_page_y(&self) -> Result<Data>;
    fn relative(&self) -> Result<Data>;
    fn absolute(&self) -> Result<Data>;
    fn absolute_x(&self) -> Result<Data>;
    fn absolute_y(&self) -> Result<Data>;
    fn indirect(&self) -> Result<Data>;
    fn indirect_x(&self) -> Result<Data>;
    fn indirect_y(&self) -> Result<Data>;
}
