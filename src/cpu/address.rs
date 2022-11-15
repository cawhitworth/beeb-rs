use std::ops::Add;

pub struct AddressDispatcher {

}

impl AddressDispatcher {
    pub fn new() -> Self {
        AddressDispatcher {  }
    }
}

impl super::AddressDispatcher for AddressDispatcher {
    fn implicit(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn accumulator(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn immediate(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn zero_page(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn zero_page_x(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn zero_page_y(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn relative(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn absolute(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn absolute_x(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn absolute_y(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn indirect(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn indirect_x(&self) -> super::Result<super::Address> {
        todo!()
    }

    fn indirect_y(&self) -> super::Result<super::Address> {
        todo!()
    }
}