use crate::geometry::angle::Angle;
use crate::geometry::size::Size;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }
    #[inline]
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
    #[inline]
    pub fn in_bounds(&self, size: Size) -> bool {
        let (width, height) = size.into();
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }
    #[inline]
    pub fn to_index(&self, width: i32) -> usize {
        (self.y * width + self.x) as usize
    }
}

impl Add for Coord {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Coord::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Coord {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Coord {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Coord::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Coord {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Div<i32> for Coord {
    type Output = Self;

    #[inline]
    fn div(self, other: i32) -> Self {
        Coord::new(self.x / other, self.y / other)
    }
}

impl DivAssign<i32> for Coord {
    #[inline]
    fn div_assign(&mut self, other: i32) {
        self.x /= other;
        self.y /= other;
    }
}

impl Mul<i32> for Coord {
    type Output = Self;

    #[inline]
    fn mul(self, other: i32) -> Self {
        Coord::new(self.x * other, self.y * other)
    }
}

impl MulAssign<i32> for Coord {
    #[inline]
    fn mul_assign(&mut self, other: i32) {
        self.x *= other;
        self.y *= other;
    }
}

impl PartialEq for Coord {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<(i32, i32)> for Coord {
    #[inline]
    fn from((x, y): (i32, i32)) -> Self {
        Coord::new(x, y)
    }
}

impl From<(u32, u32)> for Coord {
    #[inline]
    fn from((x, y): (u32, u32)) -> Self {
        Coord::new(x as i32, y as i32)
    }
}

impl From<(usize, usize)> for Coord {
    #[inline]
    fn from((x, y): (usize, usize)) -> Self {
        Coord::new(x as i32, y as i32)
    }
}

impl From<Coord> for (i32, i32) {
    #[inline]
    fn from(coord: Coord) -> Self {
        (coord.x, coord.y)
    }
}

impl From<Coord> for (u32, u32) {
    #[inline]
    fn from(coord: Coord) -> Self {
        (coord.x as u32, coord.y as u32)
    }
}

impl From<Coord> for (usize, usize) {
    #[inline]
    fn from(coord: Coord) -> Self {
        (coord.x as usize, coord.y as usize)
    }
}
