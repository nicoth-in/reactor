pub use std::ffi::CString;
pub use std::mem;
pub use std::io;
pub use std::time::Duration;
pub use std::time::SystemTime;
pub use std::time::Instant;
pub use std::io::Read;
pub use std::thread;
pub use std::ptr;
pub use std::str;
pub use std::sync::mpsc;

// pub use colored::*;

pub use glm::*;
pub use glm::ext::*;
pub use glm::builtin::*;

pub use std::ops::Add;
pub use std::ops::Neg;
pub use std::ops::Mul;

pub use ggez::*;
pub use ggez::event::*;
pub use ggez::graphics::*;
pub use ggez::filesystem::*;
pub use ggez::nalgebra as na;

mod params;
pub use params::*;

mod builder;
pub use builder::*;

pub use rand::prelude::*;


#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Element {
    top: f32,
    left: f32,
    right: f32,
    bottom: f32,
    height: f32,
    width: f32,

    border_width: f32,
    border_style: f32,

    border_top_left_radius: f32,
    border_top_right_radius: f32,
    border_bottom_left_radius: f32,
    border_bottom_right_radius: f32,

    border_color: ColorRGBA,


    background_color: ColorRGBA,
    pub key: usize,
}

#[allow(dead_code)]
impl Element {
    pub fn new(t: f32, l: f32, w: f32, h: f32) -> Element {
        Element {
            top: t,
            left: l,
            right: 0.,
            bottom: 0.,
            height: h,
            width: w,

            border_width: 0.,
            border_style: 0.,

            border_top_left_radius: 0.,
            border_top_right_radius: 0.,
            border_bottom_left_radius: 0.,
            border_bottom_right_radius: 0.,

            border_color: ColorRGBA::new(0, 0, 0, 0),

            background_color: ColorRGBA::new(0, 0, 0, 0),

            key: rand::random::<usize>(),
         }
    }
    // Setters
    pub fn set_background_color(&mut self, c: ColorRGBA) {
        self.background_color = c;
    }
    pub fn set_border(&mut self, w: f32, s: f32, c: ColorRGBA) {
        self.border_width = w;
        self.border_style = s;
        self.border_color = c;
    }
    // Draw
    pub fn draw_pixel(&mut self, x: f32, y: f32) -> ColorRGBA {

        // Calc pos
        let ex = self.left + self.width;
        let ey = self.top + self.height;

        let bx = self.left + self.width + self.border_width;
        let by = self.top + self.height + self.border_width;

        //Border params
        let cx = self.left;
        let cy = self.top;

        let vx = self.left + self.width;
        let vy = self.top + self.height;

        let dot_step: usize = 5;

        // Pre-color
        let mut pixel_color = self.background_color.clone();

        // If is rendering
        if ((x as f32) <= ex) & ((x as f32) >= self.left) {
            if ((y as f32) <= ey) & ((y as f32) >= self.top) {

                // Rounds
                if ((x as f32) < (cx + self.border_top_left_radius)) & ((y as f32) < (cy + self.border_top_left_radius)) {
                    if (x - cx - self.border_top_left_radius).powf(2.0) + (y - cy - self.border_top_left_radius).powf(2.0) > self.border_top_left_radius.powf(2.0) {
                        // Round top-left
                        pixel_color = ColorRGBA::new(0, 0, 0, 0);
                    }
                }
                if ((x as f32) < (cx + self.border_bottom_left_radius)) & ((y as f32) > (vy - self.border_bottom_left_radius)) {
                    if (x - cx - self.border_bottom_left_radius).powf(2.0) + (y - vy + self.border_bottom_left_radius).powf(2.0) > self.border_bottom_left_radius.powf(2.0) {
                        // Round bottom-left
                        pixel_color = ColorRGBA::new(0, 0, 0, 0);
                    }
                }
                if ((x as f32) > (vx - self.border_bottom_right_radius)) & ((y as f32) > (vy - self.border_bottom_right_radius)) {
                    if (x - vx + self.border_bottom_right_radius).powf(2.0) + (y - vy + self.border_bottom_right_radius).powf(2.0) > self.border_bottom_right_radius.powf(2.0) {
                        // Round bottom-right
                        pixel_color = ColorRGBA::new(0, 0, 0, 0);
                    }
                }
                if ((x as f32) > (vx - self.border_top_right_radius)) & ((y as f32) < (cy + self.border_top_right_radius)) {
                    if (x - vx + self.border_top_right_radius).powf(2.0) + (y - cy - self.border_top_right_radius).powf(2.0) > self.border_top_right_radius.powf(2.0) {
                        // Round top-right
                        pixel_color = ColorRGBA::new(0, 0, 0, 0);
                    }
                }
            }
            else {
               pixel_color = ColorRGBA::new(0, 0, 0, 0)
           }
        } else {
            pixel_color = ColorRGBA::new(0, 0, 0, 0)
        }
        if ((x as f32) <= bx) & ((x as f32) >= self.left - self.border_width) {
            if ((y as f32) <= by) & ((y as f32) >= self.top - self.border_width) {

                // Borders
                if ((x as f32) - self.left + self.border_width) < self.border_width {
                    // border-left
                    if self.border_style == 1. {
                        if Element::_if_in_dotted(y as usize, dot_step) {
                            pixel_color = self.border_color.clone();
                        }
                    } else {
                        pixel_color = self.border_color.clone();
                    }
                }
                if (ex + self.border_width - (x as f32)) < self.border_width {
                    // border-right
                    if self.border_style == 1. {
                        if Element::_if_in_dotted(y as usize, dot_step) {
                            pixel_color = self.border_color.clone();
                        }
                    } else {
                        pixel_color = self.border_color.clone();
                    }
                }
                if ((y as f32) - self.top + self.border_width) < self.border_width {
                    // border-top
                    if self.border_style == 1. {
                        if Element::_if_in_dotted(x as usize, dot_step) {
                            pixel_color = self.border_color.clone();
                        }
                    } else {
                        pixel_color = self.border_color.clone();
                    }
                }
                if (ey + self.border_width - (y as f32)) < self.border_width {
                    // border-bottom
                    if self.border_style == 1. {
                        if Element::_if_in_dotted(x as usize, dot_step) {
                            pixel_color = self.border_color.clone();
                        }
                    } else {
                        pixel_color = self.border_color.clone();
                    }
                }



            }
        }

        // Return rgba
        return pixel_color
    }

