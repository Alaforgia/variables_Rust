fn main() {
    let  x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

 // addition
 let _sum = 5 + 10;
 // subtraction
 let _difference = 95.5 - 4.3;
 // multiplication
 let _product = 4 * 30;
 // division
 let _quotient = 56.7 / 32.2;
 // remainder
 let _remainder = 43 % 5;
}

// constants are values that are bound to a name and are not allowed to change, but there are a few differences between 
// constants and variables. You aren't allowed to use mut w/ constants.

// A scalar type represents a single value. Rust has four primary scalar types: 
// integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages.

// integer types are numbers w/o a fractional component. u32 is an unsigned integer type. A signed integer type would start
// with i instead of u.

//floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64
// because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
// The f32 type is a single-precision float, and f64 has double precision.

// char type is the language’s most primitive alphabetic type, and the following code shows one way to use it. 
// (Note that char literals are specified with single quotes, as opposed to string literals, which use double quotes.)
// fn main() {
//     let c = 'z';
//     let z = 'Ƶ';
//     let heart_eyed_cat = '�';
//    }

//A tuple is a general way of grouping together some number of other values with a variety of types into one compound type. 
// Tuples have a fixed length: once declared, they cannot grow or shrink in size.
// We create a tuple by writing a comma-separated list of values inside
// parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same. 
// We’ve added optional type annotations in this example:
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//    }
//
// The variable tup binds to the entire tuple, because a tuple is considered a single compound element. 
// To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
// fn main() {
//  let tup = (500, 6.4, 1);
//  let (x, y, z) = tup;
//  println!("The value of y is: {}", y);
// }