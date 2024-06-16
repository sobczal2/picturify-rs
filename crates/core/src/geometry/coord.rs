use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::geometry::angle::Angle;
use crate::geometry::size::Size;

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    #[inline(always)]
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    #[inline(always)]
    pub fn from_1d_index(index: usize, width: usize) -> Self {
        Coord::new((index % width) as i32, (index / width) as i32)
    }

    #[inline(always)]
    pub fn rotate(&self, angle: Angle, origin: Coord) -> Coord {
        let x = (self.x - origin.x) as f32;
        let y = (self.y - origin.y) as f32;
        let (sin, cos) = angle.to_sin_cos();
        let new_x = x * cos - y * sin;
        let new_y = x * sin + y * cos;
        (
            new_x.round() as i32 + origin.x,
            new_y.round() as i32 + origin.y,
        )
            .into()
    }

    #[inline(always)]
    pub fn in_bounds(&self, size: Size) -> bool {
        let (width, height) = size.into();
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }

    #[inline(always)]
    pub fn to_index(&self, width: i32) -> usize {
        (self.y * width + self.x) as usize
    }

    #[inline(always)]
    pub fn x(&self) -> i32 {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> i32 {
        self.y
    }

    #[inline(always)]
    pub fn array_index(&self, width: usize) -> usize {
        (self.y as usize) * width + (self.x as usize)
    }
}

impl Add for Coord {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Coord::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Coord {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Coord {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Coord::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Coord {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Sub<i32> for Coord {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: i32) -> Self {
        Coord::new(self.x - other, self.y - other)
    }
}

impl SubAssign<i32> for Coord {
    #[inline(always)]
    fn sub_assign(&mut self, other: i32) {
        self.x -= other;
        self.y -= other;
    }
}

impl Div<i32> for Coord {
    type Output = Self;

    #[inline(always)]
    fn div(self, other: i32) -> Self {
        Coord::new(self.x / other, self.y / other)
    }
}

impl DivAssign<i32> for Coord {
    #[inline(always)]
    fn div_assign(&mut self, other: i32) {
        self.x /= other;
        self.y /= other;
    }
}

impl Mul<i32> for Coord {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: i32) -> Self {
        Coord::new(self.x * other, self.y * other)
    }
}

impl MulAssign<i32> for Coord {
    #[inline(always)]
    fn mul_assign(&mut self, other: i32) {
        self.x *= other;
        self.y *= other;
    }
}

impl PartialEq for Coord {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<(i32, i32)> for Coord {
    #[inline(always)]
    fn from((x, y): (i32, i32)) -> Self {
        Coord::new(x, y)
    }
}

impl From<(u32, u32)> for Coord {
    #[inline(always)]
    fn from((x, y): (u32, u32)) -> Self {
        Coord::new(x as i32, y as i32)
    }
}

impl From<(usize, usize)> for Coord {
    #[inline(always)]
    fn from((x, y): (usize, usize)) -> Self {
        Coord::new(x as i32, y as i32)
    }
}

impl From<Coord> for (i32, i32) {
    #[inline(always)]
    fn from(coord: Coord) -> Self {
        (coord.x, coord.y)
    }
}

impl From<Coord> for (u32, u32) {
    #[inline(always)]
    fn from(coord: Coord) -> Self {
        (coord.x as u32, coord.y as u32)
    }
}

impl From<Coord> for (usize, usize) {
    #[inline(always)]
    fn from(coord: Coord) -> Self {
        (coord.x as usize, coord.y as usize)
    }
}
