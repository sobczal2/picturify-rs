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
            current: 0,
            width,
            total_size: width * height,
        }
    }
}

pub struct XyKernelsIterator<'a> {
    xy_kernels: &'a XyKernels,
    current: usize,
    width: usize,
    total_size: usize,
}

impl<'a> Iterator for XyKernelsIterator<'a> {
    type Item = (Coord, f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.total_size {
            return None;
        }

        let coord= Coord::from_1d_index(self.current, self.width);
        let x_value = self.xy_kernels.get_x(coord);
        let y_value = self.xy_kernels.get_y(coord);
        
        self.current += 1;

        Some((coord, x_value, y_value))
    }
}
