use super::Data;

pub struct DataDispatcher {

}

impl DataDispatcher {
    pub fn new() -> Self {
        DataDispatcher {  }
    }
}

impl super::DataDispatcher for DataDispatcher {
    fn implicit(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn accumulator(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn immediate(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn zero_page(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn zero_page_x(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn zero_page_y(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn relative(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn absolute(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn absolute_x(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn absolute_y(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn indirect(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn indirect_x(&self) -> super::Result<super::Data> {
        todo!()
    }

    fn indirect_y(&self) -> super::Result<super::Data> {
        todo!()
    }
}