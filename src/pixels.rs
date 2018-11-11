pub trait Pixel {
    fn new() -> Self;
    fn values(&self) -> Vec<u8>;
}

#[derive(Debug, Clone, Copy)]
pub struct RGBAPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct RGBPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel for RGBPixel {
    fn new() -> RGBPixel {
        RGBPixel { r: 0, g: 0, b: 0 }
    }

    fn values(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }
}

impl Pixel for RGBAPixel {
    fn new() -> RGBAPixel {
        RGBAPixel { r: 0, g: 0, b: 0, a: 0 }
    }

    fn values(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b, self.a]
    }
}
