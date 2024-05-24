use std::env;
use std::fs;
use std::path::Path;

fn main() {
    
    let dir = env::current_dir().unwrap();
    let path = dir.join("src").join("input.txt");
    
    let input = read_lines(&path);

    let seeds:Vec<i64> = input[0].split(":")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()[1..]
        .iter()
        .map(|x| x.parse().unwrap_or_default())
        .collect();

    let mut mapping:Vec<Vec<(i64, i64, i64)>> = vec![];

    let mut index:usize = 0;
    let mut is_pattern = false;
    for line in input[2..].iter() {
        if line.eq("") {
            is_pattern = false;
            index += 1;
            continue;
        }
        if line.starts_with(&['s', 'f', 'w', 'l', 't', 'h']) {
            is_pattern = true;
            mapping.push(vec![]);
            continue;
        }

        if is_pattern {

            let nums:Vec<i64> = line.split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse().unwrap_or_default())
                .collect();
            
            let range = nums[2].clone();
            let source = nums[1].clone();
            let output = nums[0].clone();
            mapping[index].push((source, output, range));
        }
    }

    let mut locations:Vec<i64> = vec![];
    let mut src:i64;
    for seed in seeds.iter() {
        src = *seed;
        for map in mapping.iter() {
            for tup in map.iter() {

                if (tup.0..tup.0+tup.2).contains(&src) {
                    src = tup.1 + (src - tup.0);
                    break;
                }
            }
        }
        locations.push(src);

    }

    println!("{:?}", locations.iter().min().unwrap());

}


fn read_lines(path: &Path) -> Vec<String> {
    return fs::read_to_string(path)
    .unwrap()
    .lines()
    .map(String::from)
    .collect();
}