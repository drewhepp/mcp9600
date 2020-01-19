#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_state() {
        const i2c_addr : u8 = 0x66;
        const thermo_type : ThermocoupleType = ThermocoupleType::TypeK;
        const filter_coeffs : FilterCoefficients = 2;
        
        let mut i2c_slave_mock = I2cSlave::new(i2c_addr);
        let mcp9600 = Mcp9600::new(thermo_type, filter_coeffs, i2c_slave_mock);

        assert!(mcp9600.thermo_type == thermo_type);
        assert!(mcp9600.filter_coeffs == filter_coeffs);
    }

    #[test]
    fn read_temp_reads_temp() {
        const i2c_addr : u8 = 0x66;
        const thermo_type : ThermocoupleType = ThermocoupleType::TypeK;
        const filter_coeffs : FilterCoefficients = 2;
        
        let mut i2c_slave_mock = I2cSlave::new(i2c_addr);
        let mcp9600 = Mcp9600::new(thermo_type, filter_coeffs, i2c_slave_mock);

        let temp_in = 0x12;
        i2c_slave_mock.write(0x0, temp_in);

        let temp_out = mcp9600.read_temp();
        assert!(temp_out == temp_in);
    }
}

struct I2cSlave {
    addr : u8,
    registers : [u32; 255],
}
impl I2cSlave {
    pub fn new(addr : u8) -> I2cSlave {
        I2cSlave {
            addr,
            registers : [0; 255]
        }
    }
    pub fn read(&self, ptr : usize) -> u32 {
        return self.registers[ptr];
    }
    pub fn write(&mut self, ptr : usize, data : u32) {
        self.registers[ptr] = data;
    }
}

type Temperature = u32; // 32 bit fixed point
type FilterCoefficients = isize; // todo
    
#[derive(PartialEq)]
enum ThermocoupleType {
    TypeK,
}

#[derive(PartialEq)]
enum Alert {
    Alert1,
    Alert2,
    Alert3,
    Alert4,
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

struct Mcp9600 {
    thermo_type : ThermocoupleType,
    filter_coeffs : FilterCoefficients,
    i2c_slave : &I2cSlave,
}

impl Mcp9600 {
    pub fn new(thermo_type : ThermocoupleType, filter_coeffs : FilterCoefficients, i2c_slave : &I2cSlave) -> Mcp9600 {
        Mcp9600 {
            thermo_type,
            filter_coeffs,
            i2c_slave,
        }
    }
    pub fn read_temp(&self) -> Temperature {
        return 1;
    }
    pub fn set_alert(&self, alert : Alert) {}
}
