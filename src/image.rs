use pixels::{Pixel};

pub struct Image<T: Pixel> {
    width: u32,
    height: u32,
    pub buffer: Vec<T>,
}

// The T: Clone contstaint is so when the buffer is initialized the pixel can
// be cloned to fill the buffer.
impl<T: Pixel> Image<T> where T: Clone {
    pub fn new(width: u32, height: u32) -> Image<T> {
        Image {
            width: width,
            height: height,
            buffer: vec![T::new(); (width * height) as usize]
        }
    }

    pub fn raw_buffer(&self) -> Vec<u8> {
        self.buffer.iter().flat_map(|pixel| pixel.values()).collect()
    }

    pub fn pixel_at_mut(&mut self, x: u32, y: u32) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&mut self.buffer[(x + y * self.width) as usize])
        }
    }
}
