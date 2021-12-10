use std::time::Instant;

fn main() {
   
    // Initial state of our lanternfish population
    // The numbers are the age in days
    let initial: [i32; 5] = [3, 4, 3, 1, 2];

    // Determine for how long we want to gorw the population
    let days = 80i32;

    // Counter which holds the entire population
    // The index corresponds to the day until next reproduction
    // The value to the number of lanternfish with this reproduction counter
    let mut population = [0i32; 9];

    initialize_population(&initial, &mut population);
    
    let now = Instant::now();
    calculate_population(&mut population, days);
    let nsecs = now.elapsed().as_nanos();

    let population_count: i32 = population.iter().sum();

    println!("Final population after {} is {:?}", days, population);
    println!("Final population count after {} is  {}", days, population_count);
    println!("Population calculated in {} ns", nsecs);
}


fn initialize_population(initial: &[i32], population: &mut [i32]) {
    // Loop over initial
    for i in 0..initial.len() {
        let n = initial[i] as usize;
        population[n] += 1;
    }
    println!("Init population to  {:?}", population);
}


fn calculate_population(population: &mut [i32], n_days: i32) {
    
    // Loop over the days to advance the population
    for _day in 0..n_days {
        
        // Store how many fish will reproduce this day
        let zero_pop = population[0];
        
        // Loop over the reproduction times in the population which will not repriduce this cycle
        for rt in 1..population.len() {
            let n_pop = population[rt];
            
            // Decrease the reproduction time for all fish at current index by 1
            population[rt-1] += n_pop;
            population[rt] -= n_pop;
        }        
        // Calculate the remaining fish with reproduction time 0 after this day 
        population[0] = population[0] - zero_pop;
        // Create zero_pop new fishes with reproduction time 8 from the reproducing fishes of his day
         population[8] += zero_pop;
        // Re-enter them to the reproductuion time 6 group
        population[6] += zero_pop; 
        
        //println!("Population after day {}:  {:?}", day+1, population);
    }
}
