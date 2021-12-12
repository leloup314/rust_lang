// Defining my struct
// Disable warning if not all fields are used
#[allow(dead_code)]
struct CryptoCoin {
    is_shitcoin: bool,
    value: f32,
    name: String,
    amount: u64,
}


// Tuple struct
struct RGB(u8, u8, u8);
struct Point3D(i32, i32, i32);

fn main() {
    // Arbitrary order of key: value possible
    let doge_coin = CryptoCoin {
        name: String::from("DogeCoin"),
        is_shitcoin: true,
        value: 420.69,
        amount: 420,
    };

    println!("{} value is {} $", doge_coin.name, doge_coin.value);
    
    let mut cum_rocket = CryptoCoin {
        name: String::from("CumRocket"),
        is_shitcoin: true,
        value: 69.69,
        amount: 69,
    };
    
    println!("{} value is {} $", cum_rocket.name, cum_rocket.value);

    cum_rocket.value = 69420.69;

    println!("{} value is {} $", cum_rocket.name, cum_rocket.value);

    let eth = generate_new_coin(false, String::from("ETH"), 10_450.87, 666);

    println!("{} value is {} $", eth.name, eth.value);
    
    // Creating new struct by updating
    let doge_coin2 = CryptoCoin {
        name: String::from("Shib"),
        ..doge_coin  // Take the rest from the initial doge_coin instance; must come last!
    };
    
    println!("{} value is {} $", doge_coin2.name, doge_coin2.value);
    
    // This print only works because doge_coin2 has explicitly given a name which is a String that does not implement the copy trait
    // All other (bool, f32, U64) implement copy therefore doge_coin is still valid
    println!("{} value is {} $", doge_coin.name, doge_coin.value);
    
    let black = RGB(0,0,0);
    let origin = Point3D(0,0,0);
    
    // Destructure with corresponding type
    let Point3D(x, y, z) = origin;
    println!("{} {} {}", x, y, z);
    
    
}

fn generate_new_coin(is_shitcoin: bool, name: String, value: f32, how_much: u64) -> CryptoCoin {
    // This is an expression, therefore the instance is returned implicitly
    CryptoCoin {
        is_shitcoin,  // Shorthand notation because fn parameter and field have same name
        name,
        value,
        amount: how_much, // No shorthand becasue of different names
    }
}
