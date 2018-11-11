use pixels::{Pixel};

pub struct Image<T: Pixel> {
    buffer: Vec<T>,
}

// The T: Clone contstaint is so when the buffer is initialized the pixel can
// be cloned to fill the buffer.
impl<T: Pixel> Image<T> where T: Clone {
    pub fn new(length: usize) -> Image<T> {
        Image {
            buffer: vec![T::new(); length]
        }
    }

    pub fn raw_buffer(&self) -> Vec<u8> {
        self.buffer.iter().flat_map(|pixel| pixel.values()).collect()
    }
}
