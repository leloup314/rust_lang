extern crate input_reader;


fn main() {
    // Test input; should yield 7
    //let depths = vec![199,200,208,210,200,207,240,269,260,263];
    // Load actual input; the depths as integers from input file
    let depths: Vec<i64>  = input_reader::load_ints_from_file("../../input.txt");

    println!("Counter is at {}", part_one(&depths));
    println!("Counter is at {}", part_two(&depths));
}


fn part_one(depths: &Vec<i64>) -> u64 {

    let mut counter = 0u64;

    for i in 1..depths.len() {
        if depths[i] > depths[i-1] {
            counter += 1;
        }
    }
    counter
}

fn part_two(depths: &Vec<i64>) -> u64 {
    
    let mut counter = 0u64;
    
    let mut current_sum: i64 = 0;

    let mut prev_sum: i64 = -1;

    let depths_len = depths.len();

    for i in 0..depths_len {
        if i <= depths_len - 3 {
            current_sum = depths[i..i+3].iter().sum();
            if prev_sum < current_sum {
                counter += 1;    
            }
            prev_sum = current_sum;
        }
    }
    // Reduce counter by one because we counted iteration 0
    counter-=1;
    counter
}
