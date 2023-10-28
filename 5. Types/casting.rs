// Without this we can't conver literals that overflow
#![allow(overflowing_literals)]

fn main() {
    // Underscore is aesthetic, similar to python e.g 1_000_000
    let decimal = 65.4321_f32;

    // There is no implicit conversion in rust between primitive types
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Some limitations in conversion rules, can't convert a float directly to a char for example
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    unsigned_casts();
    signed_casts();
}

// Casting to unsigned types, T
// T::MAX + 1 is added or subtracted until the value fits into the new type
fn unsigned_casts() {
    // 1000 already fits, nothing needed here
    println!("1000 as a u16 is: {}", 1000 as u16);

    // u8 can hold up to 255, so following the rule above we see
    // u8      = T
    // T::MAX  = 255
    // MAX + 1 = 256
    // 1000 - 256 - 256 - 256 = 232
    //
    // In terms of binary the 8 least significant bits (LSB) are kept (u8 = 8 bits/1 byte) and the rest are truncated
    // NOTE: assuming little_endian here.
    //
    // example: 1000 in binary is 1111101000
    // keep 8 LSB = 11101000
    // 11101000 = 232
    println!("1000 as a u8 is: {}", 1000 as u8);

    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);
}

// Casting to signed types follows the same process as unsigned types. Then
// if the most significant bit (MSB) is 1, then the value is negative.
fn signed_casts() {
    // Doesn't apply if it fits
    println!("128 as a i16 is: {}", 128 as i16);

    // In this case, conversion amounts to -128
    // 128 is represented as 10000000 in binary, funnily enough applying two's complement results in the same binary representation.
    // This is counted as a boundary case
    // i8 can contain between -128 and 127 inclusive.
    //
    // The math still works, if we followed the comments on line 48/49.
    //
    // 128 is bigger than 127 so u8::MAX = 256
    // 128 - 256 = -128.
    println!("128 as a i8 is: {}", 128 as i8);

    println!("1000 as a u8  is: {}", 1000 as u8);

    // 232 in 8bit two's complement is -24
    // i8 size = -128 to 127
    // 232 - 128 - 128 = -24
    println!("232 as a i8 is: {}", 232 as i8);

    // Using the as keyword performs a saturating cast. This essentially means any overflow will be ignored
    // and the boundary crossed will be the returned value.
    // example: u8 size = 255, so converting 300 into u8 will return 255
    println!("300.0 as u8 is: {}", 300.0_f32 as u8);

    // -100 as u8 is 0
    println!("-100 as u8: {}", -100.0_f32 as u8);

    // nan as u8 is 0
    println!("nan as u8 is: {}", f32::NAN as u8);

    // We can get around this by using unsafe methods, RBE states the results may overflow and return
    // <unsounds values>.
    unsafe {
        println!("300.0 as u8 is: {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
