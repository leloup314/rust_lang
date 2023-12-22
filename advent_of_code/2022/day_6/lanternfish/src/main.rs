extern crate input_reader;


fn main() {
   
    // Initial state of our lanternfish population
    // The numbers are the age in days
    //let initial = vec![3, 4, 3, 1, 2];
    let initial = input_reader::load_lines_from_file("../../input.txt");
    let initial = initial[0].split(",")
                            .map(|number| number.parse::<u32>()
                                                .unwrap())
                            .collect::<Vec<u32>>();
    println!("Population count after 80 days is {}", part_one(&initial));
    println!("Population count after 256 days is {}", part_two(&initial));
}


fn part_one(initial: &Vec<u32>) -> u64 {
    // Determine for how long we want to grow the population
    let days: i32 = 80;    
    
    // Counter which holds the entire population
    // The index corresponds to the day until next reproduction
    // The value to the number of lanternfish with this reproduction counter
    let mut population = [0u64; 9];

    initialize_population(&initial, &mut population);
    
    for _day in 0..days {
       advance_population(&mut population);
    }
    
    population.iter().sum()
}


fn part_two(initial: &Vec<u32>) -> u64 {
    // Determine for how long we want to grow the population
    let days: i32 = 256;    
    
    // Counter which holds the entire population
    // The index corresponds to the day until next reproduction
    // The value to the number of lanternfish with this reproduction counter
    let mut population = [0u64; 9];

    initialize_population(&initial, &mut population);
    
    for _day in 0..days {
       advance_population(&mut population);
    }
    
    population.iter().sum()
}


fn initialize_population(initial: &Vec<u32>, population: &mut [u64]) {
    // Loop over initial
    for i in 0..initial.len() {
        let n = initial[i] as usize;
        population[n] += 1;
    }
}


fn advance_population(population: &mut [u64]) {
    // Advances *population* by one time unit (e.g. day)

    // Get population amount with reproduction timer 0
    // These fish will spawn new fish and re-enter the population with reproduction timer 6
    let reproducing_pop = population[0];

    // Loop over population with reproduction time > 0
    // These fish will not reprodice
    for rt in 1..population.len() {
        
        // Get number of fish with current rt
        let n_rt = population[rt];
        
        // Decrease them to one rt less, e.g. move the count to rt-1
        population[rt-1] += n_rt;

        // Reset population with current rt to 0
        population[rt] = 0;
    }
    
    // Correct how many fish will have reproduction timer 0  after this time step
    population[0] -= reproducing_pop;
    
    // Create new fishes with reproduction time 8 from the reproducing fishes of this time step
    population[8] += reproducing_pop;
    
    // Re-enter them to the reproductuion time 6 group
    population[6] += reproducing_pop;    
}
