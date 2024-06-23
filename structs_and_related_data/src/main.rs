#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i16,
}

fn main() {
    // structs can be defined by
    let user = User {
        active: true,
        username: String::from("mark"),
        email: String::from("me@example.com"),
        sign_in_count: 1,
    };

    // structs can be copied using shorthand syntax
    //
    // NOTE: Since we've copied the "username" string from user1 to user2, it has been copied so,
    // user is no longer available to access. :boom:.. This isn't true for sign_in_count and
    // active since both fields implement the "copy" trait (as stipulated in the previous chapter)
    //
    let _user2 = User {
        email: String::from("you@example.com"),
        ..user
    };

    //
    // Tuple Structs
    //
    #[allow(dead_code)]
    struct Color(i32, i32, i32);
    #[allow(dead_code)]
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    //
    // Unit Structs
    //
    struct AlwaysEquals;

    let _subject = AlwaysEquals;

    //
    // Structs need to own their own data, and will be valid until the struct is.
    //
    // Structs with &str won't work and will through a "expected named lifetime parameter"
    // So use String instead
    //

    //
    // We can put dbg! statements which only exectute when debug is written!
    //
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let scale = 2;
        let rect = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        println!("{}x{}", rect.width, rect.height);
        dbg!(&rect);
    }

    //
    // We can implement functions with the same name as struct.fiels
    // what's interesting here is that is we do something like return a bool, we can use it in a if
    // block and Rust will know to call the function instead of accessing the field!
    //
    {
        #[allow(dead_code)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            //
            // We can also define associated functions that don't take self as a parameter
            //
            fn square(size: u32) -> Self {
                Self {
                    width: size,
                    height: size,
                }
            }

            fn width(&self) -> bool {
                self.width > 0
            }

            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        let rec2 = Rectangle {
            width: 40,
            height: 60,
        };

        let _rec3 = Rectangle::square(10);

        println!("The area of a rect is {} square pixels", rect.area());

        if rect.width() {
            println!("The rectangle has a nonzero width; it is {}", rect.width);
        }

        if rect.can_hold(&rec2) {
            println!("The rectangle can hold rec2");
        }
    }
}
