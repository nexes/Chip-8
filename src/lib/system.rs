use crate::display::{Display, Key};
use crate::{Instruction, Memory, CPU};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{thread, time};

pub(crate) struct Flags {
    pub(crate) draw: bool,
    pub(crate) clear: bool,
    pub(crate) sound: bool,
    pub(crate) key: Key,
}

pub struct System {
    cpu: CPU,
    mem: Memory,
    display: Display,
    flags: Flags,
}

impl System {
    pub fn create(display: Display) -> System {
        let mem = Memory::allocate();
        let cpu = CPU::init();

        System {
            cpu,
            mem,
            display,
            flags: Flags {
                draw: false,
                clear: false,
                sound: false,
                key: Key::NONE,
            },
        }
    }

    // read the rom file from disk and load into memory
    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        match File::open(path) {
            Ok(mut f) => {
                let mut data: Vec<u8> = Vec::new();
                f.read_to_end(&mut data).map_err(|e| e.to_string())?;

                // write the data from the file into memory
                self.mem.write_rom_data(data);
                self.mem.print_memory();
                Ok(())
            }
            Err(_) => Err(String::from("Failed to open ROM file")),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        'running: loop {
            let key_press = self.display.user_event()?;

            match key_press {
                Key::QUIT => break 'running,
                _ => self.flags.key = key_press,
            }

            // fetch the 2 byte instruction at memory address held by the PC register
            let mem_addr = self.cpu.register_pc();
            let data = self.mem.read_word(mem_addr)?;

            // decode
            let instr = Instruction::decode(data);

            println!("address: {:#09x}, data = {:#05x}", mem_addr, data);
            println!("{}", instr);

            // execute
            self.cpu.execute(instr, &mut self.flags, &mut self.mem)?;

            if self.flags.clear {
                self.flags.clear = false;
                self.display.clear();
            }

            if self.flags.draw {
                self.flags.draw = false;
                self.display.draw(self.mem.get_vram());
            }

            self.cpu.tick_timer();
            thread::sleep(time::Duration::from_millis(20));
        }

        Ok(())
    }
}
