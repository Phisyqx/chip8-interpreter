pub mod tools;
use tools::*;

type Reg = u8;
type Add = u16;
#[derive(Debug)] // Deprecate once used
pub enum Instruction{
    Cls, // 00E0
    Nop, //0nnn
    Jmp(Add), // 1nnn
    Setr(Reg, u8), //6xkk
    Addr(Reg, u8), //7xkk
    Seti(u16), //Annn
    Draw(Reg, Reg, u8), //Dxyn
}

use Instruction::*;
pub fn decode(opcode: u16) -> Instruction {
    let nnn = twelvebit(opcode);
    let kk = eightbit(opcode);

    match nibbles(opcode) {
        (0, 0, 0xE, 0) => Cls,
        (0, _, _, _) => Nop,
        (1, _, _, _) => Jmp(nnn),
        (6, x, _, _) => Setr(x, kk),
        (7, x, _, _) => Addr(x, kk),
        (0xA, _, _, _) => Seti(nnn),
        (0xD, x, y, n) => Draw(x, y, n),
        _ => panic!("Invalid instruction encountered: {:#06X}", opcode),

    }
}