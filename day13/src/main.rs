#![warn(clippy::all)]

use std::cmp::Ordering;
use itertools::Itertools;
use sscanf::regex::Regex;


#[derive(Debug,Clone,PartialEq, Eq)]
enum Packet{
    Empty,
    List(Vec<Packet>),
    Number(i32)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;
        use Packet::*;
        match (self,other) {
            (Packet::Empty, Packet::Empty) => Some(Equal),
            (Packet::Empty, _) => Some(Less),
            (_, Packet::Empty) => Some(Greater),
            (Packet::List(l1), Packet::Number(n2)) => l1.partial_cmp(&vec![Number(*n2)]),
            (Packet::Number(n1), Packet::List(l2)) => vec![Number(*n1)].partial_cmp(l2),
            (Packet::Number(n1), Packet::Number(n2)) => n1.partial_cmp(n2),
            (Packet::List(l1), Packet::List(l2)) => l1.partial_cmp(l2)
        }
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn packet_from_string(input: &str) -> Packet{
    use Packet::*;
    let mut list_stack = vec![];
    let mut to_return = Empty;
    let mut current = Empty;
    let token_regx= Regex::new(r"[\[\]]|\d+").unwrap();

    for token in token_regx.captures_iter(input){
        match &token[0] {
            "[" =>{
                let l = List(vec![]);
                match current {
                    Empty => {current = l},
                    List(_) => {list_stack.push(current); current = l;},
                    Number(_) => todo!(),
                }
            },
            "]" => {
                match current {
                    List(_) => {
                        if let Some(mut n1) = list_stack.pop(){
                            if let List(l1) = &mut n1{
                                l1.push(current);
                                current = n1;
                            }
                        }else {
                            // We finished popping the stack
                            to_return = current;
                            current = Empty;
                        }
                    },
                    _ => todo!(),
                }
            },
            n => {
                match &mut current {
                    List(l) => l.push(Number(n.parse::<i32>().unwrap())),
                    _ => todo!(),
                }
            }
        }
    } 
    to_return
}

fn main(){
    let input = include_str!("input.txt");
    let packet_paris = input.lines().chunks(3).into_iter()
    .map(|mut c|(packet_from_string(c.next().unwrap().trim()),packet_from_string(c.next().unwrap().trim())))
    .collect_vec();

    let amount_in_oder:usize = packet_paris.iter().map(|p|p.0<=p.1).enumerate().filter(|e|e.1).map(|e|e.0+1).sum();
    println!("The sum of indices for paris in order is: {}", amount_in_oder);
    
    let mut all_items = packet_paris.into_iter().flat_map(|p|vec![p.0,p.1]).collect_vec();
    use Packet::*;
    let p2 = List(vec!(List(vec!(Number(2)))));
    let p6 = List(vec!(List(vec!(Number(6)))));
    all_items.push(p2.clone());
    all_items.push(p6.clone());


    all_items.sort();

    let index_p2 = all_items.binary_search(&p2).unwrap() + 1;
    let index_p6 = all_items.binary_search(&p6).unwrap() + 1;
    println!("The product of the inserted packet indices is {}", index_p2 *  index_p6);
}
