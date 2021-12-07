fn main() {

    let number = 420i32;

    if number < 69 {
        println!("Not nice");
    } else {
        println!("Maybe nice");
    }
    
    // This does not work! Rust needs a bool, does not implicitly try to convert int to bool such as e.g. Python
    // if number { }  // Check if number is non-zero
    // Instead
    if number != 0 {
        println!("Number is not 0")
    }

    // Multiple if else and else if
    let number: i32 = 69;

    if number % 69 == 0 {
        println!("Is nice number");
    } else if number % 420 == 0 {
        println!("Is also nice");
    } else {
        println!("Not nice")
    }

    // ternary operation
    let condition: bool = true;
    let val: i32 = if condition {69} else {420};
    println!("Value is {}", val);
}
