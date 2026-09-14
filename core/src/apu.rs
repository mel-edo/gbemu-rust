use crate::utils::BitOps;

pub const APU_START: u16 = 0xFF10;
pub const APU_STOP: u16 = 0xFF3F;

const DUTY_TABLE: [[u8; 8]; 4] = [
    [0,0,0,0,0,0,0,1], // 12.5%
    [1,0,0,0,0,0,0,1], // 25%
    [1,0,0,0,0,1,1,1], // 50%
    [0,1,1,1,1,1,1,0], // 75%
];

pub struct PulseChannel {
    pub enabled: bool,
    pub channel_dac: bool,
    duty: u8,
    duty_step: u8,
    
    length_timer: u8,
    length_enable: bool,
    
    timer: u16,
    freq: u16,
    
    vol: u8,
    env_vol: u8,
    env_dir: i8,
    env_period: u8,
    env_timer: u8,
    
    // Sweep (Channel 1 only)
    sweep_timer: u8,
    sweep_period: u8,
    sweep_negate: bool,
    sweep_shift: u8,
    sweep_enabled: bool,
    shadow_freq: u16,
}

impl PulseChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            channel_dac: false,
            duty: 0,
            duty_step: 0,
            length_timer: 0,
            length_enable: false,
            timer: 0,
            freq: 0,
            vol: 0,
            env_vol: 0,
            env_dir: 0,
            env_period: 0,
            env_timer: 0,
            sweep_timer: 0,
            sweep_period: 0,
            sweep_negate: false,
            sweep_shift: 0,
            sweep_enabled: false,
            shadow_freq: 0,
        }
    }
    
    pub fn step_timer(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            self.timer = (2048 - self.freq) * 4;
            self.duty_step = (self.duty_step + 1) & 7;
        }
    }

    pub fn step_length(&mut self) {
        if self.length_enable && self.length_timer > 0 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn step_env(&mut self) {
        if self.env_period > 0 {
            if self.env_timer > 0 {
                self.env_timer -= 1;
            }
            if self.env_timer == 0 {
                self.env_timer = self.env_period;
                let new_vol = self.env_vol as i8 + self.env_dir;
                if new_vol >= 0 && new_vol <= 15 {
                    self.env_vol = new_vol as u8;
                }
            }
        }
    }

    fn calculate_sweep_freq(&self) -> u16 {
        let offset = self.shadow_freq >> self.sweep_shift;
        if self.sweep_negate {
            self.shadow_freq.wrapping_sub(offset)
        } else {
            self.shadow_freq.wrapping_add(offset)
        }
    }

    pub fn step_sweep(&mut self) {
        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }
        
        if self.sweep_timer == 0 {
            self.sweep_timer = if self.sweep_period == 0 { 8 } else { self.sweep_period };
            
            if self.sweep_enabled && self.sweep_period > 0 {
                let new_freq = self.calculate_sweep_freq();
                if new_freq <= 2047 && self.sweep_shift > 0 {
                    self.shadow_freq = new_freq;
                    self.freq = new_freq;
                    
                    // Run overflow check again
                    if self.calculate_sweep_freq() > 2047 {
                        self.enabled = false;
                    }
                } else if new_freq > 2047 {
                    self.enabled = false;
                }
            }
        }
    }

    pub fn trigger(&mut self) {
        self.enabled = true;
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        self.timer = (2048 - self.freq) * 4;
        self.env_timer = self.env_period;
        self.env_vol = self.vol;
        
        self.shadow_freq = self.freq;
        self.sweep_timer = if self.sweep_period == 0 { 8 } else { self.sweep_period };
        self.sweep_enabled = self.sweep_period > 0 || self.sweep_shift > 0;
        
        if self.sweep_shift > 0 && self.calculate_sweep_freq() > 2047 {
            self.enabled = false;
        }

        if !self.channel_dac {
            self.enabled = false;
        }
    }

    pub fn get_amplitude(&self) -> u8 {
        if self.enabled && self.channel_dac {
            DUTY_TABLE[self.duty as usize][self.duty_step as usize] * self.env_vol
        } else {
            0
        }
    }
}

pub struct WaveChannel {
    pub enabled: bool,
    pub channel_dac: bool,
    
    length_timer: u16,
    length_enable: bool,
    
    timer: u16,
    freq: u16,
    
    vol_code: u8,
    position: u8,
    sample_buffer: u8,
}

