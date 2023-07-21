// Chip-8 memory is 4096 bytes, byte addressable from 0x000 to 0xFFF inclusive.
// The programs (ROM) will start at location 0x200
// Memory address are 12 bits wide, giving Chip-8 2^12 (4096) memory address
pub struct Memory {
    rom_location: u16,
    stack: [u8; 0x1000],
}

impl Memory {
    pub fn allocate() -> Memory {
        let mut mem = Memory {
            rom_location: 0x200,
            stack: [0; 0x1000],
        };

        // load the static font starting at memory location 0x000
        mem.write_font_data();
        mem
    }

    // write the data read from the rom file and load it into the stack starting
    // at memory location 0x200.
    pub(crate) fn write_rom_data(&mut self, data: Vec<u8>) {
        for offset in 0..data.len() {
            self.stack[self.rom_location as usize + offset] = data[offset];
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
            self.stack[i] = fonts[i];
        }
    }

    // loc is the memory address likely taken from the PC register.
    // word length is 12 bits. If the location is greater than
    // 0xFFF and error is returned. Chip-8 instructions are 2 bytes
    // long stored in big-endian
    pub(crate) fn read_word(&self, loc: u16) -> Result<u16, String> {
        if loc > 0xFFF {
            return Err(stringify!("Invalid memory location: {}", loc).to_string());
        }

        let msb: u16 = self.stack[loc as usize] as u16;
        let lsb: u16 = self.stack[loc as usize + 1] as u16;
        let data: u16 = (msb << 8) | lsb;

        Ok(data)
    }

    // read a single 8 bit value from memory location loc
    pub(crate) fn read_byte(&self, loc: u16) -> Result<u8, String> {
        if loc > 0xFFF {
            return Err(stringify!("Invalid memory location: {}", loc).to_string());
        }

        Ok(self.stack[loc as usize])
    }

    // print the contents of the stack from 0x000 to 0xFFF inclusive
    pub(crate) fn print_stack(&self) {
        let mut addr: u16 = 0x000;

        print!("{:#09x}  ", addr);
        for i in 0..self.stack.len() {
            if i > 0 && i % 8 == 0 {
                addr += 8;
                print!("\n{:#09x}  ", addr);
            }

            print!("{:#04x} ", self.stack[i]);
        }
        println!("");
    }
}
