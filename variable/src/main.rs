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
    println!("f32 limits: {}, {}", f32::MIN, f32::MAX);
    println!("f64 limits: {}, {}", f64::MIN, f64::MAX);

    let hex = 5u8;
    let ascii_en = b'A';
}
