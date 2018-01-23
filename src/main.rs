//! rust-sudoku

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use piston::window::WindowSettings;
use piston::input::*;
use piston::event_loop::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use std::path::Path;

mod app;
mod field;
mod settings;

use settings::{Look, Vec2f};

fn main() {
    let look = Look::standard33();

    let opengl = OpenGL::V3_2;
	let wind_cells_f: Vec2f = look.wind_cells.clone().into();
	let wind_size_f = Vec2f { x: look.cell_size.x * wind_cells_f.x + look.thick_line_thickness,
			                  y: look.cell_size.y * wind_cells_f.y + look.thick_line_thickness + look.message_height};
    let mut window: GlutinWindow =
        WindowSettings::new("Sudoku", [wind_size_f.x as u32, wind_size_f.y as u32])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();
    let ref mut gl = GlGraphics::new(opengl);

    let font_path = Path::new("assets/Verdana.ttf");
    let ref mut cache = GlyphCache::new(font_path).unwrap();

    let mut app = app::App::new(look);
	let mut modifier_key: keyboard::ModifierKey = Default::default();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.on_render(&args, gl, cache);
        }

        if let Some(button) = e.press_args() {
			modifier_key.event(&e);
            app.on_button_press(&button, &modifier_key);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.on_mouse_move(&args);
        }
    }
}
