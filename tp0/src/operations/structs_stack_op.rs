struct Dup {
    quantity: u8,
    operands: i16,
}
struct Drop {
    quantity: u8,
    operands: i16,
}
struct Swap {
    quantity: u8,
    operands: Vec<i16>,
}
struct Over {
    quantity: u8,
    operands: i16,
}
struct Rot {
    quantity: u8,
    operands: Vec<i16>,
}
