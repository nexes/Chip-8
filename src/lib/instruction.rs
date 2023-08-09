use std::fmt;

// Chip-8 instructions are all 16 bits long and stored in big-endian
//
// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
// kk or byte - An 8-bit value, the lowest 8 bits of the instruction
pub(crate) struct Instruction {
    itype: u8,
    n: u8,
    x: u8,
    y: u8,
    kk: u8,
    nnn: u16,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.itype {
            0 => {
                if self.n == 0 {
                    write!(f, "0: Clear display\n\tkk: {:#03x}", self.kk)
                } else {
                    write!(f, "0: Return subroutine\n\tkk: {:#03x}", self.kk)
                }
            }
            1 => write!(f, "1: Jump to nnn\n\tnnn: {:#04x}", self.nnn),
            2 => write!(f, "2: Call\n\taddr: {:#04x}", self.nnn),
            3..=5 => write!(
                f,
                "{}: Skip Instruction\n\tx: {:#03x}\n\ty:{:#03x}\n\tkk:{:#04x}",
                self.itype, self.x, self.y, self.kk
            ),
            6..=8 => write!(
                f,
                "{}: Set Vx\n\tx: {:#03x}\n\ty: {:#03x}\n\tkk: {:#04x}",
                self.itype, self.x, self.y, self.kk
            ),
            9 => write!(
                f,
                "9: Skip if x != y\n\tx: {:#03x}\n\ty: {:#03x}",
                self.x, self.y
            ),
            10 => write!(f, "A: Set I to nnn\n\tnnn: {:#04x}", self.nnn),
            11 => write!(f, "B: Jump nnn + V0\n\tnnn: {:#04x}", self.nnn),
            12 => write!(
                f,
                "C: Set random + kk\n\tx: {:#03x}\n\tkk: {:#04x}",
                self.x, self.kk
            ),
            13 => write!(
                f,
                "D: Draw\n\tx: {:#03x}\n\ty:{:#03x}\n\tn: {:#02x}",
                self.x, self.y, self.n
            ),
            14 => write!(f, "E: Skip\n\tx: {:#03x}\n\tkk: {:#04x}", self.x, self.kk),
            15 => write!(
                f,
                "F: Set Store Read\n\tx: {:#03x}\n\tkk: {:#04x}",
                self.x, self.kk
            ),
            _ => write!(f, "unrecognized type: {:#03x}", self.itype),
        }
    }
}

impl Instruction {
    pub(crate) fn decode(data: u16) -> Instruction {
        Instruction {
            itype: ((data & 0xF000) >> 12) as u8,
            n: (data & 0x000F) as u8,
            x: ((data & 0x0F00) >> 8) as u8,
            y: ((data & 0x00F0) >> 4) as u8,
            kk: (data & 0x00FF) as u8,
            nnn: (data & 0x0FFF),
        }
    }

    // return the instruction type
    pub(crate) fn itype(&self) -> u8 {
        self.itype
    }

    // return the instruction n (lowest 4 bits)
    pub(crate) fn n(&self) -> u8 {
        self.n
    }

    // return the instruction x (lower 4 bits of the high byte)
    pub(crate) fn x(&self) -> u8 {
        self.x
    }

    // return the instruction y (upper 4 bits of the low byte)
    pub(crate) fn y(&self) -> u8 {
        self.y
    }

    // return the instruction kk (lowest 8 bits)
    pub(crate) fn kk(&self) -> u8 {
        self.kk
    }

    // return the instruction nnn (lowest 12 bits)
    pub(crate) fn nnn(&self) -> u16 {
        self.nnn
    }
}
