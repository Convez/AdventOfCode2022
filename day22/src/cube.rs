use std::ops::{Index, IndexMut};

use itertools::Itertools;

use crate::{common::{Tile, Direction}, status_cube::StatusCube};

#[derive(Debug,Clone, Copy)]
pub enum Section{
    Section1,
    Section2,
    Section3,
    Section4,
    Section5,
    Section6
}
impl<T> Index<Section> for Vec<T> {
    type Output = T;

    fn index(&self, index: Section) -> &Self::Output {
        &self[index as usize]
    }
}
impl<T> IndexMut<Section> for Vec<T> {
    fn index_mut(&mut self, index: Section) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

pub struct Cube{
    pub faces: Vec<Vec<Vec<Tile>>>,
    pub horizontal_size:usize,
    pub vertical_size:usize,
    is_test:bool
}
impl Cube{
    pub fn get_row_for_section(&self,section:&Section)->usize{
        if self.is_test{
            match section {
                Section::Section1 => 0,
                Section::Section2 => self.vertical_size,
                Section::Section3 => self.vertical_size,
                Section::Section4 => self.vertical_size,
                Section::Section5 => self.vertical_size*2,
                Section::Section6 => self.vertical_size*2,
            }
        }else {
            match section {
                Section::Section1 => 0,
                Section::Section2 => 0,
                Section::Section3 => self.vertical_size,
                Section::Section4 => self.vertical_size*2,
                Section::Section5 => self.vertical_size*2,
                Section::Section6 => self.vertical_size*3,
            }
        }
    }
    pub fn get_col_for_section(&self,section:&Section)->usize{
        if self.is_test{
            match section {
                Section::Section1 => self.horizontal_size*2,
                Section::Section2 => 0,
                Section::Section3 => self.horizontal_size,
                Section::Section4 => self.horizontal_size*2,
                Section::Section5 => self.horizontal_size*2,
                Section::Section6 => self.horizontal_size*3,
            }
        }else {
            match section {
                Section::Section1 => self.horizontal_size,
                Section::Section2 => self.horizontal_size*2,
                Section::Section3 => self.horizontal_size,
                Section::Section4 => 0,
                Section::Section5 => self.horizontal_size,
                Section::Section6 => 0,
            }
        }
    }
    pub fn from_map(map:&Vec<Vec<Tile>>, is_test:bool)->Self{
        if is_test{
            // 3 vertical sections
            let vertical_size = map.len()/3;
            // 4 horizontal sections
            let horizontal_size = map[0].len()/4;
            let mut cube = Self{ faces: Default::default(),horizontal_size,vertical_size,is_test };
            // Section 1
            cube.faces.push(map[0..vertical_size].iter().map(|i|i[horizontal_size*2..horizontal_size*3].iter().map(|t|*t).collect_vec()).collect_vec());
            // Section 2
            cube.faces.push(map[vertical_size..vertical_size*2].iter().map(|i|i[0..horizontal_size].iter().map(|t|*t).collect_vec()).collect_vec());
            // Section 3
            cube.faces.push(map[vertical_size..vertical_size*2].iter().map(|i|i[horizontal_size..horizontal_size*2].iter().map(|t|*t).collect_vec()).collect_vec());
            // Section 4
            cube.faces.push(map[vertical_size..vertical_size*2].iter().map(|i|i[horizontal_size*2..horizontal_size*3].iter().map(|t|*t).collect_vec()).collect_vec());
            // Section 5
            cube.faces.push(map[vertical_size*2..vertical_size*3].iter().map(|i|i[horizontal_size*2..horizontal_size*3].iter().map(|t|*t).collect_vec()).collect_vec());
            // Section 6
            cube.faces.push(map[vertical_size*2..vertical_size*3].iter().map(|i|i[horizontal_size*3..horizontal_size*4].iter().map(|t|*t).collect_vec()).collect_vec());
            cube.faces.iter().for_each(|f|assert!(f.len()==f[0].len()));
            cube
        }else{
            // 3 vertical sections
            let vertical_size = map.len()/4;
            // 4 horizontal sections
            let horizontal_size = map[0].len()/3;
            let mut cube = Self{ faces: Default::default(),horizontal_size,vertical_size, is_test };
            // Section 1
            cube.faces.push(map[0..vertical_size].iter().map(|i|i[horizontal_size..horizontal_size*2].iter().map(|t|*t).collect_vec()).collect_vec());
            // Section 2
            cube.faces.push(map[0..vertical_size].iter().map(|i|i[horizontal_size*2..horizontal_size*3].iter().map(|t|*t).collect_vec()).collect_vec());
            // Section 3
            cube.faces.push(map[vertical_size..vertical_size*2].iter().map(|i|i[horizontal_size..horizontal_size*2].iter().map(|t|*t).collect_vec()).collect_vec());
            // Section 4
            cube.faces.push(map[vertical_size*2..vertical_size*3].iter().map(|i|i[0..horizontal_size].iter().map(|t|*t).collect_vec()).collect_vec());
            // Section 5
            cube.faces.push(map[vertical_size*2..vertical_size*3].iter().map(|i|i[horizontal_size..horizontal_size*2].iter().map(|t|*t).collect_vec()).collect_vec());
            // Section 6
            cube.faces.push(map[vertical_size*3..vertical_size*4].iter().map(|i|i[0..horizontal_size].iter().map(|t|*t).collect_vec()).collect_vec());
            cube.faces.iter().for_each(|f|assert!(f.len()==f[0].len()));
            cube
        }
    }

