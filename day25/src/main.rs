use itertools::Itertools;

fn snafu_symbol_to_number(symbol:char)->i64{
    match symbol {
        '2'=> 2,
        '1'=> 1,
        '0'=> 0,
        '-'=>-1,
        '='=>-2,
        _=>todo!()
    }
}

fn number_to_snafu_symbol(number:i64)->char{
    match number {
        0=>'0',
        1=>'1',
        2=>'2',
        3=>'=',
        4=>'-',
        _=>todo!()
    }
}

fn snafu_string_to_number(snafu:&str) -> i64{
    let mut number = 0;
    for i in 0..snafu.len(){
        let snaso_number = snafu_symbol_to_number(snafu.chars().nth(snafu.len()-i-1).unwrap());
        let base_multiply = (5 as i64).pow(i as u32);
        number += snaso_number * base_multiply;
    }
    number
}

fn number_to_snafu_string(number:i64) -> String{
    let mut n = number;
    let mut c = "".to_string();
    while n != 0{
        let i = n%5;
        if i > 2{
            n += 5-i;
        }
        n = n/5;
        c.push(number_to_snafu_symbol(i));
        
   }
   c.chars().rev().collect()
}
fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    
    let snaso = input.lines().map(|l|snafu_string_to_number(l.trim())).collect_vec();

    let fuel_sum = snaso.iter().sum::<i64>();
    println!("Fuel sum in decimal: {}",fuel_sum);
    let converted = number_to_snafu_string(fuel_sum);
    println!("Fuel sum in SNASO {}", converted);
}