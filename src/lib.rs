extern crate embedded_hal as hal;
extern crate shared_bus;

pub mod mcp9600 {

    #[cfg(test)]
    mod tests {
        use super::*;
        use i2cmock::*;

        #[test]
        fn new_sets_state() {
            const thermo_type : ThermocoupleType = ThermocoupleType::TypeK;
            const filter_coeffs : FilterCoefficients = 2;

            let i2c = I2cMock::new();
            let i2c_manager = shared_bus::StdBusManager::new(i2c);
            
            let mcp9600 = Mcp9600::new(i2c_manager.acquire(), thermo_type, filter_coeffs);

            assert!(mcp9600.thermo_type == thermo_type);
            assert!(mcp9600.filter_coeffs == filter_coeffs);
        }

        #[test]
        fn read_temp_reads_temp() {
            const thermo_type : ThermocoupleType = ThermocoupleType::TypeK;
            const filter_coeffs : FilterCoefficients = 2;

            let i2c = I2cMock::new();
            let i2c_manager = shared_bus::StdBusManager::new(i2c);
            let mcp9600 = Mcp9600::new(i2c_manager.acquire(), thermo_type, filter_coeffs);

            let i2c_mock_handle = i2c_manager.acquire();
            let temp_in = 0x12;
            // TODO write temp_in to reg 0x00

            let temp_out = mcp9600.read_temp();
            assert!(temp_out == temp_in);
        }

        pub mod i2cmock {
            pub enum Error {
                Generic,
            }
            pub struct I2cMock {
                data : u8,
            }
            impl hal::blocking::i2c::Write for I2cMock {
                type Error = Error;
                fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
                    Err(Error::Generic)
                }
            }
            impl hal::blocking::i2c::Read for I2cMock {
                type Error = Error;
                fn read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
                    Err(Error::Generic)
                }
            }
            impl I2cMock {
                pub fn new() -> I2cMock {
                    return I2cMock{ data : 1}
                }
            }

        }
    }

    pub type Temperature = u32; // 32 bit fixed point
    pub type FilterCoefficients = isize; // todo
        
    #[derive(PartialEq)]
    pub enum ThermocoupleType {
        TypeK,
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

    pub struct Mcp9600 {
        thermo_type : ThermocoupleType,
        filter_coeffs : FilterCoefficients,

    }

    impl Mcp9600 {
        pub fn new<T: hal::blocking::i2c::Write + hal::blocking::i2c::Read>
            (i2c_bus: T, thermo_type: ThermocoupleType, filter_coeffs: FilterCoefficients) -> Mcp9600 {
            return Mcp9600 {
                thermo_type,
                filter_coeffs,
            }
        }
        pub fn read_temp(&self) -> Temperature {
            return 1;
        }
    }
}
