pub fn run() {
    // functions look like any other function.
    // and can be called the same way
    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }

    print_labeled_measurement(5, 'h');

    // Statesments & Expressions:
    //
    // statements perform an action that don't return a value
    // expressions evaluate to a resultant value

    // Statements do not return values.
    // Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error
    //
    // let x = (let y = 6);
    //
    // But we can write an expression
    let y = {
        let x = 3;
        let _ = x + 1;
    };
    println!("The value of y is: {:?}", y);
}
