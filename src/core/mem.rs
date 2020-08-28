pub struct Memory {
    mem: [u8; 0xFFFF] 
}

/*
 * 0000 	3FFF 	16KB ROM bank 00 	From cartridge, usually a fixed bank
 * 4000 	7FFF 	16KB ROM Bank 01~NN 	From cartridge, switchable bank via MB (if any)
 * 8000 	9FFF 	8KB Video RAM (VRAM) 	Only bank 0 in Non-CGB mode Switchable bank 0/1 in CGB mode
 * A000 	BFFF 	8KB External RAM 	In cartridge, switchable bank if any
 * C000 	CFFF 	4KB Work RAM (WRAM) bank 0 	
 * D000 	DFFF 	4KB Work RAM (WRAM) bank 1~N 	Only bank 1 in Non-CGB mode Switchable bank 1~7 in CGB mode
 * E000 	FDFF 	Mirror of C000~DDFF (ECHO RAM) 	Typically not used
 * FE00 	FE9F 	Sprite attribute table (OAM) 	
 * FEA0 	FEFF 	Not Usable 	
 * FF00 	FF7F 	I/O Registers 	
 * FF80 	FFFE 	High RAM (HRAM) 	
 * FFFF 	FFFF 	Interrupts Enable Register (IE)
 */

pub enum MemAddress {
    ROMBank,
    VRAM,
    ExternalRAM,
    WRAM,
    EchoRAM,
    OAM,
    Unusable,
    IO,
    HRAM,
    IE
}


impl Default for Memory {
    fn default() -> Memory {
        Memory {
            mem: [0; 0xFFFF]
        }
    }  
}

impl Memory {
    pub fn write(&mut self, mem_addr: u16, data: u8) {
        self.mem[mem_addr as usize] = data; 
    }

    pub fn read(&self, mem_addr: u16) -> u8 {
        self.mem[mem_addr as usize]
    }

    pub fn clear(&mut self) {
        for elem in self.mem.iter_mut() { *elem = 0; }
    }
    
    pub fn addr_to_type(mem_addr: u16) -> MemAddress {
        match mem_addr {
            0x0000..=0x7FFF => MemAddress::ROMBank,
            0x8000..=0x9FFF => MemAddress::VRAM,
            0xA000..=0xBFFF => MemAddress::ExternalRAM,
            0xC000..=0xDFFF => MemAddress::WRAM,
            0xE000..=0xFDFF => MemAddress::EchoRAM,
            0xFE00..=0xFE9F => MemAddress::OAM,
            0xFEA0..=0xFEFF => MemAddress::Unusable,
            0xFF00..=0xFF7F => MemAddress::IO,
            0xFF80..=0xFFFE => MemAddress::HRAM,
            0xFFFF => MemAddress::IE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_to_mem() {
        let mut memory = Memory::default();
        memory.write(0xABCD, 0x32);
        assert_eq!(memory.read(0xABCD), 50);
    }
}
