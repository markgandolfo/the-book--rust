pub fn run() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condtion was false");
    }

    // numbers aren't truthy. you can't do this
    //
    //  if number {
    //     ^^^^^^ expected `bool`, found integer

    //
    // can do else if's
    //
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //
    // because if is an expression we can use let on the left
    //
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of nubmer is: {}", number);

    //
    // either true/false evaluated conditions need to be of the same type
    // otherwise the compiler compains.
    //
    // let number = if condition { 5 } else { "six" };
    //                                        ^^^^^ expected integer, found `&str`
    //

    //
    // LOOPS
    //
    loop {
        println!("This is a loop, but i'm breaking out so i don't crash");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //
    // You can give loops lablels, to distinguish when you're nesting them
    //
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //
    // While Loops
    //
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    //
    // Looping through an array
    //
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //
    // Looping through a range
    //
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
