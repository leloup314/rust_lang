const ELEMENTARY_CHARGE: f32 = 1.602e-19;


fn mutable() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    println!("The value of e is {}", ELEMENTARY_CHARGE);
}

fn shadowing() {
    let x = 5;
    let x  = x + 1;
        {
        let x = x * 2;
        println!("Value of x is in inner scope is {}", x);
        }
    println!("Value of x is in outer scope is {}", x);
}

fn reuse_with_type_conversion() {
    let spaces = "     ";  // spaces is String
    let spaces = spaces.len();  // spaces is int
    println!("Number of spaces {}", spaces);
}

fn main() {
    mutable();
    shadowing();
    reuse_with_type_conversion()
}
