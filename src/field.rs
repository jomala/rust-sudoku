use rand;
use rand::Rng;
use std::iter::Iterator;
use std::rc::Rc;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coords {
    pub x: u8,
    pub y: u8
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub digit: Option<u8>,
    pub fixed: bool
}

pub enum Neighbour {
	In,
	Out,
	OutOfBounds,
}

#[derive(Clone)]
pub struct Region {
	pub cells: Vec<Coords>,
	pub sum: u32,
}

impl Region {
	pub fn new(loc: Coords, sum: u32) -> Region {
		let mut vec = Vec::new();
		vec.push(loc);
		Region { cells: vec, sum: sum }
	}
	
	pub fn join(mut self, mut other: Region) -> Region {
		self.cells.append(&mut other.cells);
		Region { cells: self.cells, sum: self.sum + other.sum }
	}
	
	pub fn includes(&self, loc: &Coords) -> bool {
		self.cells.iter().find(|&x| x == loc).is_some()
	}
	
	pub fn is_neighbour(&self, loc: &Coords) -> bool {
        !self.includes(&loc) &&
		((loc.x > 0 && self.includes(&Coords { x: loc.x-1, y: loc.y })) ||
		 (loc.y > 0 && self.includes(&Coords { x: loc.x, y: loc.y-1 })) ||
		 (loc.x < 8 && self.includes(&Coords { x: loc.x+1, y: loc.y })) ||
		 (loc.y < 8 && self.includes(&Coords { x: loc.x, y: loc.y+1 })))
	}
	
	pub fn neighbours(&self) -> Vec<Coords> {
		let mut vec = Vec::new();
		for x in 0..9 {
			for y in 0..9 {
				let coords = Coords { x, y };
				if self.is_neighbour(&coords) {
					vec.push(coords);
				}
			}
		}
		vec		
	}
}

#[derive(Clone)]
pub struct Field {
    pub cells: [[Cell; 9]; 9],
	pub regions: Rc<Vec<Region>>,
}

impl Field {
    pub fn new() -> Field {
        let mut field = Field {
            cells: [[Cell{ digit: None, fixed: false }; 9]; 9],
			regions: Rc::new(Vec::new()),
        };
        field.fill_random();
        field
    }

    pub fn new_with_regions() -> Field {
        let mut field = Field {
            cells: [[Cell{ digit: None, fixed: false }; 9]; 9],
			regions: Rc::new(Vec::new()),
        };
        field.fill_random_regions();
        field
    }

	pub fn count_empty(&self) -> u32 {
		let mut count = 0;
		for x in 0..9 {
			for y in 0..9 {
				if self.cells[y as usize][x as usize].digit.is_none() {
					count += 1;
				}
			}
		}
		count
	}
	
	pub fn count_options(&self) -> u32 {
		let mut temp = self.clone();
		let mut count = 0;
		for x in 0..9 {
			for y in 0..9 {
				if temp.cells[y as usize][x as usize].digit.is_none() {
					for n in 0..9 {
						if temp.find_conflict(&Coords{ x, y }, n).is_none()  {
							count += 1;						
						}
					}
				}
			}
		}
		count
	}
	
    pub fn get_cell(&mut self, x: u8, y: u8) -> &mut Cell {
        &mut self.cells[y as usize][x as usize]
    }
	
	pub fn get_region(&self, loc: &Coords) -> Option<(usize, &Region)> {
		(*self.regions).iter().enumerate().find(|&x| x.1.includes(loc))
	}

    pub fn find_conflict(&mut self, coords: &Coords,
                          digit: u8) -> Option<Coords> {
        for x in 0..9 {
            if x != coords.x {
                if let Some(cell_digit) = self.get_cell(x, coords.y).digit {
                    if cell_digit == digit {
                        return Some(Coords{ x: x, y: coords.y});
                    }
                }
            }
        }

        for y in 0..9 {
            if y != coords.y {
                if let Some(cell_digit) = self.get_cell(coords.x, y).digit {
                    if cell_digit == digit {
                        return Some(Coords{ x: coords.x, y: y});
                    }
                }
            }
        }

        let section = Coords{ x: coords.x / 3, y: coords.y / 3};
        for x in section.x * 3 .. (section.x + 1) * 3 {
            for y in section.y * 3 .. (section.y + 1) * 3 {
                if x != coords.x || y != coords.y {
                    if let Some(cell_digit) = self.get_cell(x, y).digit {
                        if cell_digit == digit {
                            return Some(Coords{ x: x, y: y});
                        }
                    }
                }
            }
        }

        None
    }

    pub fn clear(&mut self) {
        for y in 0..9 {
            for x in 0..9 {
                self.cells[x][y] = Cell{ digit: None, fixed: false };
            }
        }
    }

