#![warn(clippy::all)]

use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Debug,PartialEq,Eq, Clone)]
enum WorryOperation{
    Addition,
    Multiplication,
    Square
}
impl Default for WorryOperation {
    fn default() -> Self {
        WorryOperation::Addition
    }
}

#[derive(Debug,Default,PartialEq, Eq)]
struct Monkey{
    id: usize,
    items: VecDeque<u128>,
    items_new_rules: VecDeque<u128>,
    operation: WorryOperation,
    operation_amount: u128,
    divisible_test: u128,
    monkey_true: usize,
    monkey_false: usize,
    inspections: u128,
    inspections_new_rules: u128
}
impl Monkey{
    fn is_round_over(&self) -> bool{
        self.items.is_empty()
    }
    
    fn round_item(&mut self) -> (u128, usize){
        self.inspections += 1;
        let mut item = self.items.pop_front().unwrap();
        item = match self.operation {
            WorryOperation::Addition => item + self.operation_amount,
            WorryOperation::Multiplication => item * self.operation_amount,
            WorryOperation::Square => item*item,
        };
        item = item / 3;
        if (item % self.divisible_test) == 0 {
            (item, self.monkey_true)
        } else {
            (item, self.monkey_false)
        }
    }

    fn is_round_over_new_rules(&self) -> bool{
        self.items_new_rules.is_empty()
    }
    fn round_item_new_rules(&mut self, common_modulo:u128) -> (u128, usize){
        self.inspections_new_rules += 1;
        let mut item = self.items_new_rules.pop_front().unwrap();
        item = match self.operation {
            WorryOperation::Addition => item + self.operation_amount,
            WorryOperation::Multiplication => item * self.operation_amount,
            WorryOperation::Square => item*item,
        };
        if (item % self.divisible_test) == 0 {
            (item % common_modulo, self.monkey_true)
        } else {
            (item % common_modulo, self.monkey_false)
        }
    }
}

fn main(){
    let input = include_str!("input.txt");
    let debug = false;
    let mut monkeys: Vec<Monkey> = Default::default();
    input.lines().chunks(7).into_iter().for_each(|mut monkey_lines|{
        let mut line= monkey_lines.next().unwrap().trim();
        let id = sscanf::sscanf!(line, "Monkey {usize}:").unwrap();
        line = monkey_lines.next().unwrap().trim();
        let items = sscanf::sscanf!(line, "Starting items: {str}")
            .unwrap()
            .split(",")
            .map(|n|n.trim().parse::<u128>().unwrap())
            .collect::<VecDeque<u128>>();
        line = monkey_lines.next().unwrap().trim();
        let (operation_str, operation_amount_str) = sscanf::sscanf!(line, "Operation: new = old {str} {str}").unwrap();
        let operation_amount = operation_amount_str.parse::<u128>().unwrap_or_default();
        let operation = if operation_amount_str == "old" {WorryOperation::Square} else if operation_str == "*" {WorryOperation::Multiplication} else {WorryOperation::Addition};
        line = monkey_lines.next().unwrap().trim();
        let divisible_test = sscanf::sscanf!(line, "Test: divisible by {u128}").unwrap();
        line = monkey_lines.next().unwrap().trim();
        let monkey_true = sscanf::sscanf!(line, "If true: throw to monkey {usize}").unwrap();
        line = monkey_lines.next().unwrap().trim();
        let monkey_false = sscanf::sscanf!(line, "If false: throw to monkey {usize}").unwrap();
        monkeys.push(Monkey { id, items:items.clone(), operation, operation_amount: operation_amount, divisible_test, monkey_true, monkey_false, inspections: Default::default(), items_new_rules: items.clone(),inspections_new_rules: Default::default() });
    });
    if debug{
        monkeys.iter().for_each(|m|println!("{:?}",m));
    }
    // We play 20 rounds
    for round in 0..20{
        for i in 0..monkeys.len(){
            let m = monkeys.get_mut(i).unwrap();
            let mut operations:Vec<(u128,usize)> = Default::default();
            while !m.is_round_over(){
                operations.push(m.round_item());
            }
            for (item, id) in operations{
                monkeys[id].items.push_back(item);
            }
        }
        if debug{
            println!("End of round {}", round+1);
            monkeys.iter().for_each(|m|println!("{:?}",m));
        }
    }
    let binding = monkeys.iter().map(|m|m.inspections)
        .sorted()
        .rev()
        .chunks(2)
        .into_iter()
        .map(|mut c|c.next().unwrap()*c.next().unwrap())
        .collect_vec();
    let monkey_business = binding.first().unwrap();
    println!("The total level of monkey business is {}", monkey_business);
    let common_modulo = monkeys.iter().map(|m|m.divisible_test).product();
    // We play 20 rounds
    for round in 0..10000{
        for i in 0..monkeys.len(){
            let m = monkeys.get_mut(i).unwrap();
            let mut operations:Vec<(u128,usize)> = Default::default();
            while !m.is_round_over_new_rules(){
                operations.push(m.round_item_new_rules(common_modulo));
            }
            for (item, id) in operations{
                monkeys[id].items_new_rules.push_back(item);
            }
        }
        if debug{
            println!("End of round {}", round+1);
            monkeys.iter().for_each(|m|println!("{:?}",m));
        }
    }
    let binding_new = monkeys.iter().map(|m|m.inspections_new_rules)
        .sorted()
        .rev()
        .chunks(2)
        .into_iter()
        .map(|mut c|c.next().unwrap()*c.next().unwrap())
        .collect_vec();
    let monkey_business_new = binding_new.first().unwrap();
    println!("The total level of monkey business with new rules is {}", monkey_business_new);
}
