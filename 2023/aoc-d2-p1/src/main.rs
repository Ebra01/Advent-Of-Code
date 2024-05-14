use std::collections::HashMap;
use std::fs;
use std::env;

fn main() {

    let mut sum:u32 = 0;
    let config:HashMap<&str, u32> = HashMap::from([
        ("green", 13),
        ("blue", 14),
        ("red", 12)
    ]);

    let dir = env::current_dir().unwrap().display().to_string();
    let path:String = String::from(dir+"\\src\\input.txt");
    
    let lines = read_lines(path);

    for line in lines.iter() {
        
        let raw_input = String::from(line);
        
        let split_input:Vec<&str> = raw_input.splitn(2, ':').collect();
        if split_input.len() != 2 {
            panic!("Invalid Input!!");
        }

        let game_id = split_input[0].trim().trim_start_matches("Game").trim().parse::<u32>().unwrap_or(0);
        sum += game_id;
        let input = split_input.get(1).unwrap_or(&"");
        let segments:Vec<&str> = input.split(";").collect();
    
        for segment in segments.iter() {
            let counts:HashMap<&str, u32> = segment.split(",")
            .map(|s: &str| s.trim())
            .filter_map(|s: &str| {
                let parts:Vec<&str> = s.split(" ").collect();
                match parts.as_slice() {
                    [count_str, color] => {
                        let count: u32 = count_str.parse::<u32>().ok()?;
                        Some((*color, count))
                    },
                    _ => None,
                }
            })
            .collect();
        
            let blue_count: &u32 = counts.get("blue").unwrap_or(&0);
            let green_count: &u32 = counts.get("green").unwrap_or(&0);
            let red_count: &u32 = counts.get("red").unwrap_or(&0);

            if blue_count > config.get("blue").unwrap() || red_count > config.get("red").unwrap() || green_count > config.get("green").unwrap() {
                sum -= game_id;
                break;
            }
    
            println!("Blue {}, Green: {}, Red: {}", blue_count, green_count, red_count);
    
        }
    
    }

    println!("Sum: {}", sum);


}

fn read_lines(path: String) -> Vec<String> {
    return fs::read_to_string(path)
    .unwrap()
    .lines()
    .map(String::from)
    .collect();
}