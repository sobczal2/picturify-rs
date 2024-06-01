use crate::geometry::angle::Angle;

#[derive(Debug, Clone, Copy)]
pub struct Size {
    width: usize,
    height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Size { width, height }
    }

    pub fn rotate(self, angle: Angle) -> Self {
        let radians = angle.to_radians();
        let width = self.width as f32;
        let height = self.height as f32;

        let new_width = width * radians.cos().abs() + height * radians.sin().abs();
        let new_height = width * radians.sin().abs() + height * radians.cos().abs();

        let new_width = new_width.round() as usize;
        let new_height = new_height.round() as usize;

        Size::new(new_width, new_height)
    }

    pub fn rotate_90(self) -> Self {
        Size::new(self.height, self.width)
    }

    pub fn increase_by(self, width: usize, height: usize) -> Self {
        Size::new(self.width + width, self.height + height)
    }
}

impl From<(usize, usize)> for Size {
    fn from((width, height): (usize, usize)) -> Self {
        Size::new(width, height)
    }
}

impl From<(u32, u32)> for Size {
    fn from((width, height): (u32, u32)) -> Self {
        Size::new(width as usize, height as usize)
    }
}

impl From<(i32, i32)> for Size {
    fn from((width, height): (i32, i32)) -> Self {
        Size::new(width as usize, height as usize)
    }
}

impl From<Size> for (usize, usize) {
    fn from(size: Size) -> (usize, usize) {
        (size.width, size.height)
    }
}

impl From<Size> for (u32, u32) {
    fn from(size: Size) -> (u32, u32) {
        (size.width as u32, size.height as u32)
    }
}

impl From<Size> for (i32, i32) {
    fn from(size: Size) -> (i32, i32) {
        (size.width as i32, size.height as i32)
    }
}
