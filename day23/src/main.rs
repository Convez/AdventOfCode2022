use std::collections::BTreeMap;

use itertools::Itertools;


#[derive(Debug,PartialEq, Eq,Clone, Copy,Default,PartialOrd, Ord)]
struct Elf{
    x:i32,
    y:i32
}
impl Elf {
    fn update(&mut self,other: &Elf){
        self.x = other.x;
        self.y = other.y;
    }
    fn get_nw(&self)->Elf{
        Elf{x:self.x-1,y:self.y-1}
    }
    fn get_n(&self)->Elf{
        Elf{x:self.x,y:self.y-1}
    }
    fn get_ne(&self)->Elf{
        Elf { x: self.x+1, y: self.y-1 }
    }
    fn get_e(&self)->Elf{
        Elf { x: self.x+1, y: self.y }
    }
    fn get_se(&self)->Elf{
        Elf { x: self.x+1, y: self.y+1 }
    }
    fn get_s(&self)->Elf{
        Elf { x: self.x, y: self.y+1 }
    }
    fn get_sw(&self)->Elf{
        Elf { x: self.x-1, y: self.y+1 }
    }
    fn get_w(&self)->Elf{
        Elf { x: self.x-1, y: self.y }
    }
    fn is_top_free(&self, elves:&Vec<Elf>)->bool{
        let to_check = vec![self.get_ne(),self.get_nw(),self.get_n()];
        to_check.iter().all(|e|!elves.contains(e))
    }
    fn is_right_free(&self, elves:&Vec<Elf>)->bool{
        vec![self.get_ne(),self.get_e(),self.get_se()].iter().all(|e|!elves.contains(e))
    }
    fn is_bottom_free(&self,elves:&Vec<Elf>)->bool{
        vec![self.get_se(),self.get_s(),self.get_sw()].iter().all(|e|!elves.contains(e))
    }
    fn is_left_free(&self,elves:&Vec<Elf>)->bool{
        vec![self.get_sw(),self.get_w(),self.get_nw()].iter().all(|e|!elves.contains(e))
    }

    fn propose_position(&self, elves: &Vec<Elf>, choice_shift:usize) -> Option<Elf> {
        let rotating_choices:Vec<fn(&Elf, &Vec<Elf>)->bool> = vec![Elf::is_top_free,Elf::is_bottom_free,Elf::is_left_free,Elf::is_right_free];
        let associated_direction:Vec<fn(&Elf,)->Elf> = vec![Elf::get_n,Elf::get_s,Elf::get_w,Elf::get_e];

        let is_direction_free = rotating_choices.iter().map(|f|f(self,&elves)).collect_vec();
        // println!("{:?}",is_direction_free);
        if is_direction_free.iter().all(|b|*b){
            return None;
        }
        for i in 0..rotating_choices.len(){
            if is_direction_free[(i+choice_shift)%4]{
                return Some(associated_direction[(i+choice_shift)%4](self));
            }
        }
        return None;
    }
}

fn print_elves(elves:&Vec<Elf>){
    let mut free = 0;
    let min_x = elves.iter().min_by_key(|e|e.x).unwrap().x;
    let min_y = elves.iter().min_by_key(|e|e.y).unwrap().y;
    let shifted_elves = elves.iter().map(|e|Elf{x:e.x-min_x,y:e.y-min_y}).collect_vec();
    let max_y = shifted_elves.iter().max_by_key(|e|e.y).unwrap().y+1;
    let max_x = shifted_elves.iter().max_by_key(|e|e.x).unwrap().x+1;
    let mut matrix = vec![vec!['.';max_x as usize];max_y as usize];
    shifted_elves.iter().for_each(|e|matrix[e.y as usize][e.x as usize]='#');
    for row in matrix{
        for col in row{
            if col =='.'{
                free +=1;
            }
            print!("{}",col);
        }
        println!();
    }
    println!("{}",free);
}

fn simulate(elves:&mut Vec<Elf>, rounds:usize){
    let mut first_choice = 0;
    for round in 0..rounds{
        println!("ROUND {}",round+1);
        let mut proposals:BTreeMap<Elf,Vec<Elf>> = Default::default();
        for elf in elves.iter(){
            if let Some(position) = elf.propose_position(&elves,first_choice){
                if let Some(proposal_elves) = proposals.get_mut(&position){
                    proposal_elves.push(*elf);
                }else{
                    proposals.insert(position, vec![*elf]);
                }
            }
        }
        if proposals.len()==0{
            println!("No elf moved after round {}", round+1);
            break;
        }
        proposals.iter().for_each(|(new_pos,proposal_elves)|{
            if proposal_elves.len()==1{
                let elf = proposal_elves.first().unwrap();
                if let Some(to_update)=elves.iter_mut().find(|e|e.x==elf.x && e.y == elf.y){
                    to_update.update(new_pos);
                }
            }
        });
        first_choice = first_choice+1;
    }
    print_elves(elves);
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};

    let mut elves = input.lines().enumerate().flat_map(|(y,l)|{
        l.trim().chars().enumerate()
        .filter_map(|(x,c)|if c=='#'{Some(Elf{x:x as i32,y:y as i32})}else{None}).collect_vec()
    }).collect_vec();
    let mut elves2 = elves.clone();
    //Problem 1
    simulate(&mut elves, 10);
    simulate(&mut elves2, usize::MAX);

}