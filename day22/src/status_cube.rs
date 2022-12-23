use crate::{common::{Direction, Tile}, cube::{Cube, Section}};

#[derive(Debug)]
pub struct StatusCube{
    pub col:usize,
    pub row:usize,
    pub dir:Direction,
    pub section:Section
}
impl StatusCube {
    pub fn go_forward(&mut self, forward_amount:usize, cube: &Cube){
        for _ in 0..forward_amount{
            let curr_section = &cube.faces[self.section];
            match self.dir {
                Direction::Right => {
                    if let Some(tile) = curr_section[self.row].get(self.col+1){
                        if *tile != Tile::Wall{
                            self.col +=1;
                        }
                    }else {
                        //Switch section
                        let (new_section,new_row,new_col,new_direction) = cube.warp(&self);
                        if cube.faces[new_section][new_row][new_col] != Tile::Wall{
                            self.section = new_section;
                            self.row = new_row;
                            self.col = new_col;
                            self.dir = new_direction;
                        }
                    }
                },
                Direction::Top => {
                    if self.row>0{
                        let tile_row = curr_section.get(self.row-1).unwrap();
                        if tile_row[self.col] != Tile::Wall{
                            self.row -=1;
                        }
                    }else {
                        // We're at row 0 and we're trying to go up. We need to warp
                        //Switch section
                        let (new_section,new_row,new_col,new_direction) = cube.warp(&self);
                        if cube.faces[new_section][new_row][new_col] != Tile::Wall{
                            self.section = new_section;
                            self.row = new_row;
                            self.col = new_col;
                            self.dir = new_direction;
                        }
                    }
                },
                Direction::Left => {
                    if self.col>0{
                        let tile = curr_section[self.row].get(self.col-1).unwrap();
                        if *tile != Tile::Wall{
                            self.col -=1;
                        }
                    }else {
                        // Column is 0 and we try to go left
                        //Switch section
                        let (new_section,new_row,new_col,new_direction) = cube.warp(&self);
                        if cube.faces[new_section][new_row][new_col] != Tile::Wall{
                            self.section = new_section;
                            self.row = new_row;
                            self.col = new_col;
                            self.dir = new_direction;
                        }
                    }
                },
                Direction::Bottom => {
                    if let Some(tile_row) = curr_section.get(self.row+1){
                        if tile_row[self.col] != Tile::Wall{
                            self.row +=1;
                        }
                    }else {
                        //Switch section
                        let (new_section,new_row,new_col,new_direction) = cube.warp(&self);
                        if cube.faces[new_section][new_row][new_col] != Tile::Wall{
                            self.section = new_section;
                            self.row = new_row;
                            self.col = new_col;
                            self.dir = new_direction;
                        }
                    }
                },
            }
        }
        println!("{:?}",self);
    }
    pub fn turn(&mut self, direction:char){
        self.dir = match direction {
            'L' => Direction::from_degrees(self.dir.to_degrees()+90),
            'R' => Direction::from_degrees(self.dir.to_degrees()-90),
            _=>todo!()
        }
    }
} 