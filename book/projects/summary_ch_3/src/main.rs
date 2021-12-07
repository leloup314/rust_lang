fn main() {
    convert_temps(31., 'C');
    convert_temps(110., 'F');
    println!("Temperature is {} F", convert_temps_fancy(31., 'C'));
    println!("Temperature is {} C\n", convert_temps_fancy(110., 'F'));

    println!("8th Fib. number is {}\n", fibonacci(8));
    
    print_n_days_of_christmas(13);
    print_n_days_of_christmas(12);
}


fn convert_temps(temp: f32, unit: char) -> f32 {
    
    // Store result
    let mut res = 0f32;

    // Check what unit our temperature is in
    if unit == 'C' {
        res = temp * 1.8 + 32.0;
        println!("Temperature is  {} F", res);
    } else if unit == 'F' {
        res = (temp - 32.0) * 5.0 / 9.0;
        println!("Temperature is  {} C", res);
    } else {
        println!("Unit must be either 'C' or 'F'");
    }
    
    res
}


fn convert_temps_fancy(temp: f32, unit: char) -> f32 {
    
    if unit == 'C' {
        temp * 9. / 5. + 32.
    } else {
        (temp - 32.) * 5. / 9.
    } 
    
}


fn fibonacci(number: i32) -> i32 {
    // nth Fib. number
    if number <= 2 {
        1
    } else {
        fibonacci(number - 1) + fibonacci(number - 2)
    }
}

fn print_n_days_of_christmas(n:usize) {
    // Gives up to nth "Twelve das of Christmas" verse
    
    if n > 12 {
        println!("Twelve Days Of Christmas has only 12 days, not {}", n);
        return ()
    }
    let days = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth", "Elevnth", "Twelth"];

    let verses = ["All their good wishes,",
                  "Gifts for one and all,",
                  "Some mistletoe,",
                  "A guardian angel,",
                  "Gold and silver tinsel,",
                  "Candles a-glowing,",
                  "Little silver bells,",
                  "A shining star,",
                  "Four colored lights,",
                  "Three boughs of holly,",
                  "Two candy canes,"];

    for day in 0..n {
        
        // Print first lines
        println!("On the {} day of Christmas", days[day]);
        println!("My good friends brought to me,");

        // Fill the missing verses
        for current in (11 - day)..11 {
            println!("{}", verses[current]);
        }        
        
        // Print last line
        println!("{}\n", if day == 0 {"A song and a Christmas tree."} else {"And a song for the Christmas tree."});
    }
}

