fn part1(input_str: &String) -> u32 {
    // Loop over each line of the input text and filter all numbers into a vector
    // Construct the respective calibration number from first and last element
    
    // Start with 0
    let mut calibration_sum: u32 = 0;

    // Loop over each line in the input string
    for line in input_str.lines() {
        
        // Collect all numbers in the
        let numbers = line.chars()
                           .filter(|c| c.is_digit(10))
                           .map(|c| c.to_digit(10).unwrap())
                           .collect::<Vec<u32>>();
        // Construct calibration number e.g. [1,7,5] results in "15" 
        calibration_sum += numbers[0] * 10 + numbers[numbers.len()-1];
         
    
    }
    calibration_sum
}

#[derive(Debug)]
struct Number {
    name: String,
    value: u32
}


fn str_starts_with_number(word: &str, numbers: &Vec<Number>) -> Option<u32> {
    for n in numbers.iter() {
        if word.starts_with(&n.name) {
            return Some(n.value)
        } 
    }
    None
}

fn str_ends_with_number(word: &str, numbers: &Vec<Number>) -> Option<u32> {
    for n in numbers.iter() {
        if word.ends_with(&n.name) {
            return Some(n.value)
        } 
    }
    None
}


fn part2(input_str: &String) -> u32 {
    // Basically same as part1 one but now we also look at writte numbers in each line
    // For each line we do a pre-processing step in which all written numbers are replace with actual digits
    // Then we apply part1
    
    // Create vector of Number structs, containig written name and value
    let numbers = "one two three four five six seven eight nine".split_whitespace()
                                                                .enumerate()
                                                                .map(|(i, n)| Number {name: n.to_string(), value: i as u32 + 1})
                                                                .collect::<Vec<Number>>();
    let mut calibration_sum: u32 = 0;
    
    // Loop over input
    for line in input_str.lines() {
        
        
        let mut first: u32 = 0;
        let mut last: u32 = 0;

        // Loop over chars in line
        for (i, c) in line.chars().enumerate() {

            // Check if first char is digit
            if c.is_digit(10) {
                first = c.to_digit(10).unwrap();
                break;
            }
            
            if let Some(x) = str_starts_with_number(&line[i..], &numbers) {
                first = x;
                break;
            }
        }

         // Loop over chars in line
//        let line_reverse = line.chars().rev().collect::<String>();
        for (j, k) in line.chars().rev().enumerate() {
//            println!("{}", &line[..line.len()-j]);
            // Check if first char is digit
            if k.is_digit(10) {
                last = k.to_digit(10).unwrap();
                break;
            }
            
            if let Some(y) = str_ends_with_number(&line[..line.len()-j], &numbers) {
                last = y;
                break;
            }
        
        }

        calibration_sum += first * 10 + last;

    }
   
    calibration_sum

}



fn main() {
    let input_file = "/home/leloup/git/rust_lang/advent_of_code/2023/day_1/input.txt";
    let input = String::from_utf8(std::fs::read(input_file).unwrap()).unwrap();
    let result_part1 = part1(&input);
    println!("{}", result_part1);
    let result_part2 = part2(&input);
    println!("{}", result_part2);
}
