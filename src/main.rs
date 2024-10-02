mod interpreter;
use env_logger;
use clap::Parser;
fn main() {
    env_logger::init();
    let args = Args::parse();

    let filename: &str = &args.rom;
    let cpu = interpreter::ChipEight::new(5)
        .load(filename)
        .unwrap_or_else(|_| panic!("Could not load ROM: {}", filename));
    
    chip8_base::run(cpu);
}
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    rom: String,
}

// fn rom_exists(f: &str) -> Result<(), &'static str> {
//     let p = std::path::Path::new(f);
//     if !p.is_file() {
//             Err("File does not exist")
//     } else {
//         Ok(())
//     }
// }