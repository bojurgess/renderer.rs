type Result<T, BufferError> = std::result::Result<T, BufferError>;

#[derive(Debug, Clone, Copy)]
enum BufferError {
    OutOfBounds,
}

impl std::fmt::Display for BufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

pub trait Buffer {
    fn new(width: usize, height: usize) -> Self;
    fn clear(&mut self);
}

mod ops {
    use super::BufferError;

    pub trait Fill<T> {
        fn fill(&mut self, color: T);
    }

    pub trait SetPixel<T> {
        fn set_pixel(&mut self, x: usize, y: usize, color: T) -> Result<(), BufferError>;
    }

    pub trait GetPixel<T> {
        fn get_pixel(&self, x: usize, y: usize) -> Result<T, BufferError>;
    }

    pub trait ToArray<T> {
        fn to_array(&self) -> Result<&[T], BufferError>;
    }
}

#[derive(Debug)]
pub struct FrameBuffer {
    pub width:  usize,
    pub height: usize,
    buffer:     Vec<u32>,
}

impl Buffer for FrameBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    fn clear(&mut self) {
        self.buffer.fill(0);
    }
}

impl ops::Fill<u32> for FrameBuffer {
    fn fill(&mut self, color: u32) {
        self.buffer.fill(color);
    }
}

impl ops::SetPixel<u32> for FrameBuffer {
    fn set_pixel(&mut self, x: usize, y: usize, color: u32) -> Result<(), BufferError> {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
            Ok(())
        } else {
            Err(BufferError::OutOfBounds)
        }
    }
}

impl ops::GetPixel<u32> for FrameBuffer {
    fn get_pixel(&self, x: usize, y: usize) -> std::result::Result<u32, BufferError> {
        if x < self.width && y < self.height {
            Ok(self.buffer[y * self.width + x])
        } else {
            Err(BufferError::OutOfBounds)
        }
    }
}

impl ops::ToArray<u32> for FrameBuffer {
    fn to_array(&self) -> std::result::Result<&[u32], BufferError> {
        Ok(&self.buffer)
    }
}

#[derive(Debug)]
pub struct DepthBuffer {
    pub width:  usize,
    pub height: usize,
    buffer:     Vec<f32>,
}

impl Buffer for DepthBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![f32::INFINITY; width * height],
        }
    }

    fn clear(&mut self) {
        self.buffer.fill(f32::INFINITY);
    }
}

impl ops::SetPixel<f32> for DepthBuffer {
    fn set_pixel(&mut self, x: usize, y: usize, depth: f32) -> Result<(), BufferError> {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = depth;
            Ok(())
        } else {
            Err(BufferError::OutOfBounds)
        }
    }
}

impl ops::GetPixel<f32> for DepthBuffer {
    fn get_pixel(&self, x: usize, y: usize) -> std::result::Result<f32, BufferError> {
        if x < self.width && y < self.height {
            Ok(self.buffer[y * self.width + x])
        } else {
            Err(BufferError::OutOfBounds)
        }
    }
}
