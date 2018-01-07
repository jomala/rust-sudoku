use std::convert::From;

#[derive(Debug, Clone, Copy)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl From<Vec2u> for Vec2f {
	fn from(orig: Vec2u) -> Vec2f {
	    Vec2f { x: orig.x as f64, y: orig.y as f64 }
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2u {
    pub x: u32,
    pub y: u32,
}

impl From<Vec2f> for Vec2u {
	fn from(orig: Vec2f) -> Vec2u {
	    Vec2u { x: orig.x as u32, y: orig.y as u32 }
	}
}

pub struct Look {
    pub cell_size: Vec2f,
	pub wind_cells: Vec2u,
	pub box_cells: Vec2u,
    pub font_size: u32,
    pub text_offset: Vec2f,
	pub overlay_font_size: u32,
	pub overlay_offset: Vec2f,
	pub overlay_text_interval: f64,
	pub thick_line_thickness: f64,
	pub thin_line_thickness: f64,	
	pub color_base: [f32; 4],
	pub color_pointed: [f32; 4],
	pub color_fixed: [f32; 4],
	pub color_matching: [f32; 4],
	pub color_conflicting: [f32; 4],
	pub color_selected: [f32; 4],
	pub color_lines: [f32; 4],
	pub color_overlay: [f32; 4],
	pub color_overlay_back: [f32; 4],
}

#[allow(dead_code)]
impl Look {
    pub fn standard33() -> Look {
        Look {
            cell_size: Vec2f{ x: 75.0, y: 75.0 },
			wind_cells: Vec2u{ x: 9, y: 9 },
			box_cells: Vec2u{ x: 3, y: 3 },
            font_size: 48,
            text_offset: Vec2f{ x: 20.0, y: 55.0 },
			overlay_font_size: 24,
			overlay_offset: Vec2f{ x: 50.0, y: 200.0 },
			overlay_text_interval: 40.0,
			thick_line_thickness: 6.0,
			thin_line_thickness: 2.0,
			color_base: [1.0; 4],
			color_pointed: [0.95, 0.95, 0.95, 1.0],
			color_fixed: [0.9, 0.9, 0.9, 1.0],
			color_matching: [0.8, 0.8, 0.9, 1.0],
			color_conflicting: [0.9, 0.8, 0.8, 1.0],
			color_selected: [0.8, 0.9, 0.8, 1.0],
			color_lines: [0.0, 0.0, 0.0, 1.0],
			color_overlay: [0.0, 0.0, 0.5, 1.0],
			color_overlay_back: [1.0, 1.0, 1.0, 0.5],
        }
    }

	pub fn funky32() -> Look {
        Look {
            cell_size: Vec2f{ x: 100.0, y: 100.0 },
			wind_cells: Vec2u{ x: 6, y: 6 },
			box_cells: Vec2u{ x: 3, y: 2 },
            font_size: 64,
            text_offset: Vec2f{ x: 30.0, y: 75.0 },
			overlay_font_size: 24,
			overlay_offset: Vec2f{ x: 250.0, y: 250.0 },
			overlay_text_interval: 50.0,
			thick_line_thickness: 8.0,
			thin_line_thickness: 2.0,
			color_base: [0.0; 4],
			color_pointed: [0.95, 0.95, 0.95, 1.0],
			color_fixed: [0.9, 0.9, 0.9, 1.0],
			color_matching: [0.8, 0.8, 0.9, 1.0],
			color_conflicting: [0.9, 0.8, 0.8, 1.0],
			color_selected: [0.8, 0.9, 0.8, 1.0],
			color_lines: [0.5, 0.5, 0.0, 1.0],
			color_overlay: [0.0, 0.0, 0.5, 1.0],
			color_overlay_back: [1.0, 1.0, 1.0, 0.5],
        }
    }
}
