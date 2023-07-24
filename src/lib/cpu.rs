use crate::instruction::Instruction;
use crate::memory::Memory;

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
    pub fn init() -> CPU {
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
    pub(crate) fn execution_address(&mut self) -> u16 {
        let loc = self.pc;
        self.pc += 0x02;

        loc
    }

    pub(crate) fn execute(&mut self, instr: Instruction, mem: &mut Memory) -> Result<(), String> {
        match instr.itype() {
            0x0 => self.opcode_0(instr, mem),
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
            _ => Err(stringify!("Couldn't execute instruction: {}", instr).to_string()),
        }
    }

    // opcode 00EE - return from subroutine
    fn opcode_0(&mut self, instr: Instruction, mem: &mut Memory) -> Result<(), String> {
        match instr.kk() {
            0xEE => {
                self.pc = mem.pop_stack();
                self.sp -= 1;
            }
            _ => {
                return Err(stringify!("Unrecognized 0 opcode {}", instr).to_string());
            }
        }

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
        let a: u8 = 200;
        let b: u8 = 200;

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
}
