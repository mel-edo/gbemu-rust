use crate::utils::BitOps;

pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

const TAC_ENABLE_BIT: u8 = 2;

#[derive(PartialEq)]
enum TimerState {
    Normal,
    OverflowCycleA,  // TIMA=00, waiting for cycle B (1 M-cycle)
    OverflowCycleB,  // Loading TMA, setting IF
}

pub struct Timer {
    sys_counter: u32,      // System counter (increments every T-cycle = 4.19 MHz)
    tima: u8,
    tma: u8,
    tac: u8,
    state: TimerState,
    overflow_pending_tma: u8,
    is_cgb: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            sys_counter: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            state: TimerState::Normal,
            overflow_pending_tma: 0,
            is_cgb: false,
        }
    }

    pub fn set_cgb(&mut self, is_cgb: bool) {
        self.is_cgb = is_cgb;
    }

    pub fn read_timer(&self, addr: u16) -> u8 {
        match addr {
            DIV => (self.sys_counter >> 8) as u8,
            TIMA => self.tima,
            TMA => self.tma,
            TAC => self.tac,
            _ => unreachable!("Trying to read a non-timer register"),
        }
    }

    pub fn write_timer(&mut self, addr: u16, val: u8) {
        match addr {
            DIV => {
                // Writing to DIV resets the system counter to 0.
                // If the TIMA clock bit was 1 and becomes 0, this is a falling edge
                // that should increment TIMA (if timer is enabled).
                let old_bit = self.tima_status();
                self.sys_counter = 0;
                let new_bit = self.tima_status();
                let enabled = self.tac.get_bit(TAC_ENABLE_BIT);
                
                if enabled && old_bit && !new_bit && self.state == TimerState::Normal {
                    self.increment_tima();
                }
                
                // Also reset overflow state if in progress
                if self.state != TimerState::Normal {
                    self.state = TimerState::Normal;
                }
            }
            TIMA => {
                // Writing to TIMA during OverflowCycleA blocks the reload
                // Writing to TIMA during OverflowCycleB is ignored
                if self.state == TimerState::OverflowCycleA {
                    // Block the reload - stay in Normal state with new value
                    self.tima = val;
                    self.state = TimerState::Normal;
                } else if self.state == TimerState::OverflowCycleB {
                    // Ignored - TIMA will be overwritten with TMA anyway
                } else {
                    self.tima = val;
                }
            }
            TMA => {
                self.tma = val;
                // If in OverflowCycleB, the new TMA value is loaded immediately
                if self.state == TimerState::OverflowCycleB {
                    self.tima = val;
                    self.overflow_pending_tma = val;
                }
            }
            TAC => {
                // Writing to TAC can change the clock source, which might cause a falling edge
                let old_bit = self.tima_status();
                let old_enabled = self.tac.get_bit(TAC_ENABLE_BIT);
                self.tac = val;
                let new_bit = self.tima_status();
                let enabled = self.tac.get_bit(TAC_ENABLE_BIT);
                
                // DMG-specific: disabling timer when selected bit is 1 triggers TIMA tick
                if !self.is_cgb && old_enabled && !enabled && old_bit && self.state == TimerState::Normal {
                    self.increment_tima();
                }
                
                // Clock source change from 1->0 triggers falling edge
                if enabled && old_bit && !new_bit && self.state == TimerState::Normal {
                    self.increment_tima();
                }
            }
            _ => unreachable!("Trying to write a non-timer register"),
        }
    }

    fn increment_tima(&mut self) {
        let (new_tima, overflow) = self.tima.overflowing_add(1);
        self.tima = new_tima;

        if overflow {
            // Enter overflow sequence: Cycle A (TIMA=00)
            self.tima = 0x00;
            self.overflow_pending_tma = self.tma;
            self.state = TimerState::OverflowCycleA;
        }
    }

    pub fn tick(&mut self, m_cycles: u8) -> bool {
        let mut interrupt = false;
        let t_cycles = 4 * m_cycles as u32;

        for _ in 0..t_cycles {
            // Handle overflow state machine (advances once per M-cycle = 4 T-cycles)
            if (self.sys_counter & 3) == 0 {
                match self.state {
                    TimerState::OverflowCycleA => {
                        // Cycle A: TIMA=00, transition to Cycle B
                        self.state = TimerState::OverflowCycleB;
                    }
                    TimerState::OverflowCycleB => {
                        // Cycle B: Load TMA, set IF
                        self.tima = self.overflow_pending_tma;
                        interrupt = true;
                        self.state = TimerState::Normal;
                    }
                    TimerState::Normal => {}
                }
            }

            let old_bit = self.tima_status();

            // Increment system counter (once per T-cycle)
            self.sys_counter = self.sys_counter.wrapping_add(1);

            let new_bit = self.tima_status();
            let enabled = self.tac.get_bit(TAC_ENABLE_BIT);

            if self.state == TimerState::Normal && enabled && old_bit && !new_bit {
                self.increment_tima();
            }
        }

        interrupt
    }

    fn get_tima_period(&self) -> u32 {
        match self.tac & 0b11 {
            0b00 => 1 << 9,   // bit 9: 1024 T-cycles period (falling edge every 1024 T-cycles)
            0b01 => 1 << 3,   // bit 3: 16 T-cycles period
            0b10 => 1 << 5,   // bit 5: 64 T-cycles period
            0b11 => 1 << 7,   // bit 7: 256 T-cycles period
            _ => unreachable!(),
        }
    }

    fn tima_status(&self) -> bool {
        (self.sys_counter & self.get_tima_period()) != 0
    }
}