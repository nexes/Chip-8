use emulator::{Memory, System, CPU};

fn main() {
    let mem = Memory::allocate();
    let cpu = CPU::init();
    let mut sys = System::create(cpu, mem);

    sys.load_rom("test_opcode.ch8").expect("Loaded ROM failed");
    sys.run();
}
