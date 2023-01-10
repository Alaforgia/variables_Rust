fn main() {
    let  x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

// constants are values that are bound to a name and are not allowed to change, but there are a few differences between 
// constants and variables. You aren't allowed to use mut w/ constants.

// A scalar type represents a single value. Rust has four primary scalar types: 
// integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages.

// integer types are numbers w/o a fractional component. u32 is an unsigned interger type. A signed integer type would start
// with i instead of u.

//floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64
// because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
// The f32 type is a single-precision float, and f64 has double precision.

