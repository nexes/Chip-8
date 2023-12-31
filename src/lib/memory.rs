// Chip-8 memory is 4096 bytes, byte addressable from 0x000 to 0xFFF inclusive.
// The programs (ROM) will start at location 0x200
// Memory address are 12 bits wide, giving Chip-8 2^12 (4096) memory address
// The stack is an array of 16 16bit values used to store return address for subroutines
pub struct Memory {
    rom_location: u16,
    ram: [u8; 0x1000],
    vram: [u8; 64 * 32],
    stack: [u16; 16],
    sp: usize,
}

impl Memory {
    pub(crate) fn allocate() -> Memory {
        let mut mem = Memory {
            rom_location: 0x200,
            ram: [0; 0x1000],
            vram: [0; 64 * 32],
            stack: [0; 16],
            sp: 0,
        };

        // load the static font starting at memory location 0x000
        mem.write_font_data();
        mem
    }

    // write the data read from the rom file and load it into the stack starting
    // at memory location 0x200.
    pub(crate) fn write_rom_data(&mut self, data: Vec<u8>) {
        for offset in 0..data.len() {
            self.ram[self.rom_location as usize + offset] = data[offset];
        }
    }

    // font data is static and loaded into the stack staring at memory
    // location 0x000. Chip-8 had a base 16 keyboard 0-9, A-F keys.
    pub(crate) fn write_font_data(&mut self) {
        let fonts = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for i in 0..fonts.len() {
            self.ram[i] = fonts[i];
        }
    }

    // write a single byte to memory location
    pub(crate) fn write_byte(&mut self, location: u16, val: u8) -> Result<(), String> {
        if location >= 0x1000 {
            return Err(stringify!("Invalid write memory location: {}", loc).to_string());
        }

        self.ram[location as usize] = val;
        Ok(())
    }

    // clears the data from vram. Usually done because of the clear instruction
    pub(crate) fn clear_vram(&mut self) {
        for i in 0..self.vram.len() {
            self.vram[i] = 0;
        }
    }

    // loc is the memory address likely taken from the PC register.
    // If the location is greater than 0xFFF and error is returned.
    // Chip-8 instructions are 2 bytes long stored in big-endian
    pub(crate) fn read_word(&self, loc: u16) -> Result<u16, String> {
        if loc > 0xFFF {
            return Err(stringify!("Invalid read memory location: {}", loc).to_string());
        }

        let msb: u16 = self.ram[loc as usize] as u16;
        let lsb: u16 = self.ram[loc as usize + 1] as u16;
        let data: u16 = (msb << 8) | lsb;

        Ok(data)
    }

    // read a single 8 bit value from memory location loc
    pub(crate) fn read_byte(&self, loc: u16) -> Result<u8, String> {
        if loc > 0xFFF {
            return Err(stringify!("Invalid memory location: {}", loc).to_string());
        }

        Ok(self.ram[loc as usize])
    }

    // read 'count' number of bytes out of memory starting from location 'loc'
    pub(crate) fn read_n_bytes(&self, count: usize, loc: usize) -> Result<Vec<u8>, String> {
        if loc + count >= self.ram.len() {
            return Err(stringify!(
                "Read memory out of bounds. Location: {}, Offset: {}",
                loc,
                count
            )
            .to_string());
        }

        let mut mem = Vec::new();
        for i in 0..count {
            mem.push(self.ram[loc + i]);
        }
        Ok(mem)
    }

    // get a copy of the contents of vram pixels
    pub(crate) fn get_vram(&mut self) -> &mut [u8; 32 * 64] {
        &mut self.vram
    }

    // remove and return the value on top of the stack
    pub(crate) fn pop_stack(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp]
    }

    // push a value to the top of the stack
    pub(crate) fn push_stack(&mut self, val: u16) {
        self.stack[self.sp] = val;
        self.sp += 1;
    }

    // print the contents of the stack
    pub(crate) fn print_stack(&self) {}

    // print the contents of the stack from 0x000 to 0xFFF inclusive
    pub(crate) fn print_memory(&self) {
        let mut addr: u16 = 0x000;

        print!("{:#09x}  ", addr);
        for i in 0..self.ram.len() {
            if i > 0 && i % 8 == 0 {
                addr += 8;
                print!("\n{:#09x}  ", addr);
            }

            print!("{:#04x} ", self.ram[i]);
        }
        println!("");
    }
}
