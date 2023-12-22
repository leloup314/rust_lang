extern crate input_reader;

fn main() {
    let input_file = "/home/leloup/git/rust_lang/advent_of_code/2023/day_1/input.txt";
    let input = input_reader::load_lines_from_file(input_file); 
    let num_part_1 = calibrate_trebuchet_part_1(&input);
    let num_part_2 = calibrate_trebuchet_part_2(&input);
    println!("{}", num_part_1);
    println!("{}", num_part_2);
}


fn get_first_digit(word: &String, reverse: bool) -> Option<String> {
    if reverse == true {
        for tmp in word.chars().rev() {
            if tmp.is_digit(10) {
                return Some(String::from(tmp))
            }
        }
    } else {
        for tmp in word.chars() {
            if tmp.is_digit(10) {
                return Some(String::from(tmp))
            }
        }
    }
    None
}


fn get_calibration_value(word: &String, nums: Vec<&str>) -> i32 {
    // Scans "word" for occurance of number 1-9
    let mut idxs = Vec<i32>;
    
    for i in 0..nums.len() {
        println!("{}", word.find(&nums[i]));
    } 
    
}


fn calibrate_trebuchet_part_2(data: &Vec<String>) -> i32 {
    // Add vec of numbers that are valid numbers
    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut calibration_sum: i32 = 0;
    // Loop over the input and get calbration values
    for i in 0..data.len() {
        // Get the entry
        let word = &data[i];
        
        get_galibration_value(word, numbers);
    }
    calibration_sum
}


fn calibrate_trebuchet_part_1(data: &Vec<String>) -> i32 {
    let mut calibration_sum: i32 = 0;
    // Loop over the input and get calbration values
    for i in 0..data.len() {
        // Get the entry
        let word = &data[i];
        
        let first_digit = get_first_digit(word, false).unwrap();
        let last_digit = get_first_digit(word, true).unwrap();
	
	let calib: i32 = format!("{}{}", first_digit, last_digit).parse().unwrap();
        calibration_sum += calib;
    }
    calibration_sum
}
