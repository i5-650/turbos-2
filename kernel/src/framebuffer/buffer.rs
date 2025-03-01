use bootloader_api::info::{FrameBufferInfo, PixelFormat};

use core::{fmt, ptr};

use crate::framebuffer::Color;

use font_constants::BACKUP_CHAR;

use lazy_static::lazy_static;

use noto_sans_mono_bitmap::{
    FontWeight, RasterHeight, RasterizedChar, get_raster, get_raster_width,
};

use spin::Mutex;

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;

mod font_constants {
    use super::{FontWeight, RasterHeight, get_raster_width};

    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);

    pub const BACKUP_CHAR: char = '�';

    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        framebuffer: None,
        info: None,
        x: BORDER_PADDING,
        y: BORDER_PADDING,
        color: Color::WHITE
    });
}

pub fn init(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let mut writer = WRITER.lock();
    writer.framebuffer = Option::from(buffer);
    writer.info = Option::from(info);
    writer.clear();
}

fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(
            c,
            font_constants::FONT_WEIGHT,
            font_constants::CHAR_RASTER_HEIGHT,
        )
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

pub struct Writer {
    framebuffer: Option<&'static mut [u8]>,
    info: Option<FrameBufferInfo>,
    x: usize,
    y: usize,
    color: Color,
}

impl Writer {
    #[inline]
    fn newline(&mut self) {
        self.y += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return()
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    #[inline]
    fn carriage_return(&mut self) {
        self.x = BORDER_PADDING;
    }

    pub fn clear(&mut self) {
        self.x = BORDER_PADDING;
        self.y = BORDER_PADDING;

        if let Some(buffer) = self.framebuffer.as_mut() {
            buffer.fill(0);
        }
    }

    #[inline]
    fn width(&self) -> usize {
        self.info
            .expect("FrameBuffer should always be present")
            .width
    }

    #[inline]
    fn height(&self) -> usize {
        self.info
            .expect("FrameBuffer should always be present")
            .height
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let next_x = self.x + font_constants::CHAR_RASTER_WIDTH;
                if next_x >= self.width() {
                    self.newline();
                }
                let next_y = self.y + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if next_y >= self.height() {
                    self.clear();
                }
                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x + x, self.y + y, *byte);
            }
        }
        self.x += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.unwrap().stride + x;
        let color = self.color;

        let r = (color.r as u16 * intensity as u16 / 255) as u8;
        let g = (color.g as u16 * intensity as u16 / 255) as u8;
        let b = (color.b as u16 * intensity as u16 / 255) as u8;

        let pixel_color = match self.info.unwrap().pixel_format {
            PixelFormat::Rgb => [r, g, b, 0],
            PixelFormat::Bgr => [b, g, r, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            other => panic!("Pixel format {:?} not supported in logger", other),
        };

        let bytes_per_pixel = self.info.unwrap().bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;

        if let Some(buffer) = self.framebuffer.as_mut() {
            buffer[byte_offset..(byte_offset + bytes_per_pixel)]
                .copy_from_slice(&pixel_color[..bytes_per_pixel]);

            let _ = unsafe { ptr::read_volatile(&buffer[byte_offset]) };
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
