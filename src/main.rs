mod interpreter;
fn main() {
    let hello = interpreter::ChipEight::new(700);
    chip8_base::run(hello);
}
