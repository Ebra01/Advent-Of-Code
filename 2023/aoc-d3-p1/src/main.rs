use std::fs;
use std::env;

fn main() {

    let symbols = ["@", "#", "$", "%", "&", "*", "-", "=", "+", "/"];    
    let dir = env::current_dir().unwrap().display().to_string();
    let path:String = String::from(dir+"\\src\\input.txt");
    
    let lines = read_lines(path);

    let mut values:Vec<(String, usize, (usize, usize), bool)> = vec![];
    let mut signs:Vec<(String, usize, (usize, usize))> = vec![];
    let mut start:usize;
    let mut end:usize;
    let mut v:&str;
    let mut counter:i32 = 0;
    let mut i:usize;
    let mut c:char;
    for (l, input) in lines.iter().enumerate() {
        i = 0;
        while i < input.len() {
            c = input.chars().nth(i).unwrap();

            if !(c.is_ascii_digit() || symbols.contains(&c.to_string().as_str())) && counter == 0 {
                i += 1;
                continue;
            }

            if c.is_ascii_digit() {
                end = i;
                start = end - counter as usize;
                v = &input[start..end];
                if end - start == 1 && symbols.contains(&v) {
                    signs.push((String::from(v), l, (start, end)));
                    counter = 0;
                    continue;
                } else {
                    counter += 1;
                    i += 1;
                }
            }

            if symbols.contains(&c.to_string().as_str()) {
                if counter == 0 {
                    counter += 1;
                    i += 1;
                } else {
                    end = i;
                    start = end - counter as usize;
                    counter = 0;
                    v = &input[start..end];
                    if end - start == 1 && symbols.contains(&v) {
                        signs.push((String::from(v), l, (start, end)));
                    } else {
                        values.push((String::from(v), l, (start, end), false));
                    }
                    continue;
                }
            }

            if !(c.is_ascii_digit() || symbols.contains(&c.to_string().as_str())) && counter > 0 {
                end = i;
                start = end - counter as usize;
                counter = 0;
                v = &input[start..end];
                if end - start == 1 && symbols.contains(&v) {
                    signs.push((String::from(v), l, (start, end)));
                } else {
                    values.push((String::from(v), l, (start, end), false));
                }
                i += 1;
            }


        }

        if counter > 0 {
            end = i;
            start = end - counter as usize;
            counter = 0;
            v = &input[start..end];
            if end - start == 1 && symbols.contains(&v) {
                signs.push((String::from(v), l, (start, end)));
            } else {
                values.push((String::from(v), l, (start, end), false));
            }
        }
        
    }

    for val in values.iter_mut() {
        let mut dim:Vec<(usize, usize)> = Vec::new();
        let mut psbl:Vec<(usize, usize)> = Vec::new();
        for i in val.2.0..val.2.1 {
            dim.push((val.1, i));
        }
        for d in dim.iter() {
            if d.0 > 0 {
                // X-1
                psbl.push((d.0-1, d.1));
                // X-1 Y+1
                psbl.push((d.0-1, d.1+1)); 
            }
            if d.1 > 0 {
                // Y-1
                psbl.push((d.0, d.1-1));
                // X+1 Y-1
                psbl.push((d.0+1, d.1-1));
            }
            if d.0 > 0 && d.1 > 0 {
                // X-1 Y-1
                psbl.push((d.0-1, d.1-1));
            }
            // X+1
            psbl.push((d.0+1, d.1));
            // Y+1
            psbl.push((d.0, d.1+1));
            // X+1 Y+1
            psbl.push((d.0+1, d.1+1));
        }

        if psbl.iter().any(|&x| signs.iter().any(|y| x == (y.1, y.2.0))) {
            val.3 = true;
        }
    }

    let mut sum:i32 = 0;
    let mut value:i32;
    for val in values.iter() {
        if val.3 {
             value = val.0.trim().parse().unwrap();
             sum += value;
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