use std::collections::{BTreeMap};

#[derive(Debug)]
enum Monkey {
    Operation(Operation),
    Value(i64)
}
type Operation = (String,String,String);

fn parse_line(line: &str)->(String,Monkey){
    let (monkey_name, monkey_operation) = sscanf::sscanf!(line.trim(),"{str}: {str}").unwrap();
    match monkey_operation.parse::<i64>() {
        Ok(operation_value) => return (monkey_name.to_string(),Monkey::Value(operation_value)),
        Err(_) => {
            let (monkey1, operation, monkey2) = sscanf::sscanf!(monkey_operation,"{str} {str} {str}").unwrap();
            return (monkey_name.to_string(),Monkey::Operation((monkey1.to_string(),operation.to_string(), monkey2.to_string())));
        },
    }
}
fn solve(monkey:&Monkey,monkeys:&BTreeMap<String,Monkey>)->i64{
    match monkey {
        Monkey::Operation((monkey1,op,monkney2)) => {
            match op.as_str(){
                "+"=>solve(&monkeys[monkey1],monkeys) + solve(&monkeys[monkney2],monkeys),
                "-"=>solve(&monkeys[monkey1],monkeys) - solve(&monkeys[monkney2],monkeys),
                "*"=>solve(&monkeys[monkey1],monkeys) * solve(&monkeys[monkney2],monkeys),
                "/"=>solve(&monkeys[monkey1],monkeys) / solve(&monkeys[monkney2],monkeys),
                _  =>todo!()
            }
        },
        Monkey::Value(val) => return *val,
    }
}

fn unwrap(monkey_name:String,monkey:&Monkey,monkeys:&BTreeMap<String,Monkey>) -> String{
    match (monkey_name.as_str(), monkey){
        ("root", Monkey::Operation((monkey1,_, monkey2)))=>{
            format!("({}){}({})", unwrap(monkey1.clone(), &monkeys[monkey1], monkeys), "=",
            unwrap(monkey2.clone(), &monkeys[monkey2], monkeys))
        },
        ("root",_)=>todo!(),
        ("humn",_)=>"h".to_string(),
        (_,Monkey::Operation((monkey1,op,monkey2)))=>{
            format!("({}){}({})", unwrap(monkey1.clone(), &monkeys[monkey1], monkeys), op,
            unwrap(monkey2.clone(), &monkeys[monkey2], monkeys))
        },
        (_,Monkey::Value(val))=>val.to_string()
    }
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};

    let monkeys: BTreeMap<_,_> = input.lines().map(|l|parse_line(l)).collect();

    let root_result = solve(&monkeys[&"root".to_string()],&monkeys);
    println!("{}",root_result);
    let unwrapped = unwrap("root".to_string(), &monkeys[&"root".to_string()], &monkeys);
    println!("{}",unwrapped);
    // At this point I just took the unwrapped equation and plugged into an online equation solver to solve for h
}