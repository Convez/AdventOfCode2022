#![warn(clippy::all)]

use std::collections::LinkedList;
use std::fmt::Display;



struct Elf{
    pub elf_id: usize,
    pub food_calories: LinkedList<u64>
}

impl Elf {
    pub fn total_calories(&self) -> u64{
        self.food_calories.iter().sum()
    }
}
impl Default for Elf{
    fn default() -> Self {
        Self { food_calories: Default::default(), elf_id: Default::default() }
    }
}
impl Display for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id: {} Calories: {}", self.elf_id, self.total_calories())
    }
}

fn populate_elves(input: &str, elves: &mut Vec<Elf>){
    let mut new_elf = true;
    for line in input.lines(){
        if line.is_empty(){
            new_elf = true;
        }else{
            if new_elf{
                let mut elf: Elf = Default::default();
                elf.elf_id = elves.len() +1;
                elves.push(elf);
            }
            new_elf = false;
            let elf = elves.last_mut().unwrap();
            elf.food_calories.push_back(line.parse().unwrap());
        }
    }
}

fn main(){
    let input = include_str!("input.txt");
    let mut elves: Vec<Elf> = Default::default();
    populate_elves(input, &mut elves);
    elves.sort_by_key(|x|x.total_calories());
    elves.reverse();

    println!("The elf with most calories is: {}", elves.first().unwrap());
    println!("The elves with the most calories are:\n- {}\n- {}\n- {}.\nThey have a total of {} calories.", 
    elves.first().unwrap(), elves.get(1).unwrap(), elves.get(2).unwrap(), elves[0..3].iter().map(|e|e.total_calories()).sum::<u64>());
    
}
