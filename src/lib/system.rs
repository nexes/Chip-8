use crate::Memory;
use crate::CPU;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct System {
    cpu: CPU,
    mem: Memory,
}

impl System {
    pub fn create(cpu: CPU, mem: Memory) -> System {
        System { cpu, mem }
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        match File::open(path) {
            Ok(mut f) => {
                let mut data: Vec<u8> = Vec::new();
                f.read_to_end(&mut data).map_err(|e| e.to_string())?;

                // write the data from the file into memory
                self.mem.write_rom_data(data);
                self.mem.print_stack();
                Ok(())
            }
            Err(_) => Err(String::from("Failed to open ROM file")),
        }
    }

    pub fn run(&self) {}
}
