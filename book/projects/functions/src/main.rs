fn main() {
    println!("Hello, world!");

    another_function(69);
    print_physical_quantity(420, "kg");
    statements_expressions();

    let nice: i32 = func_with_return_val();
    println!("{}, nice!", nice);

    let nice_again = add_one(68);
    println!("{}, nice!", nice_again);
}

fn another_function(x: i32) {
    println!("The value of x is {}.", x);
}

fn print_physical_quantity(quantity: i32, unit: &str) {
    println!("Physical quantity is {} {}", quantity, unit.to_string());
}

fn statements_expressions() {
    // Statements don't return anything
    let x: i32 = 420;

    // Expressions evaluate to a result
    let y = {
        let x = 3;
        x * x + 60  // This is an expression: They do not end with ; , otherwise they would be statements and return nothing
    };
    
    println!("Value of y is {}", y);
}


fn func_with_return_val() -> i32 {
    // Last expression is returned automatically
    // You can return early using return keyword
    35 + 34
}


fn add_one(x: i32) -> i32 {
    x + 1
}
