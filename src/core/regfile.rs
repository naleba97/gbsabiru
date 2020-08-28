enum_from_primitive! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum Register {
        // 8-bit registers
        B,
        C,
        D,
        E,
        H,
        L,
        F,
        A,
        SP_HIGH,
        SP_LOW,
        PC_HIGH,
        PC_LOW,
        NumRegisters,
        // 16-bit registers
        AF,
        BC,
        DE,
        HL,
        SP,
        PC,
        // 1-bit registers or Flags
        ZERO_F,
        NEGATIVE_F,
        HALF_CARRY_F,
        CARRY_F

    }
}

pub struct RegFile([u8; Register::NumRegisters as usize]);

impl RegFile {
    pub fn default() -> RegFile {
        RegFile (
            [0; Register::NumRegisters as usize]
        )
    }

    pub fn get_8reg(&self, reg: Register) -> u8 {
        use Register::*;
        match reg {
            _ if (reg as usize) < (NumRegisters as usize) => self.0[reg as usize],
            _ => panic!("Not a valid register."),
        }
    }

    pub fn get_16reg(&self, reg: Register) -> u16 {
        use Register::*;
        match reg {
            AF => ((self.get_8reg(A) as u16) << 8) | (self.get_8reg(F) as u16),
            BC => ((self.get_8reg(B) as u16) << 8) | (self.get_8reg(C) as u16),
            DE => ((self.get_8reg(D) as u16) << 8) | (self.get_8reg(E) as u16),
            HL => ((self.get_8reg(H) as u16) << 8) | (self.get_8reg(L) as u16),
            SP => ((self.get_8reg(SP_HIGH) as u16) << 8) | (self.get_8reg(SP_LOW) as u16),
            PC => ((self.get_8reg(PC_HIGH) as u16) << 8) | (self.get_8reg(PC_LOW) as u16),
            _ => panic!("Not a valid 16-bit register.")
        }
    }

    pub fn get_flag(&self, reg: Register) -> bool {
        use Register::*;
        match reg {
            ZERO_F => ((self.0[F as usize] & 0x80) >> 7) != 0,
            ADD_SUB_F => ((self.0[F as usize] & 0x40) >> 6) != 0,
            HALF_CARRY_F => ((self.0[F as usize] & 0x20) >> 5) != 0,
            CARRY_F => ((self.0[F as usize] & 0x10) >>  4) != 0,
            _ => panic!("Not a valid flag."),
        }
    }

    pub fn get_byte_pair(&self, reg: Register) -> (i32, i32) {
        use Register::*;
        match reg {
            AF => (self.get_8reg(A), self.get_8reg(F)),
            BC => (self.get_8reg(B), self.get_8reg(C)),
            DE => (self.get_8reg(D), self.get_8reg(E)),
            HL => (self.get_8reg(H), self.get_8reg(L)),
            SP => (self.get_8reg(SP_HIGH), self.get_8reg(SP_LOW)),
            PC => (self.get_8reg(PC_HIGH), self.get_8reg(PC_LOW)),
            _ => panic!("Not a 16-bit register.")
        }
    }

    pub fn set_8reg(&mut self, reg: Register, val: u8){
        use Register::*;
        match reg {
            _ if (reg as usize) < (NumRegisters as usize) => self.0[reg as usize] = val,
            _ => panic!("Invalid register."),
        }
    }

    pub fn set_16reg(&mut self, reg: Register, val: u16){
        use Register::*;
        match reg {
            AF => {
                self.0[A as usize] = ((val & 0xFF00) >> 8) as u8;
                self.0[F as usize] = (val & 0x00FF) as u8;
            },
            BC => {
                self.0[B as usize] = ((val & 0xFF00) >> 8) as u8;
                self.0[C as usize] = (val & 0x00FF) as u8;
            },
            DE => {
                self.0[D as usize] = ((val & 0xFF00) >> 8) as u8;
                self.0[E as usize] = (val & 0x00FF) as u8;
            },
            HL => {
                self.0[H as usize] = ((val & 0xFF00) >> 8) as u8;
                self.0[L as usize] = (val & 0x00FF) as u8;
            },
            SP => {
                self.0[SP_HIGH as usize] = ((val & 0xFF00) >> 8) as u8;
                self.0[SP_LOW as usize] = (val & 0x00FF) as u8;
            },
            PC => {
                self.0[PC_HIGH as usize] = ((val & 0xFF00) >> 8) as u8;
                self.0[PC_LOW as usize] = (val & 0x00FF) as u8;
            }
            _ => panic!("Not a valid 16-bit register."),
        }
    }

    pub fn set_flag(&mut self, reg: Register, val: bool) {
        use Register::*;
        match reg {
            ZERO_F => self.0[F as usize] = (self.0[F as usize] & 0x7F) | ((val as u8) << 7),
            ADD_SUB_F => self.0[F as usize] = (self.0[F as usize] & 0xBF) | ((val as u8) << 6),
            HALF_CARRY_F => self.0[F as usize] = (self.0[F as usize] & 0xDF) | ((val as u8) << 5),
            CARRY_F => self.0[F as usize] = (self.0[F as usize] & 0xEF) | ((val as u8) << 4),
            _ => panic!("Not a valid flag.")
        }
    }

    pub fn increment(&mut self, reg: Register) {
        use Register::*;
        match reg {
            AF | BC | DE | SP | HL | PC => {self.set_16reg(reg, self.get_16reg(reg) + 1);}
            A | F | B | C | D | E | H | L => {self.set_8reg(reg, self.get_8reg(reg) + 1);} 
        }
    }

    pub fn decrement(&mut self, reg: Register) {
        use Register::*;
        match reg {
            AF | BC | DE | SP | HL | PC => {self.set_16reg(reg, self.get_16reg(reg) - 1);}
            A | F | B | C | D | E | H | L => {self.set_8reg(reg, self.get_8reg(reg) - 1);}
            _ => {} 
        }
    }

    pub fn num_to_8reg(num: u8) -> Option<Register> {
        match num {
            0 => Some(Register::B),
            1 => Some(Register::C),
            2 => Some(Register::D),
            3 => Some(Register::E),
            4 => Some(Register::H),
            5 => Some(Register::L),
            7 => Some(Register::A),
            _ => None
        }
    }

    pub fn num_to_16reg(num: u8) -> Option<Register> {
        match num {
            0 => Some(Register::BC),
            1 => Some(Register::DE),
            2 => Some(Register::HL),
            3 => Some(Register::SP),
            _ => None 
        }
    }
    
    pub fn num_to_16reg_stack(num: u8) -> Option<Register> {
        match num {
            0 => Some(Register::BC),
            1 => Some(Register::DE),
            2 => Some(Register::HL),
            3 => Some(Register::AF),
            _ => None
        }
    }
}

