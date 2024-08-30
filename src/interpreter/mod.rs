use std::time::Duration;

use chip8_base::Interpreter;

pub struct ChipEight {
      memory         : [ u8; 4096 ]
    , registers      : [ u8; 16 ]
    , display        : chip8_base::Display
    , program_counter: u16
    , stack_pointer  : u8
    , stack          : [ u16; 16 ]
    , speed          : Duration
}

impl ChipEight {
    pub fn new(freq: u32) -> Self {
        ChipEight {
              memory         : [0; 4096]
            , registers      : [0; 16]
            , program_counter: 0x200
            , display        : [[chip8_base::Pixel::default(); 64]; 32]
            , stack_pointer  : 0
            , stack          : [0; 16]
            , speed          : Duration::from_secs_f64(1_f64 / freq as f64)
        }        
        
    }

} 

impl Interpreter for ChipEight {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        return None
    }

    fn speed(&self) -> std::time::Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        return false
    }
}