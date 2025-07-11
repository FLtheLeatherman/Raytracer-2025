use stb_image::image;

pub const BYTES_PER_PIXEL: usize = 3;
static MAGENTA: [u8; BYTES_PER_PIXEL] = [255, 0, 255];

#[derive(Default)]
pub struct RtwImage {
    data: Vec<u8>,
    pub image_width: usize,
    pub image_height: usize,
    bytes_per_scanline: usize,
}
impl RtwImage {
    pub fn new(image_filename: &str) -> Self {
        let filename = image_filename;
        let imagedir = std::env::var("RTW_IMAGES").unwrap_or_else(|_| String::from("assets"));
        let mut _self = Self::default();
        if !imagedir.is_empty() && _self.load(&format!("{}/{}", imagedir, filename)) {
            return _self;
        }
        if _self.load(filename) {
            return _self;
        }
        if _self.load(&format!("assets/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../../images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../../../images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../../../../images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../../../../../images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../../../../../../images/{}", filename)) {
            return _self;
        }
        panic!("ERROR: Could not load image file \"{}\".", filename);
    }
    pub fn load(&mut self, filename: &str) -> bool {
        let load_result = image::load_with_depth(filename, BYTES_PER_PIXEL, false);
        match load_result {
            image::LoadResult::Error(_) => false,
            image::LoadResult::ImageU8(image) => {
                assert_eq!(image.depth, BYTES_PER_PIXEL);
                self.data = image.data;
                self.image_width = image.width;
                self.image_height = image.height;
                self.bytes_per_scanline = image.depth * image.width;
                true
            }
            image::LoadResult::ImageF32(_) => false,
        }
    }
    pub fn width(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.image_width
        }
    }
    pub fn height(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.image_height
        }
    }
    fn clamp(x: usize, low: usize, high: usize) -> usize {
        if x < low {
            return low;
        }
        if x < high {
            return x;
        }
        high - 1
    }
    pub fn pixel_data(&self, x: usize, y: usize) -> &[u8] {
        if self.data.is_empty() {
            &MAGENTA
        } else {
            let x = Self::clamp(x, 0, self.image_width);
            let y = Self::clamp(y, 0, self.image_height);
            &self.data[(y * self.bytes_per_scanline) + (x * BYTES_PER_PIXEL)
                ..(y * self.bytes_per_scanline) + (x * BYTES_PER_PIXEL) + BYTES_PER_PIXEL]
        }
    }
}
