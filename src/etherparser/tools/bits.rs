pub fn bit_bool(position: u16, byte: u16) -> bool {
    (byte >> position) & 1 == 1
}
