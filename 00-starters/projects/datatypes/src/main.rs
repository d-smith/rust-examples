fn main() {
    
    // signed and unsigned integer types
    let _i: i8 = 0b1;
    let _i: i16 = 0xff;
    let _i: i32 = 0o77;
    let _i: i64 = 1;
    let _i: i128 = 1;
    let i: isize= 1_000_000;

    println!("{i}");


    let _i: u8 = b'A';
    let _i: u16 = 1;
    let _i: u32 = 1;
    let _i: u64 = 1;
    let _i: u128 = 1;
    let i: usize= 1;

    println!("{i}");

    // Next: floating point types

    let _f: f32 = 1.0;
    let _f: f64 = 2.0;
    println!("{_f}");

    // Numeric operators include the usual suspects
    let _x = 20 + 30 - 40 * 50 / 60 % 70;
    println!("{_x}");

}