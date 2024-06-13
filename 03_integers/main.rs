// --= First task --solved
fn main() {
    let x: u32 = 5;
    let mut y: u32 = 5;

    // when assigning types MUST match!
    y = x;

    let z: i32 = 10; // i32. i32 is default integer type

    println!("Success");
}

// --= Second task --solved
fn main() {
    // number 38 is of type u8
    let v: u16 = 38_u8 as u16;

    println!("Success");
}

// --=Third task --solved
fn main() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success");
}

fn type_of<T>(_: &T) -> String {
    return format!("{}", std::any::type_name::<T>()); // "i32"
}

// --=Fourth task --solved
fn main() {
    // ::MAX is a constant that returns the largest possible value
    //          that can be represented by a given type 
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

// --=Fifth task --solved
fn main() {
    let v1 = 251_u16 + 8;
    let v2: i16 = i16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}

// --=Sixth task --solved
fn main() {
    // _ is a delimiter for readability. It does not affect anything
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert!(v == 1597);

    println!("Success!");
}

// --=Seventh task --solved
fn main() {
    // f64 is default floating-point type
    let x: f64 = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    return format!("{}", std::any::type_name::<T>());
}

// --=Eighth task v1 --solved
fn main() {
    // floating point precision
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32); // 0.1 + 0.2 = 0.30000000000002

    println!("Success!");
}
// v2 --solved
fn main() {
    // floating point precision
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32); // 0.1 + 0.2 = 0.30000000000002

    println!("Success!");
}

// --=Ninth task --done
fn main() {
    let mut sum: i32 = 0; // (-3, -5, -6, -6, -5)

    // -3..2 is a special syntax for range [-3: 1]
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    // 'a'..='z' is a special syntax for range
    //            from a to z, included because of sign "=".
    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}

// --=Tenth task --done
// std - standard library, ops - ops module, Range and RangeInclusive
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

// --=Eleventh task --
fn main() {
    // Integer addition
    // type infering will do the best. so all is u32 type
    // _ is just a delimiter
    assert!(1u32 + 2 == 3);

    // Integer substraction
    assert!(1i32 - 2i32 == -1i32);
    assert!(1i8 - 2i8 == -1i8);

    // Integer multiplying
    assert!(3 * 50 = 150); // i32

    // Integer division
    assert!(9.6f32 / 3.2f32 == 3.0f32);

    // Modulus operator
    assert!(24 % 5 == 4);

    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
