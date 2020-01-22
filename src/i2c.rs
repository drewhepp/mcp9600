pub enum Error {
    Generic,
}

pub struct I2cDummy {
    addr: u8,
}
impl I2cDummy {
    pub fn new(addr: u8) -> I2cDummy {
        I2cDummy{
            addr,
        }
    }
}
impl hal::blocking::i2c::Write for I2cDummy {
    type Error = Error;
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        return Ok(());
    }
}
impl hal::blocking::i2c::Read for I2cDummy {
    type Error = Error;
    fn read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
        return Ok(());
    }
}
impl hal::blocking::i2c::WriteRead for I2cDummy {
    type Error = Error;
    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        return Ok(());
    }
}
