// A number inside the schematic file
// It hold the actual value of the number, the index within the line and the number of digits
#[derive(Debug, PartialEq, Clone, Copy)]
struct SchematicNumber {
    value: usize,
    index: usize
}

impl SchematicNumber {
    // The number of digits of the value, useful for navigating inside the SchematicLine
    fn n_digits(&self) -> usize {
        self.value.to_string().len()
    }
}

// Struct to hold one line of the engione schematic
struct SchematicLine<'a> {
    line: &'a str
}

impl SchematicLine<'_> {

    // Method returning a Vec<SchemticNumber> within this line
    fn get_schematic_numbers(&self, symbol_idxs: Option<Vec<usize>>) -> Vec<SchematicNumber> {
        
        let mut schematic_numbers: Vec<SchematicNumber> = vec![];
        let mut line_chars = self.line.chars();
        let schematic_symbols = symbol_idxs.unwrap_or(self.get_symbol_idxs());

        let mut idx: usize = 0;
        while let Some(c) = line_chars.next() {
            
            if c.is_digit(10) {

                let mut n_len: usize = 1;

                while let Some(pd) = line_chars.next() {
                    if !pd.is_digit(10) {
                        break;
                    }
                    n_len +=1;
                }

                let actual_number: usize = self.line[idx..idx+n_len].parse().unwrap();

                let current_sn = SchematicNumber{value:actual_number,index:idx};
                
                let lower_idx = if current_sn.index <= 1 {0} else {current_sn.index - 1};
                let upper_idx = current_sn.index + current_sn.n_digits() + 1;
                
                // Check if we have symbols adjacent to number
                for sym in schematic_symbols.iter() {
                    if lower_idx <= *sym && *sym <= upper_idx {
                        schematic_numbers.push(current_sn);
                        break;
                    }
                }

                //schematic_numbers.push();

                idx += n_len;

            }

            idx += 1;
        }
        schematic_numbers
    }

    // Method returning a vector if indices of symol positions
    fn get_symbol_idxs(&self) -> Vec<usize> {

        let mut schematic_symbols: Vec<usize> = vec![];

        for (i, c) in self.line.chars().enumerate() {
            if !c.is_digit(10) && c != '.' {
                schematic_symbols.push(i)
            }
        }
        schematic_symbols
    }
    
}


fn part1(input: &String) -> usize {
    
    let mut engine_part_sum: usize = 0;

    let part_table = input.lines().filter(|x| !x.is_empty()).collect::<Vec<&str>>();

    let mut res: Vec<SchematicNumber> = vec![];

    for i in 1..part_table.len()-1  {

        res.clear();

        let line1: SchematicLine<'_> = SchematicLine{line:part_table[i-1]};
        let line2 = SchematicLine{line:part_table[i]};
        let line3 = SchematicLine{line:part_table[i+1]};

        res.extend(line2.get_schematic_numbers(None).iter());
        res.extend(line2.get_schematic_numbers(Some(line1.get_symbol_idxs())).iter());
        res.extend(line2.get_schematic_numbers(Some(line3.get_symbol_idxs())).iter());
        res.dedup();

        engine_part_sum += res.iter().map(|x| x.value).sum::<usize>();

        if i == 1 {
            res.clear();
            res.extend(line1.get_schematic_numbers(None).iter());
            res.extend(line1.get_schematic_numbers(Some(line2.get_symbol_idxs())).iter());
            res.dedup();

            engine_part_sum += res.iter().map(|x| x.value).sum::<usize>();
        }

        if i == part_table.len()-2 {
            res.clear();
            res.extend(line3.get_schematic_numbers(None).iter());
            res.extend(line3.get_schematic_numbers(Some(line2.get_symbol_idxs())).iter());
            res.dedup();
            
            engine_part_sum += res.iter().map(|x| x.value).sum::<usize>();
        }
    }
    engine_part_sum
    
}

fn main() {
    let input_path: &str = "/home/leloup/git/rust_lang/advent_of_code/2023/day_3/test.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let res_part1 = part1(&input);
    println!("{}", res_part1);
}