    fn _if_in_dotted(x: usize, st: usize) -> bool {
        ((x / st) % 2) == 0
    }
    pub fn calc(&mut self, parent_width: f32, parent_height: f32) {

        self.right = parent_width - self.left - self.width;
        self.bottom = parent_height - self.top - self.height;

    }
    /// If cord in element container
    pub fn is_in(&mut self, x: f32, y: f32) -> bool {
        if (x <= self.left + self.width) & (x >= self.left) {
            if (y <= self.top + self.height) & (y >= self.top) {
                return true
            }
        }
        return false
    }
}
#[allow(dead_code)]
pub struct TextNode {
    pub content: String,
    pub font: Font,
    pub x: f32,
    pub y: f32,
}
#[allow(dead_code)]
impl TextNode {
    pub fn new(s: String, ctx: &mut Context, x: f32, y: f32) -> TextNode {

        let mut buffer = Vec::new();
        match filesystem::open(ctx, "/Fonts/segoeuisl.ttf") {
            Err(e) => {
                println!("{:?}", e);
            },
            Ok(mut f) => {
                f.read_to_end(&mut buffer).unwrap();
            }
        }
        let font = Font::new_glyph_font_bytes(ctx, &buffer).unwrap();

        TextNode { content: s, font: font, x: x, y: y }
    }
}
#[allow(dead_code)]
pub struct Layer {
    pub layout: Vec<u8>,
    pub elements: Vec<Element>,
    pub textnodes: Vec<TextNode>,
    pub w: f32,
    pub h: f32,
    pub need_calc: bool,
}
#[allow(dead_code)]
impl Layer {
    pub fn new(w: f32, h: f32) -> Layer {
        Layer { layout: Vec::new(), elements: Vec::new(), textnodes: Vec::new(), w: w, h: h, need_calc: true, }
    }
    pub fn calc(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
        if self.need_calc {
            for el in &mut self.elements {
                el.calc(self.w, self.h);
            }
            self.need_calc = false;
        }
    }
    pub fn draw(&mut self) {
        self.layout = Vec::new();
        for col in 0..(self.h as usize) {
            for row in 0..(self.w as usize) {
                let mut color = ColorRGBA::new(0, 0, 0, 0);
                for el in &mut self.elements {
                    let m_c = el.draw_pixel(row as f32, col as f32);
                    if m_c.a != 0 {
                        color = m_c;
                    }
                }
                self.layout.push(color.r);
                self.layout.push(color.g);
                self.layout.push(color.b);
                self.layout.push(color.a);
            }
        }
    }
    /// Search an element in this layer by the key.
    pub fn search_by_key(&mut self, key: &mut usize) -> Result<&mut Element, &'static str> {
        for el in &mut self.elements {
            if el.key == *key {
                return Ok(el)
            }
        }
        Err("Nothing found in this layer")
    }
}
