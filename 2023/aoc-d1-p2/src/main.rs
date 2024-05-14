use std::collections::HashMap;
use std::fs;
use std::env;


fn main() {
    let dir = env::current_dir().unwrap().display().to_string();
    let path:String = String::from(dir+"\\src\\input.txt");
    
    let lines = read_lines(path);

    let mut _val:Vec<u64> = vec![];
    let map:HashMap<String, i32> = HashMap::from([
        ("zero".to_string(), 0),
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);

    for i in 0..lines.len() {

        let mut s: usize = 0;
        let mut e: usize = lines[i].len()-1;
        let mut s_count:usize; // 0 Single, _ Letters -> Digit
        let mut e_count:usize; // 0 Single, _ Letters -> Digit
        let mut sd:bool;
        let mut ed:bool;
        while s <= e {

            (s_count, sd) = get_first_digit(&lines, s, i);
            (e_count, ed) = get_last_digit(&lines, s, e, i);
            
            if !sd { 
                s += 1;
            }
            
            if !ed { 
                e -= 1;
            }

            if sd && ed {
                let v1:u64;
                if s_count == 0 {
                    v1 = lines[i][s..s+1].trim().parse().unwrap();
                } else {
                    v1 = *(map.get(lines[i][s..=s+s_count].trim()).unwrap()) as u64;
                }
                let v2:u64;
                if e_count == 0 {
                    v2 = lines[i][e..e+1].trim().parse().unwrap();
                } else {
                    v2 = *(map.get(lines[i][e-e_count..=e].trim()).unwrap()) as u64;
                }
                _val.push(format!("{}{}", v1,v2).trim().parse().unwrap());
                break;
            }

        }

    }
    

    println!("{}", _val.iter().sum::<u64>());

}

fn read_lines(path: String) -> Vec<String> {
    return fs::read_to_string(path)
    .unwrap()
    .lines()
    .map(String::from)
    .collect();
}

fn get_first_digit(lines: &Vec<String>, s:usize, i:usize) -> (usize, bool) {
    if (lines[i]).as_bytes()[s].is_ascii_digit() {
        (0, true)
    } else if s+3 < lines[i].len() && vec!["one", "two", "six"].contains(&&lines[i][s..=s+2]) {
        // Check 3s {ONE, TWO, SIX}
        (2, true)
    } else if s+4 < lines[i].len() && vec!["zero", "four", "five", "nine"].contains(&&lines[i][s..=s+3]) {
        // Check 4s {ZERO, FOUR, FIVE, NINE}
        (3, true)
    } else if s+5 < lines[i].len() && vec!["three", "seven", "eight"].contains(&&lines[i][s..=s+4]) {
        // Check 5s {THREE, SEVEN, EIGHT}
        (4, true)
    } else {
        (0, false)
    }
}

fn get_last_digit(lines: &Vec<String>, s:usize, e:usize, i:usize) -> (usize, bool) {
    if (lines[i]).as_bytes()[e].is_ascii_digit() {
        (0, true)
    } else if e >= 2 && e-2 >= s && vec!["one", "two", "six"].contains(&&lines[i][e-2..=e]) {
        // Check 3s {ONE, TWO, SIX}
        (2, true)
    } else if e >= 3 && e-3 >= s && vec!["zero", "four", "five", "nine"].contains(&&lines[i][e-3..=e]) {
        // Check 4s {ZERO, FOUR, FIVE, NINE}
        (3, true)
    } else if e >= 4 && e-4 >= s && vec!["three", "seven", "eight"].contains(&&lines[i][e-4..=e]) {
        // Check 5s {THREE, SEVEN, EIGHT}
        (4, true)
    } else {
        (0, false)
    }
}