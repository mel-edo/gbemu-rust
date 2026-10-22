use crate::utils::BitOps;

pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

const TAC_ENABLE_BIT: u8 = 2;
const TIMA_COOLDOWN_OVERFLOW: u8 = 4;

pub struct Timer {
    div_counter: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    tima_cooldown: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div_counter: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            tima_cooldown: 0,
        }
    }

    pub fn read_timer(&self, addr: u16) -> u8 {
        match addr {
            DIV => (self.div_counter >> 8) as u8,
            TIMA => self.tima,
            TMA => self.tma,
            TAC => self.tac,
            _ => unreachable!("Trying to read a non-timer register"),
        }
    }

    pub fn write_timer(&mut self, addr: u16, val: u8) {
        match addr {
            DIV => {
                // Writing to DIV resets the divider counter to 0.
                // If the TIMA clock bit was 1 and becomes 0, this is a falling edge
                // that should increment TIMA (if timer is enabled).
                let old_bit = self.tima_status();
                self.div_counter = 0;
                let new_bit = self.tima_status();
                let enabled = self.tac.get_bit(TAC_ENABLE_BIT);
                
                if enabled && old_bit && !new_bit && self.tima_cooldown == 0 {
                    let (new_tima, overflow) = self.tima.overflowing_add(1);
                    self.tima = new_tima;
                    if overflow {
                        self.tima_cooldown = TIMA_COOLDOWN_OVERFLOW;
                    }
                }
            }
            TIMA => {
                self.tima = val;
                self.tima_cooldown = 0;
            }
            TMA => {
                self.tma = val;
            }
            TAC => {
                // Writing to TAC can change the clock source, which might cause a falling edge
                let old_bit = self.tima_status();
                self.tac = val;
                let new_bit = self.tima_status();
                let enabled = self.tac.get_bit(TAC_ENABLE_BIT);
                
                if enabled && old_bit && !new_bit && self.tima_cooldown == 0 {
                    let (new_tima, overflow) = self.tima.overflowing_add(1);
                    self.tima = new_tima;
                    if overflow {
                        self.tima_cooldown = TIMA_COOLDOWN_OVERFLOW;
                    }
                }
            }
            _ => unreachable!("Trying to write a non-timer register"),
        }
    }

    pub fn tick(&mut self, m_cycles: u8) -> bool {
        let mut interrupt = false;
        let t_cycles = 4 * m_cycles;
        let _period = self.get_tima_period();

        for _ in 0..t_cycles {
            let old_bit = self.tima_status();

            self.div_counter = self.div_counter.wrapping_add(1);

            let new_bit = self.tima_status();
            let enabled = self.tac.get_bit(TAC_ENABLE_BIT);

            if self.tima_cooldown != 0 {
                self.tima_cooldown -= 1;

                if self.tima_cooldown == 0 {
                    self.tima = self.tma;
                    interrupt = true;
                }
            } else if enabled && old_bit && !new_bit {
                let (new_tima, overflow) = self.tima.overflowing_add(1);
                self.tima = new_tima;

                if overflow {
                    self.tima_cooldown = TIMA_COOLDOWN_OVERFLOW;
                }
            }
        }

        interrupt
    }

    fn get_tima_period(&self) -> u16 {
        match self.tac & 0b11 {
            0b00 => 1 << 9,
            0b01 => 1 << 3,
            0b10 => 1 << 5,
            0b11 => 1 << 7,
            _ => unreachable!(),
        }
    }

    fn tima_status(&self) -> bool {
        (self.div_counter & self.get_tima_period()) != 0
    }
}