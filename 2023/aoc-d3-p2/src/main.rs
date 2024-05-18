use std::collections::HashSet;
use std::fs;
use std::env;

fn main() {

    let symbols = ["*"];
    let dir = env::current_dir().unwrap().display().to_string();
    let path:String = dir + "\\src\\input.txt";
    
    let lines = read_lines(path);

    let mut values:Vec<(String, usize, (usize, usize))> = vec![];
    let mut signs:Vec<(String, usize, (usize, usize), Vec<String>)> = vec![];
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
                    signs.push((String::from(v), l, (start, end), vec![]));
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
                        signs.push((String::from(v), l, (start, end), vec![]));
                    } else {
                        values.push((String::from(v), l, (start, end)));
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
                    signs.push((String::from(v), l, (start, end), vec![]));
                } else {
                    values.push((String::from(v), l, (start, end)));
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
                signs.push((String::from(v), l, (start, end), vec![]));
            } else {
                values.push((String::from(v), l, (start, end)));
            }
        }
        
    }


    let mut sum: i64 = 0;
    for sign in signs.iter_mut() {
        let mut psbl: HashSet<(usize, usize)> = HashSet::new();
        for i in sign.2.0..sign.2.1 {
            psbl.insert((sign.1, i));
            if sign.1 > 0 {
                psbl.insert((sign.1 - 1, i));
                psbl.insert((sign.1 - 1, i + 1));
            }
            if i > 0 {
                psbl.insert((sign.1, i - 1));
                psbl.insert((sign.1 + 1, i - 1));
            }
            if sign.1 > 0 && i > 0 {
                psbl.insert((sign.1 - 1, i - 1));
            }
            psbl.insert((sign.1 + 1, i));
            psbl.insert((sign.1, i + 1));
            psbl.insert((sign.1 + 1, i + 1));
        }

        for val in values.iter() {
            if (val.2.0..val.2.1).any(|i| psbl.contains(&(val.1, i))) {
                sign.3.push(val.0.clone());
            }
        }

        if sign.3.len() >= 2 {
            let mul_vs: i64 = sign.3
                .iter()
                .map(|val| val.trim().parse::<i64>().unwrap())
                .product();
            sum += mul_vs;
        }
    }

    println!("{}", sum);

}

fn read_lines(path: String) -> Vec<String> {
    return fs::read_to_string(path)
    .unwrap()
    .lines()
    .map(String::from)
    .collect();
}