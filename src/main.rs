// pub extern crate gl;
// pub extern crate glutin;

pub use std::ffi::CString;
pub use std::mem;
pub use std::io;
pub use std::time::Duration;
pub use std::time::SystemTime;
pub use std::time::Instant;
pub use std::thread;
pub use std::ptr;
pub use std::str;
pub use std::sync::mpsc;
// pub use gl::types::*;
// pub use gl::*;
// pub use glutin::*;
// pub use glutin::error::*;
// pub use glutin::event::*;
// pub use glutin::event_loop::*;
// pub use glutin::monitor::*;
// pub use glutin::platform::*;
// pub use glutin::window::*;
// pub use glutin::dpi::*;
pub use colored::*;
pub use glm::*;
pub use glm::ext::*;
pub use glm::builtin::*;
pub use std::ops::Add;
pub use std::ops::Neg;
pub use std::ops::Mul;

pub use ggez::*;
pub use ggez::event::*;
pub use ggez::graphics::*;

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Color {
    fn new(r: u8, g: u8, b:u8, a: u8) -> Color {
        Color {r: r, g: g, b: b, a: a,}
    }
}
#[allow(dead_code)]
struct Element {
    top: f32,
    left: f32,
    height: f32,
    width: f32,

    border_width: f32,
    border_style: f32,
    border_color: Color,

    background_color: Color,

    _need_render: bool,
}
#[allow(dead_code)]
impl Element {
    fn new(t: f32, l: f32, w: f32, h: f32) -> Element {
        Element { top: t, left: l, height: h, width: w, border_width: 1., border_style: 0., background_color: Color::new(255, 0, 0, 255), border_color: Color::new(0, 0, 255, 255), _need_render: true}
    }
    fn calc_pixel(&mut self, x: f32, y: f32) -> (u8, u8, u8, u8) {
        // Calc pos

        let ex = self.left + self.width;
        let ey = self.top + self.height;
        //
        // let bx = self.left + self.border_width;
        // let by = self.top + self.border_width;

        // Pre-color
        let mut r: u8 = self.background_color.r;
        let mut g: u8 = self.background_color.g;
        let mut b: u8 = self.background_color.b;
        let mut a: u8 = self.background_color.a;

        let dot_step: usize = 5;

        // If is rendering
        if ((x as f32) < ex) & ((x as f32) > self.left) {
            if ((y as f32) < ey) & ((y as f32) > self.top) {
                if ((x as f32) - self.left) <= self.border_width {
                    // border-left
                    if Element::_if_in_dotted(y as usize, dot_step) {
                        r = self.border_color.r;
                        g = self.border_color.g;
                        b = self.border_color.b;
                        a = self.border_color.a;
                    }
                }
                if (ex - (x as f32)) <= self.border_width {
                    // border-right
                    if Element::_if_in_dotted(y as usize, dot_step) {
                        r = self.border_color.r;
                        g = self.border_color.g;
                        b = self.border_color.b;
                        a = self.border_color.a;
                    }
                }
                if ((y as f32) - self.top) <= self.border_width {
                    // border-top
                    if Element::_if_in_dotted(x as usize, dot_step) {
                        r = self.border_color.r;
                        g = self.border_color.g;
                        b = self.border_color.b;
                        a = self.border_color.a;
                    }
                }
                if (ey - (y as f32)) <= self.border_width {
                    // border-bottom
                    if Element::_if_in_dotted(x as usize, dot_step) {
                        r = self.border_color.r;
                        g = self.border_color.g;
                        b = self.border_color.b;
                        a = self.border_color.a;
                    }
                }
            }
            else {
               a = 0;
           }
        } else {
            a = 0;
        }
        // Return rgba
        return (r, g, b, a)
    }
    fn _if_in_dotted(x: usize, st: usize) -> bool {
        ((x / st) % 2) == 0
    }
}

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("game_name", "author_name")
           .build()
           .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct MyGame {
    layout: Vec<u8>,
    w: f32,
    h: f32,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        MyGame { layout: Vec::new(), w: 0., h: 0., }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        let mut ele = Element::new(10.0, 40.0, 100.0, 200.0);

        let cords = graphics::screen_coordinates(ctx);
        let width = cords.w;
        let height = cords.h;
        let _full_size = (4. * width * height) as usize;

        let mut paint: Vec<u8> = Vec::new();

        if ele._need_render {
            for col in 0..(height as usize) {
                for row in 0..(width as usize) {

                        let (r, g, b, a) = ele.calc_pixel(row as f32, col as f32);
                        paint.push(r);
                        paint.push(g);
                        paint.push(b);
                        paint.push(a);

                }
            }
            self.layout = paint;
            ele._need_render = false;
        }

        self.w = width;
        self.h = height;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        let rgba: &[u8] = &self.layout[..]; // Layout - rendered context
        let imge_to_render = Image::from_rgba8(ctx, self.w as u16, self.h as u16, rgba)?;
        let _result = graphics::draw(ctx, &imge_to_render, DrawParam::default())?;
        //thread::sleep(Duration::from_millis(1));
        graphics::present(ctx)
    }
}
