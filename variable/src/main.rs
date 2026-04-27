fn main() {
    let mut x = 10;
    println!("Hello, Rust! {}", x);
    x = x * 2;
    println!("x = {}", x);

    const KM_TO_M: i32 = 1000;
    println!("{}km = {}m", x, x * KM_TO_M);
    println!("i8 limits: {}, {}", i8::MIN, i8::MAX);
    println!("i16 limits: {}, {}", i16::MIN, i16::MAX);
    println!("u8 limits: {}, {}", u8::MIN, u8::MAX);
    println!("f32 Digits: {}", f32::DIGITS);
    println!("f64 Digits: {}", f64::DIGITS);

    let hex = 5u8;
    let base_8 = 0o764;
    let bin = 0b1101_1101;
    let ascii_en = b'A';

    show_number(bin);
    show_number(base_8);
    println!("sum of {} and {} = {}", hex, ascii_en, add_number(hex, ascii_en));
}

fn add_number(a: u8, b: u8) -> u8 {
    let sum = a + b;
    return sum;
}

fn show_number(num: i32) -> () {
    println!("Number is {}", num);
}