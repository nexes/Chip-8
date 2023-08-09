use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::system::Flags;
use rand::{thread_rng, Rng};

// Chip-8 instructions are 2 bytes long
pub struct CPU {
    // delay timer decrements to zero at a rate of 60Hz
    dt: u8,
    // sound timer decrements to zero at a rate of 60Hz
    st: u8,
    // this register is used to store memory addresses (lowest 12 bits)
    i: u16,
    // program counter register stores the currently executing address
    pc: u16,
    // stack pointer register stores the top most stack address
    sp: u8,
    // gerneral purpose registers V[x] from (0 <= x <= F)
    reg: [u8; 16],
}

impl CPU {
    pub(crate) fn init() -> CPU {
        CPU {
            dt: 60,
            st: 60,
            i: 0,
            pc: 0x200,
            sp: 0,
            reg: [0; 16],
        }
    }

    // returns the memory address held by the PC register.
    // the program counter is incremented by two bytes
    pub(crate) fn register_pc(&mut self) -> u16 {
        let loc = self.pc;
        self.pc += 0x02;
        loc
    }

    pub(crate) fn execute(
        &mut self,
        instr: Instruction,
        flags: &mut Flags,
        mem: &mut Memory,
    ) -> Result<(), String> {
        match instr.itype() {
            0x0 => self.opcode_0(instr, flags, mem),
            0x1 => self.opcode_1(instr),
            0x2 => self.opcode_2(instr, mem),
            0x3 => self.opcode_3(instr),
            0x4 => self.opcode_4(instr),
            0x5 => self.opcode_5(instr),
            0x6 => self.opcode_6(instr),
            0x7 => self.opcode_7(instr),
            0x8 => self.opcode_8(instr),
            0x9 => self.opcode_9(instr),
            0xA => self.opcode_A(instr),
            0xB => self.opcode_B(instr),
            0xC => self.opcode_C(instr),
            0xD => self.opcode_D(instr, flags),
            0xE => self.opcode_E(instr, flags),
            0xF => self.opcode_F(instr, mem, flags),
            _ => Err(stringify!("Couldn't execute instruction: {}", instr).to_string()),
        }
    }

    // 00EE - return from subroutine
    // 00E0 - set clear flag to clear the display
    fn opcode_0(
        &mut self,
        instr: Instruction,
        flags: &mut Flags,
        mem: &mut Memory,
    ) -> Result<(), String> {
        match instr.kk() {
            0xE0 => flags.clear = true,
            0xEE => {
                self.pc = mem.pop_stack();
                self.sp -= 1;
            }
            _ => {
                return Err(stringify!("Unrecognized 0 opcode {}", instr).to_string());
            }
        };

        Ok(())
    }

    // 1nnn - JP addr, jump to location nnn
    fn opcode_1(&mut self, instr: Instruction) -> Result<(), String> {
        self.pc = instr.nnn();
        Ok(())
    }

    // 2nnn - Call addr, call subroutine nnn
    fn opcode_2(&mut self, instr: Instruction, mem: &mut Memory) -> Result<(), String> {
        mem.push_stack(self.pc);
        self.sp += 1;
        self.pc = instr.nnn();

        Ok(())
    }

    // 3xkk - SE Vx, byte, skip next instruction if Vx == kk
    fn opcode_3(&mut self, instr: Instruction) -> Result<(), String> {
        if self.reg[instr.x() as usize] == instr.kk() {
            self.pc += 2;
        }

        Ok(())
    }

    // 4xkk - SNE Vx, byte, skip next instruction if Vx != kk
    fn opcode_4(&mut self, instr: Instruction) -> Result<(), String> {
        if self.reg[instr.x() as usize] != instr.kk() {
            self.pc += 2;
        }

        Ok(())
    }

    // 5xy0 - SE Vx, Vy, skip next instruction if Vx = Vy
    fn opcode_5(&mut self, instr: Instruction) -> Result<(), String> {
        if self.reg[instr.x() as usize] == self.reg[instr.y() as usize] {
            self.pc += 2;
        }

        Ok(())
    }

    // 6xkk - LD VX, byte, set Vx = kk
    fn opcode_6(&mut self, instr: Instruction) -> Result<(), String> {
        self.reg[instr.x() as usize] = instr.kk();
        Ok(())
    }

