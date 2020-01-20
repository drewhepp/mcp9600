extern crate embedded_hal as hal;
extern crate shared_bus;

pub mod mcp9600 {

    #[cfg(test)]
    mod tests {
        use super::*;
        use i2c_dummy::*;

        #[test]
        fn new_sets_i2c_addr() {
            const i2c_addr : u8 = 0x66;
            const thermo_type : ThermocoupleType = ThermocoupleType::TypeK;
            const filter_coeffs : FilterCoefficients = FilterCoefficients::NoFilter;

            let mcp9600 = Mcp9600::new(I2cDummy::new(), i2c_addr, thermo_type, filter_coeffs);

            assert_eq!(mcp9600.i2c_addr, i2c_addr);
        }

        #[test]
        fn new_writes_config_reg() {
            const i2c_addr : u8 = 0x66;
            const thermo_type : ThermocoupleType = ThermocoupleType::TypeK;
            const filter_coeffs : FilterCoefficients = FilterCoefficients::NoFilter;

            let mcp9600 = Mcp9600::new(I2cDummy::new(), i2c_addr, thermo_type, filter_coeffs);

            // expect Mcp9600::write_reg(0x05, 0b0xxx0xxx)
            // verify expectations
        }

        #[test]
        fn read_temp_reads_temp_reg() {
            const i2c_addr : u8 = 0x66;
            const thermo_type : ThermocoupleType = ThermocoupleType::TypeK;
            const filter_coeffs : FilterCoefficients = FilterCoefficients::NoFilter;

            let mcp9600 = Mcp9600::new(I2cDummy::new(), i2c_addr, thermo_type, filter_coeffs);

            let temp_in = 0x12;
            // expect Mcp9600::read_reg(0x00)
            // return temp_in

            let temp_out = mcp9600.read_temp();

            //verify expectation

            assert!(temp_out == temp_in);
        }


        pub mod i2c_dummy {
            pub enum Error {
                Generic,
            }
            pub struct I2cDummy {
                junk : u8,
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
            impl I2cDummy {
                pub fn new() -> I2cDummy {
                    return I2cDummy{ junk: 42 };
                }
            }

        }
    }

    pub type Temperature = u16; // 16 bit fixed point


    #[derive(PartialEq)]
    pub enum FilterCoefficients {
        NoFilter,
        FullFilter,
    }
        
    #[derive(PartialEq)]
    pub enum ThermocoupleType {
        TypeK,
    }

    #[derive(PartialEq)]
    enum RegPtr {
        ThermoCfg,
    }
        
    struct Status {
        burst_complete : bool,
        hot_junction_temperature_updated : bool,
        input_range : bool,
        alert_4_status : bool,
        alert_3_status : bool,
        alert_2_status : bool,
        alert_1_status : bool,
    }

    pub struct Mcp9600<T> {
        i2c_bus: T,
        i2c_addr: u8,
    }

    impl<T> Mcp9600 <T>
    where T: hal::blocking::i2c::Write + hal::blocking::i2c::Read + hal::blocking::i2c::WriteRead {
        pub fn new(i2c_bus: T, i2c_addr: u8, thermo_type: ThermocoupleType, filter_coeffs: FilterCoefficients) -> Mcp9600<T> {

            // Store the i2c bus and address to communicate with the slave
            let mcp9600 =  Mcp9600 {
                i2c_bus,
                i2c_addr,
            };

            let thermo_type = match thermo_type {
                ThermocoupleType::TypeK => 0b000,
            };

            let filter_coeffs = match filter_coeffs{
                FilterCoefficients::NoFilter => 0b000,
                FilterCoefficients::FullFilter => 0b111,
            };

            // Write thermocouple type and filter coefficients to config reg
            let byte : u8 = ((thermo_type   & 0b111) << 4) |
                             (filter_coeffs & 0b111);
            let bytes: [u8; 1] = [byte; 1];
            mcp9600.reg_write(RegPtr::ThermoCfg, &bytes);

            return mcp9600;
        }
        pub fn read_temp(&self) -> Temperature {
            let mut buffer: [u8; 2] = [0; 2];
            self.reg_read(RegPtr::ThermoCfg, &mut buffer);
            let temp: Temperature = (buffer[0] as u16) | ((buffer[1] as u16) << 8);
            return temp;
        }
        fn reg_read(&self, reg_ptr: RegPtr, bytes: &mut [u8]) {
            return;
        }
        fn reg_write(&self, reg_ptr: RegPtr, bytes: &[u8]) {
            return;
        }
    }
}
