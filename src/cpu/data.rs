use crate::cpu::{Data, Result};

pub struct DataDispatcher {}

impl DataDispatcher {
    pub fn new() -> Self {
        DataDispatcher {}
    }
}

impl super::DataDispatcher for DataDispatcher {
    fn implicit(&self) -> Result<Data> {
        todo!()
    }

    fn accumulator(&self) -> Result<Data> {
        todo!()
    }

    fn immediate(&self) -> Result<Data> {
        todo!()
    }

    fn zero_page(&self) -> Result<Data> {
        todo!()
    }

    fn zero_page_x(&self) -> Result<Data> {
        todo!()
    }

    fn zero_page_y(&self) -> Result<Data> {
        todo!()
    }

    fn relative(&self) -> Result<Data> {
        todo!()
    }

    fn absolute(&self) -> Result<Data> {
        todo!()
    }

    fn absolute_x(&self) -> Result<Data> {
        todo!()
    }

    fn absolute_y(&self) -> Result<Data> {
        todo!()
    }

    fn indirect(&self) -> Result<Data> {
        todo!()
    }

    fn indirect_x(&self) -> Result<Data> {
        todo!()
    }

    fn indirect_y(&self) -> Result<Data> {
        todo!()
    }
}
