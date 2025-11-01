pub mod opcodes;

use crate::utils::*;

pub struct Cpu {
    // CPU registers: 16bit program counter and stack pointer
    // 8 general purpose 8 bit registers
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8
}

impl Cpu {
    // Initializing all registers to 0
    pub fn new() -> Self {
        Cpu {
            pc: 0x0000,
            sp: 0x0000,
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00
        }
    }

    // Getter and setter functions for the registers
    // for register access based on op instructions
    pub fn get_r8(&self, r: Regs) -> u8 {
        match r {
            Regs::A => { self.a },
            Regs::B => { self.b },
            Regs::C => { self.c },
            Regs::D => { self.d },
            Regs::E => { self.e },
            Regs::F => { self.f },
            Regs::H => { self.h },
            Regs::L => { self.l }
        }
    }

    pub fn set_r8(&mut self, r: Regs, val: u8) {
        match r {
            Regs::A => { self.a = val },
            Regs::B => { self.b = val },
            Regs::C => { self.c = val },
            Regs::D => { self.d = val },
            Regs::E => { self.e = val },
            // note: bottom 4 bits of F are always 0
            Regs::F => { self.f = val & 0xF0 },
            Regs::H => { self.h = val },
            Regs::L => { self.l = val }
        }
    }

    pub fn get_r16(&self, r: Regs16) -> u16 {
        match r {
            Regs16::AF => { merge_bytes(self.a, self.f) },
            Regs16::BC => { merge_bytes(self.b, self.c) },
            Regs16::DE => { merge_bytes(self.d, self.e) },
            Regs16::HL => { merge_bytes(self.h, self.l) },
            Regs16::SP => { self.sp }
        }
    }

    pub fn set_r16(&mut self, r: Regs16, val: u16) {
        let high = val.high_byte();
        let low = val.low_byte();
        match r {
            Regs16::AF => {
                self.set_r8(Regs::A, high);
                self.set_r8(Regs::F, low);
            },
            Regs16::BC => {
                self.set_r8(Regs::B, high);
                self.set_r8(Regs::C, low);
            },
            Regs16::DE => {
                self.set_r8(Regs::D, high);
                self.set_r8(Regs::E, low);
            },
            Regs16::HL => {
                self.set_r8(Regs::H, high);
                self.set_r8(Regs::L, low);
            },
            Regs16::SP => { self.sp = val }
        }
    }

    pub fn get_flag(&self, f: Flags) -> bool {
        match f {
            Flags::Z => { (self.f & 0b1000_0000) != 0 },
            Flags::N => { (self.f & 0b0100_0000) != 0 },
            Flags::H => { (self.f & 0b0010_0000) != 0 },
            Flags::C => { (self.f & 0b0001_0000) != 0 }
        }
    }

    pub fn set_flag(&mut self, f: Flags, val: bool) {
        if val {
            match f {
                Flags::Z => { self.f |= 0b1000_0000 },
                Flags::N => { self.f |= 0b0100_0000 },
                Flags::H => { self.f |= 0b0010_0000 },
                Flags::C => { self.f |= 0b0001_0000 }
            }
        } else {
            match f {
                Flags::Z => { self.f &= 0b0111_0000 },
                Flags::N => { self.f &= 0b1011_0000 },
                Flags::H => { self.f &= 0b1101_0000 },
                Flags::C => { self.f &= 0b1110_0000 }
            }
        }
    }
}

// 8 bit registers of gameboy processor
// allows us to access registers without referring to fields like 'self.a', 'self.b' directly
#[derive(Clone, Copy)]
pub enum Regs {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L
}

// Gameboy converts 2 8bit registers into a 16 bit register
#[derive(Clone, Copy)]
pub enum Regs16 {
    AF,
    BC,
    DE,
    HL,
    SP
}

pub enum Flags {
    Z,
    N,
    H,
    C
}