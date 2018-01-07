use piston::input::*;
use graphics;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;

use field;
use settings::{Vec2f, Look};

lazy_static! {
	static ref OVERLAY_TEXT: Vec<&'static str> = vec![
		"Use mouse or arrow keys to select cells.",
		"",
		"Keys:",
		"* **1-9** - fill in a digit",
		"* **Backspace** - clear a cell",
		"* **R** - generate new",
		"* **S** - show solution",
		"* **Esc** - exit",
	];
}

pub struct App {
    look: Look,
    mouse_coords: Vec2f,
    field: field::Field,
    selected_cell: Option<field::Coords>,
    conflicting_cell: Option<field::Coords>,
	show_overlay: bool,
}

impl App {
    pub fn new(look: Look) -> App {
        App {
            look: look,
            mouse_coords: Vec2f{ x: 0.0, y: 0.0 },
            field: field::Field::new(),
            selected_cell: None,
            conflicting_cell: None,
			show_overlay: false,
        }
    }

    pub fn on_render(&mut self, args: &RenderArgs,
                     gl: &mut GlGraphics, cache: &mut GlyphCache) {
        gl.draw(args.viewport(), |c, g| {
            use graphics::*;
            clear(self.look.color_base, g);
			
			let grid_trans = c.transform.trans(
                            self.look.thick_line_thickness / 2.0,
                            self.look.thick_line_thickness / 2.0);

            let pointed_cell = field::Coords{
                x: (self.mouse_coords.x / f64::from(self.look.cell_size.x))
                    .floor() as u8,
                y: (self.mouse_coords.y / f64::from(self.look.cell_size.y))
                    .floor() as u8 };
            rectangle(self.look.color_pointed,
                      [(pointed_cell.x as f64) * self.look.cell_size.x,
                       (pointed_cell.y as f64) * self.look.cell_size.y,
                       self.look.cell_size.x, self.look.cell_size.y],
                      grid_trans, g);

            for y in 0..9 {
                for x in 0..9 {
                    let cell = self.field.get_cell(x, y);
                    if cell.fixed {
                        rectangle(self.look.color_fixed,
                            [(x as f64) * self.look.cell_size.x,
                             (y as f64) * self.look.cell_size.y,
                             self.look.cell_size.x,
                             self.look.cell_size.y],
                            grid_trans, g);
                    }
                }
            }

            if let Some(ref cell) = self.selected_cell {
                if let Some(digit) = self.field.get_cell(cell.x, cell.y).digit {
                    for y in 0..9 {
                        for x in 0..9 {
                            if let Some(other_digit) =
                                    self.field.get_cell(x, y).digit {
                                if other_digit == digit {
                                    rectangle(self.look.color_matching,
                                        [(x as f64) * self.look.cell_size.x,
                                         (y as f64) * self.look.cell_size.y,
                                         self.look.cell_size.x,
                                         self.look.cell_size.y],
                                        grid_trans, g);
                                }
                            }
                        }
                    }
                }
            }

            if let Some(ref cell) = self.conflicting_cell {
                rectangle(self.look.color_conflicting,
                          [(cell.x as f64) * self.look.cell_size.x,
                           (cell.y as f64) * self.look.cell_size.y,
                           self.look.cell_size.x, self.look.cell_size.y],
                          grid_trans, g);
            }

            if let Some(ref cell) = self.selected_cell {
                rectangle(self.look.color_selected,
                          [(cell.x as f64) * self.look.cell_size.x,
                           (cell.y as f64) * self.look.cell_size.y,
                           self.look.cell_size.x, self.look.cell_size.y],
                          grid_trans, g);
            }

            for y in 0..9 {
                for x in 0..9 {
                    if let Some(ref digit) = self.field.cells[y][x].digit {
                        let transform = grid_trans.trans(
                            (x as f64) * self.look.cell_size.x +
                                self.look.text_offset.x,
                            (y as f64) * self.look.cell_size.y +
                                self.look.text_offset.y);
                        let text = graphics::Text::new(self.look.font_size);
                        text.draw(&digit.to_string(), cache,
                                  &c.draw_state, transform, g);
                    }
                }
            }

			let wind_cells_f: Vec2f = self.look.wind_cells.clone().into();
            for n in 0..(self.look.wind_cells.x + 1) {
                let thick = match n % self.look.box_cells.x {
					0 => self.look.thick_line_thickness,
					_ => self.look.thin_line_thickness,
                };
                rectangle(self.look.color_lines,
                          [f64::from(n) * self.look.cell_size.x - thick / 2.0,
                           - thick / 2.0, 
						   thick,
						   wind_cells_f.y * self.look.cell_size.y + thick],
                           grid_trans, g);
		    }
            for n in 0..(self.look.wind_cells.y + 1) {
                let thick = match n % self.look.box_cells.y {
					0 => self.look.thick_line_thickness,
					_ => self.look.thin_line_thickness,
                };
                rectangle(self.look.color_lines,
                          [- thick / 2.0, 
						   f64::from(n) * self.look.cell_size.y - thick / 2.0,
						   wind_cells_f.x * self.look.cell_size.x + thick,
                           thick],
                           grid_trans, g);
            }
			
			if self.show_overlay {
				rectangle(self.look.color_overlay_back,
						  [0.0,
						  0.0,
						  (self.look.wind_cells.x as f64) * self.look.cell_size.x + self.look.thick_line_thickness,
						  (self.look.wind_cells.y as f64) * self.look.cell_size.y + self.look.thick_line_thickness],
						  grid_trans, g);
				for (n, s) in OVERLAY_TEXT.iter().enumerate() {
					let transform = grid_trans.trans(
						self.look.overlay_offset.x,
						self.look.overlay_offset.y +
						self.look.overlay_text_interval * (n as f64));
					let mut text = graphics::Text::new(self.look.overlay_font_size);
					text.color = self.look.color_overlay;
					text.draw(s, cache,
							  &c.draw_state, transform, g);
				}
			}
        });
    }

