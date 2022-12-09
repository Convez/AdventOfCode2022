#![warn(clippy::all)]

use itertools::Itertools;



fn main(){
    let input = include_str!("input.txt");

    let mut grid: Vec<Vec<usize>> = Default::default();
    // Fill grid
    for line in input.lines(){
        grid.push(Default::default());
        grid.last_mut().unwrap().append(&mut line.chars().map(|c|c.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>());
    }
    let base_visible = grid.len() * 2 + grid.first().unwrap().len() *2 -4;
    println!("Visible edges trees are {}", base_visible);
    let mut inner_visible = 0;
    let mut max_score = 0;
    for i in 1..grid.len()-1{
        for j in 1..grid[i].len()-1{
            let scene_left = grid[i][0..j].to_vec();
            let scene_right = grid[i][j+1..grid[i].len()].to_vec();
            let scene_top = grid[0..i].iter().map(|l|l[j]).collect_vec();
            let scene_bottom = grid[i+1..grid.len()].iter().map(|l|l[j]).collect_vec();
            // Left visibility
            let visible_left = *scene_left.iter().max().unwrap() < grid[i][j];
            let visible_right = *scene_right.iter().max().unwrap() < grid[i][j];
            let visible_top = *scene_top.iter().max().unwrap() < grid[i][j];
            let visible_bottom = *scene_bottom.iter().max().unwrap() < grid[i][j];
            if visible_bottom || visible_left || visible_right || visible_top {
                inner_visible+=1;
            }
            let mut tree_score = 1;
            let mut left_score = 0;
            for tree in scene_left.iter().rev(){
                left_score +=1;
                if *tree >= grid[i][j]{
                    break;
                }
            }
            let mut right_score = 0;
            for tree in scene_right.iter(){
                right_score +=1;
                if *tree >= grid[i][j]{
                    break;
                }
            }
            let mut top_score = 0;
            for tree in scene_top.iter().rev(){
                top_score +=1;
                if *tree >= grid[i][j]{
                    break;
                }
            }
            let mut bottom_score = 0;
            for tree in scene_bottom.iter(){
                bottom_score +=1;
                if *tree >= grid[i][j]{
                    break;
                }
            }
            tree_score *= left_score*right_score*top_score*bottom_score;
            if tree_score > max_score{
                max_score = tree_score;
            }

        }
    }
    println!("Inner trees that are visible are {}", inner_visible);
    let total_visible = base_visible + inner_visible;
    println!("Total visible trees are {}", total_visible);
    println!("Max visibility score is {}", max_score);
}
