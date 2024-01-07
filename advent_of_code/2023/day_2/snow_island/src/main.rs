fn part1(input: &String) -> usize {
    
    let mut game_id_total: usize = 0;
 
    // Loop over each game
    for line in input.lines(){

        if !line.is_empty() {
        
            let mut game_data = line.split(":");

            let game_id: usize = game_data.next()
                               .unwrap()
                               .split_whitespace()
                               .last()
                               .unwrap()
                               .parse().unwrap();
            
            let game_sets = game_data.next().unwrap();
            
            let mut game_is_possible: bool = true;
            
            for gs in game_sets.split_terminator(";") {
                
                for colors in gs.split_terminator(",") {
                    
                    let mut clr = colors.split_whitespace();
                    let value: usize = clr.next().unwrap().parse().unwrap();
                    let clr = clr.next().unwrap();
                    
                    let current_limit: usize = match clr {
                                               
                                                   "red" => 12,
                                                   "green" => 13,
                                                   "blue" => 14,
                                                    _ => 0
                                               
                                               };
                    // This game is impossible
                    if current_limit < value {
                        game_is_possible = false;
                        break;
                    }
                }
                
                if !game_is_possible {
                    break;
                }
            }
            
            if game_is_possible {
                game_id_total += game_id;
            }
        }
    }


    game_id_total
}


fn part2(input: &String) -> usize {
    
    let mut power_total: usize = 0;
    
    #[derive(Debug)]
    struct ColorMaxVals {
        red: usize,
        green: usize,
        blue: usize
    }

    impl ColorMaxVals {
        fn power(&self) -> usize {
            self.red*self.green*self.blue
        }
        
        fn reset(&mut self) -> () {
            self.red = 0;
            self.green = 0;
            self.blue = 0;
        }
        fn update_max(&mut self, color: &str, value: usize) -> () {
            match color {
                "red" => { self.red = if value > self.red {value} else {self.red};},
                "green" =>  { self.green = if value > self.green {value} else {self.green};},
                "blue" =>  { self.blue = if value > self.blue {value} else {self.blue};},
                _ => ()
            }
        }
    }

    let mut color_max = ColorMaxVals { red: 0, green: 0, blue: 0 };            
 
    // Loop over each game
    for line in input.lines(){

        if !line.is_empty() {
        
            let game_data = line.split(":");
  
            let game_sets = game_data.last().unwrap();
            
            color_max.reset();
            
            for gs in game_sets.split_terminator(";") {

                for colors in gs.split_terminator(",") {
                    
                    let mut clr = colors.split_whitespace();
                    let value: usize = clr.next().unwrap().parse().unwrap();
                    let clr = clr.next().unwrap();

                    color_max.update_max(&clr, value);
                
                }
            }
            power_total += color_max.power();
        }
    }
    power_total
}




fn main() {
    // Input path
    let input_file = "/home/leloup/git/rust_lang/advent_of_code/2023/day_2/input.txt";
    
    // Parse input txt file
    let input = String::from_utf8(std::fs::read(input_file).unwrap()).unwrap();    
    
    let result_part1 = part1(&input);
    println!("{}", result_part1);
    let result_part2 = part2(&input);
    println!("{}", result_part2);
}
