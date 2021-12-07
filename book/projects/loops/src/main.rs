fn main() {

    // Looping until break
    // Break counter
    let mut counter: i32 = 0;

    // Label a loop in order to be able to break from it specifically
    'counter: loop {
        // Print current counter
        println!("Oh boi, here we go counting: {}", counter);
        
        // Decrement this 
        let mut remainder = 10i8;

        // Make inner loopy boi
        'inner: loop {
            println!("Remaining = {}", remainder);
            
            if remainder == 7 {
                break 'inner;
            }
            
            if counter == 3 {
                break 'counter;
            }
            remainder -= 1;
            
        }

        counter += 1;
    }
    println!("Stop the count at {}", counter);

    loop_return(69);

    let mut reduce_me = 3;
    println!("Started at {}", reduce_me);
    while_loop(&mut reduce_me);
    println!("Now at {}", reduce_me);


    loop_collection();

    countdown(10);

}

fn loop_return(upper: i32) {
    let mut counter = 0i32;

    let res = loop {
        
        counter += 1;

        if counter == upper {
            break counter;
        }
    };  // Important ; because let is statement
    
    println!("Counter is {}", res);
}


fn while_loop(lim: &mut i32) {
    while *lim != 0 {
        println!{"{}!", lim};
        *lim -= 1;
    }
    println!("Reduced to atoms");
}

fn loop_collection() {
    // Make 5 zeros to loop through
    let a = [0i32; 5];
    
    let mut idx: usize = 0;

    while idx < 5 {
        println!("Value at index {} is {}", idx, a[idx]);
        idx += 1;
    }

    
    // Now nicer
    for elem in a {
        println!("Looping over elements directly: {}", elem);
    }
}   


fn countdown(n: i32) {
    for number in (0..n).rev() {  // This is a reversed range!
        println!("Liftoff in {}", number);
    }
}
