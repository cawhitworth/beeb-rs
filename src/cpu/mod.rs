pub mod address;
pub mod ram;
pub mod registers;
pub mod rom;

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

trait AddressDispatcher {
    fn implicit(self) -> Result<Address>;
    fn accumulator(self) -> Result<Address>;
    fn immediate(self) -> Result<Address>;
    fn zero_page(self) -> Result<Address>;
    fn zero_page_x(self) -> Result<Address>;
    fn zero_page_y(self) -> Result<Address>;
    fn relative(self) -> Result<Address>;
    fn absolute(self) -> Result<Address>;
    fn absolute_x(self) -> Result<Address>;
    fn absolute_y(self) -> Result<Address>;
    fn indirect(self) -> Result<Address>;
    fn indirect_x(self) -> Result<Address>;
    fn indirect_y(self) -> Result<Address>;
}

trait DataDispatcher {
    fn implicit(self) -> Result<Data>;
    fn accumulator(self) -> Result<Data>;
    fn immediate(self) -> Result<Data>;
    fn zero_page(self) -> Result<Data>;
    fn zero_page_x(self) -> Result<Data>;
    fn zero_page_y(self) -> Result<Data>;
    fn relative(self) -> Result<Data>;
    fn absolute(self) -> Result<Data>;
    fn absolute_x(self) -> Result<Data>;
    fn absolute_y(self) -> Result<Data>;
    fn indirect(self) -> Result<Data>;
    fn indirect_x(self) -> Result<Data>;
    fn indirect_y(self) -> Result<Data>;
}
