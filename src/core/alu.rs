pub fn is_half_carry_u8(a: u8, b: u8) -> bool {
    if ((a & 0x0f) + (b & 0x0f)) & 0x10 == 0x10 {
        return true;
    }
    false
}

pub fn is_half_borrow_u8(a: u8, b: u8) -> bool {
    if ((a & 0x0f) + (b & 0x0f)) & 0x10 == 0x10 {
        return true;
    }
    false
}

pub fn is_half_carry_u16(a: u16, b: u16) {

}
