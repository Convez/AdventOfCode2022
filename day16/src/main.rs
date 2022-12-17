use std::{collections::{HashMap}};

use itertools::Itertools;
use petgraph::{graph::UnGraph, stable_graph::NodeIndex,algo::dijkstra};

type VectorEdges = Vec<(String, i32, Vec<String>)>;


fn parse (input:&str) -> VectorEdges{
    input.lines().map(|l|{
        let edited_line = l.trim().replace("valves", "valve").replace("tunnels", "tunnel").replace("leads", "lead");
        let (vector,flow,edges) = sscanf::sscanf!(edited_line,"Valve {str} has flow rate={i32}; tunnel lead to valve {str}").unwrap();
        (vector.to_string(),flow,edges.split(",").map(|e|e.trim().to_string()).collect_vec())
    }).collect_vec()
}

fn fill_graph(graph: &mut UnGraph<String,String>,items: &VectorEdges, node_map: &mut HashMap<String,(NodeIndex, i32)>){
    items.iter().for_each(|(v,f,_)| {
        node_map.insert(v.clone(), (graph.add_node(v.clone()),*f));
    });
    items.iter().for_each(|(v,_f,es)|{
        let vn = node_map[v];
        for e in es{
            graph.add_edge(vn.0, node_map[e].0, "e".to_string());
        }
    });
}

fn explore_options(distances:&HashMap<String,HashMap<NodeIndex,i32>>,
    current_item:String, 
    minutes_left: i32,
    left_items: HashMap<String, i32>,
    helper_map: &HashMap<String,(NodeIndex, i32)>
) -> i32{
    let current_item_distances = &distances[&current_item];
    let curr_max = left_items.iter().filter_map(|(item,flow)|{
        if let Some(distance) = current_item_distances.get(&helper_map[item].0){
            if minutes_left-distance-1>=0{
                // Current item is no longer eligible
                let mut other_items = left_items.clone();
                other_items.remove(item);
                return Some(flow * (minutes_left - distance-1) + explore_options(distances, item.to_string(), minutes_left-distance-1, other_items, helper_map))
            }
            else {
                return None;
            }
        }else{
            return None
        }
    }).max().unwrap_or(0);
    curr_max
}

fn explore_options_together(distances:&HashMap<String,HashMap<NodeIndex,i32>>,
    current_item_human:String,
    current_item_elephant:String, 
    minutes_left_human: i32,
    minutes_left_elephant:i32,
    left_items: HashMap<String, i32>,
    helper_map: &HashMap<String,(NodeIndex, i32)>
) -> i32{
    // If one of the two operators comes to an halt, the other continues by themselves
    if current_item_elephant == "STOP" && current_item_human == "STOP"{
        return 0;
    }
    if current_item_elephant == "STOP"{
        return explore_options(distances, current_item_human, minutes_left_human, left_items, helper_map);
    }
    if current_item_human == "STOP"{
        return explore_options(distances, current_item_elephant, minutes_left_elephant, left_items, helper_map);
    }
    let current_item_human_distances = &distances[&current_item_human];
    let current_item_elephant_distances = &distances[&current_item_elephant];
    
    // We check for all permutations of 2 items available 
    //(one path for human, other path for elephant, order seems to matter
    // since using combination renders the output not consistent)
    let curr_max = left_items.iter().permutations(2)
    .filter_map(|pair|{
        // Unwrapping is safe, since elements with non-0 flow are 14
        let (human_route,human_flow) = pair.first().unwrap();
        let (elephant_route,elephant_flow) = pair.last().unwrap();
        if let Some(distance_human) = current_item_human_distances.get(&helper_map[&human_route.to_string()].0){ 
            if let Some(distance_elephant) = current_item_elephant_distances.get(&helper_map[&elephant_route.to_string()].0){
                // It should always enter in here, since all distances are computed
                let flow_human = if minutes_left_human-distance_human-1>=0 {**human_flow * (minutes_left_human - distance_human-1)} else {0};
                let flow_elephant = if minutes_left_elephant-distance_elephant-1>=0 {**elephant_flow * (minutes_left_elephant - distance_elephant-1)} else {0};
                let mut other_items = left_items.clone();
                other_items.remove(&human_route.to_string());
                other_items.remove(&elephant_route.to_string());
                return Some(flow_human + flow_elephant +
                    explore_options_together(distances, 
                        if flow_human >0 {human_route.to_string()} else {"STOP".to_string()},
                        if flow_elephant >0 {elephant_route.to_string()} else {"STOP".to_string()},
                        minutes_left_human-distance_human-1, 
                        minutes_left_elephant-distance_elephant-1, 
                        other_items, 
                        helper_map) );
            }
            else {
                return None;
            }
        }
        else {
            return None;
        }
    }).max().unwrap_or(0);
    curr_max
}



fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let items = parse(input);
    let mut graph:UnGraph<String,String> = UnGraph::new_undirected();
    let mut node_map:HashMap<String,(NodeIndex, i32)> =  Default::default();

    fill_graph(&mut graph, &items, &mut node_map);
    
    // Only the items with non-zero flow are eligible
    let left_items:HashMap<_,_> = items.iter().map(|(a,b,_)|(a.to_string(),*b))
        .filter(|(_,b)|*b>0).collect();
    
        // Pre-compute dijkstra distances
    let mut distances: HashMap<String,HashMap<NodeIndex,i32>> = left_items.iter().map(|(item,_)|
        (item.to_string(), dijkstra(&graph, node_map[item].0, None, |_|1))
    ).collect();
    // Add dijkstra distance for first item (the only one with 0 flow)
    let mut current = items.iter().find(|i|i.0 == "AA").unwrap().0.clone();
    let current_item = node_map[&current];
    distances.insert(current, dijkstra(&graph, current_item.0, None, |_|1));
    current = items.iter().find(|i|i.0 == "AA").unwrap().0.clone();
    
    // PROBLEM 1
    // Execute graph exploration
    let minutes_left = 30;
    let flow = explore_options(&distances, current.to_string(), minutes_left, left_items, &node_map);
    
    println!("{}", flow);

    // PROBLEM 2
    // Execute graph exploration with 2 operators
    let minutes_after_teaching = 26;
    let left_items_2:HashMap<_,_> = items.iter().map(|(a,b,_)|(a.to_string(),*b))
    .filter(|(_,b)|*b>0).collect();
    let flow_together = explore_options_together(&distances, current.to_string(), current.to_string(), minutes_after_teaching, minutes_after_teaching, left_items_2, &node_map);
    println!("{}",flow_together);
}