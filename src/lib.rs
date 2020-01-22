extern crate embedded_hal as hal;

mod register_file;
use crate::register_file::*;

const REG_HOT_JUNC_TEMP: u8 = 0x00;
const REG_THERMO_CFG: u8 = 0x05;

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

struct Mcp9600<T> {
    registers : T
}
impl<T> Mcp9600<T>
where T: MemoryAddressReader + MemoryAddressWriter {
    pub fn new(registers: T, thermo_type: ThermocoupleType, filter_coeffs: FilterCoefficients) -> Mcp9600<T> {
        let mcp9600 = Mcp9600 {
            registers,
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
        mcp9600.registers.write(REG_THERMO_CFG, &bytes);

        return mcp9600;
    }

    pub fn read_temp(&self) -> Temperature {
        let mut buffer: [u8; 2] = [0; 2];
        self.registers.read(REG_HOT_JUNC_TEMP, &mut buffer);
        let temp: Temperature = (buffer[0] as u16) | ((buffer[1] as u16) << 8);
        return temp;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_initializes_driver() {
        let registers = RegisterFileFake::new();
        let thermo_type = ThermocoupleType::TypeK;
        let filter_coeffs = FilterCoefficients::FullFilter;

        let mcp9600 = Mcp9600::new(registers, thermo_type, filter_coeffs);
    }

    #[test]
    fn new_writes_thermo_config_register() {
        let registers = RegisterFileFake::new();
        let thermo_type = ThermocoupleType::TypeK;
        let filter_coeffs = FilterCoefficients::FullFilter;

        let mcp9600 = Mcp9600::new(registers, thermo_type, filter_coeffs);

        let mut buffer: [u8; 1] = [0; 1];
        mcp9600.registers.read(REG_THERMO_CFG, &mut buffer);
        assert!(buffer[0] == 0b00000111);
    }

    #[test]
    fn read_temp_reads_temp_reg() {
        let registers = RegisterFileFake::new();

        let thermo_type = ThermocoupleType::TypeK;
        let filter_coeffs = FilterCoefficients::FullFilter;

        let mcp9600 = Mcp9600::new(registers, thermo_type, filter_coeffs);

        let temp_in : u16 = 0x12;
        mcp9600.registers.write(REG_HOT_JUNC_TEMP, &temp_in.to_be_bytes());

        let temp_out = mcp9600.read_temp();
        assert!(temp_out == temp_in);
    }
}
