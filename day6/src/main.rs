#![warn(clippy::all)]

use itertools::Itertools;

fn main(){
    let input = include_str!("input_tst.txt");
    for line in input.lines(){
        for i in 4..line.len(){
            if line[i-4..i].chars().into_iter().unique().collect::<Vec<char>>().len() == 4{
                println!("The fist packet marker appears at index: {} and is {}", i, line[i-4..i].to_string());
                break;
            }
        }
        for i in 14..line.len(){
            if line[i-14..i].chars().into_iter().unique().collect::<Vec<char>>().len() == 14{
                println!("The fist message marker appears at index: {} and is {}", i, line[i-14..i].to_string());
                break;
            }
        }
    }
}
