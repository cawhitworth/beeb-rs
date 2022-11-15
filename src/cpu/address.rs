use crate::cpu::{Address, Result};

pub struct AddressDispatcher {}

impl AddressDispatcher {
    pub fn new() -> Self {
        AddressDispatcher {}
    }
}

impl super::AddressDispatcher for AddressDispatcher {
    fn implicit(&self) -> Result<Address> {
        todo!()
    }

    fn accumulator(&self) -> Result<Address> {
        todo!()
    }

    fn immediate(&self) -> Result<Address> {
        todo!()
    }

    fn zero_page(&self) -> Result<Address> {
        todo!()
    }

    fn zero_page_x(&self) -> Result<Address> {
        todo!()
    }

    fn zero_page_y(&self) -> Result<Address> {
        todo!()
    }

    fn relative(&self) -> Result<Address> {
        todo!()
    }

    fn absolute(&self) -> Result<Address> {
        todo!()
    }

    fn absolute_x(&self) -> Result<Address> {
        todo!()
    }

    fn absolute_y(&self) -> Result<Address> {
        todo!()
    }

    fn indirect(&self) -> Result<Address> {
        todo!()
    }

    fn indirect_x(&self) -> Result<Address> {
        todo!()
    }

    fn indirect_y(&self) -> Result<Address> {
        todo!()
    }
}
