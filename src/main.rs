use emulator::{Display, System};

fn main() -> Result<(), String> {
    let display = Display::create("Chip-8".to_string(), 10);
    let mut system = System::create(display);

    system.load_rom("IBM_Logo.ch8")?;
    system.run()
}
