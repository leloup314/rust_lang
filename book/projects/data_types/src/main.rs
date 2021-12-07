fn num_ops() {
    // addition
    let sum = 34 + 35;

    // subtraction
    let diff = 70 - 1;

    // multiplictaion
    let prod = 21 * 20;

    // division
    let quot = 12.6 / 4.0;

    // integer div
    let rem = 2 / 3;

    // modulo
    let modulo = 420 % 69;
}

fn compounds() {
    // Tuples and arrays
    let tup: (i32, f64, u8) = (69, 420.69, 69);

    // Destructuring a tuple into its elements
    let (e1, e2, e3) = tup;

    println!("{} {}", e1, e2);

    // Nice index-based acces to tuple elements
    let nice = tup.2;
    let very_nice = tup.1;
    println!("{} {}", nice, very_nice);

    // Unit type and unit value
    let unit_type = ();

    // Arrays have the same data type!
    let array = [1, 2, 3, 4];
    let months = ["Pink", "Guy", "Franku"];

    // Declare array type
    let array_u8: [u8; 3] = [253, 254, 255];

    // Array of same elements; 420 length, fill value 69 float 32
    let same = [69f32; 420];

    let first = same[0];
    let second = same[1];

}

fn main() {
    // Integer types
    let integer_1_ubyte: u8 = 255;
    let integer_1_byte: i8 = 127;
    println!("My integers: {} {}\n", integer_1_ubyte, integer_1_byte);
    
    // Same but shorter
    let integer_2_ubyte = 1_000u16;
    let integer_1_byte = 127i8;
    println!("My integers: {} {}\n", integer_2_ubyte, integer_1_byte);

    // Floats
    let float = 3.1415;  // 64 bit
    let float_4_byte: f32 = 3.1415;  // 32 bit
    let float_4_byte_other = 3.1415f32;  // 32 bit
    println!("My floats: {} {} {}\n", float, float_4_byte, float_4_byte_other);
   
    // Bool
    let f: bool = false;
    let t: bool = true;
    let impl_t = true;
     
    num_ops();

    // Char
    let c = 'A';

    // String; see double quotes
    let s = "A";

    println!("{} {}", c, s);

    compounds();
}
