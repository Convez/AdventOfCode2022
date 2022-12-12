#![warn(clippy::all)]

use std::{collections::HashMap};

use itertools::Itertools;
use petgraph::{graph::{DiGraph,NodeIndex}, Graph,algo::dijkstra};

type NodePos = (usize,usize);

fn value_from(c: char) -> u8{
    match c {
        'S' => 0,
        'E' => 'z' as u8 - 'a' as u8,
        any => any as u8 - 'a' as u8
    }
}

//Return start and finish positions
fn fill_graph(input: &str, g: &mut Graph<NodePos,i32>, allow_jump:bool) -> (NodeIndex,Vec<NodeIndex>, NodeIndex){
    // Turn input into array of chars
    let points = input.lines().map(|l|l.trim().chars().collect_vec()).collect_vec();
    let mut start:NodePos = Default::default();
    let mut starts:Vec<NodePos> = Default::default();
    let mut finish:NodePos = Default::default();
    let mut edges : Vec<(NodePos,NodePos, i32)> = Default::default();
    let mut nodes: HashMap<NodePos, NodeIndex> = Default::default();
    for i in 0..points.len(){
        for j in 0..points[i].len(){
            match points[i][j] {
                'S' => {start = (i,j); starts.push((i,j))},
                'E' => {finish = (i,j)},
                'a' => {starts.push((i,j))}
                _ => ()
            };
            nodes.insert((i,j),g.add_node((i,j)));
            let value = value_from(points[i][j]);

            let neighbors = vec![(i as i32-1,j as i32),(i as i32+1,j as i32),(i as i32,j as i32-1),(i as i32,j as i32+1)];
            for (x,y) in neighbors{
                if x >=0 && x < points.len() as i32 && y >=0 && y < points[i].len() as i32{
                    let n_value = value_from(points[x as usize][y as usize]);
                    let edge_size = n_value as i32 - value as i32;
                    if edge_size <= 1{
                        edges.push(((i,j),(x as usize,y as usize), edge_size));
                    } else if allow_jump {
                        edges.push(((i,j),(x as usize,y as usize), edge_size));
                    }
                }
            }
        }
    }
    edges.iter().map(|e|(nodes.get(&e.0).unwrap(), nodes.get(&e.1).unwrap(),&e.2))
    .for_each(|e|{ g.add_edge(*e.0, *e.1, *e.2); });
    (*nodes.get(&start).unwrap(),starts.iter().map(|s|*nodes.get(s).unwrap()).collect_vec(),*nodes.get(&finish).unwrap())
}

fn main(){
    let input = include_str!("input.txt");
    let mut g = DiGraph::<NodePos,i32>::new();
    let (start,starts,finish)=fill_graph(input, &mut g, false);
    let paths_cost = dijkstra::dijkstra(&g, start, Some(finish), |_|1);
    println!("Shortest path from {:?} for the graph without considering big leaps is: {}", start, paths_cost.get(&finish).unwrap_or(&i32::MAX));
    
    let min_path = starts.iter().map(|s|dijkstra(&g, *s, Some(finish), |_|1))
    .filter(|d|d.contains_key(&finish))
    .map(|d|d[&finish])
    .min().unwrap();
    println!("Shortest path ever for the graph without considering big leaps is: {}", min_path);

}
