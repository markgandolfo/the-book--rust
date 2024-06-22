pub fn run() {
    // rust is statically typed, so if we specificy a type:number, then we get a compilation error
    //
    //  let guess: u32 = "42".parse().expect("Not a number");
    //      ^^^^^ help: if this is intentional, prefix it with an underscore: `_guess`

    // Integer types in rust
    // -----------------------
    //
    // Length   Signed  Unsigned
    // 8-bit    i8      u8
    // 16-bit   i16     u16
    // 32-bit   i32     u32
    // 64-bit   i64     u64
    // 128-bit  i128    u128
    // arch     isize   usize
    //
    // arch is calculated based on the 32/64 bit architecture of the computer the program is
    // running on.
    //
    // to calculate the range (where n is the amount of bits)
    // signed is ranged => -(2^n-1) to (2^n-1)
    // unsigned is ranged => 0 - 2^n
    //
    // Integer Literals in Rust
    // ------------------------
    //
    // Number literals  Example
    // Decimal          98_222
    // Hex              0xff
    // Octal            0o77
    // Binary           0b1111_0000
    // Byte (u8 only)   b'A'

    // floating point numbers
    // Rust has two floating point numbers, f32 and f64.
    let _x = 2.0; // rusts default type is f64
    let _y: f32 = 3.0; // f32

    // numeric operations
    println!("Numberic Operations\n");
    //
    // addtion
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    // subtraction
    let difference = 10 - 5;
    println!("10 - 5 = {}", difference);

    // multiplication
    let product = 5 * 10;
    println!("5 * 10 = {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; //results in -1
    println!("56.7 / 32.2 = {}", quotient);
    println!("-5 / 3 = {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);

    // Compound Types
    println!("\nCompound Types\n");
    let tup = (500, 6.4, 1);
    let (_, y, _) = tup;

    println!("The value of y is: {y}");

    println!("You can also access a tuple by using a .index");
    println!("tup.0: {}", tup.0);

    // Arrays
    println!("\nArrays are stored on the stack. And are fixed sized as a result");
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("You can access arrays using the months[0]: {}", months[0]);

    // trying to access an element out of biounds of the array will allow the program to compile,
    // but it'll panic on run.
    //
    // let element = months[15];
    //     ^^^^^^^^^^ index out of bounds: the length is 12 but the index is 15
}
