use uefi::proto::console::gop::BltPixel;

#[derive(Clone)]
pub enum Glyph {
    Narrow([BltPixel; 8 * 16]),
    Wide([BltPixel; 16 * 16]),
}

impl Glyph {
    pub fn from_pixels<T>(pixels: T) -> Result<Glyph, &'static str>
    where
        T: AsRef<[BltPixel]>,
    {
        let pixels = pixels.as_ref();
        Ok(match pixels.len() {
            128 => {
                let mut buffer = [BltPixel::new(0, 0, 0); 8 * 16];
                buffer.copy_from_slice(&pixels);
                Glyph::Narrow(buffer)
            }
            256 => {
                let mut buffer = [BltPixel::new(0, 0, 0); 16 * 16];
                buffer.copy_from_slice(&pixels);
                Glyph::Wide(buffer)
            }
            _ => return Err("Invalid number of pixels"),
        })
    }

    pub fn metrics(&self) -> (usize, usize) {
        let width = match self {
            Glyph::Narrow(..) => 8,
            Glyph::Wide(..) => 16,
        };
        let height = 16;
        (width, height)
    }

    pub fn buffer(&self) -> &[BltPixel] {
        match self {
            Glyph::Narrow(buffer) => buffer,
            Glyph::Wide(buffer) => buffer,
        }
    }
}
