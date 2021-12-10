fn main() {
    let s = String::from("Hello world, my friends what are you doing tonight?");

    println!("The string is: {}", s);  // Apparently  println! macro takes implicit reference so no need to &s

    for n_word in 0..9 {

        let word = get_nth_word(n_word, &s);
        println!("The {}(st/nd/rd/th) word is {}\n", n_word, word);
    }

    let a = [0, 1, 2, 3, 4, 5];

    let b = &a[1..3];
    
    println!("{:?}", &b);
}

fn get_nth_word(n: u32, string: &str) -> &str {
    
    // Make byte array
    let byte_array = string.as_bytes();

    // Lenght of input string
    let str_len = string.len();
    
    // Count the words we have encountered within string
    let mut word_counter: u32 = 0;
    
    // Remember the start index of the las word
    let mut last_word_start_idx: usize = 0;
    
    for (idx, &byte_str) in byte_array.iter()
                                      .enumerate() {
        if byte_str == b' ' || idx == str_len - 1 {
            // We found a word
            word_counter += 1;
            
            if n == word_counter - 1 {
                // We arrived at the word we want to have
                let last = if idx == str_len - 1 {str_len} else {idx};
                return &string[last_word_start_idx..last];
            }

            last_word_start_idx = idx;
        }
    }
    
    &string[..]
}
