#![warn(clippy::all)]

use itertools::Itertools;

#[derive(Debug,Default,Clone)]
struct Head{
    x:i32,
    y:i32
}

impl Head{
    fn walk(&mut self, direction: &str){
        match direction {
            "R" => {self.x +=1},
            "L" => {self.x -=1},
            "U" => {self.y +=1},
            "D" => {self.y -=1},
            _ => ()
        }
    }
}

#[derive(Debug,Default)]
struct Tail{
    x: i32,
    y: i32,
    history: Vec<(i32,i32)>
}
impl Tail{
    fn distance_from(&self, head:&Head) -> (i32,i32){
        (head.x - self.x, head.y - self.y)
    }
    fn need_to_catch_up(&self, head:&Head) -> bool{
        let (horizontally, vertically) = self.distance_from(head);
        horizontally.abs() > 1 || vertically.abs() > 1
    }
    fn catch_up(&mut self, head:&Head){
        self.history.push((self.x.clone(),self.y.clone()));
        //Do we need to catch up vertically?
        if self.x == head.x{
            self.y += if head.y > self.y {1} else {-1};
        }else 
        //Do we need to catch up horizontally?
        if self.y == head.y{
            self.x += if head.x > self.x {1} else {-1};
        }else{ // We need to catch up diagonally
            self.y += if head.y > self.y {1} else {-1};
            self.x += if head.x > self.x {1} else {-1};
        }
    }
}

fn main(){
    let input = include_str!("input.txt");
    let mut head: Head = Default::default();
    let mut tails: Vec<Tail> = Default::default();
    for _ in 0..9{
        tails.push(Default::default());
    }
    for line in input.lines(){
        let (direction, amount) = sscanf::sscanf!(line.trim(), "{str} {i32}").unwrap();
        for _ in 0..amount{
            head.walk(direction);
            for i in 0..tails.len(){
                let head_to_use = if i==0 {head.clone()} else{Head{x:tails[i-1].x, y:tails[i-1].y}};
                if tails[i].need_to_catch_up(&head_to_use){
                    tails[i].catch_up(&head_to_use);
                }
            }
        }
    }
    for i in 0..tails.len(){
        let tail_curr = tails.get_mut(i).unwrap();
        tail_curr.history.push((tail_curr.x, tail_curr.y));
        println!("The {} tail visited {} locations", i+1, tail_curr.history.iter().unique().collect_vec().len());
    }
}
