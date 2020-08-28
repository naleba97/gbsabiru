use super::regfile::*;
use super::mem::*;

pub struct SharpCpu {
    pub regfile: RegFile,
    pub memory: Memory
}

impl Default for SharpCpu {
    fn default() -> SharpCpu {
        SharpCpu {
            regfile: RegFile::default(),
            memory: Memory::default()
        }
    }
}

impl SharpCpu {
    pub fn fetch(&mut self) -> u8 {
        use Register::*;
        // Service any pending interrupts
        // If not, get the opcode at PC, increment PC.
        self.regfile.increment(PC);
        0
    }

    pub fn decode(&self) -> (u8, u8) {
        // Decode the opcode and return the opcode along with the number of bytes/args.
        (0, 0)
    }

    pub fn execute(&mut self, opcode: u8, data: &[u8], num_data_bytes: u8) -> u32 {
        use Register::*;
        match opcode {
            0x00 => { // NOP
                0
            },
            0x02 => { // LD (BC), a
                let mem_dst = self.regfile.get_16reg(BC);
                self.memory.write(mem_dst, self.regfile.get_8reg(A));
                8
            },
            0x08 => { // LD (nn), SP
                let data_from_reg = self.regfile.get_16reg(SP);
                let mem_dst = ((data[1] as u16) << 8) | (data[0] as u16);
                self.memory.write(mem_dst, data_from_reg as u8);
                20
            }
            0x0A => { // LD A, (BC)
                let data_from_mem = self.memory.read(self.regfile.get_16reg(BC));
                self.regfile.set_8reg(A, data_from_mem);
                8
            },
            0x12 => { // LD (DE), a
                let mem_dst = self.regfile.get_16reg(DE);
                self.memory.write(mem_dst, self.regfile.get_8reg(A));
                8
            },
            0x1A => { // LD A, (DE)
                let data_from_mem = self.memory.read(self.regfile.get_16reg(DE));
                self.regfile.set_8reg(A, data_from_mem); 
                8
            },
            0x22 => { // LD (HL+), A
                let mem_dst = self.regfile.get_16reg(HL); 
                self.memory.write(mem_dst, self.regfile.get_8reg(A));
                self.regfile.increment(HL);
                8
            },
            0x2A => { // LD A, (HL+)
                let mem_src = self.regfile.get_16reg(HL);
                let mem_data = self.memory.read(mem_src);
                self.regfile.set_8reg(A, mem_data);
                self.regfile.increment(HL);
                8
            },
            0x32 => { // LD (HL-), A
                let mem_dst = self.regfile.get_16reg(HL);
                self.memory.write(mem_dst, self.regfile.get_8reg(A));
                self.regfile.decrement(HL);
                8
            },
            0x36 => { // LD (HL), n
                let mem_dst = self.regfile.get_16reg(HL); 
                self.memory.write(mem_dst, data[0]);
                12
            },
            0x3A => { // LD A, (HL-)
                let mem_src = self.regfile.get_16reg(HL);
                let mem_data = self.memory.read(mem_src);
                self.regfile.set_8reg(A, mem_data);
                self.regfile.decrement(HL);
                8
            },
            0xE0 => { // LDH (n), A
                let mem_dst = ((0xFF as u16) << 8) | (data[0] as u16);
                self.memory.write(mem_dst, self.regfile.get_8reg(A));
                12
            },
            0xE2 => { // LDH (C), A
                let mem_dst = ((0xFF as u16) << 8) | (self.regfile.get_8reg(C) as u16);
                self.memory.write(mem_dst, self.regfile.get_8reg(A));
                8
            },
            0xEA => { // LD (nn), A
                let mem_dst = ((data[1] as u16) << 8) | (data[0] as u16);
                self.memory.write(mem_dst, self.regfile.get_8reg(A));
                16
            },
            0xF0 => { // LDH A, (n)
                let mem_src = ((0xFF as u16) << 8) | (data[0] as u16);
                let mem_data = self.memory.read(mem_src);
                self.regfile.set_8reg(A, mem_data);
                12
            },
            0xF2 => { // LDH A, (C)
                let mem_src = ((0xFF as u16) << 8) | (self.regfile.get_8reg(C) as u16);
                let mem_data = self.memory.read(mem_src);
                self.regfile.set_8reg(A, mem_data);
                8
            },
            0xF8 => { // LD HL, SP + i8
                let reg_src = self.regfile.get_16reg(SP) as i16;
                let offset_i8 = data[0] as i16;
                self.regfile.set_flag(HL, (reg_src + offset_i8) as u16);
                self.regfile.set_flag(ZERO_F, false);
                self.regfile.set_flag(HALF_CARRY_F, (reg_src & 0x0FFF) + (offset_i8 & 0x0FFF) >= 0x1000);
                self.regfile.set_flag(NEGATIVE_F, false);
                self.regfile.set_flag(CARRY_F, reg_src.overflowing_add(offset_i8).1);
                12
            },
            0xF9 => { // LD SP, HL
                self.regfile.set_16reg(SP, self.regfile.get_16reg(HL));
                8
            }
            0xFA => { // LD A, (nn)
                let mem_src = ((data[1] as u16) << 8) | (data[0] as u16);
                let mem_data = self.memory.read(mem_src);
                self.regfile.set_8reg(A, mem_data);
                16
            },
            opcode if ((opcode & 0xF8) == 0x80) => { // ADD A, r
                let a_data = self.regfile.get_8reg(A);
                let r_data = self.regfile.get_8reg(RegFile::num_to_8reg(opcode & 0x07).unwrap());
                self.regfile.set_8reg(A, a_data + r_data);
                4
            },
            opcode if ((opcode & 0xCF) == 0xC5) => { // PUSH rr 
                let reg_src = RegFile::num_to_16reg_stack((opcode & 0x30) >> 4).unwrap();
                let (msb, lsb) = self.regfile.get_byte_pair(reg_src);
                self.regfile.decrement(SP);
                self.memory.write(self.regfile.get_16reg(SP) as u16, msb as u8);
                self.regfile.decrement(SP);
                self.memory.write(self.regfile.get_16reg(SP) as u16, lsb as u8);
                16
            },
            opcode if ((opcode & 0xCF) == 0xC5) => { // POP rr 
                let reg_dst = RegFile::num_to_16reg_stack((opcode & 0x30) >> 4).unwrap();
                let mut data_from_stack = self.memory.read(self.regfile.get_16reg(SP)) as u16;
                self.regfile.increment(SP);
                data_from_stack |= (self.memory.read(self.regfile.get_16reg(SP)) as u16) << 8;
                self.regfile.increment(SP);
                self.regfile.set_16reg(reg_dst, data_from_stack);
                12
            },
            opcode if ((opcode & 0xCF) == 0x01) => { // LD rr, nn
                let data_imm = ((data[1] as u16) << 8) | (data[0] as u16);
                let reg_dst = RegFile::num_to_16reg((opcode & 0x30) >> 4).unwrap();
                self.regfile.set_16reg(reg_dst, data_imm);
                12
            },
            opcode if ((opcode & 0xC7) == 0x06) => { // LD r, n
                let reg_dst = RegFile::num_to_8reg((opcode & 0x38) >> 3).unwrap();
                self.regfile.set_8reg(reg_dst, data[0]);
                8
            },
            opcode if ((opcode & 0xC7) == 0x46) => { // LD r, (HL)
                let reg_dst = RegFile::num_to_8reg((opcode & 0x38) >> 3).unwrap();
                let data_from_mem = self.memory.read(self.regfile.get_16reg(HL));
                self.regfile.set_8reg(reg_dst, data_from_mem);
                8
            },
            opcode if ((opcode & 0xF8) == 0x70) => { // LD (HL), r
                let reg_src = RegFile::num_to_8reg(opcode & 0x07).unwrap();
                let data_from_reg = self.regfile.get_8reg(reg_src);
                let mem_dst = self.regfile.get_16reg(HL);
                self.memory.write(mem_dst, data_from_reg);
                8
            },
            opcode if ((opcode & 0xC1) == 0x40) => { // LD r, r'
                let reg_dst = RegFile::num_to_8reg((opcode & 0x38) >> 3).unwrap();
                let reg_src = RegFile::num_to_8reg(opcode & 0x07).unwrap();
                self.regfile.set_8reg(reg_dst, self.regfile.get_8reg(reg_src));
                4
            },
            _ => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Register::*;
    
    #[test]
    fn multi_byte_opcode() {
        let mut cpu = SharpCpu::default();
        
        // Load register B with data.
        let mut opcode = 0x06;
        let mut data = [0xAB];
        let mut num_data_bytes = 1;
        let mut num_cycles = cpu.execute(opcode, &data, num_data_bytes); 
        assert_eq!(num_cycles, 4);
        assert_eq!(cpu.regfile.get(B), 0xAB);

        // Load register C with the contents of register C.
        opcode = 0x48;
        data = [0];
        num_data_bytes = 1;
        num_cycles = cpu.execute(opcode, &data, num_data_bytes);
        assert_eq!(num_cycles, 4);
        assert_eq!(cpu.regfile.get(C), 0xAB);
        assert_eq!(cpu.regfile.get(B), 0xAB);
    }

    #[test]
    fn reg_to_mem() {
        let mut cpu = SharpCpu::default();
        
        //Load register B with data.
        let mut num_cycles = cpu.execute(0x06, &[0xAB], 1); 
        assert_eq!(num_cycles, 4);
        assert_eq!(cpu.regfile.get(B), 0xAB);
//Load address 0x00AB with register B.
        cpu.execute(0x36, &[0xAB], 1);
        assert_eq!(cpu.regfile.get(HL), 0x00AB);
        assert_eq!(cpu.regfile.get(L), 0xAB);
        assert_eq!(cpu.regfile.get(H), 0x00);

        num_cycles = cpu.execute(0x70, &[0], 1);
        assert_eq!(num_cycles, 4);
        assert_eq!(cpu.memory.read(0x00AB), 0xAB);

        //Load register C with the contents of address 0x00AB.
        num_cycles = cpu.execute(0x4E, &[0], 0);
        assert_eq!(num_cycles, 4);
        assert_eq!(cpu.regfile.get(C), 0xAB);
    }
}
