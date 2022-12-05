#![warn(clippy::all)]

use std::collections::LinkedList;


fn main(){
    let input = include_str!("input.txt");
    let mut are_crates_over = false;
    let mut stacks_9000: Vec<LinkedList<char>> = Default::default();
    let mut stacks_9001: Vec<LinkedList<char>> = Default::default();
    
    for line in input.lines(){
        if line.is_empty(){
            are_crates_over = true;
            stacks_9001 = stacks_9000.clone();
        }
        if !are_crates_over{
            let mut i = 0;
            // Here I split the lines in groups of 4 ([N]<emptyspace or newline>)
            line.chars().collect::<Vec<char>>().chunks(4)
            .for_each(|chunk|{
                if stacks_9000.get(i).is_none(){
                    stacks_9000.push(Default::default());
                }
                if chunk[1] != ' ' && !chunk[1].is_numeric(){
                    match stacks_9000.get_mut(i){
                        Some(list) => {list.push_front(chunk[1].clone())},
                        None => todo!(),
                    }
                }
                i+=1;
            });
        }else {
            if !line.is_empty(){
                let scan = sscanf::sscanf!(line.trim(), "move {} from {} to {}",usize,usize,usize);
                match scan {
                    Ok((amount,from,to)) => {
                        let mut help: LinkedList<char> = Default::default();
                        for _ in 0..amount{
                            let c = stacks_9000.get_mut(from-1).unwrap().pop_back().unwrap();
                            stacks_9000.get_mut(to-1).unwrap().push_back(c);
                            let c1 = stacks_9001.get_mut(from-1).unwrap().pop_back().unwrap();
                            help.push_front(c1);
                        }
                        stacks_9001.get_mut(to-1).unwrap().extend(help.iter());
                    },
                    Err(err) => println!("{}",err.to_string()),
                }
            }
        }
    }
    stacks_9000.iter().for_each(|s|print!("{}",s.back().unwrap()));
    print!("\n");
    stacks_9001.iter().for_each(|s|print!("{}",s.back().unwrap()));
    print!("\n");
}
