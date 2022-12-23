use std::fmt::Display;

#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub enum Tile {
    Warp,
    Wall,
    Move
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Tile::Warp => " ",
            Tile::Wall => "#",
            Tile::Move => ".",
        };
        write!(f, "{}",char)
    }
}
#[derive(Debug,Clone, Copy)]
pub enum Direction {
    Right,
    Top,
    Left,
    Bottom
}
impl Direction {
    pub fn to_degrees(&self)->i16{
        match self {
            Direction::Right => 0,
            Direction::Top => 90,
            Direction::Left => 180,
            Direction::Bottom => 270,
        }
    }
    pub fn from_degrees(degrees:i16)->Self{
        // Adding 360 in for the -90 degree case
        match (degrees+360)%360 {
            0=>Direction::Right,
            90=>Direction::Top,
            180=>Direction::Left,
            270=>Direction::Bottom,
            _=>todo!()
        }
    }
    pub fn to_score(&self)->usize{
        match self {
            Direction::Right => 0,
            Direction::Top => 3,
            Direction::Left => 2,
            Direction::Bottom => 1,
        }
    }
}