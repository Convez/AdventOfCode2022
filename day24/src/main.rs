use std::{fmt::Display, collections::{VecDeque,HashSet}};

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Pos = (usize,usize);
#[derive(Debug,Clone, Copy,PartialEq, Eq,PartialOrd, Ord)]
enum Action {
    Wait(Pos),
    Move(Pos)
}
#[derive(Debug,Clone, Copy,PartialEq, Eq,PartialOrd, Ord,EnumIter,Hash)]
enum Direction {
    North,
    South,
    West,
    East
}
impl Direction {
    fn next_valid_position(&self, my_pos:Pos, map: &Vec<Vec<Tile>>)->Pos{
        use Direction::*;
        let last_x = map.len()-2;
        let last_y = map[0].len()-2;
        match (self,my_pos) {
            (North,(1,col)) => (map.len()-2,col),
            (North,(row,col)) => (row-1,col),
            (West, (row,1)) => (row, map[row].len()-2),
            (West,(row,col)) => (row,col-1),
            (South,(r,col))=> if r == last_x {(1,col)} else {(r+1,col)}
            (East,(row,c)) => if c == last_y {(row,1)} else {(row,c+1)}
        }
    }
    fn into_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::East => '>',
        }
    }
}

#[derive(Debug,Clone, PartialEq, Eq,PartialOrd, Ord,Hash)]
enum Tile {
    Wall,
    Floor,
    Character,
    Blizzards(Vec<Direction>)
}
impl Display for Tile{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char:char = match self {
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::Character => 'C',
            Tile::Blizzards(dirs) => if dirs.len()==1 {dirs.first().unwrap().into_char()} else {char::from_digit((dirs.len()%10) as u32, 10).unwrap()},
        };
        write!(f, "{}",char)
    }
}

fn generate_next_map(map: &Vec<Vec<Tile>>)->Vec<Vec<Tile>>{
    use Tile::*;

    let mut next_map = map.clone();
    // Clean new map blizzards
    for row in 0..next_map.len(){ for col in 0..next_map[row].len(){
        if next_map[row][col] != Tile::Wall{
            next_map[row][col] = Tile::Floor;
        }
    }} 
    for row in 0..map.len(){ for col in 0..map[row].len(){
        if let Blizzards(blizzards) = &map[row][col]{
            for blizzard in blizzards{
                let (new_row,new_col) = blizzard.next_valid_position((row,col), map);
                if let Blizzards(new_blizzards) = &mut next_map[new_row][new_col]{
                    new_blizzards.push(blizzard.clone());
                }else {
                    next_map[new_row][new_col] = Blizzards(vec![blizzard.clone()]);
                }
            }
        } 
    }
    }
    next_map
}

fn print_map(map: &Vec<Vec<Tile>>,character:&Pos){
    let mut map = map.clone();
    map[character.0][character.1] = Tile::Character;
    map.iter().for_each(|row|{row.iter()
        .for_each(|col|print!("{}",col));println!()});
}
#[derive(Debug,Clone, Copy,PartialEq, Eq)]
struct State{
    character_pos: Pos,
    minutes_passed:usize
}
impl State {
    fn get_potential_action(&self, next_map: &Vec<Vec<Tile>>) -> Vec<Action> {
        use Tile::*;
        use Action::*;
        let mut actions = vec![];
        // Let's say that if possible, I should always move
        for dir in Direction::iter(){
            let next_coords = match dir {
                Direction::North => {
                    if self.character_pos.0<=0{
                        continue;
                    }
                    (self.character_pos.0-1,self.character_pos.1)
                },
                Direction::South => {
                    if self.character_pos.0>=next_map.len()-1{
                        continue;
                    }
                    (self.character_pos.0+1,self.character_pos.1)
                },
                Direction::West => {
                    if self.character_pos.1<=0{
                        continue;
                    }
                    (self.character_pos.0,self.character_pos.1-1)
                },
                Direction::East => {
                    if self.character_pos.1>=next_map[0].len()-1{
                        continue;
                    }
                    (self.character_pos.0,self.character_pos.1+1)
                },
            };
            if next_map[next_coords.0][next_coords.1] == Floor{
                actions.push(Move((next_coords.0 as usize, next_coords.1 as usize)));
            }
        }
        // Otherwise I stay in place, if possible
        if next_map[self.character_pos.0][self.character_pos.1] == Floor{
            actions.push(Wait(self.character_pos));
        }
        
        actions
    }
}

fn simulate(starting_map: &mut Vec<Vec<Tile>>, starting_position: &Pos, target_pos:&Pos, trips:usize, current_trip:usize)->usize{
    let mut states: VecDeque<State> = Default::default();
    states.push_back(State{
        character_pos: starting_position.clone(),
        minutes_passed: 0,
    });
    // We need to limit the amout of time we take in the recursion
    let mut min_passed = 0;
    let mut next_map = starting_map.clone();
    // Hashset takes care of multiple path arriving at the same position
    let mut positions_to_consider:HashSet<(usize,usize)> = vec![starting_position].iter().map(|p|(p.0,p.1)).collect();
    loop {
        println!("Minute {}",min_passed);
        if positions_to_consider.iter().any(|p|p.0==target_pos.0 && p.1==target_pos.1){
            println!("For trip {} passed {} minutes",current_trip, min_passed);
            if current_trip == trips{
                return min_passed;
            }
            return min_passed+simulate(&mut next_map, target_pos, starting_position, trips, current_trip+1);            
        }
        next_map = generate_next_map(&next_map);
        print_map(&next_map, positions_to_consider.iter().find(|_|true).unwrap());
        positions_to_consider = positions_to_consider.iter()
        .flat_map(|pos|State{ character_pos: pos.clone(), minutes_passed: min_passed }.get_potential_action(&next_map))
        .map(|na|{
            match na {
                Action::Wait(place) =>place,
                Action::Move(to) => to,
            }
        }).collect();
        min_passed+=1;
    }
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let mut map = input.lines().map(|l|l.trim().chars().map(|c|{
        use Tile::*;
        use Direction::*;
        match c {
            '#' => Wall,
            '.' => Floor,
            '>' => Blizzards(vec![East]),
            '<' => Blizzards(vec![West]),
            '^' => Blizzards(vec![North]),
            'v' => Blizzards(vec![South]),
            _ =>todo!()
        }
    }).collect_vec()).collect_vec();
    let my_pos:(usize,usize) = (0,map[0].iter().enumerate().find(|t|*t.1==Tile::Floor).unwrap().0);
    let target: (usize,usize) = (map.len()-1,map[0].len()-2);
    print_map(&map, &my_pos);

    let total = simulate(&mut map,&my_pos,&target,3,1);
    println!("Total for the 3 trips is {}",total);
}