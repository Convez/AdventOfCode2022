use std::{collections::{VecDeque}, ops::{Index, IndexMut}};

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug,Clone, Copy,PartialEq, Eq,EnumIter)]
enum BotType{
    Ore,
    Clay,
    Obsidian,
    Geode,
}

// We store everything in the state as u8 as the state exploration is heavy on the memory
#[derive(Debug,Clone, PartialEq, Eq)]
struct State{
    bots: Vec<u8>,
    collected: Vec<u8>,
    life: u8
}

type BotRequrements = Vec<Vec<u8>>;

impl<T> Index<BotType> for Vec<T> {
    type Output = T;

    fn index(&self, index: BotType) -> &Self::Output {
        &self[index as usize]
    }
}
impl<T> IndexMut<BotType> for Vec<T> {
    fn index_mut(&mut self, index: BotType) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl State{
    // Function returns in how many turns I can create the bot, given its cost and my resources
    fn time_until_bot_resources_ready(&self, bot_cost:&Vec<u8>, countdown:u8) -> u8{
        bot_cost.iter().enumerate()
        .map(|(b,cost)|{
            match cost{
                // If we already have the resources, we can just build
                cost if *cost <= self.collected[b] =>0,
                //If we don't have bots for the resources, we'll wait forever
                _ if self.bots[b] == 0 => countdown as u8+1,
                // Otherwise we calculate the rounds needed to gather resources
                _ => (cost + self.bots[b] - self.collected[b]-1)/self.bots[b],
            }
        }).max().unwrap()
    }
}

fn perform_simulation(first_state:State, requrements:&BotRequrements, countdown:u8)->u8{
    use BotType::*;
    // For part2, the search space explodes. Need to limit the decision tree
    // For this we limit the amount of bots we can potentially create
    // No need to create more bots if at every turn we already collect 
    // enough resource for a type (excluded geodes, geodes are never enough)
    // Doing this also improves performance for part 1, as a side effect
    let mut max_bots = vec![u8::MAX;4];
    for bot in BotType::iter().take(3){
        max_bots[bot] = requrements.iter().map(|bot_cost|bot_cost[bot]).max().unwrap();
    }
    println!("{:?}",max_bots);

    let mut states_stack: VecDeque<State> = Default::default();
    let mut target_geodes: u8 = 0;
    states_stack.push_back(first_state);
    
    while let Some(state) = states_stack.pop_front().as_mut() {
        for bot in BotType::iter(){
            // If we have already enough bots for this type, we skip the decision tree
            if state.bots[bot] >= max_bots[bot]{
                continue;
            }
            let bot_cost = &requrements[bot];

            // Which resource is preventing from building the bot?
            let needed_rounds = state.time_until_bot_resources_ready(bot_cost, countdown);

            let time_lived = state.life + needed_rounds +1;
            if time_lived >= countdown{
                // This decision branch takes too long
                continue;
            }

            let next_collection: Vec<u8> = BotType::iter().map(|b|
             (state.collected[b] + state.bots[b]*(needed_rounds+1)-bot_cost[b])).collect();
            
            let mut next_bots = state.bots.clone();
            next_bots[bot] +=1;
            
            states_stack.push_back(State{
                bots: next_bots,
                collected:  next_collection,
                life: time_lived,
            });
        }
        // Calculate final collection
        let final_geodes = state.collected[Geode] + state.bots[Geode] * (countdown - state.life);
        target_geodes = std::cmp::max(target_geodes, final_geodes);
        state.collected.clear();
        state.bots.clear();
    }
    target_geodes
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    
    let geodes_per_blueprint = input.lines().map(|l|{
        use BotType::*;
        let (bp_id,ore_robot_ore_cost, clay_robot_ore_cost, obsidian_robot_ore_cost, obsidian_robot_clay_cost,geode_robot_ore_cost, geode_robot_obsidian_cost) =
        sscanf::sscanf!(l.trim(), "Blueprint {u8}: Each ore robot costs {u8} ore. Each clay robot costs {u8} ore. Each obsidian robot costs {u8} ore and {u8} clay. Each geode robot costs {u8} ore and {u8} obsidian.").unwrap();
        let mut requrements =vec![vec![0;4];4];
        requrements[Ore][Ore]= ore_robot_ore_cost;
        requrements[Clay][Ore] = clay_robot_ore_cost;
        requrements[Obsidian][Ore] = obsidian_robot_ore_cost;
        requrements[Obsidian][Clay] = obsidian_robot_clay_cost;
        requrements[Geode][Ore] = geode_robot_ore_cost;
        requrements[Geode][Obsidian] = geode_robot_obsidian_cost;
        let start_state = State{
            bots: vec![1,0,0,0],
            collected: vec![0;4],
            life: 0,
        };
        let collected_geodes = perform_simulation(start_state, &requrements, 24);
        println!("{}",collected_geodes);
        (bp_id as u16, collected_geodes as u16)
    }).collect_vec();
    let max_score = geodes_per_blueprint.iter().map(|(id,geodes)|id * geodes).sum::<u16>();
    println!("Tot scores is: {}", max_score);
    //Funny thing, this solution overflows my RAM for test data, but not for real input(lucky)
    let geodes_per_blueprint2 = input.lines().take(3).map(|l|{
        use BotType::*;
        let (_,ore_robot_ore_cost, clay_robot_ore_cost, obsidian_robot_ore_cost, obsidian_robot_clay_cost,geode_robot_ore_cost, geode_robot_obsidian_cost) =
        sscanf::sscanf!(l.trim(), "Blueprint {u8}: Each ore robot costs {u8} ore. Each clay robot costs {u8} ore. Each obsidian robot costs {u8} ore and {u8} clay. Each geode robot costs {u8} ore and {u8} obsidian.").unwrap();
        let mut requrements =vec![vec![0;4];4];
        requrements[Ore][Ore]= ore_robot_ore_cost;
        requrements[Clay][Ore] = clay_robot_ore_cost;
        requrements[Obsidian][Ore] = obsidian_robot_ore_cost;
        requrements[Obsidian][Clay] = obsidian_robot_clay_cost;
        requrements[Geode][Ore] = geode_robot_ore_cost;
        requrements[Geode][Obsidian] = geode_robot_obsidian_cost;
        let start_state = State{
            bots: vec![1,0,0,0],
            collected: vec![0;4],
            life: 0,
        };
        let collected_geodes = perform_simulation(start_state, &requrements, 32);
        println!("{}",collected_geodes);
        // Store geodes as u64 instead of u8 since we'll have to multiply them together
        collected_geodes as u64
    }).reduce(|a,b|a*b).unwrap();
    println!("Max score after 32 minutes is {}",geodes_per_blueprint2);

}