    // 7xkk - ADD Vx, byte, set Vx = Vx + kk
    fn opcode_7(&mut self, instr: Instruction) -> Result<(), String> {
        self.reg[instr.x() as usize] += instr.kk(); // overflow??
        Ok(())
    }

    // 8xy0 - 8xy7, 8xyE opcodes
    fn opcode_8(&mut self, instr: Instruction) -> Result<(), String> {
        match instr.n() {
            // 8xy0 - LD Vx, vY: set Vx = Vy
            0x0 => self.reg[instr.x() as usize] = self.reg[instr.y() as usize],

            // 8xy1 - OF Vx, Vy: set Vx = Vx OR Vy
            0x1 => self.reg[instr.x() as usize] |= self.reg[instr.y() as usize],

            // 8xy2 - AND Vx, Vy: Set Vx = Vx AND Vy
            0x2 => self.reg[instr.x() as usize] &= self.reg[instr.y() as usize],

            // 8xy3 - XOR Vx, Vy: Set Vx = Vx XOR Vy
            0x3 => self.reg[instr.x() as usize] ^= self.reg[instr.y() as usize],

            // 8xy4 - ADD Vx, Vy: Set Vx = Vx + Vy, set Vf = carry
            0x4 => {
                let sum: u16 = (self.reg[instr.x() as usize] + self.reg[instr.y() as usize]).into();
                if sum > 255 {
                    self.reg[0xF] = 1;
                } else {
                    self.reg[0xF] = 0;
                }

                self.reg[instr.x() as usize] = sum as u8;
            }
            // 8xy5 - SUB Vx, Vy: Set Vx = Vx - Vy, set VF = NOT borrow, if Vx > Vy, then VF is set to 1
            0x5 => {
                let x = self.reg[instr.x() as usize];
                let y = self.reg[instr.y() as usize];

                if x > y {
                    self.reg[0xF] = 1;
                } else {
                    self.reg[0xF] = 0;
                }
                // wrapping_sub to keep from overflowing
                self.reg[instr.x() as usize] = x.wrapping_sub(y);
            }
            // 8xy6 - SHR Vx {, Vy}: Set Vx = Vx SHR 1
            0x6 => {
                let vx = instr.x() as usize;

                self.reg[0xF] = self.reg[vx] & 0x01;
                self.reg[vx] = self.reg[vx] >> 1;
            }
            // 8xy7 - SUBN Vx, Vy: Set Vx = Vy - Vx, set VF = NOT borrow
            0x7 => {
                let x = self.reg[instr.x() as usize];
                let y = self.reg[instr.y() as usize];

                if y > x {
                    self.reg[0xF] = 1;
                } else {
                    self.reg[0xF] = 0;
                }
                // wrapping_sub to keep from overflowing
                self.reg[instr.x() as usize] = y.wrapping_sub(x);
            }
            // 8xyE - SHL Vx {, Vy}: Set Vx = Vx SHL 1
            0xE => {
                let vx = instr.x() as usize;

                self.reg[0xF] = self.reg[vx] & 0x80;
                self.reg[vx] = self.reg[vx] << 1;
            }
            _ => {
                return Err(stringify!("Unrecognized 8 opcode {}", instr).to_string());
            }
        }

        Ok(())
    }

    // 9xy0 - SNE Vx, Vy: Skip next instruction if Vx != Vy
    fn opcode_9(&mut self, instr: Instruction) -> Result<(), String> {
        match instr.n() {
            0 => {
                if self.reg[instr.x() as usize] != self.reg[instr.y() as usize] {
                    self.pc += 2;
                }
                Ok(())
            }
            _ => Err(stringify!("Unrecognized 9 opcode {}", instr).to_string()),
        }
    }

    // Annn - LD I, addr: Set I = nnn
    fn opcode_A(&mut self, instr: Instruction) -> Result<(), String> {
        self.i = instr.nnn();
        Ok(())
    }

    // Bnnn - JP V0, addr: Jump to location nnn + V0
    fn opcode_B(&mut self, instr: Instruction) -> Result<(), String> {
        self.pc = instr.nnn() + (self.reg[0x0] as u16);
        Ok(())
    }

