mod interpreter;
use env_logger;
fn main() {
    env_logger::init();
    let hello = interpreter::ChipEight::new(300);
    chip8_base::run(hello);
}