    // Warp is called only if I'm at the edge for each direction, we can skip some sanity checks
    pub fn warp(&self,status:&StatusCube)->(Section,usize,usize,Direction){
        use Section::*;
        println!("Warping from {:?} in {:?}",status.section,status.dir);
        if self.is_test{
            match (status.section,status.dir) {
                (Section::Section1, Direction::Right) => (Section6,self.faces[Section6].len()-1-status.row,self.faces[Section6][0].len()-1,Direction::Left),
                (Section::Section1, Direction::Top) => (Section2,0,self.faces[Section2][0].len()-1-status.col,Direction::Bottom),
                (Section::Section1, Direction::Left) => (Section3,0,status.row,Direction::Bottom),
                (Section::Section1, Direction::Bottom) => (Section4,0,status.col,status.dir),
                (Section::Section2, Direction::Right) => (Section3,status.row,0,status.dir),
                (Section::Section2, Direction::Top) => (Section1,0,self.faces[Section1][0].len()-1-status.col,Direction::Bottom),
                (Section::Section2, Direction::Left) => (Section6,self.faces[Section6].len()-1,self.faces[Section6][0].len()-1-status.row,Direction::Top),
                (Section::Section2, Direction::Bottom) => (Section5,self.faces[Section5].len()-1,self.faces[Section5][0].len()-1-status.col,Direction::Top),
                (Section::Section3, Direction::Right) => (Section4,status.row,0,status.dir),
                (Section::Section3, Direction::Top) => (Section1,status.col,0,Direction::Right),
                (Section::Section3, Direction::Left) => (Section2,status.row,self.faces[Section2][0].len()-1,status.dir),
                (Section::Section3, Direction::Bottom) => (Section5,self.faces[Section5].len()-1-status.col,0,Direction::Right),
                (Section::Section4, Direction::Right) => (Section6,0,self.faces[Section6].len()-1-status.row,Direction::Bottom),
                (Section::Section4, Direction::Top) => (Section1,self.faces[Section1].len()-1,status.col,status.dir),
                (Section::Section4, Direction::Left) => (Section3,status.row,self.faces[Section3][0].len()-1,status.dir),
                (Section::Section4, Direction::Bottom) => (Section5,0,status.col,status.dir),
                (Section::Section5, Direction::Right) => (Section6,status.row,0,status.dir),
                (Section::Section5, Direction::Top) => (Section4,self.faces[Section4].len()-1,status.col,status.dir),
                (Section::Section5, Direction::Left) => (Section3,self.faces[Section3].len()-1,self.faces[Section3].len()-1-status.row,Direction::Top),
                (Section::Section5, Direction::Bottom) => (Section2,self.faces[Section2].len()-1,self.faces[Section2][0].len()-1-status.col,Direction::Top),
                (Section::Section6, Direction::Right) => (Section1,self.faces[Section1].len()-1-status.row,self.faces[Section1][0].len()-1,Direction::Left),
                (Section::Section6, Direction::Top) => (Section4,self.faces[Section4].len()-status.col-1,self.faces[Section4].len()-1,Direction::Left),
                (Section::Section6, Direction::Left) => (Section5,status.row,self.faces[Section5].len()-1,status.dir),
                (Section::Section6, Direction::Bottom) => (Section2,self.faces[Section2].len()-1-status.col,0,Direction::Right),
            }
        }else {
            /*
                1122
                1122
                33
                33
              4455
              4455
              66  
              66
            */
            match (status.section,status.dir) {
                (Section1, Direction::Right) => (Section2,status.row,0,Direction::Right),
                (Section1, Direction::Top) => (Section6,status.col,0,Direction::Right),
                (Section1, Direction::Left) => (Section4,self.faces[Section4].len()-1-status.row,0,Direction::Right),
                (Section1, Direction::Bottom) => (Section3,0,status.col,Direction::Bottom),
                (Section2, Direction::Right) => (Section5,self.faces[Section5].len()-1-status.row,self.faces[Section5][0].len()-1,Direction::Left),
                (Section2, Direction::Top) => (Section6,self.faces[Section6].len()-1,status.col,Direction::Top),
                (Section2, Direction::Left) => (Section1,status.row,self.faces[Section1][0].len()-1,Direction::Left),
                (Section2, Direction::Bottom) => (Section3,status.col,self.faces[Section3][0].len()-1,Direction::Left),
                (Section3, Direction::Right) => (Section2,self.faces[Section3].len()-1,status.row,Direction::Top),
                (Section3, Direction::Top) => (Section1,self.faces[Section1].len()-1,status.col,Direction::Top),
                (Section3, Direction::Left) => (Section4,0,status.row,Direction::Bottom),
                (Section3, Direction::Bottom) => (Section5,0,status.col,Direction::Bottom),
                (Section4, Direction::Right) => (Section5,status.row,0,Direction::Right),
                (Section4, Direction::Top) => (Section3,status.col,0,Direction::Right),
                (Section4, Direction::Left) => (Section1,self.faces[Section1].len()-1-status.row,0,Direction::Right),
                (Section4, Direction::Bottom) => (Section6,0,status.col,Direction::Bottom),
                (Section5, Direction::Right) => (Section2,self.faces[Section2].len()-1-status.row,self.faces[Section2][0].len()-1,Direction::Left),
                (Section5, Direction::Top) => (Section3,self.faces[Section3].len()-1,status.col,Direction::Top),
                (Section5, Direction::Left) => (Section4,status.row, self.faces[Section4][0].len()-1,Direction::Left),
                (Section5, Direction::Bottom) => (Section6,status.col,self.faces[Section6][0].len()-1,Direction::Left),
                (Section6, Direction::Right) => (Section5,self.faces[Section5].len()-1,status.row,Direction::Top),
                (Section6, Direction::Top) => (Section4,self.faces[Section4].len()-1,status.col,Direction::Top),
                (Section6, Direction::Left) => (Section1,0,status.row,Direction::Bottom),
                (Section6, Direction::Bottom) => (Section2,0,status.col,Direction::Bottom),
            }
        }
    }

    pub fn print_cube(&self){
        let mut section = 0;
        for f in &self.faces{
            println!("Section {}",section);
            for row in f{
                for col in row{
                    print!("{}",col);
                }
                println!()
            }
            section +=1;
        }
    }
}