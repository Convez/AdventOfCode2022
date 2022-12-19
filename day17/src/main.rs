use std::{collections::HashSet};

use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;


type Pos = (u64,u64);

#[derive(Debug,Clone)]
struct Tetromino{
    shape: Vec<Pos>,
    shape_type: Shapes
}
impl Tetromino {
    fn get_top_y(&self) -> u64{
        self.shape.iter().map(|s|s.1).max().unwrap()
    }
    fn can_move_horizontally(&self, max_x: u64, direction:JetStream, occupied_spaces: &HashSet<Pos>) -> bool{
        let xs = self.shape.iter().minmax_by_key(|s|s.0).into_option().unwrap();
        if direction == JetStream::Left && xs.0.0 == 0{
            return false;
        }
        if direction == JetStream::Right && xs.1.0 == max_x{
            return false;
        }
        let add_or_sub:i128 = if direction == JetStream::Left {-1} else {1};
        !self.shape.iter().map(|p|((p.0 as i128 + add_or_sub) as u64,p.1))
        .any(|p|occupied_spaces.contains(&p))
    }
    fn move_horizontally(&mut self, max_x: u64, direction:JetStream, occupied_spaces: &HashSet<Pos>){
        if self.can_move_horizontally(max_x, direction, occupied_spaces){
            self.shape.iter_mut().for_each(|p|{
                if direction == JetStream::Left{
                    p.0 -= 1;
                }else{
                    p.0 += 1;
                }
            });
        }
    }
    fn can_move_vertically(&mut self, occupied_spaces: &HashSet<Pos>)->bool{
        !self.shape.iter().map(|p|(p.0,p.1-1))
        .any(|p|occupied_spaces.contains(&p))
    }
    fn move_vertically(&mut self){
        self.shape.iter_mut().for_each(|p|{
            p.1-=1;
        })
    }
}

#[derive(Debug,Clone, Copy,PartialEq, Eq)]
enum JetStream{
    Left,
    Right
}
#[derive(Debug,Clone,Copy,FromPrimitive,PartialEq, Eq)]
enum Shapes {
    HorizontalLine = 0,
    Plus,
    LShape,
    VerticalLine,
    Square
}

fn pattern_found(cycle_detector: &Vec<(Vec<JetStream>, Tetromino)>) -> Option<Vec<(Vec<JetStream>, Tetromino)>>{
    let mut cycle: Vec<(Vec<JetStream>, Tetromino)> = Default::default();
    for (commands,tetromino) in cycle_detector.iter().rev(){
        if cycle.is_empty(){
            // First element
            cycle.push((commands.clone(),tetromino.clone()));
        }else {
            if *commands == cycle.first().unwrap().0 && tetromino.shape_type == cycle.first().unwrap().1.shape_type{
                //Found a cycle
                // Let's verify the found cycle is actually repeating
                if cycle_detector.len()<=cycle.len()*2{
                    return None;
                }
                for i in 0..cycle.len(){
                    if cycle_detector[cycle_detector.len()-cycle.len()-i-1].0 != cycle[i].0 ||
                        cycle_detector[cycle_detector.len()-cycle.len()-i-1].1.shape_type != cycle[i].1.shape_type{
                            return None;
                        } 
                }
                return Some(cycle);
            }else{
                cycle.push((commands.clone(),tetromino.clone()));
            }
        }
    }
    None
}