    // Cxkk - RND Vx, byte: Set Vx = random byte AND kk.
    fn opcode_C(&mut self, instr: Instruction) -> Result<(), String> {
        let rnd_num = thread_rng().gen::<u8>();
        self.reg[instr.x() as usize] = rnd_num & instr.kk();

        Ok(())
    }

    // Dxyn - DRW Vx, Vy, nibble
    // the drawing will be handled from the display object
    fn opcode_D(&mut self, instr: Instruction, flags: &mut Flags) -> Result<(), String> {
        flags.draw = true;
        Ok(())
    }

    // Ex9E, ExA1 opcodes
    fn opcode_E(&mut self, instr: Instruction, flags: &mut Flags) -> Result<(), String> {
        let key = flags.key.as_u8();

        match instr.kk() {
            // Ex9E - SKP Vx
            // Skip next instruction if key with the value of Vx is pressed.
            0x9E => {
                if self.reg[instr.x() as usize] == key {
                    self.pc += 2
                }
            }
            // ExA1 - SKNP Vx
            // Skip next instruction if key with the value of Vx is not pressed.
            0xA1 => {
                if self.reg[instr.x() as usize] != key {
                    self.pc += 2
                }
            }
            _ => return Err(stringify!("Unrecognized E opcode {}", instr).to_string()),
        };

        Ok(())
    }

    // Fx07 - Fx65 opcodes
    fn opcode_F(
        &mut self,
        instr: Instruction,
        mem: &mut Memory,
        flags: &mut Flags,
    ) -> Result<(), String> {
        let key = flags.key.as_u8();

        match instr.kk() {
            // Fx07 - LD Vx, DT, Set Vx = delay timer value.
            0x07 => self.reg[instr.x() as usize] = self.dt,

            // Fx0A - LD Vx, K, Wait for a key press, store the value of the key in Vx.
            0x0A => {
                // if the keypressed is one on the chip-8 keyboard 0-F store it.
                // otherwise we will decrement the pc register and wait for a keypress
                if key < 16 {
                    self.reg[instr.x() as usize] = key;
                } else {
                    self.pc -= 2;
                }
            }
            // Fx15 - LD DT, Vx, Set delay timer = Vx.
            0x15 => self.dt = self.reg[instr.x() as usize],

            // Fx18 - LD ST, Vx, Set sound timer = Vx.
            0x18 => self.st = self.reg[instr.x() as usize],

            // Fx1E - ADD I, Vx, Set I = I + Vx.
            0x1E => self.i = self.i + (self.reg[instr.x() as usize] as u16),

            // Fx29 - LD F, Vx, Set I = location of sprite for digit Vx.
            // fonts are stored at memory location 0x000 - 0x1FF. each font takes 5 bytes
            0x29 => self.i = 0x000 + ((self.reg[instr.x() as usize] * 5) as u16),

            // Fx33 - LD B, Vx, Store BCD of Vx in memory locations I, I+1, and I+2.
            // places the hundreds digit in memory at location in I, the tens digit
            // at location I+1, and the ones digit at location I+2
            0x33 => {
                let mut x = self.reg[instr.x() as usize];
                mem.write_byte(self.i + 2, x % 10)?;
                x = x / 10;
                mem.write_byte(self.i + 1, x % 10)?;
                x = x / 10;
                mem.write_byte(self.i, x % 10)?;
            }

            // Fx55 - LD [I], Vx, Store regs V0 through Vx in memory starting at location I.
            // The interpreter copies the values of registers V0 through Vx into memory,
            // starting at the address in I
            0x55 => {
                for loc in 0..self.reg.len() {
                    mem.write_byte(self.i + loc as u16, self.reg[loc as usize])
                        .unwrap();
                }
            }

            //Fx65 - LD Vx, [I], Read regs V0 through Vx from memory starting at location I.
            0x65 => {
                for loc in 0..self.reg.len() {
                    self.reg[loc] = mem.read_byte(self.i + loc as u16).unwrap();
                }
            }
            _ => return Err(stringify!("Unrecognized F opcode {}", instr).to_string()),
        }

        Ok(())
    }
}
