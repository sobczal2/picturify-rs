use crate::common::angle::Angle;

pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Size { width, height }
    }
    
    pub fn rotate(&self, angle: Angle) -> Self {
        let radians = angle.to_radians();
        let width = self.width as f32;
        let height = self.height as f32;

        let new_width = width * radians.cos().abs() + height * radians.sin().abs();
        let new_height = width * radians.sin().abs() + height * radians.cos().abs();
        
        let new_width = new_width.round() as usize;
        let new_height = new_height.round() as usize;
        
        Size::new(new_width, new_height)
    }
}