fn calculate_final_height(cycle: Vec<(Vec<JetStream>, Tetromino)>, target_tetrominos: u64, current_tetrominos:u64)->u64{
    // Cycle contains cycle_size tetrominos
    let cycle_size = cycle.len();
    println!("Cycle size is {}", cycle_size);
    let remaining_tetrominos = target_tetrominos - current_tetrominos + cycle_size as u64;
    println!("We processed {} tetrominos. Remaining {} to process", current_tetrominos, remaining_tetrominos);
    let remaining_cycles_int = remaining_tetrominos/cycle_size as u64;
    let remaining_cycles_reminder = remaining_tetrominos%cycle_size as u64;
    println!("There are {} cycles left, with a remainder of {}", remaining_cycles_int, remaining_cycles_reminder);
    let cycle_shapes:HashSet<Pos> = cycle.iter().flat_map(|i|i.1.shape.clone()).collect();
    let cycle_max_height = cycle_shapes.iter().max_by_key(|i|i.1).unwrap().1;
    let cycle_min_height = cycle_shapes.iter().min_by_key(|i|i.1).unwrap().1;
    let cycle_height = cycle_max_height - cycle_min_height;
    println!("The cycle has a max y of {} and a min y of {}, making its height {}", cycle_max_height,cycle_min_height, cycle_height);
    let end_height = cycle_height * remaining_cycles_int;
    println!("The height delta at the end of all cycles is {}", end_height);
    let mut last_cycle = cycle.iter().map(|i|i.1.clone()).collect_vec();
    last_cycle.iter_mut().for_each(|i|i.shape.iter_mut().for_each(|p|p.1+=end_height));
    let mut last_cycle_shapes:HashSet<Pos> = last_cycle.iter().flat_map(|i|i.shape.clone()).collect();
    if remaining_cycles_reminder !=0{
        for i in 0..remaining_cycles_reminder as usize{
            let extended_shape = last_cycle[last_cycle.len()-i-1].shape.iter().map(|i|(i.0,i.1+cycle_height)).collect_vec();
            last_cycle_shapes.extend(extended_shape.iter());
        }
    }
    let final_max_height = last_cycle_shapes.iter().max_by_key(|i|i.1).unwrap().1;
    println!("Final height is: {}", final_max_height);
    final_max_height
}

fn simulate_problem_1(jet_pushes: &Vec<JetStream>, target_tetrominos: u64, max_x: u64) -> (u64,u64){
    let mut current_tetrominos = 0;
    let mut new_tetromino = Shapes::HorizontalLine;
    let mut command = jet_pushes.iter().cycle();
    let mut occupied_spots: HashSet<Pos> = vec![(0,0),(1,0),(2,0),(3,0),(4,0),(5,0),(6,0)].iter().map(|(a,b)|(*a,*b)).collect();
    let mut current_max_y = 0;
    // This contains a vector
    let mut cycle_detector: Vec<(Vec<JetStream>, Tetromino)> = Default::default();
    while current_tetrominos < target_tetrominos{
        let mut tetromino = make_tetromino(new_tetromino, current_max_y +4);
        let mut commands :Vec<JetStream> = Default::default();
        loop {
            let current_command = command.next().unwrap();
            commands.push(current_command.clone());
            tetromino.move_horizontally(max_x, *current_command,&occupied_spots);
            if tetromino.can_move_vertically(&occupied_spots) {
                tetromino.move_vertically();
            }
            else {
                break;
            }
        }
        current_max_y = std::cmp::max(current_max_y, tetromino.get_top_y());
        occupied_spots.extend(tetromino.shape.iter());
        current_tetrominos += 1;
        cycle_detector.push((commands,tetromino.clone()));
        let pattern = pattern_found(&cycle_detector);
        if let Some(cycle) = pattern{
            let final_max_height = calculate_final_height(cycle, target_tetrominos, current_tetrominos);
            return (target_tetrominos, final_max_height);
        }
        new_tetromino = FromPrimitive::from_u64((new_tetromino as u64 + 1) % 5).unwrap();
    }
    (current_tetrominos,current_max_y)
}

fn make_tetromino(shape_type: Shapes, y:u64)->Tetromino{
    match shape_type {
        Shapes::HorizontalLine => Tetromino { shape: vec![(2,y),(3,y),(4,y),(5,y)],shape_type},
        Shapes::Plus => Tetromino { shape: vec![(2,y+1),(3,y+1),(4,y+1),(3,y+2),(3,y)],shape_type},
        Shapes::LShape => Tetromino { shape: vec![(2,y),(3,y),(4,y),(4,y+1),(4,y+2)],shape_type},
        Shapes::VerticalLine => Tetromino { shape: vec![(2,y),(2,y+1),(2,y+2),(2,y+3)],shape_type},
        Shapes::Square => Tetromino { shape: vec![(2,y),(3,y),(2,y+1),(3,y+1)],shape_type},
    }
}

fn main(){
    let is_test = true;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let jet_pushes = input.lines().next().unwrap().trim().chars()
        .map(|c|if c == '<' {JetStream::Left} else {JetStream::Right}).collect_vec();

    let (stopped_tetrominos,max_height) = simulate_problem_1(&jet_pushes, 2022, 6);
    println!("{} tetrominos stopped", stopped_tetrominos);
    println!("Highest point is at {}",max_height);

    let (stopped_tetrominos2,max_height2) = simulate_problem_1(&jet_pushes, 1000000000000, 6);
    println!("{} tetrominos stopped", stopped_tetrominos2);
    println!("Highest point is at {}",max_height2);
    
}