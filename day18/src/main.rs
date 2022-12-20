use std::{collections::HashSet};

type Cube = (i32,i32,i32);

fn calculate_exposed_faces(cube:&Cube, cubes: &HashSet<Cube>)-> usize{
    let mut exposed = 0;
    let adjacents = make_neighbors(cube);
    // The exposed faces are the faces for which the adjacent space is not filled
    for adj in adjacents{
        if !cubes.contains(&adj){
            exposed += 1;
        }
    }
    exposed
}


fn make_neighbors(cube:&Cube)->Vec<Cube>{
    let adjacents =
    vec![(cube.0+1,cube.1,cube.2),
         (cube.0,cube.1+1,cube.2),
         (cube.0,cube.1,cube.2+1),
         (cube.0-1,cube.1,cube.2),
         (cube.0,cube.1-1,cube.2),
         (cube.0,cube.1,cube.2-1)
         ];
    adjacents
}

fn is_inside(cube:&Cube, max_x:i32,max_y:i32,max_z:i32,min_x:i32,min_y:i32,min_z:i32)->bool{
    cube.0 >= min_x-1 && cube.0 <= max_x+1 &&
    cube.1 >= min_y-1 && cube.1 <= max_y+1 &&
    cube.2 >= min_z-1 && cube.2 <= max_z+1
}

fn generate_outside(cubes: &HashSet<Cube>, max_x:i32,max_y:i32,max_z:i32,min_x:i32,min_y:i32,min_z:i32) -> HashSet<Cube> {
    let mut exposed = HashSet::new();

    let start = Cube::default();
    let mut stack = Vec::new();
    let mut seen = HashSet::new();

    stack.push(start);
    seen.insert(start);

    while let Some(coord) = stack.pop() {
        for neighbour in make_neighbors(&coord) {
            if cubes.contains(&neighbour) || !is_inside(&neighbour, max_x, max_y, max_z, min_x, min_y, min_z) {
                continue;
            }
            if seen.insert(neighbour) {
                stack.push(neighbour);
                exposed.insert(neighbour);
            }
        }
    }

    exposed
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};

    let cubes:HashSet<Cube> = input.lines().map(|l|sscanf::sscanf!(l.trim(),"{i32},{i32},{i32}").unwrap()).collect();
    let total_exposed = cubes.iter().map(|c|calculate_exposed_faces(c, &cubes))
    .sum::<usize>();
    println!("{}",total_exposed);
    let max_x = cubes.iter().max_by_key(|i|i.0).unwrap().0;
    let max_y = cubes.iter().max_by_key(|i|i.1).unwrap().1;
    let max_z = cubes.iter().max_by_key(|i|i.2).unwrap().2;
    let min_x = cubes.iter().min_by_key(|i|i.0).unwrap().0;
    let min_y = cubes.iter().min_by_key(|i|i.1).unwrap().1;
    let min_z = cubes.iter().min_by_key(|i|i.2).unwrap().2;

    let outside_cubes = generate_outside(&cubes, max_x, max_y, max_z, min_x, min_y, min_z);

    let total_exposed_no_air = cubes.iter()
    .flat_map(|i|make_neighbors(i))
    .filter(|c|outside_cubes.contains(c))
    .count();

    println!("{}",total_exposed_no_air);
}