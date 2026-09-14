use crate::utils::BitOps;

pub const SB: u16 = 0xFF01;
pub const SC: u16 = 0xFF02;

pub struct Serial {
    sb: u8,
    sc: u8,
    transferring: bool,
    bits_shifted: u8,
    timer: u32,
    cpu_freq: u32,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            sb: 0,
            sc: 0,
            transferring: false,
            bits_shifted: 0,
            timer: 0,
            cpu_freq: 4194304,
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            SB => self.sb,
            SC => self.sc | 0x7E, // Unused bits read as 1
            _ => 0xFF,
        }
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            SB => self.sb = val,
            SC => {
                self.sc = val;
                if val.get_bit(7) && val.get_bit(0) {
                    // Start transfer, internal clock
                    self.transferring = true;
                    self.bits_shifted = 0;
                    self.timer = self.cpu_freq / 8192; // Internal clock rate
                }
                
                // Logging for test ROMs like blargg's which output via serial
                if val == 0x81 {
                    print!("{}", self.sb as char);
                }
            },
            _ => {}
        }
    }

    pub fn tick(&mut self, cycles: u8) -> bool {
        if !self.transferring {
            return false;
        }

        let mut interrupt = false;
        let cycles_u32 = cycles as u32;

        if self.timer > cycles_u32 {
            self.timer -= cycles_u32;
        } else {
            // Shift one bit
            self.sb = (self.sb << 1) | 1; // Shift in 1s for unconnected link cable
            self.bits_shifted += 1;
            
            if self.bits_shifted == 8 {
                // Transfer complete
                self.transferring = false;
                self.sc.set_bit(7, false);
                interrupt = true;
            } else {
                self.timer = self.cpu_freq / 8192;
            }
        }

        interrupt
    }
}
