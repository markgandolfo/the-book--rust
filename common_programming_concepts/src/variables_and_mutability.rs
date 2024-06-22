pub fn run() {
    // multiple assignments to the same variable is called shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // when shadowing we can change the type, i.e. string to usize
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}\n");
}