impl WaveChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            channel_dac: false,
            length_timer: 0,
            length_enable: false,
            timer: 0,
            freq: 0,
            vol_code: 0,
            position: 0,
            sample_buffer: 0,
        }
    }

    pub fn step_timer(&mut self, ram: &[u8]) {
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            self.timer = (2048 - self.freq) * 2;
            self.position = (self.position + 1) & 31;
            
            // Read sample from Wave RAM (ram offset 0x20)
            let byte_idx = self.position / 2;
            let byte = ram[0x20 + byte_idx as usize];
            if self.position % 2 == 0 {
                self.sample_buffer = byte >> 4;
            } else {
                self.sample_buffer = byte & 0x0F;
            }
        }
    }

    pub fn step_length(&mut self) {
        if self.length_enable && self.length_timer > 0 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn trigger(&mut self) {
        self.enabled = true;
        if self.length_timer == 0 {
            self.length_timer = 256;
        }
        self.timer = (2048 - self.freq) * 2;
        self.position = 0;
        if !self.channel_dac {
            self.enabled = false;
        }
    }

    pub fn get_amplitude(&self) -> u8 {
        if self.enabled && self.channel_dac {
            match self.vol_code {
                0 => 0,
                1 => self.sample_buffer,
                2 => self.sample_buffer >> 1,
                3 => self.sample_buffer >> 2,
                _ => 0,
            }
        } else {
            0
        }
    }
}

pub struct NoiseChannel {
    pub enabled: bool,
    pub channel_dac: bool,
    
    length_timer: u8,
    length_enable: bool,
    
    vol: u8,
    env_vol: u8,
    env_dir: i8,
    env_period: u8,
    env_timer: u8,
    
    timer: u32,
    lfsr: u16,
    
    shift_clock: u8,
    step_width: bool,
    div_ratio: u8,
}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            channel_dac: false,
            length_timer: 0,
            length_enable: false,
            vol: 0,
            env_vol: 0,
            env_dir: 0,
            env_period: 0,
            env_timer: 0,
            timer: 0,
            lfsr: 0x7FFF,
            shift_clock: 0,
            step_width: false,
            div_ratio: 0,
        }
    }

    pub fn step_timer(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            let divisor = if self.div_ratio == 0 { 8 } else { (self.div_ratio as u32) * 16 };
            self.timer = divisor << self.shift_clock;
            
            let xor_bit = (self.lfsr & 1) ^ ((self.lfsr >> 1) & 1);
            self.lfsr = (self.lfsr >> 1) | (xor_bit << 14);
            if self.step_width {
                self.lfsr = (self.lfsr & !(1 << 6)) | (xor_bit << 6);
            }
        }
    }

    pub fn step_length(&mut self) {
        if self.length_enable && self.length_timer > 0 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn step_env(&mut self) {
        if self.env_period > 0 {
            if self.env_timer > 0 {
                self.env_timer -= 1;
            }
            if self.env_timer == 0 {
                self.env_timer = self.env_period;
                let new_vol = self.env_vol as i8 + self.env_dir;
                if new_vol >= 0 && new_vol <= 15 {
                    self.env_vol = new_vol as u8;
                }
            }
        }
    }

    pub fn trigger(&mut self) {
        self.enabled = true;
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        let divisor = if self.div_ratio == 0 { 8 } else { (self.div_ratio as u32) * 16 };
        self.timer = divisor << self.shift_clock;
        self.lfsr = 0x7FFF;
        self.env_timer = self.env_period;
        self.env_vol = self.vol;
        if !self.channel_dac {
            self.enabled = false;
        }
    }

    pub fn get_amplitude(&self) -> u8 {
        if self.enabled && self.channel_dac && (self.lfsr & 1 == 0) {
            self.env_vol
        } else {
            0
        }
    }
}

pub struct Apu {
    ram: [u8; (APU_STOP - APU_START + 1) as usize],
    
    // Channels
    pub ch1: PulseChannel,
    pub ch2: PulseChannel,
    pub ch3: WaveChannel,
    pub ch4: NoiseChannel,
    
    // Master controls
    pub master_en: bool,
    pub left_vol: u8,
    pub right_vol: u8,
    pub pan: u8, // NR51
    
    // Timing
    pub audio_buffer: Vec<f32>,
    cycles: u32,
    sample_rate: u32,
    cpu_freq: u32,
    
