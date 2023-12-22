extern crate input_reader;


fn main() {
    let mv_cmds: Vec<String> = input_reader::load_lines_from_file("../../input.txt");
    //let mv_cmds = vec!["forward 5".to_string(), "down 5".to_string(), "forward 8".to_string(), "up 3".to_string(), "down 8".to_string(), "forward 2".to_string()];     
    println!("Final product of position is {}", part_one(&mv_cmds));
    println!("Final product of position is {}", part_two(&mv_cmds));
}

fn part_one(mv_cmds: &Vec<String>) -> i32 {
    // Create initial position
    let mut position: (i32, i32) = (0, 0);
    // Loop over commands
    for package in mv_cmds {
        // Get command and command data in 2 vector
        let cmd_data = package.split_whitespace().collect::<Vec<&str>>();

        let data: i32 = cmd_data[1].parse().unwrap();

        let cmd: &str = cmd_data[0];

        match cmd {
            "forward" => position.0 += data,
            "up" => position.1 -= data,
            "down" => position.1 += data,
            _ => (),
        }
    }
    position.0 * position.1
}


fn part_two(mv_cmds: &Vec<String>) -> i32 {
    // Create initial position
    let mut position: (i32, i32) = (0, 0);

    let mut aim: i32 = 0;

    // Loop over commands
    for package in mv_cmds {
        // Get command and command data in 2 vector
        let cmd_data = package.split_whitespace().collect::<Vec<&str>>();

        let data: i32 = cmd_data[1].parse().unwrap();

        let cmd: &str = cmd_data[0];

        match cmd {
            "forward" => {position.0 += data; position.1 += data * aim;}
            "up" => aim -= data,
            "down" => aim += data,
            _ => (),
        }
    }
    position.0 * position.1
}
