pub struct Memory {
    rom_location: u16,
    stack: Vec<u8>,
}

impl Memory {
    pub fn allocate() -> Memory {
        Memory {
            rom_location: 0x200,
            stack: vec![0; 0xFFF],
        }
    }

    pub(crate) fn load_rom_data(&mut self, data: Vec<u8>) {}
}
