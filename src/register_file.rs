extern crate embedded_hal as hal;

pub trait MemoryAddressReader {
    fn read(&self, idx: u8, bytes: &mut [u8]);
}
pub trait MemoryAddressWriter {
    fn write(&mut self, idx: u8, bytes: &[u8]);
}

pub struct RegisterFile<T> {
    i2c_bus: T,
    i2c_addr: u8,
}
impl<T> RegisterFile<T>
where T: hal::blocking::i2c::Write + hal::blocking::i2c::Read + hal::blocking::i2c::WriteRead {
    pub fn new(i2c_bus: T, i2c_addr: u8) -> RegisterFile<T> {
        RegisterFile {
            i2c_bus,
            i2c_addr,
        }
    }
}
impl<T> MemoryAddressReader for RegisterFile<T> {
    fn read(&self, idx: u8, bytes: &mut [u8]) {
        // TODO
        return;
    }
}
impl<T> MemoryAddressWriter for RegisterFile<T> {
    fn write(&mut self, idx: u8, bytes: &[u8]) {
        // TODO
        return;
    }
}

pub struct RegisterFileFake {
    mem: [[u8; 4]; 256],
}
impl RegisterFileFake {
    pub fn new() -> RegisterFileFake {
        RegisterFileFake {
            mem: [[0; 4]; 256],
        }
    }
}
impl MemoryAddressReader for RegisterFileFake {
    fn read(&self, idx: u8, bytes: &mut [u8]) {
        for i in 0..bytes.len() {
            bytes[i] = self.mem[idx as usize][i]
        }
        println!("read {:?} from {:?}", self.mem[idx as usize], idx);
        println!("bytes coming back as {:?}", bytes);

    }
}
impl MemoryAddressWriter for RegisterFileFake {
    fn write(&mut self, idx: u8, bytes: &[u8]) {
        for i in 0..bytes.len() {
            self.mem[idx as usize][i] = bytes[i];
        }
        println!("wrote {:?} to {:?}", bytes, idx);
        println!("mem now reads {:?}", self.mem[idx as usize]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_generates_correct_i2c_traffic() {
        // TODO
    }
    #[test]
    fn write_generates_correct_i2c_traffic() {
        // TODO
    }
}