    pub fn fill_random(&mut self) {
        self.clear();

        let x = rand::thread_rng().gen_range(0u8, 9u8);
        let y = rand::thread_rng().gen_range(0u8, 9u8);
        let digit = rand::thread_rng().gen_range(1u8, 10u8);
        self.get_cell(x, y).digit = Some(digit);
        let solution = self.find_solution().unwrap();
		
        self.cells = solution.cells;
		let mut fails = 20;

        while fails > 0 {
            let mut x;
            let mut y;
            let digit;

            loop {
                x = rand::thread_rng().gen_range(0u8, 9u8);
                y = rand::thread_rng().gen_range(0u8, 9u8);
                if self.get_cell(x, y).digit.is_none() {
                    continue;
                }
                digit = self.get_cell(x, y).digit.unwrap();
                self.get_cell(x, y).digit = None;
                break;
            }

            let solutions = self.find_solutions(2);
            if solutions.len() == 1 {
                continue;
            }
            self.get_cell(x, y).digit = Some(digit);
			fails -= 1;
        }
    }

    pub fn fill_random_regions(&mut self) {
        self.clear();

        let x = rand::thread_rng().gen_range(0u8, 9u8);
        let y = rand::thread_rng().gen_range(0u8, 9u8);
        let digit = rand::thread_rng().gen_range(1u8, 10u8);
        self.get_cell(x, y).digit = Some(digit);
        let mut solution = self.find_solution().unwrap();
        self.clear();
		
		{
			let region_list = Rc::get_mut(&mut self.regions).expect("Region found");
			for y in 0..9 {
				for x in 0..9 {
					let sum = solution.get_cell(x, y).digit.unwrap() as u32;
					let reg = Region::new(Coords { x, y }, sum);
					region_list.push(reg);
				}
			}
		}

		let mut fails = 100;

        while fails > 0 {
			let region;
			let region2;
			{
				assert!(self.regions.len() > 1);
				let r_idx = {
					let x = rand::thread_rng().gen_range(0u8, 9u8);
					let y = rand::thread_rng().gen_range(0u8, 9u8);
					let loc = Coords { x, y };
					self.get_region(&loc).expect("All cells in a region").0
				};
				region = Rc::get_mut(&mut self.regions).expect("Region found").swap_remove(r_idx);
				
				let n_list = region.neighbours();
				assert!(n_list.len() > 0);
				let n_idx = rand::thread_rng().gen_range(0, n_list.len());
				let n = n_list[n_idx];
				
				let (r2_idx, _) = self.get_region(&n).expect("All neighbours in another region");
				let region_list = Rc::get_mut(&mut self.regions).expect("Region found");
				region2 = region_list.swap_remove(r2_idx);
				
				let region_comb = region.clone().join(region2.clone());
				region_list.push(region_comb);
			}
			
            let solutions = self.find_solutions(2);
            if solutions.len() == 1 {
                continue;
            }
			
			{
				let region_list = Rc::get_mut(&mut self.regions).expect("Region found");
				region_list.pop();
				region_list.push(region);
				region_list.push(region2);
			}
			fails -= 1;
        }
    }

    pub fn fill_solution(&mut self) {
        if let Some(s) = self.find_solution() {
            self.cells = s.cells;
        }
    }

    pub fn find_solution(&mut self) -> Option<Field> {
        let solutions = self.find_solutions(1);
        if solutions.len() > 0 {
            return Some(solutions[0].clone());
        }
        None
    }

    pub fn find_solutions(&mut self, stop_at: u32) -> Vec<Field> {
        let mut solutions = Vec::new();
        let mut field = self.clone();
        field.find_solutions_impl(&mut solutions, stop_at);
        solutions
    }

    fn find_solutions_impl(&mut self, solutions: &mut Vec<Field>,
                           stop_at: u32) -> bool {
        let mut empty_cell: Option<Coords> = None;
        'outer: for y in 0..9 {
            'inner: for x in 0..9 {
                if self.get_cell(x, y).digit.is_none() {
                    empty_cell = Some(Coords{ x: x, y: y });
                    break 'outer;
                }
            }
        }

        if empty_cell.is_none() {
            solutions.push(self.clone());
            return solutions.len() >= (stop_at as usize);
        }
        let coords = empty_cell.unwrap();

        let mut digits: Vec<u8> = (1..10).collect();
        rand::thread_rng().shuffle(&mut digits);

        for &digit in digits.iter() {
            if self.find_conflict(&coords, digit).is_none() { 
                self.get_cell(coords.x, coords.y).digit = Some(digit);
                if self.find_solutions_impl(solutions, stop_at) {
                    return true;
                }
                self.get_cell(coords.x, coords.y).digit = None;
            }
        }

        false
    }
}

#[test]
pub fn test_regions() {
	let field = Field::new_with_regions();
	for i in (*field.regions).iter() {
		print!("Region: sum {}", i.sum);
		for _j in i.cells.iter() {
			print!("Cell");
		}
	}
}