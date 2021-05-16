use std::error::Error;

use freetype::{face::LoadFlag, Face};
use graphics::{Context, Image, Transformed};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

pub struct Font {
    glyphs: Vec<Glyph>,
}

pub struct Glyph {
    pub texture: Texture,
    pub size: Size,
    pub bearing: Bearing,
    pub advance: f32,
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub struct Bearing {
    pub left: i32,
    pub top: i32,
}

pub struct Text {
    pub width: f64,
    pub height: f64,
    pub str: String,
}

pub enum Layout {
    Centered,
    LeftAlign,
}

impl Font {
    pub fn new(face: &Face) -> Result<Self, Box<dyn Error>> {
        let mut glyphs = Vec::new();
        for char_code in 0..127 {
            face.load_char(char_code, LoadFlag::RENDER)?;
            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            let vector = glyph.advance();

            let texture = Texture::from_memory_alpha(
                bitmap.buffer(),
                bitmap.width() as u32,
                bitmap.rows() as u32,
                &TextureSettings::new(),
            )
            .unwrap();

            let character = Glyph {
                texture,
                size: Size {
                    width: bitmap.width() as u32,
                    height: bitmap.rows() as u32,
                },
                bearing: Bearing {
                    // integer pixels, possibly negative.
                    left: glyph.bitmap_left(),
                    top: glyph.bitmap_top(),
                },
                advance: vector.x as f32 / 64.0,
            };
            glyphs.push(character);
        }

        Ok(Self { glyphs })
    }

    pub fn create_text(&self, text: &str) -> Result<Text, Box<dyn Error>> {
        let mut height: f64 = 0.0;
        let mut width: f64 = 0.0;
        for char in text.chars() {
            let codepoint = char as u32;
            if codepoint > 127 {
                return Err(format!("Non-ascii character {}", char).into());
            }
            let glyph = &self.glyphs[codepoint as usize];
            height = height.max(glyph.size.height as f64);
            width += glyph.advance as f64;
        }

        Ok(Text {
            width,
            height,
            str: text.to_owned(),
        })
    }

    pub fn render_lines(
        &self,
        lines: &Vec<Text>,
        layout: Layout,
        center: (f64, f64),
        color: [f32; 4],
        c: &Context,
        g: &mut GlGraphics,
    ) -> Result<(), Box<dyn Error>> {
        const TOP_PAD: f64 = 5.0;

        let width: f64 = lines.iter().map(|x| x.width).fold(0.0, |a, b| a.max(b));
        let height: f64 = lines.iter().map(|x| x.height).intersperse(TOP_PAD).sum();

        let mut top = center.1 - height / 2.0;
        for line in lines {
            let left = match &layout {
                Layout::Centered => center.0 - line.width / 2.0,
                Layout::LeftAlign => center.0 - width / 2.0,
            };

            self.render_line(&line, (left, top), color, &c, g)?;

            top += line.height + TOP_PAD;
        }

        Ok(())
    }

    fn render_line(
        &self,
        text: &Text,
        origin: (f64, f64),
        color: [f32; 4],
        c: &Context,
        gl: &mut GlGraphics,
    ) -> Result<(), Box<dyn Error>> {
        let mut x = origin.0;
        let y = origin.1;
        for char in text.str.chars() {
            let glyph = &self.glyphs[char as usize];
            // let offset_y = glyph.size.height as f64 - glyph.bearing.top as f64;
            let offset_y = -glyph.bearing.top as f64;
            let offset_x = glyph.bearing.left as f64;
            Image::new_color(color).draw(
                &glyph.texture,
                &c.draw_state,
                c.transform.trans(x + offset_x, y + offset_y as f64),
                gl,
            );
            x += glyph.advance as f64;
        }

        Ok(())
    }
}
