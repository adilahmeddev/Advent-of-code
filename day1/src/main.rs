
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    num,
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let mut calories: Vec<u32> = Vec::new();
    let mut i = 0;
    calories.push(0);
    if let Ok(lines) = read_lines("lines.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    i += 1;
                    calories.push(0);
                    continue;
                }
                if let Ok(num) = ip.parse::<u32>() {
                    calories[i] += num;
                }
            }
        }
    } else if let Err(err) = read_lines("./lines.txt"){
        panic!("{:?}", err)
    }
    let mut highest = 0;
    for numz in calories {
        if numz > highest {
            highest = numz;
        }
    }
    print!("{}", highest)
}
