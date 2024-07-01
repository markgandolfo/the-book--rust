use std::net::IpAddr;

#[allow(dead_code)]
fn main() {
    // Enums are a way of expressing one of a possible set of values
    println!("");

    {
        enum IpAddrKind {
            V4,
            V6,
        }

        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }

    {
        // the above isn't very consise. We can do things better

        enum IpAddrKind {
            V4(String),
            V6(String),
        }

        let _home = IpAddrKind::V4(String::from("127.0.0.1"));
        let _loopback = IpAddrKind::V6(String::from(":1"));
    }

    {
        // we can implement functions on enums like we can structs.
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {
                // method body would be defined here
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        // Option is an enum that is used to determine whether there is some content or none.
        // it is used in place of null as rust doesn't have nulls
        let x: i8 = 5;
        let y: Option<i8> = Some(5);

        //
        // Option types won't resolve to the value.
        //
        // let sum = x + y;
        //   |           ^ no implementation for `i8 + Option<i8>`

        // instead
        // to get the values, there are a number of ways but the one that's preferred is the match
        //
        match y {
            Some(i) => println!("{}", x + i),
            None => eprintln!("Error"),
        }
    }

    {
        //
        // Match allows us to navigate enums to fetch data
        //
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
    }

    {
        //
        // Enums can take a type (as we saw earlier) and during match we can fetch the value
        //
        #[derive(Debug)] // so we can inspect the state in a minute
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {state:?}!");
                    25
                }
            }
        }
    }

    {
        // because of generics & enums, we can simply write code like this
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let _six = plus_one(five);
        let _none = plus_one(None);
    }

    {
        // if let syntax lets us combine if and let into a less verbose way to handle values that
        // match one pattern while ignoring the rest.
        //
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configure to be {max}"),
            _ => (),
        }

        // the following can be rewaritten as.
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {max}");
        } else {
            println!("something else");
        }
    }
}
