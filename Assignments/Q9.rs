use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> 
{
    let file = File::open("data.txt").expect("failed file");
    let reader = BufReader::new(file);
    let mut lines = 0;
    
    for _line in reader.lines() {
        lines += 1;
    }
    
    println!("data.txt has total of {} lines", lines);
    Ok(())
}
