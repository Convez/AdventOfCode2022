use itertools::Itertools;


fn vector_mix(operations: &mut Vec<(i64,usize)>, rounds: usize){
    // Size to use for the % operation, since after removing the old item, the size is smaller
    let circular_size = (operations.len()-1) as i64;
    // Perform n rounds of mizing
    for _ in 0..rounds{
        for id in 0..operations.len(){
            // Get current index of operation originally at index i
            let old_index = operations.iter().position(|(_,pos)| *pos == id).unwrap();
            let new_index = ((old_index as i64 + operations[old_index].0)%circular_size + circular_size)%circular_size;
            let operation = operations.remove(old_index);
            operations.insert(new_index as usize, operation);
        }
    }
    let zero_index = operations.iter().position(|(switch,_)|*switch ==0).unwrap();
    let element_1000 = operations[(zero_index + 1000)%operations.len()].0;
    let element_2000 = operations[(zero_index + 2000)%operations.len()].0;
    let element_3000 = operations[(zero_index + 3000)%operations.len()].0;
    println!("{} {} {}", element_1000,element_2000,element_3000);

    println!("The sum of the requested elements is {}", element_1000 + element_2000 + element_3000);
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    
    let mut operations = input.lines().map(|l|l.trim().parse::<i64>().unwrap())
    .enumerate().map(|(pos,shift)|(shift,pos)).collect_vec();
    let mut operations2 = operations.iter().map(|(shift,pos)|(shift*811589153,*pos)).collect_vec();
    if is_test{
        println!("{:?}",operations);
    }
    // Problem 1
    vector_mix(&mut operations, 1);

    //Problem 2
    vector_mix(&mut operations2, 10);
}