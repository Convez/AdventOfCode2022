use crate::common::{Direction, Tile};

#[derive(Debug)]
pub struct StatusMap{
    pub col:usize,
    pub row:usize,
    pub dir:Direction
}
impl StatusMap {
    fn move_horizontally<'a, I>(&mut self, amount:usize, curr_it:&mut I)
    where
        I: Iterator<Item = (usize,&'a Tile)>,{
        for _ in 0..amount{
            if let Some(next_tile) = self.get_next_tile_h_map(curr_it){
                self.col = next_tile.0;
            }else {
                return;
            }
        }
    }

    fn move_vertically<'a, I>(&mut self, amount:usize, curr_it:&mut I)
    where
        I: Iterator<Item = (usize,&'a Vec<Tile>)>,{
        for _ in 0..amount{
            if let Some(next_tile) = self.get_next_tile_v_map(curr_it){
                self.row = next_tile.0;
            }else {
                return;
            }
        }
    }

    fn get_next_tile_h_map<'a, I> (&self, curr_it:&mut I) -> Option<(usize,Tile)>
    where
        I: Iterator<Item = (usize,&'a Tile)>,{
        let next_tile = curr_it.next().unwrap();
        // println!("{:?}",next_tile);
        if *next_tile.1 == Tile::Wall{
            return None;
        }
        if *next_tile.1 == Tile::Move{
            return Some((next_tile.0.clone(), next_tile.1.clone()));
        }
        // tile is warp
        loop {
            let next_tile = curr_it.next().unwrap();
            // println!("{:?}",next_tile);
            if *next_tile.1 == Tile::Wall{
                return None;
            }
            if *next_tile.1 == Tile::Move{
                return Some((next_tile.0.clone(), next_tile.1.clone()));
            }
        }
    }

    fn get_next_tile_v_map<'a, I> (&self, curr_it:&mut I) -> Option<(usize,Tile)>
    where
        I: Iterator<Item = (usize,&'a Vec<Tile>)>,{
        let next_tile = curr_it.next().unwrap();
        // println!("{} {:?}",next_tile.0, next_tile.1[self.col]);
        if next_tile.1[self.col] == Tile::Wall{
            return None;
        }
        if next_tile.1[self.col] == Tile::Move{
            return Some((next_tile.0.clone(), next_tile.1[self.col].clone()));
        }
        // tile is warp
        loop {
            let next_tile = curr_it.next().unwrap();
            if next_tile.1[self.col] == Tile::Wall{
                return None;
            }
            if next_tile.1[self.col] == Tile::Move{
                return Some((next_tile.0.clone(), next_tile.1[self.col].clone()));
            }
        }
    }

    pub fn go_forward_map(&mut self, amount:usize, map:&Vec<Vec<Tile>>){
        match self.dir {
            Direction::Left => {
                let mut curr_it = map[self.row].iter().enumerate().rev().cycle();
                for _ in self.col..map[self.row].len(){ curr_it.next();}
                self.move_horizontally(amount, &mut curr_it);
            },
            Direction::Right =>{
                let mut curr_it = map[self.row].iter().enumerate().cycle();
                for _ in 0..=self.col{ curr_it.next();}
                self.move_horizontally(amount, &mut curr_it);
            },
            Direction::Top => {
                let mut curr_it = map.iter().enumerate().rev().cycle();
                for _ in self.row..map.len() { curr_it.next(); }
                self.move_vertically(amount, &mut curr_it);
            },
            Direction::Bottom => {
                let mut curr_it = map.iter().enumerate().cycle();
                for _ in 0..=self.row{curr_it.next();}
                self.move_vertically(amount, &mut curr_it);
            },
        }
    }
    pub fn turn(&mut self, direction:char){
        self.dir = match direction {
            'L' => Direction::from_degrees(self.dir.to_degrees()+90),
            'R' => Direction::from_degrees(self.dir.to_degrees()-90),
            _=>todo!()
        }
    }
}