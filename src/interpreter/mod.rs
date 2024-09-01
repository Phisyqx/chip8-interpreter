use std::{iter::TakeWhile, str::FromStr, time::Duration};

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


    fn fetch(&mut self) -> u16 {

        let ins = u16::from_be_bytes([
            self.memory[self.program_counter as usize],
            self.memory[(self.program_counter + 1) as usize]
        ]);
        self.program_counter += 2;
        self.program_counter &= 0x0FFF;
        
        ins
    }

    fn get_nibbles(ins: u16) -> (u8, u8, u8, u8) {
        let n3 = ( ins >> 12) as u8;
        let n2 = ((ins >> 8) & 0b1111) as u8;
        let n1 = ((ins >> 4) & 0b1111) as u8;
        let n0 = ( ins       & 0b1111) as u8;
        (n3, n2, n1, n0)
    }
    fn twelvebits (a: u8, b: u8, c: u8) -> u16 {
        ((a as u16) << 8 + (b as u16) << 4 + c) & 0xFFF
    }
    fn execute(&mut self, ins: u16) {
        match Self::get_nibbles(ins) {
            (0x0, 0x0, 0x0, 0x0) => println!("Nothing"), // NOP
            // 00EE return from subroutine
            (0x0, 0x0, 0xE, 0xE) => {
                self.program_counter = self.stack[self.stack_pointer as usize];
                self.program_counter -= 1
            },
            // 8xy2 AND Vx, Vy, set Vx = Vx AND Vy
            (8, x, y, 2) => self.registers[x as usize] &= self.registers[y as usize],
            (0, 0, 0xE, 0) => self.display = [[chip8_base::Pixel::Black; 64]; 32],
            (6, x, k, n) => self.registers[x as usize] = (k << 4) + n,
            (7, x, k, n) => self.registers[x as usize] = (self.registers[x as usize] + (k << 4) + n) & 0xFF,
            (1, k, n, m) => self.program_counter = (self.registers[0] as u16 + ChipEight::twelvebits(k, n, m)) & 0xFFF,
            (0xA, k, n, m) => self.program_counter = ChipEight::twelvebits(k, n, m),
            _ => panic!("Not implemented")
        }
    }

} 

impl Interpreter for ChipEight {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        let ins = self.fetch();
        self.execute(ins);
        None
    }

    fn speed(&self) -> std::time::Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}
