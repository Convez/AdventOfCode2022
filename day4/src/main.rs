#![warn(clippy::all)]
use std::ops::Range;

fn range_contains(r1: &Range<u64>, r2: &Range<u64>) -> bool{
    (r1.start <= r2.start) && (r1.end >= r2.end)
}

fn range_overlaps(r1: &Range<u64>, r2: &Range<u64>) -> bool{
    (r1.start>=r2.start && r1.start<=r2.end) || (r1.end>=r2.start && r1.end <= r2.end)
}

fn main(){
    let input = include_str!("input.txt");
    let mut num_containing = 0;
    let mut num_overlapping = 0;
    for line in input.lines(){
        let mut ranges = line.split(",");
        let mut range1_nums = ranges.next().unwrap().split("-");
        let range1 = (range1_nums.next().unwrap().parse::<u64>().unwrap())..(range1_nums.next().unwrap().parse::<u64>().unwrap());
        let mut range2_nums = ranges.next().unwrap().split("-");
        let range2 = (range2_nums.next().unwrap().parse::<u64>().unwrap())..(range2_nums.next().unwrap().parse::<u64>().unwrap());
        num_containing += (range_contains(&range1, &range2) || range_contains(&range2, &range1) ) as u64;
        num_overlapping += (range_overlaps(&range1, &range2) || range_overlaps(&range2, &range1) ) as u64;
    }
    println!("The number of work locations that fully overlap is {}", num_containing);
    println!("The number of work locations that (at least) partially overlap is {}", num_overlapping);
}
