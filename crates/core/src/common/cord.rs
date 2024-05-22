use std::ops::{Add, AddAssign, Div, DivAssign, Sub, SubAssign};
use crate::common::angle::Angle;

#[derive(Debug, Copy, Clone)]
pub struct Cord {
    x: i32,
    y: i32,
}

impl Cord {
    pub fn new(x: usize, y: usize) -> Self {
        Cord { x: x as i32, y: y as i32 }
    }
    
    pub fn from_i32(x: i32, y: i32) -> Self {
        Cord { x, y }
    }
    
    pub fn get_x(&self) -> usize {
        self.x as usize
    }
    
    pub fn get_y(&self) -> usize {
        self.y as usize
    }

    pub fn rotate(&self, angle: Angle, origin: Cord) -> Self {
        let x = self.x - origin.x;
        let y = self.y - origin.y;
        let (sin, cos) = angle.to_sin_cos();
        let new_x = x as f32 * cos - y as f32 * sin;
        let new_y = x as f32 * sin + y as f32 * cos;
        Cord::from_i32((new_x + origin.x as f32) as i32, (new_y + origin.y as f32) as i32)
    }
    
    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < width as i32 && self.y < height as i32
    }
}

impl Add for Cord {
    type Output = Cord;
    
    fn add(self, other: Cord) -> Cord {
        Cord::from_i32(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Cord {
    fn add_assign(&mut self, other: Cord) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Cord {
    type Output = Cord;
    
    fn sub(self, other: Cord) -> Cord {
        Cord::from_i32(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Cord {
    fn sub_assign(&mut self, other: Cord) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Div<i32> for Cord {
    type Output = Cord;
    
    fn div(self, other: i32) -> Cord {
        Cord::from_i32(self.x / other, self.y / other)
    }
}

impl DivAssign<i32> for Cord {
    fn div_assign(&mut self, other: i32) {
        self.x /= other;
        self.y /= other;
    }
}