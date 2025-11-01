// Since 16 bit registers are made up of 2 8 bit registers
pub fn merge_bytes(high: u8, low: u8) -> u16 {
    (high as u16) << 8 | (low as u16)
}

// For extracting the high byte and low byte from the 16 bit register
// to get the corresponding 8 bit registers
pub trait ByteOps {
    fn high_byte(&self) -> u8;
    fn low_byte(&self) -> u8;
}
impl ByteOps for u16 {
    fn high_byte(&self) -> u8 {
        (self >> 8) as u8
    }
    fn low_byte(&self) -> u8 {
        (self & 0xFF) as u8
    }
}