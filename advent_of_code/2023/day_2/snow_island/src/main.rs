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




fn main() {
    // Input path
    let input_file = "/home/leloup/git/rust_lang/advent_of_code/2023/day_2/input.txt";
    
    // Parse input txt file
    let input = String::from_utf8(std::fs::read(input_file).unwrap()).unwrap();    
    
    let result_part1 = part1(&input);
    
    println!("{}", result_part1);
}
