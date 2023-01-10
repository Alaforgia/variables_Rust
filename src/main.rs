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
//
// We can access a tuple element directly by using a period (.) followed by the index of the value we want to access. 
// For example:
// fn main() {
//  let x: (i32, f64, u8) = (500, 6.4, 1);
//  let five_hundred = x.0;
//  let six_point_four = x.1;
//  let one = x.2;
//
//
// Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some 
// other languages because arrays in Rust have a fixed length, like tuples.
// In Rust, the values going into an array are written as a comma-separated list inside square brackets:
// fn main() {
//  let a = [1, 2, 3, 4, 5];
// }
// Vector types are allowed to shrink and grow. Array types cannot.
//
// You would write an array’s type by using square brackets, and within the brackets include the 
// type of each element, a semicolon, and then the number of elements in the array, like so:
// let a: [i32; 5] = [1, 2, 3, 4, 5];
// i32 is the type of each element. The number 5 indicates the element contains five items.
//
// let a = [3; 5];
// The array named a will contain 5 elements that will all be set to the value 3 initially. 
// This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
//
//
// An array is a single chunk of memory allocated on the stack. You can access elements of an array using indexing, like this:
// fn main() {
//  let a = [1, 2, 3, 4, 5];
//  let first = a[0];
//  let second = a[1];
// }
// In this example, the variable named first will get the value 1, because that is the value at index [0] in the array. 
// The variable named second will get the value 2 from index [1] in the array.