    // Frame Sequencer
    fs_timer: u32,
    fs_step: u8,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            ram: [0; (APU_STOP - APU_START + 1) as usize],
            ch1: PulseChannel::new(),
            ch2: PulseChannel::new(),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),
            master_en: false,
            left_vol: 7,
            right_vol: 7,
            pan: 0xFF,
            audio_buffer: Vec::new(),
            cycles: 0,
            sample_rate: 44100, // standard sample rate
            cpu_freq: 4194304,
            fs_timer: 0,
            fs_step: 0,
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        if addr >= APU_START && addr <= APU_STOP {
            // Some registers have unreadable bits set to 1
            let mask = match addr {
                0xFF10 => 0x80,
                0xFF11 | 0xFF16 | 0xFF20 => 0x3F,
                0xFF12 | 0xFF17 | 0xFF21 | 0xFF22 => 0x00,
                0xFF13 | 0xFF18 | 0xFF1D | 0xFF2D => 0xFF,
                0xFF14 | 0xFF19 | 0xFF1E | 0xFF23 => 0xBF,
                0xFF1A | 0xFF1B | 0xFF1C => 0xFF,
                0xFF24 | 0xFF25 => 0x00,
                0xFF26 => 0x70,
                0xFF30..=0xFF3F => 0x00, // Wave RAM
                _ => 0xFF,
            };
            
            if addr == 0xFF26 {
                let mut res = 0x70;
                if self.master_en { res |= 0x80; }
                if self.ch1.enabled { res |= 0x01; }
                if self.ch2.enabled { res |= 0x02; }
                if self.ch3.enabled { res |= 0x04; }
                if self.ch4.enabled { res |= 0x08; }
                return res | (self.ram[(addr - APU_START) as usize] & !mask);
            }
            
            self.ram[(addr - APU_START) as usize] | mask
        } else {
            0xFF
        }
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        if addr == 0xFF26 {
            self.master_en = val.get_bit(7);
            if !self.master_en {
                // power off resets everything
                for i in 0xFF10..=0xFF25 {
                    self.ram[(i - APU_START) as usize] = 0;
                }
                self.ch1 = PulseChannel::new();
                self.ch2 = PulseChannel::new();
                self.ch3 = WaveChannel::new();
                self.ch4 = NoiseChannel::new();
                self.fs_timer = 0;
                self.fs_step = 0;
            }
            self.ram[(addr - APU_START) as usize] = val;
            return;
        }
        if !self.master_en {
            return;
        }
        
        if addr >= APU_START && addr <= APU_STOP {
            self.ram[(addr - APU_START) as usize] = val;
            
            match addr {
                // Channel 1
                0xFF10 => {
                    self.ch1.sweep_period = (val >> 4) & 7;
                    self.ch1.sweep_negate = val.get_bit(3);
                    self.ch1.sweep_shift = val & 7;
                },
                0xFF11 => {
                    self.ch1.duty = val >> 6;
                    self.ch1.length_timer = 64 - (val & 0x3F);
                },
                0xFF12 => {
                    self.ch1.vol = val >> 4;
                    self.ch1.env_dir = if val.get_bit(3) { 1 } else { -1 };
                    self.ch1.env_period = val & 7;
                    self.ch1.channel_dac = val & 0xF8 != 0;
                    if !self.ch1.channel_dac { self.ch1.enabled = false; }
                },
                0xFF13 => {
                    self.ch1.freq = (self.ch1.freq & 0x700) | val as u16;
                },
                0xFF14 => {
                    self.ch1.freq = (self.ch1.freq & 0xFF) | (((val & 7) as u16) << 8);
                    self.ch1.length_enable = val.get_bit(6);
                    if val.get_bit(7) {
                        self.ch1.trigger();
                    }
                },
                
                // Channel 2
                0xFF16 => {
                    self.ch2.duty = val >> 6;
                    self.ch2.length_timer = 64 - (val & 0x3F);
                },
                0xFF17 => {
                    self.ch2.vol = val >> 4;
                    self.ch2.env_dir = if val.get_bit(3) { 1 } else { -1 };
                    self.ch2.env_period = val & 7;
                    self.ch2.channel_dac = val & 0xF8 != 0;
                    if !self.ch2.channel_dac { self.ch2.enabled = false; }
                },
                0xFF18 => {
                    self.ch2.freq = (self.ch2.freq & 0x700) | val as u16;
                },
                0xFF19 => {
                    self.ch2.freq = (self.ch2.freq & 0xFF) | (((val & 7) as u16) << 8);
                    self.ch2.length_enable = val.get_bit(6);
                    if val.get_bit(7) {
                        self.ch2.trigger();
                    }
                },
                
                // Channel 3
                0xFF1A => {
                    self.ch3.channel_dac = val.get_bit(7);
                    if !self.ch3.channel_dac { self.ch3.enabled = false; }
                },
                0xFF1B => {
                    self.ch3.length_timer = 256 - (val as u16);
                },
                0xFF1C => {
                    self.ch3.vol_code = (val >> 5) & 3;
                },
                0xFF1D => {
                    self.ch3.freq = (self.ch3.freq & 0x700) | val as u16;
                },
                0xFF1E => {
                    self.ch3.freq = (self.ch3.freq & 0xFF) | (((val & 7) as u16) << 8);
                    self.ch3.length_enable = val.get_bit(6);
                    if val.get_bit(7) {
                        self.ch3.trigger();
                    }
                },
                
                // Channel 4
                0xFF20 => {
                    self.ch4.length_timer = 64 - (val & 0x3F);
                },
                0xFF21 => {
                    self.ch4.vol = val >> 4;
                    self.ch4.env_dir = if val.get_bit(3) { 1 } else { -1 };
                    self.ch4.env_period = val & 7;
                    self.ch4.channel_dac = val & 0xF8 != 0;
                    if !self.ch4.channel_dac { self.ch4.enabled = false; }
                },
                0xFF22 => {
                    self.ch4.shift_clock = val >> 4;
                    self.ch4.step_width = val.get_bit(3);
                    self.ch4.div_ratio = val & 7;
                },
                0xFF23 => {
                    self.ch4.length_enable = val.get_bit(6);
                    if val.get_bit(7) {
                        self.ch4.trigger();
                    }
                },
                
                // Master controls
                0xFF24 => {
                    self.left_vol = (val >> 4) & 7;
                    self.right_vol = val & 7;
                },
                0xFF25 => {
                    self.pan = val;
                },
                _ => {}
            }
        }
    }

    pub fn tick(&mut self, cycles: u8) {
        if !self.master_en {
            return;
        }
        
        let cycles_u32 = cycles as u32;
        
        // Frame sequencer (8192 cycles = 512Hz)
        self.fs_timer += cycles_u32;
        while self.fs_timer >= 8192 {
            self.fs_timer -= 8192;
            
            // Step length (256Hz: steps 0, 2, 4, 6)
            if self.fs_step % 2 == 0 {
                self.ch1.step_length();
                self.ch2.step_length();
                self.ch3.step_length();
                self.ch4.step_length();
            }
            
            // Step sweep (128Hz: steps 2, 6)
            if self.fs_step == 2 || self.fs_step == 6 {
                self.ch1.step_sweep();
            }
            
            // Step volume envelope (64Hz: step 7)
            if self.fs_step == 7 {
                self.ch1.step_env();
                self.ch2.step_env();
                self.ch4.step_env();
            }
            
            self.fs_step = (self.fs_step + 1) & 7;
        }
        
        // Channel timers
        for _ in 0..cycles_u32 {
            self.ch1.step_timer();
            self.ch2.step_timer();
            self.ch3.step_timer(&self.ram);
            self.ch4.step_timer();
        }

        self.cycles += cycles_u32;
        
        let cycles_per_sample = self.cpu_freq / self.sample_rate;

        // Generate audio samples based on cycles elapsed
        while self.cycles >= cycles_per_sample {
            self.cycles -= cycles_per_sample;
            
            // Mix channels
            let amp1 = self.ch1.get_amplitude() as f32 / 15.0; // 0.0 to 1.0
            let amp2 = self.ch2.get_amplitude() as f32 / 15.0; // 0.0 to 1.0
            let amp3 = self.ch3.get_amplitude() as f32 / 15.0; // 0.0 to 1.0
            let amp4 = self.ch4.get_amplitude() as f32 / 15.0; // 0.0 to 1.0
            
            let mut left = 0.0;
            let mut right = 0.0;
            
            // Panning NR51
            if self.pan.get_bit(4) { left += amp1; }
            if self.pan.get_bit(0) { right += amp1; }
            if self.pan.get_bit(5) { left += amp2; }
            if self.pan.get_bit(1) { right += amp2; }
            if self.pan.get_bit(6) { left += amp3; }
            if self.pan.get_bit(2) { right += amp3; }
            if self.pan.get_bit(7) { left += amp4; }
            if self.pan.get_bit(3) { right += amp4; }
            
            // Mix down (divide by total possible channels to prevent clipping, though realistically 4 channels)
            left /= 4.0;
            right /= 4.0;
            
            // Apply master volume NR50 (0-7)
            let vol_l = (self.left_vol as f32 + 1.0) / 8.0;
            let vol_r = (self.right_vol as f32 + 1.0) / 8.0;
            
            self.audio_buffer.push(left * vol_l);
            self.audio_buffer.push(right * vol_r);
        }
    }
}
