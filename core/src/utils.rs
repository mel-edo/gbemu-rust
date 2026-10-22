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

pub fn check_c_carry_u8(lhs: u8, rhs: u8) -> bool {
    lhs.checked_add(rhs).is_none()
}
pub fn check_c_carry_u16(lhs: u16, rhs: u16) -> bool {
    lhs.checked_add(rhs).is_none()
}
pub fn check_c_borrow_u8(lhs: u8, rhs: u8) -> bool {
    lhs.checked_sub(rhs).is_none()
}
pub fn check_c_borrow_u16(lhs: u16, rhs: u16) -> bool {
    lhs.checked_sub(rhs).is_none()
}
pub fn check_h_carry_u8(lhs: u8, rhs: u8) -> bool {
    ((lhs & 0xF) + (rhs & 0xF)) > 0xF
}
pub fn check_h_carry_u16(lhs: u16, rhs: u16) -> bool {
    (lhs & 0xFFF) + (rhs & 0xFFF) > 0x0FFF
}
pub fn check_h_borrow_u8(lhs: u8, rhs: u8) -> bool {
    (lhs & 0xF).checked_sub(rhs & 0xF).is_none()
}
pub fn check_h_borrow_u16(lhs: u16, rhs: u16) -> bool {
    (lhs & 0xFFF).checked_sub(rhs & 0xFFF).is_none()
}

pub trait BitOps {
    fn get_bit(&self, bit: u8) -> bool;
    fn set_bit(&mut self, bit: u8, set: bool);
}

macro_rules! impl_bitops {
    ($T:ty) => {
        impl BitOps for $T {
            fn get_bit(&self, bit: u8) -> bool {
                let mask = 0b1 << bit;
                (self & mask) != 0
            }

            fn set_bit(&mut self, bit: u8, set: bool) {
                let mask = 0b1 << bit;
                if set {
                    *self |= mask;
                } else {
                    *self &= !mask;
                }
            }
        }
    }
}

impl_bitops!(u8);
impl_bitops!(u16);

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const DISPLAY_BUFFER: usize = SCREEN_HEIGHT * SCREEN_WIDTH * 4;

pub const GB_PALETTE: [[u8; 4]; 4] = [
    [255, 255, 255, 255],
    [128, 128, 128, 255],
    [64, 64, 64, 255],
    [0, 0, 0, 255]
];

// CGB Color Palettes - 32768 colors (15-bit RGB555)
pub const CGB_PALETTE_SIZE: usize = 64; // 8 palettes * 4 colors each for BG and OBJ

// CGB Register addresses
pub const KEY1: u16 = 0xFF4D;      // Speed switch
pub const VBK: u16 = 0xFF4F;       // VRAM bank select
pub const HDMA1: u16 = 0xFF51;     // HDMA source high
pub const HDMA2: u16 = 0xFF52;     // HDMA source low
pub const HDMA3: u16 = 0xFF53;     // HDMA dest high
pub const HDMA4: u16 = 0xFF54;     // HDMA dest low
pub const HDMA5: u16 = 0xFF55;     // HDMA length/mode/start
pub const RP: u16 = 0xFF56;        // Infrared (CGB only)
pub const BCPS: u16 = 0xFF68;      // BG palette index/spec
pub const BCPD: u16 = 0xFF69;      // BG palette data
pub const OCPS: u16 = 0xFF6A;      // OBJ palette index/spec
pub const OCPD: u16 = 0xFF6B;      // OBJ palette data
pub const SVBK: u16 = 0xFF70;      // WRAM bank select

// HDMA constants
pub const HDMA_GDMA_MODE: u8 = 0x80;  // Bit 7: 0=GDMA (immediate), 1=HDMA (during HBlank)
pub const HDMA_LENGTH_MASK: u8 = 0x7F; // Bits 0-6: length (blocks - 1)
pub const HDMA_BLOCK_SIZE: usize = 0x10; // 16 bytes per block

#[derive(Clone, Copy)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        Point { x, y }
    }
}

pub fn unpack_u8(val: u8) -> [u8; 4] {
    let mut output = [0; 4];
    output[0] = val & 0b0000_0011;
    output[1] = (val & 0b0000_1100) >> 2;
    output[2] = (val & 0b0011_0000) >> 4;
    output[3] = (val & 0b1100_0000) >> 6;
    return output;
}

pub fn pack_u8(a: &[u8]) -> u8 {
    let mut output = a[0];
    output |= a[1] << 2;
    output |= a[2] << 4;
    output |= a[3] << 6;
    return output;
}