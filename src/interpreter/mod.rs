use std::time::Duration;
mod instructions;
use chip8_base::{Display, Interpreter, Pixel};
use instructions::{decode, Instruction};
use crate::interpreter::instructions::tools::byte_to_pixels;
#[derive(Debug)]
pub struct ChipEight {
      memory         : [ u8; 4096 ]
    , registers      : [ u8; 16 ]
    , display        : chip8_base::Display
    , program_counter: u16
    , index  : u16
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
            , index          : 0
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

    fn execute(&mut self, ins: Instruction) -> Option<Display> { 
        log::info!("Executing instruction {ins:X?}");
        match ins {
            Instruction::Cls => {
                self.display = [[Pixel::Black; 64]; 32];
                return Some(self.display);
            },
            Instruction::Nop => (),
            Instruction::Jmp(addr) => {
                self.program_counter = addr;
            },
            Instruction::Setr(r, byte) => {
                self.registers[r as usize] = byte;
            },
            Instruction::Addr(r, byte) => {
                self.registers[r as usize] = self.registers[r as usize].wrapping_add(byte)
            },
            Instruction::Seti(nnn) => {
                self.index = nnn;
            },
            Instruction::Draw(rx, ry, n) => {
                let range = (self.index as usize)..((self.index + n as u16) as usize);
                let sprite = &self.memory[range];
                let x = self.registers[rx as usize] % 64;
                let y = self.registers[ry as usize] % 32;
                self.registers[0xF] = 0;
                for (i, row) in sprite.iter().enumerate() {
                    if y + i as u8 > 31 {
                        break
                    }
                    for (j, sprite_px) in (0..8).zip(byte_to_pixels(*row)) {
                        if x + j as u8 > 63 {
                            break;
                        }
                        let display_px = &mut self.display[y as usize + i][x as usize + j];
                        if (*display_px & sprite_px).into() {
                            self.registers[0xF] = 1;
                        }
                        *display_px ^= sprite_px;
                    }
                }
                return Some(self.display);
            },
        };
        None
                
    }

    
    pub fn load(mut self, filename: &str) -> std::io::Result<Self> {
        let program = std::fs::read(filename)?;
        log::info!("Loaded into memery");
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program);
        self.program_counter = 0x200;
        Ok(self)
    }

}


impl Interpreter for ChipEight {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        let opcode = self.fetch();
        let ins = decode(opcode);
        let update = self.execute(ins);
        update
    }
    

    fn speed(&self) -> std::time::Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}
// struct PixIterator {
//     byte: u8,
//     idx: u8,
// }

// impl PixIterator {
//     pub fn new(byte: &u8) -> Self {
//         Self {
//             byte: *byte,
//             idx: 0,
//         }
//     }
// }

// impl Iterator for PixIterator {
//     type Item = Pixel;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.idx < 8 {
//             let bit = self.byte >> (7 - self.idx) & 1;
//             self.idx += 1;
//             assert!(bit == 1 || bit == 0);
//             Some(bit.try_into().unwrap()) //safe to unwrap because we assert
//         } else {
//             None
//         }
//     }
// }