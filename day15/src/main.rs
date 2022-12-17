use itertools::Itertools;
use std::{cmp::max};

type Pos = (i32,i32);
type SensorDistance = Vec<(Pos,Pos,i32)>;
type SensorDistanceCandidate = Vec<(Option<Pos>,(Pos,Pos,i32))>;

fn is_beacon_eligible(sensor:Pos,current_beacon:Pos, beacon_distance:i32, potential_beacon:Pos)->bool{
    if sensor.0 == potential_beacon.0 && sensor.1 == potential_beacon.1{
        return false;
    }
    if current_beacon.0 == potential_beacon.0 && current_beacon.1 == potential_beacon.1 {
        return true;
    }
    if distance(sensor, potential_beacon)<= beacon_distance{
        return false;
    }
    return true
}

fn distance(pos1:Pos, pos2:Pos) -> i32{
    (pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)) as i32
}

fn parse(input: &str) -> SensorDistance{
    input.lines().map(|l|{
        let (sx,sy,bx,by) = sscanf::sscanf!(l.trim(),"Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}").unwrap();
        ((sx,sy),(bx,by),distance((sx,sy), (bx,by)))
    }).collect_vec()
}

fn calculate_x_candidates(sensor_distance:&SensorDistance, for_y:i32)->Vec<(i32,i32)>{
    sensor_distance.iter()
    .filter_map(|(sensor,_,distance)|{
        let delta_y = sensor.1.abs_diff(for_y) as i32;
        let delta_x = distance - delta_y;
        //Delta x is sensor.0.abs_diff(target_x). Must be >=0
        if delta_x >=0 {
            // The two candidates are the x for which the abs_diff is >=0
            Some((sensor.0 - delta_x, sensor.0 + delta_x))
        }else{
            None
        }
    }).collect_vec()
}

pub fn solve_problem1(sensor_distance:&SensorDistance, y_to_solve:i32) {
    let candidates = calculate_x_candidates(sensor_distance, y_to_solve);
    let min_x = candidates.iter().min_by_key(|i|i.0).unwrap().0;
    let max_x = candidates.iter().max_by_key(|i|i.1).unwrap().1;
    let mut count = 0;
    for x in min_x..=max_x{
        for (s,b,d) in sensor_distance{
            if !is_beacon_eligible(*s, *b,*d, (x,y_to_solve)){
                count +=1;
                break;
            }
        }
    }
    println!("{}",count);
    
}

fn add_free_candidates(sensor_distance_candidate:&mut SensorDistanceCandidate, y_to_solve:i32){
    sensor_distance_candidate.iter_mut()
    .for_each(|(candidate,(sensor,_,distance))|{
        let delta_y = sensor.1.abs_diff(y_to_solve) as i32;
        let delta_x = *distance - delta_y;
        //Delta x is sensor.0.abs_diff(target_x). Must be >=0
        if delta_x >=0 {
            // The two candidates are the x for which the abs_diff is >=0
            let _ =candidate.insert((sensor.0 - delta_x, sensor.0 + delta_x));
        }else{
            // We don't have a candidate anymore, let's empty the option
            candidate.take();
        }
    });
}

fn calculate_free_area(sensor_distance_candidate:&SensorDistanceCandidate,max_range:i32) -> Option<i32>{
    let mut area = 0;
    // This works only if the first item is of type x<0 y>0, but for the current problem is enough
    for (range, _) in sensor_distance_candidate{
        match range {
            Some(r) => {
                if r.0 > area{
                    return Some(r.0 - 1);
                }
                if r.1 > max_range{
                    return None;
                }
                area = max(area, r.1);
            },
            None => continue,
        }
    }
    None
}

pub fn solve_problem_2(sensor_distance:&SensorDistance, range_to_solve:i32) {
    // Let's pre-prepare the structure
    let mut sensor_distance_with_candidate:SensorDistanceCandidate = vec![None;sensor_distance.len()]
    .iter().zip(sensor_distance)
        .map(|(a,b)|(*a,*b)).collect_vec();
    // complexity is search_area * number of s2ensors
    for y in 0..=range_to_solve {
        add_free_candidates(&mut sensor_distance_with_candidate, y);
        // As for problem 1, we sort by min_x
        sensor_distance_with_candidate.sort_unstable_by_key(|sdc|sdc.0.unwrap_or_default().0);
        if let Some(result) = calculate_free_area(&sensor_distance_with_candidate, range_to_solve){
            let power = (result as i64) * 4000000 + y as i64;
            println!("{}",power);
            break;
        }
    }
}

fn main(){    
    let is_test = false;
    if is_test{
        let input = include_str!("input_tst.txt");
        let sensor_distance = parse(input);
    
        solve_problem1(&sensor_distance,10);
        solve_problem_2(&sensor_distance, 20);
    }else{
        let input = include_str!("input.txt");
        let sensor_distance = parse(input);

        solve_problem1(&sensor_distance,2000000);
        solve_problem_2(&sensor_distance, 4000000);
    }
}