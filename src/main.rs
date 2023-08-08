use emulator::{Display, System};

fn main() -> Result<(), String> {
    let display = Display::create("Chip-8".to_string(), 640, 380);
    let mut system = System::create(display);

    system.load_rom("test_opcode.ch8")?;
    system.run()
}
