use std::time::Duration;

use chip8_base::{Display, Interpreter, Pixel};
#[derive(Debug)]
pub struct ChipEight {
      memory         : [ u8; 4096 ]
    , registers      : [ u8; 16 ]
    , display        : chip8_base::Display
    , program_counter: u16
    , stack_pointer  : u16
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
    // fn step(&mut self) -> Option<Display> {
    //     let opcode = self.fetch();
    //     let update = self.execute(opcode);
    //     update
    // }

    fn execute(&mut self, ins: u16) -> Option<Display> { 
        log::debug!("Executing instruction {ins:X?}");
        match get_nibbles(ins) {
            
            (0x0, 0x0, 0xE, 0xE) => {
                self.program_counter = self.stack[self.stack_pointer as usize];
                self.program_counter -= 1
            },
            // 8xy2 AND Vx, Vy, set Vx = Vx AND Vy
            (8, x, y, 2) => self.registers[x as usize] &= self.registers[y as usize],

            (0, 0, 0xE, 0) => self.display = [[chip8_base::Pixel::Black; 64]; 32],
            (0x0, _, _, _) => (), // NOP
            (6, x, k, n) => {self.registers[x as usize] = (k << 4) | n;
            log::debug!("Added {} to {x}", ((k << 4) | n).to_string())},
            (7, x, k, n) => self.registers[x as usize] = self.registers[x as usize].wrapping_add(self.registers[x as usize] + (k << 4) + n),
            (1, k, n, m) => self.program_counter = twelvebits(k, n, m),
            (0xA, k, n, m) => self.stack_pointer = twelvebits(k, n, m),
            (0xD, x, y, n) => {
                        self.registers[0xF] = 0;
                        for byte in 0..n {
                            let y = (self.registers[y as usize] as usize + byte as usize) % 32;
                            let sprite_byte = self.memory[self.stack_pointer as usize + byte as usize];

                            for bit in 0..8 {
                                let x = (self.registers[x as usize] as usize + bit) % 64;
                                let sprite_px = (sprite_byte >> (7-bit)) & 1;

                                if self.display[y][x] == Pixel::White && sprite_px == 1 {
                                    self.registers[0xF] = 1;
                                }
                                let display_px = &mut self.display[y as usize + byte as usize][x as usize + bit];
                                let conv_px = match Pixel::try_from(sprite_px) {
                                    Ok(px) => px,
                                    Err(_) => panic!("Tried to convert not a 0 or 1 into a pixel"),
                                };
                                *display_px ^= conv_px;
                            }
                        }
                        
                        
                        return Some(self.display)

                    }
                    _ => panic!("Not implemented")

                };
                None
                
            }
            
            
            //TODO: Refactor instructions into their own enum
            //TODO: currently broken cus program counter is increasing incorrectly but it does fetch decode and execute proopperly!!!!

    
    pub fn load(mut self, filename: &str) -> std::io::Result<Self> {
        let program = std::fs::read(filename)?;
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program);
        self.program_counter = 0x200;
        Ok(self)
    }

}

// fn bits_to_array(n: &u8) -> [u8 ; 8] {
//     [
//         (n >> 7) & 1,
//         (n >> 6) & 1,
//         (n >> 5) & 1,
//         (n >> 4) & 1,
//         (n >> 3) & 1,
//         (n >> 2) & 1,
//         (n >> 1) & 1,
//         n & 1,
//     ]
// }
fn get_nibbles(ins: u16) -> (u8, u8, u8, u8) {
    let n3 = ( ins >> 12) as u8;
    let n2 = ((ins >> 8) & 0b1111) as u8;
    let n1 = ((ins >> 4) & 0b1111) as u8;
    let n0 = ( ins       & 0b1111) as u8;
    (n3, n2, n1, n0)
}
fn twelvebits (a: u8, b: u8, c: u8) -> u16 {
    (a as u16) << 8 | (b as u16) << 4 | (c as u16)
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
