#[inline(always)]
pub fn cord_1d_to_2d(cord: usize, width: usize) -> (usize, usize) {
    (cord % width, cord / width)
}

#[inline(always)]
pub fn cord_2d_to_1d(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}
