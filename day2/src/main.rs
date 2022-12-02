#![warn(clippy::all)]

use std::{collections::HashMap, rc::Rc};

#[derive(Debug,Clone, Copy,Hash,PartialEq, Eq)]
enum GameMove{
    Rock = 1,
    Paper = 2,
    Scissors = 3
}
struct Round{
    elf_move: GameMove,
    your_move: GameMove,
    win_map: Rc<HashMap<GameMove,GameMove>>,
    lose_map: Rc<HashMap<GameMove,GameMove>>
}
impl Round{
    fn win(&self) -> bool{
        self.win_map[&self.elf_move] == self.your_move
    }
    fn draw(&self)->bool{
        self.elf_move == self.your_move
    }
    fn lose(&self)->bool{
        self.lose_map[&self.elf_move] == self.your_move
    }
    fn round_points(&self) -> u64{
        if self.win() {
            return 6 + self.your_move as u64;
        }else if self.draw() {
            return 3 + self.your_move as u64;
        }else if self.lose() {
            return 0 + self.your_move as u64;
        }else{
            return 0;
        }
    }
    fn recalculate_move(&mut self, outcome: &str){
        self.your_move = match outcome {
            "X" => self.lose_map[&self.elf_move],
            "Y" => self.elf_move,
            "Z" => self.win_map[&self.elf_move],
            _ => todo!()
        }
    }
}

fn fill_rounds(input:&str,rounds: &mut Vec<Round>, 
    elf_moves: &HashMap<&str,GameMove>, 
    your_moves: &HashMap<&str,GameMove>, 
    win_map: Rc<HashMap<GameMove,GameMove>>, 
    lose_map: Rc<HashMap<GameMove,GameMove>>){
    for line in input.lines(){
        let mut items = line.split_ascii_whitespace();
        rounds.push(Round{
            elf_move: elf_moves[items.next().unwrap()],
            your_move: your_moves[items.next().unwrap()],
            win_map: Rc::clone(&win_map),
            lose_map: Rc::clone(&lose_map)
        });
    }
}
fn fill_rounds_outcome(input:&str,
    rounds: &mut Vec<Round>, 
    elf_moves: &HashMap<&str,GameMove>, 
    win_map: Rc<HashMap<GameMove,GameMove>>, 
    lose_map: Rc<HashMap<GameMove,GameMove>>){
    for line in input.lines(){
        let mut items = line.split_ascii_whitespace();
        let elf_move = elf_moves[items.next().unwrap()];
        rounds.push(Round { 
            elf_move, 
            your_move: elf_move,
            win_map: Rc::clone(&win_map),
            lose_map: Rc::clone(&lose_map)
        });
        rounds.last_mut().unwrap().recalculate_move(items.next().unwrap());
    }
}

fn main(){
    let win_map: HashMap<GameMove,GameMove> = [
            (GameMove::Rock, GameMove::Paper),
            (GameMove::Paper, GameMove::Scissors),
            (GameMove::Scissors, GameMove::Rock)
            ].iter().cloned().collect();
    let lose_map: HashMap<GameMove,GameMove> = [
        (GameMove::Paper, GameMove::Rock),
        (GameMove::Scissors, GameMove::Paper),
        (GameMove::Rock, GameMove::Scissors)
        ].iter().cloned().collect();
    let win_rc = Rc::new(win_map);
    let lose_rc = Rc::new(lose_map);
    let input = include_str!("input.txt");
    let elf_moves: HashMap<&str,GameMove> = [
        ("A", GameMove::Rock),
        ("B", GameMove::Paper),
        ("C", GameMove::Scissors)
        ].iter().cloned().collect();
    let your_moves: HashMap<&str,GameMove> = [
        ("X", GameMove::Rock),
        ("Y", GameMove::Paper),
        ("Z", GameMove::Scissors)
        ].iter().cloned().collect();
    let mut rounds: Vec<Round> = Default::default();
    fill_rounds(input, &mut rounds, &elf_moves, &your_moves, Rc::clone(&win_rc), Rc::clone(&lose_rc));
    
    println!("By setting the moves, I would make {} points.", rounds.iter().map(|r|r.round_points()).sum::<u64>());
    let mut rounds2: Vec<Round> = Default::default();
    fill_rounds_outcome(input, &mut rounds2, &elf_moves, Rc::clone(&win_rc), Rc::clone(&lose_rc));
    println!("By calculating the outcome, I would make {} points.", rounds2.iter().map(|r|r.round_points()).sum::<u64>());
}
