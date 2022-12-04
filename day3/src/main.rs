#![warn(clippy::all)]
use itertools::Itertools;

fn main(){
    let input = include_str!("input.txt");
    let mut score: u64 = 0;
    for line in input.lines(){
        let (half1,half2) = line.split_at(line.len()/2);
        let common = half1.chars().find(|c|half2.contains(*c)).unwrap();
        score += (common.to_lowercase().next().unwrap() as u64 - 'a' as u64) +1+ 26*common.is_uppercase() as u64;
    }
    println!("The total priority of the misplaced item types is {}", score);
    let mut score2:u64 = 0;
    let mut lines: Vec<&str> = Default::default();
    input.lines().for_each(|l|lines.push(l));
    lines.chunks(3).for_each(|chunk|chunk[0].chars().into_iter().unique().for_each(
        |c|{
            if chunk[1].contains(c) && chunk[2].contains(c){
                score2 +=(c.to_lowercase().next().unwrap() as u64 - 'a' as u64) +1+ 26*c.is_uppercase() as u64;
            }
        }
    ));
    println!("The total priorities for the badges is: {}", score2);
}
