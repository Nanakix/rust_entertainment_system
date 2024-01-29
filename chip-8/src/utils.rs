
// Thanks to https://stackoverflow.com/questions/70542217/how-do-i-split-a-16-bit-value-into-two-8-bit-values
pub fn shift_idiomatic_split_u16(short_16: u16) -> [u8; 2] {
    [(short_16 >> 8) as u8, (short_16 & 0xff) as u8]
}