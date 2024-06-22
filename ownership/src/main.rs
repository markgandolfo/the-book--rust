// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
fn main() {
    println!("---- STRINGS ----");
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appens a literal to a String
    println!("{s}");

    //
    // Rust doesn't have a GC. instead it it "returns the memory" (releases it) once the variable
    // is out of scope.... isn't that kind of similar to a GC??? maybe... it seems that rust calls
    // a function called drop() on variables that are out of scope automatically at compile time??
    //
    {
        let _s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    //
    // INTERACTING WITH MOVE
    //
    //
    // this is assigning 5 to x, and then assining 5 to y.
    // in the compiler this is actually copying the data to a new stack location
    // this is done because copying data on the stack (especially when we know the size) is quick
    //
    // This is done with the "copy trait".
    // Other variables are things like integers, bools, floating point numbers, chars, and tuples
    // (as long as it doesn't contain a string)
    //
    let x = 5;
    let _y = x;

    //
    // the string version is a little different
    //
    // in this example, both s1 and s2 are in the stack with the information (len, capacity, and
    // ptr). the PTRs for both s1 and s2 are both pointed to the SAME position in the heap where
    // the String ("hello") is stored..
    //
    // To get around this problem. s1 is no longer valid after being assigned to s2. this is to
    // reduce the issue of double freeing up memory
    //
    let s1 = String::from("hello");
    let _s2 = s1;

    // if we try access s1 we'll get a compilation error
    //
    // let s1 = String::from("hello");
    //     -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    // let s2 = s1;
    //          -- value moved here
    //
    // println!("{}, world!", s1);
    //                        ^^ value borrowed here after move

    //
    // if we wanted both to exist we need to use clone
    //
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    //
    // Ownership and functions
    //
    // the mechanic of passing a value to a function are similar to those when assigning a value to
    // a variable. Passing a variable to a function will move or copy, just as assignment does.
    //
    {
        let s = String::from("hello");
        takes_ownership(s); // s's value moved into this function
                            // so is no longer valid in this scope

        let x = 5;
        makes_copy(x); // x's value is moved into the function
                       // but it's an int, so it's a copy, so we can use x afterwards
        println!("OG: {x}");

        fn takes_ownership(some_string: String) {
            println!("{}", some_string);
        }

        fn makes_copy(an_integer: i32) {
            println!("{}", an_integer);
        }
    }

    //
    // returning values can also transfer ownership
    //
    {
        let _s1 = gives_ownership(); // gives_ownership moves its return
                                     // value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3

        fn gives_ownership() -> String {
            // gives_ownership will move its
            // return value into the function
            // that calls it

            let some_string = String::from("yours"); // some_string comes into scope

            some_string // some_string is returned and
                        // moves out to the calling
                        // function
        }

        // This function takes a String and returns one
        fn takes_and_gives_back(a_string: String) -> String {
            // a_string comes into
            // scope

            a_string // a_string is returned and moves out to the calling function
        }
    }

    //
    // You can also borrow and return back using a tuple.
    // but this is a bit shit, is it not?
    //
    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{s2}' is {len}.");

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len(); // len() returns the length of a String
            (s, length)
        }
    }

    //
    // References and Borrowing
    //  Instead of borrowing and returning a tuple, we can pass references around.
    //
    {
        let s1 = String::from("hello");
        let _len = calculate_length(&s1);

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
    }

    // When we borrow we can't modify within the function. trying to do that results in a
    // mutability error. Just as variables are immutable by default, so are references.
    //
    // some_string.push_str(", world");
    // ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    //
    // in order to borrow as mutable though, we can pass a mut flag on the variable
    {
        let mut s = String::from("hello");
        change(&mut s);

        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }
    }

    // Creating two mutable references to one variable fails
    //
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    //
    // The above code will result in
    //
    // let r1 = &mut s;
    //          ------ first mutable borrow occurs here
    //  let r2 = &mut s;
    //           ^^^^^^ second mutable borrow occurs here
    //  println!("{}, {}", r1, r2);
    //                     -- first borrow later used here

    // slices
    // given a string, a slice will return a pointer to the string, with a len being the distance
    // of the range.
    let s = String::from("hello");

    let _slice = &s[0..2];
    let _slice = &s[..2]; // these two are equal

    // as are these
    let len = s.len();
    let _slice = &s[0..len];
    let _slice = &s[0..];

    // we can also select the entire thing
    let _slice = &s[..];

    // so a way to get the first word would be
    #[allow(dead_code)]
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
}
