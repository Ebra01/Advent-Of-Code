use std::fs;
use std::env;
use std::path::Path;


fn main() {

    // let inputs:Vec<String> = vec![
    //     "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
    //     "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
    //     "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
    //     "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
    //     "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
    //     "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
    // ];

    let dir = env::current_dir().unwrap();
    let path = dir.join("src").join("input.txt");
    
    let inputs = read_lines(&path);

    let mut inst:String;
    let mut set:Vec<&str>;
    let mut winners:Vec<Vec<u32>> = vec![];
    let mut nums:Vec<Vec<u32>> = vec![];
    for (i, input) in inputs.iter().enumerate() {
        winners.push(vec![]);
        nums.push(vec![]);
        inst = input.split(":").last().unwrap().to_string();
        set = inst.split("|").collect();
        for el in set[0].split(" ").into_iter() {
            if el.parse().unwrap_or(-1) != -1 {
                winners[i].push(el.parse().unwrap_or_default());
            }
        }
        for el in set[1].split(" ").into_iter() {
            if el.parse().unwrap_or(-1) != -1 {
                nums[i].push(el.parse().unwrap_or_default());
            }
        }
    }

    let mut sum:u32 = 0;
    for (i, winner) in winners.iter().enumerate() {
        let mut points:u32 = 0;
        for w in winner.iter() {
            if nums[i].contains(w) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        sum += points;
    }

    println!("{}", sum);
    
}

fn read_lines(path: &Path) -> Vec<String> {
    return fs::read_to_string(path)
    .unwrap()
    .lines()
    .map(String::from)
    .collect();
}