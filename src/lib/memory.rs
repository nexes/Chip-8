pub struct Memory {
    rom_location: u16,
    stack: Vec<u8>,
}

impl Memory {
    pub fn allocate() -> Memory {
        let mut mem = Memory {
            // Chip-8 programs will start at location 0x200
            rom_location: 0x200,

            // stack size is 4096 bytes with addressable memory from
            // 0x000 to 0xFFF inclusive with a WORD size of 12 bits
            stack: vec![0; 0x1000],
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