    pub fn on_button_press(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(key) => {
                self.on_key_down(&key);
            },
            &Button::Mouse(button) => {
                self.on_mouse_click(&button);
            }
            &Button::Controller(_) => {}
        }
    }

    fn on_key_down(&mut self, pressed_key: &Key) {
        let key_digit_mapping = [
            (Key::D1, 1), (Key::D2, 2), (Key::D3, 3),
            (Key::D4, 4), (Key::D5, 5), (Key::D6, 6),
            (Key::D7, 7), (Key::D8, 8), (Key::D9, 9) ];
        for &(key, digit) in key_digit_mapping.iter() {
            if pressed_key == &key {
                if let Some(ref cell) = self.selected_cell {
                    if !self.field.get_cell(cell.x, cell.y).fixed {
                        match self.field.find_conflict(cell, digit) {
                            Some(coords) => {
                                self.conflicting_cell = Some(coords);
                            },
                            None => {
                                self.field.get_cell(cell.x, cell.y).digit =
                                    Some(digit);
                                self.conflicting_cell = None;
                            }
                        }
                    }
                }
            }
        }
        if pressed_key == &Key::Backspace {
            if let Some(ref cell) = self.selected_cell {
                if !self.field.get_cell(cell.x, cell.y).fixed {
                    self.field.get_cell(cell.x, cell.y).digit = None;
                    self.conflicting_cell = None;
                }
            }
        }
        if pressed_key == &Key::S {
            self.field.fill_solution();
            self.conflicting_cell = None;
            self.selected_cell = None;
        }
        if pressed_key == &Key::R {
            self.field.fill_random();
            self.conflicting_cell = None;
            self.selected_cell = None;
        }
        if pressed_key == &Key::Up {
            match self.selected_cell {
                Some(ref mut cell) => if cell.y > 0 { cell.y -= 1; },
                None => self.selected_cell = Some(field::Coords{ x: 0, y: 0})
            }
        }
        if pressed_key == &Key::Down {
            match self.selected_cell {
                Some(ref mut cell) => if (cell.y as u32) < (self.look.wind_cells.y - 1) { cell.y += 1; },
                None => self.selected_cell = Some(field::Coords{ x: 0, y: 0})
            }
        }
        if pressed_key == &Key::Left {
            match self.selected_cell {
                Some(ref mut cell) => if cell.x > 0 { cell.x -= 1; },
                None => self.selected_cell = Some(field::Coords{ x: 0, y: 0})
            }
        }
        if pressed_key == &Key::Right {
            match self.selected_cell {
                Some(ref mut cell) => if (cell.x as u32) < (self.look.wind_cells.x - 1) { cell.x += 1; },
                None => self.selected_cell = Some(field::Coords{ x: 0, y: 0})
            }
        }
        if pressed_key == &Key::Tab {
            self.show_overlay = !self.show_overlay;
        }
    }

    fn on_mouse_click(&mut self, button: &MouseButton) {
        if let &MouseButton::Left = button {
            self.selected_cell = Some(field::Coords{
                x: (self.mouse_coords.x / self.look.cell_size.x) as u8,
                y: (self.mouse_coords.y / self.look.cell_size.y) as u8 });
        }
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coords.x = args[0];
        self.mouse_coords.y = args[1];
    }
}
