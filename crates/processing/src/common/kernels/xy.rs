use picturify_core::geometry::coord::Coord;
use crate::common::kernels::convolution::ConvolutionKernel;

#[derive(Clone)]
pub struct XyKernels {
    x: ConvolutionKernel,
    y: ConvolutionKernel,
}

impl XyKernels {
    pub fn new(x: ConvolutionKernel, y: ConvolutionKernel) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn get_x(&self, coord: Coord) -> f32 {
        self.x.get(coord)
    }

    #[inline(always)]
    pub fn get_y(&self, coord: Coord) -> f32 {
        self.y.get(coord)
    }

    pub fn get_x_kernel(&self) -> &ConvolutionKernel {
        &self.x
    }

    pub fn get_y_kernel(&self) -> &ConvolutionKernel {
        &self.y
    }

    pub fn radius(&self) -> usize {
        self.x.radius()
    }

    pub fn validate(&self) -> bool {
        self.x.validate() && self.y.validate() && self.x.size() == self.y.size()
    }

    pub fn iter(&self) -> XyKernelsIterator {
        let (width, height): (usize, usize) = self.x.size().into();
        XyKernelsIterator {
            xy_kernels: self,
            current_x: 0,
            current_y: 0,
            width,
            height,
        }
    }
}

pub struct XyKernelsIterator<'a> {
    xy_kernels: &'a XyKernels,
    current_x: usize,
    current_y: usize,
    width: usize,
    height: usize,
}

impl<'a> Iterator for XyKernelsIterator<'a> {
    type Item = (Coord, f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.height {
            return None;
        }

        let coord = Coord::new(self.current_x as i32, self.current_y as i32);
        let x_value = self.xy_kernels.get_x(coord);
        let y_value = self.xy_kernels.get_y(coord);

        self.current_x += 1;
        if self.current_x >= self.width {
            self.current_x = 0;
            self.current_y += 1;
        }

        Some((coord, x_value, y_value))
    }
}
