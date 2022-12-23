use day22::{common::{Tile, Direction}, status_map::StatusMap, cube::{Cube, Section}, status_cube::StatusCube};
use itertools::Itertools;
use sscanf::regex::Regex;


fn print_map(map:&Vec<Vec<Tile>>){
    for row in map{
        for col in row{
            print!("{}",col);
        }
        println!()
    }
}
fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};

    let mut map = input.lines().dropping_back(2).map(|l|l.chars().map(|c|{
        match c{
            '.'=>Tile::Move,
            '#'=>Tile::Wall,
            _=>Tile::Warp
        }
    }).collect_vec()).collect_vec();
    let max_col = map.iter().map(|v|v.len()).max().unwrap();
    print_map(&map);
    map.iter_mut().for_each(|v|if v.len()< max_col {v.extend(vec![Tile::Warp;max_col-v.len()].iter())});
    println!("{} {}",map.len(),map[0].len());
    let cube = Cube::from_map(&map,is_test);
    cube.print_cube();
    
    let mut my_status = StatusMap{
        row: 0,
        col: map[0].iter().enumerate().find(|(_,tile)|**tile==Tile::Move).unwrap().0,
        dir: Direction::Right,
    };
    let mut my_status_cube = StatusCube{
        section:Section::Section1,
        row: 0,
        col: 0,
        dir: Direction::Right,
    };
    let token_regx= Regex::new(r"(\d+[A-Z]*)").unwrap();
    let commands = input.lines().last().unwrap();
    for capture in token_regx.captures_iter(commands){
        let command = &capture[0];
        print!("{} ",command);
        let is_turning_present = command.chars().last().unwrap().is_alphabetic(); 
        // First we execute the movement command
        let forward_amount = if is_turning_present{command.chars().dropping_back(1).collect::<String>().parse::<usize>().unwrap()} else {command.parse::<usize>().unwrap()};
        my_status.go_forward_map(forward_amount, &map);
        my_status_cube.go_forward(forward_amount, &cube);
        if is_turning_present {
            my_status.turn(command.chars().last().unwrap());
            my_status_cube.turn(command.chars().last().unwrap());
        }
        println!();
        // println!("{:?}",my_status);
        // std::thread::sleep(Duration::from_millis(300));
    }
    let final_score_map = 1000*(my_status.row as u64 +1)+4*(my_status.col as u64+1)+my_status.dir.to_score() as u64;
    println!("Score map: {}", final_score_map);

    let final_score_cube = 1000*(my_status_cube.row as u64 +1 + cube.get_row_for_section(&my_status_cube.section) as u64)+4*(my_status_cube.col as u64+1+cube.get_col_for_section(&my_status_cube.section) as u64) + my_status_cube.dir.to_score() as u64;
    println!("Score cube: {}", final_score_cube);
    
}