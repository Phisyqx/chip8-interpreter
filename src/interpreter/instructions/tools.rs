// Tools
pub fn nibbles(n: u16) -> (u8, u8, u8, u8) {
    let n3 = (n >> 12) as u8;
    let n2 = ((n >> 8) & 0b1111) as u8;
    let n1 = ((n >> 4) & 0b1111) as u8;
    let n0 = (n & 0b1111) as u8;
    (n3, n2, n1, n0)
}


pub fn twelvebit(n: u16) -> u16 {
    n & 0xfff
}


pub fn eightbit(n: u16) -> u8 {
    (n & 0xff) as u8
}