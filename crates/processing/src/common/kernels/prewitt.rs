use crate::common::kernels::convolution::ConvolutionKernel;
use crate::common::kernels::xy::XyKernels;
use picturify_core::error::processing::{ProcessingPicturifyError, ProcessingPicturifyResult};
use picturify_core::utils::vec::rotate_left_2d;

pub struct PrewittKernels;

impl PrewittKernels {
    pub fn create() -> ProcessingPicturifyResult<XyKernels> {
        let x = ConvolutionKernel::new(Self::create_prewitt_kernel_x())?;
        let y = ConvolutionKernel::new(Self::create_prewitt_kernel_y())?;

        let kernel = XyKernels::new(x, y);

        if kernel.validate() {
            Ok(kernel)
        } else {
            Err(ProcessingPicturifyError::InvalidKernel)
        }
    }
    fn create_prewitt_kernel_x() -> Vec<Vec<f32>> {
        vec![
            vec![1.0, 1.0, 1.0],
            vec![0.0, 0.0, 0.0],
            vec![-1.0, -1.0, -1.0],
        ]
    }
    fn create_prewitt_kernel_y() -> Vec<Vec<f32>> {
        rotate_left_2d(Self::create_prewitt_kernel_x())
    }
}
