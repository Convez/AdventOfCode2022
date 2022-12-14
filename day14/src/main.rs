#![warn(clippy::all)]

use itertools::Itertools;
use std::{cmp::{max,min}, fmt::{Display}};

#[derive(Debug,PartialEq, Eq,Clone, Copy)]
enum Tile{
    Air,
    Rock,
    MovingSand,
    RestingSand
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_print = match self{
            Tile::Air => ".",
            Tile::Rock => "#",
            Tile::MovingSand => "+",
            Tile::RestingSand => "o",
        };
        write!(f, "{}", to_print)
    }
}
//Returns number of stable sands
fn simulate(spots:&mut Vec<Vec<Tile>>, sand_start_x:usize,debug:bool )->usize{
    let mut reached_void = false;
    let mut stable_grains = 0;
    while !reached_void{
        // Create a new grain
        let mut grain = (0 as usize, sand_start_x, Tile::MovingSand);
        if spots[grain.0][grain.1] == Tile::RestingSand{
            break;
        }
        loop{ 
            if grain.0+1 >= spots.len(){
                reached_void = true;
                break;
            }
            let previous = grain.clone();
            let new_pos = can_move(spots,previous);
            match new_pos {
                Some(pos) => {grain.0 = pos.0; grain.1 = pos.1;},
                None => {grain.2 = Tile::RestingSand; stable_grains+=1;},
            }
            spots[previous.0][previous.1] = Tile::Air;
            spots[grain.0][grain.1] = grain.2;
            if debug{
                print_spots(spots);
                std::thread::sleep(std::time::Duration::from_millis(300));
            }
            if grain.2 == Tile::RestingSand{
                break;
            }
        }
    }
    stable_grains
}
//Returns the new position of the grain, if applicable
fn can_move(spots:&mut Vec<Vec<Tile>>, grain:(usize,usize,Tile)) -> Option<(usize,usize)>{
    if spots[grain.0+1][grain.1] == Tile::Air{
        return Some((grain.0+1,grain.1));
    }
    if grain.1 >0 && spots[grain.0+1][grain.1-1] == Tile::Air{
        return Some((grain.0+1,grain.1-1))
    }
    if grain.0+1<spots.len() && grain.1+1<spots[grain.0+1].len()&& spots[grain.0+1][grain.1+1] == Tile::Air{
        return Some((grain.0+1,grain.1+1))
    }
    None
}

fn main(){
    let input = include_str!("input.txt");
    let debug = false;
    let filled_spots = input.lines()
    .map(|l|l.trim().split("->")
        .map(|ti|
            {
                let mut s = ti.trim().split(","); 
                return(s.next().unwrap().parse::<u16>().unwrap(),s.next().unwrap().parse::<u16>().unwrap());
            }).collect_vec()
        ).collect_vec();

    let max_x = filled_spots.iter().flat_map(|i|i)
    .map(|i|i.0).max().unwrap();
    let max_y = filled_spots.iter().flat_map(|i|i)
    .map(|i|i.1).max().unwrap();
    let min_x = filled_spots.iter().flat_map(|i|i)
    .map(|i|i.0).min().unwrap();

    let span = max_x;
    let sand_start_x = 500 - min_x+span/2;

    let mut spots:Vec<Vec<Tile>> = Default::default();
    for i in 0..max_y+10{
        spots.push(Default::default());
        for _ in 0..max_x-min_x+span{
            spots[i as usize].push(Tile::Air);
        }
    }
    filled_spots.iter().for_each(|lines|{
        for i in 0..lines.len()-1{
            let item_i = lines[i];
            let item_next = lines[i+1];
            if item_i.0 == item_next.0{
                //Moving vertically
                for j in min(item_i.1, item_next.1)..=max(item_i.1,item_next.1){
                    spots[(j) as usize][(item_i.0-min_x+span/2) as usize] = Tile::Rock;
                }
            }else{
                //Moving horizontally
                for j in min(item_i.0, item_next.0)..=max(item_i.0,item_next.0){
                    spots[(item_i.1) as usize][(j-min_x+span/2) as usize] = Tile::Rock;
                }
            }
        }
    });
    let mut spots2 = spots.clone();
    let bottom_y =(max_y+2)as usize; 
    for x in 0..spots2[bottom_y].len(){
        spots2[bottom_y][x] =  Tile::Rock;
    }
    let stable_grains = simulate(&mut spots, sand_start_x as usize,debug);
    if debug{
        print_spots(&spots);
    }
    println!("The number of stable grains is: {}", stable_grains);
    
    let stable_grains2 = simulate(&mut spots2, sand_start_x as usize,debug);
    if debug{
        print_spots(&spots2);
    }
    println!("The number of stable grains with bottom is: {}", stable_grains2);
}

fn print_spots(spots: &Vec<Vec<Tile>>){
    for l in spots {
        for t in l{
            print!("{}",t);
        }
        println!();
    }
    println!();
}