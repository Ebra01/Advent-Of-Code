use std::fs;
use std::env;


fn main() {
    let dir = env::current_dir().unwrap().display().to_string();
    let path:String = String::from(dir+"\\src\\input.txt");
    
    let lines = read_lines(path);

    let mut _val:Vec<u64> = vec![];

    for i in 0..lines.len() {

        let mut s: usize = 0;
        let mut e: usize = lines[i].len()-1;

        while s <= e {

            let sd = (lines[i]).as_bytes()[s].is_ascii_digit();
            let ed = (lines[i]).as_bytes()[e].is_ascii_digit();
            if !sd { 
                s += 1;
            }
            
            if !ed { 
                e -= 1;
            }

            if sd && ed { 
                let v1:u64 = lines[i][s..s+1].trim().parse().unwrap();
                let v2:u64 = lines[i][e..e+1].trim().parse().unwrap();
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